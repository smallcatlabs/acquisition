# Codex review: the h2 fix round, `7c4310da^..fa3335bf`

The outside review of the seven fix-round commits (`roadmap/briefs/h2-fixes.md`), run 2026-10-26 by the committing session
(`codex exec --sandbox read-only`, the prompt `roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner reads them before any fix is built.

7c4310da^..fa3335bf — the brief’s done-when does not fully hold: one remedy remains incorrect; test and gate success are **not verified**.

- **P2 — `crates/acquisition-plan/src/pricing.rs:244` — shapes 1 and 6:** The modified refusal still promises that a Standard status report displays a row belonging to another league. With Alice and Bob known, a readable price row on `tab/xbox/t` recorded only in Allflame, run `acq price list --account Alice --league Standard --in tab/xbox/t` (or the corresponding `price show`). The recommended `acq price status --expand --account 'Alice#1234' --realm xbox` reads Standard, where `place_other_leagues` moves that row into `other_league`; the text counts it but never displays it. Keep the status remedy for orphan rows; when another league holds the target, print an account-qualified `price show` command naming that league. Pin both callers. This remaining defect shares one helper across CLI and MCP.

What I did not check: builds, tests, fixture regeneration or gate execution, as instructed; their success and stage exit statuses are **not verified**. Static pins cover the seven faults, the review deletion and citation are present, and every changed golden/reference line has a corresponding source edit. The grep commit records the user-facing hits and summarizes other matches rather than pasting the complete raw hit list.

Nitpicks for unlikely cases: none.

Brief: restrict the old-word acceptance check to user-facing text, since the prescribed raw grep necessarily matches internal identifiers and game data.

Plan: no separate implementation plan was supplied.

Prompt: no change needed; the range and preserved review commit made the evidence accessible.
