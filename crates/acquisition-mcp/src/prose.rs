//! The text block an answer or a refusal is spoken as: a document's
//! JSON with this server's spellings, and its prose fields kept whole.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

/// The CLI's words a daemon's text still speaks, each with what it is
/// here, longest first where one begins another (C143: a shared text
/// speaks the request's own words; since #97 every text the shared
/// crates make does, and no entry translates one). A phrase follows the
/// word in the spelling (`acq accounts` is `accounts`); an empty phrase
/// is a person's command, which the words give a person at a terminal,
/// and which no tool here performs. Drawn from a grep of the daemon's
/// and the client's texts for `acq ` and `--`.
const SPOKEN: [(&str, &str); 9] = [
    ("acq accounts", "is `accounts`"),
    ("acq daemon status", "is `daemon_status`"),
    ("acq daemon stop", ""),
    ("acq daemon reset-tripwire", ""),
    ("acq auth check", ""),
    ("acq auth status", ""),
    ("acq auth", ""),
    ("acq profile", ""),
    (
        "--account",
        "is `account` (`scope.account` in a search request)",
    ),
];

/// Where `word` stands in `words` as a word: not inside a longer one.
fn stands(words: &str, word: &str) -> bool {
    words.match_indices(word).any(|(at, _)| {
        let before = words[..at].chars().next_back();
        let after = words[at + word.len()..].chars().next();
        !before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            && !after.is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    })
}

/// What this server says after shared words that speak the CLI (C143:
/// the shared words first, an adapter's spelling after them): `— here,`
/// and each command or flag they speak with what it is here, or nothing
/// when they speak none. A command begun by a longer one the words also
/// speak is the longer one's (`acq auth` in `acq auth check`).
fn spelling(words: &str) -> String {
    let spoken = SPOKEN;
    let said: Vec<String> = spoken
        .iter()
        .filter(|(word, _)| stands(words, word))
        .filter(|(word, _)| {
            !spoken.iter().any(|(longer, _)| {
                longer.len() > word.len() && longer.starts_with(word) && stands(words, longer)
            })
        })
        .filter(|(_, here)| !here.is_empty())
        .map(|(word, here)| format!("`{word}` {here}"))
        .collect();
    if said.is_empty() {
        String::new()
    } else {
        format!(" — here, {}", said.join(", "))
    }
}

/// Shared words, then this server's [`spelling`] of the CLI they speak.
pub(crate) fn spelled(words: &str) -> String {
    format!("{words}{}", spelling(words))
}

/// The keys of an answer whose strings are the tool's own words, spelled
/// where they speak the CLI; every other string of an answer is data,
/// sent verbatim. A refusal's and `describe`'s strings are all words.
const WORDS: [&str; 2] = ["why", "hint"];

/// The keys whose string names an entry, asked by that name: never
/// rewritten, their spelling carried by the entry's `what` instead.
const NAMES: [&str; 2] = ["name", "construct"];

/// The keys whose value is payload, not prose: what an agent sends back
/// as printed (an offer, a request, its query and params, a
/// continuation, a count's route) or types as written (an example of
/// the language). Nothing under one is spelled, at any depth: the
/// spelling is prose's, and a request it touched would no longer run as
/// printed (the readiness seat's fix-round review, P2). A refusal's
/// `readings` are payload on every refusal: how what was typed could be
/// read, in the request's words (C143); and a refusal's `current` and
/// `accounts` are data, a stored row and names, carried verbatim as the
/// read that prints them does (round 4's review, P3); a receipt's `undo`
/// is an offer, sent back as printed (round 5 of the 2g build);
/// `describe`'s `patterns` echo each pattern as it was typed; and a
/// `body` is evidence, a stored upstream body (`store_refused`'s, `show`'s)
/// returned verbatim, a string of GGG's never the tool's words (the
/// review of #18's build at c1af7364, P3).
const PAYLOAD: [&str; 13] = [
    "offers", "request", "query", "params", "next", "route", "examples", "readings", "current",
    "accounts", "undo", "patterns", "body",
];

/// Which strings of a document are the tool's words.
#[derive(Clone, Copy)]
pub(crate) struct Prose {
    /// Every string not under [`PAYLOAD`] (a refusal, `describe`); else
    /// those under [`WORDS`] alone (an answer).
    pub(crate) all: bool,
}

/// Every text this server serves, spelled (C143): `text`, a serialized
/// JSON document, with [`spelling`] appended inside each string literal
/// that is the tool's words ([`Prose`]), never one under [`PAYLOAD`],
/// and an entry's name's spelling appended to its `what`. Done on the
/// text, so the document's key order (C115) stands.
pub(crate) fn spoken(text: &str, prose: Prose) -> String {
    enum Frame {
        Object {
            key: Option<String>,
            expecting_key: bool,
            name: Option<String>,
            payload: bool,
        },
        Array {
            key: Option<String>,
            payload: bool,
        },
    }
    let payload_key = |key: Option<&str>| key.is_some_and(|k| PAYLOAD.contains(&k));
    // a value is payload when its frame is, or the key it stands under is
    let payload_here = |frames: &[Frame]| match frames.last() {
        Some(Frame::Object { key, payload, .. }) | Some(Frame::Array { key, payload }) => {
            *payload || payload_key(key.as_deref())
        }
        None => false,
    };
    let mut out = String::with_capacity(text.len());
    let mut frames: Vec<Frame> = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((at, c)) = chars.next() {
        match c {
            '"' => {
                // the literal whole, escapes included
                let mut end = at + 1;
                let mut escaped = false;
                for (i, c) in chars.by_ref() {
                    end = i + c.len_utf8();
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == '"' {
                        break;
                    }
                }
                let literal = &text[at..end];
                let value: String = serde_json::from_str(literal).unwrap_or_default();
                match frames.last_mut() {
                    Some(Frame::Object {
                        key, expecting_key, ..
                    }) if *expecting_key => {
                        *key = Some(value);
                        out.push_str(literal);
                        continue;
                    }
                    _ => {}
                }
                let payload = payload_here(&frames);
                let key = match frames.last() {
                    Some(Frame::Object { key, .. }) | Some(Frame::Array { key, .. }) => key.clone(),
                    None => None,
                };
                let key = key.as_deref().unwrap_or("");
                let mut more = String::new();
                if payload {
                } else if NAMES.contains(&key) {
                    if let Some(Frame::Object { name, .. }) = frames.last_mut() {
                        *name = Some(value);
                    }
                } else if prose.all || WORDS.contains(&key) {
                    let named = match frames.last_mut() {
                        Some(Frame::Object { name, .. }) if key == "what" => name.take(),
                        _ => None,
                    };
                    let said = match named {
                        Some(name) => format!("{name} {value}"),
                        None => value,
                    };
                    more = spelling(&said);
                }
                if more.is_empty() {
                    out.push_str(literal);
                } else {
                    let escaped = serde_json::to_string(&more).unwrap_or_default();
                    out.push_str(&literal[..literal.len() - 1]);
                    out.push_str(&escaped[1..escaped.len() - 1]);
                    out.push('"');
                }
            }
            '{' => {
                let payload = payload_here(&frames);
                let key = match frames.last() {
                    Some(Frame::Array { key, .. }) => key.clone(),
                    _ => None,
                };
                frames.push(Frame::Object {
                    key,
                    expecting_key: true,
                    name: None,
                    payload,
                });
                out.push(c);
            }
            '[' => {
                let payload = payload_here(&frames);
                let key = match frames.last() {
                    Some(Frame::Object { key, .. }) | Some(Frame::Array { key, .. }) => key.clone(),
                    None => None,
                };
                frames.push(Frame::Array { key, payload });
                out.push(c);
            }
            '}' | ']' => {
                frames.pop();
                out.push(c);
            }
            ':' => {
                if let Some(Frame::Object { expecting_key, .. }) = frames.last_mut() {
                    *expecting_key = false;
                }
                out.push(c);
            }
            ',' => {
                if let Some(Frame::Object { expecting_key, .. }) = frames.last_mut() {
                    *expecting_key = true;
                }
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}
