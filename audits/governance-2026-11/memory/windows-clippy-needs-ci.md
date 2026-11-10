---
name: windows-clippy-needs-ci
description: "The one check only CI runs is the doc stage (cargo doc -D warnings, left out of the local gate's full run); Windows clippy has been a local gate stage, winclippy, since 2026-10-29"
metadata:
  type: project
---

`cargo clippy --target x86_64-pc-windows-msvc` from the Mac fails in libsqlite3-sys's bundled C build (`stdlib.h` not found: no Windows headers). The target is installed, but it is no use for linting the workspace (2026-10-27, r1).

**Why:** r1's first push uncovered Windows-only dead code (helpers whose only callers are `#[cfg(unix)]` tests) that only the Windows CI clippy sees.

**How to apply:** when adding a helper or an import used only by `#[cfg(unix)]` tests, gate it `#[cfg(unix)]` too.

**Superseded 2026-10-29 (checked against `tools/gate.sh` on 2026-11-09):** the gate now has a `winclippy` stage that lints the Windows target on any host through `tools/windows-cc-stub.sh` (a stub C compiler, so the bundled SQLite build no longer stops it). The full local gate therefore sees Windows-only lints before a push; it runs no Windows test and links nothing, so the [[vm-test-lab]] guest and the Windows CI gate still hold behaviour.

**The doc stage too** (2026-11-01, the describe step's close): `tools/gate.sh`'s full run leaves out `cargo doc` with warnings denied; CI runs it, and a doc comment linking a private item (`[`index`]` in `describe.rs`) went red there after a green local gate. Before pushing a range that touches Rust doc comments, run `tools/gate.sh --only doc` (about 40 s).
