`roadmap/briefs/4-full-1-form-link.md` at `d4ab30ff`: a builder can start the composer, but cannot complete the step as drawn without resolving the following gaps.

- **Decisions 2 and 4 — changes what gets built.** The promise that a clause omitted locally “runs there” also covers clauses the link explicitly leaves out. For `{"league":"Standard","query":{},"extra":1}`, `translate.rs` reports `envelope.extra` as unsupported, but the link encodes only `{}`. Smallest change: distinguish clauses carried inside the linked query from omitted envelope fields, and require an envelope-only partial-answer case to hold that distinction.

- **Decisions 1–2 — changes what gets built.** The brief leaves the report’s shape and its necessary input path unresolved. `TradeOut::from` receives only `Translation`, which holds neither the original query nor its league; `QueryInput::read` discards `Decoded`, and translation first removes nulls. The first builder guess is where the untouched source survives and how the report receives its resolved realm. Smallest change: specify that path through the answer and `show` makers, and settle the proposed report field’s shape before dependent implementation, consistently with the brief’s assignment of design to the design session.

- **Done when — changes what gets built.** The checks do not establish that an answer’s emitted link preserves its source or that Decision 5 reaches every named consumer. A correct standalone composer plus presence-only answer tests could satisfy the lines while actual reports encode normalized queries; the CLI’s `shown_text` also currently ignores `against.query.trade`. Smallest change: decode links obtained from actual reports and compare them with the original query, league and resolved realm, including nested nulls and absent status; explicitly cover a reopened saved search and `show`’s report and text rendering.

- **Find first, MCP command — wording.** The command does not find where the MCP hands on the document. Running it finds a parameter comment and tests, but misses `server.rs`’s `search` method and `Ok(answer) => told(&answer)`. Smallest change: add a search for those adapter and serialization calls, and retain the existing command for report assertions.

- **Find first, final command; Done when, sitting — wording.** A search-package-only run cannot find “every test the landing moves,” and filtering for `FAIL` hides other failure diagnostics. The CLI’s `web/fill.rs` directly includes `captures.json` and tests every captured search, outside this command’s package. Smallest change: discover capture consumers with `git grep -n 'captures.json' -- crates`, run the affected packages without filtering away diagnostics, and describe the result as failing tests rather than every affected test.

- **Decisions 7; Find first — wording.** The brief carries the splitter’s classification of `zd01`–`zd08` without a complete command that reproduces it. `trade-split.py --write` supplies no recording argument; the script requires exactly one. Smallest change: add `tools/trade-split.py search/pseudo-stats/raw/searches/zd-fragments/` as the initial read-only discovery command and make the later write invocation explicit.

What I did not check: builds, tests, generators, browser behavior or live services, as instructed. I did not verify GitHub issue/comment content, CI outcomes or the uncommitted recorder files and their digest; those are outside the committed tree.

The prompt’s request to execute read-only discovery commands while prohibiting execution generally was workable: I treated repository reads and searches as permitted.

The “Not in scope” section presented no additional material gap beyond the unresolved distinctions above.
