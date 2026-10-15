//! The trade boundary (C99, C113; the contract detail K6 in
//! `decisions/search.md`): a site search URL, its id or its query JSON,
//! decoded with no network. The brief is `search/TRADE-PLAN.md`.
//!
//! # As built (step 1 of the brief: decode)
//!
//! - **A URL is a request** (K6). `/trade/search/<league>/<id>` names the
//!   pc realm; `/trade/search/(xbox|sony)/<league>/<id>` that console. The
//!   realm is the scope and the league a term — both leave here as words,
//!   for the translation to place (step 4). The site's `/live` suffix is
//!   its live view of the same search and is dropped; a query string or
//!   fragment is dropped; the league is percent-decoded. A `/trade2/` URL,
//!   or the `poe2` realm, is refused by name before anything decodes:
//!   inbound is PoE1 only (note 36, ruling 5).
//! - **The id is the query object**, gzipped and base64url-encoded without
//!   padding (S56; the eight captured ids, `tests/trade_decode.rs`). A
//!   bare id decodes the same way. A malformed id — not base64, not gzip,
//!   not JSON, not an object — is a structured error of kind `trade`
//!   (C47), and the inflated size is bounded ([`MAX_INFLATED`]), so a
//!   pasted id can never fill memory.
//! - **The JSON is what the site posts**: the `{query, sort}` envelope, or
//!   the query object alone. A sort is carried out for the report and
//!   never applied (K6). Neither an id nor JSON fills a scope: a
//!   [`Decoded`] from either carries no realm and no league.
//! - Nothing here reads the vocabulary or writes a word of the language:
//!   that is the translation (step 4), which consumes a [`Decoded`].

use std::io::Read as _;

use base64::Engine as _;
use serde::Serialize;
use serde_json::Value as Json;

use crate::error::{ErrorKind, LanguageError};

/// What the input was, for the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Url,
    Id,
    Json,
}

/// A site search, decoded: its query object as posted, and what the
/// input carried beside it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Decoded {
    pub source: Source,
    /// The realm the URL names — `pc`, `xbox` or `sony` — and none for an
    /// id or JSON, which fill no scope (K6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    /// The league the URL names, percent-decoded; none for an id or JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,
    /// The site's query object, as posted: an object, read no further here.
    pub query: Json,
    /// The envelope's sort, when the JSON carried one; a URL carries none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Json>,
}

/// The most an id may inflate to: a megabyte, where the longest captured
/// query inflates to under a kilobyte.
pub const MAX_INFLATED: usize = 1 << 20;

/// The site's realm words that take a path segment; pc takes none.
const CONSOLE_REALMS: [&str; 2] = ["xbox", "sony"];

/// A site search URL, a bare id, or the query JSON as text, to one
/// [`Decoded`]. Which it is read as: text that opens with `{` is JSON,
/// text that holds a `/trade` path is a URL, anything else an id.
pub fn decode(input: &str) -> Result<Decoded, LanguageError> {
    let input = input.trim();
    if input.starts_with('{') {
        let value: Json = serde_json::from_str(input)
            .map_err(|e| trade(format!("the query JSON does not parse: {e}")))?;
        return decode_json(&value);
    }
    if input.contains("/trade") {
        return decode_url(input);
    }
    let query = inflate(input)?;
    Ok(Decoded {
        source: Source::Id,
        realm: None,
        league: None,
        query,
        sort: None,
    })
}

/// The query JSON as a value: the `{query, sort}` envelope, or the query
/// object alone. An envelope is an object whose `query` is an object.
pub fn decode_json(value: &Json) -> Result<Decoded, LanguageError> {
    let Json::Object(object) = value else {
        return Err(trade(format!(
            "the query JSON is {}, not an object",
            json_kind(value)
        )));
    };
    let (query, sort) = match object.get("query") {
        Some(Json::Object(inner)) => (Json::Object(inner.clone()), object.get("sort").cloned()),
        Some(other) => {
            return Err(trade(format!(
                "the envelope's `query` is {}, not an object",
                json_kind(other)
            )));
        }
        None => (value.clone(), None),
    };
    Ok(Decoded {
        source: Source::Json,
        realm: None,
        league: None,
        query,
        sort,
    })
}

fn decode_url(input: &str) -> Result<Decoded, LanguageError> {
    // the path: after the host when a scheme is given, else from the first `/`
    let path = match input.find("://") {
        Some(at) => {
            let rest = &input[at + 3..];
            match rest.find('/') {
                Some(slash) => &rest[slash..],
                None => "",
            }
        }
        None => match input.find('/') {
            Some(slash) => &input[slash..],
            None => "",
        },
    };
    let path = path.split(['?', '#']).next().unwrap_or("");
    let mut segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.last() == Some(&"live") {
        segments.pop();
    }
    match segments.first().copied() {
        Some("trade") => {}
        Some("trade2") => return Err(poe2("a `/trade2/` URL is the PoE2 site")),
        _ => return Err(trade("not a trade search URL: no `/trade/search/` path")),
    }
    if segments.get(1).copied() != Some("search") {
        return Err(trade("not a trade search URL: no `/trade/search/` path"));
    }
    let (realm, league, id) = match segments[2..] {
        [league, id] => ("pc", league, id),
        [realm, league, id] => {
            if realm.eq_ignore_ascii_case("poe2") {
                return Err(poe2("the `poe2` realm"));
            }
            match CONSOLE_REALMS
                .iter()
                .find(|r| r.eq_ignore_ascii_case(realm))
            {
                Some(realm) => (*realm, league, id),
                None => {
                    return Err(trade(format!(
                        "`{realm}` is no realm of a trade search URL: none for pc, or xbox, sony"
                    )));
                }
            }
        }
        _ => {
            return Err(trade(
                "not a trade search URL: `/trade/search/<league>/<id>` or `/trade/search/<realm>/<league>/<id>`",
            ));
        }
    };
    let query = inflate(id)?;
    Ok(Decoded {
        source: Source::Url,
        realm: Some(realm.to_string()),
        league: Some(percent_decoded(league)),
        query,
        sort: None,
    })
}

/// A search id to its query object: base64url without padding, gzip, JSON.
fn inflate(id: &str) -> Result<Json, LanguageError> {
    let id = id.trim_end_matches('=');
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(id)
        .map_err(|_| trade("not a search id: not base64url"))?;
    if bytes.len() < 2 || bytes[0] != 0x1f || bytes[1] != 0x8b {
        return Err(trade("not a search id: not gzip"));
    }
    let mut inflated = Vec::new();
    flate2::read::GzDecoder::new(bytes.as_slice())
        .take(MAX_INFLATED as u64 + 1)
        .read_to_end(&mut inflated)
        .map_err(|e| trade(format!("not a search id: the gzip does not inflate: {e}")))?;
    if inflated.len() > MAX_INFLATED {
        return Err(trade(format!(
            "not a search id: it inflates past {MAX_INFLATED} bytes"
        )));
    }
    let value: Json = serde_json::from_slice(&inflated).map_err(|e| {
        trade(format!(
            "not a search id: the inflated text is not JSON: {e}"
        ))
    })?;
    match value {
        Json::Object(_) => Ok(value),
        other => Err(trade(format!(
            "not a search id: it inflates to {}, not a query object",
            json_kind(&other)
        ))),
    }
}

/// `%20` and its kin, as a URL spells a league; a `+` is a plus, as the
/// site spells none.
fn percent_decoded(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Some(byte) = s
                .get(i + 1..i + 3)
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
        {
            out.push(byte);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn trade(message: impl Into<String>) -> LanguageError {
    LanguageError::new(ErrorKind::Trade, message)
}

fn poe2(what: &str) -> LanguageError {
    trade(format!(
        "{what}: the import is inbound for PoE1 only; PoE2's trade data is parked (decisions/search.md)"
    ))
}

fn json_kind(value: &Json) -> &'static str {
    match value {
        Json::Null => "null",
        Json::Bool(_) => "a boolean",
        Json::Number(_) => "a number",
        Json::String(_) => "a string",
        Json::Array(_) => "an array",
        Json::Object(_) => "an object",
    }
}
