`24052df0^..65aca586` — the brief’s done-when does not fully hold: check 5 does not enforce the transport boundary by package identity.

- **[P2] [tools/arch-check.sh:209](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/arch-check.sh:209) — shape 1.** The reverse-edge reader discards package ids, allowing a different package named `acquisition-transport` to link reqwest. Concrete repository input—not client, persisted or upstream input—is a daemon dependency aliased as `transport_bridge`, pointing to an external `acquisition-transport` at another version whose library re-exports reqwest. Its normal edge becomes the same `normal acquisition-transport` row as the legitimate crate’s and is deduplicated; the other guards permit the daemon’s dependency. This provides a second client without changing the reviewed transport crate’s API. Retain source package ids, allow only the actual workspace transport’s id (and daemon’s id for dev edges), and add this case to the existing breakers.

Not verified: builds, tests, gate statuses, breaker execution, mutation counts and compiler experiments; execution was prohibited. External issue and live CI state were not fetched.

Nitpicks: none. The duplicate-name case is uncommon, but it defeats the boundary this round promises to enforce.

Brief: no additional obstacle.

Plan: no additional obstacle.

Prompt: no change suggested.
