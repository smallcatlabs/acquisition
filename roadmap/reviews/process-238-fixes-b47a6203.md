`81cc5a1f^..b47a6203` — done-when does not fully hold: the push check can still admit Claude commits through shallow history.

- **P2 — [.githooks/pre-push:33](/Users/tom/Development/GitHub/smallcatlabs/acquisition/.githooks/pre-push:33), shape 1:** The fallback treats locally traversable history as every reachable commit, so shallow history can bypass the refusal. **Failing persisted Git input:** remote `main` holds A; remote `cloud` holds A → C (Claude committer) → H (human committer). A depth-one clone of `cloud` pushing `cloud:main` lacks A and C, so `git log H` reads only H and the hook allows C onto `main`. **Smallest fix:** refuse main updates from shallow repositories, print instructions to fetch complete history, and add a breaker for this sequence.

- **Not checked:** Gate status, breaker results and test counts are **not verified**; execution was prohibited. Server ruleset configuration and historical “first landed” hashes were not independently checked.
- **Nitpicks:** None.
- **Brief:** The supplied issue has no explicit done-when list; quote removal and automatic refusal served as its acceptance criteria.
- **Plan:** No separate plan was supplied.
- **Prompt:** The range also implements #240, but only #238’s issue text was supplied; include both issue texts when reviewing both concerns together.
