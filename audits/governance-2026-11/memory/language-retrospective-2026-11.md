---
name: language-retrospective-2026-11
description: "The 2026-11-09 retrospective on Rust vs Go/Zig/C/C++/F#/TypeScript for Acquisition; where it lives, its verdict, and the items it raised for the owner's review"
metadata:
  type: project
---

On 2026-11-09 Fable wrote `roadmap/LANGUAGE-RETROSPECTIVE-2026-11-09.md`
(committed on main at 5f3ad0d1 and pushed on his word, 2026-11-09; its raised items are issues #196–#204) from four Opus
readers over the tree and six Opus language studies.

Verdict: Rust was right, for reasons other than the usual ones. The
borrow checker cost the agents near nothing (9 "borrow" commits in 3,212);
the cost was Cargo's feature unification, compile time and disk; the gain
was a free memory-safety floor (0 of 77 checklist rows), checked
arithmetic landing on the limiter, a few compile-time proofs at the choke
point and credential, and cargo/clippy as a refusal engine. Go is the one
defensible equal; its axis is reviewer attention vs agent loop time.

Raised for review, not ruled: the TLS stack (reqwest with rustls-tls) as a
ground-truth claim beside the user-agent (two studies flagged the
ClientHello fingerprint under Cloudflare); typed JobKind (issue #100);
typed lints (dylint) in place of the spelling-sensitive greps; the daemon
carrying the unwrap lint via a lock helper; serde_json preserve_order; an
untuned release profile; the 1M-item corpus layout before phase 4.

**Why:** a later session asked about the language, the GUI stack or any
of the raised items should start from the file, not redo the study.

**How to apply:** read the file's §5 and §6 before proposing any of the
raised items; cite it rather than re-deriving. Related: [[orchestrating-steps]],
[[gui-investigation-2026-10]].
