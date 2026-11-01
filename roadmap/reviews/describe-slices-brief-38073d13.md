`roadmap/briefs/describe-slices.md` at `38073d13`, identical to the current file: buildable in design, but one new test requirement contradicts its matching rule.

1. **Answered** — removing clap’s delimiter preserves commas; `Option<Vec<String>>` infers `Append`, compatible with `num_args = 0..=1`; splitting each occurrence separately supports repeated flags. MCP’s JSON array needs no transport change.
2. **Answered** — a pattern requires a character after `~`; bare `~` remains the operator’s name, explicitly pinned.
3. **Answered** — the index-key assertion is now `(SECTIONS − {trade}) ∪ {trade_rows, next}`.
4. **Narrower (sound)** — “answers whole” covers entries, sections and categories; individual not-built constructs retain their refusal.
5. **Not fully answered** — the escaped dot is correct, but the new anchored-definition negative case contradicts settlement 6.

Findings, most severe first:

- **F1 — Do 5 / settlement 6:** The prescribed test excludes an entry whose name matches, although settlement 6 explicitly matches either name or definition. **Concrete reading:** [brief:448](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/describe-slices.md:448) says “not one whose name does”; [brief:334](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/describe-slices.md:334) requires testing both strings independently. **Smallest change:** replace that negative case with an entry whose definition contains the text only after its start and whose name does not match.

What I did not check: builds, tests, gates, measurements or runtime behavior, as requested; external issue bodies and comments were assessed through the brief’s account.

Nitpicks: F1 is a local test-wording defect, with no design change required.

The step may start after that one-sentence correction to Do 5.
