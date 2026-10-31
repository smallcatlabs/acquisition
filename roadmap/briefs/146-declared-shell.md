# 146 — a declared shell for printed commands, and their words beside them in JSON

## Goal
"Pull-forward" item 1 (`roadmap/README.md`), at the owner's word of
2026-10-31 ("i want to address 146 immediately"). Today C156 (a printed
command is quoted for the shell it will be pasted into) quotes for a
POSIX shell, or PowerShell 7.3 or newer on Windows outside Git Bash,
and refuses the two shells Windows ships, Windows PowerShell 5.1 and
cmd.exe, with a hint to install a third. When this step is done:
- detection still decides for a user who sets nothing;
- `ACQ_SHELL` declares `posix`, `pwsh`, `powershell5` or `cmd`, and
  every printed command, the daemon's one included, is quoted for it;
- a JSON field whose value is one printed command carries the command's
  words beside it, unquoted, so an agent, the GUI to come or a script
  runs it with no shell;
- the refusal for another shell's leftovers names the declaration in
  that shell's own syntax, not an install.

Why now: before the league seat, which then checks the words in their
final shape, and before `alpha.2` ships them to Windows users.

## Read
- `AGENTS.md`; `decisions/frontends.md`: C156 (the ruling this step
  amends, Do 1), C53 (JSON is the contract, text a function of it),
  C146 (a JSON field's name is the contract: additive only), C116 (no id
  in user-facing text), C88 (the provider knob checked once at a
  frontend's construction, and an unknown word refused: the model for an
  unknown `ACQ_SHELL`). `AGENTS.md`: C1 (the daemon never links the
  search, which keeps the two quoting functions apart), C122 (a wording
  is one golden; every test process takes the harness's one isolation
  list), C153 (a finding joins the surface that holds it).
- #146, the issue, its proposal and done-when; the two reviews of step
  `words` that led to C156: `roadmap/reviews/words-fd5f3d1f.md` (the
  5.1 pipe's `Caf?`, the stripped double quotes) and
  `roadmap/reviews/words-fixes-ee0b6397.md` (the daemon quotes for its
  own start shell, not the requester's; the parser's refusal; the `--`
  claim). `roadmap/briefs/words.md`, #16's part.
- The quoting: `shell_quoted`, `powershell`, `posix_quoted` and
  `powershell_quoted` in `crates/acquisition-search/src/answer.rs`
  (around lines 3193 to 3260); the daemon's twin `shell_word`,
  `posix_word` and `powershell_word` in
  `crates/acquisition-daemon/src/daemon/login.rs` (around 766 to 820),
  used only for the logout retry's username where the index holds no
  uuid (around line 702).
- The printed-command builders: `opening`, `command`, `command_with`,
  `Next::command`, `Route::command`, `Request::command` and
  `tabs_pointer` in `answer.rs` (around 2850 to 3065);
  `crates/acquisition-search/src/error.rs` (`Offered::reading`, around
  183); `crates/acquisition-plan/src/pricing.rs` (`not_on_record` and
  the `quote` it is handed, around 280 to 360 and 750);
  `crates/acquisition-cli/src/plan_cmd.rs` (`policy_set_line`, around
  97); the CLI's `Terminal` surface in
  `crates/acquisition-cli/src/main.rs` (around 1140 to 1240, a receipt's
  `undo`); and every other caller in "Input".
- The refusal: `pasted_shell_hint` and `shell_hint` in
  `crates/acquisition-cli/src/main.rs` (around 1385 to 1420), called
  from the parser's refusal (around 995) and after a failed command;
  its pin `c156_a_refusal_names_the_windows_shell_a_pasted_command_met`
  (around 2972).
- The harness: `printed`, `offline_printed`, `offline_printed_in_sh`,
  `sh_printed` and the two `shell_printed` in
  `crates/acquisition-cli/tests/harness/mod.rs` (around 70 to 140);
  `isolation` in `crates/acquisition-client/tests/harness/mod.rs`
  (around 160), which removes every `ACQ_` variable, so a developer's
  `ACQ_SHELL` reaches no test unless the test sets it.
- The Windows gate: `.github/workflows/gate-windows.yml`, its pwsh
  version step and the comment that names the shells the tests run.
- `README.md`: the Windows bullet (around lines 77 to 85) and the knob
  table (around 253 to 268). `CLI-REFERENCE.md` and `MCP-REFERENCE.md`
  for any help text that names the shell.
- `REVIEW-CHECKLIST.md`, the frontends' rows.

## Input
The last push's CI, read 2026-10-31 at `ba2b2660`: `Gate (Windows)` run
41201511590 and `Gate (Linux)` run 41201511540, both success.

The amendment to C156, the owner's ruling, Do 1. The draft below
passes `python3 -I tools/bothy.py check` (run with it in place in
`decisions/frontends.md`, the file restored from a copy after). Its
*Pinned:* names today's pins; the build adds its own.

> - **C156 — A printed command is quoted for the shell it will be pasted into: the shell `ACQ_SHELL` declares, else the one the environment says.** `ACQ_SHELL` declares `posix`, `pwsh` (PowerShell 7.3 or newer), `powershell5` (Windows PowerShell 5.1) or `cmd` (cmd.exe), on any platform; a frontend refuses any other value, never reading it as unset. Undeclared, the shell is PowerShell 7.3 or newer on Windows outside Git Bash, which sets `MSYSTEM`, and a POSIX shell elsewhere; the binary's platform alone never decides. A JSON field whose value is one printed command stands beside what a program runs with no shell: the command's words, unquoted, or the request it spells. A refusal meeting what another shell left of a printed command names the declaration for that shell. No word is piped to dodge a shell. Under the owner's word. *Why:* detection cannot tell the two shells Windows ships from PowerShell 7 and a declaration can, while words a program runs need no quoting at all (a pipe for 5.1 corrupted every character past ASCII, the review of step words). *Pinned:* `acquisition-search/tests/answer.rs` (`a_printed_word_is_quoted_for_the_platforms_shell`), `acquisition-cli/tests/search_json.rs` (the Git Bash route through `sh`), `acquisition-cli/tests/plan_json.rs` (a non-ASCII undo through the declared shell), `acquisition-cli/src/main.rs` (`c156_`).

The rules for the two new shells, unverified, reasoned from Microsoft's
documentation (#146 says so) and from the C runtime's argument parse,
which Rust's `std::env::args` follows on Windows. They are what Do 2
runs before any code relies on them:
- **Windows PowerShell 5.1** passes a native argument as its string,
  wrapped in double quotes when it holds whitespace, an inner double
  quote not escaped. So a word is PowerShell-quoted as for 7.3, with
  each inner `"` written `\"` and the backslashes before it doubled;
  a word ending in a backslash that 5.1 will wrap has that run doubled
  too. An empty word, `''`, is dropped by 5.1: it is printed `'""'`.
  Non-ASCII reaches the program whole: an argument is UTF-16 to
  `CreateProcessW`, never the pipe's ASCII.
- **cmd.exe** reads no single quote. A word is double-quoted by the C
  runtime's rules (an inner `"` as `\"`, the backslashes before a quote
  or the closing quote doubled); then, because an escaped quote flips
  cmd's own quote state and exposes `&|<>^()`, every cmd metacharacter
  in the line, the quotes included, is escaped with `^`, so cmd never
  enters a quote state. `%` is the open risk: interactive cmd and
  `cmd /c` leave `%name%` alone only where no variable `name` exists,
  and mod text carries `%` (`#% increased`): a caret inside the pair
  (`%^name%`) is the reading to test. `!` matters only under delayed
  expansion, off by default.

The census of printed commands that reach JSON, read at `ba2b2660` (a
subagent's sweep, its main claims spot-checked by the brief's writer).
The class is measured by two greps, rerun by the build before Do 4 and
at its close:

```sh
grep -rn -E 'shell_quoted|shell_word|posix_word|powershell_word|policy_set_line|command_with\(|opening\(' crates --include='*.rs' | grep -v '/tests/'
grep -rn -E '(`|: |"|^\s+)acq [a-z-]' crates --include='*.rs' | grep -v '/tests/' | grep -v -E '^\S+:[0-9]+:\s*(//|#\[)'
```

The finding that shapes Do 4: **a search answer's own JSON holds no
printed command.** `next`, every count's route, the zero answer's
offers, `resolved.rest` and `removed` are serialized as request objects
(`answer.rs` around 1294 to 1306, 1450 and 1561; the comment at 1445:
the token is never a second copy in the JSON). `Next::command`,
`Route::command`, `tabs_pointer` and `show_command` are text only.

**List A, a field whose whole value is one command:**

| | Field | Surface | Built at | The MCP's same field |
| --- | --- | --- | --- | --- |
| A1 | `undo.request`, the `price set` and `price clear` receipts | CLI | `acquisition-cli/src/main.rs` around 1197 to 1228 (the `Terminal` surface) | tool arguments and `tool` |
| A2 | `undo.request`, the `policy set` receipt | CLI | `main.rs` around 1229, through `policy_set_line` | tool arguments and `tool` |
| A3 | `offers[].request`, an account that cannot be picked, a command repeated from its parse | CLI | `main.rs` around 1160 (`parsed_line`) | the call's arguments, `account` swapped |
| A4 | `offers[].request`, `price show` and `price list` not found (`tabs`, `price show`, `price status`) | CLI | `main.rs` around 1172 to 1194 | tool arguments and `tool` |
| A5 | `readings[]`, a search refusal of kind `basis_changed` or `not_an_item` | both | `acquisition-search/src/error.rs` around 183 (`request.command()`) | the same shell command |
| A6 | `offers[].says` in the same refusals | both | the same | the same shell command |

A5 and A6 stand beside `offers[].request`, already a request object a
program sends; A1 to A4 have no structured twin on the CLI. Every
offer and receipt passes through one function,
`acquisition-verbs/src/refusal.rs`'s `offer` (around 218), which
inserts `request`.

**List B, a command inside a sentence** (`error`, `next`): `pricing.rs`'s
`not_on_record`, `read_back`, `standing`, `refused_write` and
`on_conflict` (both surfaces); the no-policy refusal through
`policy_set_line` (CLI); `acquisition-verbs/src/frontend.rs`'s
`start_command` (around 291, a fault's `next` and an absent job's
`next`, both surfaces); the daemon's logout retry (CLI). Quoted for the
declared shell; no array.

The tests that assert on list A, each touched by Do 4: in
`acquisition-cli/tests/price_json.rs`,
`every_offer_a_refusal_prints_runs_as_printed`,
`a_folders_child_asked_as_a_substash_is_offered_what_runs`,
`a_refused_command_is_repeated_from_its_parse_and_one_that_read_stdin_is_not`,
`a_route_offered_as_another_account_keeps_its_count_at_a_terminal`,
`a_continuation_offered_as_another_account_restarts_at_a_terminal`,
`a_request_offered_again_keeps_the_command_lines_routes` and
`every_json_receipts_undo_runs_as_printed_and_its_revision_is_the_next_writes`;
in `plan_json.rs`,
`a_policy_receipts_undo_runs_as_printed_and_a_first_write_says_why_it_has_none`;
in `search_json.rs`, `rule_5_every_command_printed_runs_with_a_second_account_known`;
in `acquisition-verbs/tests/documents.rs`,
`every_refusal_is_one_shape_from_one_function` and
`a_not_found_offers_what_would_find_it_in_the_scope_it_carries`; in
`acquisition-search/tests/offers.rs`,
`a_changed_basis_offers_the_request_from_its_start`; the MCP's
`spoken.rs` `PAYLOAD` list, duplicated in
`acquisition-mcp/tests/harness/mod.rs` (around 337).

A finding of the read, for Do 3: the twins' POSIX bare sets differ.
`posix_word` (the daemon's) leaves `,%+` bare and `posix_quoted` (the
search's) quotes them, so "the same rules held by the same cases"
(`login.rs`, the twin's doc) holds only over the cases each table
happens to list: `a,b` is `'a,b'` in one and `a,b` in the other, both
correct for a POSIX shell. The two tables are copies.

## Do
1. **The ruling.** The amended C156 above goes to the owner with a
   recommendation, before any code, with the four sub-choices it makes
   named apart: the knob is the variable alone, no `--shell`; the
   values' spellings; an unknown value refused; the daemon's word
   follows the `ACQ_SHELL` it was started with. Landed in
   `decisions/frontends.md` in its own commit, his word quoted in
   `roadmap/README.md`'s rulings list, before Do 2.
2. **The probe, on the Windows gate, before the quoting relies on it.**
   A Windows-only test that runs a fixed word list through
   `powershell.exe` (a `-File` script, as `shell_printed` does for
   pwsh) and through `cmd.exe /d /s /c` with the line passed raw
   (`CommandExt::raw_arg`, so no second quoting stands between), into a
   program that prints its `std::env::args` as JSON (the test binary
   itself, or `acq`'s own refusal of an unknown word, whichever is
   simpler), and compares each word with the original. The words:
   plain, a space, `'`, a typographic `’`, `"`, a JSON object with
   quotes and spaces, a trailing backslash with and without a space, a
   backslash before a quote, the empty word, `Café` and `日本`, `%PATH%`,
   `#% increased`, `&|<>^()!`, `@x`, `a,b`, a leading `-`, `0123`,
   `1e3`. The test is the quoting's own table (C153: one table, not a
   probe file and a pin file), written first with the rules in
   "Input", run on the Windows gate (a push at the owner's word, or the
   `acqtest` VM), and its result recorded in the commit message: each
   rule that held, each that did not and what replaced it. A word no
   rule carries through a shell is named, and the command that would
   print it says so instead of printing a command that does not run.
3. **The quoting.** A `Shell` (posix, pwsh, powershell5, cmd) and one
   function per shell; `shell_quoted` and the daemon's `shell_word`
   take the declared shell, else detection, unchanged. The twins' cases
   become one table both test files read (a fixture file under the
   search crate's tests, `include_str!` from the daemon's), so a case
   added to one is a case of both; the bare-set difference above is
   resolved to one set or kept and named in the table. The declared
   value is checked once where C88 checks the provider word
   (`acquisition-verbs/src/frontend.rs`), refused in words naming the
   four values; the daemon, which a frontend spawns with its
   environment, reads it and falls back to detection on a value it does
   not know.
4. **The words beside the command.** The builders of list A make the
   command as words first and render it for the shell after, so the
   string and the array cannot disagree; A1 to A4 gain the array beside
   `request`, through `refusal.rs`'s `offer`, additive (C146), its name
   the step's agent-facing design, recorded in the commit and listed at
   the close. A5 and A6 already stand beside a request object a program
   sends; the build says whether they also take the array or the
   request is their machine form, and the close lists it. A command
   inside a sentence (list B) and every text-only command stay quoted
   text.
5. **The refusal.** `shell_hint` fires where the shell is pwsh (declared
   or detected) and names the declaration in the shell it saw: `set
   ACQ_SHELL=cmd` for cmd.exe's leftovers, `$env:ACQ_SHELL =
   'powershell5'` for 5.1's, beside PowerShell 7.3; its golden and the
   `c156_` pin follow.
6. **The harness and the Windows gate.** `printed` runs a command
   through the shell `ACQ_SHELL` names on the child: `powershell.exe`
   and `cmd.exe` beside `pwsh` and `sh`; a non-ASCII policy undo (the
   `plan_json.rs` case) runs through each of the four on Windows. The
   workflow's comment names the shells the tests run.
7. **The words a user meets.** The README's Windows bullet names the
   declaration for 5.1 and cmd.exe; a knob row for `ACQ_SHELL`; any help
   text that names the shell, regenerated into the references under
   `ACQ_UPDATE_FIXTURES=1`.
8. The gate, then the review, as "A step" says.

## Done when
- C156 in `decisions/frontends.md` is the owner's amended text, landed
  in a commit before the first code commit, and the rulings list
  quotes his word.
- A Windows-only test runs every word of Do 2's list through
  `powershell.exe` and `cmd.exe` and compares what the program received;
  the commit that adds it names the CI run (or the VM) it passed on, and
  every rule of "Input" it overturned.
- `shell_quoted` and `shell_word` take the declared shell; one case
  table, read by both crates' tests, holds every shell's words.
- An unknown `ACQ_SHELL` is refused by the CLI and the MCP server with
  the four values named, pinned through the binary.
- Every field of the census's list A carries its words beside it in
  JSON, pinned by a test that runs the array with no shell and the
  string through the declared shell and finds the same result; A5 and
  A6 may instead stand on the request object beside them, as Do 4
  allows. Settled in the round: the review of the build
  (`reviews/146-0f6515da.md`) found the line narrower than Do 4.
- The refusal names the declaration for the shell it saw; its golden
  holds the words.
- On Windows the harness runs a printed non-ASCII policy undo through
  `pwsh`, `powershell.exe`, `cmd.exe` and `sh`, and the policy read back
  is the one written.
- The README's Windows bullet and knob table name `ACQ_SHELL`;
  `readme_tour.rs` passes.
- `tools/gate.sh` green on the step's tree, and both CI gates green on
  the pushed build.

## Not in scope
- A `--shell` flag, or a declaration in a file: the variable is the one
  knob (Do 1); a flag would be a second channel to keep in step.
- fish, nushell, Elvish and other shells: POSIX quoting is what they
  get, as today; a user's report is the trigger (`trigger:user`).
- Detecting 5.1 or cmd.exe from the environment: #146 records that
  detection cannot tell them, and a guess that is wrong prints a
  command that does not run.
- A command inside a sentence gaining an array: list B stays text.
- The logout retry's username fallback beyond Do 3: the uuid is printed
  wherever the index holds one (none without, since 2026-08-31, the
  words close).
- The league seat's checks: the seat's, after this step.

## Rules
`TEMPLATE.md`. What differs for this step:
- The step runs in the main tree, one Opus session, no parallel worker:
  the census's builders touch the search, the plan, the verbs, the CLI,
  the MCP and the daemon at once.
- Every `acq` the session runs is the mock world's (the mock-session
  skill); a shell probe runs in a scratch directory, never against a
  store.
- A push for Do 2's Windows run is the owner's word (`AGENTS.md`, "Hard
  rules": a commit or push waits for it).
These bind the session's own work.

## Report
`TEMPLATE.md`.
