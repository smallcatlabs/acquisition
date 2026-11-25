# Four defects the planning read found and nothing filed

Labels: area:ci trigger:ci

From `audits/whole-review-2026-11/README.md`, each to be its own issue when filed: the browser test `two_stat_groups_one_weighted_…` failing on the Windows gate after #312 closed; cargo-deny running only in CI, where advisories were half of Linux's failures on `main`; `rust-toolchain.toml` naming no target, so `winclippy` fails on a fresh clone; the registry entries C79, C110, C120, C155, C189 and C191 pinned by nothing.
