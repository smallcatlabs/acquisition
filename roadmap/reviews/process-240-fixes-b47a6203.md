`8be3a57d^..b47a6203` — the ruling and earlier fixes landed, but the done-when does not fully hold: history inspection and printed recovery still have gaps.

- **P2 — [.githooks/pre-push:36](/Users/tom/Development/GitHub/smallcatlabs/acquisition/.githooks/pre-push:36), shape 1:** A shallow clone can hide Claude commits from the supposedly complete history inspection. **Failing persisted input:** the remote has no `main`, and `cloud` contains a Claude commit followed by a person’s commit; a depth-one clone of `cloud`, with the hook enabled, sees only the person’s commit and permits `HEAD:main`, although the remote already holds the prohibited ancestor. **Smallest fix:** refuse inspection from a shallow repository, explain how to fetch complete history, and pin this sequence with a shallow-clone breaker.

- **P3 — [.githooks/pre-push:43](/Users/tom/Development/GitHub/smallcatlabs/acquisition/.githooks/pre-push:43), shape 6:** Dynamic arguments enter the printed commands without shell quoting. **Failing push input:** `git push '/tmp/remote repo.git' cloud:main` produces a recovery push whose remote path splits into two arguments; a valid branch containing `$` likewise undergoes shell expansion. **Smallest fix:** shell-quote every dynamic command argument through one function, and extend the printed-command breaker with these inputs.

- **Not checked:** execution results, breaker counts and gate status are **not verified**, as requested; no builds, tests or checks were run. GitHub’s server ruleset was outside the supplied evidence. No daemon, frontend-request or GGG-response path changed.
- **Nitpicks:** the second finding concerns uncommon push arguments; I would call it a nitpick for this repository’s usual `origin` workflow.
- **Brief:** the supplied issue has no explicit done-when list; I used its “No” outcome and the saved reviews’ findings as acceptance criteria.
- **Plan:** no separate plan was supplied.
- **Prompt:** the range also fixes #238, whose issue text was absent; its saved review supplied the fix-round criteria.
