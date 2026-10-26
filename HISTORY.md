# History

One line per closed slice: what it was, its commit range, when it
closed, and the commit at which its full record was last whole.
Everything else is `git log`. A review finding's story is the fixing
commit's message; a shape that came back is `REVIEW-CHECKLIST.md`'s.

| Slice | Range | Closed | Record whole at |
| --- | --- | --- | --- |
| Network cleanup, packages N0 to N6: strict parsing, the gate, the dispatcher, the stress tests | `1e17e812..29a2214b` | 2026-08-22 | `25332463` (the full narratives at `8198bef8`) |
| The ladder: ten live rungs, the daemon halts rather than floods | the runs of 2026-08-22 to 2026-08-27 | 2026-08-27 | `944406ff` (the rung table, the soak mechanics, the postmortems); `LIVE-TESTING.md` whole at `b6197a82` |
| Testing notes: the register walk and three experiments | 2026-08-24 | 2026-08-24 | `7aee76ee` (the experiments); the file before its cut at `b6197a82` |
| The refresh slice: policy, plan, apply, characters, legible output | `8363f8b8..d660d1f5` | 2026-09-03 | `f4546bdd` (the narratives at `d660d1f5`, the tracer's review rounds at `35fb35d9`) |
| The pricing slice: render-first pricing | `ecdb9d3d..04993ca5` | 2026-09-09 | `12acf409` (the open record at `6203e984`) |
| The daemon split: `acqd`, the protocol and client crates, the world | `10f9bab5..599e8577` | 2026-09-11 | `e7b813b8` (the open record at `abef9a45`; the packet, note 18 at `decda84e`) |
| The item search slice: the crate, the language, the derivation, the answer | `080a8581..6b940b70` | 2026-10-10 | `ab67cbfa` (the plan at `2d25cb03`; the brief and the observations at `453a4429`) |
| The relicense audit: MIT OR Apache-2.0, nothing transcribed | `audits/relicense-2026-10/`, from `401a1ea9` | 2026-10-11 | the audit directory |
| Rehome: the whole-CLI seat, its fix round and rerun | `audits/rehome-2026-10/`, from `9054f2f1` | 2026-10-13 | the audit directory |
| Three platforms: Windows, macOS, Linux, released from GitHub | `4f49e285..eccd22c8` | 2026-10-15 | `06c34401` (the open record at `eccd22c8`) |
| The trade import: a site URL as a third spelling of a query | `6745e5ba..16bba23e` | 2026-10-18 | `807f237e` (the brief's steps at `77666ea4` and `6e31d630`; note 38 at `16bba23e`) |
| The release, `v1.0.0-alpha.1`: pricing on the MCP, the README, the policies, the seat | `05e6c70c..d0ab421e` | 2026-10-19 | `e7f279b5` (the open record and the brief at `d0ab421e`) |
| Roadmap phase 0: the week's fixes | `roadmap/README.md`'s ledger | 2026-10-20 | `cca73395` (the briefs and the review files) |
| Roadmap phase 1: the governance reset | `roadmap/README.md`'s ledger | 2026-10-20 | `0704a8f7` (the briefs, the drafts and the review files) |
| The honesty slice: the daemon reports no success it did not earn, a count says what it counts, the doors take the world (C144 to C147; #71, #59, #61, #62, #49, #69, #75 closed; #11, #63, #16 in part; #74 opened) | `efc6e745..0b199df3` | 2026-10-26 | the briefs under `roadmap/briefs/h1-*.md`, `h2-*.md`, `h3-*.md`; the reviews whole at `0e7ed91d`, `65b18c90`, `a519fb02`, `117c0aa6`, `e82fa488`, `684262bb`, `a0bb6ab1` |
