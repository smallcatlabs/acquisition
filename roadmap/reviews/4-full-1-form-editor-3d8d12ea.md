f5352957^..3d8d12ea — the brief’s done-when does not fully hold; the filled editor still has a recovery gap and a partition edge case.

- **P2 — [page.rs:839](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/page.rs:839), shape 4:** The filled form offers counting while carrying row-only settings invisibly, leaving the resulting refusal impossible to correct through its controls. **Client input:** open `search?realm=pc&text=class:rings&view.sort=ilvl`, enter `rarity` under “Or count by”, and submit: the hidden sort causes a refusal and remains hidden afterward. **Smallest fix:** provide a visible way to clear row-only settings while retaining the query; preserve the shared conflict refusal.

- **P3 — [page.rs:1500](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/page.rs:1500), shape 7:** Counted answers never receive the newly implemented “Saved without its count” notice because saving independently constructs a default `BoxView` for every non-row answer. **Client input:** `search?realm=pc&text=&view.count=rarity`. **Smallest fix:** pass the editor’s existing view carrier to `save_forms`, including its count.

- **P3 — [fill.rs:155](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/fill.rs:155), shape 1:** A string stat ID that cannot be held can cause pruning to remove a group whose type was already marked held, violating the partition. **Client-supplied trade JSON:** `{"query":{"stats":[{"type":"and","filters":[{"id":" padded "}]}]}}`. The ID is listed as omitted, but submission also drops the unlisted group type; subsequent groups can shift indices. **Smallest fix:** use the whole-stats fallback when an ID cannot survive rendering and folding, and add this case to the partition test.

- **Not checked:** gate results, test counts and browser execution are **not verified**, as requested; review was read-only. Referenced remote issue discussions were not supplied or fetched.
- **Nitpicks:** the padded-ID finding concerns unlikely malformed input; the count-save finding is a wording defect on an ordinary path.
- **Brief:** explicitly specify how users clear hidden row settings when switching to counts.
- **Plan and prompt:** no other impediments.
