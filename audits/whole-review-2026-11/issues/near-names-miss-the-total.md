# A near name for `life` never offers `pseudo.total_life`

Labels: area:search trigger:step

`near()` (`crates/acquisition-search/src/bind.rs:530-556`) keeps the first three suggestions after an alphabetical sort within a score, and a name that holds the word whole ranks below a prefix match. So `life>=90` offers `level`, `links`, `pseudo.life_regen`; `pseudo.life>=90` offers three names and not `pseudo.total_life`; `res` offers `resolve`.

Seen on the owner's store by the orchestrator and two investigators. Done when `life` and `res` each offer the total a player means first.
