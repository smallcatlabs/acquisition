`transport-seam-design.md` at `855ddcf6`: the seam is buildable, but the brief needs corrections beyond the owner’s §7 rulings before its build briefs are ready.

The numbers and lines of §1:

| Checked | Result |
|---|---|
| Committed brief versus current file | Identical; current HEAD is `855ddcf6`. The inspected code and rulings have no diff from `e812a0d7`. |
| One client; construction and timeouts | Right: `choke.rs:201`, builder 230–244, constants 216–217, configuration 240–241. No test names either constant. |
| Four send sites | Right: HEAD 564, POST 1000, bearer GET 1028, unauthenticated GET 1057; its daemon caller is `kinds.rs:817`. |
| Twelve `reqwest` lines in `choke.rs` | Right, including comment line 198 and diagnostic string line 243. |
| Type leakage | Status types at `choke.rs:109` and `kinds.rs:1014,1044`, and builder at `auth.rs:63`: right. **Five**, not six, `HeaderMap` sites in `policy.rs`: 111, 338, 427, 523, 590. Line 524 names `reqwest::header::LOCATION`. |
| Lock and manifests | Right: `http` 1.5.0 is locked and absent as a direct manifest dependency; reqwest is declared with default features off, `json` and `rustls-tls`. Locked reqwest is 0.12.28. |
| Check 5 | Right: reqwest is forbidden to search/store at `arch-check.sh:311,313`. Adding `http` to the daemon violates no listed edge. Cached dependency manifests also show its existing `std` feature is already enabled through reqwest. |
| Production listeners | Right: callback TCP at `login.rs:23`, mock TCP at `mockggg.rs:658`, frontend endpoint at `endpoint.rs:47` or its Windows named-pipe implementation. |
| Fixture bind counts | All right: harness 4; dispatch 1; bearer 3; log 3; sessions 2; exit 1; login 2; auth 2; spec 12. Auth’s binds are 796 and 834; 815 and 879 are the `ChokePoint::new()` constructions. |
| `Server` occurrences | Right as **textual occurrences**: Script 19, Account 19, Token 4, Mock 5, own 11, None 14. `own` includes four documentation occurrences: there are **seven calls**, backed by **five distinct bind sites**. |
| Harness/oracle citations | Right: `Received` 344; scripted server 373/381; delayed server 436/445; answering token server 1135/1138; account server 1282/1290; oracles 881, 947, 979, 999, 1095; door finish 1211. |
| Census | Independently counted **197 attributed tests across the 20 scanned files**; 19 files contain tests. The classification breakdown was not independently reproduced; see below. |
| Spec tests and clocks | Right: 16 tokio tests, 49 synchronous tests, six `with_clock_and_rails` sites, each using `ParkingClock`; no executable `ChokePoint::new()` call. |
| Door calls | Right: `hold_clock` 3, `quiet` 8, `beside` 2. Also counted `settle`: 15. Named locations match. |
| Hook ordering | Right: quiet → scenario holds → occupancy → blocking count → settle → advance → stuck verdict (`door.rs:267–302`). |

Findings, most severe first:

1. **§2, §3 and §6 step 1 — Removing the client user-agent changes HTTPS proxy behaviour, contradicting “no behaviour change.”**  
   Today [choke.rs:238](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:238) configures it on the client. Locked reqwest passes that default into its connector (`async_impl/client.rs:663`) and adds it to HTTPS proxy `CONNECT` (`connect.rs:863–866`); a prepared-request header does not supply that connector value. The proposed direct-listener tests and an ordinary direct live rung miss this path. **Smallest correction:** retain the client default from the same authoritative user-agent value while also preparing the request header, and include the proxy path in the wiring evidence.

2. **§4 and §6 step 3 — The promised unchanged pacing oracle cannot validate the new lost-send row followed by a correctly paced send.**  
   [harness.rs:1003](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/harness.rs:1003) requires a status before recognising the headerless counted case; after a timeout with no status, its fallback demands zero wait at line 1040. The choke point instead calls `note_lost_send` at `choke.rs:814`. A row ending at the timeout would avoid the disagreement but would not pin the promised subsequent pacing. **Smallest correction:** explicitly amend the oracle for counted sends lost before headers, and make the row exercise the next send.

3. **§2, §4 and §6 — The deadline’s named holder covers less than the whole-request timeout being retired, and step 1 inherits a step-3 test.**  
   The existing timeout spans headers and body (`choke.rs:213–241`); body consumption occurs at [choke.rs:840](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:840). A silent-before-headers row cannot detect restarting a fresh budget for the body. Step 1 nevertheless requests all §2 wiring tests, including daemon-clock expiry, while retaining reqwest’s timers. **Smallest correction:** assign deadline-expiry wiring to step 3, and add a case where headers consume part of the budget and the body expires at the original instant. Cover status precedence, cancellation and permit release there. `within_on` is a useful select shape, but takes a relative duration; the new helper must take the absolute instant and preserve unconstrained timer polling.

4. **§3 and §6 step 2 — `beside` registration assumes a shared clock that an existing caller does not supply.**  
   One caller shares its first door’s clock (`dispatch.rs:1122`); the other constructs [a new `ParkingClock` at line 2296](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/dispatch.rs:2296). Moreover, registering only blocking counts and a verdict slot omits the holds, quiet/settle slots and clock sleepers that determine the hook’s decision (`door.rs:204–216`). **Smallest correction:** specify how a beside door receives the running hook and clock, registers its complete accounting before restoration waits, and unregisters on teardown. Explicitly migrate the separate-clock caller.

5. **§2–§4 — Recording transport arrival does not justify deleting `quiet`’s entire arrival precondition.**  
   An async function records nothing merely when called; recording must happen during its first poll before any suspension. More substantially, the daemon can park **before reaching the transport**, on pacing or blocking credential work. Quiet precedes both kinds of accounting at [door.rs:267](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:267), so it can still return before the request arrives. **Smallest correction:** require recording before the transport’s first `Pending`, retire the socket-handoff qualification, and retain an arrival/held-request requirement wherever an assertion depends on that request having reached the fixture. A transport hold released after observing the scenario’s release is sound; it does not itself prove arrival or permit clock-driven progress while held.

6. **§7 — The draft ruling exceeds the actual boundary and embeds the proposed mechanism in its property sentence.**  
   “Every send the daemon makes” exceeds invariant 1’s GGG-request boundary and C130’s exclusions; “nothing a scenario starts is real I/O” contradicts the retained callback GET at [door.rs:850](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:850), callback bind at `login.rs:23`, and the design’s own §2. The byte claim is correct: **792 bytes**, 793 with newline, when flattened and numbered C161. **Smallest correction:** scope the property to GGG/provider exchanges, explicitly preserve callback/frontend exclusions, and move private ownership and fixture-hold mechanics after the bold sentence. The Why/Pinned form is otherwise suitable.

7. **§6–§7 — Recording the C160 amendments at the closes reverses the repository’s required ruling order.**  
   Step 2 changes its occupancy accounting and beside qualification; step 3 changes its timeout exception, while the brief defers registry changes to their closes. [AGENTS.md:50](/Users/tom/Development/GitHub/smallcatlabs/acquisition/AGENTS.md:50) requires recording the ruling before dependent code. **Smallest correction:** place the corresponding owner-approved amendment before each implementation step; let the close update pins and record findings. Scope the replacement timeout sentence to request deadlines rather than declaring broadly that “no timer is off the clock.”

8. **§2, §4 and §7 — “Unchanged” conflates the journal schema, error wording and the meaning of fixture evidence.**  
   At [choke.rs:950](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:950), only an outer transport failure supplies journal `error`; a landed response with a body failure keeps `error` null. Thus §7’s promise that any deadline-cut send puts the new words there would change that contract for body expiry. Also, current `.to_string()` produces reqwest’s “error sending request for url (…)”, not its nested cause’s “operation timed out”. Finally, `Received` currently reads parsed wire data (`harness.rs:393–401`); after migration it proves transport input. **Smallest correction:** state the two deadline phases’ exact field/word behaviour and describe the oracle as journal-versus-transport evidence, with actual serialization held by the retained TCP tests. That composition is reasonable; its evidence is different.

9. **§3–§4 — The occupancy exclusions’ retirement has a conditional holder despite an existing concrete pin.**  
   `server.rs:405,446` are production reads, not tests. The explicit zero/one/released assertions already exist at [dispatch.rs:2565–2589](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/dispatch.rs:2565), and the exclusions name that test as their evidence. Removing the hook read should let those mutants fail without starving virtual time. **Smallest correction:** explicitly retain and name those assertions as the holder, or migrate their daemon half to protocol occupancy reads before retirement. Do not defer identifying the holder to the final mutants compare.

10. **§3–§4 — Text occurrences have become a false `Server::own` retirement inventory.**  
    The ten listed non-callback fixture binds are real, but five belong to component fixtures in `log.rs` and `sessions.rs`; they are not `Server::own` binds. Its seven calls are in `bearer.rs`, `exit.rs` and `dispatch.rs`. **Smallest correction:** call this “ten scenario/component fixture bind sites”, distinguish them from the seven own-builder calls, and include the component constructors in the migration inventory.

What I did not check and why:

- No builds, tests, gate stages, project scripts, mutations or live traffic were executed. Consequently, the census classification split, claimed CI results, registry-check results and mutants artifacts remain unverified.
- The `151-stuck-verdict-after` baseline is a sensible **conditional** baseline once its compare is accepted; its remote completion and contents were not checked.
- No execution proves the transport-hold mechanism yet. Its first-poll, body-lifetime, cancellation and concurrent-request behaviour needs the proposed measurement and boundary cases.
- The before/after league windows fit the recorded roadmap. A live rung does not necessarily exercise proxies or code exchange—the latter is skipped when the account already has a UUID (`tracer-rung.sh:393`). Those paths need explicitly named evidence.
- Retaining the 10-second production connect bound remains compatible with fully virtual in-process tests. Retiring it is an owner-approved behaviour tradeoff, not preservation of that bound. The two offered choices are workable, but not exhaustive: a clock-backed connector bound is another, larger design.

Nitpicks: §1’s six-versus-five `HeaderMap` wording; “call” versus “first poll” once the required ordering is explicit. The fixture inventory error affects migration scope and is not merely a nitpick.

Step 1’s build brief may **not** be written from this design as it stands: first resolve the proxy user-agent regression and remove its dependency on step 3’s expiry test.
