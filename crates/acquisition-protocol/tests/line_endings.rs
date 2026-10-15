//! The contract revision is a digest of checked-in bytes (`build.rs`,
//! C84), so a checkout must give every platform the same bytes. Git on
//! Windows converts text files to CRLF unless `.gitattributes` says
//! otherwise; `Cargo.lock` and the store's `.sql` were left to `text=auto`
//! and gave the Windows release archive another revision than the four
//! Unix ones built from the same commit (release run 38668066191,
//! 2026-10-15). Every kind the digest reads is pinned to LF.

use std::path::Path;

#[test]
fn every_kind_the_digest_reads_is_checked_out_with_lf_everywhere() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let digest = std::fs::read_to_string(root.join("crates/acquisition-protocol/digest.rs"))
        .expect("digest.rs");
    let line = digest
        .lines()
        .find(|l| l.starts_with("const KINDS: &[&str] = &["))
        .expect("digest.rs declares KINDS on one line");
    let kinds: Vec<&str> = line.split('"').skip(1).step_by(2).collect();
    assert!(
        kinds.contains(&"rs") && kinds.contains(&"lock"),
        "{kinds:?}"
    );
    let attributes = std::fs::read_to_string(root.join(".gitattributes")).expect(".gitattributes");
    for kind in kinds {
        let rule = format!("*.{kind} text eol=lf");
        assert!(
            attributes.lines().any(|l| l.trim() == rule),
            "`.gitattributes` lacks `{rule}`: a Windows checkout would digest another revision"
        );
    }
}
