39f3b7dc — the brief’s done-when does not fully hold: persistence isolation and replayable-command guarantees have gaps.

1. **[P1] crates/acquisition-store/src/user.rs:35 — `user.db` collides with an account’s facts file.** The supported mock account name `user` also maps to `user.db` through `account_path`. With saved searches already present, opening that account’s facts can add the facts tables and stamp schema 9, after which every saved-search operation refuses the file; with facts present first, saved searches cannot open it. This meets an upstream mock account name or its persisted index entry. Escape reserved account filenames centrally, handle existing collisions safely, and pin both opening orders.

2. **[P2] crates/acquisition-verbs/src/saved.rs:825 — shape 6: a conflict offers a partial write that cannot succeed over an unreadable row.** For persisted input containing a version-2 row at revision 2, `saved set --id ID --name renamed --if-revision 1` offers the same command at revision 2; replay then fails because replacing that row requires every field. Offer its show and whole-replacement instructions instead when the partial retry cannot pass. Findings 2 and 3 share a cause: recovery writes are offered without checking the writer’s validation requirements.

3. **[P2] crates/acquisition-verbs/src/saved.rs:218 — shape 6: an unreadable query produces an unusable undo.** A persisted version-1 value with `text: "rarity=="` passes the store’s intentionally language-free parser. Clearing it succeeds and offers a whole-value undo, but replay fails query validation; replacing its text likewise offers an invalid undo of the prior. Validate the prior’s query before offering restoration and return `no_undo` with the reason when this build cannot write it back; pin both receipts.

4. **[P2] crates/acquisition-cli/src/saved_cmd.rs:207 — shape 6: printed run commands discard the selected account.** With Alice and Bob known and `ACQ_ACCOUNT` unset, `acq --account Alice saved show ID` prints `acq search --saved ID`, which fails as `ambiguous_account`; write receipts repeat the omission at line 265. Pass the caller’s account into both text renderers and replay their printed commands with two accounts known.

5. **[P3] crates/acquisition-verbs/src/saved.rs:730 — a valid persisted padded name cannot be opened by name.** The store accepts—and explicitly tests—names such as `" spaced "`, but resolution trims the requested name and compares it with the untrimmed stored name, so neither spelling finds that row. Compare trimmed names consistently without rewriting persisted values, and pin this store-accepted case.

- **Not checked:** compilation, tests, gate exit status and execution-dependent done-when lines — **not verified**, because this review was read-only as requested.
- **Nitpicks:** finding 5; current verbs trim names before writing, so it requires another store writer.
- **Brief:** add reserved-filename and stored-but-unparseable-query cases to its fixed inputs.
- **Plan:** no further change suggested.
- **Prompt:** no change suggested.
