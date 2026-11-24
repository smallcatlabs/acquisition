`roadmap/briefs/4-full-1-form-link.md` at `63593987`: a builder could start as written; I found no invariant or dependency-boundary contradiction.

- **Find first, captures’ readers — wording.** Searching only for `captures.json` misses consumers of the paired `bodies.json` extract. The command omits [trade_forms.rs](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/tests/trade_forms.rs:29), whose count assertion at line 453 can move when this sitting lands. Smallest change: use `git grep -l -e 'captures.json' -e 'bodies.json' -- crates`. Running the resulting packages unfiltered already covers this test, so this is a discovery correction.

- **Find first, MCP — wording.** The command misses both the `show` document handoff and an existing test that reads the trade report. It finds neither [server.rs’s `told(&shown)`](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/src/server.rs:367) nor [search_tools.rs’s report equality](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/search_tools.rs:810), although Decision 5 includes `show`. Smallest change: add patterns for `told(&shown)` and the literal `["trade"]`.

I did not build, test, execute the splitter or other project scripts, open links, verify ignored recording contents, or query GitHub/CI. Those fall outside the requested read-only review of the committed tree. I ran the six `git grep` discovery commands against the named revision.

The brief’s phrase “each count pin … each move said” made the sitting’s acceptance criterion unnecessarily difficult to parse. Nothing in the prompt impeded the review.
