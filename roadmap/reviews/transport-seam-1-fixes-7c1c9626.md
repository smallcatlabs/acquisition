3365183b^..7c1c9626 — fix-round done-when does not fully hold: the inherited-alias and HEAD findings are addressed, but the source alias boundary remains incomplete.

- **[P2] tools/arch-check.sh:794 — shape 1.** The extern-crate reader still matches a spelling rather than the guarded binding, allowing a reqwest alias to escape check 13. Concrete repository source input—not client, persisted or upstream input—is `pub(crate) extern crate r#reqwest as web;` inside `transport.rs`: `EXTERN` misses the raw identifier, `USE` ignores extern declarations, and the word check permits `reqwest` in that file. Re-exporting `transport::web` through `choke` then permits another module to construct a client without spelling `reqwest`, defeating the promised protection against sends outside the choke point and gate. Read extern-crate items across the whole source, normalize raw identifiers, and add a breaker in the permitted transport file.

Not verified: builds, tests, gate statuses, breaker execution and the claimed failing-first experiments; execution was prohibited.

Nitpicks: the raw-identifier spelling is uncommon, but its bypass of the boundary this round promises to enforce is substantive.

Brief: no additional obstacle.

Plan: no additional obstacle.

Prompt: no change suggested.
