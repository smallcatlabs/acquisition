# Phase 0: ground truth

What the orchestrator measured and met before briefing anyone, at
`94514399`. The scouts ran nothing; these numbers were run. The store
is a copy of the owner's, made once into the session's scratch on his
word of 2026-11-24 ("yes, you may measure my data and inspect it"), read
with `ACQ_NO_SPAWN=1` and by `sqlite3 'file:…?mode=ro&immutable=1'`.
The original was never opened. Machine: Apple M4, nothing else running.

## The store as it stands

| What | Number | Command |
| --- | --- | --- |
| Live items | 44,188 (pc 44,090, poe2 98), 223 removed | `select realm, league, count(*) from items where removed_at is null group by 1,2` |
| Verbatim item JSON | 48.6 MB | `select sum(length(json)) from items` |
| Facts file | 99.6 MB | `ls -l GERWARIC_7694.db` |
| Responses, events, tabs | 4,946 / 50,847 / 3,606 (2,635 map substashes) | `select count(*)` of each |
| `daemon.db` | 69.6 MB, one table `jobs`, 4,003 rows | `ls -l`, `.schema` |
| Of that, done `stash` outcomes | 60.0 MB in 3,498 rows | `select kind, state, count(*), sum(length(outcome))/1024 from jobs group by 1,2` |
| Accounts on disk | 3; two hold 90 KB and 131 KB of facts and no intent | `accounts.json`, `ls -l` |
| Prices set | 17, one sitting in September, all by the CLI | `select * from annotations` of the account's file |
| Saved searches | 4 (three by the web UI, one by the MCP) | `select * from annotations` of `user.db` |
| Top-level jobs | 8 applies and 4 listings in four days, none in the five since | `select date(submitted_at,'unixepoch'), kind, count(*) from jobs where parent is null group by 1,2` |

Two things follow at once. The job ledger holds every fetched stash
body a second time: 60 MB beside the 48.6 MB the facts store holds of
the same bodies, kept seven days. And the tool's real use so far is
refresh and search; pricing and the shop have been used once.

## The read path, release build

Three runs each, `/usr/bin/time -p ./target/release/acq --account
'GERWARIC#7694' …`:

| Verb | Seconds |
| --- | --- |
| `search --realm pc ''` | 0.86, 0.86, 0.87 |
| `search --realm pc 'pseudo.total_life>=90'` | 0.88, 0.89, 0.89 |
| the same, `--json` (18.2 KB for ten rows) | 0.88, 0.89, 0.89 |
| `show <id>` | 0.26, 0.26, 0.26 |
| `tabs --realm pc --league Standard` | 0.05 |
| `store status` | 0.04 |

The last recorded number was 0.44 s at 22.7k items (`search/M3-LEDGER.md`,
October). The store has doubled and so has the time: every CLI search
derives the whole corpus from its JSON again.

## Sitting at the tool

The orchestrator asked the plainest question a player has, "which of my
items have 90 or more life", knowing only `acq --help`:

1. `acq store status`: refused. Three accounts are known; name one.
2. `acq --account … search 'life >= 90'`: refused. No whitespace
   around an operator.
3. `search 'life>=90'`: refused. No such field; near it `level`,
   `links`, `pseudo.life_regen`.
4. `search 'pseudo.life>=90'`: refused. No such computed value; near
   it `pseudo.life_regen`, `pseudo.life_regen_pct`,
   `pseudo.phys_attack_life_leech`. The one wanted,
   `pseudo.total_life`, was not offered.
5. `search 'pseudo.total_life>=90'`: refused. The store holds two
   realms; name one.
6. `search --realm pc 'pseudo.total_life>=90'`: 586 matches, answered
   well.

Each refusal is right by its own rule and says what to do. Together
they are five round trips before a first answer, and the full
description of the language is 34.6 KB (`acq search --describe | wc
-c`). The answer itself is the best thing met: it says its scope, what
was never fetched, what matched, failed and lacked, and how to get each
count's members. It ends in a basis line of thirteen version
identifiers.

`acq --help` is about 11 KB. It lists thirty verbs, and beside the
plan path stand the raw endpoint verbs (`profile`, `characters`,
`character`, `stashes`, `stash`), `submit`, `demo`, `dash` and
`set-priority`.

## What this changed in the plan

- The plan's pattern 2 names the realm and league *defaults* as the
  next convenience to remove. From the seat the cost is on the other
  side: the refusals that replaced a default. The question is not
  defaults against refusals but why a scope is asked for piece by piece
  and never shown whole.
- Pattern 6 says nothing measures the calls an agent needs. One
  sitting measured six for the plainest question.
- The persisted queue (track D5) is not only a design question. On the
  owner's disk it is 41% of the store's bytes and a second copy of the
  facts.

## Corrected later

- The job rows span 2026-11-03 to 11-20 with three active days, and the
  ledger prunes only when a daemon starts (`queue.rs:326`), so "kept
  seven days" understates it.
- "A second copy of the facts" is too strong. A stored response has its
  item arrays removed and `items` keeps one current row per item, so a
  done job's payload is the only place that response survives as it was
  (`dissent/theses-6-10.md`, thesis 6).
- `acq --help` is 13,250 bytes.
