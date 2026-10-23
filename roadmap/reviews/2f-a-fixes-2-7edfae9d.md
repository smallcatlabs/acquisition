# Codex third look, round A second fix round 7edfae9d (read-only, 100,776 tokens)

7edfae9d — done-when does not hold: submit bounds are present, but a client-reachable refusal remains unbounded; runtime behavior is not verified.

- **[P2] `crates/acquisition-daemon/src/daemon/login.rs:384`, shapes 3 and 5:** The account-index error is still forwarded verbatim, allowing serde’s quoted value to overflow the response frame. Concrete input: `accounts.json` containing `{"accounts":[{"username":"A","last_login":S,"persisted":false}]}`, where decoded string `S` contains 20 MiB of backslashes; after hello, `{"req":"auth_logout","account":"unknown"}` reaches `forget_account`, whose wrong-type error becomes an approximately 80 MiB serialized refusal. Startup logs an unreadable index and continues, so this state is reachable. Smallest fix: bound the formatted index error with `bounded_message` and add framed-request coverage for this malformed-index case.

- **Not checked:** Builds, tests, gates, runtime behavior, or live traffic, as instructed; execution-dependent criteria remain **not verified**. List and Quote response sizes were excluded as requested. No goldens or reference files changed.

- **Nitpicks:** The finding requires an unusually large malformed index and is unlikely in ordinary use, but directly violates the explicit universal refusal bound.

- **Brief:** No brief is named in the commit; I used the prompt’s explicit done-when criteria.
- **Plan:** No change suggested beyond covering errors originating from persisted data.
- **Prompt:** Explicitly distinguish request-controlled inputs from arbitrary malformed persisted or upstream data if the intended refusal guarantee is narrower.
