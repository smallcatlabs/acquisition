All numbers come from `git ls-files` at HEAD 122c44fa. The script is `scratch/census.py`; it runs the whole census, except that the Cargo.lock counts, per-crate manifest listings and byte totals in tables 6 and 7 were done with awk, `git grep` and a short inline `python3 -I -c`. No cargo, tests or `acq*` ran, and nothing was written inside the repo.

Rules used:
- TEST = path contains `/tests/` or `/unit/`; everything else is SOURCE (including `build.rs`, `examples/`, and inline `#[cfg(test)]` modules in `src/*.rs`).
- Every `.rs` file is under `crates/`; there are none elsewhere.
- Test-attribute counts cover SOURCE and TEST files.
- Table 4 counts are on comment- and string-stripped code, so they exclude hits in comments and string literals. They include inline `#[cfg(test)]` code in SOURCE files, so `unwrap` etc. are not purely production counts.

## 1. Per-crate size

Command summary: lines are split on newlines; a comment line is a trimmed line starting with `//`; a doc line is one starting with `///` or `//!` (a subset of comment); code = lines − comment − blank. Non-Rust files are tracked files in the crate directory that are not `.rs`.

| crate | SRC files | SRC lines | comment | of which doc | blank | code | TEST files | TEST lines | test attrs | non-Rust files | non-Rust bytes |
|---|---|---|---|---|---|---|---|---|---|---|---|
| acquisition-cli | 24 | 27953 | 4964 | 4121 | 1025 | 21964 | 35 | 31686 | 345 | 81 | 1081255 |
| acquisition-client | 7 | 5196 | 1266 | 1136 | 264 | 3666 | 6 | 3002 | 40 | 1 | 1289 |
| acquisition-daemon | 28 | 19657 | 4652 | 3602 | 943 | 14062 | 26 | 28518 | 369 | 2 | 31636 |
| acquisition-mcp | 6 | 2905 | 648 | 571 | 150 | 2107 | 15 | 7002 | 93 | 1 | 1329 |
| acquisition-plan | 9 | 8991 | 2356 | 2217 | 408 | 6227 | 10 | 9089 | 135 | 11 | 1939474 |
| acquisition-protocol | 9 | 1967 | 791 | 759 | 112 | 1064 | 3 | 1064 | 21 | 40 | 9796 |
| acquisition-search | 38 | 35279 | 7766 | 7003 | 1661 | 25852 | 43 | 37433 | 457 | 17 | 2662161 |
| acquisition-store | 14 | 10037 | 2906 | 2577 | 510 | 6621 | 13 | 8791 | 164 | 2 | 8086 |
| acquisition-transport | 1 | 236 | 94 | 86 | 15 | 127 | 0 | 0 | 1 | 1 | 1105 |
| acquisition-verbs | 14 | 7332 | 1628 | 1504 | 367 | 5337 | 2 | 6676 | 98 | 1 | 1445 |
| TOTAL | 150 | 119553 | 27071 | 23576 | 5455 | 87027 | 153 | 133261 | 1723 | 155 | 5737576 |

Non-Rust files by extension (count, bytes):

| crate | extensions |
|---|---|
| cli | .txt 74 (1049903), .js 4 (17070), .css 1 (10981), .toml 1 (2880), .html 1 (421) |
| client | .toml 1 (1289) |
| daemon | .json 1 (28843), .toml 1 (2793) |
| mcp | .toml 1 (1329) |
| plan | .json 7 (1917483), .toml 2 (16266), .txt 2 (5725) |
| protocol | .json 39 (8944), .toml 1 (852) |
| search | .toml 15 (2641302), .json 2 (20859) |
| store | .sql 1 (6693), .toml 1 (1393) |
| transport | .toml 1 (1105) |
| verbs | .toml 1 (1445) |

## 2. Largest files

The comment % is comment lines over total lines. The `fn` count is `\bfn\s+\w+` on stripped code, so it includes inline test functions.

### 40 largest SOURCE files

| file | lines | comment % | fn defs |
|---|---|---|---|
| cli/src/web/page.rs | 5630 | 16.3 | 128 |
| cli/src/web/mod.rs | 4550 | 23.5 | 119 |
| search/src/answer.rs | 4222 | 25.9 | 104 |
| cli/src/search_cmd.rs | 3319 | 18.9 | 121 |
| store/src/lib.rs | 3166 | 28.8 | 77 |
| client/src/client.rs | 3082 | 27.6 | 112 |
| search/src/trade/translate.rs | 2764 | 18.4 | 67 |
| plan/src/lib.rs | 2482 | 30.7 | 54 |
| cli/src/plan_cmd.rs | 2432 | 11.6 | 66 |
| cli/src/price_cmd.rs | 2274 | 11.1 | 48 |
| daemon/src/rails.rs | 2245 | 19.9 | 63 |
| search/src/pseudo.rs | 2062 | 25.5 | 47 |
| plan/src/listing.rs | 2015 | 23.9 | 48 |
| daemon/src/daemon/queue.rs | 1924 | 28.5 | 50 |
| daemon/src/daemon.rs | 1841 | 36.0 | 93 |
| search/src/bind.rs | 1743 | 18.2 | 52 |
| search/src/parse.rs | 1607 | 8.8 | 56 |
| daemon/src/mockggg.rs | 1597 | 14.9 | 51 |
| search/src/eval.rs | 1577 | 21.8 | 47 |
| store/src/annotations.rs | 1555 | 29.7 | 50 |
| search/src/counts.rs | 1464 | 18.6 | 26 |
| search/src/derive.rs | 1443 | 27.9 | 35 |
| plan/src/pricing.rs | 1441 | 21.8 | 32 |
| search/src/category.rs | 1423 | 20.4 | 47 |
| cli/src/store_cmd.rs | 1371 | 11.5 | 40 |
| daemon/src/daemon/kinds.rs | 1323 | 20.4 | 24 |
| search/src/totals.rs | 1291 | 22.3 | 28 |
| plan/src/shop.rs | 1248 | 24.3 | 16 |
| daemon/src/daemon/server.rs | 1238 | 21.6 | 29 |
| search/src/group.rs | 1227 | 20.7 | 61 |
| verbs/src/saved.rs | 1210 | 17.2 | 38 |
| client/src/door.rs | 1204 | 15.2 | 23 |
| cli/src/web/form.rs | 1203 | 19.6 | 32 |
| search/src/properties.rs | 1197 | 17.9 | 42 |
| daemon/src/ratelimit/choke.rs | 1177 | 21.8 | 47 |
| search/src/describe.rs | 1157 | 14.3 | 19 |
| verbs/src/jobs.rs | 1115 | 19.1 | 41 |
| daemon/src/auth.rs | 1105 | 22.9 | 47 |
| store/src/world.rs | 1097 | 34.7 | 50 |
| search/src/corpus.rs | 1081 | 25.8 | 33 |

Paths drop the `crates/acquisition-` prefix.

### 25 largest TEST files

| file | lines | tests |
|---|---|---|
| cli/tests/web.rs | 8339 | 52 |
| verbs/tests/documents.rs | 5296 | 67 |
| daemon/src/ratelimit/tests/spec.rs | 4715 | 72 |
| cli/tests/seat_faults.rs | 4393 | 65 |
| daemon/src/daemon/tests/dispatch.rs | 3440 | 25 |
| cli/tests/search_json.rs | 3212 | 31 |
| daemon/src/daemon/tests/queue.rs | 3102 | 32 |
| store/unit/tests/lib.rs | 3033 | 51 |
| plan/unit/tests/lib.rs | 2892 | 49 |
| search/tests/common/generated.rs | 2885 | 0 |
| cli/tests/price_json.rs | 2826 | 26 |
| search/tests/rows.rs | 2393 | 26 |
| cli/tests/browser.rs | 2366 | 16 |
| daemon/src/daemon/tests/door.rs | 2132 | 7 |
| daemon/src/daemon/tests/exit.rs | 2065 | 30 |
| cli/tests/harness/mod.rs | 2016 | 0 |
| search/tests/pseudo.rs | 1967 | 18 |
| store/unit/tests/annotations.rs | 1918 | 27 |
| plan/unit/tests/listing.rs | 1882 | 19 |
| daemon/src/daemon/tests/login.rs | 1863 | 32 |
| search/tests/seat_faults.rs | 1789 | 25 |
| plan/unit/tests/shop.rs | 1644 | 18 |
| client/tests/contract.rs | 1631 | 18 |
| search/tests/answer.rs | 1631 | 28 |
| mcp/tests/search_tools.rs | 1626 | 32 |

## 3. Function length (SOURCE files)

The script `census.py` strips comments, strings, char literals and raw strings, then finds `fn name` and brace-matches from the first `{` outside parens. Trait methods ending in `;` are skipped. Length is the end line minus the start line plus 1, and nested fns and closures are not separated. Inline `#[cfg(test)]` functions are included.

Count 3482, median 11, p90 50, p99 178, max 745.

| len | file:line | name |
|---|---|---|
| 745 | cli/src/run.rs:78 | run |
| 589 | store/src/lib.rs:1575 | record_in |
| 577 | search/src/describe.rs:353 | describe |
| 533 | client/src/door.rs:634 | table |
| 476 | daemon/src/daemon/kinds.rs:450 | execute_inner |
| 444 | cli/src/web/mod.rs:2988 | places |
| 417 | search/src/answer.rs:2566 | unaged |
| 386 | search/src/trade/translate.rs:1543 | stat |
| 356 | daemon/src/daemon/queue.rs:322 | restore_jobs |
| 342 | search/src/pseudo.rs:1118 | percentile |
| 306 | search/src/category.rs:450 | parse |
| 306 | cli/src/web/mod.rs:1756 | search |
| 305 | cli/src/search_cmd.rs:352 | search |
| 273 | plan/src/listing.rs:1739 | resolve |
| 268 | search/src/counts.rs:1107 | vocabulary |
| 255 | daemon/src/daemon/server.rs:254 | handle_request |
| 254 | search/src/trade/translate.rs:1959 | group |
| 251 | search/src/totals.rs:482 | parse |
| 243 | daemon/src/mockggg.rs:756 | answer |
| 230 | search/src/corpus.rs:622 | read |
| 227 | cli/src/web/page.rs:5327 | describe |
| 225 | search/src/trade/forms.rs:405 | parse |
| 217 | cli/src/price_cmd.rs:354 | render_status |
| 216 | search/src/properties.rs:430 | parse |
| 215 | verbs/src/diagnose.rs:170 | diagnose |
| 211 | daemon/src/ratelimit/choke.rs:601 | head |
| 209 | cli/src/price_cmd.rs:869 | render_show |
| 208 | cli/src/search_cmd.rs:1766 | answer_tail |
| 200 | mcp/src/told.rs:212 | request |
| 198 | daemon/src/daemon/dispatch.rs:134 | process |
| 195 | daemon/src/daemon/quote.rs:16 | quote |
| 184 | cli/src/terminal.rs:399 | argv |
| 183 | cli/src/search_cmd.rs:2677 | describe_text |
| 181 | cli/src/price_cmd.rs:1438 | c53_price_status_opens_with_one_line_and_ends_with_the_next_action |
| 178 | cli/src/web/page.rs:3978 | place_rows |
| 174 | daemon/src/daemon/start.rs:413 | run_with_log |
| 172 | verbs/src/saved.rs:381 | set |
| 171 | cli/src/web/page.rs:1883 | answer |
| 169 | search/src/totals.rs:1046 | the_loader_refuses_what_a_reviewer_would |
| 163 | search/src/eval.rs:1202 | everything |

Several of these are large `match` bodies or table builders (`table`, `places`, `percentile`), which may be data-like rather than logic.

## 4. Pattern counts over SOURCE files

All counts are per crate. `pub fn` also counts `pub async/const/unsafe fn`. `private fn` = all `fn` − pub − pub(crate) − pub(super/in). The `fn` totals include nested fns, trait fns and inline test fns.

| crate | .unwrap() | .expect( | panic! | unreachable! | todo!/unimpl! | debug_assert | unsafe | #[allow( | #[cfg(test)] | .clone() |
|---|---|---|---|---|---|---|---|---|---|---|
| cli | 104 | 0 | 11 | 0 | 0 | 0 | 0 | 3 | 19 | 230 |
| client | 87 | 16 | 6 | 4 | 0 | 0 | 3 | 0 | 8 | 23 |
| daemon | 263 | 52 | 4 | 8 | 0 | 4 | 1 | 5 | 48 | 223 |
| mcp | 3 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 2 | 5 |
| plan | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 10 | 178 |
| protocol | 3 | 4 | 4 | 0 | 0 | 0 | 6 | 0 | 3 | 0 |
| search | 57 | 8 | 5 | 0 | 0 | 0 | 0 | 9 | 14 | 490 |
| store | 0 | 0 | 0 | 0 | 0 | 0 | 27 | 2 | 19 | 62 |
| transport | 7 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 |
| verbs | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 112 |

| crate | pub fn | pub(crate) fn | pub(super/in) fn | private fn | all fn | pub struct | pub enum | enum *Error/*Refusal | json!( | env::var calls |
|---|---|---|---|---|---|---|---|---|---|---|
| cli | 37 | 245 | 0 | 467 | 749 | 4 | 0 | 0 | 85 | 0 |
| client | 56 | 4 | 0 | 107 | 167 | 15 | 9 | 2 | 12 | 4 |
| daemon | 108 | 29 | 154 | 353 | 644 | 22 | 9 | 5 | 124 | 13 |
| mcp | 2 | 16 | 0 | 69 | 87 | 0 | 1 | 0 | 57 | 0 |
| plan | 101 | 0 | 0 | 112 | 213 | 58 | 32 | 8 | 18 | 0 |
| protocol | 28 | 2 | 0 | 13 | 43 | 18 | 12 | 0 | 18 | 4 |
| search | 316 | 179 | 0 | 546 | 1041 | 137 | 54 | 9 | 57 | 2 |
| store | 178 | 16 | 0 | 121 | 315 | 65 | 8 | 2 | 6 | 5 |
| transport | 2 | 0 | 0 | 6 | 8 | 3 | 0 | 0 | 0 | 0 |
| verbs | 132 | 2 | 0 | 96 | 230 | 59 | 13 | 0 | 50 | 1 |

- `unsafe` hits: the store's 27 are in `world.rs` (libc `getuid`/`getpwuid_r`) and `world_windows.rs` (23); others are in client `client.rs` (4), protocol `provider.rs` (6) and daemon `endpoint.rs` (1). Lint-attribute hits and `forbid(unsafe_code)` are not counted.
- `#[allow(` names, 21 in total: `clippy::too_many_arguments` 10, `dead_code` 9, `clippy::type_complexity` 1, `clippy::needless_borrow` 1. That is only 4 distinct names, so there is no top 10.
- `TODO|FIXME|XXX|HACK` in SOURCE files: 0 hits.
- Env vars read with a literal name, as distinct names with the number of files that read each: ACQ_JOURNAL 3, ACQ_SHELL 2, MSYSTEM 2, and 1 file each for ACQ_HELLO_WITHIN, ACQ_NO_SPAWN, ACQ_STARTUP_WITHIN, ACQ_NO_KEYRING, ACQ_KEYRING_ROUNDTRIP, ACQ_IDLE_SHUTDOWN, ACQ_LOGIN_WITHIN, ACQ_MOCK_DEGRADED_HEAD, ACQ_TRIPWIRE, ACQ_MAX_SENDS, CARGO_MANIFEST_DIR, ACQ_ANNOTATION_WAIT, ACQ_STORE_WAIT, ACQ_STORE_DIR, ACQ_LOG_DIR.
- Non-literal env reads, whose names sit in constants or variables so they are not in the list above: client `client.rs:2678` (`key`), daemon `start.rs:105` (`var`), daemon `mockggg.rs:659` (`WINDOW_DIVISOR`), protocol `provider.rs:37,53,59` (`KNOB`, `RETIRED_KNOB`), store `lib.rs:1221` (`NOW_KNOB`).
- The CLI has 0 `env::var` calls, so clap's `env` feature probably handles its environment reads. I did not check this.

## 5. Comment-content signals

Command summary: each column counts SOURCE comment lines (trimmed text starts with `//`) matching the pattern, in Python. The regexes are `20[0-9]{2}-[0-9]{2}-[0-9]{2}`, `#[0-9]+`, `\bC[0-9]{1,3}\b`, and `\b(round|review|reviewer|seat|sitting|step|rung|slice)\b` case-insensitive. `#[0-9]+` can also match non-issue text, such as `#1` used as a numbering.

| crate | comment lines | date | issue #N | ruling Cn | words |
|---|---|---|---|---|---|
| cli | 4964 | 2 | 31 | 345 | 56 |
| client | 1266 | 0 | 0 | 154 | 6 |
| daemon | 4652 | 2 | 7 | 638 | 16 |
| mcp | 648 | 0 | 1 | 57 | 0 |
| plan | 2356 | 3 | 1 | 254 | 13 |
| protocol | 791 | 1 | 2 | 87 | 3 |
| search | 7766 | 10 | 10 | 564 | 60 |
| store | 2906 | 1 | 2 | 307 | 8 |
| transport | 94 | 0 | 1 | 11 | 2 |
| verbs | 1628 | 0 | 3 | 316 | 3 |
| total | 27071 | 19 | 58 | 2733 | 223 |

## 6. Dependencies

Cargo.lock has 351 packages (`grep -c '^name = '`). 22 packages appear in more than one version:

| package | versions |
|---|---|
| base64 | 0.22.1, 0.23.1 |
| block-buffer | 0.10.4, 0.12.1 |
| core-foundation | 0.9.4, 0.10.1 |
| cpufeatures | 0.2.17, 0.3.0 |
| crypto-common | 0.1.7, 0.2.2 |
| digest | 0.10.7, 0.11.3 |
| getrandom | 0.2.17, 0.4.3 |
| rand | 0.8.8, 0.10.3 |
| rand_core | 0.6.4, 0.10.1 |
| security-framework | 2.11.1, 3.7.0 |
| sha2 | 0.10.9, 0.11.1 |
| syn | 2.0.119, 3.0.3 |
| windows-sys | 0.52.0, 0.59.0, 0.60.2, 0.61.2 |
| windows-targets | 0.52.6, 0.53.5 |
| windows_aarch64_gnullvm | 0.52.6, 0.53.1 |
| windows_aarch64_msvc | 0.52.6, 0.53.1 |
| windows_i686_gnu | 0.52.6, 0.53.1 |
| windows_i686_gnullvm | 0.52.6, 0.53.1 |
| windows_i686_msvc | 0.52.6, 0.53.1 |
| windows_x86_64_gnu | 0.52.6, 0.53.1 |
| windows_x86_64_gnullvm | 0.52.6, 0.53.1 |
| windows_x86_64_msvc | 0.52.6, 0.53.1 |

Workspace dependencies, from the root `Cargo.toml`:
- tokio 1 [full]
- serde 1 [derive]
- serde_json 1 [preserve_order]
- anyhow 1
- clap 4 [derive, env]
- toml 1
- tempfile 3

Direct dependencies per crate. `.workspace` means the entry inherits the root definition above.

| crate | dependencies | dev-dependencies |
|---|---|---|
| cli | acquisition-{client, protocol, store, plan, search, verbs}; tokio.ws; serde.ws; serde_json.ws; anyhow.ws; clap.ws; axum 0.8 [no default; http1, tokio, query, form]; num-traits 0.2 [std]; getrandom 0.4 [std, sys_rng]; unix-only: rustix 1 [fs] | rusqlite 0.41 |
| client | acquisition-protocol; acquisition-store; tokio.ws; serde.ws; serde_json.ws; anyhow.ws; sha2 0.11 | proptest 1.11.0; tempfile.ws |
| daemon | tokio.ws; serde.ws; serde_json.ws; anyhow.ws; acquisition-{protocol, store, transport}; http 1; keyring 3 [apple-native, windows-native, async-secret-service, crypto-rust, async-io]; sha2 0.11; base64 0.23; rand 0.10; url 2 | reqwest 0.12 [no default; json, rustls-tls]; proptest 1.11.0; rusqlite 0.41; tempfile.ws |
| mcp | acquisition-{client, protocol, store, plan, search, verbs}; tokio.ws; serde.ws; serde_json.ws; anyhow.ws; rmcp 3.1 [transport-io] | rusqlite 0.41 |
| plan | acquisition-{protocol, store}; serde.ws; serde_json.ws; toml.ws; anyhow.ws | proptest 1.11.0; sha2 0.11; tempfile.ws; rusqlite 0.41 |
| protocol | serde.ws; serde_json.ws | (none); build-deps: sha2 0.11 |
| search | acquisition-{plan, store}; anyhow.ws; regex 1; regex-syntax 0.8; sha2 0.11; base64 0.23; flate2 1; serde.ws; serde_json.ws; toml.ws | proptest 1.11.0; tempfile.ws; rusqlite 0.41 [bundled, hooks]; build-deps: sha2 0.11 |
| store | serde.ws; serde_json.ws; anyhow.ws; rusqlite 0.41 [bundled, hooks, fallible_uint]; directories 6; sha2 0.11; libc 0.2; windows-only: windows-sys 0.61 [Win32_Foundation, Win32_Security, Win32_Security_Authorization, Win32_Storage_FileSystem, Win32_System_Pipes, Win32_System_Threading, Win32_UI_Shell] | tempfile.ws |
| transport | http 1; reqwest 0.12 [no default; json, rustls-tls] | tokio.ws |
| verbs | acquisition-{client, protocol, store, plan, search}; tokio.ws; serde.ws; serde_json.ws; anyhow.ws; getrandom 0.4 [std, sys_rng] | rusqlite 0.41; tempfile.ws |

## 7. Tracked-file bytes

The list covers 1590 tracked files totalling 74,444,801 bytes, by top-level directory. `(root files)` is files directly in the repo root.

| top-level dir | files | bytes |
|---|---|---|
| search | 82 | 48992549 |
| crates | 460 | 15749585 |
| roadmap | 732 | 4689106 |
| audits | 202 | 3256324 |
| tools | 51 | 927376 |
| (root files) | 25 | 412358 |
| decisions | 11 | 145270 |
| docs | 2 | 130899 |
| .claude | 12 | 97771 |
| .github | 7 | 30597 |
| .githooks | 4 | 7103 |
| .cargo | 1 | 4812 |
| .config | 1 | 1051 |

30 largest tracked files:

| file | bytes |
|---|---|
| search/pseudo-stats/data/captures.json | 18292312 |
| search/pseudo-stats/data/bodies.json | 11430212 |
| search/pseudo-stats/data/ranged-expected.csv | 2944206 |
| search/repoe/data/trade-stat-map.csv | 2476242 |
| search/trade-query/data/stats-2026-11-17.json | 2148932 |
| search/trade-query/data/stats-2026-09-12.json | 2099466 |
| search/pseudo-stats/data/percentile-expected.csv | 1983358 |
| search/pseudo-stats/data/search-sheet.csv | 1534257 |
| crates/acquisition-search/reference/trade-stats-v1.toml | 1355111 |
| crates/acquisition-plan/reference/pricing-snapshot-2026-09-07.json | 1001353 |
| search/repoe/data/template-vs-translation.csv | 979868 |
| search/item-facts/data/mod-templates.csv | 909302 |
| crates/acquisition-search/reference/trade-game-forms-v1.toml | 752549 |
| search/pseudo-stats/data/candidates.csv | 682552 |
| search/pseudo-stats/data/defence-twins.csv | 551850 |
| crates/acquisition-plan/reference/site-listings-2026-09-09.json | 441170 |
| search/trade-query/data/items-2026-11-17.json | 353688 |
| search/trade-query/data/items-2026-09-12.json | 344615 |
| crates/acquisition-cli/tests/web.rs | 333342 |
| search/trade-query/data/fetch-census.json | 284223 |
| search/category/data/expected-answers.csv | 277812 |
| crates/acquisition-cli/tests/goldens/web/search_pages.txt | 277746 |
| crates/acquisition-plan/reference/site-listings-2026-09-08.json | 275368 |
| crates/acquisition-cli/src/web/page.rs | 223590 |
| tools/trade-sheet.py | 199716 |
| crates/acquisition-cli/src/web/mod.rs | 189920 |
| crates/acquisition-verbs/tests/documents.rs | 189141 |
| crates/acquisition-search/src/answer.rs | 174172 |
| search/pseudo-stats/data/raw-searches.sha256 | 173933 |
| crates/acquisition-cli/tests/seat_faults.rs | 168812 |

Not counted: `target/` and `runs/` are present on disk but not tracked.
