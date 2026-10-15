//! The trade boundary (C99, C113; the contract detail K6 in
//! `decisions/search.md`): a site search URL, its id or its query JSON,
//! decoded with no network. The brief is `search/TRADE-PLAN.md`.
//!
//! # As built (step 1 of the brief: decode)
//!
//! - **A URL is a request** (K6). `/trade/search/<league>/<id>` names the
//!   pc realm; `/trade/search/(xbox|sony)/<league>/<id>` that console. The
//!   realm is the scope and the league a term — both leave here as words,
//!   for the translation to place (step 4). The path is what stands
//!   between the host and the first `?` or `#`, read segment by segment:
//!   an empty segment inside it is refused, never skipped, so a doubled
//!   slash can never move a realm into the league's place (the first
//!   outside review of the step); one trailing slash is nothing; the
//!   site's `/live` suffix is its live view of the same search and is
//!   dropped. The league is percent-decoded, and an escape that is no two
//!   hex digits, or a decoding that is no UTF-8, is refused rather than
//!   altered — a league is a term, and a changed one would answer nothing
//!   with no word why. A `/trade2/` URL, or the `poe2` realm, is refused
//!   by name before anything decodes: inbound is PoE1 only (note 36,
//!   ruling 5).
//! - **The id is the query object**, gzipped and base64url-encoded without
//!   padding (S56; the eight captured ids, `tests/trade_decode.rs`). A
//!   bare id decodes the same way. The id's text is bounded before it is
//!   decoded ([`MAX_ID`]) and what it inflates to after ([`MAX_INFLATED`]),
//!   so a pasted id can fill neither the decoder nor the inflater; the
//!   bytes must be exactly one gzip member with nothing after it, so a
//!   second member or trailing junk is refused, never dropped. A
//!   malformed id — not base64, not gzip, not one member, not JSON, not an
//!   object — is a structured error of kind `trade` (C47).
//! - **The JSON is what the site posts**: the `{query, sort}` envelope, or
//!   the query object alone. A sort is carried out for the report and
//!   never applied (K6). Neither an id nor JSON fills a scope: a
//!   [`Decoded`] from either carries no realm and no league. The JSON
//!   text is the caller's and is not bounded here.
//! - Nothing here reads the vocabulary or writes a word of the language:
//!   that is the translation (step 4), which consumes a [`Decoded`]. The
//!   vocabulary it will read is the stat table, [`stats`] (step 2), and
//!   what each clause becomes the catalogue, [`catalogue`] (its loader
//!   step 3's).

use std::io::Read as _;

use base64::Engine as _;
use serde::Serialize;
use serde_json::Value as Json;

use crate::error::{ErrorKind, LanguageError};

pub mod catalogue;
pub mod stats;

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

/// The longest id decoded: 64 KiB of text, where the longest captured id
/// is 368 characters and the site refuses a query of 220 ids as too
/// complex (the site-sitting skill's traps).
pub const MAX_ID: usize = 64 << 10;

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

/// The path of a URL: what stands between the host and the first `?` or
/// `#`. The query and fragment are cut first, from the whole input, so a
/// `/` inside them is never taken for the path — with a scheme or
/// without, the one rule.
fn path_of(input: &str) -> &str {
    let before_query = input.split(['?', '#']).next().unwrap_or("");
    let after_scheme = match before_query.find("://") {
        Some(at) => &before_query[at + 3..],
        None => before_query,
    };
    after_scheme
        .find('/')
        .map_or("", |slash| &after_scheme[slash..])
}

fn decode_url(input: &str) -> Result<Decoded, LanguageError> {
    const SHAPE: &str = "not a trade search URL: `/trade/search/<league>/<id>` or `/trade/search/<realm>/<league>/<id>`";
    let path = path_of(input);
    let Some(path) = path.strip_prefix('/') else {
        return Err(trade(SHAPE));
    };
    // one trailing slash is nothing; an empty segment anywhere else is refused
    let path = path.strip_suffix('/').unwrap_or(path);
    let mut segments: Vec<&str> = path.split('/').collect();
    if segments.iter().any(|s| s.is_empty()) {
        return Err(trade(
            "not a trade search URL: an empty path segment (a doubled slash)",
        ));
    }
    if segments.last() == Some(&"live") {
        segments.pop();
    }
    match segments.first().copied() {
        Some("trade") => {}
        Some("trade2") => return Err(poe2("a `/trade2/` URL is the PoE2 site")),
        _ => return Err(trade(SHAPE)),
    }
    if segments.get(1).copied() != Some("search") {
        return Err(trade(SHAPE));
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
        _ => return Err(trade(SHAPE)),
    };
    let league = percent_decoded(league)?;
    let query = inflate(id)?;
    Ok(Decoded {
        source: Source::Url,
        realm: Some(realm.to_string()),
        league: Some(league),
        query,
        sort: None,
    })
}

/// A search id to its query object: base64url without padding, exactly
/// one gzip member, JSON; each bound checked before the step it guards.
fn inflate(id: &str) -> Result<Json, LanguageError> {
    let id = id.trim_end_matches('=');
    if id.len() > MAX_ID {
        return Err(trade(format!(
            "not a search id: {} characters, longer than {MAX_ID}",
            id.len()
        )));
    }
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(id)
        .map_err(|_| trade("not a search id: not base64url"))?;
    if bytes.len() < 2 || bytes[0] != 0x1f || bytes[1] != 0x8b {
        return Err(trade("not a search id: not gzip"));
    }
    let mut inflated = Vec::new();
    // the bufread decoder reads one member and leaves the slice after it
    let mut decoder = flate2::bufread::GzDecoder::new(bytes.as_slice());
    (&mut decoder)
        .take(MAX_INFLATED as u64 + 1)
        .read_to_end(&mut inflated)
        .map_err(|e| trade(format!("not a search id: the gzip does not inflate: {e}")))?;
    if inflated.len() > MAX_INFLATED {
        return Err(trade(format!(
            "not a search id: it inflates past {MAX_INFLATED} bytes"
        )));
    }
    let after = decoder.into_inner();
    if !after.is_empty() {
        return Err(trade(format!(
            "not a search id: {} bytes follow the gzip member",
            after.len()
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

/// `%20` and its kin, as a URL spells a league. An escape that is no two
/// hex digits, or bytes that are no UTF-8, is refused: the league is a
/// term, and an altered one would answer nothing with no word why.
fn percent_decoded(s: &str) -> Result<String, LanguageError> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            // two hex digits and nothing else: a number parser would take a sign
            let byte = s
                .get(i + 1..i + 3)
                .filter(|hex| hex.bytes().all(|b| b.is_ascii_hexdigit()))
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
                .ok_or_else(|| {
                    trade(format!(
                        "not a trade search URL: `{s}` has a `%` that is no escape"
                    ))
                })?;
            out.push(byte);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).map_err(|_| {
        trade(format!(
            "not a trade search URL: `{s}` does not decode to UTF-8"
        ))
    })
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
