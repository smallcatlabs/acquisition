`roadmap/briefs/poe2-prep.md` at `c3c54f7c`: a builder would have to guess before starting.

Findings, most severe first:

- **Decisions 3 and 6 — changes what gets built.** The trigger “GGG documents a stash endpoint” does not establish that every URL shape enabled by the family flip is documented. `daemon/routes.rs::route_for` renders listings, tabs and substashes, with separate `stash-list/poe2` and `stash/poe2` routes; C59 forbids sending an unobserved shape. Require documentation covering every enabled shape, with a stop condition if it covers only part of the family, and specify first contact for each distinct route.

- **Decisions 3, 4 and 6 — changes what gets built.** The promised mock body requires a production change that neither the “one arm” design nor the failure-driven test search specifies. In `mockggg.rs`, the response dispatcher independently restricts tab bodies to `Realm::Pc` and returns empty listings for every other realm, so flipping acceptance leaves PoE2 tabs answering 404. The first builder guess is whether to preserve those answers or introduce PoE2 fixtures. Choose the intended mock behaviour, include that change explicitly in scope, and require a test of its response and stored result.

- **Decisions 6 — changes what gets built.** A ledger template cannot have every column filled except the result without inventing observations. `RUN-LEDGER.md` requires the actual date, daemon tip, POST/HEAD/GET counts, violations and observed policy; the live-run skill also permits different send counts depending on token validity. Leave every future observation as a labelled placeholder, keeping expected ceilings separate from observed counts.

- **Done when — changes what gets built.** The acceptance criteria require a checklist file but never require it to contain decision 6’s procedure. An empty checklist, a journaled empty listing, a green gate and a saved cold review can satisfy every line while leaving the owner unable to perform first contact. Add an explicit criterion requiring the checklist’s documentation checks, executable command, ceilings, journal checks, stopping conditions, evidence placeholders and merge order.

- **Find first, first command — changes what gets built.** The claimed sweep misses affected wording and includes restrictions that must remain. Running it misses `PlanParams`’ “a poe2 plan can carry character work only” in `crates/acquisition-mcp/src/params.rs` and its generated `MCP-REFERENCE.md` entry; it also returns trade-import restrictions and the account-leagues restriction, which the flip does not invalidate. Broaden discovery to equivalent realm restrictions and state the criterion: amend current stash-capability claims, preserving trade restrictions, account-leagues restrictions and historical observations.

- **Done when, first and fourth lines — wording.** These criteria depend on mutable remote refs and a separate `main` commit rather than evidence identifiable from the reviewed commit. `main...origin/prep/poe2-stash` changes as either ref moves, and the branch cannot itself contain the review saved on `main`. Require the launching session’s close commit to name the immutable base and branch hashes and the saved review’s path and commit; assess the range using those hashes.

- **Goal and Decisions 6 — wording.** They prescribe opposite launch orders: the goal says merge then first contact, while the checklist ends with “then the merge.” State one order consistently, including which checkout supplies the first-contact binaries.

What I did not check and why: I ran the read-only discovery commands, but no build, tests, gate, daemon or live request, as instructed. GitHub issue 45 could not be read because `gh issue view 45` failed to connect; I therefore could not verify its three required facts. I did not verify current CI or GGG documentation because those are external state, not evidence in the named commit.

What got in the way: The brief’s reference to issue 45 leaves required checklist content outside the reviewable tree.

What got in the way: The prompt’s “do not … run anything” conflicts literally with its instruction to run read-only commands; I followed the explicit read-only exception.
