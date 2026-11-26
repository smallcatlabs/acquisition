//! Write the seeded world (`acquisition-client/tests/harness/seeded.rs`)
//! under a store directory and print the provider directory it wrote:
//! three accounts, two realms, three leagues, prices, a sync policy and a
//! saved search, recorded through the store's own ingest, so a seat, a
//! builder checking a change or a counted task reads one world with no
//! daemon and no login. For reads: run `acq` over it with
//! `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR=<dir>` and `ACQ_NO_SPAWN=1` (the
//! mock-session skill). A mock login over it orphans its intent.

#[path = "../../acquisition-client/tests/harness/seeded.rs"]
mod seeded;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(dir) = std::env::args().nth(1) else {
        return Err("usage: seeded-world <store-dir>".into());
    };
    let dir = std::path::Path::new(&dir);
    if dir.join("mock").exists() {
        return Err(format!("{} already holds a mock world", dir.display()).into());
    }
    let mock = seeded::Seeded::owner().write(dir, acquisition_store::now());
    println!("{}", mock.display());
    Ok(())
}
