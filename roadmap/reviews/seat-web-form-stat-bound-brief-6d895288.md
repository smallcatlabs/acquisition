`roadmap/briefs/seat-web-form-stat-bound.md` at `6d895288`: a builder needs the general plural criterion clarified before starting as drawn.

- **Goal / Decisions 2–3 / Done when — changes what gets built.** The acceptance checks allow the abyssal fix while leaving other forms covered by the universal goal unsummed. In `tools/trade-forms-table.py`, `word()` handles `is/are`, `ies` and trailing `s`, but fails `Prefix/Prefixes` and `Suffix/Suffixes`; those pairs already appear as `other` in `trade-game-forms-v1.toml`. Removing the literal-`#` guard can satisfy the listed abyssal checks while leaving these behind, and reading only moved forms never discovers them. The first guess is whether the existing normalizer or grammatical number defines membership. Smallest change: require an audit of retained `other` forms against the stated criterion and boundary fixtures covering these existing plural pairs.

- **Find first, claim-number command — changes what gets built.** Taking the last heading finds document order rather than the highest claim number. The supplied `grep … | tail -1` returns `T15`, while the file already contains `T41`; incrementing its answer would reuse `T16`. Smallest change: extract claim numbers, sort numerically, and take the maximum.

- **Find first, form-kind readers command — wording.** The command does not locate the digit-to-line implementation it promises and includes unrelated sums. Its results contain `counts.rs` aggregation calls, but omit `Shown::of` at `translate.rs:866` and its `template::fixed` call. Smallest change: search explicitly for `Shown`, `template::fixed` and form-kind reads, restricting the paths to the trade modules and their tests.

- **Inputs, sitting and export bullets — wording.** The brief carries measurements without their measurement commands and carries historical command output, contrary to C174. The hash pipeline measures neither “7 files” nor “136429 bytes”, and the export bullet records that `--check` “printed `ok`”. Smallest change: supply file-count and byte-count commands, and retain the export pin and check command without its historical result.

Not checked: builds, tests, generators or the gate, as instructed; GitHub issues/comments, CI conclusions, recorder contents and export contents, because those are outside the committed tree.

Form/prompt: the owner’s choices and sitting answers depend partly on external issue comments, so their exact wording could not be verified from the brief’s commit alone.
