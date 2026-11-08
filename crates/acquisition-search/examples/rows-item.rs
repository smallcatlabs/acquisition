//! M9 (`search/MEASUREMENTS.md`; 4-full part 1, step 5): a rows answer
//! asked `fields: ["icon", "item"]` over a held corpus, at 20, 200 and
//! 1,000 rows of the empty query — the answer's compact JSON bytes, and
//! the median of eleven asks of the answer and its JSON, the projection
//! of each row's item by `show`'s one maker inside it. Given a sqlite
//! `.backup` copy by its path, opened as `copy-as-store.rs` opens one,
//! never a store directory, beside an empty intent file in memory. Never
//! in the gate: the input is a track's `raw/`. Since the cap
//! (`acquisition_store::bound::ITEM_ROWS`) the 1,000-row ask answers 200
//! rows: M9's 1,000-row figure was taken before it, at `b90102be`, and is
//! re-taken only at that commit; at a later one this measures 20 and 200.

use std::time::Instant;

use acquisition_search::{Corpus, Realm, Request};
use acquisition_store::corpus::RealmScope;
use acquisition_store::{Annotations, Store};
use serde_json::json;

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(copy) = std::env::args().nth(1) else {
        return Err("usage: rows-item <copy.db>".into());
    };
    let store = Store::open(copy.as_ref())?;
    let uuid = store.read_corpus(RealmScope::All, |header, _| Ok(header.account_uuid.clone()))?;
    let intent = Annotations::open_memory_for(&uuid)?;
    let realm = Realm::parse("pc")?;
    let t = Instant::now();
    let corpus = Corpus::load(&store, &intent, Some(&realm))?;
    println!(
        "load (pc, live): {:.0} ms, {} items",
        t.elapsed().as_secs_f64() * 1000.0,
        corpus.items().len()
    );
    for n in [20usize, 200, 1000] {
        let request: Request = serde_json::from_value(json!({
            "scope": { "realm": "pc" },
            "query": { "text": "" },
            "view": { "rows": { "limit": n, "fields": ["icon", "item"] } },
        }))?;
        let answered = acquisition_search::answer(&corpus, &request)?;
        let bytes = serde_json::to_vec(&answered)?.len();
        let pretty = serde_json::to_vec_pretty(&answered)?.len();
        let mut ms = Vec::new();
        for _ in 0..11 {
            let t = Instant::now();
            let answered = acquisition_search::answer(&corpus, &request)?;
            let _ = serde_json::to_vec(&answered)?;
            ms.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        println!(
            "N={n}: {} rows, {bytes} B compact ({pretty} B pretty); answer and its JSON {:.1} ms (median of 11)",
            answered.rows.len(),
            median(ms)
        );
    }
    Ok(())
}
