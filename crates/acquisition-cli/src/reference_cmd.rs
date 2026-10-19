//! Reference data's CLI surface: `acq reference currency` enumerates the
//! currency table the binary ships (C68: reference data is enumerable
//! through every surface and cited by version wherever used) and resolves
//! one word. Reads nothing but the binary — no store, no daemon, no
//! network — so it answers in every mode, logged in or not.
//!
//! Under C53: the default text is the table (a reference lookup is the
//! enumeration; the list *is* the answer, so the ten-or-fewer grouping
//! rule does not apply), one row per line with the words a parser
//! accepts and a retired mark; `--expand` adds each row's evidence and
//! the table's sources; `--json` is the table itself, whole, evidence
//! included. A word that does not resolve is a failure that names the
//! word and the version and says what to type next.

use acquisition_plan::currency::{Currency, CurrencyTable};
use acquisition_plan::pricing::{currency_document, currency_row, currency_table};
use anyhow::Result;

/// `acq reference currency [WORD] [--expand]`. `--json` is the document
/// the plan crate's pricing verbs make, the one the MCP's
/// `reference_currency` returns (C46).
pub fn currency(word: Option<&str>, expand: bool, json: bool) -> Result<()> {
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&currency_document(word)?)?
        );
        return Ok(());
    }
    match word {
        Some(word) => resolve(word),
        None => enumerate(currency_table()?, expand),
    }
}

fn enumerate(t: &CurrencyTable, expand: bool) -> Result<()> {
    let active = t.active().count();
    let retired = t.rows().len() - active;
    let aliased = t.rows().iter().filter(|r| !r.aliases.is_empty()).count();
    println!(
        "currency table v{} ({}): {active} currencies the game writes, {retired} retired, {aliased} with aliases",
        t.version(),
        t.status()
    );
    for row in t.rows() {
        print_row(row, expand);
    }
    if expand {
        println!("sources:");
        for s in t.sources() {
            println!("  {s}");
        }
    }
    println!("a price cites the word at left; `acq reference currency <word>` resolves one");
    Ok(())
}

fn resolve(word: &str) -> Result<()> {
    let (t, row) = currency_row(word)?;
    let via = if word == row.tag {
        "the tag".to_string()
    } else if word == row.emit {
        "the game's word".to_string()
    } else {
        format!("an alias of {}", row.tag)
    };
    println!("{word} is {via}: currency table v{}", t.version());
    print_row(row, true);
    Ok(())
}

fn print_row(row: &Currency, expand: bool) {
    let mut notes = Vec::new();
    if row.emit != row.tag {
        notes.push(format!("game writes {}", row.emit));
    }
    if !row.aliases.is_empty() {
        notes.push(format!("also {}", row.aliases.join(", ")));
    }
    if let Some(when) = &row.retired {
        notes.push(format!("retired {when}"));
    }
    let line = format!("  {:<22} {:<28} {}", row.tag, row.display, notes.join("; "));
    println!("{}", line.trim_end());
    if expand {
        for e in &row.evidence {
            println!("{:>26} {e}", "");
        }
    }
}
