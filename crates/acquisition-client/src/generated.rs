//! How a generated property runs in this crate's tests (#85): the search's
//! shape (`acquisition-search/tests/common/generated.rs`, `cases` and
//! `config`), copied because a test module is no other crate's to link.
//! The daemon and the client hold the same text; a change here is made in
//! both, and in the search's when it is a change of the shape.

use proptest::test_runner::{Config as ProptestConfig, RngSeed};

/// How many cases a generated property runs: `local` in a gate run on a
/// session's machine and on the Windows gate, `full` where
/// `ACQ_CASES=full` is set, which the Linux gate's workflow sets. The cases
/// are fresh on every run (see [`config`]), so the local count is a share
/// of the full one drawn each time. `PROPTEST_CASES=N` is proptest's own
/// and replaces either, for a long run by hand.
pub(crate) fn cases(local: u32, full: u32) -> u32 {
    match std::env::var("ACQ_CASES").as_deref() {
        Ok("full") => full,
        _ => local,
    }
}

/// The config of a generated property: its count ([`cases`]), no
/// regression file, and a seed that is new on every run and printed, so
/// `PROPTEST_RNG_SEED=<seed>` (with `ACQ_CASES` as it was, and the one
/// test named) draws the same cases again.
pub(crate) fn config(local: u32, full: u32) -> ProptestConfig {
    let seed = std::env::var("PROPTEST_RNG_SEED")
        .ok()
        .and_then(|seed| seed.parse::<u64>().ok())
        .unwrap_or_else(|| {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos() as u64);
            nanos ^ (u64::from(std::process::id()) << 32)
        });
    let cases = cases(local, full);
    eprintln!("{cases} cases from seed {seed}: PROPTEST_RNG_SEED={seed} draws them again");
    ProptestConfig {
        cases,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(seed),
        ..ProptestConfig::default()
    }
}
