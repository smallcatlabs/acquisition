//! `acq diagnose` (#79): the layer's diagnosis as text, each section the
//! words its own verb prints (`daemon status`, `store status`, the
//! rails), the two tails last, the whole redacted by the layer's one
//! function before it is printed. JSON is the document whole (C53: text
//! one screen before the tails, JSON the contract). Exits 0 whatever it
//! found: a section that could not be read says why (C146).

use std::fmt::Write as _;

use acquisition_verbs::Tails;
use acquisition_verbs::diagnose::{Diagnosis, JournalLine, Rails, Tail};
use anyhow::Result;

use crate::Ctx;

pub async fn run(cx: &Ctx, lines: Option<usize>, json: bool) -> Result<()> {
    let tails = lines.map_or(Tails::DEFAULT, Tails::both);
    let diagnosis = cx.frontend.diagnose(tails).await;
    if json {
        println!("{}", serde_json::to_string_pretty(&diagnosis.document())?);
        return Ok(());
    }
    let (body, made) = diagnosis.redact_text(&text(&diagnosis));
    println!(
        "acq diagnose: paste all of it into the issue, between two lines of three backticks.\n\
         Replaced before printing: {} of account names and uuids, {} of this machine's paths, \
         {} of token-shaped strings; character, league and tab names are kept.",
        counted(made.accounts, "occurrence"),
        counted(made.paths, "occurrence"),
        counted(made.tokens, "occurrence"),
    );
    print!("{body}");
    Ok(())
}

fn counted(n: usize, what: &str) -> String {
    format!("{n} {what}{}", if n == 1 { "" } else { "s" })
}

/// The diagnosis as text, unredacted: the caller redacts it whole.
fn text(d: &Diagnosis) -> String {
    let mut out = String::new();
    let word = |key: &str| d.version[key].as_str().unwrap_or("?").to_string();
    out.push_str("\n== build\n");
    let _ = writeln!(
        out,
        "acq {} (contract {}), provider {}, {}",
        word("version"),
        word("contract"),
        word("provider"),
        word("platform")
    );
    if let Some(why) = d.version["acqd_absent"].as_str() {
        let _ = writeln!(out, "acqd: none beside acq — {why}");
    }

    out.push_str("\n== daemon\n");
    match &d.daemon {
        Ok(report) => out.push_str(&crate::daemon_text(report)),
        Err(why) => unread(&mut out, why),
    }

    out.push_str("\n== store\n");
    match &d.store {
        Ok(rows) if rows.is_empty() => {
            out.push_str("no account has logged in in this world (`acq accounts` lists none)\n")
        }
        Ok(rows) => {
            for row in rows {
                match &row.status {
                    Ok(status) => out.push_str(&crate::store_cmd::status_text(status)),
                    Err(why) => {
                        let _ = writeln!(out, "{}: not read: {why}", row.username);
                    }
                }
            }
        }
        Err(why) => unread(&mut out, why),
    }

    out.push_str("\n== rails\n");
    match &d.rails {
        Ok(Rails::Daemon(_)) => {
            out.push_str("the running daemon's, printed under daemon above\n");
        }
        Ok(Rails::Disk { path, state }) => {
            let _ = writeln!(
                out,
                "no daemon of this client's runs; the world's rails file ({}) holds",
                path.display()
            );
            match state {
                Some(state) if !state.holds_nothing() => {
                    if let Some(cause) = &state.blocked {
                        let _ = writeln!(
                            out,
                            "blocked: {cause} — every daemon holds until `acq daemon reset-tripwire`"
                        );
                    }
                    if let Some(cause) = &state.tripped {
                        let _ = writeln!(
                            out,
                            "tripped: {cause} — a daemon started with the tripwire holds until `acq daemon reset-tripwire`"
                        );
                    }
                    let mut marks: Vec<_> = state.refresh_failed_by_account.iter().collect();
                    marks.sort();
                    for (account, cause) in marks {
                        let _ = writeln!(
                            out,
                            "refresh disabled for {account}: {cause} — a login with `acq auth` clears it"
                        );
                    }
                    if !state.unknown.is_empty() {
                        let names: Vec<&str> = state.unknown.keys().map(String::as_str).collect();
                        let _ = writeln!(
                            out,
                            "kept for a later build, which wrote them: {}",
                            names.join(", ")
                        );
                    }
                }
                _ => {
                    out.push_str("nothing: no block mark, no trip, no account's refresh disabled\n")
                }
            }
        }
        Err(why) => unread(&mut out, why),
    }

    out.push_str("\n== journal");
    match &d.journal {
        Ok(tail) => {
            heading(&mut out, tail, "sends");
            for line in &tail.lines {
                match line {
                    JournalLine::Send(send) => {
                        let _ = writeln!(out, "{send}");
                    }
                    JournalLine::Unread(why) => {
                        let _ = writeln!(out, "(not read: {why})");
                    }
                }
            }
        }
        Err(why) => {
            out.push('\n');
            unread(&mut out, why);
        }
    }

    out.push_str("\n== log");
    match &d.log {
        Ok(tail) => {
            heading(&mut out, tail, "lines");
            for line in &tail.lines {
                let _ = writeln!(out, "{line}");
            }
        }
        Err(why) => {
            out.push('\n');
            unread(&mut out, why);
        }
    }
    out
}

fn unread(out: &mut String, why: &str) {
    let _ = writeln!(out, "not read: {why}");
}

/// A tail's heading: how many lines it carries from where, and whether
/// the file holds earlier ones (C146: the count says what it counts).
fn heading<T>(out: &mut String, tail: &Tail<T>, empty: &str) {
    // Whose file it is (r4's review, finding 4): a stopped daemon's
    // journal is the one this shell's ACQ_JOURNAL names, or the world's.
    let whose = match tail.from {
        "daemon" => "the running daemon's",
        "override" => "the one ACQ_JOURNAL names in this shell",
        _ => "the world's",
    };
    let path = format!("{} ({whose})", tail.path.display());
    if tail.lines.is_empty() {
        let _ = writeln!(
            out,
            ": {path} holds no {empty}{}",
            if tail.earlier_left_out {
                " in the lines asked for (`--lines N` carries more)"
            } else {
                ""
            }
        );
        return;
    }
    let _ = writeln!(
        out,
        ": the last {} of {path}, {}",
        counted(tail.lines.len(), "line"),
        if tail.earlier_left_out {
            "earlier lines left out (`--lines N` carries more)"
        } else {
            "the whole file"
        }
    );
}
