//! C116: no id in user-facing text. The trade report's rule
//! (`the_reports_words_cite_no_record`, `acquisition-search`'s
//! `tests/trade_translate.rs`) widened to every surface, as one test.

use std::path::{Path, PathBuf};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

/// A record's id as a reader meets it: one capital letter and one to
/// three digits, a whole word — a ruling (`C81`), a claim (`N47`), a
/// sitting's or a census's row (`T11`, `Q7`), a digest's limit (`S52`),
/// a ledger's entry (`J10`, `K6`). C116 names the first five letters;
/// the surfaces held the others too, the same class, so the pattern is
/// the shape rather than a list of letters. What the surfaces hold that
/// is shaped so and is no id is one literal, [`NO_ID`]; the rest is
/// written otherwise — a map tier `Tier 16`, a quality field `armour.q20`,
/// lower case, a token `acq1.…` and an item id lower-case hex — and a
/// base the game names with one (`Enigmatic Assembly C5`) is in the class
/// table, which no surface prints whole. A sitting is the same class in
/// words (the trade seat met "for a sitting" and "no sitting has asked"
/// in the catalogue's words): the builders' name for a session with the
/// site, which no user can look up.
fn ids(text: &str) -> Vec<String> {
    let mut found: Vec<String> = text
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .filter(|w| {
            let mut chars = w.chars();
            let first = chars.next();
            let rest = chars.as_str();
            first.is_some_and(|c| c.is_ascii_uppercase())
                && (1..=3).contains(&rest.len())
                && rest.chars().all(|c| c.is_ascii_digit())
        })
        .map(str::to_string)
        .collect();
    for sitting in ["a sitting", "no sitting", "the sitting", "sittings"] {
        if text.contains(sitting) {
            found.push(sitting.to_string());
        }
    }
    found
}

/// A sitting's search as a reader meets it, `t41`, `u090`, `i06`: a
/// small letter and two or three digits, a whole word not after a dot (a
/// field's `armour.q20` is one). Swept in the printed documents alone —
/// the references and `describe` — since a literal of the binaries may be
/// data so shaped, a mock's ids (`u001`, `m001`) among them.
fn searches(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut before = ' ';
    let mut word = String::new();
    for c in text.chars().chain([' ']) {
        if c.is_ascii_alphanumeric() || c == '_' {
            if word.is_empty() && before == '.' {
                word.push('.');
            }
            word.push(c);
            continue;
        }
        let mut chars = word.chars();
        if chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && (2..=3).contains(&chars.as_str().len())
            && chars.all(|c| c.is_ascii_digit())
        {
            found.push(word.clone());
        }
        word.clear();
        before = c;
    }
    found
}

/// The literals shaped as an id that are none: `S256`, the PKCE method's
/// name the daemon sends GGG at a login, which no reader is shown.
const NO_ID: &[&str] = &["S256"];

/// The text of every string literal in a Rust source, with its line:
/// what the binaries can print — every refusal, reason, note and tool
/// word is made of these — outside comments, which are the builders',
/// and outside a `#[cfg(test)]` item, which no binary carries.
fn literals(src: &str) -> Vec<(usize, String)> {
    let b = src.as_bytes();
    let (mut i, mut line, mut depth) = (0usize, 1usize, 0usize);
    // the depth at which a test item's block closes, while inside one
    let mut skip_until: Option<usize> = None;
    let mut test_item_next = false;
    let mut out = Vec::new();
    while i < b.len() {
        let rest = &src[i..];
        if rest.starts_with("//") {
            i += rest.find('\n').unwrap_or(rest.len());
            continue;
        }
        if rest.starts_with("/*") {
            let end = rest.find("*/").map_or(rest.len(), |e| e + 2);
            line += rest[..end].matches('\n').count();
            i += end;
            continue;
        }
        if rest.starts_with("#[cfg(test)]") {
            test_item_next = true;
            i += "#[cfg(test)]".len();
            continue;
        }
        // a raw string: r"…", r#"…"#, br"…"
        let raw_at = if rest.starts_with("br") {
            Some(2)
        } else if rest.starts_with('r') {
            Some(1)
        } else {
            None
        };
        let word_before = i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
        if let Some(at) = raw_at.filter(|_| !word_before) {
            let hashes = rest[at..].bytes().take_while(|c| *c == b'#').count();
            if rest[at + hashes..].starts_with('"') {
                let open = at + hashes + 1;
                let close = format!("\"{}", "#".repeat(hashes));
                let end = rest[open..].find(&close).map_or(rest.len(), |e| open + e);
                let text = &rest[open..end];
                if skip_until.is_none() {
                    out.push((line, text.to_string()));
                }
                line += text.matches('\n').count();
                i += end + close.len();
                continue;
            }
        }
        match b[i] {
            b'"' => {
                let mut j = i + 1;
                while j < b.len() && b[j] != b'"' {
                    j += if b[j] == b'\\' { 2 } else { 1 };
                }
                let text = &src[i + 1..j.min(b.len())];
                if skip_until.is_none() {
                    out.push((line, text.to_string()));
                }
                line += text.matches('\n').count();
                i = j + 1;
            }
            b'\'' => {
                // a char literal, never a lifetime: '"', '\'', '\u{2014}'
                let tail = &rest[1..];
                let len = if tail.starts_with("\\u{") {
                    tail.find('}').map(|e| e + 1)
                } else if tail.starts_with('\\') {
                    Some(2)
                } else {
                    tail.chars().next().map(char::len_utf8)
                };
                match len.filter(|l| tail[*l..].starts_with('\'')) {
                    Some(l) => i += l + 2,
                    None => i += 1,
                }
            }
            b'{' => {
                if test_item_next {
                    test_item_next = false;
                    if skip_until.is_none() {
                        skip_until = Some(depth);
                    }
                }
                depth += 1;
                i += 1;
            }
            b'}' => {
                depth = depth.saturating_sub(1);
                if skip_until == Some(depth) {
                    skip_until = None;
                }
                i += 1;
            }
            b';' => {
                // `#[cfg(test)] mod tests;` or a test-only `use`: no block
                test_item_next = false;
                i += 1;
            }
            b'\n' => {
                line += 1;
                i += 1;
            }
            _ => i += rest.chars().next().map_or(1, char::len_utf8),
        }
    }
    out
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every surface a user reads, swept for a record's id (C116): the help
/// of every verb (`CLI-REFERENCE.md`, which `cli_reference.rs` keeps
/// equal to the binary), the MCP server's instructions and tool listing
/// (`MCP-REFERENCE.md`, kept so by `mcp_reference.rs`), `describe` whole
/// with the trade catalogue and the category rows — the reference
/// tables' own words, a field's definition among them — and every string
/// literal of every crate's `src`, which is where every refusal, reason,
/// note and `says` is made: a refusal's words are composed at the moment
/// it is said, so the literal it is made of is the one place a sweep
/// reaches them all without a case for each. Code doc comments and
/// commit messages keep their ids; a comment is no literal.
#[test]
fn c116_no_user_facing_text_cites_a_record_id() {
    let root = Path::new(ROOT);
    let mut met: Vec<String> = Vec::new();
    for reference in ["CLI-REFERENCE.md", "MCP-REFERENCE.md"] {
        let text = std::fs::read_to_string(root.join(reference)).unwrap();
        for (n, line) in text.lines().enumerate() {
            let mut found = ids(line);
            found.extend(searches(line));
            if !found.is_empty() {
                met.push(format!("{reference}:{}: {found:?}", n + 1));
            }
        }
    }
    let names = |n: &[&str]| n.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    for asked in [names(&[]), names(&["trade", "category"])] {
        let described =
            serde_json::to_string(&acquisition_search::describe::describe(&asked).unwrap())
                .unwrap();
        let mut found = ids(&described);
        found.extend(searches(&described));
        if !found.is_empty() {
            met.push(format!("describe {asked:?}: {found:?}"));
        }
    }
    let mut files = Vec::new();
    sources(&root.join("crates"), &mut files);
    files.retain(|f| {
        f.components().any(|c| c.as_os_str() == "src")
            && !f.components().any(|c| c.as_os_str() == "tests")
    });
    files.sort();
    assert!(files.len() > 50, "the walk found {} sources", files.len());
    for file in &files {
        let src = std::fs::read_to_string(file).unwrap();
        for (line, text) in literals(&src) {
            if NO_ID.contains(&text.as_str()) {
                continue;
            }
            let found = ids(&text);
            if !found.is_empty() {
                let at = file.strip_prefix(root).unwrap_or(file).display();
                met.push(format!("{at}:{line}: {found:?}"));
            }
        }
    }
    assert!(met.is_empty(), "{} ids met:\n{}", met.len(), met.join("\n"));
}

/// The sweep's own reader: a literal is read whole, a comment is not, and
/// a test module's literals are no binary's.
#[test]
fn c116_the_sweep_reads_literals_and_leaves_comments_and_tests() {
    let src = r##"
// a comment cites C81
fn f() -> &'static str { "the rule (C81)" }
const R: &str = r#"raw "T11" here"#;
const C: char = '"';
#[cfg(test)]
mod tests { fn g() { "C99 in a test"; } }
fn h() { "after the tests, N47" }
"##;
    let texts: Vec<String> = literals(src).into_iter().map(|(_, t)| t).collect();
    assert_eq!(
        texts,
        ["the rule (C81)", "raw \"T11\" here", "after the tests, N47"]
    );
    assert_eq!(ids("the rule (C81)"), ["C81"]);
    assert_eq!(
        ids("armour.q20, Tier 16, acq1.abc, CLI, a sitting"),
        ["a sitting"]
    );
    assert_eq!(
        searches("as the site reads it (u090), armour.q20, v1, t41"),
        ["u090", "t41"]
    );
}
