#!/usr/bin/env bash
# arch-check.sh — the structure half of the quality gate: what links what,
# and who reads a group's meaning. Both checks moved here whole from
# `tools/docs-check.sh` (step 1e of the roadmap) and keep their numbers,
# which code and records cite:
#
#   5. Dependency direction: the layer rules as edges the crates cannot
#      cross, read from `cargo metadata` — the split's edge table
#      (the packet, note 18 §2.1 at decda84e; C1 as amended) for every crate that
#      exists: the protocol crate's purity (serde only), the store's
#      blindness to everything above it, the planner's and the client's
#      independence from the daemon, the daemon's from the client, the
#      planner and any frontend, and the daemon named by no package but
#      itself — each forbidden edge refused at any depth.
#
#   7. One reader of a group's meaning: outside the search crate's modules
#      that build, print and validate the query tree, and the one that
#      computes what a line's group means, no source file names a
#      group's tree (the build plan, step 4b; `group.rs`).
#
#   8. The test layout (C122): under a crate's `tests/`, at most one file
#      calls no `session(` — the crate's one binary of modules — and every
#      other is a binary that sets the process environment, none of them
#      a module of the plain one; and the binaries Cargo builds are those
#      files, the manifest's `[[test]]` list naming each where it turns
#      discovery off, and discovery off wherever a file lists modules.
#
#   9. A test spawns a workspace binary through the harness (C122): under
#      a crate's `tests/`, outside `harness/`, no file names
#      `CARGO_BIN_EXE_` and every `Command::new(` names an allowed system
#      tool (`kill`, `ps`); the daemon crate's tests excepted (#87).
#
# Exit 1 on any failure; the report names each offender.
set -euo pipefail
cd "$(dirname "$0")/.."

fail=0

# ---- 5. dependency direction ------------------------------------------
# The four-layer rule (C34), the planner's home (C39) and the daemon as
# its own artifact (C1, C13, C82) are enforced by what links what, not by
# discipline: the daemon crate (`acquisition-daemon`, binary `acqd`)
# links the protocol crate and the store — never the client crate, the
# planner or a frontend — and never names the intent API (it writes facts
# through the store and reads nothing else); no package but the daemon
# names the daemon, so a frontend cannot be in-process with it (C13) and
# the client crate cannot embed it; the client crate links the protocol
# crate, the store (the world, C83) and tokio, never the daemon or the
# planner, so a client or locator edit recompiles no daemon; the store
# crate links none of them and no HTTP client, so a store read cannot
# initiate traffic (C41); the protocol crate — the daemon's contract as a
# frontend sees it — links serde and serde_json, nothing else (sha2 at
# build time only), because a dependency there is every frontend's and
# the daemon's both, and the revision it computes must move only when
# the contract does (§2.1); the planner links the protocol crate and the
# store, never the daemon or the client (§2.1: every path it took from
# core was the wire's or the vocabulary's, so a planner that links the
# daemon again has grown a third door); the search crate links the store,
# and the planner once a query reads the effective price, never the
# daemon, the client, a frontend, an HTTP client or an async runtime, and
# neither the daemon nor the planner links it — a plan takes item ids,
# never a query, and a search edit recompiles no daemon (C89). The verbs
# crate (`acquisition-verbs`, C1 as amended, C46) holds what needs the
# daemon and the store together and links the client, the protocol
# crate, the store, the planner and the search; a frontend may link it,
# and nothing below it may. It needs no list of its own: each lower
# crate's list already refuses, at any depth, a crate the verbs crate
# links, and the daemon's `only_self` and its client entry cover the
# daemon in both directions.
#
# One graph, from Cargo: the `resolve` section of `cargo metadata
# --all-features` — every package Cargo resolves for this workspace with
# every member feature on and no platform filter, so every optional edge
# a member can activate is present, and an optional edge nothing can
# activate is absent because no build can link it either. The edges are
# the union over targets, on purpose: a target predicate is not followed
# along a path, so `plan → helper` under `cfg(unix)` with `helper → daemon`
# under `cfg(windows)` is refused although no single target links the
# two — the boundary is what the manifests declare, not what one
# platform happens to build (review 2026-09-10, accepted as the
# conservative reading). Vertices are package ids, never names: the same
# name at two versions is two vertices, and a name is used only to state
# a rule and to print. Edges carry their kinds; the closure walks normal
# and build edges at every depth and dev edges from the root only, since
# Cargo links no one else's dev-dependencies. Membership is
# `.workspace_members`, never a guess from the package's source.
#
# Never the manifest's text (review 2026-09-10: a lexical parser missed
# three valid forms); never `cargo tree` (the host's active closure under
# default features missed a `cfg(windows)` edge and an optional one);
# never the declared table of `packages[].dependencies` (an inactive
# optional dependency is declared but has no package entry, so its own
# edges are invisible). Six review rounds on this check found the same
# shape each time: a reader whose empty or partial answer satisfied
# every rule. So the graph is read twice, by two jq programs whose exit
# status is checked directly and whose answers must agree row by row.
# The first proves the graph whole — every member is a node, every edge
# lands on a node, every node has a package — and names, by a set
# fixpoint of its own, every fact the table must hold, by package id,
# sorted. The second walks the graph and emits the table in that order.
# Bash compares the two by identity and by name with builtins alone —
# no awk, grep or comm in a process substitution can answer for it — so
# a missing row, a duplicate in its place, a fabricated row, an extra
# one, or a name that does not belong to its id all refuse; then every
# package the rules are about must have its own row with every direct
# edge inside its closure. The rules read the names so authenticated;
# the path beside a row is printed on a hit and never read. What remains
# is a reader that forges a whole table, its identities and its names to
# match: a consistent lie, accepted. `tools/arch-check-breakers.sh`
# stages every case above and requires the refusal that names it; run
# it whenever an edge is added here.
meta=$(mktemp)
if ! cargo metadata --format-version 1 --all-features --offline >"$meta" 2>/dev/null \
   && ! cargo metadata --format-version 1 --all-features >"$meta"; then
  echo 'EDGE    cargo metadata failed — the dependency graph could not be read'
  rm -f "$meta"; exit 1
fi
# The graph must be whole before it is read, and the first reader states
# what the table must contain: after `whole`, one line per fact, by
# package id, sorted, each carrying the names the rules will read —
# `direct <member id> <kind> <dep id> <member> <name>` and `closure
# <member id> <dep id> <member> <name>` — from a set fixpoint and an
# id-to-name map of its own.
if ! partial=$(jq -r '
    ([.resolve.nodes[]?.id] | unique) as $nodes
    | ([.packages[].id] | unique) as $pkgs
    | ([.resolve.nodes[]? | {key: .id, value: .deps}] | from_entries) as $deps
    | ([.packages[] | {key: .id, value: .name}] | from_entries) as $name
    | def reach($m): reduce range(0; 10000) as $_ (
        { seen: [$m], front: ([$deps[$m][] | .pkg] | unique) };
        if .front == [] then . else
          (.seen + .front) as $seen
          | .front = (([ .front[] | $deps[.][] | select(any(.dep_kinds[]; .kind != "dev")) | .pkg ] | unique) - $seen)
          | .seen = $seen
        end) | .seen;
    if (.workspace_members | length) == 0 then "no workspace member"
      elif ($nodes | length) == 0 then "no resolve graph"
      elif any(.workspace_members[]; . as $m | ($nodes | index($m)) == null) then "a workspace member is not a node"
      elif any(.resolve.nodes[].deps[].pkg; . as $d | ($nodes | index($d)) == null) then "an edge lands on no node"
      elif any($nodes[]; . as $n | ($pkgs | index($n)) == null) then "a node has no package entry"
      else "whole",
        ([.workspace_members[] as $m | $deps[$m][] | .pkg as $d | .dep_kinds[] | [$m, (.kind // "normal"), $d]] | sort | .[] | "direct\t\(.[0])\t\(.[1])\t\(.[2])\t\($name[.[0]])\t\($name[.[2]])"),
        ([.workspace_members[] as $m | reach($m)[] | [$m, .]] | sort | .[] | "closure\t\(.[0])\t\(.[1])\t\($name[.[0]])\t\($name[.[1]])")
      end' "$meta"); then
  echo 'EDGE    jq failed or is not installed — the dependency graph could not be read'
  rm -f "$meta"; exit 1
fi
if [[ $partial != "whole"* ]]; then
  printf 'EDGE    the metadata is partial (%s) — the dependency graph could not be read\n' "$partial"
  rm -f "$meta"; exit 1
fi
# The table, one tab-separated line per fact, in the first reader's order
# (direct rows by member id, kind, dep id; closure rows by member id,
# dep id); ids and names are compared, the path is diagnostic only:
#   direct  <member> <kind> <name> <member id> <dep id>
#   closure <member> <name> <member id> <dep id> <path by names>
if ! table=$(jq -r '
    ([.packages[] | {key: .id, value: .name}] | from_entries) as $name
    | ([.resolve.nodes[] | {key: .id, value: .deps}] | from_entries) as $deps
    | ([.workspace_members[] as $m | $deps[$m][] | .pkg as $d | .dep_kinds[] | [$m, (.kind // "normal"), $d]]
       | sort | .[] | "direct\t\($name[.[0]])\t\(.[1])\t\($name[.[2]])\t\(.[0])\t\(.[2])"),
      ([.workspace_members[] as $m
        | { seen: {($m): $name[$m]}, queue: [ $deps[$m][] | .pkg ], from: {} }
        | .from = ([ $deps[$m][] | {key: .pkg, value: $m} ] | from_entries)
        | until(.queue == [];
            .queue[0] as $x | .queue |= .[1:]
            | if .seen[$x] then . else
                .seen[$x] = (.seen[.from[$x]] + " → " + $name[$x])
                | reduce ($deps[$x][] | select(any(.dep_kinds[]; .kind != "dev")) | .pkg) as $y
                    (.; if .seen[$y] or .from[$y] then . else .from[$y] = $x | .queue += [$y] end)
              end)
        | .seen | to_entries[] | [$m, .key, .value]]
       | sort | .[] | "closure\t\($name[.[0]])\t\($name[.[1]])\t\(.[0])\t\(.[1])\t\(.[2])")' "$meta"); then
  echo 'EDGE    jq failed reading the dependency graph'
  rm -f "$meta"; exit 1
fi
rm -f "$meta"
# The two answers must agree row by row, by identity and by name: the
# table's rows, projected to (member id, kind, dep id, member, name) and
# (member id, dep id, member, name), must be exactly the first reader's
# lines in order — a missing row, a duplicate in its place, a fabricated
# row, an extra one, or a row whose name does not belong to its id all
# refuse here, before any rule is consulted (review 2026-09-10: matching
# counts had passed a duplicate standing in for the dropped row; matching
# ids had passed a renamed dependency the rules then could not find).
# The path column is not compared: it is printed on a hit, never read by
# a rule. Builtins only.
expect=()
IFS=$'\n' read -d '' -ra expect <<<"${partial#whole}" || true
actual=()
while IFS=$'\t' read -r kind a b c d e _; do
  case $kind in
    direct)  actual+=("direct	$d	$b	$e	$a	$c") ;;
    closure) actual+=("closure	$c	$d	$a	$b") ;;
    '')      ;;
    *)       printf 'EDGE    the dependency table has a row of unknown kind (%s) — the table is not the graph\n' "$kind"; exit 1 ;;
  esac
done <<<"$table"
if ((${#expect[@]} == 0)); then
  echo 'EDGE    the first reader named no fact — the graph was not read'; exit 1
fi
if ((${#actual[@]} != ${#expect[@]})); then
  printf 'EDGE    the dependency table has %d rows, the graph has %d — the table is partial\n' "${#actual[@]}" "${#expect[@]}"; exit 1
fi
for ((i = 0; i < ${#expect[@]}; i++)); do
  if [[ ${actual[i]} != "${expect[i]}" ]]; then
    printf 'EDGE    the dependency table differs from the graph at row %d: the graph has "%s", the table "%s" — the table is not the graph\n' \
      "$((i + 1))" "${expect[i]//	/ }" "${actual[i]//	/ }"; exit 1
  fi
done
table=$'\n'"$table"$'\n'
has_row() { [[ $table == *$'\n'"$1"* ]]; }   # has_row <line prefix>: builtins only
named() {  # named <package>: its own rows must be there — itself in its closure, every direct edge in it
  local pkg=$1 kind name
  if ! has_row "closure	$pkg	$pkg	"; then
    printf 'EDGE    the dependency table has no row for %s — the graph was not read\n' "$pkg"
    exit 1
  fi
  while IFS=$'\t' read -r _ _ kind name _; do
    if ! has_row "closure	$pkg	$name	"; then
      printf 'EDGE    %s links %s directly but its closure does not contain it — the graph was not read\n' "$pkg" "$name"
      exit 1
    fi
  done < <(direct_rows "$pkg")
}
direct_rows() {  # direct_rows <package>: the direct rows, builtins only
  local line
  while IFS= read -r line; do
    [[ $line == "direct	$1	"* ]] && printf '%s\n' "$line"
  done <<<"$table"
}
path_to() {  # path_to <package> <name>: how the package reaches the name
  local line k m n mi di path
  while IFS=$'\t' read -r k m n mi di path; do
    [[ $k == closure && $m == "$1" && $n == "$2" ]] && { printf '%s\n' "$path"; return; }
  done <<<"$table"
}
edge_bad=0
forbid() {  # forbid <package> <why> <name>...: refuse any of these names, in any section, at any depth
  local pkg=$1 why=$2; shift 2
  local name direct='' deep='' d
  named "$pkg"
  for name in "$@"; do
    while IFS=$'\t' read -r _ _ _ d _; do [[ $d == "$name" ]] && direct+="$name " && break; done < <(direct_rows "$pkg")
    [[ " $direct" == *" $name "* ]] && continue
    has_row "closure	$pkg	$name	" && deep+="$name "
  done
  if [[ -n $direct ]]; then
    printf 'EDGE    %-22s links %s— %s\n' "$pkg" "$direct" "$why"
    fail=1; edge_bad=1
  fi
  if [[ -n $deep ]]; then
    printf 'EDGE    %-22s links %stransitively — %s\n' "$pkg" "$deep" "$why"
    for name in $deep; do printf '          %s\n' "$(path_to "$pkg" "$name")"; done
    fail=1; edge_bad=1
  fi
}
allow() {  # allow <package> <kind> <name>...: refuse any other direct dependency of that kind
  local pkg=$1 kind=$2; shift 2
  local k name extra='' ok a
  named "$pkg"
  while IFS=$'\t' read -r _ _ k name _; do
    [[ $k == "$kind" ]] || continue
    ok=0; for a in "$@"; do [[ $a == "$name" ]] && ok=1; done
    ((ok)) || extra+="$name "
  done < <(direct_rows "$pkg")
  if [[ -n $extra ]]; then
    printf 'EDGE    %-22s [%s] links %s— the protocol crate is serde-only (§2.1)\n' "$pkg" "$kind" "$extra"
    fail=1; edge_bad=1
  fi
}
only_self() {  # only_self <package> <why>: no other member reaches it, at any depth
  local pkg=$1 why=$2 k m n mi di path bad='' seen=' '
  named "$pkg"
  while IFS=$'\t' read -r k m n mi di path; do
    [[ $k == closure && $n == "$pkg" && $m != "$pkg" && $seen != *" $m "* ]] && { bad+="$m "; seen+="$m "; }
  done <<<"$table"
  if [[ -n $bad ]]; then
    printf 'EDGE    %-22s is linked by %s— %s\n' "$pkg" "$bad" "$why"
    for m in $bad; do printf '          %s\n' "$(path_to "$m" "$pkg")"; done
    fail=1; edge_bad=1
  fi
}
# The frontends are named here; a new one (the GUI, a queue TUI) is added
# to the daemon's list when its package exists — the reverse direction,
# that nothing links the daemon, needs no list.
forbid acquisition-daemon 'the daemon links protocol and store, never the client crate, the planner, the search or a frontend (C1, C39, C89)' \
  acquisition-client acquisition-plan acquisition-search acquisition-cli acquisition-mcp
only_self acquisition-daemon 'no package but acquisition-daemon names the daemon — a frontend is never in-process with it, and the client never embeds it (C1, C13, C82)'
forbid acquisition-client 'the client links protocol, store and tokio, never the daemon or the planner (C1, §2.1)' \
  acquisition-daemon acquisition-plan
forbid acquisition-plan  'the planner links protocol and store, never the daemon, the client or the search (C39, C89, §2.1)' \
  acquisition-daemon acquisition-client acquisition-search
forbid acquisition-search 'the search links the store, and the planner once a query reads the effective price — never the daemon, the client, a frontend, an HTTP client or an async runtime (C89)' \
  acquisition-daemon acquisition-client acquisition-cli acquisition-mcp reqwest tokio
forbid acquisition-store 'the store links no daemon, no client, no protocol, no planner, no search and no HTTP client (C41, C89; the split, §2.1)' \
  acquisition-daemon acquisition-client acquisition-protocol acquisition-plan acquisition-search reqwest tokio
allow acquisition-protocol normal serde serde_json
allow acquisition-protocol dev    serde serde_json
allow acquisition-protocol build  sha2
if grep -rqE 'Annotations|annotations_path' crates/acquisition-daemon/src crates/acquisition-protocol/src crates/acquisition-client/src; then
  echo 'EDGE    crates/acquisition-{daemon,protocol,client}/src    names the intent API — the daemon, the wire and the client are permanently blind to intent (C34)'
  fail=1; edge_bad=1
fi
# The doors into semantics only the verbs crate opens (C46, every
# frontend an adapter; C1 as amended): an index loaded, a facts or intent
# file opened, a plan compiled or gated, an apply's parameters built, a
# quote asked, the ledger read, the use door — named by no frontend's
# source, so a frontend that answers for itself is refused here, not
# found by a review. `Client::observe` is not a door: `auth status`,
# `daemon stop` and `reset-tripwire` observe with no semantics to move.
# The grep reads every file under the two source trees, the web UI's
# module (`acquisition-cli/src/web`, C139) among them.
doors='Index::load|Store::open|Annotations::open_for|plan_refresh|check_spendable|apply_params_of|\.quote\(|Persisted::observe|Client::connect'
if grep -rqE "$doors" crates/acquisition-cli/src crates/acquisition-mcp/src; then
  echo 'EDGE    crates/acquisition-{cli,mcp}/src    opens a door only the verbs crate opens — an index, a store, a plan, a quote, the ledger or the use door is the shared layer'"'"'s (C46)'
  fail=1; edge_bad=1
fi
if ((edge_bad == 0)); then
  echo 'ok      dependencies  daemon ∌ client/planner/search/frontend, nothing ∌ daemon but itself, client ∌ daemon/planner, planner ∌ daemon/client/search, search ∌ daemon/client/frontend/HTTP/async, store ∌ daemon/client/protocol/search/HTTP, daemon/protocol/client ∌ intent API, frontends ∌ the layer'"'"'s doors, protocol = serde only (C1, C34, C39, C41, C46, C89, §2.1)'
fi

# ---- 7. a group's meaning has one reader ---------------------------------
# What `line( … )` means — what it selects, which sources it admits, its
# together bound, whether its selector asks a flag, its template tests — is
# computed once, when the query is bound, in acquisition-search's `group`
# module, and read from there. Four outside audits of the first surface
# found one fault five times: a meaning read again off the syntax by the
# function that needed it next, which parentheses then changed (invariant 7
# of the surface; `SEARCH-SLICE.md` at `b6197a82`, the findings of step 4b). P5: a rule that can
# be broken silently becomes structure. So no source file of any crate
# names a group's tree, authored (`Member`) or bound (`BMember`), but the
# modules that build, print and validate it, and that one — names, and not
# only matches on, so that an alias is no way round; the crate's re-export
# is the one line let through, for a caller that builds a tree.
group_modules='^crates/acquisition-search/src/(tree|parse|print|json|template|group)\.rs:'
group_readers=$(grep -rnE '(^|[^A-Za-z0-9_])B?Member([^A-Za-z0-9_]|$)' crates/*/src --include='*.rs' \
  | grep -vE "$group_modules" | grep -vE '^crates/acquisition-search/src/lib\.rs:[0-9]+:pub use tree::\{' || true)
if [[ -n $group_readers ]]; then
  echo "GROUP   a group's tree is named outside the modules that build, print and validate it and the one that computes its meaning (acquisition-search: tree, parse, print, json, template, group) — a fact of a group is added to group.rs and read from there:"
  printf '%s\n' "$group_readers" | cut -c1-160 | sed 's/^/          /'
  fail=1
elif [[ ! -f crates/acquisition-search/src/group.rs ]]; then
  echo "GROUP   crates/acquisition-search/src/group.rs is gone: the rule names a module that does not exist"
  fail=1
else
  printf 'ok      %-13s a group'"'"'s tree is named by tree, parse, print, json, template and group alone (step 4b)\n' group
fi


# ---- 8. the test layout ------------------------------------------------------
# A crate's integration tests are one binary (C122): `tests/main.rs` lists
# every other file as a module, so the shared modules and the crate's own
# dependencies link once. A file that sets the process environment (a
# `session(`, the harness's one lock and isolation list applied to the
# test's own process) is its own binary, since a process-wide write leaks
# into every test beside it under `cargo test` (measured: six and nine
# tests failed when the CLI's and the MCP's were merged). Read as files:
# the roots are the files no other file under `tests/` lists as a module;
# at most one root calls no `session(`, and it lists none that does. A
# file a manifest no longer discovers and no root lists is a second plain
# root, refused alike. Then held to the manifest, read with tomllib:
# where it sets `autotests = false` its `[[test]]` paths are the roots
# exactly, each named once, so an entry deleted, which drops a binary
# silently, is refused, an entry naming no root too, and a root two
# entries name, whose suite would run twice; where discovery is on every
# file is its own binary, so a root that lists the others as modules
# would run each of them twice, and is refused.
if ! layout=$(python3 -I - <<'PY'
import glob, os, re, sys, tomllib
MOD = re.compile(r"(?m)^\s*(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
bad = []; n = 0
for tests in sorted(glob.glob("crates/*/tests")):
    text = {os.path.basename(f)[:-3]: open(f, encoding="utf-8").read() for f in sorted(glob.glob(tests + "/*.rs"))}
    if not text:
        continue
    n += 1
    listed = {m for t in text.values() for m in MOD.findall(t)}
    roots = [stem for stem in text if stem not in listed]
    plain = [stem for stem in roots if "session(" not in text[stem]]
    if len(plain) > 1:
        bad.append(f"LAYOUT  {tests}: {len(plain)} files call no session( ({', '.join(plain)}) — one is the crate's binary of modules, and the rest are modules of it or binaries that set the process environment (C122)")
    for root in plain:
        for m in MOD.findall(text[root]):
            if m in text and "session(" in text[m]:
                bad.append(f"LAYOUT  {tests}/{root}.rs lists {m} as a module, and {m}.rs calls session( — a file that sets the process environment is its own binary (C122)")
    crate = os.path.dirname(tests)
    with open(os.path.join(crate, "Cargo.toml"), "rb") as f:
        manifest = tomllib.load(f)
    rooted = {f"tests/{stem}.rs" for stem in roots}
    if manifest.get("package", {}).get("autotests", True) is False:
        paths = [os.path.normpath(t.get("path", f"tests/{t['name']}.rs")) for t in manifest.get("test", [])]
        for path in sorted({p for p in paths if paths.count(p) > 1}):
            bad.append(f"LAYOUT  {crate}/Cargo.toml names {path} in {paths.count(path)} [[test]] entries: Cargo builds and runs that suite once per entry (C122)")
        declared = set(paths)
        for path in sorted(rooted - declared):
            bad.append(f"LAYOUT  {crate}/{path} is a test binary's root and its manifest, which turns discovery off, names it in no [[test]] entry: its tests never run (C122)")
        for path in sorted(declared - rooted):
            bad.append(f"LAYOUT  {crate}/Cargo.toml names {path} in a [[test]] entry, and it is no root under tests/: a module of another binary, or no file (C122)")
    elif listed & set(text):
        bad.append(f"LAYOUT  {crate}/Cargo.toml leaves test discovery on while a file under tests/ lists the others as modules: every file is then its own binary as well, and runs twice — set autotests = false and name the binaries (C122)")
print("\n".join(bad) if bad else f"ok      {'layout':<13} {n} crates' tests: one binary of modules each, the files that set the process environment their own (C122)")
sys.exit(1 if bad else 0)
PY
); then fail=1; fi
printf '%s\n' "$layout"


# ---- 9. a test spawns a workspace binary through the harness ------------
# Every test process takes the harness's one isolation list (C122): a
# child started with a hand-written environment inherits whatever the
# shell exports, an `ACQ_*` knob or the owner's store among them, and two
# had (#87: a printed command run through `sh`, and `acq-mcp` started
# bare). So under a crate's `tests/`, outside its `harness/` directory,
# no file names `CARGO_BIN_EXE_` — a test reaches its binary through the
# harness's `acq_exe` or `exe` and spawns it through a harness function,
# each of which applies the list — and every `Command::new(` names a
# system tool on the allowlist below, by its bare literal: never a path or
# a binary's name, which a literal can carry as well as a variable, and
# never a shell. A new tool is added to the list by name. Comment
# lines are not read. The daemon crate's `tests/` is excepted by name: it
# cannot link the harness, which links the client (check 5), and starts
# `acqd` from an emptied environment; its in-crate tests are #64's.
if ! spawns=$(python3 -I - <<'PY'
import glob, re, sys
SPAWN = re.compile(r'Command::new\(\s*([^)]*)\)?')
TOOLS = {'"kill"', '"ps"'}
bad = []; n = 0
for path in sorted(glob.glob("crates/*/tests/**/*.rs", recursive=True)):
    if "/harness/" in path or path.startswith("crates/acquisition-daemon/"):
        continue
    n += 1
    lines = open(path, encoding="utf-8").read().splitlines()
    text = "\n".join("" if l.lstrip().startswith("//") else l for l in lines)
    for m in re.finditer(r"CARGO_BIN_EXE_", text):
        at = text.count("\n", 0, m.start()) + 1
        bad.append(f"SPAWN   {path}:{at} names CARGO_BIN_EXE_ outside the harness: a test reaches its binary through the harness (acq_exe, exe) and spawns it through a harness function, which applies the one isolation list (C122)")
    for m in SPAWN.finditer(text):
        arg = m.group(1).strip()
        if arg in TOOLS:
            continue
        at = text.count("\n", 0, m.start()) + 1
        what = "through a shell" if arg == '"sh"' else f"{arg[:40]}, which is not an allowed system tool (kill, ps)"
        bad.append(f"SPAWN   {path}:{at} spawns {what} outside the harness: a workspace binary is spawned by a harness function, which applies the one isolation list, and a printed command runs through the harness's printed (C122)")
print("\n".join(bad) if bad else f"ok      {'spawns':<13} {n} test files outside the harness name no binary and spawn only an allowed system tool: every workspace binary a test starts takes the one isolation list (C122)")
sys.exit(1 if bad else 0)
PY
); then fail=1; fi
printf '%s\n' "$spawns"

exit $fail
