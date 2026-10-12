# Test read — frontends (`acquisition-cli`, `acquisition-mcp`)

114 rows read from source, one per inventory row (`data/read-frontends.tsv`); nothing was run.
Classes: boundary 108 · weak 4 · mechanism 1 · harness 1 · cannot-fail 0 · duplicate 0.
Confidence: high 102 · medium 12 · low 0. No row was answered from a test's name alone.
No test in the group skips: a missing `acqd`, `acq` or fixture panics (`tests/harness/mod.rs:152`, `acquisition-mcp/tests/harness/mod.rs:71`, `src/real_scale_fixture.rs:21`).
The two golden references fail on any help or schema change with `ACQ_UPDATE_FIXTURES` unset; set, they rewrite first and can fail only on their sanity line.
`data/cases-frontends.tsv`: one generated test (6,733 cases, no knob); five more rows are scale or race counts, labelled as not generated.

## Weak (4)

- `crates/acquisition-cli/src/main.rs:2095` `the_daemons_parent_failure_line_is_read_back_whole` — parses the test's own literal (`main.rs:2099`), not the daemon's format string (`acquisition-daemon/src/daemon.rs:2071`). A daemon that rewords the line passes, and `report_apply` silently falls back to the raw error. The parser's doc comment says this test "pins the coupling"; it pins one side.
- `crates/acquisition-cli/src/plan_cmd.rs:2115` `the_expanded_form_keeps_one_line_per_action_and_the_json_is_untouched` — the text half is held; nothing in the body (`plan_cmd.rs:2115-2157`) reads the plan's JSON, so an `--expand` that changed the `--json` document passes.
- `crates/acquisition-mcp/tests/ggg_refusal.rs:16` `ggg_mode_never_spawns_a_daemon_for_the_spending_tools` — `submit_job`'s half is real. `apply_plan` is sent a string for a plan (`ggg_refusal.rs:45`) and is refused at `main.rs:877-878`, before `connect` at `:891`: an `apply_plan` that spawned in real mode passes. Both tools share `connect` today, which is what actually holds it. C13 names this file as its pin.
- `crates/acquisition-mcp/tests/search_tools.rs:890` `describe_is_as_large_as_its_text_says` — the sizes are the test's own constants (`search_tools.rs:898-911`); the description's numbers are never read, only its section names. A text stating other sizes passes. It does fail when the language outgrows the constants.

## Mechanism (1)

- `crates/acquisition-cli/src/store_cmd.rs:800` `events_group_by_location_newest_first_with_names` — asserts on the private summary rows' fields (`store_cmd.rs:820-828`), not on `acq store events` output; one assertion reaches text (`counts()`). "Newest first" is checked for the first row only.

## Boundary rows worth a second look (all medium)

- `crates/acquisition-cli/tests/price_story.rs:233` and `:361` — the two race tests. A lost update or a torn page shows only in rounds where the two processes overlap, which the tests leave to the machine and print rather than assert (`:293`, `:410`). With no overlap they are sequential tests of the same receipts. `assert!(landed >= 5)` at `:292` is implied by `:253`.
- `crates/acquisition-cli/src/price_cmd.rs:1692` and `src/shop_cmd.rs:637` — the "linear" guards are wall-clock cliffs (5 s, 2 s in a debug build) plus counts pinned to the real-scale fixture; a slow machine or a fixture edit fails them as a product change does.
- `crates/acquisition-cli/tests/price_json.rs:163` — text and JSON are each compared with literals, never one rendered from the other; "the views the text reads" is held at a few lines.
- `crates/acquisition-cli/src/plan_cmd.rs:2018`, `:2047` — assert on the private `common_action_reason`'s exact strings; classed boundary because that string is the line printed (C53).
- `crates/acquisition-cli/tests/daemon_stop_checkpoint.rs:108` — the boundary is the files other tools read; no decision id is cited.

## Environment, not product

- `crates/acquisition-cli/src/plan_cmd.rs:1843` binds its wedged listener in the real runtime directory (`:1856`), not a scratch one.
- `crates/acquisition-cli/tests/world.rs:254` starts real-mode daemons (`ACQ_MAX_SENDS=0`) and contends with a live daemon of the owner's, as its module doc says.
- `crates/acquisition-cli/tests/search_json.rs:1240` compares `--describe`'s field list whole (`:1253`), so an additive field fails it; C53 calls JSON changes additive.
