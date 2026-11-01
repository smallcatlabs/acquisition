015ccce1^..34c4f4bc — the statically checkable done-when requirements hold; one documentation nitpick remains.

- **[P3] crates/acquisition-daemon/src/daemon/tests/bearer.rs:7 — shape 1:** The module claims every fixture answers by request content and records the wire, but `cloudflare_401_server` answers identical GETs with 401 then 200 by arrival order, and `bearer_aware_server` keeps no request record. This concerns simulated upstream input. Smallest fix: describe these properties per server instead of claiming them for every scenario.

- **Not checked:** Gate results, runtime behavior, Windows execution, and reported test counts/timings — **not verified**, because execution was prohibited.
- **Nitpicks:** The sole finding is documentation only; I found no behavioral regression or invariant bypass in the changed code.
- **Brief:** No change suggested.
- **Plan:** No change suggested.
- **Prompt:** The commit messages do contain execution figures despite the prompt saying they never do; I treated those figures as author claims, not verification.
