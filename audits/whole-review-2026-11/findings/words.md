# Where the words live

The plan's D2. Investigator's report: `reports/investigators/words.md`. Portfolio: proposal 1.

## What was found

The claim "one document, three projections" is true of the data and false of the words. The JSON answer carries no sentence. Every sentence a reader meets is composed in the CLI; the web page calls the CLI's functions to say the same things; the MCP gets none of them and words the same rules again, in other words, in `describe`.

| Measure | Number | Command |
| --- | --- | --- |
| Sentence functions in the CLI | 116, 80 of them in `search_cmd.rs` | `git grep -hoE 'fn [a-z_]+_(words\|text\|sentence\|says)[(<]' -- crates/acquisition-cli/src \| sort -u \| wc -l` |
| The same, search / daemon / mcp / verbs / plan / store | 11 / 18 / 2 / 1 / 0 / 0 | the same over each crate |
| Symbols `web/` takes from the CLI's `*_cmd` modules | 86 (the investigator's rule gives 89; 78 are sentences) | `git grep -ohE '(search_cmd\|plan_cmd\|…)::[A-Za-z_]+' -- crates/acquisition-cli/src/web \| sort -u \| wc -l` |
| `Way` variants | 20, each spelled in `terminal.rs` and `told.rs`, ten also in `web/` | the enum in `verbs/src/refusal.rs` |
| Remedies spelled by hand outside any `Way` | about 150 lines | the investigator's `way.sh` |
| Wording differences between doors, sampled | 30: 5 must differ, 10 are spelling, 15 differ for no reason | read from the lines cited |

## Checked by the orchestrator

- `acq price status --league Nowhere` prints `next: acq refresh --plan …`; the same with `--json` has no `offers` and no `next` key. An MCP agent never gets that remedy.
- `search_cmd.rs:2146` and `describe.rs:291` word one rule twice.
- The counts in the table above marked with a command were re-run; the first run of each returned 0 because of the orchestrator's own bad path filter, then matched.

## Where this parts from the plan

- The advisor's T1 says the MCP patches the CLI's spellings. It does not get them at all.
- The advisor's T4 proposes a `take` verb that accepts any offer. If a way on is a call of a verb in the same argument names at every door, there is nothing for `take` to do.
- "The frontends' tests mostly re-pin spelling" is too strong for test code and true of the goldens (see `tests.md`).
- The net deletion is modest, about 1 to 1.5k source lines. Most of the code moves below the frontends. What shrinks is what the move makes possible.

## Position

A document carries its sentences and its ways on, made in the crate that makes the document; a way on is a call. A door lays out and spells. What a door must still own is small and nameable: its own controls' words, a person's act addressed to its reader, layout, and quoting for its shell or URL. That was a sixth of the differences sampled.

The strongest objection is the investigator's own: some sentences interleave a count, a gloss and a printed command, so `says` needs one slot for an embedded call, and that is the start of a template language. One slot is the limit.

No invariant is touched. C143 (no door's words on another door) becomes a dependency edge instead of a scan. C139's stated reasons survive, and the web can become a library crate the verb calls.

## Route

Proposal 1; it is a design, so a design brief by Fable for the owner to rule (C173). Issues drafted: `price-status-json-has-no-way-on`, `json-offer-request-has-two-types`, `cli-ignores-a-faults-next`.

## After the skeptic

Narrowed (`dissent/theses-1-5.md`, thesis 1).

- "The JSON carries no sentence" is true of the search answer and false in general: a refusal's JSON carries `error` (`verbs/src/refusal.rs:228`) and an apply's carries `note` (`verbs/src/refresh.rs:219`).
- Some wording must differ by door, and the MCP's account of why it cannot start a daemon (`told.rs:82`) is one. That is the "must differ" sixth, not an objection to the rest.
- Comparing documents across doors cannot hold what a door adds: exit codes, escaping, an HTTP status, a permission. Those tests stay.
- Cutting the MCP to fourteen tools by sending `store_refused` to the CLI removes a function from an agent that has only the MCP. A tool list may be regrouped; nothing may be lost.

What stands: the words are composed above the documents, twice over; remedies exist that one door never gets; one home for words and calls is sound.
