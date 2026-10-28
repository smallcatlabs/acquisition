#!/usr/bin/env bash
# arch-check-breakers.sh — proves the dependency guard in arch-check.sh
# (§5) and the one-reader rule on a line's group (§7), fail closed, by
# breaking the code and the tool and watching it refuse. Both checks and
# these cases lived in docs-check.sh and its breakers until step 1e of the
# roadmap; the registry's cases stayed there (`docs-check-breakers.sh`).
#
# Six review rounds on the guard (`DAEMON-SPLIT-SLICE.md` at `b6197a82`, rounds 5–11)
# found the same shape each time: a reader whose empty, partial or
# renamed answer satisfied every rule. Each case here is one of those
# findings, or a route a future crate could take, and states what the
# guard must say about it — a refusal is evidence only when it names
# the property, because twice a malformed breaker printed a refusal
# that was not the property (a shell quote closed early; BSD grep
# without -P). So an edit that does not apply, or leaves the tool
# unparseable, fails the suite instead of passing as a refusal.
#
# The spawn rule's cases (§9) append to a test file the tree has, so no
# case is also a layout refusal.
#
# Run it when an edge is added to arch-check.sh (the split's step 3 added
# the client and daemon crates and their edges), not in the gate: it
# runs the check once per case. Every case restores the files it touched from copies
# and compares them byte for byte — never `git checkout`, which
# discards uncommitted work (the record's observation) — and a trap
# restores on any exit. Needs python3 and perl beside bash and jq.
set -uo pipefail
cd "$(dirname "$0")/.."

# The paths the cases create and every restore removes, named once: a
# path already there is someone's (uncommitted work, or a link to it),
# and removing it would lose it, so the suite refuses to start.
owned=(crates/acquisition-helper crates/acquisition-client/src/zz_breaker.rs crates/acquisition-cli/src/zz_breaker.rs
       crates/acquisition-mcp/src/zz_breaker.rs crates/acquisition-search/src/zz_breaker.rs crates/acquisition-cli/tests/zz_breaker.rs)
for p in "${owned[@]}"; do
  if [[ -e $p || -L $p ]]; then
    echo "refusing to start: $p exists, and the suite would delete it on every restore"; exit 2
  fi
done

scratch=$(mktemp -d "${TMPDIR:-/tmp}/acq-breakers.XXXXXX")
files=(Cargo.toml Cargo.lock crates/acquisition-plan/Cargo.toml crates/acquisition-store/Cargo.toml crates/acquisition-search/Cargo.toml crates/acquisition-cli/Cargo.toml crates/acquisition-mcp/Cargo.toml crates/acquisition-client/Cargo.toml crates/acquisition-daemon/Cargo.toml crates/acquisition-cli/tests/main.rs crates/acquisition-cli/tests/harness/mod.rs crates/acquisition-daemon/tests/main.rs tools/arch-check.sh)
for f in "${files[@]}"; do mkdir -p "$scratch/keep/$(dirname "$f")"; cp "$f" "$scratch/keep/$f"; done
restore() {
  local f
  for f in "${files[@]}"; do
    cp "$scratch/keep/$f" "$f"
    cmp -s "$scratch/keep/$f" "$f" || { echo "RESTORE FAILED: $f"; exit 2; }
  done
  rm -rf "${owned[@]}"
}
trap 'restore; rm -rf "$scratch"' EXIT

# ---- scratch crates outside the tree -----------------------------------
root=$PWD
crate() {  # crate <dir> <name> <version> <manifest tail>
  mkdir -p "$scratch/x/$1/src"; printf 'pub fn f() {}\n' >"$scratch/x/$1/src/lib.rs"
  printf '[package]\nname = "%s"\nversion = "%s"\nedition = "2024"\n%b' "$2" "$3" "$4" >"$scratch/x/$1/Cargo.toml"
}
crate bridge  external-bridge  0.1.0 "\n[dependencies]\nacquisition-daemon = { path = \"$root/crates/acquisition-daemon\" }\n"
crate dup1    duplicate-helper 0.1.0 ""
crate dup2    duplicate-helper 0.2.0 "\n[dependencies]\nacquisition-protocol = { path = \"$root/crates/acquisition-protocol\" }\n"
crate devonly devonly          0.1.0 ""
crate outside outside          0.1.0 "\n[dev-dependencies]\ndevonly = { path = \"$scratch/x/devonly\" }\n"

# ---- edits ---------------------------------------------------------------
adddep() {  # adddep <manifest> <line> [<table, default [dependencies]>]
  python3 - "$1" "$2" "${3:-[dependencies]}" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_text(); a = sys.argv[3] + '\n'
if s.count(a) == 0: s += '\n' + a
assert s.count(a) == 1, sys.argv[3]
p.write_text(s.replace(a, a + sys.argv[2] + '\n', 1))
PY
}
helper() {  # helper <manifest tail>: a workspace member the planner links
  mkdir -p crates/acquisition-helper/src; printf 'pub fn helper() {}\n' >crates/acquisition-helper/src/lib.rs
  printf '[package]\nname = "acquisition-helper"\nversion.workspace = true\nedition.workspace = true\n\n%b' "$1" >crates/acquisition-helper/Cargo.toml
  python3 - <<'PY'
import pathlib
p = pathlib.Path('Cargo.toml'); s = p.read_text(); a = '    "crates/acquisition-plan",\n'; assert s.count(a) == 1
p.write_text(s.replace(a, a + '    "crates/acquisition-helper",\n', 1))
PY
}
member_links_helper() { helper "$2"; adddep "crates/$1/Cargo.toml" 'acquisition-helper = { path = "../acquisition-helper" }' "${3:-[dependencies]}"; }   # <member> <helper manifest tail> [table]
plan_links_helper() { member_links_helper acquisition-plan "$1" "${2:-[dependencies]}"; }
tool() {  # tool <python replace old> <new>: an exact-once substitution in arch-check.sh
  python3 - "$1" "$2" <<'PY'
import pathlib, sys
p = pathlib.Path('tools/arch-check.sh'); s = p.read_text()
assert s.count(sys.argv[1]) == 1, 'edit anchor not found once: ' + sys.argv[1][:60]
p.write_text(s.replace(sys.argv[1], sys.argv[2]))
PY
}
TABLE_END='\t\(.[0])\t\(.[1])\t\(.[2])")'"'"' "$meta"); then'
shadow_table() { tool "$TABLE_END" "${TABLE_END/\"\$meta\"); then/\"\$meta\" | $1); then}"; }   # a filter on the table reader's answer
META_CMD='if ! cargo metadata --format-version 1 --all-features --offline >"$meta" 2>/dev/null \
   && ! cargo metadata --format-version 1 --all-features >"$meta"; then'
shadow_meta() { tool "$META_CMD" "if ! cat \"$1\" >\"\$meta\"; then"; }   # a file in place of cargo metadata

# ---- the harness -----------------------------------------------------------
pass=0; failed=0
run_case() {  # run_case <name> <refuse|pass> <pattern the output must contain> <setup...>
  local name=$1 want=$2 pattern=$3; shift 3
  local out status verdict
  if ! "$@" 2>"$scratch/setup.err"; then
    printf 'FAIL    %-52s the breaker did not apply: %s\n' "$name" "$(head -c 200 "$scratch/setup.err")"; failed=$((failed+1)); restore; return
  fi
  if ! bash -n tools/arch-check.sh 2>/dev/null; then
    printf 'FAIL    %-52s the breaker left the tool unparseable\n' "$name"; failed=$((failed+1)); restore; return
  fi
  out=$(tools/arch-check.sh 2>&1); status=$?
  if [[ $want == refuse && $status -ne 0 && $out == *"$pattern"* ]] \
     || [[ $want == pass && $status -eq 0 && $out == *"$pattern"* ]]; then
    printf 'ok      %-52s %s: %s\n' "$name" "$want" "$(grep -m1 -F "$pattern" <<<"$out" | cut -c1-90)"; pass=$((pass+1))
  else
    printf 'FAIL    %-52s wanted %s with "%s"; exit %d, got:\n%s\n' "$name" "$want" "$pattern" "$status" "$(grep -E 'EDGE|GROUP|LAYOUT|dependencies|layout' <<<"$out" | sed 's/^/          /')"
    failed=$((failed+1))
  fi
  restore
}
none() { :; }
P='acquisition-plan → acquisition-helper → acquisition-daemon'
TR='links acquisition-daemon transitively'

echo '== the code: edges the planner must not have'
run_case 'clean tree'                                            pass   'ok      dependencies' none
run_case 'plan links core directly'                              refuse 'links acquisition-daemon —' adddep crates/acquisition-plan/Cargo.toml 'acquisition-daemon = { path = "../acquisition-daemon" }'
run_case 'plan → helper → core'                                  refuse "$P" plan_links_helper '[dependencies]\nacquisition-daemon = { path = "../acquisition-daemon" }\n'
run_case "helper → core under cfg(windows)"                      refuse "$P" plan_links_helper "[target.'cfg(windows)'.dependencies]\nacquisition-daemon = { path = \"../acquisition-daemon\" }\n"
run_case 'helper → core optional behind a feature'               refuse "$P" plan_links_helper '[dependencies]\nacquisition-daemon = { path = "../acquisition-daemon", optional = true }\n\n[features]\ndaemon = ["dep:acquisition-daemon"]\n'
run_case 'helper → external-bridge (optional, no member) → core' refuse 'acquisition-helper → external-bridge → acquisition-daemon' plan_links_helper "[dependencies]\nexternal-bridge = { path = \"$scratch/x/bridge\", optional = true }\n"
run_case 'union over targets: cfg(unix) then cfg(windows)'       refuse "$P" plan_links_helper "[target.'cfg(windows)'.dependencies]\nacquisition-daemon = { path = \"../acquisition-daemon\" }\n" "[target.'cfg(unix)'.dependencies]"

echo '== the code: the edges of the split'"'"'s step 3 (C1 as amended; §2.1)'
DP='acquisition-daemon = { path = "../acquisition-daemon" }'
CP='acquisition-client = { path = "../acquisition-client" }'
run_case 'client links daemon directly'                          refuse 'acquisition-client     links acquisition-daemon —' adddep crates/acquisition-client/Cargo.toml "$DP"
run_case 'client links plan directly'                            refuse 'acquisition-client     links acquisition-plan —' adddep crates/acquisition-client/Cargo.toml 'acquisition-plan = { path = "../acquisition-plan" }'
run_case 'client → helper → daemon'                              refuse 'acquisition-client → acquisition-helper → acquisition-daemon' member_links_helper acquisition-client "[dependencies]\n$DP\n"
run_case 'daemon links client directly'                          refuse 'acquisition-daemon     links acquisition-client —' adddep crates/acquisition-daemon/Cargo.toml "$CP"
run_case 'daemon links client as a dev-dependency'               refuse 'acquisition-daemon     links acquisition-client —' adddep crates/acquisition-daemon/Cargo.toml "$CP" '[dev-dependencies]'
run_case 'daemon links plan directly'                            refuse 'acquisition-daemon     links acquisition-plan —' adddep crates/acquisition-daemon/Cargo.toml 'acquisition-plan = { path = "../acquisition-plan" }'
run_case 'daemon → helper → client'                              refuse 'acquisition-daemon → acquisition-helper → acquisition-client' member_links_helper acquisition-daemon "[dependencies]\n$CP\n"
run_case 'the CLI links the daemon directly'                     refuse 'is linked by acquisition-cli' adddep crates/acquisition-cli/Cargo.toml "$DP"
run_case 'the MCP links the daemon directly'                     refuse 'is linked by acquisition-mcp' adddep crates/acquisition-mcp/Cargo.toml "$DP"
run_case 'the CLI → helper → daemon'                             refuse 'acquisition-cli → acquisition-helper → acquisition-daemon' member_links_helper acquisition-cli "[dependencies]\n$DP\n"
run_case 'the CLI → helper → daemon under cfg(windows)'          refuse 'acquisition-cli → acquisition-helper → acquisition-daemon' member_links_helper acquisition-cli "[target.'cfg(windows)'.dependencies]\n$DP\n"
run_case 'plan links client directly'                            refuse 'acquisition-plan       links acquisition-client —' adddep crates/acquisition-plan/Cargo.toml "$CP"
# store → daemon and store → client are cycles (the daemon and, since the
# split's step 5, the client link the store), and Cargo refuses a cycle
# before any guard reads the graph: the refusal is Cargo's, named as
# such — the edge is unreachable by construction, not by the rule.
run_case 'store links client directly (a cycle: Cargo refuses it first)' refuse 'cargo metadata failed' adddep crates/acquisition-store/Cargo.toml "$CP"
run_case 'store links daemon directly (a cycle: Cargo refuses it first)' refuse 'cargo metadata failed' adddep crates/acquisition-store/Cargo.toml "$DP"
run_case 'the store links tokio through a [dependencies.tokio] table' refuse 'acquisition-store      links tokio' adddep crates/acquisition-store/Cargo.toml 'workspace = true' '[dependencies.tokio]'

echo '== the code: the search crate'"'"'s edges (C89)'
SP='acquisition-search = { path = "../acquisition-search" }'
# The search links the store and the planner, the edges C89 allows, and
# both are in the tree as it stands (the planner for the price a query
# reads, C81): the clean tree is the case, an edit adding either again
# only a manifest Cargo refuses.
run_case 'search links store and plan: what C89 allows'          pass   'ok      dependencies' none
run_case 'search links daemon directly'                          refuse 'acquisition-search     links acquisition-daemon —' adddep crates/acquisition-search/Cargo.toml "$DP"
run_case 'search links client directly'                          refuse 'acquisition-search     links acquisition-client —' adddep crates/acquisition-search/Cargo.toml "$CP"
run_case 'search links an async runtime'                         refuse 'acquisition-search     links tokio' adddep crates/acquisition-search/Cargo.toml 'tokio.workspace = true'
run_case 'search → helper → daemon'                              refuse 'acquisition-search → acquisition-helper → acquisition-daemon' member_links_helper acquisition-search "[dependencies]\n$DP\n"
run_case 'daemon links search directly'                          refuse 'acquisition-daemon     links acquisition-search —' adddep crates/acquisition-daemon/Cargo.toml "$SP"
run_case 'daemon → helper → search'                              refuse 'acquisition-daemon → acquisition-helper → acquisition-search' member_links_helper acquisition-daemon "[dependencies]\n$SP\n"
# The search links the planner, so this edge is a cycle and the refusal
# is Cargo's, as for the store below.
run_case 'plan links search directly (a cycle: Cargo refuses it first)' refuse 'cargo metadata failed' adddep crates/acquisition-plan/Cargo.toml "$SP"
# The search links the store (the build plan, step 4), so this edge is a
# cycle and the refusal is Cargo's, as for the store's other upward edges
# above; the rule's own refusal is what it was while the search linked none.
run_case 'store links search directly (a cycle: Cargo refuses it first)' refuse 'cargo metadata failed' adddep crates/acquisition-store/Cargo.toml "$SP"
intent_in_client() { printf '// breaker: a client that reads intent\npub fn f() -> Option<u32> { let annotations_path = 1; Some(annotations_path) }\n' >crates/acquisition-client/src/zz_breaker.rs; }
run_case 'the intent API named in client/src'                    refuse 'names the intent API' intent_in_client

echo '== the code: a frontend opens a door only the verbs crate opens (C46)'
DR='opens a door only the verbs crate opens'
door_in() { printf '// breaker: a frontend opening a door itself\nfn f() { let _ = %s; }\n' "$2" >"crates/$1/src/zz_breaker.rs"; }   # <frontend> <expression>
run_case 'the CLI loads the account index'                       refuse "$DR" door_in acquisition-cli 'Index::load(dir)'
run_case 'the MCP opens a facts file'                            refuse "$DR" door_in acquisition-mcp 'Store::open(path)'
run_case 'the CLI opens an intent file'                          refuse "$DR" door_in acquisition-cli 'Annotations::open_for(dir, uuid)'
run_case 'the MCP compiles a plan'                               refuse "$DR" door_in acquisition-mcp 'plan_refresh(provider, snapshot, now)'
run_case 'the CLI runs the spend gate'                           refuse "$DR" door_in acquisition-cli 'plan.check_spendable(provider, uuid, intent)'
run_case 'the MCP builds an apply'"'"'s parameters'                refuse "$DR" door_in acquisition-mcp 'apply_params_of(actions, None)'
run_case 'the CLI asks for a quote'                              refuse "$DR" door_in acquisition-cli 'client.quote(jobs, None)'
run_case 'the MCP reads the ledger'                              refuse "$DR" door_in acquisition-mcp 'Persisted::observe(provider)'
run_case 'the CLI opens the use door'                            refuse "$DR" door_in acquisition-cli 'Client::connect(options)'

echo '== the code: a group'"'"'s meaning has one reader (the build plan, step 4b)'
GR="a group's tree is named outside"
tree_in_cli()    { printf '// breaker: a frontend reading a selector off the syntax\nfn f(m: &Member) -> bool { matches!(m, Member::Is(_)) }\n' >crates/acquisition-cli/src/zz_breaker.rs; }
bound_in_search() { printf '// breaker: a second reader of the bound form\nfn f(m: &BMember) -> bool { matches!(m, BMember::Const(true)) }\n' >crates/acquisition-search/src/zz_breaker.rs; }
qualified()      { printf '// breaker: the same, by its path\nfn f(m: &crate::tree::Member) -> bool { matches!(m, crate::tree::Member::Const(_)) }\n' >crates/acquisition-search/src/zz_breaker.rs; }
run_case 'a frontend matches on a group'"'"'s tree'                 refuse "$GR" tree_in_cli
run_case 'a search module matches on the bound form'             refuse "$GR" bound_in_search
aliased()        { printf '// breaker: the tree under another name\nuse acquisition_search::Member as M;\nfn f(m: &M) -> bool { matches!(m, M::Is(_)) }\n' >crates/acquisition-cli/src/zz_breaker.rs; }
run_case 'the tree under an alias'                               refuse "$GR" aliased
run_case 'the tree named by its path'                            refuse "$GR" qualified
no_group_module() { tool "group_modules='^crates/acquisition-search/src/(tree|parse|print|json|template|group)\\.rs:'" "group_modules='^crates/acquisition-search/src/(tree|parse|print|json|template)\\.rs:'"; }
run_case 'the rule forgets the module that computes the meaning' refuse "$GR" no_group_module

echo '== the code: one test binary of modules per crate, the files that set the environment their own (C122)'
second_plain() { printf '// breaker: a second binary that sets nothing\n#[test]\nfn f() {}\n' >crates/acquisition-cli/tests/zz_breaker.rs; }
merged_session() { printf 'mod watch_recovery;\n' >>crates/acquisition-cli/tests/main.rs; }
run_case 'a second file without session( under tests/'           refuse 'LAYOUT  crates/acquisition-cli/tests: 2 files call no session(' second_plain
run_case 'a session( file listed as a mod of the plain one'      refuse 'lists watch_recovery as a module' merged_session
unlisted_root() {  # the CLI manifest's watch_recovery entry deleted
  python3 - <<'PY'
import pathlib
p = pathlib.Path('crates/acquisition-cli/Cargo.toml'); s = p.read_text()
a = '\n[[test]]\nname = "watch_recovery"\npath = "tests/watch_recovery.rs"\n'
assert s.count(a) == 1, 'the watch_recovery entry is not there once'
p.write_text(s.replace(a, ''))
PY
}
discovery_on() {  # the search manifest's autotests = false deleted
  python3 - <<'PY'
import pathlib
p = pathlib.Path('crates/acquisition-search/Cargo.toml'); s = p.read_text()
a = 'autotests = false\n'
assert s.count(a) == 1, 'autotests = false is not there once'
p.write_text(s.replace(a, ''))
PY
}
run_case 'a binary'"'"'s [[test]] entry deleted'                    refuse 'tests/watch_recovery.rs is a test binary'"'"'s root' unlisted_root
run_case 'discovery on beside a binary of modules'               refuse 'acquisition-search/Cargo.toml leaves test discovery on' discovery_on
second_entry() { printf '\n[[test]]\nname = "main_copy"\npath = "tests/main.rs"\n' >>crates/acquisition-cli/Cargo.toml; }
run_case 'a root named by two [[test]] entries'                  refuse 'names tests/main.rs in 2 [[test]] entries' second_entry
run_case 'the layout as it stands'                               pass   'ok      layout' none

echo '== the code: a test spawns a workspace binary through the harness (C122, #87)'
in_tests()   { printf '%b' "$2" >>"$1"; }   # <file> <rust appended>
cli_test()   { in_tests crates/acquisition-cli/tests/main.rs "$1"; }
BIN='CARGO_BIN_'; BIN+='EXE_acq'   # built, so no source here holds the name whole
run_case 'a test names the binary'                               refuse 'names CARGO_BIN_EXE_ outside the harness' cli_test "\nfn zz() -> &'static str { env!(\"$BIN\") }\n"
run_case 'a test spawns by a path'                               refuse 'spawns by a path'           cli_test '\nfn zz(p: &std::path::Path) { let _ = std::process::Command::new(p); }\n'
run_case 'a test spawns by a path on the next line'              refuse 'spawns by a path'           cli_test '\nfn zz(p: &std::path::Path) {\n    let _ = std::process::Command::new(\n        p,\n    );\n}\n'
run_case 'a test spawns a shell'                                 refuse 'spawns through a shell'     cli_test '\nfn zz() { let _ = std::process::Command::new("sh"); }\n'
run_case 'a test runs a system tool by name'                     pass   'ok      spawns'            cli_test '\nfn zz() { let _ = std::process::Command::new("kill"); }\n'
run_case 'a comment names the binary'                            pass   'ok      spawns'            cli_test "\n// the binary is $BIN, through the harness\n"
run_case 'the harness names the binary and spawns it by a path'  pass   'ok      spawns'            in_tests crates/acquisition-cli/tests/harness/mod.rs "\nfn zz() { let _ = Command::new(env!(\"$BIN\")); }\n"
run_case 'the daemon crate'"'"'s tests, which cannot link the harness' pass 'ok      spawns'           in_tests crates/acquisition-daemon/tests/main.rs '\nfn zz(p: &std::path::Path) { let _ = Command::new(p); }\n'

echo '== the code: shapes that must pass'
dup() { adddep crates/acquisition-store/Cargo.toml "duplicate-helper = { path = \"$scratch/x/dup1\" }"; adddep crates/acquisition-cli/Cargo.toml "duplicate-helper = { path = \"$scratch/x/dup2\", version = \"0.2.0\" }"; }
run_case 'same name, two versions: store v1, cli v2 → protocol'  pass   'ok      dependencies' dup
run_case 'an excluded path crate with a dev-only path dependency' pass  'ok      dependencies' adddep crates/acquisition-plan/Cargo.toml "outside = { path = \"$scratch/x/outside\" }"
# A frontend is a bin-only package: Cargo drops a dependency on it as
# invalid ("missing a lib target") and resolves no edge, so nothing can
# link a frontend and the daemon's rule against one cannot be staged
# today. The case documents the fact; it fails the day a frontend gains a
# library target (the Tauri GUI's shape), when the rule earns a breaker.
run_case 'the daemon declares a bin-only frontend: Cargo drops it, nothing links' pass 'ok      dependencies' adddep crates/acquisition-daemon/Cargo.toml 'acquisition-cli = { path = "../acquisition-cli" }'

echo '== the tool: the table reader answers wrongly, the staged plan → helper → core path standing'
staged() { plan_links_helper '[dependencies]\nacquisition-daemon = { path = "../acquisition-daemon" }\n'; }
with_staged() { staged && "$@"; }
run_case 'the plan→core row replaced by a duplicate row'         refuse 'differs from the graph at row' with_staged shadow_table "perl -ne 'next if /^closure\\tacquisition-plan\\tacquisition-daemon\\t/; print; print if /^closure\\tacquisition-plan\\tacquisition-helper\\t/'"
run_case 'the row replaced by a fabricated row'                  refuse 'differs from the graph at row' with_staged shadow_table "perl -ne 'next if /^closure\\tacquisition-plan\\tacquisition-daemon\\t/; print; END { print \"closure\\tx\\ty\\tz\\tw\\tv\\n\" }'"
run_case 'two rows swapped, nothing lost'                        refuse 'differs from the graph at row' with_staged shadow_table "perl -e '@l=<>; (\$l[100],\$l[101])=(\$l[101],\$l[100]); print @l'"
run_case 'the dependency name of the row renamed'                refuse 'differs from the graph at row' with_staged shadow_table "perl -pe 's/^closure\\tacquisition-plan\\tacquisition-daemon\\t/closure\\tacquisition-plan\\tmasked-core\\t/'"
run_case 'the member name of the row renamed'                    refuse 'differs from the graph at row' with_staged shadow_table "perl -pe 's/^closure\\tacquisition-plan\\tacquisition-daemon\\t/closure\\tmasked-plan\\tacquisition-daemon\\t/'"
run_case 'a direct row'"'"'s dependency name renamed'             refuse 'differs from the graph at row' with_staged shadow_table "perl -pe 's/^direct\\tacquisition-plan\\tnormal\\tacquisition-store\\t/direct\\tacquisition-plan\\tnormal\\tmasked-store\\t/'"
run_case 'only the path column altered (diagnostic, unread)'     refuse "$TR" with_staged shadow_table "perl -pe 's/(^closure\\tacquisition-plan\\tacquisition-daemon\\t.*\\t).*$/\\1masked path/'"
run_case 'the self rows only'                                    refuse 'rows, the graph has' with_staged shadow_table "perl -ne 'print if /^closure\\t(acquisition-[a-z]+)\\t\\1\\t/'"
run_case 'every row two or more steps deep dropped'              refuse 'rows, the graph has' with_staged shadow_table "grep -v ' → .* → '"
run_case 'the direct rows dropped'                               refuse 'rows, the graph has' with_staged shadow_table "grep -v '^direct'"
run_case 'the table empty, exit 0'                               refuse 'rows, the graph has' with_staged shadow_table 'sed d'

echo '== the tool: the metadata partial, or the tools absent'
capture() { staged && { cargo metadata --format-version 1 --all-features --offline >"$scratch/staged.json" 2>/dev/null || cargo metadata --format-version 1 --all-features >"$scratch/staged.json"; }; }
no_node() { capture && jq '.resolve.nodes |= map(select(.id | test("acquisition-helper") | not))' "$scratch/staged.json" >"$scratch/no-node.json" && shadow_meta "$scratch/no-node.json"; }
no_pkg()  { capture && jq '.packages |= map(select(.name != "acquisition-helper"))' "$scratch/staged.json" >"$scratch/no-pkg.json" && shadow_meta "$scratch/no-pkg.json"; }
run_case 'a member node missing from the resolve'                refuse 'a workspace member is not a node' no_node
run_case 'a package entry missing'                               refuse 'a node has no package entry' no_pkg
run_case 'the table jq answers nothing'                          refuse 'rows, the graph has' with_staged tool '([.packages[] | {key: .id, value: .name}] | from_entries) as $name
    | ([.resolve.nodes[] | {key: .id, value: .deps}] | from_entries) as $deps
    | ([.workspace_members[] as $m' 'empty | ([.packages[] | {key: .id, value: .name}] | from_entries) as $name
    | ([.resolve.nodes[] | {key: .id, value: .deps}] | from_entries) as $deps
    | ([.workspace_members[] as $m'
nojq() { mkdir -p "$scratch/nojq"; printf '#!/bin/sh\nexit 127\n' >"$scratch/nojq/jq"; chmod +x "$scratch/nojq/jq"; tool 'cd "$(dirname "$0")/.."' "cd \"\$(dirname \"\$0\")/..\"; export PATH=\"$scratch/nojq:\$PATH\""; }
run_case 'jq absent'                                             refuse 'jq failed or is not installed' with_staged nojq
run_case 'cargo metadata misspelled, both calls'                 refuse 'cargo metadata failed' with_staged tool "$META_CMD" "${META_CMD//cargo metadata/cargo metadataa}"

echo
printf '%d ok, %d failed\n' "$pass" "$failed"
((failed == 0))
