# Review of 97-c143-reach fix round and Part B at d291b1b0^..39a6d7b7

Codex (`codex exec --sandbox read-only`, `roadmap/briefs/REVIEW-PROMPT.md`), its answer verbatim.

- `d291b1b0^..39a6d7b7` — both prior review findings are addressed; Part B’s done-when is not fully met because its rule still skips generated answer prose.

- **[P2] [crates/acquisition-mcp/tests/harness/mod.rs:384](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/harness/mod.rs:384) — shape 1.** Ordinary answers check only `why`, `hint`, and selected offer containers, leaving receipt `note`, `no_undo`, `clear.says`, and `counts_unavailable.reads[].says` unchecked. For example, the rule accepts a successful `price_set` document containing `{"note":"acq price show item/i-map","clear":{"says":"acq price clear item/i-map"}}`. These fields hold generated prose, unlike persisted actor metadata or an upstream body. Smallest fix: classify these prose fields consistently and add breaker cases beside the existing rule test, preserving payload exclusions.

- **[P3] [crates/acquisition-search/src/describe.rs:736](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/describe.rs:736).** Flag compatibility is normalized only for built entries and only at the beginning of the whole name, so previously accepted client input `{"names":["show --against"]}` now returns `unknown_name`, and `{"names":["--rebind"]}` loses its `not_built` refusal. Smallest fix: normalize flag tokens once for both built-entry and unbuilt-name lookup, with cases covering these existing spellings.

- **Not checked:** execution evidence, including gate results, the pre-change red run, command execution, first-turn size, and fixture regeneration, is **not verified**; execution was prohibited.
- **Nitpicks:** the second finding concerns uncommon compatibility inputs. The first leaves an explicit acceptance requirement insufficiently pinned.
- **Brief:** no obstacle or change suggested.
- **Plan:** no separate worker plan was supplied or used.
- **Prompt:** no obstacle or change suggested.
