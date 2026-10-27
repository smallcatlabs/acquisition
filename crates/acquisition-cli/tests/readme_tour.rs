//! The README's tour is a hand-written copy of the CLI's shape, and a
//! copy is honest only while something checks it (P5: structure over
//! discipline). Every `acq …` line in the README's ```sh blocks — the
//! tour, the login, the agent's daemon — must resolve under clap: the
//! verb path, and every `--flag` the line names on that subcommand (or a
//! global). A verb's semantics is its `--help`; the tour shows the
//! shape, and this test keeps the shape true — a renamed verb or a
//! dropped flag fails here, which the identifier scan in
//! `tools/docs-check.sh` cannot see.
//!
//! Driven through the binary (`acq <path> --help` must exit 0 and its
//! text must name each flag) so the crate needs no library target.
//!
//! The tour's grammar, as this test reads it: a line starting `acq`,
//! after any `NAME=value` the shell sets for it, is a verb line, `#`
//! opens a comment, ` | ` separates alternatives — an alternative that
//! starts with a flag adds flags to the same path, one that starts with
//! a word replaces the previous path's last verb — and a token starting
//! with `<`, `[`, `'`, `@` or `-` ends the verb path.
//!
//! The install lines download, so no test runs them; what a test can
//! hold of them is held here: every archive the README names is one the
//! release workflow makes, for this version, and every target it makes is
//! named.

use std::process::Command;

use crate::harness;
use harness::{acq, login_with, scratch, sole_json};

fn help(path: &[&str]) -> Result<String, String> {
    let out = Command::new(env!("CARGO_BIN_EXE_acq"))
        .args(path)
        .arg("--help")
        .env("ACQ_NO_SPAWN", "1")
        .output()
        .expect("running acq --help");
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

const README: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../README.md");

fn readme() -> String {
    std::fs::read_to_string(README).unwrap_or_else(|e| panic!("{README}: {e}"))
}

/// Every line of every ```sh block, with the `NAME=value` words the
/// shell sets for a command taken off its front.
pub(crate) fn tour_lines() -> Vec<String> {
    let mut inside = false;
    let mut lines = Vec::new();
    for line in readme().lines() {
        if !inside && line.starts_with("```sh") {
            inside = true;
        } else if inside && line.starts_with("```") {
            inside = false;
        } else if inside {
            lines.push(without_env(line).to_string());
        }
    }
    assert!(!lines.is_empty(), "no ```sh block in {README}");
    lines
}

fn without_env(line: &str) -> &str {
    let mut rest = line;
    while let Some((word, tail)) = rest.split_once(' ') {
        let assigns = word.split_once('=').is_some_and(|(name, _)| {
            !name.is_empty()
                && name
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
        });
        if !assigns {
            break;
        }
        rest = tail.trim_start();
    }
    rest
}

fn is_verb(tok: &str) -> bool {
    !tok.is_empty()
        && tok
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !tok.starts_with('-')
}

fn flags_in(text: &str) -> Vec<String> {
    let mut flags = Vec::new();
    for tok in text.split_whitespace() {
        let tok = tok.trim_start_matches('[');
        if let Some(rest) = tok.strip_prefix("--") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
                .collect();
            if !name.is_empty() {
                flags.push(name);
            }
        }
    }
    flags
}

/// `(verb path, flags)` for each alternative of one tour line.
pub(crate) fn parse(line: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let code = line.split(" #").next().unwrap_or("").trim();
    let mut out = Vec::new();
    let mut path: Vec<String> = Vec::new();
    for (i, alt) in code.split(" | ").enumerate() {
        let mut toks = alt.split_whitespace().peekable();
        if i == 0 {
            assert_eq!(toks.next(), Some("acq"), "not a verb line: {line}");
            path.clear();
        } else if toks.peek().is_some_and(|t| is_verb(t)) {
            path.pop();
        }
        while let Some(t) = toks.peek() {
            if is_verb(t) {
                path.push(t.to_string());
                toks.next();
            } else {
                break;
            }
        }
        out.push((path.clone(), flags_in(alt)));
    }
    out
}

#[test]
fn every_tour_line_resolves_under_clap() {
    let mut checked = 0;
    let mut failures = Vec::new();
    for line in tour_lines() {
        if !line.starts_with("acq ") {
            continue;
        }
        for (path, flags) in parse(&line) {
            let path_ref: Vec<&str> = path.iter().map(String::as_str).collect();
            let text = match help(&path_ref) {
                Ok(t) => t,
                Err(e) => {
                    failures.push(format!("`acq {}`: no such verb\n{e}", path.join(" ")));
                    continue;
                }
            };
            for flag in flags {
                let needle = format!("--{flag}");
                let named = text.match_indices(&needle).any(|(i, _)| {
                    text[i + needle.len()..]
                        .chars()
                        .next()
                        .is_none_or(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'))
                });
                if !named {
                    failures.push(format!(
                        "`acq {}`: `{needle}` is not in its --help\n    {line}",
                        path.join(" ")
                    ));
                }
            }
            checked += 1;
        }
    }
    assert!(
        checked >= 20,
        "the tour has only {checked} verb lines — was the block renamed?"
    );
    assert!(
        failures.is_empty(),
        "the README's tour no longer matches the CLI (a verb's semantics is its --help; the tour shows the shape):\n{}",
        failures.join("\n")
    );
}

/// The targets `release.yml` builds, from its build matrix's
/// `{ os: …, target: … }` rows.
fn release_targets() -> Vec<String> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../.github/workflows/release.yml"
    );
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let targets: Vec<String> = text
        .lines()
        .filter(|l| l.contains("os:"))
        .filter_map(|l| l.split_once("target:"))
        .map(|(_, t)| t.trim().trim_end_matches('}').trim().to_string())
        .collect();
    assert_eq!(targets.len(), 5, "release.yml's matrix: {targets:?}");
    targets
}

/// The archive `tools/release-package.sh` names for a target: the
/// version and the target, `.zip` for Windows.
fn archive(version: &str, target: &str) -> String {
    let ext = if target.contains("windows") {
        "zip"
    } else {
        "tar.gz"
    };
    format!("acquisition-{version}-{target}.{ext}")
}

#[test]
fn the_install_lines_name_the_archives_the_release_makes() {
    let version = env!("CARGO_PKG_VERSION");
    let text = readme();
    let made: Vec<String> = release_targets()
        .iter()
        .map(|t| archive(version, t))
        .collect();
    let named: Vec<&str> = text
        .match_indices("acquisition-")
        .map(|(i, _)| {
            let len = text[i..]
                .find(|c: char| !(c.is_ascii_alphanumeric() || "._-".contains(c)))
                .unwrap_or(text.len() - i);
            &text[i..i + len]
        })
        .filter(|w| w.ends_with(".tar.gz") || w.ends_with(".zip"))
        .collect();
    for name in &named {
        assert!(
            made.iter().any(|m| m == name),
            "the README names `{name}`, which the release does not make: {made:?}"
        );
    }
    for archive in &made {
        assert!(
            named.contains(&archive.as_str()),
            "the release makes `{archive}`, which the README does not name"
        );
    }
    let tag = format!("v{version}");
    let mut urls = 0;
    for (i, _) in text.match_indices("/releases/download/") {
        let rest = &text[i + "/releases/download/".len()..];
        let url = &rest[..rest.find(char::is_whitespace).unwrap_or(rest.len())];
        let (at, file) = url.split_once('/').unwrap_or((url, ""));
        assert_eq!(at, tag, "a download URL names another tag: {url}");
        assert!(
            made.iter().any(|m| m == file),
            "a download URL names no archive the release makes: {url}"
        );
        urls += 1;
    }
    assert!(urls > 0, "the README downloads nothing");
}

#[test]
fn the_grammar_reads_alternatives() {
    let parsed = parse("acq jobs [--watch] | status <id> | result <id>   # x");
    assert_eq!(parsed[0], (vec!["jobs".into()], vec!["watch".into()]));
    assert_eq!(parsed[1], (vec!["status".into()], vec![]));
    assert_eq!(parsed[2], (vec!["result".into()], vec![]));
    let parsed = parse("acq auth status | check | logout");
    assert_eq!(parsed[2].0, vec!["auth".to_string(), "logout".to_string()]);
    let parsed = parse("acq refresh --tabs a,b,c | --all [--deep]");
    assert_eq!(
        parsed[1],
        (vec!["refresh".into()], vec!["all".into(), "deep".into()])
    );
    let parsed = parse("acq policy set '<json>'|-|@FILE [--if-revision N]");
    assert_eq!(
        parsed[0],
        (
            vec!["policy".into(), "set".into()],
            vec!["if-revision".into()]
        )
    );
    let parsed = parse("acq refresh --apply[=plan.json] [--max-requests N]");
    assert_eq!(
        parsed[0].1,
        vec!["apply".to_string(), "max-requests".to_string()]
    );
    let parsed = parse("acq <verb> --help");
    assert_eq!(parsed[0], (vec![], vec!["help".into()]));
    assert_eq!(
        without_env("ACQ_IDLE_SHUTDOWN=3600 acq auth check   # x"),
        "acq auth check   # x"
    );
    assert_eq!(without_env("acq x=1"), "acq x=1");
}

/// The README's "Your agent" section, from its heading to the next.
fn agent_section() -> String {
    let text = readme();
    let start = text
        .find("\n## Your agent")
        .expect("the README has a \"Your agent\" section");
    let rest = &text[start + 1..];
    let end = rest[1..].find("\n## ").map_or(rest.len(), |i| i + 1);
    rest[..end].to_string()
}

/// The agent's daemon recipe runs as the README prints it, in the case
/// this harness can stage: no keyring, so the line for that case is the
/// login itself, and the keyring line is the same verb's `check`, which
/// the harness cannot run. With the idle knob left to its default (C3,
/// the hour) the recipe needs no knob: the section names none, and the
/// daemon the login starts reports an idle exit of 3600 s.
#[test]
fn the_agent_recipe_starts_a_daemon_that_idles_out_after_the_hour() {
    let section = agent_section();
    for line in ["acq auth check   #", "acq auth   #"] {
        assert!(
            section.lines().any(|l| l.starts_with(line)),
            "the README's agent section has no line `{line}`"
        );
    }
    assert!(
        !section.contains("ACQ_IDLE_SHUTDOWN"),
        "the agent section still names the idle knob"
    );
    let dir = scratch("recipe");
    let base = &dir.0;
    let status = sole_json(&acq(base, &["daemon", "status", "--json"]));
    assert_eq!(status["running"], serde_json::json!(false), "{status}");
    // The harness isolates every daemon at a 30 s idle exit; an empty
    // knob is misread and keeps the default, the README's hour.
    login_with(base, "Alice#1", &[("ACQ_IDLE_SHUTDOWN", "")]);
    let status = sole_json(&acq(base, &["daemon", "status", "--json"]));
    assert_eq!(
        (
            &status["running"],
            &status["provider"],
            &status["idle_shutdown_seconds"]
        ),
        (
            &serde_json::json!(true),
            &serde_json::json!("mock"),
            &serde_json::json!(3600)
        ),
        "{status}"
    );
    let auth = sole_json(&acq(base, &["auth", "status", "--json"]));
    assert!(
        auth["accounts"].as_array().is_some_and(|a| !a.is_empty()),
        "{auth}"
    );
    let stopped = sole_json(&acq(base, &["daemon", "stop", "--json"]));
    assert_eq!(stopped["stopped"], serde_json::json!(true), "{stopped}");
}
