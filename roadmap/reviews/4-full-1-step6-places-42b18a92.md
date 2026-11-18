d7837ef7^..42b18a92 — done-when partially holds: the build is present, but request-text equality and the character-table pin have gaps; the required close is absent.

- **P3 — crates/acquisition-cli/src/web/page.rs:2279 — shape 1.** “Stash tabs (6)” counts top-level places, including folders, without naming that denominator. The committed golden simultaneously reports 15 tabs and this six-row summary. **Smallest fix:** label it “6 top-level places,” or omit the repeated count.
- **P3 — crates/acquisition-cli/tests/web.rs:2694 — shape 2.** The character-link check follows JSON requests against never-fetched characters, so it does not establish the required HTML table of that character’s items. Removing the character link’s table presentation would still satisfy this helper’s query/scope assertions. **Smallest fix:** fetch one character, follow its name as HTML, and assert the table and an expected item.
- **P3 — crates/acquisition-search/src/places.rs:191 — shape 7.** Twin-league detection uses persisted listings here but item-bearing leagues in `counts::league_twins`, so the promised identical request text is not universal. Persist `Standard/d1` with an item and a listed-but-empty `standard` league: the place request uses `league~…`, while the bucket uses `league=Standard`. **Smallest fix:** derive twin detection from the same declared set for both makers and extend the equality test with an empty twin.

Not verified: gate, `--only doc`, test outcomes/counts, and browser behavior; execution was prohibited. Remote issue closure was not checked.

Nitpicks: the count label is wording alone; the character finding is a coverage gap rather than an observed implementation failure; the case-variant league example is unlikely outside fixtures. No P1 or P2 defect found.

Brief: distinguish the top-level row count from the league’s tab count. Plan: none was supplied. Prompt: distinguish build-review requirements from the close that follows cold review.
