0ca39d50 — fix-round done-when does not fully hold: legacy files are detected and pinned, but the recovery remedy is unsafe for supported persisted states.

- **[P2] [crates/acquisition-store/src/index.rs:220](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/index.rs:220) — shapes 1, 2:** Renaming only the main database can abandon committed facts in its WAL. Concrete persisted input: an earlier build exits abruptly after recording facts, leaving `user.db-wal`; the read-only probe recognizes the database without checkpointing it, but following the refusal moves only `user.db`. The destination then lacks those committed facts. The test closes its store cleanly before renaming, excluding this case. Smallest fix: describe recovery with all database users stopped and the WAL safely checkpointed or preserved with the database, and pin an uncheckpointed legacy file.

- **[P2] [crates/acquisition-store/src/index.rs:220](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/index.rs:220) — shape 1:** The same unconditional rename remedy ignores an existing destination. Concrete persisted input: legacy `user.db` remains while account reads create `~user.db`, and subsequent fetches populate it; renaming over that destination can discard newer facts, while a non-overwriting rename cannot complete the proposed recovery. Smallest fix: require preserving any existing destination database and its sidecars before recovery, and pin that case. Both findings share the assumption that recovery is a single-file rename into an unused path.

- **Not checked:** compilation, tests, gate status and execution-dependent conditions — **not verified**, as requested.
- **Nitpicks:** none; both findings concern preserving persisted facts during the prescribed recovery.
- **Brief:** no change suggested.
- **Plan:** no change suggested.
- **Prompt:** no change suggested.
