e33a8e8b^..4a1e21c3 — the brief’s done-when does not fully hold; trade-value identity and displayed fraction coverage remain incomplete.

- **P1 — [show.rs:342](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/show.rs:342), shapes 1 and 7:** Treating every template number as open can give a card a different value from its trade filter. Concrete input: client filter `explicit.stat_2918708827` with minimum 10 and the persisted upstream line `15% chance to gain Phasing for 4 seconds on Kill`, already held in `tests/trade-translate.toml`. Translation compares `.arg1` (15); the card generates `.avg` (9.5), adding a second column and ordering by a different quantity. Share the trade layer’s slot mapping, including which numbers its text leaves open, and pin equality using this existing fixture.

- **P2 — [show.rs:391](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/show.rs:391):** Numerator and denominator fields receive the same display identity, so deduplication drops the denominator. Concrete persisted upstream property: `{"name":"Wings Revealed","values":[["1/3",0]],"displayMode":0}`. Both fields occupy property value 0; `number_fields` retains only `wings_revealed: 1`, and the entire `1/3` links to that field, leaving no way to select `wings_revealed.of`. Distinguish the two numeric pieces, retain both fields, and pin their separate links using the existing fraction fixtures.

Not verified: tests, gate, documentation build, browser behavior and byte measurements; execution was prohibited.

Nitpicks: none; both findings concern supported item shapes already represented in committed fixtures.

Brief: explicitly require fraction properties and trade texts containing literal digits in the value-identity coverage.

Plan: the revised one-link shape is recorded in the commit, but its handling of trade-specific slots is unresolved.

Prompt: nothing impeded the review.
