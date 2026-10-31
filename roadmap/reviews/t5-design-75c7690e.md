[The brief](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-design.md) at `75c7690e` matches the working file: **yes**, round 1’s build brief may be written after the owner rules on §5.

1. **Answered:** the auth-change notification covers failure before a profile job, completion after that job, and logout; subscribe-before-read through `AuthStatus` addresses the races in `daemon/login.rs:472`, `:364`, `:521` and `protocol.rs:455`.
2. **Narrower (sound):** all five `responses.rs` tests remain (`:25`, `:36`, `:92`, `:111`, `:129`); the corrected existing holders are dispatch `:518`/`:553` and `:595`/`:744`, with three new twins. No retirement means no missing C154 holder.
3. **Answered:** round 2 now adapts every `Life` caller, including the retained restore/drain path at `credentials.rs:564`, `:781` through `harness.rs:633`.
4. **Narrower (sound):** §3 confines door assertions to boundary outputs and retains classification as a component property, consistent with the ruling; stale wording remains below.
5. **Answered:** the scratch path overrides absent or supplied journal configuration, construction fails if opening fails, and `None` scenarios retain journals; this matches the requirements exposed by `daemon.rs:1232`, `rails.rs:447–480` and `harness.rs:506`.
6. **Answered:** the ruling fits the byte limit, states the property and component partition, and uses `nothing yet`; the timeout trigger now has the registry’s form.
7. **Answered:** `daemon.rs:267` is the drain-deadline assignment.

- **§5 — stale classification wording contradicts §3’s revised observation boundary.** [Brief:341](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-design.md:341) still says the design reads a daemon-internal enum, whereas `:269` keeps classification in component tests. **Smallest change:** replace that sentence with “Classification stays in component tests”; retain the separate `Outcome` question.
- **§2.6 — stale retirement wording contradicts the decision to retain `responses.rs`.** [Brief:227](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-design.md:227) says “until its holders land,” while `:253` and `:301` explicitly retain it without retirement. **Smallest change:** delete “until its holders land.”
- **§3 and round 4 — stale waiter names refer to an operation §2.1 no longer defines.** [Brief:255](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-design.md:255) and `:305` name `await_state`, while `:146` specifies `await_auth`. **Smallest change:** use `await_auth` for login waits and the applicable event wait in round 4.

The ruling bullet alone, unwrapped through its complete *Pinned:* clause, is **745 UTF-8 bytes**, including `- ` and `C<next>`, excluding the newline and following explanatory paragraph.

Not checked: builds, tests, gate results, runtime determinism, timing or mutation evidence, as requested. I did not repeat the full census or independently verify remote issue comments or artifacts.

Nitpicks: all three remaining findings are editorial residue; none changes round 1’s design or introduces a dependency on a later round.

Round 1’s build brief may be written from this design as it stands: **yes, after the owner rules on §5**.
