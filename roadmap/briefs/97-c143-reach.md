# 97-c143-reach — every shared text in the request's words

Issue #97's remaining items, built at the owner's word of 2026-11-05
("Go ahead with both"), on his ruling that day: C143 amended in
`decisions/frontends.md` at `ddef4980` to cover every shared text, not
only refusals. Written by the orchestrating session against the tree
at `fc657318`. An Opus worker, in two parts, each reported and
reviewed before the next.

## Goal

C143 now says a shared text speaks the request's own words, a view's
field or a term, never a frontend's flag. It also says a way on that a
shared text names is a value each surface spells. Last night's #97
step (`briefs/97-own-terms.md`) built that shape for the search's
offers: `refusal::Way` and `refusal::Surface`, spelled by the CLI's
`Terminal`, the MCP's `Spoken` and the web's `Paged`.

Sixty lines in the shared crates still print a CLI command or flag (the
sweep below). An MCP agent or a page reader meets `--league` or
`acq price show …` where it should meet its own way on. When this step
is done, every one of them is either a value its surface spells, or
words that name the request's field, or a person's act kept as a
command under the rule's own exception.

## Read

- `AGENTS.md`.
- `decisions/frontends.md`:
  - C143 as amended;
  - C116;
  - C46, which puts shared semantics in Rust.
- `decisions/search.md`, for Part B.
- `REVIEW-CHECKLIST.md`: the frontends section ("A sibling sentence
  left standing"; "A printed command that does not run as printed")
  and the search section.
- `roadmap/briefs/97-own-terms.md` and the commits `215ef295`,
  `5f400352`, `6c5d7cbf`, `c8a3ed0f`, `3ce22d81`, `c2b97ddb`: the
  pattern this step extends. Read 215ef295's message for how an offer
  that carries a request is said.
- `crates/acquisition-verbs/src/refusal.rs`:
  - the module doc;
  - `Way`;
  - `Surface` (line 138), its `reading` (line 159) and its implementors
    (`grep -rn 'impl.*Surface for' crates`).
- `crates/acquisition-mcp/src/main.rs`: `SPOKEN`, the spelling table
  whose glosses exist only because shared text carries CLI words. A
  gloss this step makes unneeded is retired under C154.
- `crates/acquisition-search/src/answer.rs`: `command` (line 2966),
  `opening` (line 3110), and the three `command` methods (lines
  3125–3147).

## Input

The sweep, run on `fc657318` (the class's measure; run it again at the
start and at the end):

```sh
for c in plan search verbs store; do grep -rn --include='*.rs' -E '"[^"]*(`acq |acq [a-z]|--[a-z][a-z-]+)[^"]*"' crates/acquisition-$c/src; done
```

It printed 60 lines, by file:

| File | Lines |
| --- | --- |
| plan `pricing.rs` | 21 |
| search `answer.rs` | 16 |
| search `describe.rs` | 9 |
| store `index.rs` | 3 |
| search `bind.rs` | 3 |
| search `price.rs` | 2 |
| verbs `refused.rs` | 1 |
| verbs `jobs.rs` | 1 |
| verbs `frontend.rs` | 1 |
| verbs `diagnose.rs` | 1 |
| store `lib.rs` | 1 |
| plan `shop.rs` | 1 |

Some hits may sit in `#[cfg(test)]` modules; the map says which.

Classified by the orchestrator. The map confirms or corrects each.

- **Part A, refusals.**
  - plan `pricing.rs`:
    - 295 ("name one with --league");
    - 343–346 and 448–450 (an `acq price show` offer built with
      `--account`, `--realm` and `--league`);
    - 397 ("--realm or another address");
    - 465–472 (`acq search`, `acq tabs`, "`--league` reads it there");
    - 485;
    - 638–639 (`--in` and `--covered-by` named in a parse refusal);
    - 1079–1080 (`read_back`, the price conflict's read-back, the
      owner's comment of 2026-11-01 on #97);
    - 1276 (`acq shop render`);
    - 1373 (`acq reference currency`).
  - verbs `refused.rs` 54 (`acq store refused`).
  - store `index.rs` 132 (`acq accounts`) and 135 ("pick one with
    --account or ACQ_ACCOUNT").
  - store `lib.rs` 421 (`acq store refused {id}`).
- **Part B, text that is no refusal.**
  - search `answer.rs` 3042 and 3110–3218: the route, tabs and
    continuation commands an answer prints.
  - search `describe.rs` 527–679: describe entries named by flag
    (`--count`, `--cross`, `--sum`, `--explain`, `--context`,
    `show --against`, `--sort`).
  - search `bind.rs` 140–150: the keys not built, named by flag
    (`--fields`, `--rebind`, `show --basis`).
  - search `price.rs` 102 and 106, and plan `shop.rs` 300: price notes
    pointing at `acq price show`.
- **Kept, a person's act** (C143's exception):
  - verbs `frontend.rs` 313 (`acq auth check`);
  - verbs `diagnose.rs` 403 (`acq daemon reset-tripwire`);
  - verbs `jobs.rs` 616 (`acq auth`, "a login, which is a person's");
  - store `index.rs` 131 (`acq auth`).

  Each stays a command and is marked as a person's act where the
  document has a place for it (the refusal document's `next`). If one
  is not a person's act, move it to its part.

The last push's CI: read both gates for the tree you start on and name
them in the report.

## Do

**Part A, refusals.** Report and stop after it; Part B comes by
message after its review.

1. **The map**, in Part A's first commit message. List every sweep
   line with its part, what it says, and the rule that answers it.
2. **Each refusal's way on is a value.**
   - Where a refusal offers a command (`acq price show`, `acq search`,
     `acq tabs`, `acq shop render`, `acq reference currency`,
     `acq store refused`, `acq accounts`), the offer becomes a `Way`.
     Add a variant only where no existing one says it. Every
     `Surface` spells it: the CLI's as today, the MCP's as its tool and
     arguments, a page's as a link where a page takes it and nothing
     where none does.
   - Where a refusal names a flag as the thing to supply (`--league`,
     `--realm`, `--account`, `--in`, `--covered-by`), the words name
     the request's field (`league`, `realm`, `account`, …). The CLI's
     `Terminal` may add its flag after them (C143's adapter clause).
   - The store links no workspace crate (C1), so its `Resolve` words
     name the field, and the verbs crate adds the way. If a store
     refusal reaches a frontend by another path, say which.
   - `read_back` is the owner's named case. On the MCP it reads as the
     `price_show` call with its arguments, never an `acq` line.
3. **Pin.**
   - Each refusal class is pinned where its documents are compared
     across surfaces: the verbs crate's refusal document tests, the
     MCP's `tool_refusals.rs` and `pricing_tools.rs`, and the web's
     refusal page (C153: cases in those files, no new file).
   - One rule test walks every MCP refusal the existing tests already
     produce and fails on `acq ` or a `--flag` outside the person's-act
     exception. Put it in the verbs crate if it can hold it; otherwise
     the MCP's tests. Show it red against the tree before Part A for
     one run, and say what it printed.
4. **Glosses.** Every `SPOKEN` entry that only translated one of these
   lines is retired, one `retired X; held by Y` line each (C154).
5. Regenerate the goldens and both references under
   `ACQ_UPDATE_FIXTURES=1`, and read every changed line. The CLI's
   text should not move except where it gains a field's words before
   its flag. Say which goldens moved and why.

**Part B, text that is no refusal.** It comes by message after Part
A's review.

6. **An answer's commands are values.** The route, tabs and
   continuation pointers `answer.rs` builds as CLI strings carry their
   request (the search's `Request`, a `tabs` read, a continuation
   token), and each surface spells it. The CLI prints the command
   exactly as today; `readme_tour.rs` and the run-as-printed tests
   hold that. The MCP gives the tool and its arguments.
   - If a pointer is serialized in the answer's JSON as a `command`
     string, that field is part of the document a user's JSON reads
     (C53). Keep the field and add the request beside it, or change
     the field. Choose the one that leaves every surface's document
     true, and say which and why. This is an agent-facing design call,
     listed for the owner.
7. **`describe` and the keys not built** name the request's word
   (`count`, `cross`, `sum`, `explain`, `context`, `sort`, `fields`,
   `rebind`, `against`, `basis`). The CLI's help and the MCP's tool
   text spell them their own way. Check the MCP's `describe` tool and
   the first-turn size bound (`surface_size.rs`, 38,000 bytes): stay
   under it by trimming your own words, never by raising it.
8. **The price notes** (`search/price.rs` 102, 106; `plan/shop.rs`
   300) name the way on as a value, or say it in words with no
   command, whichever the note's document allows.
9. **Pin.**
   - Part A's rule test widens to every MCP answer the existing
     tests produce, not only refusals, with the person's-act exception.
   - The amended C143 names `acquisition-search/tests/explain.rs` and
     ends "Unbuilt beyond refusals, #97". When Part B lands, the
     worker replaces that clause with the rule test's name. That is
     the only registry edit.
10. Goldens and references, as in item 5.

## Done when

- **Part A:**
  - The map is in its first commit.
  - Every Part A sweep line is a `Way` or words naming a field, with
    the CLI's flag after them where the CLI adds it.
  - The rule test holds MCP refusals free of `acq ` and `--flag`
    outside the person's-act exception, and was seen red before.
  - Retired glosses each have their C154 line.
  - The gate is green.
- **Part B:**
  - Every Part B sweep line is a value its surface spells, or the
    request's word.
  - The CLI's printed commands are unchanged and still run as printed.
  - The rule test covers every MCP answer.
  - C143's *Pinned* names it.
  - The gate is green.
- At the end, the sweep prints only:
  - the person's-act lines;
  - lines inside the CLI's own `Surface` spelling (if any moved into a
    shared crate, say why);
  - lines in `#[cfg(test)]` modules.

  The report lists what remains.

## Not in scope

- New web routes. A way no page takes is not offered on a page, as
  last night; the morning report lists those lines.
- The protocol and the daemon.
- The CLI's own text in `acquisition-cli` and the MCP's own tool text,
  except where a retired gloss or a field's word lands.
- Any wording beyond what moves a command or flag out of shared text.

## Rules

`TEMPLATE.md`, plus:

- Work in the main checkout on `main`. Commit locally, one commit per
  Do item or a logical pair, each naming this brief; do not push.
- **This is the owner's machine, with his real store and keyring.
  Never start a real-provider daemon, and never run a bare `acq`.**
  Every `acq` and `acq-mcp` runs under the mock-session skill, with
  `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR` under your scratch.
- C153, C154.
- Run the gate in the foreground, bare. Read its exit status alone,
  never through a pipe or `tail`.
- Stop after Part A and report.

## Report

`TEMPLATE.md`, plus:

- The map (Part A).
- The sweep's count before and after each part.
- The design calls.
- What remains of the sweep, and why.
