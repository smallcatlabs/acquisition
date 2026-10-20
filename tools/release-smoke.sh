#!/usr/bin/env bash
# release-smoke.sh — C82's acceptance check on one release archive
# (`PLATFORMS-SLICE.md` at `b6197a82`, step 4), run by `.github/workflows/release.yml`
# on every archive it makes and runnable by hand on any of them:
#
#   1. Extracted into an empty directory, the archive holds `acq`, `acqd`
#      and `acq-mcp` (`.exe` on Windows), both licences and the notices,
#      and a `.tar.gz` lists no AppleDouble `._` entry.
#   2. `acq version --json` names the `acqd` beside it — the extracted
#      one, by file, not by spelling.
#   3. On macOS and Linux, `acq` symlinked from another directory names
#      the same `acqd` (the locator canonicalises first). Not on Windows:
#      a standard user's symlink is an untrusted mount point there
#      (`PLATFORMS-SLICE.md` at `b6197a82`, "What the runs taught").
#   4. A mock login, the daemon's status, and a stop, through the
#      extracted binaries: the login completes, the daemon that answers
#      is the extracted `acqd` and is compatible, and the stop stops it.
#   5. Before the stop, the extracted `acq-mcp` run over stdio: it
#      answers `initialize` and a `daemon_status` call, reports that same
#      daemon running, compatible and mock, and exits when its stdin
#      closes.
#
# Mock only (C88): ACQ_PROVIDER=mock, ACQ_NO_KEYRING=1, and a scratch
# store and log directory — never the real provider, never a keychain.
# Needs bash, curl and jq, and python3 for a `.tar.gz` (all on GitHub's
# runners; Git Bash on Windows).
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
    *.tar.gz)
        tar -xzf "$archive" -C "$work/x"
        # No AppleDouble `._` entry: macOS's `tar` merges them into the
        # file they describe and never shows them, but a `tar` elsewhere
        # and Python's `tarfile` unpack them as files of their own. A
        # macOS `tar` writes them unless COPYFILE_DISABLE=1 is set.
        appledouble=$(python3 -I -c '
import sys, tarfile
with tarfile.open(sys.argv[1]) as t:
    print(" ".join(m.name for m in t if m.name.rsplit("/", 1)[-1].startswith("._")))
' "$archive") || fail "python3 could not list $archive"
        [ -z "$appledouble" ] || fail "the archive holds AppleDouble entries (a macOS tar without COPYFILE_DISABLE=1): $appledouble"
        ;;
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
# 5. The extracted acq-mcp, with the daemon up: `initialize`, the
# `notifications/initialized` notification and a `tools/call` of
# `daemon_status`, one JSON-RPC message per line on its stdin. Its stdin
# stays open until the call's answer is out (bounded), so the server is
# never asked to answer after its input closed; then it must exit on the
# close, and a server still running when the bound runs out is killed
# and fails the smoke. The daemon it reports must be the running,
# compatible mock daemon the login started — the extracted acqd.
mcp_out=$work/mcp.out
answered() { grep -q '"id":2' "$mcp_out" 2>/dev/null; }
{
    printf '%s\n' \
        '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"release-smoke","version":"0"}}}' \
        '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
        '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"daemon_status","arguments":{}}}'
    for _ in $(seq 200); do answered && break; sleep 0.1; done
} | "$dir/acq-mcp$exe" >"$mcp_out" 2>"$work/mcp.err" &
mcp=$!
for _ in $(seq 300); do kill -0 "$mcp" 2>/dev/null || break; sleep 0.1; done
if kill -0 "$mcp" 2>/dev/null; then
    kill "$mcp" 2>/dev/null || true
    fail "acq-mcp did not exit when its stdin closed: $(cat "$mcp_out" "$work/mcp.err")"
fi
wait "$mcp" || fail "acq-mcp exited non-zero: $(cat "$mcp_out" "$work/mcp.err")"
answered || fail "acq-mcp never answered the daemon_status call: $(cat "$mcp_out" "$work/mcp.err")"
reported=$(grep '"id":2' "$mcp_out" | jq -c '.result.structuredContent')
jq -e '.running == true and .compatible == true and .provider == "mock"' >/dev/null <<<"$reported" \
    || fail "acq-mcp does not report a running, compatible mock daemon: $(cat "$mcp_out")"
ran=$(jq -r '.artifact.path // empty' <<<"$reported")
[ -n "$ran" ] && [ "$(native "$ran")" -ef "$dir/acqd$exe" ] || fail "acq-mcp reports a daemon running ${ran:-nothing}, not the extracted acqd"
echo "release-smoke: acq-mcp ok (daemon_status over stdio)"
stopped=$("$dir/acq$exe" daemon stop --json)
jq -e '.stopped == true' >/dev/null <<<"$stopped" || fail "the stop did not stop it: $stopped"
echo "release-smoke: mock login, status and stop ok"
echo "release-smoke: ok — $(basename "$archive")"
