5d3ed1c9^..35f9195f — the brief’s source-level done-when holds; full completion depends on execution evidence.

- **[P3] [crates/acquisition-store/src/index.rs:592](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/index.rs:592), shape 1:** Inserting the new selector test below the existing documentation attaches the crash-test description to the wrong test. Reading `a_selector_matches_a_held_keyring_name` now claims it stages a crash after one write, although it only checks selector matching. Move the original documentation block above `c144_a_stop_after_the_renames_write_leaves_the_old_name_in_reach`. This concerns documentation, not client, persisted, or upstream input.
- **Not verified:** test results, gate status, and byte-identical regeneration; execution was prohibited. GitHub issue/comment state was not checked because it is outside the committed evidence.
- **Nitpicks:** The sole finding is a documentation nitpick with no runtime effect. No behavioural or invariant-bypass defect found.
- **Brief:** Correct #74’s initial-state description: the failed rename cannot persist B among held names; the subsequent failed logout does, as the implementation commit explains.
- **Plan or prompt:** No changes suggested.
