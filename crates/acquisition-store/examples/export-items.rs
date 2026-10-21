//! `export-items` — an owner-run read of item bodies for the mock's
//! fixture (roadmap 2c, `roadmap/briefs/2c-build.md` item 3): each named
//! item's body exactly as the store holds it, the verbatim GGG item, with
//! the location the store records for it beside it, written to stdout as
//! one JSON array. The owner (or an agent under his word) runs it, reads
//! the output, and takes from it what the hand-written fixture needs;
//! nothing here is shipped, and nothing it prints is redacted.
//!
//! ```sh
//! cargo run -p acquisition-store --example export-items -- <item id>... > items.json
//! ```
//!
//! Flags: `--provider <ggg|mock>` (ggg); `--account <selector>` (the sole
//! account, else exact username, name without discriminator, or uuid —
//! `acq accounts` lists them); `--dir <provider store dir>` (the
//! platform's, `acq daemon status` prints it). Then one or more item ids
//! (`acq search --json` prints them).
//!
//! **What it reads.** The store as every `acq` read opens it
//! (`Store::open`: WAL, the busy timeout — a daemon may be running), then
//! one `Store::item` per id. No fact and no row is written by this
//! program; no daemon is contacted; nothing touches the network. One
//! caveat `Store::open` carries: a store behind this build's facts
//! schema is migrated on its first open, which is a write, and so was
//! the owner's v8 store against the v9 build of 2026-10-21. To export
//! from a store you mean to leave as it is, export from a copy
//! (`sqlite3 <db> ".backup <copy>"`, then `--dir` at the copy).
//!
//! **What it prints.** For each id found, `{"location": …, "item": …}`:
//! `item` is the stored body, which is GGG's item minus its lifted
//! `socketedItems` (each socketed gem is its own row, whose `location`
//! names its host under `socketed_in`; export the gem's id beside it);
//! `location` is realm, league, location kind and id, container,
//! `socketed_in` and `removed_at`. An id the store does not hold is a
//! line on stderr, and the run exits 1 after printing the rest.

use std::io::Write;
use std::path::PathBuf;

use acquisition_store::{Index, Store, account_path, store_dir};
use anyhow::{Result, anyhow, bail};
use serde_json::json;

const USAGE: &str = "usage: cargo run -p acquisition-store --example export-items -- \
[--provider ggg] [--account <selector>] [--dir <provider store dir>] <item id>... > <file>.json";

fn main() -> Result<()> {
    let mut provider = "ggg".to_string();
    let mut dir: Option<PathBuf> = None;
    let mut account: Option<String> = None;
    let mut ids: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |name: &str| {
            args.next()
                .ok_or_else(|| anyhow!("{name} takes a value\n{USAGE}"))
        };
        match arg.as_str() {
            "--provider" => provider = value("--provider")?,
            "--dir" => dir = Some(PathBuf::from(value("--dir")?)),
            "--account" => account = Some(value("--account")?),
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            flag if flag.starts_with("--") => bail!("unknown argument {flag:?}\n{USAGE}"),
            id => ids.push(id.to_string()),
        }
    }
    if ids.is_empty() {
        bail!("at least one item id is required\n{USAGE}");
    }
    let dir = dir.unwrap_or_else(|| store_dir(&provider));
    let index = Index::load(&dir)?;
    let entry = index.resolve(account.as_deref())?.clone();
    let store = Store::open(&account_path(&dir, &entry.username))?;

    let mut found = Vec::new();
    let mut missing = 0usize;
    for id in &ids {
        match store.item(id)? {
            Some(row) => found.push(json!({
                "location": {
                    "realm": row.realm,
                    "league": row.league,
                    "location_kind": row.location_kind,
                    "location_id": row.location_id,
                    "container": row.container,
                    "socketed_in": row.socketed_in,
                    "removed_at": row.removed_at,
                },
                "item": row.json,
            })),
            None => {
                eprintln!("no item {id:?} in the store");
                missing += 1;
            }
        }
    }

    let mut out = std::io::stdout().lock();
    serde_json::to_writer_pretty(&mut out, &found)?;
    out.write_all(b"\n")?;
    out.flush()?;
    if missing > 0 {
        eprintln!("{missing} of {} ids not found", ids.len());
        std::process::exit(1);
    }
    Ok(())
}
