`transport-seam-design.md` at `94339bf3`: the seam is buildable, but its later build briefs need corrections beyond the owner’s §7 rulings.

The numbers and lines of §1:

| Checked | Result |
|---|---|
| Committed brief versus working file | Identical; HEAD is `94339bf3`. The cited code has no diff from `e812a0d7`. |
| Client and construction | Right: `choke.rs:201`, builder 230–244, user-agent 238, timeout constants 216–217 and configuration 240–241. No test names either constant. |
| Four send sites | Right: HEAD 564, POST 1000, bearer GET 1028, unauthenticated GET 1057; its caller is `kinds.rs:817`. |
| Twelve `reqwest` lines | Right, including the comment and diagnostic string. |
| Type leakage | Right: status at `choke.rs:109`, `kinds.rs:1014,1044`; five `HeaderMap` sites at `policy.rs:111,338,427,523,590`, `LOCATION` at 524; authorization builder at `auth.rs:63`, called at `choke.rs:566,1028`. |
| Dependency versions and features | Right: reqwest 0.12.28, declared with defaults off and `json`, `rustls-tls`; `http` 1.5.0 locked, absent from workspace manifests. Reqwest already enables `http`’s default features. |
| Proxy user-agent citations | Right in the cached locked dependency: `async_impl/client.rs:663` passes the default to the connector; `connect.rs:863–866` puts it on CONNECT. |
| Check 5 | Right: reqwest forbidden to search/store at `arch-check.sh:311,313`; a daemon dependency on `http` violates no listed edge. |
| Production listeners | Right: callback at `login.rs:23`, mock at `mockggg.rs:658`, frontend Unix listener at `endpoint.rs:47` or Windows named pipe. No additional production listener found. |
| Fixture bind counts | All right: harness 4, dispatch 1, bearer 3, log 3, sessions 2, exit 1, login 2, auth 2, spec 12. Listed locations match. Auth’s 815/879 identify constructions, not binds; the brief distinguishes them. |
| `Server` occurrences | Right: Script 19, Account 19, Token 4, Mock 5, own 11, None 14. `own` has seven calls over five binds. **None has thirteen selections plus the match arm at `door.rs:652`; §3’s “14 sites” is misleading.** |
| Harness and oracle locations | Right: `Received` 344; scripted 373/381; delayed token 436/445; answering token 1135/1138; account 1282/1290; oracles 881, 947, 979, 999, 1095; finish 1211. |
| Census | Counted 197 test attributes across twenty files, nineteen containing tests. The stated classification totals sum correctly; their individual classifications were not independently reproduced. |
| Spec clocks and tests | Right: sixteen tokio attributes, forty-nine synchronous attributes, including the generated property at 3842; six `with_clock_and_rails` sites, all using `ParkingClock`; no executable `ChokePoint::new()` call. |
| Door calls | Right: `hold_clock` 3, `quiet` 8, `beside` 2; also counted fifteen `settle` calls. Named locations match. |
| Hook ordering | Right: quiet → scenario holds → occupancy → blocking count → settle → clock advance → stuck verdict, `door.rs:267–302`. |
| Draft ruling size | Right: flattened with `C161`, 781 bytes, 782 with newline. |

Findings, most severe first:

1. **§3, §4, §5, §6 and §8 — The claim that the transport hold retires #151’s ten timeouts is unsupported by the actual tests and omits the tenth mutation.**  
   This is one inventory-and-holder error, affecting every repetition of that claim (shapes 1 and 10).

   | Mutations named or omitted | Concrete reading |
   |---|---|
   | `setup_after_probe_429`, `degrade_for`, both `parse_retry_after` caps | Their hanging test is explicitly a component test at [dispatch.rs:387](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/dispatch.rs:387). It awaits its first terminal result at 413 before reaching its clock-driving helper. The mutations can leave pacing sleepers; removing a fixture hold does not move that clock. |
   | HEAD’s `delete !`, `||` → `&&`, `record_completed` → `()` | Their hanging test is another component test, [dispatch.rs:477](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/dispatch.rs:477). Its condition loop at 508–526 repeatedly yields while waiting for a waiting probe **and** a halt. There is no door hook or pending door verdict; yielding does not produce the proposed decisive park. |
   | Both `verdict_of` mutations | The hanging mark-file test is explicitly a component case at [dispatch.rs:622](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/dispatch.rs:622). Its `wait_terminal` at 651 uses the polling helper at `harness.rs:665`; a misclassified 429 can leave the retry waiting on its parking clock. |
   | **Omitted:** HEAD’s `status == Some(401)` → `!=` at `choke.rs:701` | The local run log identifies both concurrent bearer scenarios and stress still running. The bearer scenarios await fixture arrivals through plain `timeout(HANG, held.recv())` at [bearer.rs:394](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/bearer.rs:394), before releasing at 406. That wait does not install the door’s stuck-verdict slot. A missing expected arrival is not established as catchable by the proposed accounting. |

   Thus the row names **nine** mutations under **three component tests**, not ten under three door scenarios. Their local logs end through the mutants runner’s cancellation, around 21 seconds; they do not demonstrate reqwest’s 60-second timeout ending the waits. The separately named `observe_landed` and `sleep_until` component hangs remain outside this mechanism’s reach, but they are not the only ones.

   **Smallest change:** replace the blanket retirement with this actual inventory, include the omitted mutation, and leave these timeouts outstanding unless a separately specified holder removes each. Keep the proposed baseline and compare, but do not prescribe `held by the verdict` before identifying a verdict that these waits actually consume.

2. **§2, §5 and §7 — The corrected body-lifetime accounting still has contradictory sibling specifications.**  
   §2 now correctly carries the hold through a fixture-controlled body, matching [gate.rs:3](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/gate.rs:3) and the headers-then-release fixture at [spec.rs:2696](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/tests/spec.rs:2696). But [brief §5:479](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/transport-seam-design.md:479) still releases the count “at the answer or at the observed release.” The draft ruling’s unconditional first-poll-to-body-end sentence also omits §2’s necessary silent-phase exception. This leaves two incompatible accounting contracts (shape 3).

   **Smallest change:** make §5 refer to §2’s single lifetime definition; qualify the ruling’s hold as covering fixture-controlled waits, with silent or clock-driven phases permitting clock advancement. Specify whether the +20-second header delay uses explicit scenario advancement or an unheld clock sleep. Retain the held-body, silent-body and cancellation boundary rows.

3. **§8 — The scope correction still leaves the claim that the design provides “no real I/O inside a scenario.”**  
   [Brief:630](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/transport-seam-design.md:630) repeats the wider promise despite §2 retaining the real callback GET, whose current implementation is [door.rs:850](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:850). The corrected ruling otherwise preserves invariant 1’s boundary and C157’s real-wiring tests (shape 3).

   **Smallest change:** say “no real provider I/O inside a door scenario.”

4. **§3 — `Server::None`’s fourteen textual occurrences are described as fourteen sites.**  
   One is the implementation’s match arm at [door.rs:652](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:652); thirteen select that fixture.

   **Smallest change:** say “fourteen occurrences, thirteen selections.”

The second review’s ruling-order, positive-header-delay and duplicate-sentence findings are answered concretely. Its body-lifetime and scope findings are substantially answered, with the siblings above remaining. The 20/60/80 expiry row now distinguishes a single absolute deadline from a restarted body budget.

What I did not check and why:

- No builds, tests, project checks, mutations, daemon runs or live traffic were executed, as requested. Reported green checks and future runtime behaviour remain unverified.
- `gh issue view 151 --comments` failed to reach GitHub; the public-page fallback also failed. I inspected `73f182bd` and the locally available outcomes, mutation diffs and relevant logs instead. I did not independently reproduce the 402-pair comparison.
- The census classification split was not independently reconstructed.
- The live rung cannot guarantee code-exchange or proxy coverage; the brief now acknowledges both. Keeping the client’s default user-agent preserves the inspected CONNECT path.
- I found no forbidden crate edge, required exposure of `Secret`’s private text, or existing non-test `reqwest` reference defeating check 13’s proposed exemptions. The deadline/helper shape, mock split and complete `beside` registration are buildable. Connect option (a) is a coherent owner tradeoff; option (b) preserves the existing shorter production bound, and the larger clock-backed connector alternative is acknowledged.

Nitpicks: findings 3 and 4. Finding 2 specifies the accounting mechanism and is more than editorial.

Step 1’s build brief **may be written independently**, subject to the owner settling and entering §7 before implementation; the later briefs need the corrections above.
