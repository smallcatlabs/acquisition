# 4-full-1-form-link — A search's link back to the trade site

## Goal
An answer whose source is a site search carries a link to it on the
trade site, written from the query object it holds, at every door
(`4-full-1-form-design.md` §7, §8 step 9, §9; #303; C195, the search
goes back as asked). The sitting of 2026-11-23, round zd, lands.
Closes #303 and the form series.

## Inputs
- #303's comment (round zd: the site takes any encoder's id, any key
  order, no `status`; which status it then reads is not shown).
- C79 (a person opens the link), C113 (the decode), C141 (the league
  beside the JSON), C142 (the defences' reading), C195, C153 (one golden
  per wording), C154 (a retirement names its holder); `SURFACES.md`'s
  row for the site's data endpoints.
- Code: the search crate's `trade.rs` whole (`decode_url`, `inflate`,
  `percent_decoded`, `MAX_ID`, `MAX_INFLATED`, `CONSOLE_REALMS`);
  `answer.rs` (`QueryInput::read`, `Read`, `TradeOut`, where the answer
  makes it); `trade/translate.rs` (`absent_nulls`); `show.rs`;
  `describe.rs` (`trade_report`); `tests/trade_decode.rs`; the CLI's
  `search_cmd.rs` (`trade_text`, `fidelity_words`), `web/page.rs`
  (`trade_section`), `web/mod.rs` (its doc's referrer policy),
  `tests/web.rs` (`c138_…`).
- The sitting: the recorder's files in `~/Downloads/acq-sitting`,
  copied to `search/pseudo-stats/raw/searches/zd-fragments/`, the folder's
  `shasum -a 256 * | awk '{print $1}' | sort | shasum -a 256`
  `c2afcd52aee173d55229c358ba2512faf64df816099aa95f63a2766a71455e0b`;
  the sheet at `6d82a1c6` (`trade-sheet.py`'s `round_zd`, `ENCODINGS`);
  the site-sitting skill's steps 6 to 10; `9024f691`, the last landing.
- CI at `6d82a1c6` (`gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, and Windows): Linux 45297900731 success, Windows
  45297900653 in progress at writing.

## Decisions
The design session's (C172, who designs), settled with the owner:
1. **One composer**, in `trade.rs` beside the decode, its rule run
   backwards: the id is the query object's text as `serde_json` writes
   it, gzipped, base64url unpadded; the URL
   `https://www.pathofexile.com/trade/search/<league>/<id>` for pc,
   `…/search/(xbox|sony)/<league>/<id>` for a console, the league
   percent-encoded; no new dependency. Its input is the `Decoded`
   before the translation drops a null, and the realm the search runs
   in (the answer's resolved scope); `Read`, which drops the `Decoded`
   today, and `TradeOut`'s makers carry it. Nothing pins the bytes: the
   site took each encoding asked (zd02 to zd04).
2. **One home, the trade report** (`TradeOut`): a key holding the link,
   or the one sentence that says why there is none; with the link, each
   thing beside the query the site cannot be asked, named: `defences:
   "displayed"` (C142), a `sort`, any other key beside the query. The
   CLI prints it, the page draws it, the MCP hands on the document.
   The key's name and shape are agent-facing and the design session's:
   the builder drafts them, and the report gives the draft.
3. **No link**, each in one sentence of the report: the search names no
   one league (an id, or a JSON with none); the scope is every realm,
   or PoE2; the link would not decode (its id longer than `MAX_ID`, or
   its query past `MAX_INFLATED`). A search in the language has no
   trade report: `describe`'s entry for the key says once that it has
   no link.
4. **What the link is**, in the search crate's words, read by every
   door: it carries the query as asked, so a clause of the query
   object not run here runs there, said where one did not run (a key
   beside it is left out, Decision 2); and what the search leaves
   unsaid the site fills with its own, its status among them, in words
   that do not name which status.
5. **The search that ran**: made with the answer, never from a form's
   unsent edits. A saved site search opened and run carries it as any
   answer does; `show`'s document carries it in its `TradeOut`, and its
   text, which prints no trade report today, gains none.
6. **The page**: the link beside the trade report, `target="_blank"`,
   `rel="noopener"`, the one link of the pages that leaves the server.
   `Referrer-Policy: same-origin` (`web/mod.rs`'s doc) keeps the token
   from the site, held by a test on a page that carries the link.
7. **The sitting lands here**: zd01 to zd05 by `trade-split.py
   --write`; zd06 to zd08, refused by it as no row's, read from the
   recorder's files, their rows left without a capture as y17's are.
8. **The records**: `SURFACES.md`'s row loses "Unbuilt, #303"; C195's
   Pinned names the tests and drops "Unbuilt". Another ruling's
   sentence the build shows stale is reported, not edited.

## Find first
- `git grep -n -e 'TradeOut' -e 'trade_report' -e 'trade_text' -e 'trade_section' -e 'fidelity_words' -- crates`:
  where the report's keys are listed, printed or drawn.
- `git grep -n -e 'trade/search' -e 'CONSOLE_REALMS' -- crates tools/trade-sheet.py ':!*/goldens/*'`:
  every place a site URL's shape is written.
- `git grep -n -i -e 'compose' -e 'outbound' -e 'mints' -e 'Unbuilt, #303' -- decisions SURFACES.md crates/*/src docs/design README.md`:
  each text saying a link cannot be made or is unbuilt.
- `git grep -n -e fidelity -e query.trade -e 'told(&answer)' -- crates/acquisition-mcp`:
  where the MCP hands on the answer and its tests read the report.
- `git grep -n -e 'target=' -e 'rel=' -e 'href="http' -- crates/acquisition-cli/src/web`:
  any link that leaves the server so far.
- `tools/trade-split.py search/pseudo-stats/raw/searches/zd-fragments`:
  what the recording holds, before `--write` on the same folder.
- `git grep -l 'captures.json' -- crates`: the captures' readers; with
  them written, `cargo nextest run --no-fail-fast` over those packages,
  unfiltered: the tests the landing fails.

## Done when
- `trade_decode.rs`: a proptest composes a link from generated query
  objects, each realm and generated leagues (a space, `%`, `/`, `?`,
  `#`, non-ASCII among them) and `decode` gives back that query, realm
  and league; a case past each bound gives the no-link sentence.
- Search crate: the answers to a URL, an id, a JSON, a reopened saved
  search and `show` each carry the link or Decision 3's sentence; a
  report's link decodes to the source's query (nested nulls, no
  `status`), league and realm; `defences: "displayed"`, a `sort` and a
  key beside the query named; a partial answer says a clause not run
  runs there, one whose remainder is an envelope key alone does not;
  `describe trade` holds the key and the language's no-link line.
- `search_json` goldens hold the CLI's lines; a `web.rs` case and a
  golden hold the link drawn with `target="_blank" rel="noopener"` and
  the page's `referrer-policy` `same-origin`; an MCP case finds the key
  in the document.
- The sitting: what the skill's steps 6 to 10 write; a `MANIFEST.md`
  paragraph and row as rounds zb and zc have; the site's answers as
  claims in `docs/design/trade-ground-truth.md`, as `9024f691`'s T36 to
  T38; each count pin the last Find-first command finds holds what the
  tests measure, each move said.
- `SURFACES.md` and C195 as Decision 8 says.
- `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the goldens regenerated
  under `ACQ_UPDATE_FIXTURES=1`, each diff read.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- A link for a query in the language or a form's unsent edits; the
  outbound translation ("Not doing", `decisions/trade.md`).
- Editable listing fields; PoE2 on the trade site; the look (#285).
- Opening or checking a link by any tool: a person opens it (C79).

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill); tooling sends nothing to the trade site. The recording stays
under `raw/`, never committed. A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
