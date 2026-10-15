#!/usr/bin/env bash
# notices-check.sh — every crate a shipped target links is named in the
# third-party notices (`about.toml` says why the notices are generated
# over every target, and why this check exists). By name: a crate two
# targets declare under one name may be listed at the other's version.
#
#   tools/notices-check.sh <THIRD-PARTY-NOTICES.md> [target ...]
#
# The targets default to the five the release ships (`release.yml`).
set -euo pipefail
[ $# -ge 1 ] || { echo "usage: $0 <notices.md> [target ...]" >&2; exit 2; }
notices=$1
shift
targets=("$@")
[ ${#targets[@]} -gt 0 ] || targets=(
    aarch64-apple-darwin x86_64-apple-darwin
    x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
    x86_64-pc-windows-msvc
)
missing=0
for t in "${targets[@]}"; do
    crates=$(cargo tree --locked --workspace --all-features --target "$t" -e normal \
        --prefix none --format '{p}' | awk '{print $1}' | sort -u)
    for c in $crates; do
        grep -q "^- $c " "$notices" || { echo "notices-check: $t links $c, which the notices do not name" >&2; missing=1; }
    done
done
[ $missing -eq 0 ] || exit 1
echo "notices-check: ok — every crate of ${#targets[@]} targets is named"
