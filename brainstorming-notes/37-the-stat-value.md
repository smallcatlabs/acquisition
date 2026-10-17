# 37 — The stat value: a bound on a stat the game spells two ways

Provenance: Fable (`claude-fable-5-1`), 2026-10-17, with the owner, the
session before step 7e of `search/TRADE-PLAN.md`. Read: `README.md`,
`CONTEXT.md`, `SEARCH-SLICE.md`, `decisions/search.md`, the brief, the
crate's `tree.rs`, `totals.rs`, `pseudo.rs`, `group.rs`, `template.rs`,
`trade/translate.rs` and its catalogue, `tests/trade-oracle.toml`, the
sitting sheet's rounds t and w, and the commits of step 7. Measured:
the stat vocabulary (`reference/trade-stats-v1.toml`), the sitting
captures (`search/pseudo-stats/data/captures.json`, `bodies.json`) and
the owner's census (`search/trade-query/data/fetch-census.json`), by
one-off scripts whose counts are in the tables below; a count here was
measured in this session and nowhere else.

This note is deliberation: it is cited by the park "A named stat value"
in `decisions/search.md` and deleted when that park is built or
dropped, cited afterwards as "note 37 at `<commit>`". The decision is
the owner's, quoted at the end; the brief carries what 7e builds.

## The question

`sum( … )` adds the numbers of the lines it selects, with no sign and no
constant, and sums nothing to zero (C95). The sitting of 2026-10-16
(rounds t to w) found that the site's bounded stat is none of those
things: it is the item's stat however displayed — a `reduced` spelling
under its `increased` id below nothing and the inverse (t01–t04), an
elided 100 % as 100 (t10), `an` as one (t12), a plural the same number
(t27), the stat's crafted and fractured lines summed with its explicit
one (t13–t20), and absent when the item has none. That is a named
total's definition (C94), and the language has no way to name one by
a text. So a bound on a stat with an opposite-sign or numberless form
has no exact text. Two ways were written in the brief:

- **A, the translation alone.** Same-sign numbered forms in the sum; a
  numberless form of a known number an alternative exactly when its
  number meets the bound; a stat of an opposite-sign form ambiguous on
  a minimum and unsupported otherwise; anything unread presence alone.
  No new tree node.
- **B, a named stat value.** `stat("#% increased Attack Speed")>=10`:
  the item's stat however displayed, from the forms and their readings,
  exact wherever a reading is known, typeable by a person as well as
  written by the import. One new value kind.

## The framing

The brief frames A against B as "translation alone" against "a new
tree node". The question sits one level up: **what is the unit of a
bounded stat?** The language's unit is the displayed line (C90, C92);
its one signed, summing unit is a named total from the table (C94).
The site's unit, as the sittings established and the owner accepted,
is the stat: every form, the opposite word below nothing, a numberless
form at its one number, summed across the sources the id admits,
lacked when none. That is a total by definition. The decision is
whether the language can name the site's total by its text (B), or
whether the translation approximates it with lines, sums, alternatives
and labels (A). Three sub-questions follow:

1. **Does the text stay honest under edit?** C113's reason for the text
   is that a person can edit it. Under A as first written, a numberless
   form becomes `or line(E)` only when its fixed number meets the
   bound, decided while the text is written: edit the bound from 100 to
   150 and the alternative is silently wrong. That is the "meaning read
   off the syntax" shape of `SEARCH-SLICE.md`'s findings, in the product
   text. Under B the text is `stat(T)<=-5` and any edit keeps its
   meaning.
2. **How many makers of the sign rule?** "The reduced spelling is its
   stat below nothing" lives twice already: the totals table's 45 rows
   at weight -1 (`totals.rs`, step 9c2) and the percentile's reading
   (`pseudo.rs`, `Does::Reduce`). A's computing rules would be a third;
   B could be the one.
3. **What does the fidelity word promise?** Under A a minimum on any
   word-signed stat is ambiguous, since an item carrying both forms
   reads high, and a maximum is unsupported. A label that is always on
   stops being read (the C93 argument, turned on the report).

## What was measured

| Measure | Count |
| --- | --- |
| Stat texts with `increased` / `reduced` / `more` / `less` | 2,455 / 152 / 173 / 42 of 10,303 ids |
| `increased` texts whose `reduced` twin is its own id | 0 (the site files the twin under the one id) |
| Bounded hash-stat clauses in the 1,150 sitting captures | 28, in 24 searches |
| Of those, on a word-signed text | 12 |
| Of those, a maximum on a word-signed text | 9 |
| Bounded hash-stat clauses in the owner's 8 searches | 0 |
| `#% chance to X` texts whose `X` is itself a listed stat | 7 of 372 |
| `Y have #% chance to Z` texts with the same collision | 2 of 286 |
| Texts carrying both `increased` and `reduced` | 4 |
| Texts carrying two sign words | 25 |

The forms themselves (7a's 72 under 71 ids, read again here over the
bodies by a script whose templating is coarser than the deriver's) are
rule-shaped with a short tail: the opposite word; `#% chance to X`
displayed as `X`; `Y have #% chance to Z` displayed as `Y Z`; `#
additional Xs` displayed as `an additional X`; a plural for a singular;
and a few the rules do not reach — `No Physical Damage` under `#%
increased Physical Damage` with a hidden number, and a line the site
files under an unrelated id (`Area contains an additional guarded
Exquisite Vaal Vessel` under `#% increased Explosive Radius`). The
collisions in the table are the cases any elision rule must refuse by
name; a test can walk the whole vocabulary for them.

Real-use frequency is unmeasured: the captures were composed to probe
pseudos and the translation, and the owner's own searches bound no
stat at all.

## The case for A

Leave the language as it is; the translator writes the best text the
existing words allow and labels the rest.

1. Nothing new to learn or maintain: no new value, definition, parser,
   printer, JSON form or generator; no new surface before the release.
2. The import stays a translation, as the brief said: the site's quirks
   stay at the boundary and never shape the language people use for
   their own stash.
3. Honest by construction: what the text cannot carry is labelled
   ambiguous or left unsupported, clause by clause.
4. The affected case looks rare, and P3 says generalise after two
   consumers; the owner's eight searches had none.
5. Reversible: nothing in A blocks B later, with evidence of what people
   type; B built and unwanted is the costlier mistake.
6. Days, not sessions, inside a calendar that holds a seat and a
   release.

Its one flaw: for those clauses the text is long and odd, and the
computed alternative bakes a decision into text an edit can break.

## The case for B

One word: `stat("…")`, the item's value for that stat however the game
displays it — the opposite spelling below nothing, a numberless line
at its one number, the lines summed, lacked when none.

1. It says what the site means: the language gains the sign it lacks.
2. The text stays short and safe to edit.
3. People and agents get it too: `stat("#% increased Attack
   Speed")<=-5` finds reduced lines without knowing the spelling trick,
   and `--describe stat` prints the rule.
4. One rule in one place, where two exist today.
5. The fidelity word keeps its meaning.
6. The outbound half, parked, becomes a bijection for bounded stats:
   one word here, one id with min and max there; PoE2 shares the model.

Its costs: a change on the owner's language boundary, so a ruling
first; a `ValueRef` variant touching some 25 match sites across
`parse`, `print`, `json`, `bind`, `eval`, `tree` and `answer`, a
describe entry, the generators, the references regenerated; two to
four sessions with review; and display rules learned from a handful of
sat searches applied across ten thousand texts — the game's own
grammar, testable against the whole vocabulary, but a generalisation.

The shape, if built: `stat(T …)` takes what `line(T …)` takes, so
`source=explicit` narrows it as the site's category does; the value is
the signed sum of the forms' numbers; a stat of nothing is lacked; a
form whose number is hidden leaves it incomplete and the comparison
undecided (C94's rule as it stands); the user's spelling is kept and
the sign is relative to it, so `stat("#% reduced X")>=5` and
`stat("#% increased X")<=-5` are one predicate, an equivalence the
generators pin (C91's one visible rule); the stat id never enters the
language and the stat table stays at the boundary (C90); exceptions
are a small reference table under C106; no `has:stat( … )` — presence
over every form is exact today as line alternatives. The owner chose
`line()` over `stat()` on 2026-09-19 because the word says what the
item displays (`group.rs`); B needs exactly that distinction: `line`
is what is displayed, `stat` is what the game means by it.

## The decision factors

1. Is "the stat as the game means it" a concept the language should
   own, or a quirk of the trade site? The site's ids are the game's
   stat ids and every community tool models the stat this way.
2. Can text that is unsafe to edit be accepted as a labelled limit?
3. How far to trust rules learned from a few sittings — both options
   need the same rules; B states them where a test can walk them, A
   applies them while writing.
4. The schedule and the owner's boundary: A ships in days with no
   ruling; B is a small slice.
5. Evidence of demand: thin; the text people carry in printed requests
   is a compatibility surface, where P3 allows first-consumer
   treatment, held loosely.

Compressed: A if the language boundary and the December calendar are
what is being protected; B if the honesty of the text and what agents
can ask are. The session's recommendation was B, built small.

## The owner's decision (2026-10-17, verbatim)

"I agree with your choice. I'm also sensitive to the timeline. Can we
fully document your analysis somehow, and park option B as a trigger
for a post-release feature?" — then, to the amendments below:
"agreed."

So: A now, amended; B parked with an evidence trigger (a trigger names
evidence, never a point in the sequence — `CONTEXT.md`, "Parked"); this
note cited by the park. The amendments to A, so that it sets B up
rather than digs in:

1. **Label, don't compute.** A's computed alternative (`or line(E)`
   when the elided number meets the bound) is dropped. A stat with a
   numberless or opposite-sign form is ambiguous with a note, the
   bound on its numbered, same-sign forms; A never becomes a third
   maker of the sign rule. The cost is a few missed matches, each a
   labelled oracle row already.
2. **The forms knowledge is data, once.** Which texts have an opposite
   word, which elide to a numberless display, which are exceptions: one
   forms table beside the catalogue, reference data under C106, read by
   the translation alone, for B to lift later. The vocabulary test
   lands now: every `#% chance to X` whose `X` is itself a stat, every
   text with both sign words, refused by name.
3. **The label points at the park.** The catalogue rows' notes for
   these clauses say what would make them exact and name the park.
4. **The seat asks.** Step 8's brief carries a friend's URL with a
   bounded `increased` stat, so the park's trigger has a fair chance to
   fire on evidence.

Unchanged: the stat-summed ruling lands in 7e as
`sum(line(T source=explicit).arg1)` with a presence guard on a maximum,
since a sum of nothing is zero.
