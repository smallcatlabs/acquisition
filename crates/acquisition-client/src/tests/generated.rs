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
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);
    // proptest's own reading of its environment, the one the `proptest!`
    // macro applies again: `PROPTEST_CASES` replaces the count and
    // `PROPTEST_RNG_SEED` the fresh seed, so what is printed is what runs.
    let config = proptest::test_runner::contextualize_config(ProptestConfig {
        cases: cases(local, full),
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(nanos ^ (u64::from(std::process::id()) << 32)),
        ..ProptestConfig::default()
    });
    let (cases, seed) = (config.cases, config.rng_seed);
    eprintln!("{cases} cases from seed {seed}: PROPTEST_RNG_SEED={seed} draws them again");
    config
}
