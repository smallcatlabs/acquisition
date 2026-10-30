# tools-hygiene — the checks and helpers that misled a session

Issues #117, #118, #126, #114, #106 and #135, the step after
league-buildables (`roadmap/README.md`, "Current step"): the owner's
pre-league plan, its fourth and last item. #135 joined at
league-buildables' close, filed as `area:process`. Every concern is a
tool, a test helper or a working-directory document; nothing here
changes what a user meets. One commit per concern. The rhythm is
"A step".

## Goal

Each tool that refused the wrong thing, passed something silently or
printed a refusal where it should have printed rulings is fixed or named
as declined. The docs check's knob refusal names the convention a
test-only file follows (#118), and its history-citation scans read a
citation wrapped after "at" (#126). `bothy pack` routes every verbs-crate
module (#114) and stops routing `README.md` to the search rulings
(#106). Each fixed check gains a breaker case. The case-count helper's
three copies say they are copies (#117). A Windows-target clippy runs on
this Mac at the cost measured below (#106). And `search/repoe/README.md`
names only extracts the tree holds (#135).

## Read

- `AGENTS.md`: "The gate decides", "Structure over discipline" (P4, P5),
  C1 (the dependency edges `tools/arch-check.sh` holds), C122 (one
  harness; a behaviour pinned once).
- `roadmap/README.md`, "A step"; this directory's `TEMPLATE.md`. There
  is no `decisions/process.md`: a declined item is named in its issue's
  close and the step's close, not as a registry line.
- The issues: #117, #118, #126, #114, #106 (its body and its one
  comment, the Windows clippy candidate), #135. Each "Closes when" or
  "Done when" is quoted below.
- The code:
  - `tools/docs-check.sh`: the header of the knob check (270 to 280) and
    its `find` (288); `AT_HASH` (77); the cited-commit scan (174 to
    179, line by line through `grep -o`); the history-citation scan
    (189, `LEAD` at 192); the document-path check (§6, 314, and the
    header's reason for skipping the roadmap at 164 and 321 to 326).
  - `tools/docs-check-breakers.sh`: its form (each case states the
    words the check must print and restores from copies, never
    `git checkout`).
  - `tools/bothy.py`: `area_of_path` (284 to 305), the basename and
    crate rule at 299 to 303; `tools/bothy-breakers.sh`.
  - `decisions/frontends.md`, its intro (line 3), which names
    `acquisition-cli` and `acquisition-mcp`, not `acquisition-verbs`.
  - `tools/trade-check.sh` (64 lines): its stages are the sitting's
    tools run as self-tests.
  - The case-count helper: `crates/acquisition-daemon/src/tests/generated.rs`
    and `crates/acquisition-client/src/tests/generated.rs` (identical,
    42 lines each, and their module doc already names the search's as
    their shape); `crates/acquisition-search/tests/common/generated.rs`.
  - `search/repoe/README.md`: the extracts table, F1 (line 36) and F3
    (line 55); `search/DIGEST.md`, S62; the two generators,
    `search/repoe/scripts/trade-stat-map.py` and `base-taxonomy.py`.
  - `.github/workflows/gate-linux.yml`, the `breakers` job (146 to 182),
    which runs each breaker suite in CI.
  - `tools/gate.sh`: its header (stages, `--docs`) and stage list.
- `python3 -I tools/bothy.py pack --path` for each file above. For the
  nine verbs files it refuses today: that is #114.

## Input

The last push's CI: `Gate (Linux)` run 40977952323 and `Gate (Windows)`
run 40977952395, both on `c7786936`, both `in_progress` when this brief
was written. Read their conclusions before starting; a red run is
handled before this step's first commit.

#114, measured on `c7786936` by running `pack --path` over every file
in `crates/acquisition-verbs/src/`: nine exit non-zero, as the issue
says: `characters.rs`, `diagnose.rs`, `events.rs`, `fault.rs`,
`frontend.rs`, `receipt.rs`, `refresh.rs`, `refusal.rs`, `refused.rs`.

#106's basename match, measured: `pack --path README.md` prints the
search and trade rulings ("Rulings for `search`", "Rulings for
`trade`"), because `search/README.md` in their intros ends with the
basename.

#106's Windows clippy, measured on this Mac on `c7786936` in a scratch
target directory, with a stub for the C toolchain (`CC_`, `CXX_` and
`AR_x86_64_pc_windows_msvc` pointing at a script that writes each output
it is named as an empty file):
`cargo clippy --keep-going --workspace --all-targets --all-features
--locked --target x86_64-pc-windows-msvc -- -D warnings`. Cold, from an
empty target directory, it took 42 s wall (cargo: 41.23 s) on this
10-core Apple M4. Warm, after touching `acquisition-daemon/src/lib.rs` and
`acquisition-cli/src/main.rs`, it took 2 s. Both exited 0 with no
warning, which agrees with the Windows gate's last green run. It needs
the `x86_64-pc-windows-msvc` target installed (`rustup target add`); this
Mac has it, and a fresh clone would not.

#117's helper copies: the daemon's and the client's are byte-identical.
The search's holds the same body inside a 2,841-line module.

#126's failing input: a citation wrapped after "at", as `e91da86b` did,
with a dead hash on the next line. Today neither scan reads it. The
breaker case stages it.

#135: the two files are written by the two generators when they run in
place. `base-taxonomy.py` reads the owner's local raw stores read-only.

## Do

1. **#114.** `decisions/frontends.md`'s intro also names
   `acquisition-verbs`, the crate its rulings bind (`AGENTS.md`'s
   workspace table already gives the verbs crate this file). `pack
   --path` then routes every file of the crate.
2. **#106, bothy's basename.** A ticked intro token names a file when
   the file's path ends with the token, not when the token ends with the
   file's basename. Before building, run `pack --path` over every tracked
   file and save which areas each routes to. Run it again after and
   report every file whose routing changed, not just `README.md`. Add a
   breaker case in `tools/bothy-breakers.sh`.
3. **#118.** State the convention in the knob check's header: a
   test-only file of `src/` lives under a `tests/` directory. The
   refusal names the convention when it fires. Teaching the check to
   resolve `mod` lines and `#[path]` is more mechanism than the one
   convention, so it is not built. Add a breaker case: a knob read in a
   test-only file outside `tests/`, refused with words that name the
   convention.
4. **#126.** Both scans, the cited commits and the history citations,
   read a line break and a comment's leader between "at" and the hash,
   as `LEAD` already does between the path and "at". The cited-commit
   scan reads line by line today, so it changes to read across a line
   end. Add a breaker case: a citation wrapped after "at" with a dead
   hash, refused. Report the count of citations each scan checks,
   before and after.
5. **#117.** Keep the copies. The owner said "no ruling now" (#85's
   close), and a shared test-support crate would be a new C1 edge.
   The search's module doc names the daemon's and the client's as
   copies of its shape, so each of the three names the others. The issue
   closes with this choice named. This is a choice that needs no ruling,
   listed at the close.
6. **#135.** Open both files after a run in place. Track them if they
   hold only what the extracts table describes (counts, classes, ids,
   GGG's text) and no item or character name from the owner's raw
   stores. Otherwise correct the table, F1, F3 and S62 to name only what
   the tree holds. The report says which and why.
7. **#106, the other three.**
   - The document-path check reads no roadmap file. Its header gives
     the reason (about eighty deletion names told as story). Declined,
     unless the build finds `roadmap/README.md` alone has no dead names,
     in which case it is read and a breaker case stages one.
   - The trade check has no breaker suite. Its stages are self-tests
     that each prove a refusal of their own. Confirm, stage by stage,
     that each self-test exercises a refusal. If each does, this is
     declined with that reason. If one does not, that stage gains the
     case it lacks.
   - The Windows clippy is a gate stage. The owner ruled this on
     2026-10-29, on the measurement above: "Make the Windows-target
     clippy a gate stage". Record it first as a dated bullet in
     `roadmap/README.md`'s rulings, in its own docs commit.
     - The stage builds into its own target directory, so the
       `artifacts` stage's hashes of the host binaries are untouched.
     - Where the Windows target is not installed, the stage fails and
       names `rustup target add x86_64-pc-windows-msvc`. It never
       passes as a stage that did not run.
     - `tools/gate.sh`'s header and stage list say what the stage sees
       (lints only, not the tests) and that it lets nothing link.
     - The stub script lives under `tools/`, with a header that says
       what it is.
     - CI runs the same `gate.sh`. Whether each workflow installs the
       target or the Windows gate skips the stage, because it already
       runs clippy natively, is the build's choice, named in the commit
       and the report.
     - `gate-breakers.sh` gains a case if its form covers a stage that
       must fail when its tool is absent.
8. The full gate, then each changed check's breaker suite run by hand
   (`tools/docs-check-breakers.sh`, `tools/bothy-breakers.sh`), then
   the close per "A step".

## Done when

- `pack --path` exits 0 for every file under `crates/acquisition-verbs/src/`
  (#114's done-when), and `pack --path README.md` names neither the
  search nor the trade rulings.
- #118: the knob check's header states the convention, and a breaker
  case shows the refusal naming it.
- #126: a breaker case with a citation wrapped after "at" and a dead
  hash is refused (#126's closes-when).
- #117: each of the three helper files names the others in its module
  doc.
- #135: every extract `search/repoe/README.md` lists is in the tree, or
  the table and its citations name only those that are (#135's
  closes-when).
- #106: each of its four items is fixed with a breaker case where it is
  a check, or named as declined in the close (#106's closes-when).
- The gate has a Windows-target clippy stage, and the owner's ruling is a dated bullet in `roadmap/README.md`.
- `tools/gate.sh` green on the step's tree; `docs-check-breakers.sh`
  and `bothy-breakers.sh` green, their output's last lines in the
  report.

## Not in scope

- A shared test-support crate (#117's other choice): a C1 ruling, not
  asked for.
- Resolving `#[cfg(test)] mod` lines in the docs check (#118's other
  choice).
- The five research scripts that type the poe1 pin (#134): phase 3b.
- #131's timing bounds, #132, hakari (#128's triggers).
- Any change a user meets: help text, the references, the README's
  tour or knob table.

## Rules

`TEMPLATE.md`'s. What differs:
- The shared checkout, no worktree; scratch under
  `$TMPDIR/tools-hygiene/`. The Windows clippy uses a scratch
  `CARGO_TARGET_DIR`, never `target/`.
- The breaker suites edit the working tree and restore it from copies.
  Never run one while the gate runs, or while another suite runs.
- Do 6 reads the owner's local raw stores read-only, through the
  generator as it opens them. Nothing else reads them.

## Report

`TEMPLATE.md`'s.
