# 91 — frontend hygiene, the clock flake, and the feature split CI cannot see

Issue #91, the first item of "Pull-forward" (`roadmap/README.md`),
with the owner's word of 2026-10-29: "yes, go ahead on #91". Two items
join it the same day. #109 comes first (the owner: "Agreed on fixing 109
first"). Then #124's ratatui 0.30 lands with its pin, and #128's item 2
becomes a CI check of the two feature graphs (the owner: the cheap
check first, cargo-hakari later on the triggers in #128's comment).
The rhythm is "A step", and the base is `1957f715`, which is
`origin/main`.

## Goal

A test that compares a row read twice no longer fails when the second
rolls over between the reads. `acq` built by a test run is the `acq`
the plain build writes, with ratatui 0.30 in it. A Dependabot PR that
splits a binary's feature graph goes red on its own checks. The five
small findings of the 2026-10-26 read each have one home. These are
the bugs and duplicates a frontend's next change would otherwise
step on.

## Read

- `AGENTS.md`: C1 (the edges), C47 (malformed external input is a
  structured error), C122 (a behaviour pinned once, at the lowest
  crate).
- `decisions/frontends.md`: C53 (ages are text beside their epoch,
  made at the answer's own time), C143 (a shared refusal speaks the
  request's words), C116 (no id in user-facing text).
- `decisions/trade.md`, C113 (K6): "`--realm` beside a URL is a
  conflict".
- `REVIEW-CHECKLIST.md`, the frontends section, and "A count bounded
  by a whole-second clock" (the shape #109 names).
- `tools/gate.sh`: its header and the `artifacts` stage, which hashes
  `acq`, `acqd` and `acq-mcp`, runs `cargo build --workspace` and
  fails if anything moved (C84, the artifact mismatch). CI does not
  run it.
- `crates/acquisition-mcp/Cargo.toml`: the two feature pins in the
  form this step copies (`num-traits` #88, `getrandom` #125).
- `crates/acquisition-mcp/tests/pricing_tools.rs`: `steady` (line
  144), which strips every `_age` key; line 926, which removes one by
  hand.
- `crates/acquisition-verbs/src/events.rs`, `Events::read`: the hours
  conversion both frontends already call (moved there at `79476501`,
  after the read). `crates/acquisition-store/src/lib.rs`:
  `refused_list`, `events_since` and `events_after`, each binding
  `limit as i64`.
- `crates/acquisition-verbs/src/refusal.rs`, `Arguments`: the
  `arguments` kind.
- `crates/acquisition-cli/src/search_cmd.rs`: the two CLI-only
  refusals (`--realm` beside a URL, an unknown `--membership`).
  `crates/acquisition-search/src/answer.rs`: `Membership`, and
  `QueryInput::read`'s realm refusal.
- `crates/acquisition-cli/src/web/form.rs:49`, and the capture's row
  in `search/trade-query/MANIFEST.md`.
- The tests that assert the words this step changes, found by
  grepping the test trees for the old words:
  `crates/acquisition-cli/tests/search_json.rs`, `--membership gone`
  (lines 2031–2042 and 2115–2119) and
  `k6_a_realm_beside_a_url_conflicts_and_an_id_takes_one` (2410). No
  golden or reference holds either wording.

## Input

Measured on `1957f715` on 2026-10-29:

- #109 failed on CI three times: Linux run 40895700194 on `main`,
  Windows run 40910593112 on PR #122, Windows run 40919534307 on
  `main`. The comparisons at `pricing_tools.rs:625` and `:639` compare
  `price_show`'s `listing.manual`, `updated_at_age` included, read
  before and after.
- A read of the MCP, CLI and verbs test trees found only that test at
  second-level risk. Five more comparisons set a whole answer read on
  one surface against another without stripping ages, with fixture
  epochs near 1970, so they can flip only in the one second a day an
  age's day count rolls over: `acquisition-mcp/tests/search_tools.rs`
  (`search_beside_the_cli`, line 223; the `tabs` comparisons at
  898–938, 1011 and 1038); `acquisition-cli/tests/search_json.rs:1016`
  and `:2376`; `acquisition-cli/tests/web.rs:358`. CLI goldens mask
  ages through the harness's `age_rule`.
- #124 (its branch `dependabot/cargo/ratatui-0.30.2`, rebased by
  Dependabot onto `1957f715`):
  ratatui 0.29 → 0.30.2 in `crates/acquisition-cli/Cargo.toml` and
  `Cargo.lock`. Compared package by package (name and version) on this
  Mac's host target, `cargo tree --workspace -e no-dev,features` and
  `-e features` agree on `main` (230 and 245 packages, no feature set
  differing). On #124's tree they name one package: `rustix 1.1.4`,
  build `alloc,std,stdio,termios`, test `alloc,default,fs,std,stdio,termios`.
  With `rustix = { version = "1", features = ["fs"] }` under
  `[target.'cfg(unix)'.dependencies]` they agree again, and the lock
  gains one line. Each tree takes 0.3 s and builds nothing.
- The same comparison by `--target`, on `main`: clean for
  `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`
  and `aarch64-pc-windows-msvc`. Not clean for `x86_64-unknown-linux-gnu`
  or `aarch64-unknown-linux-gnu`: `rustix 1.1.4` takes `default` only
  in a test build (tempfile's dev-dependency asks it), and on Linux
  rustix reaches `acquisition-daemon` through keyring, secret-service,
  zbus, and async-io and polling. A scratch crate showed that a feature
  list differing by `default` alone, `std` already on, compiles to
  another rlib and another binary. So on Linux a test run writes
  another `acqd`: the second hakari trigger in #128's comment.
- The five `plural` copies: `search_cmd.rs:690`, `price_cmd.rs:128`,
  `shop_cmd.rs:95`, `plan_cmd.rs:614`, `web/page.rs:441`, identical,
  all in `acquisition-cli`. The cancel document
  `{"job_id", "cancel_requested": true}` is written in the CLI's
  `main.rs` (`Cmd::Cancel`) and the MCP's `cancel_job`. The 24-hour
  default is written twice: the CLI's `default_value_t = 24.0` and the
  MCP's `unwrap_or(24.0)`.

## Do

1. **The ruling the step needs.** The CI check (Do 4) cannot be clean
   on today's `main` for the Linux targets without a daemon pin or
   cargo-hakari, and either is the owner's call on his recorded
   trigger. Raised to the orchestrating session with a
   recommendation: the check covers the macOS and Windows targets,
   Linux named as unchecked, and an issue carries the trigger. The
   check's commit waits for the answer. The other concerns do not
   depend on it.
   - 2026-10-29, the orchestrating session under the owner's
     delegation ("If the decision is clear cut, you may take the
     recommendation"): (c), the check over the targets clean today,
     the Linux targets on a list of unchecked targets with their
     reason, and the evidence on #128 as a comment, with no new issue.
     Settled in the build: #124's pin in `acquisition-cli` (Do 3)
     closes the Linux split too, because the workspace build unifies
     rustix's features across `acq` and `acqd`. So the list came out
     empty and was not built, and the check covers all five release
     targets. A second comment on #128 says so.
   - 2026-10-29, the owner, after a comparison of the pins against
     cargo-hakari: "yes, agree with keeping the pins and cleaning them
     up." Settled in the fix round: the three pins (`num-traits` #88,
     `getrandom` #125, `rustix` here) stand in one commented block in
     `crates/acquisition-cli/Cargo.toml`, each with the dev-dependency
     that asks for its features and the binaries that rely on it, and
     the `features` stage, which checks them, names that block. #128's
     hakari triggers are replaced by a comment there.
2. **#109.** One rule over the file: every comparison of a document
   read twice goes through `steady`. The blind-replace test's `shown`
   reads its row through it, and line 926's by-hand removal becomes
   the same call. The other five comparisons are reported, not
   changed, because each is another file's and flips once a day.
   Closes #109.
3. **#124 with its pin.** #124's `Cargo.lock` and manifest line as
   Dependabot wrote them, plus the Unix-target `rustix` pin in
   `acquisition-cli` with a comment in the existing pins' form. The
   proof is measured, not assumed: the full gate's `artifacts` stage
   fails without the pin and passes with it.
4. **The feature-graph check.** A gate stage, `features`, that
   compares the two graphs package by package for each target it
   names and fails naming each package and both feature sets. CI's
   Linux job runs it as a step. The `artifacts` stage's failure names
   the stage, closing #128's item 3 by pointing at the packages. Its
   targets are the release's (settled by Do 1). Shown red on `main`'s
   Linux targets and on #124's tree without the pin, and clean on the
   step's tree.
5. **The bound.** `Events::read` refuses an `hours` that is negative
   or not a number, as `arguments`, in the request's word `hours`, and
   subtracts with saturation. The store binds a `limit` past `i64::MAX`
   as `i64::MAX` through one helper at its three reads. Pinned in the
   verbs and store crates.
6. **One home for each duplicate.** The 24-hour default is one
   constant in the verbs crate, read by both frontends. The cancel
   document is one verbs function both call. `plural` is one function
   of the CLI crate.
7. **The two CLI-only refusals (C143).** The membership's words move
   to the search crate, so `Membership::parse` and its `Deserialize`
   say the same words to the CLI flag and the MCP request. A `--realm`
   naming another realm than the URL's is refused by the search
   crate's shared words, on every surface. A `--realm` naming the
   URL's own realm stays the terminal's K6 conflict, the one case the
   request has no field for, worded after the shared words.
8. **The form's dump.** The filters capture is shipped from
   `crates/acquisition-search/reference/trade-filters-2026-09-12.json`.
   It is a verbatim copy of the research capture, its generator named
   in the search crate's doc beside its `include_str!`: the owner's
   browser capture of 2026-09-12, through `search/trade-query/MANIFEST.md`'s
   row. A test holds it to the manifest's sha256. The CLI's form and
   the search crate's catalogue test read the shipped copy. The
   research scripts keep reading the research copy.

## Done when

- `pricing_tools.rs` compares no `price_show` document read twice
  except through `steady`, and #109's commit says "Closes #109".
- `Cargo.lock` holds ratatui 0.30.2, `acquisition-cli` pins `rustix`
  under `cfg(unix)` with a comment, and the report holds the
  `artifacts` stage's status without the pin (failing) and with it
  (passing).
- The `features` stage is in `tools/gate.sh` and a step of
  `gate-linux.yml`, over the release's five targets; the report shows
  it clean on the step's tree and naming `rustix` on #124's tree
  without the pin.
- `Events::read` refuses `hours` below zero and not a number, with a
  test in the verbs crate. The store's three limit binds go through
  one helper, with a test of a `usize::MAX` limit.
- `fn plural` stands once in `acquisition-cli`, the cancel document
  once in `acquisition-verbs`, and the 24-hour default once.
- An unknown membership's words come from the search crate on both
  surfaces, and a mismatched realm beside a URL is the search crate's
  refusal on the CLI, with the tests in Read updated to the words.
- No `include_str!` in a crate's `src/` names a path under `search/`
  (a test may still read the research captures it checks against;
  corrected from "in `crates/`", which the catalogue test's stats and
  grammar captures make false).
- `tools/gate.sh` green on the step's tree.

## Not in scope

- The five once-a-day comparisons outside `pricing_tools.rs`: reported,
  with a proposed fate.
- cargo-hakari, and any pin in `acquisition-daemon`: Do 1.
- The research scripts' and generated tables' paths to the capture,
  which stay on the research copy.
- `price_set`'s or `set_priority`'s documents, unless one is the same
  duplicate the cancel is.
- The words (#11 and the rest of "Pull-forward" item 2): after the
  league build's first seat.

## Rules

`TEMPLATE.md`. Main tree, on `main`, scratch under the session's
scratchpad, `91/`. Never push. `gh` writes one call at a time. Any `acq`
runs in a mock world.

## Report

`TEMPLATE.md`.
