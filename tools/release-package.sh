#!/usr/bin/env bash
# release-package.sh — one target's release archive (`PLATFORMS-SLICE.md` at `b6197a82`,
# step 4), from a `cargo build --release --target <target>` already run:
# `acq`, `acqd` and `acq-mcp` side by side (C82: the daemon beside every
# frontend), both licences and the third-party notices, under one
# directory named for the version and the target. A `.tar.gz` on macOS
# and Linux, a `.zip` for Windows. Prints the archive's path.
#
#   tools/release-package.sh <target> <THIRD-PARTY-NOTICES.md> <out-dir>
#
# The binaries are taken as built (signed or not: `release.yml` signs the
# macOS ones in place before this runs); `tools/release-smoke.sh` checks
# the result.
set -euo pipefail
[ $# -eq 3 ] || { echo "usage: $0 <target> <notices.md> <out-dir>" >&2; exit 2; }
target=$1
notices=$2
out=$3
here=$(cd "$(dirname "$0")/.." && pwd)
version=$(cargo metadata --format-version 1 --no-deps --manifest-path "$here/Cargo.toml" \
    | jq -r '[.packages[] | select(.name == "acquisition-cli")][0].version')
[ -n "$version" ] && [ "$version" != null ] || { echo "release-package: no version" >&2; exit 2; }

case "$target" in *windows*) exe=.exe ;; *) exe= ;; esac
name=acquisition-$version-$target
built=$here/target/$target/release
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
mkdir "$stage/$name"
for b in acq acqd acq-mcp; do
    [ -f "$built/$b$exe" ] || { echo "release-package: no $built/$b$exe — build the target first" >&2; exit 2; }
    cp "$built/$b$exe" "$stage/$name/"
done
cp "$here/LICENSE-MIT" "$here/LICENSE-APACHE" "$stage/$name/"
cp "$notices" "$stage/$name/THIRD-PARTY-NOTICES.md"

mkdir -p "$out"
out=$(cd "$out" && pwd)
if [ -n "$exe" ]; then
    archive=$out/$name.zip
    rm -f "$archive"
    if command -v cygpath >/dev/null; then
        powershell -NoProfile -Command \
            "Compress-Archive -Path '$(cygpath -w "$stage/$name")' -DestinationPath '$(cygpath -w "$archive")'"
    else
        (cd "$stage" && zip -qr "$archive" "$name")
    fi
else
    archive=$out/$name.tar.gz
    # COPYFILE_DISABLE: macOS's tar otherwise writes an AppleDouble `._`
    # entry beside each file, which macOS hides and every other unpacker
    # extracts as a file; the smoke refuses them. Ignored elsewhere.
    COPYFILE_DISABLE=1 tar -czf "$archive" -C "$stage" "$name"
fi
echo "$archive"
