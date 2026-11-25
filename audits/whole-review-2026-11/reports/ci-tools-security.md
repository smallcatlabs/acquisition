## 1. Model

The **local `tools/gate.sh` is the real gate**. C171 says a session pushes on a local green, and CI is a lagging second opinion: about half of all pushes to main are cancelled by `concurrency: cancel-in-progress` before they finish. Across 605 main commits in 15 days, roughly one in four got a CI verdict.

The gate has 14 stages. Six are cargo (build, test via nextest, clippy, winclippy, doc, fmt). Four are home-grown scripts (`docs-check.sh` 426 lines, `arch-check.sh` 1,183, `trade-check.sh` driving ~8k lines of Python table generators and sitting tools, and `bothy.py` 835). The last three guard the dev loop itself: `features`, `artifacts` and `tree`.

`features` and `artifacts` exist only for C84/C197, the artifact identity. A client compares the contract revision (a digest of the protocol and store sources) and the daemon executable's hash before it reuses a running daemon. So a test run must write byte-identical binaries to a plain build. Feature pins in `acquisition-cli/Cargo.toml` hold that, and the two stages check it.

CI runs on Linux (self-hosted on main, about 4 min) and Windows (hosted, about 15 min, the critical path). A separate **breakers** job runs the 10 `*-breakers.sh` scripts (1,882 lines), each proving a check refuses what it should. There is no macOS gate in CI: macOS gets a weekly Keychain round trip and the sign breakers, and the owner's Mac is the de facto macOS gate. Weekly mutants run in 6 shards for about 2.5–3 h. cargo-deny runs only in CI, never locally.

## 2. Measurements

| What | Value | Command |
|---|---|---|
| Local full gate, wall time | median 65 s, p90 110 s, max 219 s (228 runs since 11-10); test stage median 39 s | awk over `runs/gate/verdicts.tsv`, `$4=="full"` |
| Local stage failure counts (of 228) | test 10, fmt 9, clippy, winclippy, tree and docs 3 each, arch 2 | same |
| CI medians per day | Linux 10–13 min; Windows 14–16 min (flat since 11-12); Weekly 147→165→175 min (rising) | `gh run list --limit 1000 --json …` |
| Main pushes CI finished | Linux 48%, Windows 49% (rest cancelled) | same, `event=push` |
| Failure rate on main (finished runs) | Linux 8/154, Windows 10/160 | same |
| Linux job time | breakers ~11 min vs gate ~4 min; `arch-check-breakers.sh` alone 439 s (its comment says "2 min") | `gh run view <id> --json jobs` |
| Breaker cases | arch 147, docs 53, gate 29, hooks 21, trade 8 | `grep -cE '^\s*run_case' tools/*-breakers.sh` |
| Lockfile packages | 351; 22 names at more than one version (mostly windows-* and the RustCrypto 0.10/0.11 split) | `grep -c '^name = ' Cargo.lock` |
| `unsafe` lines in crates | 43 (libc `getpwuid_r`, Win32 SID/pipe calls, `set_var` in tests); no `unsafe_code` lint anywhere | `git grep -n unsafe -- 'crates/*.rs'` |
| `target/` | 38 GB (debug 28 GB, winclippy 6.5 GB, release 3.2 GB) | `du -sh target/*` |
| Release binaries | acq 21 MB, acq-mcp 19.5 MB, acqd 9.8 MB; no `[profile.release]` (no strip or LTO) | `ls -la target/release` |
| Python tools | 23 files; 0 of 266 `def`s carry a return annotation | `grep -cE 'def .*\) ->'` |

## 3. Where a deep review would pay (ranked)

**H1. Turn arch-check's spelling greps into compiler- and cargo-held rules. (High confidence; the evidence is already filed and nobody acted on it.)**

The numbered checks are 5, 7, 8, 9, 10, 11, 12, 14, 15 and 16 (13 is retired), holding roughly 25 sub-properties. Here is what each could become:

- **Could be a cargo-deny `bans.deny` with `wrappers`, or `bans.features`:** check 5's HTTP-client and TLS rules.
- **Could be a dependency edge, a crate split or visibility:** the member layering in check 5 and the intent-API grep, which a separate intent crate would hold. Feature unification across the workspace defeats doing it with a feature.
- **Could be a type or visibility change:** check 7, the `Member` token grep. `group.rs:697` `pub selector_tree` is still `pub`, which the semantic-model audit flagged.
- **Could be clippy `disallowed_methods` in a crate-local `clippy.toml`:** checks 10, 12, 14 and 15, and the door grep. #230's spike (closed 11-11) showed this works and found one trap: a warm clippy does not re-lint when only `clippy.toml` changes. #196, which would do the work, has been open since 11-09. `clippy.toml` already uses `disallowed-methods`, so this is a shape the tool already has.
- **Must stay a script:** checks 8, 11 (copies that cross into Python) and 16.

The door grep at `tools/arch-check.sh:555` is unchanged since the audit. The CLI still calls `World::observe_at` and `Lock::acquire` directly (`run.rs:710,718`) and `store.rebuild()` (`store_cmd.rs:1113`); #212 is still open.

*Payoff (judgment):* most of the 147 arch breaker cases, perhaps 600–900 shell lines, and 7 min of CI per push. *Read:* `tools/arch-check.sh`, `tools/arch-check-breakers.sh`, `deny.toml`, #196, #230, #212.

**H2. The breakers' cost now sets CI's pace. (High confidence on the numbers; whether they are worth it is judgment.)**

The breakers job takes 11 min against the gate's 4 on the same self-hosted pool, and `arch-check-breakers.sh` alone takes 439 s, because each of its 148 cases reruns the whole arch-check including `cargo metadata`. 209 commits mention breakers.

*The question for a reviewer:* how many real faults did a breaker catch that a review or a test would not have? If the guards become types and lints (H1), the compiler proves the lint fires, and the breakers shrink to the docs and gate scripts. This is the project's most rigorous idea and its heaviest ceremony at the same time. *Read:* `gate-linux.yml` (the `breakers` job), `git log -i --grep=breaker`.

**H3. A flaky test came back after its issue closed. (Verified.)**

`browser::two_stat_groups_one_weighted_are_made_edited_and_run_from_the_page` caused 6 of the 10 Windows failures on main since 11-23. #312 was closed by `51d99a3d` at 2026-11-24 04:50Z. The test then timed out at 240 s on Windows twice more: run 45539750220 at `0701b7d6` and run 45547968713 at `178d51bb`, both descended from the fix (checked with `git merge-base --is-ancestor`). The failure mode changed from FAIL at about 10 s to TIMEOUT. The owner's standing rule (#312, "flaky gates are fixed when found") is being missed because nobody reads CI on main.

The other causes on main:
- cargo-deny advisory (the `url` crate's `Url::origin()` stack overflow): 4 Linux failures, unseen locally because the local gate has no cargo-deny.
- One Windows flake each in `the_offline_reset_holds_the_world_lock` (twice), `a_token_rejected_is_not_restored_after_a_restart` and the install-action bash startup.

*Question:* should the local gate or the close read CI's last finished run? Should cargo-deny be a local stage?

**H4. The cold-clone gate fails on things no document names. (Verified.)**

- `rust-toolchain.toml` lists no `targets`, so `winclippy` fails on a fresh clone until someone runs `rustup target add x86_64-pc-windows-msvc`. One line in the file (`targets = [...]`) would fix it.
- `AGENTS.md`'s once-per-clone list leaves out `jq` (arch-check, release and driver scripts) and the 6.5 GB second target directory.
- Chromium and nextest are listed. `cargo-deny` and `cargo-about` are not, and they are CI-only.

*Payoff:* small edits, and the first gate an agent runs stops failing.

**H5. Is the artifact-identity chain proportionate? (Medium confidence; judgment.)**

The chain is C84, C197, the `features` and `artifacts` stages, the feature pins, `protocol/build.rs` hashing the store and lock, the preflight provenance, and 314 lines in crates naming contract or artifact.

It does have user value: after an upgrade, the next command replaces the older daemon (README line 274). But a release archive always ships matched siblings, so most of the machinery serves the dev loop, where an agent rebuilds under a live daemon.

*Question:* would version plus contract revision plus "dev build: always respawn" retire two gate stages, the pins and the `features` cross-target check? *Read:* `decisions/daemon.md` C84, `tools/gate.sh:27-53,106-240`, `crates/acquisition-protocol/build.rs`.

**H6. A security pass is warranted, scoped narrowly.**

There is no threat model anywhere (`git grep -i 'threat model'` finds nothing), and `SECURITY.md` is 25 lines. The posture is better than the documents say:
- The world directory is checked for owner and mode 0700 (`world.rs:623-645`).
- The Windows pipe compares the server's SID (`world_windows.rs:215-238`).
- `acq web` binds 127.0.0.1 with a random path token and host/origin checks (`web/mod.rs:599-608`).
- 94 lines name redaction.

Supply-chain loose ends:
- Actions are pinned by tag, not SHA, and `sha_pinning_required` is false. A push to main runs them on the owner's self-hosted hardware.
- cargo-deny and cargo-about install unpinned.
- `bans.multiple-versions = "allow"`, and there is no `bans.deny` list.
- The gate workflows have no `permissions:` block (the repository default is read, so this is low risk).
- macOS signing has no notarization, Windows binaries are unsigned, and there is no SBOM or provenance attestation.

*Proposed scope:* one page that writes down what a local attacker and a hostile API body (item names, JSON) can reach — the socket, the web page's rendering of item text and HTML escaping, the log and `diagnose` output — plus Actions pinning. Skip a general audit.

**H7. Release and distribution.**

`alpha.1` (2026-10-22) is 2,257 commits behind. Installing and upgrading is a manual download plus a SHA256SUMS check, and the README says so. The release workflow is solid: 5 targets, a smoke test of each archive, a signed macOS build with the designated requirement checked, a draft gated on the Keychain job. Missing: a `[profile.release]`, which would likely cut the 50 MB of binaries a good deal (unmeasured), and the version is still `1.0.0-alpha.1` in the workspace.

## 4. Low yield / already covered

- `gate.sh` itself: the record of stage results, the `tree` stage and the exit-status table are sound and measured. A median of 65 s is good.
- The hooks (C184, C186) are small (commit-msg 29 lines, pre-push 99) and the CI committer check has never failed (315 of 315). Assessment #293 item 9 (drop the printed rebase recipe, about 30 lines) is still undone and still the whole of it.
- The `persist-check` driver (assessment item 8) is still present; it is the owner's call.
- The trade and sitting Python generators: their output is checked in the gate (C198), so they are load-bearing. They are untyped, but `--check` and `--self-test` hold them.

## 5. Blind spots

- Cold build time on any machine: nothing records it.
- Whether the breakers ever caught a real regression, as opposed to a malformed breaker. Answering that needs a read of 209 commits.
- Weekly mutants runner-hours (estimated 15 h a week, not measured per shard).
- `daemon-tests-census.py`, `site-listings.py`, `trade-changes.py`, `trade-evidence.py`, `mutants-compare.py` and `trade-pages.py` (last touched 10-17) are called by no gate, CI job or skill that I found; they look like one-off or occasional tools. I did not trace them through `search/` scripts.
- I did not read the `diagnose` redaction or the web page's escaping in depth.

## 6. Surprises

- The `breakers` job, not the gate, is the slowest Linux job. The workflow comment says arch-check's breakers take 2 min; they take 7.3.
- #312 is closed, yet its test failed twice the next day.
- The local gate cannot run cargo-deny, so half of all Linux failures on main were advisories that no local green could have predicted.
- `rust-toolchain.toml` says "nothing floats", but the gate needs a target it does not install.

**Verified by re-reading myself:** H2's timings (step data from run 1 of 5), H3's ancestry, H4's toolchain file and the `AGENTS.md` prerequisites, and the door grep being unchanged at `arch-check.sh:555` with the CLI call sites above.
