7a7538c5^..079b0200 — done-when does not fully hold: fault 1 remains incomplete; the other scoped fixes are addressed by inspection.

- **P2 — crates/acquisition-cli/src/web/mod.rs:1005:** The fallback league control can display a different league from the page’s actual selection. With persisted places only in Settlers and no fetched league list, client navigation to `/tabs` loads Standard while the browser displays Settlers, the first available option; “List this league’s stash tabs” still submits Standard. Preserve the current league as a selected option when absent from the offered list, and pin this case.

- **P2 — crates/acquisition-cli/src/web/page.rs:2617 — shape 2:** Empty place pages return before rendering the new match totals and “elsewhere” links, leaving fault 1 unresolved when every match is elsewhere. With persisted rings only in Settlers, `/tabs?realm=pc&league=Standard&text=class%3Arings` shows no matching-count explanation or link to those rings; the new test always supplies Standard tabs. Render `matched_line` outside the nonempty-list branch and extend the existing test to cover an empty selected league and an empty selected place kind.

- **Not checked:** Execution, gate status and test count: not verified, as instructed; no builds, tests, daemon, browser or live API were run.

- **Nitpicks:** Neither finding; both arise during ordinary navigation over a copied collection.

- **Brief:** The fixed seat record and approved split were sufficient.
- **Plan:** No separate plan was supplied or needed for this review.
- **Prompt:** No changes proposed.
