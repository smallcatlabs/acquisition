# Investigator's report: scope resolved once

As returned by an Opus investigator on 2026-11-25, read-only, no cargo and no `acq`. It was handed a hypothesis to break: one `Scope` value, made in one place below the frontends, carrying the account, the realm set and the league set and how each was resolved; reads with nothing named span everything held. Its claims are its own; what the orchestrator re-ran is in `../../NEXT.md`.

**Short answer:** the hypothesis is partly right and too wide. The account part is already built. The realm part ("reads span everything held") runs into an owner ruling. League is a query term in search, not a scope. And "resolved from what the world holds" can't be a value made before the read.

## 1. Where account, realm and league get their value today

| Family | Account | Realm | League |
|---|---|---|---|
| all doors | resolved in one place: `Frontend::account` (`verbs/src/frontend.rs:279`); refused when ambiguous (C121) | | |
| search | ″ | named, else the only one held, refused when several: `search/src/corpus.rs:647-660` (ruled, C96 `decisions/search.md:23`) | a query term; every league by default (C96) |
| show | ″ | every realm (`RealmScope::All`, `search/src/show.rs:670`) | none |
| tabs | ″ | fixed pc: `cli.rs:171` (`default_value = "pc"`, which the 36-site grep misses), `mcp/params.rs:57` | fixed Standard: `cli.rs:168`, `mcp/server.rs:176` |
| store characters | ″ | every realm unless named (`cli.rs:780`, `params.rs:96`) | every league unless named (`cli.rs:784`) |
| price status | ″ | pc: `run.rs:387`, `server.rs:536` | Standard: `cli.rs:508`, `run.rs:377`, `server.rs:546` |
| price show | ″ | inferred from where the facts hold the target (`plan/src/pricing.rs:338-366`); refused when several places (`several_places`) | ″ |
| price list | ″ | the address's own realm, else pc (`pricing.rs:418-430`) | the named target's league, else Standard; refused when several (`pricing.rs:318-336`) |
| price set/clear | ″ | the target's own (`PriceTarget`), no league by ruling C67 | none |
| policy show/set | ″ | the value names realms; no default | ″ |
| refresh plan/selection | ″ | pc: `run.rs:327`, `run.rs:363`, `plan_cmd.rs:422`, `server.rs:500`, `web/mod.rs:3808` | Standard: `run.rs:330`, `run.rs:366`, `plan_cmd.rs:423`, `server.rs:500`, `web/mod.rs:3814` |
| refresh apply | ″ | the plan envelope's; a flag only asserts (`run.rs:340`) | ″ |
| shop render | ″ | pc: `run.rs:467`, `server.rs:635` | Standard: `cli.rs:724`, `server.rs:645` |
| saved | ″; the MCP create narrows over every known account (`params.rs:63`) | optional, `all` allowed (`cli.rs:692`) | a term |
| jobs stashes/stash | ″ | pc: `cli.rs:206`, `cli.rs:222` | Standard: `cli.rs:202`, `cli.rs:218`; the **daemon** also defaults Standard: `daemon/routes.rs:68`, `routes.rs:80`, `kinds.rs:553`, `kinds.rs:607` |
| web | `web.served()` | `realm_of` falls back to pc (`web/mod.rs:1682`); `editors` starts at pc (`web/mod.rs:983`) | Standard: `web/mod.rs:2288`, `3052`, `3713`, `3814` |

## 2. Types that carry part of this today

There are about 12.

- **Realm, three types:**
  - `protocol::Realm` (`protocol/src/realm.rs:75`)
  - `search::Realm` (All or One(String), `search/src/corpus.rs:125`). Search doesn't link the protocol crate (`search/Cargo.toml:15-16`).
  - `store::RealmScope` (`store/src/corpus.rs:144`). Most store APIs also take the realm as a plain string.
- **Scope:** `search::answer::Scope {account, realm, membership}` (`answer.rs:435`) and `ScopeOut` (`answer.rs:1685`).
- **Membership, two types:** in the store (`store/src/corpus.rs:155`) and in search (`answer.rs:453`).
- **Places with no league:** `PriceTarget` (`plan/src/price.rs:109`) mirrors `FactAddress` (`store/src/lib.rs:2659`).
- **Selections:** `Selection` and `PlanSelection` (`plan/src/lib.rs:573`, `1278`) hold bare ids. `RefreshPlan` (`lib.rs:1336`) carries the realm and league.
- **Already-resolved values:** `Account` (`verbs/src/frontend.rs:407`); `Several` (`pricing.rs:383`); `LeaguesHeld` (`verbs/src/leagues.rs:113`) and `LeagueHeld` (`store/src/lib.rs:876`).

## 3. Where the hypothesis breaks

1. **The account part already exists.** `Account` is made in exactly one place (`frontend.rs:279`). Each account's facts are its own file, and a facts file holding two identities is refused (`store/src/snapshot.rs` `account_identity`). Putting the account into `Scope` would only rename what's there.
2. **Spanning realms runs into C96, an owner ruling.** C96 says a query naming no realm over a store with several "is an error that lists them". So "reads with nothing named span everything held" needs a ruling first; it isn't agent-facing design. C58 (`decisions/store.md:27`) also says realm defaults "to pc only at the decode boundary". Every frontend pc default already goes against an existing ruling. Enforcing C58 is a stronger argument than a new type.
3. **League is a term in search, not a scope (C96).** A league set inside `Scope` would give search two places to say league.
4. **Resolution happens inside the read.** Search picks "the only realm held" from the corpus header inside the read transaction (`corpus.rs:647-660`; the comment there says the error "costs no read"). The refresh and pricing snapshots are also read in one transaction (`snapshot.rs:280-296`). A `Scope` resolved before the read would be a second read and open a gap between the two (inference). The real shape is two values: what the request named (plain options, which a frontend can build) and how it was resolved, which the read returns.
5. **Writes and jobs need exactly one realm and league.** `RefreshPlan` is one realm and one league by schema (`lib.rs:1355-1357`), and its wire form refuses an action in another league. Spanning a refresh would mean more GGG requests, which touches the invariants. These verbs should take "the only one held, else refuse", never span.
6. **Place ids without a league are ruled (C67).** "An id is only ever looked up inside a scope" contradicts it. Today `league_of` guards it by refusing a target held in several leagues (`pricing.rs:331-335`).
7. **The daemon can't take a verbs value.** C1 limits what the daemon links. Its Standard defaults stay unless the protocol's kind table makes league required: `required: &[]` at `daemon/queue.rs:1474-1484`.
8. **"Every verb takes it" is too wide.** jobs, policy, price set/clear, saved and store status need an account only, or a target.

## 4. The 36 default sites

| What happens | Sites |
|---|---|
| Stay (decode boundary, C58) | `protocol/src/realm.rs:130`, `daemon/src/daemon/kinds.rs:606` |
| Go once nothing uses them | the constant `pricing.rs:297`, its re-export `plan/src/lib.rs:235`, the doc at `pricing.rs:344` |
| Different concept, keep | `pricing.rs:354`: where a character with no listed league is held |
| Reads that can become "everything held" or inferred (about 14) | tabs; price status; `league_of` (`pricing.rs:325`, `331`); `realm_of` (`pricing.rs:430`); web `983`, `1682`, `2288`, `3052`; `mcp/params.rs:57` |
| Become "the only one held, else refuse" (about 16) | refresh (`run.rs`, `plan_cmd.rs`, `server.rs:500`, web `3814`); web list jobs `3713`; stashes/stash; shop render |

Not in the 36: five clap `default_value = "pc"` lines in `cli.rs` and four daemon Standard literals (`routes.rs:68`, `routes.rs:80`, `kinds.rs:553`, `kinds.rs:607`).

## 5. Smallest inputs that change, and where spanning costs

- **A store holding only a challenge league:** `acq tabs`, `acq price` and `acq shop render` answer that league instead of an empty Standard. `acq refresh --plan` plans that league instead of compiling Standard.
- **A store with two realms:** `acq price` stops silently reading pc.
- **Cost of spanning price status:** one `pricing_snapshot` per realm and league, each re-reading every buyout row (`snapshot.rs:325-329`), plus a `leagues_of` call per target.
- **Wrong if spanned:** shop render (its pages are per league, inference) and refresh (one envelope per realm and league).

## 6. Can a tab id still be looked up outside its league?

- **Plans: no, not as a wrong authorization.** `select_tabs` only walks the tabs read for the plan's realm and league (`snapshot.rs:283`, `lib.rs:2017-2056`), and unknown ids are reported, never turned into actions (`lib.rs:2057-2063`). The web posts the ticked ids beside a league field. If that field is missing it falls back to Standard (`web/mod.rs:3814`), so the result is "unknown tabs": a silently wrong answer, not a wrong write.
- **Still possible:** a raw `stash` job with no league goes to `/stash/Standard/<id>` (`daemon/routes.rs:76-87`).
- **By design:** `price set tab/pc/<id>` has no league (C67).

## 7. Steps

- **First step that pays alone:** one resolver in verbs, beside `league_of`: "named, else the only one held, else refuse with the held list". That is search's realm rule applied to league. Use it for tabs, price status, shop render, refresh and the web, and delete the Standard fallbacks.
- **Mechanical:** the roughly 30 `unwrap_or` sites, the clap defaults, the MCP parameter text, regenerating the goldens.
- **Needs judgment, mostly the owner's:**
  - Realm spanning versus C96.
  - What "held" means for a refresh: the leagues the policy names but the store hasn't fetched (the skeptic's point) belong in it.
  - Making league required in the protocol's kind table.
  - One realm type, which means the store linking the protocol crate (the portfolio's page 9).
