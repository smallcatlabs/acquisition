# Author's Note

This is an experimental rewrite of [acquisition-qt](https://github.com/smallcatlabs/acquisition-qt).

This project is changing rapidly, but still incomplete. For example, there's no UI and the documentation is mainly for agents. All you get right now are a CLI, an MCP, and the ability to run trade site searches against your own downloaded stash by copying and pasting search URLs as arguments. It will probably take several months of work to get this into a decent enough shape to seriously compete with the original acquisition, so I plan on maintaining both.

--@gerwaric

---

# Acquisition

Acquisition downloads your Path of Exile stash tabs and characters to
your own machine, where you can search them offline, price what you
want to sell, and render the forum shop pages that list it. This
release is the command line `acq`, the MCP server `acq-mcp` that lets
an AI agent do the same, and the daemon `acqd` that sends every request
to Grinding Gear Games under their rate limits. It runs on macOS, Linux
and Windows, for Path of Exile 1; Path of Exile 2 follows at its
launch. The desktop app, [acquisition-qt](https://github.com/smallcatlabs/acquisition-qt),
keeps working beside it, and a GUI for this build comes later.

You log in on GGG's own page. The tool keeps one refresh token in your
operating system's keyring and sends it nowhere but GGG. It never reads
the trade site and never posts to the forum: shop pages print to your
terminal and you paste them.

Development starts at `AGENTS.md`.

## Install

Each [release](https://github.com/smallcatlabs/acquisition/releases)
carries an archive per platform, holding `acq`, `acqd`, `acq-mcp`,
the licences and the third-party notices, and `SHA256SUMS`. This one
is a pre-release, so GitHub does not mark it latest. Every other file
this page names is in the repository, not the archive.

| Platform | Archive |
| --- | --- |
| macOS, Apple silicon | `acquisition-1.0.0-alpha.1-aarch64-apple-darwin.tar.gz` |
| macOS, Intel | `acquisition-1.0.0-alpha.1-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 | `acquisition-1.0.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `acquisition-1.0.0-alpha.1-aarch64-unknown-linux-gnu.tar.gz` |
| Windows x86_64 (and Windows on ARM, emulated) | `acquisition-1.0.0-alpha.1-x86_64-pc-windows-msvc.zip` |

On macOS and Linux, fetch it with your platform's name, check it and
unpack it:

```sh
curl -LO https://github.com/smallcatlabs/acquisition/releases/download/v1.0.0-alpha.1/acquisition-1.0.0-alpha.1-aarch64-apple-darwin.tar.gz
shasum -a 256 acquisition-1.0.0-alpha.1-aarch64-apple-darwin.tar.gz   # the digest SHA256SUMS lists for it
tar -xzf acquisition-1.0.0-alpha.1-aarch64-apple-darwin.tar.gz
```

Keep the three binaries together: `acq` starts the `acqd` beside it.
Put the directory on your `PATH`, or symlink `acq` into a directory that
is; the link is followed.

- **macOS**: the binaries are signed with a Developer ID and not
  notarized. Fetched with `curl` they run. A browser download is
  quarantined, and macOS refuses an unnotarized binary that is;
  `xattr -dr com.apple.quarantine <directory>` clears it.
- **Linux**: built on Ubuntu 22.04, so glibc 2.35 or newer. The login
  is kept in the Secret Service (GNOME Keyring, or KWallet), unlocked
  in your desktop session.
- **Windows**: download the `.zip` from the release page, check
  `Get-FileHash` on it against `SHA256SUMS`, and extract it. The
  binaries are not signed, so Windows may show a SmartScreen prompt.
  Add the directory to your `PATH`; do not symlink `acq`.

On every platform, run `acq` at your desktop, not over ssh: over ssh
the keyring is out of reach, and on Windows the daemon ends with the
session. From source: `cargo build --release`.

## Log in

```sh
acq auth   # opens GGG's login page in your browser; --no-browser prints the link
acq auth status   # who is logged in, and whether the keyring holds the login
```

Logging in is a person's act at a terminal: `acq` refuses to start a
daemon against GGG without one. GGG is the provider unless
`ACQ_PROVIDER=mock` is set; with it, a built-in stand-in serves
made-up stashes and a login page that takes any name, for trying the
tool or an agent with no account. Without a keyring, a login lasts as
long as the daemon that holds it. A second login makes every verb
need `--account`.

## Your agent

`acq-mcp` is an MCP server over stdio. In Claude Code's `.mcp.json` or
Claude Desktop's configuration, with the path where you unpacked it:

```json
{
  "mcpServers": {
    "acquisition": { "command": "/path/to/acquisition/acq-mcp" }
  }
}
```

That is your own account. The server takes its knobs through `env`:
`"env": { "ACQ_PROVIDER": "mock" }` gives the agent the stand-in's
made-up stashes instead, after `ACQ_PROVIDER=mock acq auth`, and
`ACQ_STORE_DIR` there must match your `acq`'s.

It shares the store and the daemon with `acq`, and its 25 tools do
what the CLI does: read the store (`search`, `describe`, `show`,
`tabs`, `characters`, `item_events`, `store_status`, `accounts`),
price items and render the shop (`price_status`, `price_list`,
`price_show`, `price_set`, `price_clear`, `shop_render`,
`reference_currency`), keep a league fresh (`sync_policy`,
`set_sync_policy`, `refresh_plan`, `apply_plan`), and run jobs
(`submit_job`, `list_jobs`, `job_status`, `job_result`, `cancel_job`,
`daemon_status`).

The store and pricing tools need no daemon. The tools that fetch do,
and against GGG `acq-mcp` never starts one: it uses the daemon you
started, which exits an hour after its last connection or job submitted
once no login is under way, no job is sending or waiting to send, and
the limiter's recent history is gone; a rails halt or a failed queue
write parks the queue on disk, and nothing waiting then holds it. When
none is running, start one:

```sh
acq auth check   # a daemon with your login back from the keyring
acq auth   # the same without a keyring
acq daemon status   # the daemon and its idle exit
```

## The tour

One line per verb. A verb's flags, defaults and meaning are its
`--help`; `CLI-REFERENCE.md` holds every one.

```sh
acq <verb> --help   # every verb's reference

# the session
acq auth [--no-browser]   # log in on GGG's page
acq auth status | check | logout   # live sessions, or with no daemon what one would restore; token check; logout
acq accounts   # accounts this machine has logged into (no daemon)

# jobs: fetched through the daemon, which starts on demand; a route's first use probes its limits
acq profile | leagues   # profile; account leagues
acq characters [--realm R]   # character list
acq character <name> [--realm R]   # equipment and inventory
acq stashes [--league L] [--realm R]   # tab list (PoE1 only)
acq stash <id> [--sub <id>] [--deep]   # one tab, optionally with substashes
acq refresh --tabs a,b,c | --all [--deep] | --characters x,y   # a selection by id: compiled into a plan, printed, applied at once
acq submit <kind> [--params JSON] [--detach]   # submit a job by kind
acq demo   # mock only: a burst of jobs with the limiter's ETAs
acq jobs [--watch | --all] | status <id> | result <id> [--wait]   # open jobs and the latest finished; one job; its payload, from disk if no daemon runs
acq cancel <id> | set-priority <id> <n>   # cancel a subtree; change priority

# keeping a league fresh: a policy, then a plan you read before it runs
acq policy [show]   # sync policy and revision
acq policy set '<json>'|-|@FILE [--if-revision N] [--replace]   # {"version":3,"realms":{"pc":{"leagues":{…}}}}: a realm is written whole, so name every league it keeps; realms not named are kept
acq refresh --plan [--realm R] [--league L] [--expand] [--json]   # compile offline; a running daemon adds its quote
acq refresh --apply[=plan.json] [--max-requests N]   # fetch exactly the plan; ends on what a new plan would still fetch

# pricing and the shop: offline, no daemon
acq reference currency [WORD] [--expand]   # the currency table
acq price status | list | show <target>   # listing state of a league; one target
acq price set <target> <type> [<amount> <currency>] [--if-revision N]   # set a price, with undo
acq price clear <target> [--if-revision N]   # remove a price, with undo
acq shop render [--size N] [--template FILE] [--page N] [--expand]   # forum pages to paste, omissions counted

# the store: no daemon, no network
acq tabs [--league L] [--realm R] [--type T] [--never-fetched] [--limit N] [--expand]   # counts, then the ten tabs with the most items; --limit more, --expand all
acq store characters [--realm R] [--league L]   # character locations, ages and live items
acq search [--realm R] ['<query>'] [--sort V [--desc]] [--limit N] [--routes]   # item search; a store of two realms needs --realm (pc, poe2, all)
acq search ['<query>'] --count KEY,… | --cross KEY,KEY [--sum V]   # the matches counted by key; line[:text,…] is the vocabulary
acq search --describe [name,…|all]   # the query language: an index, or entries whole
acq search --trade <URL|ID> | --trade-file <FILE|->   # a trade site search (PoE1) translated to a query, no network
acq show <id> [--body]   # one item as the search derives it; the stored body on request
acq web [--no-browser]   # the search as a page shaped like the trade site's, with item art, and the refresh loop to tick, plan and apply; loopback only, until Ctrl-C
acq store status | events [--hours N] | refused [id]   # path and counts; ingest events; malformed bodies
acq store import <snapshot.json> | rebuild   # replay a snapshot from the retired `acq pull`; re-extract columns

acq daemon status | stop | reset-tripwire   # inspect; stop (a login not in the keyring goes too); reset rails
acq version [--json]   # build identity and sibling acqd
acq diagnose [--lines N]   # what a bug report needs, account names, paths and tokens replaced: paste it into the issue
```

Every command takes `--json`, and it is total: a failure is
`{"error":…}` on stdout with exit 1, a refused command line the same
with `"kind":"usage"` and exit 2. Every command takes
`--account <username|name|uuid>` (or `ACQ_ACCOUNT`) too. Without it, a
verb that reads the store takes the one account known on disk and a job
verb the one logged in; where there are several, it refuses and names
them.

## Before the first command

- **The rate limit is the clock.** Every request waits its turn under
  GGG's limits, and on tabs the limit seen so far is thirty every five
  minutes, so a first sync of a few hundred tabs takes most of an hour.
  A hold can last five minutes; `acq jobs` shows each job's ETA.
- **What needs the daemon.** A job verb sends through it, prints
  GGG's JSON (progress on stderr) and writes what it fetched to the
  store; `auth`, `jobs`, `status`, `cancel` and `daemon` talk to it
  too. The store, search, pricing and shop verbs read the store without
  one and see only what was fetched. Your own writes are offline as
  well: `price set` and `policy set` write your prices and policy
  beside the facts, and `store import` replays a snapshot of facts.
- **Standard unless you name a league.** `stashes`, `stash`,
  `refresh --plan`, `refresh --apply`, `refresh --tabs`, `tabs`,
  `price status` and `shop render` read or fetch Standard without
  `--league` (a saved plan carries its own); `price show` reads the
  league the facts hold the target in, `store characters` every
  league. `acq leagues` lists yours.
- **A new league.** `acq leagues` names it the day it opens; add it to
  the policy by writing the realm back with every league it keeps (a
  policy naming only the new league drops Standard); then `acq refresh
  --plan --league <name>` and `--apply --league <name>` until the plan
  says nothing to do; the first plan is the listings, one or two (the
  tab list, and the character list when it is stale), and the plans
  after it fetch every tab the policy covers, so read each plan's work
  and its ceiling before applying.
- **Your data stays on your machine**, under
  `~/Library/Application Support/com.smallcatlabs.acq` on macOS,
  `~/.local/share/acq` on Linux and `%APPDATA%\smallcatlabs\acq\data`
  on Windows, one database per account in `store/ggg` (`store/mock`
  for the stand-in); `acq store status` prints its path. The daemon's
  log is under `~/Library/Logs/acq`, `~/.local/state/acq/log` and
  `%LOCALAPPDATA%\smallcatlabs\acq\data\logs`; `acq daemon status`
  prints it.
- **After an update**, the next job command replaces the older build's
  daemon. `jobs`, `status` and `daemon status` only observe: they
  report a daemon and never start or replace one.

## Knobs

| Knob | Default | Effect | Read in |
| --- | --- | --- | --- |
| `ACQ_PROVIDER=mock` | `ggg` | the built-in stand-in for GGG; a real daemon starts only from a terminal; the retired `ACQ_GGG` is refused | [provider.rs][provider] |
| `ACQ_ACCOUNT=<sel>` | the sole account | the environment's `--account`, matched exactly | [main.rs][cli] |
| `ACQ_STORE_DIR=<dir>` | the platform data dir | the store, and the daemon that serves it: one daemon per directory | [world.rs][world] |
| `ACQ_LOG_DIR=<dir>` | the platform log dir | bounded daemon log and default journal, by store and provider | [world.rs][world] |
| `ACQ_NO_KEYRING=1` | off | logins in memory only, never plaintext on disk | [auth.rs][auth] |
| `ACQ_NO_SPAWN=1` | off | never start or replace a daemon; for cron, ssh and headless shells | [client.rs][client]; keyring context: [auth.rs][auth] |
| `ACQ_IDLE_SHUTDOWN=<s>` | 3600 | idle exit, this many seconds after the last connection or job submitted once no login is under way, no job is sending or waiting to send, and the limiter's recent history is gone; a rails halt or a failed queue write parks the queue on disk, and nothing waiting then holds it | [start.rs][daemon] |
| `ACQ_HELLO_WITHIN=<s>` | 5 | how long a daemon has to answer the handshake before it is reported unresponsive | [client.rs][client] |
| `ACQ_STARTUP_WITHIN=<s>` | 5 | how long a daemon a command started has to bind its socket before the start is reported failed | [client.rs][client] |
| `ACQ_ANNOTATION_WAIT=<s>` | 5 | how long a write waits for another writer's lock on the annotation file before it is refused as busy | [annotations.rs][annotations] |
| `ACQ_JOB_RETENTION_DAYS`, `ACQ_FAILED_JOB_RETENTION_DAYS` | 7, 30 | finished-job retention for `acq result`; invalid values keep defaults | [start.rs][daemon] |
| `ACQ_TRIPWIRE=1` | off | rail 1: halt on 429/401/403/503 until reset, persisted | [rails.rs][rails] |
| `ACQ_MAX_SENDS=<n>` | off | rail 5: halt after `n` real sends this lifetime, not persisted | [rails.rs][rails] |
| `ACQ_JOURNAL=<path>` | `<log dir>/sends.jsonl`, bounded; `0` disables | rail 4: one JSON line per send, never a token or body | [rails.rs][rails] |
| `ACQ_MOCK_DEGRADED_HEAD=1` | off | the mock answers HEAD as GGG did in a December 2023 regression | [mockggg.rs][mock] |

[provider]: crates/acquisition-protocol/src/provider.rs
[cli]: crates/acquisition-cli/src/main.rs
[world]: crates/acquisition-store/src/world.rs
[annotations]: crates/acquisition-store/src/annotations.rs
[auth]: crates/acquisition-daemon/src/auth.rs
[client]: crates/acquisition-client/src/client.rs
[daemon]: crates/acquisition-daemon/src/daemon/start.rs
[rails]: crates/acquisition-daemon/src/rails.rs
[mock]: crates/acquisition-daemon/src/mockggg.rs

A knob the daemon reads takes effect when it starts: set it on the
command that starts the daemon, or stop the daemon first. The rails are
for test runs against GGG; `acq daemon status` prints their state.

## Known gaps

- **Path of Exile 2 is partial.** GGG offers no PoE2 stash tabs, so
  only characters are fetched, and the search's tables cover PoE1, so a
  PoE2 item's class and totals are undecided.
- **A character fetched with an empty body shows `0` items.** GGG
  sends the same empty arrays for a stripped character and for one that
  carries nothing; `acq store characters` counts them in its footer
  alone ("with empty bodies").
- Everything else not built yet is a GitHub issue, labelled with its
  area (`area:*`) and what would fire it (`trigger:*`); what the project
  declines until a trigger is a "Not doing" line at the foot of the
  area's `decisions/<area>.md`.

## What it does not do

It does not say what an item is worth: a price is the one you set, or
your note or tab name in a public tab in the game; a note in a private
tab is no price. You post the shop pages in your league's shop
subforum on pathofexile.com; the trade site then lists the linked
items, a private tab's and a character's too, at their prices. Leave
them in place: a moved item may lose its link. It does not touch the
game client, and there are no installers, packages or automatic
updates.

## Reporting and contributing

A security problem: `SECURITY.md`, which asks for GitHub's private
reporting. A bug or a request: an issue on this repository. For a bug,
paste the output of `acq diagnose` into the issue, between two lines of
three backticks: the build, the daemon, the store's counts, the rails
and the ends of the send journal and the daemon log. It replaces your
account names, your paths and anything shaped like a token, and keeps
character, league and tab names; read it before you post. A change:
`CONTRIBUTING.md`.

## Licence

Licensed under MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`);
contributors keep their copyright and license their work under the same
terms. The Rust tree was audited against the GPL C++ app before the
declaration: `audits/relicense-2026-10/`.
