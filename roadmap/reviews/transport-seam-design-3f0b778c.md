`transport-seam-design.md` at `3f0b778c` is buildable; after the owner rules on §7, step 1 can be briefed, but the later briefs need the inventory correction below.

The numbers and lines of §1:

| Checked | Result against the tree |
|---|---|
| Committed brief versus working file | Identical; HEAD is `3f0b778c`. Production sources are unchanged from `e812a0d7`. |
| Client and configuration | Right: `choke.rs:201`, builder 230–244, user-agent 238, timeout constants 216–217 and configuration 240–241. No test names either constant. |
| Four send sites | Right: HEAD 564, POST 1000, bearer GET 1028, unauthenticated GET 1057; the latter’s daemon caller is `kinds.rs:817`. |
| Twelve `reqwest` lines | Right, including the comment and diagnostic string. |
| Type leakage | Right: status at `choke.rs:109`, `kinds.rs:1014,1044`; authorization builder at `auth.rs:63`, called at `choke.rs:566,1028`; five `HeaderMap` sites at `policy.rs:111,338,427,523,590`, plus `LOCATION` at 524. |
| Dependencies and features | Right: reqwest 0.12.28; defaults disabled, `json` and `rustls-tls`. `http` 1.5.0 is locked and absent from manifests. Reqwest’s dependency already enables `http`’s default features. |
| Proxy user-agent citations | Right in the cached locked dependency: `async_impl/client.rs:663` supplies it to the connector; `connect.rs:863–866` adds it to CONNECT. |
| Check 5 | Right: reqwest forbidden to search/store at `arch-check.sh:311,313`; adding `http` to the daemon violates no listed edge. |
| Production listeners | Right: callback `login.rs:23`, mock `mockggg.rs:658`, frontend Unix listener `endpoint.rs:47` or Windows named pipe. |
| Fixture binds | All right: harness 4, dispatch 1, bearer 3, log 3, sessions 2, exit 1, login 2, auth 2, spec 12. Listed locations match. Auth’s 815/879 identify constructions; its binds are 796/834. |
| `Server` occurrences | Right: Script 19, Account 19, Token 4, Mock 5, own 11, None 14. `own` has seven calls over five binds. The corrected None row correctly distinguishes thirteen selections from the match arm at `door.rs:652`. |
| Harness and oracle locations | Right: `Received` 344; scripted 373/381; delayed token 436/445; answering token 1135/1138; account 1282/1290; oracles 881, 947, 979, 999, 1095; finish 1211. |
| Census | Reproduced 197 test attributes across twenty files, nineteen containing tests. The classification totals sum correctly; their individual classification was not independently reconstructed. |
| Spec tests and clocks | Right: sixteen tokio attributes, forty-nine synchronous attributes; six `with_clock_and_rails` sites, all using `ParkingClock`; no executable `ChokePoint::new()` call. |
| Door calls | Right: `hold_clock` 3, `settle` 15, `quiet` 8, `beside` 2. Named locations match. |
| Hook ordering | Right: quiet → scenario holds → occupancy → blocking count → settle → clock advance → stuck verdict, `door.rs:267–302`. |
| Draft ruling size | Right: flattened with `C161`, 787 bytes; 788 with newline. |

Findings, most severe first:

1. **§3 and its §8/closing inventory — The revised #151 inventory still omits a mutation left timing out by the comparison.** This is an incomplete evidence inventory, shape 1, rather than another unsupported retirement.

   The omitted mutation is `&&` → `||` in `ChokePoint::finish_send` at [choke.rs:807](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:807). It calls `note_lost_send` after successful counted responses. Its saved after-run log ends with the component stress test still running at [log:981](/Users/tom/Development/GitHub/smallcatlabs/acquisition/runs/151-stuck-verdict-after/mutants-shard-1/log/crates__acquisition-daemon__src__ratelimit__choke.rs_line_807_col_20.log:981); that test awaits route establishment before driving its clock at [stress.rs:96](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/stress.rs:96).

   Reading the three saved runs by the comparison’s duplicate-preserving key leaves **eleven** of the original caught→timeout mutations still timing out: the nine mutations named under the three dispatch component tests, `observe_landed`’s comparison, and this `finish_send` mutation. The 401-guard mutation and `sleep_until` removal are additional timeout cases, rather than members of those original eleven.

   **Smallest change:** add the stress mutation and distinguish that original remainder from the additional cases. Keep the withdrawal of promised retirements: no holder is established here.

The third review’s other three findings are answered sufficiently. §5 now refers to §2’s lifetime; the ruling permits silent phases to release the clock; the +20-second delay explicitly uses an unheld clock sleep, making the +60-versus-+80 row discriminating. Provider-I/O scope and the None occurrence count are corrected.

What I did not check and why:

- No builds, tests, gate stages, mutations, daemon runs or live traffic were executed. Claimed execution results remain unverified.
- I checked the local saved mutants artifacts, reproducing 402 pairs, 36 timeout→caught and 67 timeout→timeout; I did not verify their remote provenance or CI status.
- The aborted bearer logs do not identify the exact awaited future. `bearer.rs:394` bypasses the verdict, while terminal waits at 376 and 409 race it through `door.rs:965`. Consequently, “no retirement promised” is sound; a categorical claim that those scenarios cannot benefit would exceed the evidence.
- The census classification split was not independently reconstructed.
- Runtime correctness remains for the proposed boundary rows. Static inspection found the first-poll recording, body accounting, delayed/silent phases, absolute deadline, mock split, browser callback and complete `beside` registration buildable.
- The live rung does not guarantee code-exchange or proxy coverage; the brief acknowledges both. The connect-timeout recommendation is an explicit owner tradeoff, and the larger clock-backed connector alternative is acknowledged.

Nitpicks: none of the finding above; it concerns the completeness of the named comparison inventory.

Step 1’s build brief **may be written from this design as it stands**, subject to the owner settling and entering §7 before implementation.
