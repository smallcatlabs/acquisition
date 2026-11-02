`roadmap/briefs/transport-seam-design.md` at `b1455e76` is buildable; its build briefs can be written after the owner settles §7, with one nonblocking citation nit below.

The numbers and lines of §1:

| Checked | Result against the tree |
|---|---|
| Committed brief versus working file | Identical; HEAD is `b1455e76`. Cited production sources are unchanged from `e812a0d7`. |
| Client and configuration | Right: `choke.rs:201`, builder 230–244, user-agent 238, timeout constants 216–217, configuration 240–241. No test names either constant. |
| Four send sites | Right: HEAD 564, POST 1000, bearer GET 1028, unauthenticated GET 1057; its daemon caller is `kinds.rs:817`. |
| Twelve `reqwest` lines | Right, including the comment and diagnostic string. |
| Type leakage | Right: status at `choke.rs:109`, `kinds.rs:1014,1044`; authorization builder at `auth.rs:63`, called at `choke.rs:566,1028`; five `HeaderMap` sites at `policy.rs:111,338,427,523,590`, plus `LOCATION` at 524. |
| Dependencies and features | Right: reqwest 0.12.28, defaults disabled, `json` and `rustls-tls`; locked `http` 1.5.0, absent from manifests. Reqwest already enables `http`’s default features. |
| Proxy user-agent citations | Right in the cached locked dependency: `async_impl/client.rs:663` supplies it to the connector; `connect.rs:863–866` puts it on CONNECT. |
| Check 5 | Right: reqwest forbidden to search/store at `arch-check.sh:311,313`; adding `http` to the daemon violates no listed edge. |
| Three production listeners | Right: callback `login.rs:23`, mock `mockggg.rs:658`, frontend Unix listener `endpoint.rs:47` or Windows named pipe. |
| Fixture binds | All right: harness 4, dispatch 1, bearer 3, log 3, sessions 2, exit 1, login 2, auth 2, spec 12. Listed locations match; auth’s 815/879 are constructions, its binds 796/834. |
| `Server` occurrences | Right: Script 19, Account 19, Token 4, Mock 5, own 11, None 14. `own` has seven calls over five binds; None has thirteen selections plus the match arm. |
| Harness and oracle locations | Right: `Received` 344; scripted 373/381; delayed token 436/445; answering token 1135/1138; account 1282/1290; oracles 881, 947, 979, 999, 1095; door finish 1211. |
| Census | Reproduced 197 attributed tests across twenty files, nineteen containing tests. The stated classification totals sum correctly; classification was not independently reconstructed. |
| Spec tests and clocks | Right: sixteen tokio attributes, forty-nine synchronous attributes; six `with_clock_and_rails` sites, all using `ParkingClock`; no executable `ChokePoint::new()` call. |
| Door calls | Right: `hold_clock` 3, `settle` 15, `quiet` 8, `beside` 2. Named locations match. |
| Hook ordering | Right: quiet → scenario holds → occupancy → blocking count → settle → advance → stuck verdict, `door.rs:267–302`. |
| §7 draft size | Right: flattened with `C161`, 787 bytes; 788 with newline. |

Findings, most severe first:

1. **§3 — The first dispatch citation points into a case rather than to the test’s component rationale.** `dispatch.rs:387` is `"non-Full"`; the named test starts at [dispatch.rs:377](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/dispatch.rs:377), and its component justification is at 374–375. The attribution itself is correct. **Smallest change:** cite 374 or 377 instead of 387.

No substantive findings. The fourth review’s finding is answered completely: duplicate-preserving pairing reproduces **402 pairs, 36 timeout→caught and 67 timeout→timeout**. Of the original 48 caught→timeout cases, the eleven remaining timeouts are all named: four under the first dispatch component test, three under the blocked-probe component test, two under the mark-file component test, `observe_landed`’s comparison, and `finish_send`’s `&&`→`||` under stress. The saved stress log confirms the omitted case. The 401 guard and `sleep_until` removal are correctly separate.

“Claims none of them” is supported. The bearer logs do not locate the precise pending future, so retaining “their terminal waits do race it and may yet benefit” is appropriately qualified.

Static inspection found the seam, body lifetime, absolute deadline, header authorization, mock split, callback browser path and complete `beside` registration buildable. The named retirement holders are sufficient as design requirements; their implementation remains to be demonstrated.

What I did not check and why:

- No builds, tests, gate stages, mutations, daemon runs or live traffic were executed. Claimed execution results remain unverified.
- Saved outcomes and relevant logs were inspected locally; their remote provenance and CI status were not verified.
- The census classification split was not independently reconstructed.
- Runtime correctness of transport holds, cancellation, registration and expiry remains for the proposed measurements and boundary rows.
- The live rung cannot guarantee code-exchange or proxy coverage; the brief states both limitations. Connect option (a) is a coherent owner tradeoff; (b) retains the shorter production bound. The larger clock-backed connector alternative is acknowledged.

Nitpicks: finding 1 only.

Step 1’s build brief **may be written from this design as it stands**, subject to the owner settling and entering the applicable §7 rulings before implementation.
