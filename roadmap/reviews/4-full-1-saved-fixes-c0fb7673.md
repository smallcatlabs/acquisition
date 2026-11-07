c0fb7673 — the fix-round done-when does not fully hold: finding 1 remains unresolved for existing worlds; findings 2–5 have source fixes and pins.

- **[P2] crates/acquisition-store/src/index.rs:162 — shape 2: the escape abandons existing reserved-name facts files without resolving their collision.** For persisted input from the previous build—an index entry named `user` and its schema-9 facts in `user.db`—account reads now create an empty `~user.db`, while every saved-search operation still opens `user.db` and fails with `SchemaTooNew`. The new opening-order test creates facts through the updated `account_path`, so it never stages that legacy file. Handle legacy reserved paths safely before switching, preserving existing contents or providing explicit recovery instructions; pin the literal legacy path rather than deriving it through the new mapping.

- **Not checked:** compilation, tests, gate status and execution-dependent conditions — **not verified**, as requested.
- **Nitpicks:** none; the finding affects an existing world with a supported mock account name.
- **Brief:** no change suggested.
- **Plan:** no change suggested.
- **Prompt:** no change suggested.
