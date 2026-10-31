# words — the words the seats stumbled on, ahead of the league seat (#20, #11, #101, #16)

## Goal
"Pull-forward" item 1 (`roadmap/README.md`), pulled ahead of the
league build's first seat at the owner's word of 2026-10-30 ("let's
pull these forward"). Four issues, in this order:
- #20: the README's checksum step is checked by the tool, not by eye.
- #11: what step h2 left open of the words nobody understood.
- #101: the three places the CLI and the MCP diverge without a ruling,
  each made one behaviour or a stated difference.
- #16: a printed command runs as printed in the shell the platform
  gives its user, PowerShell on Windows included.

Why now: the machine is free until the 3.30 launch (2026-11-13), and
alpha.2 ships these words to Windows users. The league seat then
checks the fixes; what it finds new becomes new rows.

## Read
- `AGENTS.md`; `decisions/frontends.md`: C53 (three output levels; JSON
  is the contract, text a function of it), C146 (a count says what it
  counts; a JSON field's name is the contract), C149 (one object, counts
  beside lists), C116 (no id in user-facing text). `decisions/search.md`:
  C118 (an answer says how old a row is). `AGENTS.md`: C1 (the daemon
  links the protocol and the store alone, never the search), C122 (a
  wording is one golden, regenerated under `ACQ_UPDATE_FIXTURES=1`,
  naming the facts a reader must find).
- `REVIEW-CHECKLIST.md`, the frontends' rows; `CLI-REFERENCE.md` and
  `MCP-REFERENCE.md` for every verb and tool this step touches.
- The golden mechanism: `crates/acquisition-cli/tests/harness/mod.rs`,
  `golden` (around line 840) and `Transcript::run` (259 to 314). A
  regenerated golden is read as a diff of what a user now reads.
- #20: `README.md` lines 47 to 70 (the install steps); the module doc
  and `the_install_lines_name_the_archives_the_release_makes` in
  `crates/acquisition-cli/tests/readme_tour.rs` (its URL check refuses
  every download that is not an archive); `.github/workflows/release.yml`
  line 230 (`sha256sum * > SHA256SUMS`, a file the release makes).
- #11: the issue and its comment of 2026-10-27; step h2's commits
  (`45badda4`, its fixes `7c4310da..fa3335bf`, tail `5bed4bc3`, close
  `18aa2112`), whose messages list what h2 left open; the readiness
  seat's `audits/readiness-2026-10/seat/REPORT.md` and `DEBRIEF.md`.
- #101: the issue; `crates/acquisition-cli/tests/apply_loop.rs` (around
  lines 206 to 250: a failed `refresh --apply` read on stdout, `acq
  result` on stderr); `CLI-REFERENCE.md`, `acq policy set` (the
  `--if-revision` text, around line 463) and `acq price set` (around
  line 700); `MCP-REFERENCE.md`, `set_sync_policy` (line 162 on) and
  `price_set`; the sizes in the `describe` text (`MCP-REFERENCE.md`
  lines 7 and 49); `roadmap/briefs/r5-surface.md`.
- #16: `acquisition_search::answer::shell_quoted`
  (`crates/acquisition-search/src/answer.rs`, around line 3196), the one
  quoting function every printed command uses (callers in `price_cmd.rs`,
  `plan_cmd.rs`, `store_cmd.rs`, `main.rs` of the CLI, `acquisition-mcp/src/main.rs`,
  `acquisition-verbs/src/frontend.rs`); its twin `posix_word` in
  `crates/acquisition-daemon/src/daemon/login.rs` (around line 762),
  which the daemon keeps because C1 bars it from linking the search;
  `c144_the_printed_logout_command_runs_as_printed` in
  `crates/acquisition-cli/tests/error_kind.rs` (Unix only);
  `a_query_with_an_apostrophe_is_read_from_stdin_and_routed_shell_quoted`
  in `crates/acquisition-cli/tests/search_json.rs`.
- The platform: the Windows-target clippy is a gate stage, but
  a Windows-only test runs only in CI's Windows gate.

## Input
The last push's CI: `Gate (Linux)` run 41155555172 and `Gate (Windows)`
run 41155555173, both on `3728233d`, both success.

What is still open, read at `3728233d` (a subagent's sweep, its
citations spot-checked by the brief's writer):

**#11.** h2 fixed the shop's `StashN` map, `store status`'s four counts,
`never_fetched_by_type`'s span, the `sum … lacking` line and `price
list`'s refusal for a container with its own row. Open:
1. **No listing of the residue or the invalid-note items**, on either
   surface. `price status` counts them (`render_status`,
   `crates/acquisition-cli/src/price_cmd.rs` around 301; the tallies in
   `acquisition-plan/src/listing.rs`, `resolve`). The readiness seat asked
   for a `price_list` filter. A filter would sit on `ListFilter`
   (`listing.rs`), the plan's `list` (`pricing.rs`), the CLI's `List`
   args and the MCP's `PriceListParams`. Each item's JSON already
   carries `game.residue`; there is no per-item invalid flag. The
   invalid-notes text line (`price_cmd.rs` around 391) has no pin.
2. **The scope's oldest and newest fetch name no location**
   (`answer_head` in `crates/acquisition-cli/src/search_cmd.rs`, around
   980; `Coverage` in `acquisition-search/src/corpus.rs`). The text
   "the oldest fetch" stands in 20 files under `crates/` (`grep -rln
   'the oldest fetch' crates | wc -l`), most of them goldens.
3. **Three MCP keys no text explains:** `freshness.position_before_listing`
   (the CLI explains it in `shop_cmd.rs`'s `freshness_lines`; the MCP's
   `describe` entry in `acquisition-plan/src/pricing.rs` around 999 does
   not); `persisted` on `accounts` (the tool's description,
   `acquisition-mcp/src/main.rs` around 1441, and `AccountRow` in
   `acquisition-verbs/src/frontend.rs` say nothing); `basis.intent` on
   `show` (the `describe` "answer" entry explains it, and `search`'s
   description points there, but `show`'s description, around 1593,
   does not). C146 bars renaming any of them.

**#101.**
1. A failed `acq refresh --apply` prints its report on stdout, and `acq
   result` on stderr (`apply_loop.rs`).
2. `acq policy set --if-revision 0` writes only where no policy is
   stored; without the flag the CLI overwrites whatever is stored. The
   MCP's `set_sync_policy` requires `if_revision` once a policy exists
   and creates when it is omitted. So the CLI's `0` is the MCP's
   omission, and the CLI's omission has no MCP twin. `acq price set`'s
   `--if-revision` names no meaning for 0.
3. The sizes the `describe` text gives (over 100 KB; computed some
   80 KB; …) are of compact JSON, while `acq --json` pretty-prints, about
   a third larger (r5's measure; the build measures it again).

**#16.** `shell_quoted` and `posix_word` quote for a POSIX shell alone:
`'\''` does not read back in PowerShell. `acq policy set '<json>'`
(`plan_cmd.rs` around 108, `main.rs` around 1013) also hands JSON with
double quotes to a native program, which Windows PowerShell 5.1 strips
and PowerShell 7.3 or newer keeps.

## Do
1. **The owner's ruling first, on #101's `--if-revision 0`**, before any
   of #101's code. The writer's recommendation, to bring with your own
   reading: a stated difference, since a human at the CLI may overwrite
   and an agent must read first, with `set_sync_policy` taking
   `if_revision: 0` as it takes an omission, so a command printed for
   one surface reads on the other; both references say so in a
   sentence. Whatever he rules is a dated bullet in
   `decisions/frontends.md` only if it is a property another area must
   know; otherwise it is recorded in the commit that builds it.
2. **#20.** The macOS and Linux steps fetch `SHA256SUMS` beside the
   archive and check with `shasum -a 256 -c --ignore-missing
   SHA256SUMS` (this Mac's `shasum` takes `--ignore-missing`; a Linux
   `sha256sum` takes the same flag); the Windows step compares
   `Get-FileHash` with the listed digest in a command, not by eye.
   `readme_tour.rs`'s URL check admits `SHA256SUMS` as a file the release
   makes, and only that one besides the archives.
3. **#11.** Agent-facing design, the step's own:
   - a filter on `price list` and `price_list` for the residue items
     and the invalid-note items, the same word on both surfaces, its
     count beside its list (C149), and the invalid-notes text line
     pinned;
   - the location of the oldest and the newest fetch in the scope, in
     JSON as an additive field and in the text; if it changes what
     C118's text says the scope line holds, bring that sentence to the
     owner first;
   - a sentence for each of the three keys where a reader meets it: the
     `describe` "shop" entry, the `accounts` description, and `show`'s
     description pointing to `describe` "answer".
4. **#101.** Items 1 and 3 are the step's own: one stream for a failed
   job's report on both verbs, or a stated difference with its reason;
   sizes that say they are of compact JSON, or a measure that matches
   what `acq --json` prints. Item 2 as the owner ruled in Do 1.
5. **#16.** The quoting follows the platform the binary is built for:
   PowerShell's on Windows (single quotes, a quote inside doubled), a
   POSIX shell's elsewhere, in `shell_quoted` and in `posix_word` alike,
   since C1 keeps them two. Decide how a printed `acq policy set` carries
   JSON on Windows so it runs in both PowerShell 5.1 and 7, `@<path>`
   or `-` from stdin being the shapes the verb already takes. Pin it
   with a test that runs a printed command through `powershell.exe` on
   Windows, beside the POSIX one that runs it through `sh`.
6. Regenerate the goldens and both references; read every changed file
   as a diff of what a user reads, and name in the report each change
   no source change explains.
7. Close each issue in the commit that finishes it ("Closes #N" closes
   the whole issue: use it only on the commit that does).

## Done when
- `README.md` checks the archive against `SHA256SUMS` with a command on
  each platform, and `readme_tour.rs` refuses a download URL that is
  neither an archive the release makes nor `SHA256SUMS`.
- `price list` and the MCP's `price_list` list the residue items and
  the invalid-note items under one filter word, each with its count; a
  test pins each surface, and the invalid-notes line of `price status`
  is pinned.
- The search scope names the location of its oldest and its newest
  fetch in JSON and in text; the goldens that print it are regenerated
  with facts that name it.
- `position_before_listing`, `persisted` and `basis.intent` each have a
  sentence where a reader of the tool that returns them meets it, in
  `MCP-REFERENCE.md`.
- Each of #101's three items is one behaviour on both surfaces or a
  difference stated in both references, pinned; item 2 as the owner
  ruled, his words quoted in the commit.
- On Windows, a printed command quotes for PowerShell, and a test runs
  one through `powershell.exe`; on Unix, the POSIX tests stand.
- `tools/gate.sh` green on the step's tree.

## Not in scope
- Renaming a JSON key (C146 bars it).
- New words the league seat finds: new rows after it.
- #131's clock-chosen words, declined (its comment of 2026-10-31).
- Any change to the store or the daemon's protocol: `posix_word` is a
  word in a refusal, not a protocol change.

## Rules
`TEMPLATE.md`. What differs:
- You work in the main checkout
  (`/Users/tom/Development/GitHub/smallcatlabs/acquisition`), on `main`,
  never a branch, with your scratch in a `words/` subdirectory of your
  session's own scratchpad. No other build runs during the step.
  Commit; never push.
- The Windows-only code is linted by the gate's Windows-target clippy
  stage and run only in CI. Say so in the report; don't claim a Windows
  result.
- Every `acq` you run is in a mock world (the mock-session skill).
- The step changes product code, so the orchestrating session reviews it.

## Report
`TEMPLATE.md`.
