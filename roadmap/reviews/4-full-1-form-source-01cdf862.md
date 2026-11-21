01cdf862 — the brief’s done-when is not fully met.

- **P2 — `crates/acquisition-cli/src/web/page.rs:312`, shape 3:** The saved-page renderer hides the new source fault, displaying an empty query instead. **Persisted input:** a version-2 row containing `query.trade: "not a site search"` produces `fault` and no `text`; the HTML list and clear confirmation then look like an unrestricted empty query. Render `fault` in the shared query cell and pin that rendering in the existing page golden.

- **P3 — `crates/acquisition-cli/src/cli.rs:465`, shapes 1 and 3:** Saved-search help still describes a text-only row and says an omitted realm comes from facts. **Client input:** save an Xbox trade URL without `--realm` while facts hold only PC; it correctly saves under Xbox, contradicting that help. Update the parent help, the default-realm paragraph at line 648, and the MCP realm description at `params.rs:540`; regenerate both references.

- **P3 — `crates/acquisition-cli/tests/saved_json.rs:366`:** The required version-1 compatibility coverage stops short of the brief. **Persisted input:** the old `{version:1,name,text,scope}` shape is exercised through the CLI, but no MCP or page test stages it, and the CLI test does not compare the stored value afterward. Add the missing adapter cases and exact persisted-value comparisons after reads.

- **P3 — `decisions/store.md:22`, shape 3:** C194 still says “Pinned: nothing yet; Unbuilt,” leaving an explicit done-when unmet. The concrete stale reading is that this committed implementation has no holding tests. Replace that tail with the tests that now hold the property.

Not checked: execution, gate status, test counts, and the MCP size measurement — **not verified**, as instructed; no builds or tests were run. Unchanged daemon transport internals were outside this diff review.

Nitpicks: the stale registry pointer is documentary; the other findings concern user-visible behavior or explicit acceptance criteria.

Brief, plan, and prompt: no obstacle or change suggested.
