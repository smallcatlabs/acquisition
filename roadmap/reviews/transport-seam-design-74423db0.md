`transport-seam-design.md` at `74423db0`: the seam is buildable, but its build briefs need the corrections below in addition to the owner’s §7 rulings.

The numbers and lines of §1:

| Checked | Tree’s value |
|---|---|
| Committed brief versus working file | Identical; HEAD is `74423db0`. The inspected code, rulings and checks are unchanged from `e812a0d7`. |
| Client and configuration | Right: field `choke.rs:201`, builder 230–244, user-agent 238, timeouts 216–217 and 240–241. No test names either timeout constant. |
| Four send sites | Right: HEAD 564, POST 1000, bearer GET 1028, unauthenticated GET 1057; the last’s daemon caller is `kinds.rs:817`. |
| Twelve `reqwest` lines in `choke.rs` | Right, including comment 198 and diagnostic string 243. |
| Type leakage | Right: `CompletedResponse` at 109; `kinds.rs:1014,1044`; `auth.rs:63`; five `HeaderMap` sites at `policy.rs:111,338,427,523,590`, plus `LOCATION` at 524. |
| Lock, manifests and feature claim | Right: locked `http` 1.5.0, no direct manifest dependency; reqwest 0.12.28, default features disabled, `json` and `rustls-tls`. Cached manifests confirm `http/std` is already enabled through reqwest. |
| Proxy citations | Right and locally readable: `async_impl/client.rs:663` passes the client user-agent into the connector; `connect.rs:863–866` adds it to HTTPS proxy `CONNECT`. Keeping the client default answers prior finding 1. |
| Check 5 | Right: reqwest forbidden to search/store at 311/313; adding `http` to the daemon crosses no forbidden edge. |
| Production listeners | Right: callback TCP `login.rs:23`, mock TCP `mockggg.rs:658`, frontend endpoint `endpoint.rs:47` or its Windows named-pipe implementation. |
| Fixture bind counts | All right: harness 4, dispatch 1, bearer 3, log 3, sessions 2, exit 1, login 2, auth 2, spec 12. Auth’s binds are 796/834; 815/879 are its choke-point constructions. |
| `Server` occurrences | All right as textual occurrences: Script 19, Account 19, Token 4, Mock 5, own 11, None 14. |
| `Server::own` inventory | Right: seven calls over five bind sites; the five log/session component binds are separate. This answers prior finding 10. |
| Harness and oracle locations | Right: `Received` 344; script 373/381; delayed token 436/445; answering token 1135/1138; account 1282/1290; oracles 881, 947, 979, 999, 1095; door finish 1211. |
| Census | Independently counted 197 attributed tests across 20 scanned files. Classification split not independently reproduced. |
| Spec tests and clocks | Right: 16 tokio tests, 49 synchronous attributes, six `with_clock_and_rails` sites using `ParkingClock`, no executable `ChokePoint::new()` call. |
| Door calls | Right: `hold_clock` 3, `quiet` 8, `beside` 2; additionally, `settle` 15. Named locations match. |
| Hook ordering | Right: quiet → scenario holds → occupancy → blocking count → settle → clock advance → stuck verdict, `door.rs:267–302`. |

Findings, most severe first:

1. **§2, §4 and §5 — The replacement clock accounting ends at headers although a fixture-controlled body can still be pending.**  
   The brief defines `Answer` as headers plus a future body at lines 214–219, then drops the transport hold “when the answer is given” at 265. Today the hook’s occupancy read covers the body too: [door.rs:276](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:276), [choke.rs:840](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:840). An existing fixture explicitly sends headers and then awaits release before sending its body at [spec.rs:2699](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/tests/spec.rs:2699). Under the proposed accounting, such a body no longer prevents `settle` or clock advancement; after step 3 it can expire while intentionally held. Naming “a body still pending” as a boundary row does not specify which behaviour it must pin. **Smallest correction:** transfer the hold into a fixture-controlled pending body, release it on observed release/completion/drop, and explicitly exempt silent or clock-driven bodies so expiry remains virtual. State that distinction in the named boundary row. This is a holder covering less than the retired accounting, shape 10.

2. **§6–§7 — The new seam ruling is scheduled after the first code that depends on it.**  
   Step 1 introduces the transport ownership boundary and check 13, but [the brief’s line 562](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/transport-seam-design.md:562) schedules the ruling before step 3 alone. [AGENTS.md:50](/Users/tom/Development/GitHub/smallcatlabs/acquisition/AGENTS.md:50) requires the ruling recorded before dependent code; the roadmap also identifies this seam’s structural boundary as the owner’s ruling. **Smallest correction:** enter the owner-approved seam ruling before step 1, retaining the separately timed C160 amendments before steps 2 and 3. Prior finding 7 is therefore only partly answered.

3. **§6 step 3 — The second expiry row can still pass with a fresh body budget.**  
   “Headers answered then a silent body, cut at the same instant” at lines 519–521 does not require the headers to consume any time. With headers at dispatch, both the intended absolute deadline and an erroneous fresh 60-second body timeout expire together. The existing whole-request bound at [choke.rs:241](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:241) spans the body read at 840. **Smallest correction:** require a positive header delay—for example, headers at dispatch +20 seconds and body expiry at dispatch +60, explicitly rejecting +80. The absolute-instant implementation description is sound; its proposed pin still permits the nearest case, shape 2. Prior finding 3’s step-order correction is answered, but this evidence gap remains.

4. **§2, §7 and §8 — The scope correction remains inconsistent, and the draft still puts implementation choices inside its bold property.**  
   The ruling’s “under a test it is in-process” includes the real-wiring tests retained by §6 and sanctioned by [C157, decisions/daemon.md:24](/Users/tom/Development/GitHub/smallcatlabs/acquisition/decisions/daemon.md:24). The broader “nothing a scenario starts is real I/O” remains at brief line 194 and is repeated in §8, despite the retained callback GET at [door.rs:850](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:850). The provider scope and explicit callback/frontend exclusions in §7 otherwise fit invariant 1 and C130. **Smallest correction:** scope the in-process statement to provider exchanges in door scenarios everywhere, preserve component real-wiring tests explicitly, and move the production/test implementation sentence after the bold property. The flattened draft is correctly **797 bytes**, 798 with newline; length and Why/Pinned form are sound. Prior finding 6 has surviving siblings, shape 3.

5. **§2 — The fixture-evidence sentence is duplicated.**  
   Lines 288–293 repeat that `Received` reads bearer and grant from the prepared request. [harness.rs:393](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/harness.rs:393) confirms the current wire-derived reading; the replacement’s changed evidentiary meaning is otherwise stated correctly. **Smallest correction:** delete the first copy.

The other prior findings are answered concretely: the pacing oracle gains the lost-send case and a subsequent send; `beside` gains complete registration and migrates its separate-clock caller; `quiet` retains its arrival requirement; deadline errors distinguish answer/body phases correctly against `record_completed`; and the occupancy exclusions now name the existing zero/held/released assertions at `dispatch.rs:2565–2590`.

What I did not check and why:

- No builds, tests, gate stages, project checks, mutations or live traffic were executed. Reported CI, registry-check and mutants results are unverified; the census classification split was not independently recomputed.
- Runtime correctness of the hold and registration mechanisms remains prospective. First-poll recording and scenario-task release are sound shapes; the body-lifetime correction above is still needed.
- The mutants baseline is sound conditionally on #151’s accepted comparison; its remote artifacts were not inspected.
- The league windows match the recorded roadmap. The rung can skip code exchange and does not guarantee proxy coverage; the brief now acknowledges both. Shared `token_request` wiring and keeping the client’s proxy user-agent are reasonable evidence for this refactor.
- Connect option (a) deliberately relaxes the production connect bound from 10 to 60 seconds; it does not preserve the old bound. Option (b) preserves it while allowing virtual fixtures. The recommendation is an owner tradeoff, and the brief correctly acknowledges the larger clock-backed connector alternative.
- I found no forbidden crate edge, necessary exposure of `Secret` outside `auth.rs`, or unexplained production `reqwest` site that defeats check 13. The `ratelimit.rs:13` doc comment is exempt by the proposed check’s stated shape.

Nitpicks: finding 5 only. Finding 4’s scope affects the registry and retained tests, so it is more than editorial wording.

Step 1’s build brief may **not** be written from this design as it stands: first correct the seam ruling’s scope and schedule its entry before step 1.
