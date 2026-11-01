`roadmap/briefs/t5-r5a.md` at `17265e8f`, unchanged in the working tree: buildable as drawn, with one nonblocking wording remnant.

1. **Answered** — T3’s log assertion stays a component case; the door gains no log reader.
2. **Narrower, sound** — T1’s post-mark grant value has no remaining send-observable property: refresh refuses before creating a flight (`daemon/refresh.rs:414–419`), and logout drops the session and clears credentials without sending the grant (`daemon/login.rs:521–600`). T4’s reset clears the block (`daemon/server.rs:432`, `rails.rs:698–711`); its third check consumes the second scripted response, since the halted second check sends nothing. T7’s malformed 200 sets neither a dead-grant mark nor a halt (`auth.rs:348–364`, `daemon/refresh.rs:470–514`); its next check can cross the headerless-response hold and send `rt-old`. T11 now names both store assertions. T21/T24’s bearer reads are feasible: `HttpRequest.headers` is public and retains the lowercase `authorization` header (`mockggg.rs:981–988`, `1012–1015`).
3. **Answered** — T17 now distinguishes the first logout’s `Internal` refusal from the printed retry’s `Cleared`; T18 logs out `C#3`, matching `daemon/tests/refresh.rs:943–1050`.
4. **Answered** — T24’s `[GET, GET, POST, GET]` script matches its three submitted jobs and the server’s positional method checks (`daemon/tests/harness.rs:557–561`).
5. **Answered** — Done-when 7 scopes the implementation comparison to `crates/` from the brief’s last cut, retaining the independent golden/reference requirement.
6. **Answered** — `DoorBuilder::world` is at `daemon/tests/door.rs:381`; `TOKEN_HEADERS` is at `daemon/tests/refresh.rs:287`.

Findings, most severe first:

- **Do 2 — The token-table surface list still includes `log_text`, contradicting T3’s explicit component split and done-when 4.** Concrete reading: [t5-r5a.md:233](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r5a.md:233), versus lines 244–246 and 393–394. Smallest change: remove `log_text` from that list. The specific instructions already resolve the intended implementation.

**Do 1’s restore has no additional defect.** `begin_restore`’s `pub(super)` visibility reaches descendants of `daemon`, including the door module (`daemon/start.rs:599`). The drawn builder explicitly starts unlanded; `begin_restore` also sets the watch value false before spawning its counted landing (`599–603`). `await_auth` subscribes before reading (`daemon/tests/door.rs:778–808`), and `keyring_summary` reports `"restoring"` until the landing publishes true and notifies (`daemon/server.rs:53–60`, `daemon/start.rs:634–636`). Thus `run` can await restoration before starting the scenario. Builder `restore_landed(false)` prevents an initially landed interval; after `begin_restore` has run, that method’s own false publication also makes `restored()` wait (`daemon.rs:1541–1545`).

What I did not check: builds, tests, gate results, timings, CI or mutant artifacts; execution was prohibited. This review establishes feasibility from the committed code, not a passing implementation.

Nitpicks: original findings 4 and 6, and the remaining `log_text` list entry.

Round 5a may start as drawn; the remaining correction is editorial.
