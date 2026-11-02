6ca5b9a3^..42c9a7b7 — the brief’s done-when does not fully hold: check 13 has alias gaps, and the HEAD case does not pin its claimed pacing property.

Findings, most severe first:

- **[P2] tools/arch-check.sh:822 — shape 1.** The manifest check misses inherited dependency aliases, allowing a sender outside the transport file without detecting it. Concrete manifest input: declare `web = { package = "reqwest", … }` in `[workspace.dependencies]` and `web = { workspace = true }` in the daemon; use `web::` in the transport and another daemon module. The existing `"reqwest client builds"` string still satisfies `home > 0`. Resolve inherited entries before checking their package name, and add a breaker for this form.

- **[P2] tools/arch-check.sh:781 — shape 1.** The import check misses crate aliases inside root import groups. Concrete source input inside `transport.rs`: `pub(crate) use {http::Method, reqwest as web};`. Neither alias regex matches, and the word check permits it there; re-exporting `web` through `choke` then lets another module construct a client without spelling `reqwest`. Recognize grouped import trees—or reject unsupported forms—and add a breaker inside the permitted transport file. Both alias findings share the cause of checking selected spellings rather than resolving the names being guarded.

- **[P3] crates/acquisition-daemon/src/ratelimit/tests/spec.rs:2438 — shape 2.** The invalid-bearer HEAD row never observes whether the failed probe added a lost-send hit, although its documentation and brief claim that property is pinned. With the synthetic `c161-token\ntext` token—representing malformed upstream or persisted keyring input, not client request input—adding `note_lost_send` before degradation would leave every current assertion satisfied. Establish a second route sharing the policy and assert that its pacing remains clear after the HEAD fails.

Not verified: builds, tests, gate statuses and breaker execution; this was a read-only review as requested.

Nitpicks: finding 3 concerns malformed token input unlikely in practice, though its missing assertion is explicitly required by the brief.

Brief: specify the alias rule independently of check 12’s implementation, so copying that reader does not define the boundary’s coverage.

Plan: no additional obstacle.

Prompt: no change suggested.
