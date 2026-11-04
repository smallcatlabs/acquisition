# seat-issues-a — the search's three seat issues

Round A of the answer-facts seat's issues (`audits/answer-facts-2026-11/seat/`,
the issues it filed on 2026-11-04), taken at the owner's word of
2026-11-04 ("Start round A") after his triage of the open issues: four
rounds by area, A the search (#174, #173, #36), B pricing, C the store,
D jobs. Written by the orchestrating session against the tree at
`20a16d25`. No new ruling: C146 (a count says what it counts and what
the reader can do next) and C143 (a refusal speaks the request's own
words). One commit per issue, a test named for each; reviewed cold,
not sat again (the seat's third round's precedent).

The seat's calls are in the worlds under
`~/seat-worlds/answer-facts-2026-11/` (`cli-opus/calls/NNN.txt` for
CLI run 2, `mcp-opus/calls/NNN.json` and `NNN.args.json` for MCP run
2); `audits/answer-facts-2026-11/seat/calls-*-2.csv` index them. Read
the calls each issue cites before its rule.

## The issues taken

1. **#174, a refusal of a bare flag word offers no `has:` form**
   (CLI run 2 fault 10; calls 66 to 68). `-priced` is refused by
   `bare_word` (`crates/acquisition-search/src/parse.rs`, about line
   645) with `name:priced`, `"priced"` and `line(template:priced)`;
   `-is:priced` by the `is:` arm in `bind.rs` (about line 1126) with
   every item flag; `has:priced` was found only in `--describe=priced`.
   The language already offers the form where a flag field is asked
   with an operator: `Kind::Presence` in `bind.rs` (about line 1339)
   reads `has:X`, `-has:X`, `undecided(X)`. Rule: a word that names a
   field `has:` takes is offered as `has:<word>` first, in both
   refusals, the existing offers after it; under a minus the reader
   typed, the offer carries it where the parser knows it (`-has:priced`).
   One source for "a field `has:` takes" (the field table and its
   `Thing`, as `has:`'s own binding decides it), never a second list.
   Pin: the refusal's golden (the two calls' queries), `refusal.rs` or
   `seat_faults.rs`, whichever holds the bare-word refusal's wording
   now (C153).
2. **#173, a counts or cross view over `league` omits the leagues the
   scope holds with no items, and does not say so** (MCP run 2 fault
   11; call 38: three buckets, `scope.leagues` eight). A bucket is a
   value an item holds; a league with no live item has none. Rule: a
   table keyed by `league` (a counts table, or either key of a cross)
   says its buckets are the items' and names the scope's leagues with
   no bucket, in JSON as a field beside `values` and in text as one
   line after the table, on both surfaces; none where every scope
   league has a bucket. The counts module does not see the scope; the
   answer does (`answer.rs`), so the field is filled where the scope
   and the view meet. A `realm: all` table splits a league by realm
   (call 38: `Standard` twice); a league counts as having a bucket if
   any realm's does. Pin: the view's golden, a scope with a league
   that holds nothing (the test world in `answer.rs` or `counts.rs`
   already lists an empty league: `list_tabs(…, "Allflame", json!([]), …)`).
3. **#36, the scope's `tabs` pointer under two `league=` terms**
   (re-sit; confirmed from the code on the issue at `20a16d25`).
   `tabs_pointer` (`answer.rs`, about line 3003) takes its league from
   `fixed_league`, which answers only one `league=` and-ed into the
   tree; under two terms (`league=A league=B`, `league=A or league=B`)
   it falls to the realm's league with the most never-fetched tabs,
   which may be neither. Rule: the pointer names a league the query
   names where it names any: of the leagues named by `league=` terms
   not under a negation, the first in `never_fetched_leagues`' order
   that holds a never-fetched tab, `other_leagues` counting the rest of
   the named ones that hold one; none where none of them does; the
   realm's most where the query names none, as now. `fixed_league`
   keeps its meaning for the scope's list age (one league, or none).
   Pin: `answer.rs` beside
   `the_scopes_list_age_under_a_query_fixing_one_league_is_that_leagues`,
   the `or` and the and-ed cases and a negated term.

## Done when

Each of the three has its commit, its message naming this brief and
the issue (`Closes #N` is not written: the orchestrator closes them),
and its pin; every regenerated line is one of the three's words or a
`# holds` line; both references regenerated where help or a tool's
text changed; `tools/gate.sh` green, run bare at the end.

## Not in scope

Rounds B to D; any other wording the reads turn up (report it); the
cross view's cell limit; `fixed_league`'s use for the list age.

## Rules

`TEMPLATE.md`; the main tree, no other worker building; scratch the
subdirectory the launch names; `ACQ_UPDATE_FIXTURES=1` once after
the holders are moved; no push; no issue opened, closed or commented.

## Report

`TEMPLATE.md`, one paragraph per issue: the rule as built, the pin,
what a reader now meets (one line of output each), and anything raised.
