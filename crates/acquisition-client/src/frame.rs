//! The frame reader both sides of the socket use (C85): one line, read
//! no further than [`MAX_FRAME_BYTES`](acquisition_protocol::protocol::MAX_FRAME_BYTES).
//! A line past the bound is
//! discarded through its newline and reported as [`Frame::Oversize`], so
//! the reader never buffers a runaway line and the next frame is still
//! aligned — the daemon answers `bad_request` and keeps the connection,
//! the client reports the answer it could not read and keeps its
//! connection too. Bytes, not text: a frame that is not UTF-8 is the JSON
//! parser's failure to report, like any other malformed frame.
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

/// Read the next frame, buffering at most `max` bytes of it.
pub async fn read_frame<R: AsyncBufRead + Unpin>(
    reader: &mut R,
    max: usize,
) -> std::io::Result<Frame> {
    let mut line = Vec::new();
    let mut discarding = false;
    loop {
        let buf = reader.fill_buf().await?;
        if buf.is_empty() {
            return Ok(if discarding || line.is_empty() {
                Frame::Closed
            } else {
                Frame::Line(line)
            });
        }
        let newline = buf.iter().position(|&b| b == b'\n');
        let chunk = &buf[..newline.unwrap_or(buf.len())];
        if !discarding {
            if line.len() + chunk.len() > max {
                discarding = true;
                line = Vec::new();
            } else {
                line.extend_from_slice(chunk);
            }
        }
        let consumed = chunk.len() + usize::from(newline.is_some());
        reader.consume(consumed);
        if newline.is_some() {
            return Ok(if discarding {
                Frame::Oversize
            } else {
                Frame::Line(line)
            });
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
        let mut reader = BufReader::with_capacity(3, &input[..]);
        assert_eq!(
            read_frame(&mut reader, max).await.unwrap(),
            Frame::Line(b"12345678".to_vec())
        );
        assert_eq!(read_frame(&mut reader, max).await.unwrap(), Frame::Oversize);
        assert_eq!(
            read_frame(&mut reader, max).await.unwrap(),
            Frame::Line(b"{}".to_vec())
        );
        assert_eq!(
            read_frame(&mut reader, max).await.unwrap(),
            Frame::Line(Vec::new())
        );
        // An unterminated last line is still a line; then the close.
        assert_eq!(
            read_frame(&mut reader, max).await.unwrap(),
            Frame::Line(b"last".to_vec())
        );
        assert_eq!(read_frame(&mut reader, max).await.unwrap(), Frame::Closed);
    }

    /// An oversize line the peer never finished is a close, not a frame
    /// to answer.
    #[tokio::test]
    async fn an_oversize_line_cut_off_by_the_close_is_a_close() {
        let input = b"123456789".to_vec();
        let mut reader = BufReader::with_capacity(4, &input[..]);
        assert_eq!(read_frame(&mut reader, 8).await.unwrap(), Frame::Closed);
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
                let mut reader = BufReader::with_capacity(capacity, input);
                let mut frames = Vec::new();
                for _ in 0..=input.len() + 1 {
                    let frame = read_frame(&mut reader, max).await.expect("a slice reads");
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
