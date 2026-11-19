57712d98^..d572f30a — the brief’s full done-when does not hold; this range also contains no close.

- **P2 — `crates/acquisition-cli/src/web/page.rs:3021`:** The new job rows discard the account although the list spans accounts and the navigation names the served account. With Alice served and Bob’s failed stash job listed, the page gives no indication that the failure belongs to Bob. **Input:** daemon documents or persisted jobs. **Smallest fix:** carry `account` into `JobRow` and display it beside each job’s description.

- **P2 — `crates/acquisition-cli/src/web/mod.rs:898`:** A `NotRecorded` failure offers “the result, which keeps what it fetched,” but its link returns to the same summary without displaying that payload. **Input:** an upstream response whose ingestion failed, retained in `Outcome::Failure.payload`; `result_words` ignores it, and requesting JSON from this URL returns the jobs list. **Smallest fix:** expose the retained failure payload in the opened result so the offered destination contains what it promises.

- **P2 — `crates/acquisition-cli/src/web/page.rs:3155`, shape 1:** The universal `NotFound` remedy incorrectly says listing the league or characters again reveals the missing target. **Input:** an upstream 404 for `stash` with `id=maps, sub=m002`; substashes are listed by fetching their parent tab, as the existing typed recovery in `jobs::way_on` already distinguishes. **Smallest fix:** derive the page’s recovery prose from those typed ways, distinguishing a parent fetch from a stash or character listing.

- **P3 — `crates/acquisition-cli/src/web/page.rs:2671`, shape 1:** `ListedExperienceDisagrees` is rendered as “more experience,” although the planner constructs it for any unequal values. **Input:** persisted character facts with a newer listing’s experience `100` and fetched experience `150`. **Smallest fix:** say “different experience,” or render the actual direction.

- **Not checked:** gate execution, `--only doc`, runtime behavior and test counts — **not verified**, as execution was prohibited. GitHub issue closure was not checked; neither commit supplies the required close account for #189 and #182.
- **Nitpicks for unlikely cases:** none; the P3 finding concerns wording alone.
- **Brief:** Decision 7 should distinguish opening a failure summary from accessing its retained payload.
- **Plan:** no separate working plan was supplied.
- **Prompt:** no change needed.
