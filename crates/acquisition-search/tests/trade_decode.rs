//! The trade boundary's decoder (C113, K6; `trade.rs`), step 1 of
//! `search/TRADE-PLAN.md`: a site search URL, its id or its query JSON to
//! one decoded query, with no network. The eight searches the owner ran
//! on 2026-09-13 are the fixture (`trade-decode.toml`): each id the site
//! returned decodes to exactly the query object the owner posted (S56),
//! which is everything offline rests on. Around them, the URL forms K6
//! names, each malformed input as a structured error of its own kind
//! (C47), and a PoE2 URL refused by name.

use std::io::Write as _;

use acquisition_search::trade::{Decoded, Source, decode, decode_json};
use acquisition_search::{ErrorKind, LanguageError};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct Fixture {
    search: Vec<Search>,
}

#[derive(Deserialize)]
struct Search {
    name: String,
    id: String,
    query: String,
}

fn fixture() -> Vec<Search> {
    let text = include_str!("trade-decode.toml");
    toml::from_str::<Fixture>(text).unwrap().search
}

fn refused(input: &str) -> LanguageError {
    match decode(input) {
        Ok(decoded) => panic!("`{input}` decoded: {decoded:?}"),
        Err(e) => e,
    }
}

/// S56, in Rust: every captured id is its request's query object, byte
/// for byte once both are values; an id fills no scope.
#[test]
fn c113_every_captured_id_decodes_to_exactly_its_request() {
    let searches = fixture();
    assert_eq!(searches.len(), 8, "the owner's eight searches");
    for s in &searches {
        let expected: Value = serde_json::from_str(&s.query).unwrap();
        let decoded = decode(&s.id).unwrap_or_else(|e| panic!("{}: {e}", s.name));
        assert_eq!(decoded.query, expected, "{}", s.name);
        assert_eq!(decoded.source, Source::Id, "{}", s.name);
        assert_eq!(decoded.realm, None, "{}: an id names no realm", s.name);
        assert_eq!(decoded.league, None, "{}: an id names no league", s.name);
        assert_eq!(decoded.sort, None, "{}: an id carries no sort", s.name);
        // padding the site never writes is tolerated
        let padded = format!("{}{}", s.id, "=".repeat((4 - s.id.len() % 4) % 4));
        assert_eq!(
            decode(&padded).unwrap().query,
            expected,
            "{} padded",
            s.name
        );
    }
}

/// K6's two URL forms, the site's `/live` view, a query string, a
/// percent-encoded league, a bare path.
#[test]
fn c113_a_url_is_a_request_with_its_realm_and_league() {
    let q1 = &fixture()[0];
    let expected: Value = serde_json::from_str(&q1.query).unwrap();
    let cases: [(&str, &str, &str); 7] = [
        (
            "https://www.pathofexile.com/trade/search/Standard/{id}",
            "pc",
            "Standard",
        ),
        (
            "https://www.pathofexile.com/trade/search/Standard/{id}/live",
            "pc",
            "Standard",
        ),
        (
            "https://www.pathofexile.com/trade/search/Standard/{id}?x=1#top",
            "pc",
            "Standard",
        ),
        (
            "http://www.pathofexile.com/trade/search/Hardcore%20Standard/{id}",
            "pc",
            "Hardcore Standard",
        ),
        (
            "https://www.pathofexile.com/trade/search/xbox/Standard/{id}",
            "xbox",
            "Standard",
        ),
        (
            "https://www.pathofexile.com/trade/search/sony/Hardcore/{id}/live",
            "sony",
            "Hardcore",
        ),
        ("/trade/search/Standard/{id}", "pc", "Standard"),
    ];
    for (form, realm, league) in cases {
        let url = form.replace("{id}", &q1.id);
        let decoded: Decoded = decode(&url).unwrap_or_else(|e| panic!("{url}: {e}"));
        assert_eq!(decoded.source, Source::Url, "{url}");
        assert_eq!(decoded.realm.as_deref(), Some(realm), "{url}");
        assert_eq!(decoded.league.as_deref(), Some(league), "{url}");
        assert_eq!(decoded.query, expected, "{url}");
        assert_eq!(decoded.sort, None, "{url}: a URL carries no sort");
    }
}

/// Inbound is PoE1 only: a `/trade2/` URL and the `poe2` realm are refused
/// by name, before the id is looked at.
#[test]
fn c113_a_poe2_url_is_refused_by_name() {
    let q1 = &fixture()[0];
    for url in [
        format!(
            "https://www.pathofexile.com/trade2/search/poe2/Standard/{}",
            q1.id
        ),
        format!(
            "https://www.pathofexile.com/trade/search/poe2/Standard/{}",
            q1.id
        ),
        "https://www.pathofexile.com/trade2/search/poe2/Standard/not-an-id".to_string(),
    ] {
        let e = refused(&url);
        assert_eq!(e.kind, ErrorKind::Trade, "{url}");
        assert!(e.message.contains("PoE1 only"), "{url}: {}", e.message);
    }
}

/// Every malformed input is a structured error of the one kind, with a
/// message that says what was not right, never a panic.
#[test]
fn c47_every_malformed_input_is_a_structured_error() {
    let q1 = &fixture()[0];
    // a gzip of text that is no JSON, of a JSON array, and one past the bound
    let gz = |bytes: &[u8]| -> String {
        let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        enc.write_all(bytes).unwrap();
        base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            enc.finish().unwrap(),
        )
    };
    let not_json = gz(b"not json");
    let an_array = gz(b"[1, 2]");
    let a_string = gz(b"\"query\"");
    let bomb = gz(&vec![b' '; acquisition_search::trade::MAX_INFLATED + 1]);
    let not_gzip = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        b"{\"status\":{\"option\":\"any\"}}",
    );
    let cases: Vec<(String, &str)> = vec![
        ("not-an-id!".into(), "base64url"),
        (not_gzip, "not gzip"),
        (not_json, "not JSON"),
        (an_array, "an array"),
        (a_string, "a string"),
        (bomb, "inflates past"),
        (
            "https://www.pathofexile.com/trade/search/Standard".into(),
            "/trade/search/",
        ),
        (
            "https://www.pathofexile.com/trade/search/Standard/a/b/c".into(),
            "/trade/search/",
        ),
        (
            format!(
                "https://www.pathofexile.com/trade/search/wii/Standard/{}",
                q1.id
            ),
            "no realm",
        ),
        (
            format!(
                "https://www.pathofexile.com/trade/exchange/Standard/{}",
                q1.id
            ),
            "/trade/search/",
        ),
        (
            "https://www.pathofexile.com/trade/search/Standard/not-an-id".into(),
            "base64url",
        ),
        ("{not json".into(), "does not parse"),
        ("{\"query\": 3}".into(), "not an object"),
    ];
    for (input, says) in cases {
        let e = refused(&input);
        assert_eq!(e.kind, ErrorKind::Trade, "{input}: {}", e.message);
        assert!(
            e.message.contains(says),
            "`{input}` → {} (expected `{says}`)",
            e.message
        );
    }
    // the JSON forms that are no object
    for value in [json!([1]), json!("q"), json!(null)] {
        let e = decode_json(&value).unwrap_err();
        assert_eq!(e.kind, ErrorKind::Trade);
        assert!(e.message.contains("not an object"), "{}", e.message);
    }
}

/// The JSON the site posts: the envelope, whose sort is carried and never
/// applied, or the query alone; neither fills a scope.
#[test]
fn c113_the_query_json_is_the_envelope_or_the_query_alone() {
    let q4 = &fixture()[5];
    assert_eq!(q4.name, "q4");
    let query: Value = serde_json::from_str(&q4.query).unwrap();
    let envelope = json!({ "query": query, "sort": { "price": "asc" } });
    let from_envelope = decode(&envelope.to_string()).unwrap();
    assert_eq!(from_envelope.source, Source::Json);
    assert_eq!(from_envelope.query, query);
    assert_eq!(from_envelope.sort, Some(json!({ "price": "asc" })));
    assert_eq!(
        (&from_envelope.realm, &from_envelope.league),
        (&None, &None)
    );
    let bare = decode(&query.to_string()).unwrap();
    assert_eq!(bare.query, query);
    assert_eq!(bare.sort, None);
    assert_eq!(decode_json(&envelope).unwrap(), from_envelope);
    // a leading newline or space is nothing
    assert_eq!(decode(&format!("\n  {}\n", query)).unwrap().query, query);
}

/// One gzip member of `bytes`, raw.
fn gzipped(bytes: &[u8]) -> Vec<u8> {
    let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    enc.write_all(bytes).unwrap();
    enc.finish().unwrap()
}

fn id_of(raw: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, raw)
}

/// The first outside review of step 1 (Codex, 2026-10-15), four faults
/// each pinned as it was found: an empty path segment that moved the
/// realm into the league's place, a host's query string read as a path,
/// an escape altered instead of refused, a second gzip member or junk
/// after the first dropped instead of refused, and an id unbounded before
/// it was decoded.
#[test]
fn c47_the_reviews_faults_a_doubled_slash_a_bad_escape_a_second_member_an_unbounded_id() {
    let q1 = &fixture()[0];
    let expected: Value = serde_json::from_str(&q1.query).unwrap();
    // a doubled slash is refused, never skipped — the realm stays where the URL put it
    for (url, says) in [
        (
            format!("https://www.pathofexile.com/trade/search/xbox//{}", q1.id),
            "empty path segment",
        ),
        (
            format!("https://www.pathofexile.com/trade/search/poe2//{}", q1.id),
            "empty path segment",
        ),
        (
            format!(
                "https://www.pathofexile.com//trade/search/Standard/{}",
                q1.id
            ),
            "empty path segment",
        ),
        (
            format!("https://example.org?next=/trade/search/Standard/{}", q1.id),
            "/trade/search/",
        ),
        (
            format!(
                "https://www.pathofexile.com#/trade/search/Standard/{}",
                q1.id
            ),
            "/trade/search/",
        ),
        (
            format!(
                "https://www.pathofexile.com/trade/search/Bad%FFLeague/{}",
                q1.id
            ),
            "UTF-8",
        ),
        (
            format!(
                "https://www.pathofexile.com/trade/search/Bad%ZZLeague/{}",
                q1.id
            ),
            "no escape",
        ),
        (
            format!(
                "https://www.pathofexile.com/trade/search/Standard%2/{}",
                q1.id
            ),
            "no escape",
        ),
        // the second look: a query string before any path, with or without a
        // scheme, and an escape a number parser would take as signed
        (
            format!("www.pathofexile.com?next=/trade/search/Standard/{}", q1.id),
            "/trade/search/",
        ),
        (
            format!("?next=/trade/search/Standard/{}", q1.id),
            "/trade/search/",
        ),
        (
            format!(
                "https://www.pathofexile.com/trade/search/Bad%+ALeague/{}",
                q1.id
            ),
            "no escape",
        ),
        (
            format!(
                "https://www.pathofexile.com/trade/search/Bad%-ALeague/{}",
                q1.id
            ),
            "no escape",
        ),
    ] {
        let e = refused(&url);
        assert_eq!(e.kind, ErrorKind::Trade, "{url}");
        assert!(
            e.message.contains(says),
            "`{url}` → {} (expected `{says}`)",
            e.message
        );
    }
    // one trailing slash is nothing; a league in any script decodes whole
    let trailing = format!(
        "https://www.pathofexile.com/trade/search/Standard/{}/",
        q1.id
    );
    let d = decode(&trailing).unwrap();
    assert_eq!(
        (d.realm.as_deref(), d.league.as_deref()),
        (Some("pc"), Some("Standard"))
    );
    assert_eq!(d.query, expected);
    let accented = format!(
        "https://www.pathofexile.com/trade/search/Stand%C3%A9rd/{}",
        q1.id
    );
    assert_eq!(
        decode(&accented).unwrap().league.as_deref(),
        Some("Standérd")
    );
    let raw = format!("/trade/search/Standé/{}", q1.id);
    assert_eq!(decode(&raw).unwrap().league.as_deref(), Some("Standé"));
    // a second member, or junk, after the one member is refused, never dropped
    let one = gzipped(b"{}");
    assert_eq!(decode(&id_of(&one)).unwrap().query, json!({}));
    let mut with_junk = one.clone();
    with_junk.extend_from_slice(b"junk");
    let mut two_members = one.clone();
    two_members.extend_from_slice(&gzipped(br#"{"status":{"option":"any"}}"#));
    for (raw, says) in [(with_junk, "4 bytes follow"), (two_members, "bytes follow")] {
        let e = refused(&id_of(&raw));
        assert_eq!(e.kind, ErrorKind::Trade);
        assert!(
            e.message.contains(says),
            "{} (expected `{says}`)",
            e.message
        );
    }
    // the id's text is bounded before it is decoded: this one never reaches base64
    let long = "A".repeat(acquisition_search::trade::MAX_ID + 1);
    let e = refused(&long);
    assert!(e.message.contains("longer than"), "{}", e.message);
    let longest = fixture().iter().map(|s| s.id.len()).max().unwrap();
    assert!(longest <= 368 && 368 < acquisition_search::trade::MAX_ID);
}
