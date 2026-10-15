#!/usr/bin/env bash
# release-smoke.sh — C82's acceptance check on one release archive
# (`PLATFORMS-SLICE.md`, step 4), run by `.github/workflows/release.yml`
# on every archive it makes and runnable by hand on any of them:
#
#   1. Extracted into an empty directory, the archive holds `acq`, `acqd`
#      and `acq-mcp` (`.exe` on Windows), both licences and the notices.
#   2. `acq version --json` names the `acqd` beside it — the extracted
#      one, by file, not by spelling.
#   3. On macOS and Linux, `acq` symlinked from another directory names
#      the same `acqd` (the locator canonicalises first). Not on Windows:
#      a standard user's symlink is an untrusted mount point there
#      (`PLATFORMS-SLICE.md`, observations).
#   4. A mock login, the daemon's status, and a stop, through the
#      extracted binaries: the login completes, the daemon that answers
#      is the extracted `acqd` and is compatible, and the stop stops it.
#
# Mock only (C88): ACQ_PROVIDER=mock, ACQ_NO_KEYRING=1, and a scratch
# store and log directory — never the real provider, never a keychain.
# Needs bash, curl and jq (all on GitHub's runners; Git Bash on Windows).
#
#   tools/release-smoke.sh <archive.tar.gz | archive.zip>
set -euo pipefail

[ $# -eq 1 ] || { echo "usage: $0 <archive>" >&2; exit 2; }
archive=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
[ -f "$archive" ] || { echo "release-smoke: no archive at $1" >&2; exit 2; }

windows=
case "${OSTYPE:-}" in msys* | cygwin*) windows=1 ;; esac
exe=${windows:+.exe}
# A path as acq prints it, in this shell's spelling.
native() { if [ -n "$windows" ]; then cygpath -u "$1"; else printf '%s\n' "$1"; fi; }
fail() { echo "release-smoke: FAIL — $*" >&2; exit 1; }

work=$(mktemp -d)
cleanup() {
    "${dir:-/nonexistent}/acq$exe" daemon stop >/dev/null 2>&1 || true
    rm -rf "$work"
}
trap cleanup EXIT

unset ACQ_GGG ACQ_TRIPWIRE ACQ_MAX_SENDS ACQ_IDLE_SHUTDOWN ACQ_ACCOUNT ACQ_NO_SPAWN
export ACQ_PROVIDER=mock ACQ_NO_KEYRING=1
export ACQ_STORE_DIR=$work/store ACQ_LOG_DIR=$work/logs

# 1. The archive's contents, under one top-level directory.
mkdir "$work/x"
case "$archive" in
    *.zip)
        if [ -n "$windows" ]; then
            powershell -NoProfile -Command \
                "Expand-Archive -LiteralPath '$(cygpath -w "$archive")' -DestinationPath '$(cygpath -w "$work/x")'"
        else
            unzip -q "$archive" -d "$work/x"
        fi
        ;;
    *.tar.gz) tar -xzf "$archive" -C "$work/x" ;;
    *) fail "not a .zip or .tar.gz: $archive" ;;
esac
tops=("$work/x"/*)
[ ${#tops[@]} -eq 1 ] && [ -d "${tops[0]}" ] || fail "the archive does not hold one top-level directory"
dir=${tops[0]}
for f in "acq$exe" "acqd$exe" "acq-mcp$exe"; do
    [ -x "$dir/$f" ] || fail "no executable $f in $(basename "$dir")"
done
for f in LICENSE-MIT LICENSE-APACHE THIRD-PARTY-NOTICES.md; do
    [ -s "$dir/$f" ] || fail "no $f in $(basename "$dir")"
done
echo "release-smoke: contents ok ($(basename "$dir"))"

# 2. The sibling, as the extracted acq names it.
version=$("$dir/acq$exe" version --json) || fail "acq version: $version"
named=$(jq -r '.acqd.path // empty' <<<"$version")
[ -n "$named" ] || fail "acq version names no acqd: $version"
[ "$(native "$named")" -ef "$dir/acqd$exe" ] || fail "acq names $named, not the acqd beside it"
echo "release-smoke: sibling ok ($(jq -r .version <<<"$version"), contract $(jq -r .contract <<<"$version"))"

# 3. A symlinked acq resolves the same acqd.
if [ -z "$windows" ]; then
    mkdir "$work/bin"
    ln -s "$dir/acq" "$work/bin/acq"
    named=$("$work/bin/acq" version --json | jq -r '.acqd.path // empty')
    [ -n "$named" ] && [ "$named" -ef "$dir/acqd" ] || fail "a symlinked acq names ${named:-no acqd}, not $dir/acqd"
    echo "release-smoke: symlinked acq ok"
fi

# 4. Mock login → status → stop. The login prints its URL and waits for
# the mock's approval, which redirects to the daemon's callback (-L).
"$dir/acq$exe" auth --no-browser --json >"$work/auth.out" 2>"$work/auth.err" &
auth=$!
url=
for _ in $(seq 100); do
    url=$(head -n 1 "$work/auth.out" 2>/dev/null | jq -r '.authorize_url // empty' 2>/dev/null || true)
    [ -n "$url" ] && break
    kill -0 "$auth" 2>/dev/null || break
    sleep 0.1
done
[ -n "$url" ] || fail "acq auth printed no authorize_url: $(cat "$work/auth.out" "$work/auth.err")"
approve=$(sed 's#/authorize?#/approve?#' <<<"$url")
curl -sfL --max-time 20 "$approve" >/dev/null || fail "the mock's approval did not complete"
wait "$auth" || fail "acq auth exited non-zero: $(cat "$work/auth.out" "$work/auth.err")"
status=$("$dir/acq$exe" daemon status --json)
jq -e '.running == true and .compatible == true and .provider == "mock"' >/dev/null <<<"$status" \
    || fail "after the login the daemon is not a running, compatible mock: $status"
ran=$(jq -r '.artifact.path // empty' <<<"$status")
[ -n "$ran" ] && [ "$(native "$ran")" -ef "$dir/acqd$exe" ] || fail "the daemon runs ${ran:-nothing}, not the extracted acqd"
stopped=$("$dir/acq$exe" daemon stop --json)
jq -e '.stopped == true' >/dev/null <<<"$stopped" || fail "the stop did not stop it: $stopped"
echo "release-smoke: mock login, status and stop ok"
echo "release-smoke: ok — $(basename "$archive")"
