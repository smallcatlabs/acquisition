# The first answer

Track added by the orchestrator (it takes in the plan's E4 and the scope half of D4). Investigator's report: `reports/investigators/first-answer.md`. Portfolio: proposals 1 and 3.

## What was found

Getting from nothing to an answer costs round trips, not bytes, and the round trips come from two shapes nobody chose.

**Scope is resolved three ways in one tool.** The account refuses when more than one is known. The realm refuses in `search` when two are held, and is silently pc everywhere else. The league is every league in `search` and silently "Standard" in `tabs`, `price`, `refresh`, the web and the MCP. The lesson of the main account (a remembered default became a family of states) was overlearned in one verb and not applied in the others.

**The plan path differs by door.** A refresh to "nothing to do" in a mock world took 5 calls by CLI and 23 by MCP, nine of them polls, with the plan envelope echoed back at 9.3 KB.

| Measure | Number |
| --- | --- |
| Five everyday tasks, cold, on the owner's store | 11 calls, 2 scope refusals, about 51 KB of help read |
| The MCP's first turn | 42,548 B (`initialize` 1,157 + `tools/list` 41,391) |
| The `account` parameter's text, repeated | 5,624 B of that |
| Fixed-name defaults | 36 lines in 10 files |
| `acq --help` | 13,250 B, 30 verbs |

## Checked by the orchestrator

- `search --realm all 'pseudo.total_life>=90'` answers 586 matches and marks the 98 PoE2 items undecided, so the realm refusal guards nothing the answer cannot say.
- `near()` at `bind.rs:530-556` is as described: three kept, a whole-word match ranked last.
- The default sites: `git grep -n "Realm::DEFAULT\|DEFAULT_LEAGUE" -- crates | grep /src/ | grep -v /tests/` prints 36 (the investigator said about 28).
- `acq --help | wc -c` is 13,250.
- Not re-run: the MCP byte counts and the 23-call loop (the investigator's script is `<scratch>/first-answer/flow.py`).

## Where this parts from the plan

The plan's pattern 2 names the realm and league defaults as the next convenience to remove. That is half right. The fixed names should go, and so should the refusal that replaced one of them: "everything held" is neither a remembered state nor a guess. The plan ranked first-turn bytes first; the seat count that never moved (18 to 21 calls across three fix rounds) is a shape, not a wording.

Seats sit in pristine worlds of one realm and one account. The owner's world has three accounts and two realms, and produces refusals no seat has counted.

## Position

Scope defaults to everything held and is shown whole in one `world` document (590 B for the owner's store, `trials/world.md`). The MCP's refresh takes the CLI's shape. The verb and tool lists are generated from one table (30 tools to about 14; nine CLI verbs go). The smallest inputs that change: `acq search 'pseudo.total_life>=90'` on a two-realm store answers instead of refusing; `acq price` on a store holding only a challenge league shows that league instead of an empty Standard.

Kept as it is: the refusals' wording, the scope and basis block on every answer, and text as the agent's default form (7.3 KB against 35.4 KB of pretty-printed JSON for one answer).

## Route

Proposals 1 and 3. Issues drafted: `near-names-miss-the-total`, `describe-fields-has-no-totals`, `mcp-leagues-stored-must-be-true`, `json-is-pretty-printed`, `refresh-help-says-two-rounds`, `ambiguous-account-shows-no-counts`, `tabs-offers-a-raw-verb`, `unused-verbs`.

## After the skeptic

Narrowed (`dissent/theses-1-5.md`, thesis 3). Three points hold against the position above:

- Choosing an account by which one holds items is unsafe for a write. Account A holds one item, B has just logged in and holds none: a policy write meant for B would land on A. An account with no items can still hold intent. So the account refusal stays for every verb; what it should gain is the count beside each name.
- "Everything held" is for reads. C96 keeps realm in the scope because one template means different things in two games; an answer over every realm has to keep saying what it could not decide, as `--realm all` does now.
- `price` already infers a named target's league and refuses when it is in several (`pricing.rs:320`); the fixed name applies when no target is named. And the `world` trial left out never-fetched places and leagues the policy names but the store does not hold, which are what a first refresh needs.

What stands: scope is resolved three ways; the fixed names are read silently at 36 sites; one document could say what is held.
