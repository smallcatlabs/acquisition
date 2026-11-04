e2736ccf..a5e6b58c — the brief’s done-when does not fully hold: #174 and #36 have gaps; no defect found in #173.

- **P2 — `crates/acquisition-search/src/bind.rs:1541` (shape 2):** The negative reading disagrees with the refusal’s span. Client input `-priced` returns span `[1,7]` and reading `-has:priced`; replacing that span produces `--has:priced`, which means the opposite. Include the consumed negation in the replacement span, and pin replacement within the original query; the current test only runs the reading alone.
- **P2 — `crates/acquisition-search/src/answer.rs:3074` (shape 1):** `named_leagues` recognizes only lowercase field names although binding accepts names in any case. With the committed test world, client input `LEAGUE=Standard or LEAGUE=Allflame` selects unrelated Phrecia and counts two other leagues. Compare the field name case-insensitively and add that case beside the existing pointer cases; leave `fixed_league` unchanged as scoped.
- **P3 — `crates/acquisition-search/src/bind.rs:839`, `crates/acquisition-search/src/describe.rs:623` (shape 7):** The single-source requirement remains incomplete: count-key refusals independently exclude `Text/Id/Price`, and describe independently excludes their names. These currently agree, but reading them shows that changing `takes_has` would leave both lists behind. Have both filters consume `takes_has`, retaining the separate priced-key exclusion.

Not verified: builds, tests, gate status, fixture regeneration, and runtime behavior; execution was prohibited. Original seat outputs were unavailable, so their words were judged against the fixed brief’s quotations.

Nitpicks: the duplicated eligibility filters are a maintenance concern without a current input failure; the other findings affect valid client behavior.

Brief: clarify how enumerating every unmatched league relates to the default list bound. Plan: no further change suggested. Prompt: no change suggested.
