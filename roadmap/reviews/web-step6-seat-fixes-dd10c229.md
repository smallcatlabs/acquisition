dd10c229 — the fix round’s done-when does not fully hold: league preservation still fails when no league options are available.

- **P2 — crates/acquisition-cli/src/web/page.rs:2465 — shape 1:** The new preservation rule skips `leagues: None`, so submitting the realm form can still silently change the page’s league. With persisted places only in PC Settlers and no fetched league list, open `/tabs?realm=pc&league=Settlers`, select Xbox and Show, then select PC and Show: the Xbox page omitted the league control, and `carried` excludes `league`, so the final client request defaults to Standard. Render the selected league as an option even when the list is `None`, and extend the existing test to cover this realm round trip.

- **Not checked:** Execution, gate status and test count: not verified, as instructed. Review used the committed sources and fixed seat record; no builds, tests, daemon, browser or live API were run.

- **Nitpicks:** None; the finding is reachable through the page’s ordinary controls.

- **Brief:** The fixed seat record and approved scope were sufficient.
- **Plan:** No separate plan was supplied or needed.
- **Prompt:** No changes proposed.
