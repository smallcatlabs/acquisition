# h2 tail — one rule: a remedy shows what it promises

The tail of step h2, over the Codex review of the fix round, saved
verbatim at `roadmap/reviews/h2-fixes-fa3335bf.md`. The owner's word,
2026-10-26: "fix the one finding as a one-commit tail with no further
Codex review, then close h2". No review follows; the h2 close commit
follows.

## Goal

The shared refusal in `acquisition-plan/src/pricing.rs` (used by
`price show` and `price list`, on the CLI and the MCP) prints a
remedy that displays the row it names. An orphan row keeps the
`price status --expand` remedy with the account and realm. A row
that another league holds gets an account-qualified `price show`
command naming that league, since the status report of the league
the user is in counts such a row under "other league" and never
displays it.

## Read

- `roadmap/reviews/h2-fixes-fa3335bf.md`, whole; `h2-fixes.md` rule 1
  (`7c4310da`, the refusal names the account it ran as).
- `acquisition-plan/src/pricing.rs` around 244 (`not_on_record` and
  `place_other_leagues`), its two callers, and the pins in
  `acquisition-cli/tests/price_json.rs` (the two-account test and the
  one-account expected text) and the MCP's `pricing_tools.rs`.
- C146 (a count says what it counts) and C53 (three output levels).

## Input

- The review's input: Alice and Bob known; a readable price row on
  `tab/xbox/t` recorded only in Allflame; `acq price list --account
  Alice --league Standard --in tab/xbox/t`, and the same with `show`.

## Do

1. In the shared helper, tell the two cases apart: no row anywhere
   (the status remedy as now), and a row in another league (a `price
   show` command carrying `--account`, `--realm` if needed, and
   `--league <that league>`, quoted as rule 1 quotes). The refusal's
   sentence names the league the row was found in. Pin both callers
   with the input above, running the printed command as printed, as
   the two-account test does.
2. One commit, naming this brief; it deletes
   `roadmap/reviews/h2-fixes-fa3335bf.md` and cites it whole at
   `e82fa488`. Regenerate any golden the sentence changes and read it.

## Done when

- The two pins green; the printed command for the other-league case
  displays the row.
- The review file deleted and cited.
- The gate green on the tail's own tree, every stage's exit status
  shown.

## Not in scope

- Anything else in h2's leftovers; the two open parts of #11.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h2-tail/`;
every `ACQ_STORE_DIR` under it; commit on `main`, never push, the line
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` last; long
commands in the background; `cargo build --workspace` before any
daemon and after any test run (C84, the artifact mismatch). No task
list: do not call `update_plan`.

## Report

`TEMPLATE.md`, as text.
