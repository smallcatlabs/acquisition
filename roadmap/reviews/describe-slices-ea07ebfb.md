eb9f29b9^..ea07ebfb — done-when not fully met: #153’s index and #154’s slicing are implemented, but shell guidance and duplicate wording pins remain.

- **[P2] crates/acquisition-cli/src/search_cmd.rs:1827 — shapes 6, 7.** The terminal guidance hardcodes single quotes instead of using the declared shell’s quoting; the help at line 155 repeats this. Concrete client input: paste `acq search --describe '~resist'` into cmd.exe with `ACQ_SHELL=cmd`; cmd preserves the apostrophes, so describe receives an unknown name instead of a pattern. Render runtime examples through the existing shell quoting helper, use a portable static example such as `--describe=~resist`, and update the reference and golden.

- **[P3] crates/acquisition-search/tests/surface_faults.rs:898 — shape 10.** New literal assertions pin wording already held by `goldens/search_json/describe_at_a_terminal.txt`; the same cause appears in CLI `search_json.rs:1251` and MCP `search_tools.rs:478`. Concrete case: reword “This is the index” and regenerate its golden—the independent prefix assertions still require the old wording. Keep kind, offer and payload assertions; compare the returned index with the bare index response instead of asserting its prose.

Execution evidence—gate statuses, test counts, measurements and shell round trips—is **not verified**; this was a read-only review. No daemon, persisted-store or upstream-response paths were exercised.

Nitpicks: finding 2 is a small maintenance finding; finding 1 affects an explicitly supported shell.

Brief: Do 6 should permit portable or shell-specific examples instead of mandating single quotes.  
Plan: no other obstruction.  
Prompt: no change needed.
