//! The frame reader both sides of the socket use (C85): one line, read
//! no further than [`MAX_FRAME_BYTES`](acquisition_protocol::protocol::MAX_FRAME_BYTES).
//! A line past the bound is
//! discarded through its newline and reported as [`Frame::Oversize`], so
//! the reader never buffers a runaway line and the next frame is still
//! aligned — the daemon answers `bad_request` and keeps the connection,
//! the client reports the answer it could not read and keeps its
//! connection too. Bytes, not text: a frame that is not UTF-8 is the JSON
//! parser's failure to report, like any other malformed frame. The
//! reader is cancel-safe, so a `select!` may race it ([`FrameReader`]
//! says why).
//!
//! One copy here and one in `acquisition-daemon/src/frame.rs`, byte for
//! byte (the daemon split's step 3): the reader needs tokio, the protocol
//! crate is serde-only (C1; `tools/arch-check.sh`'s allowlist refuses a
//! tokio feature there), and the daemon never links this crate. The
//! constant both copies read is the protocol's, so the bound cannot
//! drift; the tests below run in both.

use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// One read from a framed connection.
#[derive(Debug, PartialEq, Eq)]
pub enum Frame {
    /// A line, without its newline; possibly empty.
    Line(Vec<u8>),
    /// A line longer than the bound, discarded through its newline; the
    /// connection is aligned on the next frame.
    Oversize,
    /// The peer closed the connection. An oversize line cut off by the
    /// close reads as `Closed` too — there is nobody left to answer.
    Closed,
}

/// A framed connection's read side: the buffered reader, and the line
/// read so far with whether it is being discarded as oversize.
///
/// Cancel-safe (#56): [`FrameReader::next`] may be dropped at any await,
/// as a `select!` drops the branch that lost, and the next call resumes
/// the same line. Its one await is the reader's `fill_buf`, which is
/// cancel-safe; between two awaits it moves bytes from the buffer into
/// the partial line and consumes them in one step, so a byte is either
/// still in the buffer or already in the line, and the line lives here,
/// never in the dropped future.
#[derive(Debug)]
pub struct FrameReader<R> {
    reader: R,
    max: usize,
    line: Vec<u8>,
    discarding: bool,
}

impl<R: AsyncBufRead + Unpin> FrameReader<R> {
    /// Frames over `reader`, buffering at most `max` bytes of each.
    pub fn new(reader: R, max: usize) -> Self {
        Self {
            reader,
            max,
            line: Vec::new(),
            discarding: false,
        }
    }

    /// The reader underneath, for a read that is not a frame's (the
    /// stop's wait for the close).
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Read the next frame.
    pub async fn next(&mut self) -> std::io::Result<Frame> {
        loop {
            let buf = self.reader.fill_buf().await?;
            if buf.is_empty() {
                let line = std::mem::take(&mut self.line);
                let discarding = std::mem::take(&mut self.discarding);
                return Ok(if discarding || line.is_empty() {
                    Frame::Closed
                } else {
                    Frame::Line(line)
                });
            }
            let newline = buf.iter().position(|&b| b == b'\n');
            let chunk = &buf[..newline.unwrap_or(buf.len())];
            if !self.discarding {
                if self.line.len() + chunk.len() > self.max {
                    self.discarding = true;
                    self.line = Vec::new();
                } else {
                    self.line.extend_from_slice(chunk);
                }
            }
            let consumed = chunk.len() + usize::from(newline.is_some());
            self.reader.consume(consumed);
            if newline.is_some() {
                let line = std::mem::take(&mut self.line);
                return Ok(if std::mem::take(&mut self.discarding) {
                    Frame::Oversize
                } else {
                    Frame::Line(line)
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::BufReader;

    /// The bound is exact: a line of `max` bytes is a line, one more is
    /// oversize, and the frame after either is read intact.
    #[tokio::test]
    async fn a_line_over_the_bound_is_discarded_through_its_newline() {
        let max = 8;
        let input = b"12345678\n123456789\n{}\n\nlast".to_vec();
        let mut reader = FrameReader::new(BufReader::with_capacity(3, &input[..]), max);
        assert_eq!(
            reader.next().await.unwrap(),
            Frame::Line(b"12345678".to_vec())
        );
        assert_eq!(reader.next().await.unwrap(), Frame::Oversize);
        assert_eq!(reader.next().await.unwrap(), Frame::Line(b"{}".to_vec()));
        assert_eq!(reader.next().await.unwrap(), Frame::Line(Vec::new()));
        // An unterminated last line is still a line; then the close.
        assert_eq!(reader.next().await.unwrap(), Frame::Line(b"last".to_vec()));
        assert_eq!(reader.next().await.unwrap(), Frame::Closed);
    }

    /// An oversize line the peer never finished is a close, not a frame
    /// to answer.
    #[tokio::test]
    async fn an_oversize_line_cut_off_by_the_close_is_a_close() {
        let input = b"123456789".to_vec();
        let mut reader = FrameReader::new(BufReader::with_capacity(4, &input[..]), 8);
        assert_eq!(reader.next().await.unwrap(), Frame::Closed);
    }

    /// Poll `future` once with a waker that does nothing, then drop it:
    /// a `select!` whose other branch won.
    fn poll_once_and_drop<F: std::future::Future>(future: F) -> std::task::Poll<F::Output> {
        let mut future = std::pin::pin!(future);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        future.as_mut().poll(&mut cx)
    }

    /// #56: a read dropped midway, after half a line and before its
    /// newline arrived, loses nothing: the next read returns the whole
    /// line once, and the frame after it intact.
    #[tokio::test]
    async fn a_read_dropped_midway_resumes_its_line() {
        use tokio::io::AsyncWriteExt;
        let (mut peer, ours) = tokio::io::duplex(64);
        let mut reader = FrameReader::new(BufReader::with_capacity(4, ours), 8);
        peer.write_all(b"{\"a\"").await.unwrap();
        assert!(
            poll_once_and_drop(reader.next()).is_pending(),
            "half a line, then nothing"
        );
        peer.write_all(b":1}\nnext\n").await.unwrap();
        assert_eq!(
            reader.next().await.unwrap(),
            Frame::Line(b"{\"a\":1}".to_vec())
        );
        assert_eq!(reader.next().await.unwrap(), Frame::Line(b"next".to_vec()));
    }

    /// #56: an oversize line whose read is dropped midway, past the bound
    /// and before its newline, still reads as one `Oversize`, and the
    /// frame after it intact.
    #[tokio::test]
    async fn an_oversize_read_dropped_midway_is_still_one_oversize() {
        use tokio::io::AsyncWriteExt;
        let (mut peer, ours) = tokio::io::duplex(64);
        let mut reader = FrameReader::new(BufReader::with_capacity(4, ours), 8);
        peer.write_all(b"123456789").await.unwrap();
        assert!(
            poll_once_and_drop(reader.next()).is_pending(),
            "past the bound, no newline"
        );
        peer.write_all(b"abc\nok\n").await.unwrap();
        assert_eq!(reader.next().await.unwrap(), Frame::Oversize);
        assert_eq!(reader.next().await.unwrap(), Frame::Line(b"ok".to_vec()));
    }

    // ---- generated streams (#85) ----------------------------------------
    //
    // What any local process may write to the socket (client input), and
    // what a daemon of any revision may answer: a stream drawn from JSON
    // frames, arbitrary bytes, newlines and runs at the bound and one past
    // it, read at a drawn buffer capacity and bound. No stream may make the
    // reader return a line over its bound or holding a newline, fail to
    // reach the close, read otherwise than the reference split below, or
    // panic the decode either side runs on a line.

    mod generated_streams {
        use super::*;
        use crate::generated::config;
        use acquisition_protocol::protocol::{Bootstrap, BootstrapReply, Request, Response};
        use proptest::prelude::*;

        /// The frames the reader must return for `input` at `max`: each
        /// newline-terminated piece a line, or oversize past the bound; an
        /// unterminated last piece a line when it is within the bound and
        /// not empty; then the close.
        fn reference(input: &[u8], max: usize) -> Vec<Frame> {
            let mut pieces: Vec<&[u8]> = input.split(|&b| b == b'\n').collect();
            let rest = pieces.pop().unwrap_or_default();
            let mut frames: Vec<Frame> = pieces
                .into_iter()
                .map(|piece| {
                    if piece.len() > max {
                        Frame::Oversize
                    } else {
                        Frame::Line(piece.to_vec())
                    }
                })
                .collect();
            if !rest.is_empty() && rest.len() <= max {
                frames.push(Frame::Line(rest.to_vec()));
            }
            frames.push(Frame::Closed);
            frames
        }

        /// One piece of a stream, for a bound of `max`.
        fn piece(max: usize) -> impl Strategy<Value = Vec<u8>> {
            const FRAMES: &[&str] = &[
                r#"{"req":"hello","version":"1.0.0","contract":"c","world":"/w"}"#,
                r#"{"req":"daemon_stop"}"#,
                r#"{"resp":"hello","pid":7,"provider":"mock","world":"/w"}"#,
                r#"{"resp":"stopping"}"#,
                r#"{"req":"list"}"#,
                "{}",
                "12345678",
                "123456789",
                "last",
            ];
            prop_oneof![
                3 => prop::sample::select(FRAMES).prop_map(|f| f.as_bytes().to_vec()),
                3 => prop::collection::vec(any::<u8>(), 0..24),
                3 => Just(b"\n".to_vec()),
                1 => Just(vec![b'x'; max]),
                1 => Just(vec![b'x'; max + 1]),
                1 => Just(b"\r\n".to_vec()),
            ]
        }

        fn stream() -> impl Strategy<Value = (Vec<u8>, usize, usize)> {
            (1usize..=64, 0usize..=64).prop_flat_map(|(capacity, max)| {
                (
                    prop::collection::vec(piece(max), 0..12).prop_map(|pieces| pieces.concat()),
                    Just(capacity),
                    Just(max),
                )
            })
        }

        /// Read `input` to the close; the reading is bounded by the
        /// stream's length, since every frame but the close consumes a
        /// byte.
        fn read_all(input: &[u8], capacity: usize, max: usize) -> Vec<Frame> {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("a current-thread runtime");
            runtime.block_on(async {
                let mut reader = FrameReader::new(BufReader::with_capacity(capacity, input), max);
                let mut frames = Vec::new();
                for _ in 0..=input.len() + 1 {
                    let frame = reader.next().await.expect("a slice reads");
                    let closed = frame == Frame::Closed;
                    frames.push(frame);
                    if closed {
                        return frames;
                    }
                }
                panic!(
                    "{} frames from {} bytes and no close",
                    frames.len(),
                    input.len()
                );
            })
        }

        proptest! {
            #![proptest_config(config(2_048, 100_000))]

            #[test]
            fn a_generated_stream_reads_as_its_reference_split_and_decodes_without_panic(
                (input, capacity, max) in stream(),
            ) {
                let frames = read_all(&input, capacity, max);
                prop_assert_eq!(&frames, &reference(&input, max));
                for frame in &frames {
                    if let Frame::Line(line) = frame {
                        prop_assert!(line.len() <= max && !line.contains(&b'\n'));
                        let _ = Bootstrap::read(line);
                        let _ = BootstrapReply::read(line);
                        let _ = serde_json::from_slice::<Request>(line);
                        let _ = serde_json::from_slice::<Response>(line);
                    }
                }
            }
        }
    }
}
