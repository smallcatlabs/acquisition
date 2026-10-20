//! The stdio transport `acq-mcp` serves on: every request it receives
//! gets a reply.
//!
//! `rmcp`'s own stdio reader drops a line `serde_json` refuses as syntax
//! with no reply, and a message nested past `serde_json`'s 127 levels is
//! refused as syntax too, so a deep query tree or a deep value was lost
//! there and the caller waited (a frontends park until this, measured at
//! 123 nots or 62 groups in a search tree). This reader sits between stdin and `rmcp`: a line that
//! parses is handed on unchanged; one that does not is answered here, in
//! the order it came, and never reaches `rmcp`.
//!
//! - Not JSON: a JSON-RPC parse error (`-32700`), id null, as JSON-RPC
//!   says when the id cannot be read.
//! - JSON nested past the limit: the id and method are read from the
//!   envelope's top level without parsing what is nested. A `tools/call`
//!   is answered as every tool refusal is, an `isError` result with
//!   `kind: "arguments"` naming the limit; any other method with an
//!   id is an invalid request (`-32600`) naming it; a notification, with
//!   no id, has no reply to get.
//!
//! Both halves of the server's output go through one writer task, a
//! whole line at a time, so a reply written here never splits one of
//! `rmcp`'s.

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, DuplexStream};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// The deepest nesting `serde_json` reads; one level more is refused
/// (its recursion limit of 128, measured: 127 brackets read, 128 not).
pub const MAX_DEPTH: usize = 127;

/// The two ends `rmcp` serves on, and the writer task: awaited after
/// the service ends, so its last lines reach stdout before the process
/// exits.
pub fn stdio() -> ((DuplexStream, DuplexStream), JoinHandle<()>) {
    let (to_rmcp, rmcp_reads) = tokio::io::duplex(64 * 1024);
    let (rmcp_writes, from_rmcp) = tokio::io::duplex(64 * 1024);
    let (replies, replies_rx) = mpsc::unbounded_channel::<String>();
    tokio::spawn(read(tokio::io::stdin(), to_rmcp, replies));
    let writer = tokio::spawn(write(from_rmcp, replies_rx, tokio::io::stdout()));
    ((rmcp_reads, rmcp_writes), writer)
}

/// Hand each line that parses to `rmcp`; answer the rest.
async fn read(
    input: impl tokio::io::AsyncRead + Unpin,
    mut to_rmcp: DuplexStream,
    replies: mpsc::UnboundedSender<String>,
) {
    let mut input = BufReader::new(input);
    let mut line = Vec::new();
    loop {
        line.clear();
        match input.read_until(b'\n', &mut line).await {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        match unreadable(&line) {
            None => {
                if to_rmcp.write_all(&line).await.is_err() {
                    break;
                }
            }
            Some(reply) => {
                if replies.send(reply.to_string()).is_err() {
                    break;
                }
            }
        }
    }
    let _ = to_rmcp.shutdown().await;
}

/// `rmcp`'s lines and this module's replies onto stdout, a line at a time.
async fn write(
    from_rmcp: DuplexStream,
    mut replies: mpsc::UnboundedReceiver<String>,
    mut out: impl AsyncWrite + Unpin,
) {
    let mut from_rmcp = BufReader::new(from_rmcp);
    let mut line = Vec::new();
    let mut replying = true;
    loop {
        tokio::select! {
            read = from_rmcp.read_until(b'\n', &mut line) => {
                match read {
                    Ok(0) | Err(_) => {
                        // `rmcp` is done; a reply already queued still goes.
                        while let Ok(reply) = replies.try_recv() {
                            let _ = out.write_all(format!("{reply}\n").as_bytes()).await;
                        }
                        let _ = out.flush().await;
                        break;
                    }
                    Ok(_) => {}
                }
                if out.write_all(&line).await.is_err() || out.flush().await.is_err() {
                    break;
                }
                line.clear();
            }
            reply = replies.recv(), if replying => match reply {
                Some(reply) => {
                    let reply = format!("{reply}\n");
                    if out.write_all(reply.as_bytes()).await.is_err() || out.flush().await.is_err() {
                        break;
                    }
                }
                None => replying = false,
            },
        }
    }
}

/// The reply a line gets here, or `None` for a line `rmcp` reads (or a
/// blank one, which it skips).
pub fn unreadable(line: &[u8]) -> Option<Value> {
    if line.iter().all(u8::is_ascii_whitespace) {
        return None;
    }
    // Parsed as a `Value`, which serde_json holds to the depth limit as
    // it holds `rmcp`'s typed parse; `IgnoredAny` skips without a limit.
    let err = serde_json::from_slice::<Value>(line).err()?;
    if !err.to_string().starts_with("recursion limit exceeded") {
        return Some(json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": { "code": -32700, "message": format!("Parse error: the message is not JSON ({err})") },
        }));
    }
    let id = top_level(line, "id")?;
    let too_deep = format!(
        "the message nests deeper than the {MAX_DEPTH} levels this server reads, so \
         nothing in it was read: send it with less nesting"
    );
    if top_level(line, "method") == Some(json!("tools/call")) {
        let doc = json!({ "error": too_deep, "kind": "arguments" });
        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [{ "type": "text", "text": doc.to_string() }],
                "structuredContent": doc,
                "isError": true,
            },
        }));
    }
    Some(json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": -32600, "message": too_deep },
    }))
}

/// The value of `key` in the top-level object of `line`, read without
/// parsing what the other members nest: each is skipped by its brackets.
fn top_level(line: &[u8], key: &str) -> Option<Value> {
    let mut at = skip_ws(line, 0);
    if line.get(at) != Some(&b'{') {
        return None;
    }
    at += 1;
    loop {
        at = skip_ws(line, at);
        let name_end = string_end(line, at)?;
        let name: String = serde_json::from_slice(&line[at..name_end]).ok()?;
        at = skip_ws(line, name_end);
        if line.get(at) != Some(&b':') {
            return None;
        }
        at = skip_ws(line, at + 1);
        let value_end = value_end(line, at)?;
        if name == key {
            return serde_json::from_slice(&line[at..value_end]).ok();
        }
        at = skip_ws(line, value_end);
        match line.get(at) {
            Some(b',') => at += 1,
            _ => return None,
        }
    }
}

fn skip_ws(line: &[u8], mut at: usize) -> usize {
    while line.get(at).is_some_and(u8::is_ascii_whitespace) {
        at += 1;
    }
    at
}

/// One past the closing quote of the string opening at `at`.
fn string_end(line: &[u8], at: usize) -> Option<usize> {
    if line.get(at) != Some(&b'"') {
        return None;
    }
    let mut i = at + 1;
    loop {
        match line.get(i)? {
            b'\\' => i += 2,
            b'"' => return Some(i + 1),
            _ => i += 1,
        }
    }
}

/// One past the end of the value starting at `at`: a string, a bracketed
/// value counted by its brackets (strings inside skipped whole), or a
/// scalar running to the next delimiter.
fn value_end(line: &[u8], at: usize) -> Option<usize> {
    match line.get(at)? {
        b'"' => string_end(line, at),
        b'{' | b'[' => {
            let mut depth = 0usize;
            let mut i = at;
            loop {
                match line.get(i)? {
                    b'"' => {
                        i = string_end(line, i)?;
                        continue;
                    }
                    b'{' | b'[' => depth += 1,
                    b'}' | b']' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(i + 1);
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        _ => {
            let mut i = at;
            while line
                .get(i)
                .is_some_and(|b| !matches!(b, b',' | b'}' | b']') && !b.is_ascii_whitespace())
            {
                i += 1;
            }
            Some(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nested(depth: usize) -> String {
        format!("{}1{}", "[".repeat(depth), "]".repeat(depth))
    }

    #[test]
    fn a_line_serde_reads_goes_on_and_a_blank_one_too() {
        assert_eq!(
            unreadable(br#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#),
            None
        );
        assert_eq!(unreadable(b"  \r\n"), None);
        // The envelope is one level: the deepest line read, and one more.
        let deepest = format!(r#"{{"id":1,"params":{}}}"#, nested(MAX_DEPTH - 1));
        assert_eq!(unreadable(deepest.as_bytes()), None);
        let past = format!(r#"{{"id":1,"params":{}}}"#, nested(MAX_DEPTH));
        assert!(unreadable(past.as_bytes()).is_some());
    }

    #[test]
    fn a_line_that_is_not_json_is_a_parse_error_with_a_null_id() {
        let reply = unreadable(br#"{"id":7,"method":"#).unwrap();
        assert_eq!(reply["id"], Value::Null, "{reply}");
        assert_eq!(reply["error"]["code"], json!(-32700), "{reply}");
    }

    /// The id is read past members nested beyond the limit, and past
    /// strings holding brackets and escaped quotes.
    #[test]
    fn a_call_nested_past_the_limit_is_a_tool_refusal_under_its_own_id() {
        let line = format!(
            r#"{{"jsonrpc":"2.0","params":{{"name":"search","note":"a \"]}}[\" b","arguments":{}}},"method":"tools/call","id":"x-9"}}"#,
            nested(MAX_DEPTH + 5)
        );
        let reply = unreadable(line.as_bytes()).unwrap();
        assert_eq!(reply["id"], json!("x-9"), "{reply}");
        assert_eq!(reply["result"]["isError"], json!(true), "{reply}");
        let doc = &reply["result"]["structuredContent"];
        assert_eq!(doc["kind"], json!("arguments"), "{reply}");
        assert!(
            doc["error"].as_str().unwrap().contains("127 levels"),
            "{reply}"
        );
    }

    #[test]
    fn another_method_nested_past_the_limit_is_an_invalid_request_and_a_notification_none() {
        let line = format!(r#"{{"id":3,"method":"ping","params":{}}}"#, nested(200));
        let reply = unreadable(line.as_bytes()).unwrap();
        assert_eq!(reply["id"], json!(3), "{reply}");
        assert_eq!(reply["error"]["code"], json!(-32600), "{reply}");
        let note = format!(r#"{{"method":"notifications/x","params":{}}}"#, nested(200));
        assert_eq!(unreadable(note.as_bytes()), None);
    }
}
