`roadmap/briefs/t5-r1.md` at `5154fd1d`, unchanged in the working file: **not fully buildable as drawn; auth-notification coverage remains incomplete.**

1. **Answered** — `DoorBuilder::run` creates the runtime first and constructs inside `block_on`, with plain `#[test]` callers; this matches the cited fixtures and spike.
2. **Answered** — `await_terminal` takes the spike’s subscribe-before-read algorithm while reading through `Request::Status` and `Request::Result`, avoiding `held`.
3. **Not answered in full** — the added login, restore, refresh and rename publications match the tree, but the inventory still omits two publications in bearer rejection.
4. **Answered** — `await_auth(predicate)` defines completion even before its first read; `!pending` works for an initiated login, with notifications treated as hints.
5. **Answered** — check 10 becomes authoritative and the mock grep excludes comment lines, consistent with check 9’s filter.
6. **Answered** — `ParkingClock` extends through line 249, and `Row` is now a struct.

Findings, most severe first:

- **Do 3 / done-when 7 — The notification inventory still misses observable auth publications.** Bearer rejection clears `access_expires_at` through `refresh.rs:249–252` and `daemon.rs:661–668`; its failed credential save changes `keyring` at `refresh.rs:317–323`. Both affect `AuthStatus` (`server.rs:79–86`), so a matching `await_auth` predicate can remain asleep. **Smallest change:** add notifications after these two publications, outside their `shared` holds, and include both in the commit’s site inventory.

- **Done-when 4 — The acceptance criterion retains the superseded door API and runtime restriction.** `roadmap/briefs/t5-r1.md:239–243` requires `Door::run` and excludes only multi-thread Tokio callers, whereas Do 4 at `:155–168` specifies `DoorBuilder::run` and excludes every Tokio-test caller. **Smallest change:** name `DoorBuilder::run` and require every door caller to be a plain `#[test]`.

Not checked: compilation, tests, gates, timing or historical CI outcomes, because this was a read-only confirming review with execution prohibited. No later-round dependency was found in the revised Do items.

Nitpicks: the second new finding is editorial; original finding 6 was also a nitpick and is resolved.

Round 1 may start as drawn: **no; complete the auth-publication inventory and align done-when 4 first.**
