## How I got the numbers

All scripts are in `<scratch>/readers/registry/`.
- `sample.py` takes the area of each id from `bothy.py list`, sets quotas by largest remainder, seeds with 20261126, then runs `random.sample` per area in a fixed order. The registry has 200 entries. In the sampled areas: daemon 38, network 22, store 21, frontends 20, search 22, plans 12, pricing 15, trade 7. Process has 24 and ci has 1.
- `entries.py <ids>` gives each entry's size, its body sentence count, the names its *Pinned* ticks, and its `git grep -c -w` counts per tree. Bodies are split at `. ` or `; ` followed by a capital letter, a backtick or a parenthesis, so a `;` clause sometimes counts as a sentence.
- `whole.py` produces the step 3 counts.
- The "exists" test in `entries.py` was wrong because it grepped file contents, not paths. I checked the three names it flagged with `git ls-files`, and all three exist.
- I counted any path under `crates/*/src` with no `/tests/` or `/unit/` in it as src. Inline `#[cfg(test)]` modules therefore count as src.
- Columns a, b-mechanism, d, e and g are my judgment. For d, I read the pinned test or check itself only for C20, C109 and C169. Everything else was judged from test names and counts (`git grep -h "fn c<n>_"`). For e and g, I read only the doc comments at the sites that `git grep` found citing the id.

Sample: daemon C6 C9 C86 C123 C144 C148 · network C20 C25 C32 C169 · store C58 C62 C108 · frontends C53 C168 C191 · search C101 C102 C109 C118 · plans C40 C42 · pricing C65 C71 C80 · trade C141 · cross-cutting C12 C35 C46 C48.

## The sample

How to read the columns:
- **d**: Y means the gate would fail on a breach, P means partly (what is unheld follows), N means no.
- **e**: Y means the doc comment states the property and its reason; Prop means the property only; Cite means only fragments that cite the id; N means none.
- **f**: counts of the id in src / tests / reading-path docs / `roadmap/reviews`.

| id | a kind | b sent (mech) | c pinned (exist) | d gate | e doc comment | f src/test/docs/rev | g why |
|---|---|---|---|---|---|---|---|
| C6 | behaviour | 3 (2) | 5 (5) | P (by name) | Prop, store jobs.rs:1 | 40/30/5/4 | no |
| C9 | boundary | 0 | 1 (1) | Y | Prop, protocol.rs:1 | 1/0/1/0 | no |
| C86 | behaviour | 1 (0; 5 s is in the headline) | 2 (2) | Y (by name) | Y, client.rs:62, :320 | 27/3/3/2 | yes |
| C123 | behaviour | 1 (1) | 4 (4) | P: the provenance "never a trade-site body" is unheld | Prop, mockggg.rs:366 | 2/3/2/0 | no |
| C144 | behaviour | 1 (1) | 1 prefix (34 tests) | P: index written "atomically, synced" | Cite, auth.rs:198, :641 | 44/46/1/2 | no |
| C148 | mechanism | 1 (1) | 2 (2; 12 tests) | P (many clauses) | Cite, scattered (client.rs:433) | 64/21/1/12 | yes, daemon.rs:219 |
| C20 | behaviour | 0 | 2 (2) | P: the test is the limiter learning from a probe; "visible job, once per lifetime" is unheld there | N (no file names C20) | 0/0/1/0 | no |
| C25 | behaviour | 3 (1) | 5 (5) | P: "ceiling not persisted" | Cite, daemon.rs:414, kinds.rs:41 | 41/23/4/0 | no |
| C32 | mechanism | 2 (1) | 4 (4) | Y (by name) | Prop, daemon/routes.rs:8 | 2/5/3/1 | no |
| C169 | boundary | 2 (1) | 3 (3) | Y (arch-check check 5, read) | Prop, transport Cargo.toml:19, arch-check.sh:517 | 1/0/1/3 | partly (ClientHello, arch-check.sh:283) |
| C58 | mechanism | 2 (2) | 1 (1, bare `tests/realm_wire.rs`) | P (by name) | Y, protocol realm.rs:3; plan lib.rs:174 | 14/3/3/0 | yes |
| C62 | behaviour | 2 (1) | 2 (2) | Y (by name) | Prop, plan lib.rs:183 | 2/1/1/0 | no |
| C108 | boundary | 3 (2) | 1 (1) | P (not read) | Prop, search corpus.rs:4 | 15/10/3/13 | no |
| C53 | convention | 13 (5) | 6 (6; 35 tests) | P (tty escapes, column order and others) | Cite, 218 src mentions, no single statement | 218/82/9/92 | no |
| C168 | behaviour | 1 (1) | 6 (6) | Y (stale-refusal test by name) | Prop, cli web/mod.rs:238 | 6/4/1/17 | no |
| C191 | boundary | 1 (0) | nothing yet | N | N | 0/0/1/0 | no |
| C101 | convention | 5 (1) | 5 (5; one is the token `a`) | P: nothing stops a raw-path accessor | Cite, search bind.rs:69 | 44/33/3/0 | no |
| C102 | behaviour | 3 (0) | 2 (2) | P | Prop, search describe.rs:179 | 7/7/3/0 | no |
| C109 | behaviour | 2 (0) | 1 (1) | Y (read) | Y, tree.rs:492 + tests/depth.rs:1 | 4/3/1/0 | yes |
| C118 | behaviour | 5 (1) | 2 (2) | P: the subject's part is Unbuilt (#155, #156) | Cite, answer.rs:3701 | 13/11/1/43 | no |
| C40 | boundary | 1 (0) | 4 (4) | Y (by name) | Prop, plan lib.rs:101 | 3/3/1/7 | no |
| C42 | behaviour | 2 (0) | 3 (3) | Y (by name) | Prop, daemon kinds.rs:1115 | 1/5/2/6 | no |
| C65 | behaviour | 3 (2) | 2 (2) | Y (by name) | Prop, store annotations.rs:51 | 22/4/2/3 | no |
| C71 | behaviour (Unbuilt) | 2 (1) | nothing yet | P: the interim rule may be held by C35/C67 tests | Cite, annotations.rs:1052 | 1/0/4/0 | no |
| C80 | behaviour | 1 (0) | 2 (2; 1 test) | Y (by name) | Prop, plan game_side.rs:60 | 16/4/2/0 | no |
| C141 | behaviour | 1 (0) | 1 (1) | Y (by name) | Prop: the entry copied word for word at search trade.rs:42 | 8/1/3/0 | no |
| C12 | boundary | 1 (0) | 1 (1) | P: wire.rs does not hold it; arch-check's edges do | Prop, cli main.rs:2 | 14/5/3/2 | no |
| C35 | boundary | 1 (1) | 2 (2; 8 tests) | Y (by name) | Cite, scattered (price_cmd.rs:16) | 32/32/6/14 | no |
| C46 | boundary | 2 (0) | 3 (3) | P: the door grep is a spelling grep | Cite, scattered | 30/13/3/28 | no |
| C48 | boundary | 0 | nothing yet | N for "no cached search"; the schema refusal is partly held | Cite, store lib.rs:246, corpus.rs:884 | 4/0/5/2 | no |

Sentence counts are by script. Mechanism counts are my judgment.

## Totals (from the table)
- **Kind:** behaviour 16, boundary 9, mechanism 3, convention 2.
- **Breach fails the gate:** yes 13/30 (43%), partly 15 (50%), no 2 (7%). Of the 13 yes, 11 rest on test names and only C109 and C169 were read.
- **Property and reason both at the code:** 3/30 (C86, C58, C109). Property only: 15. Fragments that cite the id: 10. Nothing: 2 (C20, C191).
- **Why found anywhere a code reader meets it:** 4 of 30, plus C169 partly.
- **Mechanism sentences:** 25 of 65 body sentences, about 38%. The headline is not counted, and some headlines carry mechanism too (C6 names `daemon.db`, C86 says 5 s, C32 names a file).

## Must have been read first (judgment)
- **C191:** nothing in code or tests names it, and only the entry says `acq web` is not to become a desktop app.
- **C20:** no file under `crates/` cites it, and its pinned test does not hold "visible job, one per lifetime".
- **C48:** "no cached search service" has no pin, and the two doc comments mention it only in passing.
- **C101:** nothing stops someone adding a raw-path accessor or a general collection syntax.
- **C71:** it is Unbuilt, and the rule for the meantime appears only in the entry.
- **C144 (partly):** "written atomically, synced, under the one lock" has no test I could find, and the doc comments that cite it are fragments.

## Could be met at the point of need with nothing lost (judgment)
- **C169:** the arch-check failure message names the rule and the id.
- **C109:** the constant's doc comment and the test's doc give both the property and the reason.
- **C86 and C58:** the doc comment gives both the property and the reason.
- **C32:** routes.rs states the property in full. The reason is only in the entry.
- **C141:** the doc comment repeats the entry word for word.
- **C9, C40, C42, C62, C65, C80, C168:** the test names and module docs state the property. Only the *Why* is lost, and it is short.

## Whole registry (step 3, `whole.py`)

| area | n | nothing yet | bytes min / median / max | total bytes |
|---|---|---|---|---|
| cross-cutting | 18 | 7 | 196 / 391 / 831 | 7,483 |
| daemon | 38 | 1 | 169 / 765 / 1,306 | 27,227 |
| frontends | 20 | 1 | 431 / 764 / 2,473 | 17,044 |
| network | 22 | 2 | 270 / 726 / 1,218 | 16,021 |
| store | 21 | 2 | 322 / 638 / 1,288 | 13,964 |
| search | 22 | 0 | 474 / 732 / 989 | 15,444 |
| plans | 12 | 0 | 398 / 608 / 802 | 7,384 |
| pricing | 15 | 3 | 386 / 651 / 746 | 9,385 |
| trade | 7 | 0 | 468 / 726 / 1,183 | 5,261 |
| process | 24 | 16 | 267 / 546 / 1,007 | 13,595 |
| ci | 1 | 1 | 671 | 671 |

- **Nothing yet:** 33 of 200 in all, 16 of them in process.
- **Entries citing another entry by id:** 93 of 200.
- **Most cited:** C53 by 6 entries. Nine entries are cited by 4 each (C10, C98, C93, C38, C79, C68, C81, C94, C106); the list cuts off arbitrarily among ties at 4.
- **Ids in no file under `crates/`:** 33. Of these, 24 are process, 3 cross-cutting (C15, C110, C154), 1 ci (C155), and one each in network (C20), frontends (C191), pricing (C73), search (C125) and store (C196).
- **Shape baseline:** C53 is listed in `decisions/shape-baseline.tsv` (headline 105 bytes, 19 clauses), so it is excused from the six-clause limit.

## Pinned names that do not exist; bodies that disagree with code
- **Missing names:** none. Every ticked name exists.
- **Weak pins:**
  - C101 ticks the token `a`. That satisfies bothy's rule 7 without naming anything.
  - C58 ticks the bare path `tests/realm_wire.rs`. It exists only as `crates/acquisition-cli/tests/realm_wire.rs`, so it is ambiguous only by form.
  - C12's pin (`wire.rs`) does not hold its property. The holder is arch-check's link edges, which is my judgment.
  - C20's pin holds a neighbouring property, not the entry's own.
- **Disagreements found:** none in what I read. One looked possible but is consistent: C6 and C83 say "provider-neutral world", while `jobs.rs:1` says `store/<provider>/daemon.db`. `world.rs:3-4` puts the provider directories under that neutral root.
- **Drift risk:** the C141 text is copied word for word at `crates/acquisition-search/src/trade.rs:42`, so two copies can drift apart.

## What I did not check, and what a sample of 30 cannot show
- I did not run tests, mutate code or run `bothy.py check`. Most "Y" verdicts in d rest on test names, not on reading the tests.
- I did not look for *Why* text beyond the cited doc-comment sites.
- The src/test split counts inline test modules as src.
- With 1 to 7 entries drawn per area, no per-area rate means anything. Process and ci (25 entries, most of the "nothing yet" and most of the ids absent from crates) were excluded by design.
- The sample cannot show what breaks when entries interact, for example the chains of glosses that cite another id.
- It cannot show whether builders actually read the entries.
- It cannot show how often an entry's text has drifted from the code over time (that would need `git log -S`), and it cannot estimate how rare the unpinned, uncited entries like C191 and C20 are.
