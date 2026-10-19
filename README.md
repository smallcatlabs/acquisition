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
is a pre-release, so GitHub does not mark it latest.

| Platform | Archive |
| --- | --- |
| macOS, Apple silicon | `acquisition-1.0.0-beta.1-aarch64-apple-darwin.tar.gz` |
| macOS, Intel | `acquisition-1.0.0-beta.1-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 | `acquisition-1.0.0-beta.1-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `acquisition-1.0.0-beta.1-aarch64-unknown-linux-gnu.tar.gz` |
| Windows x86_64 (and Windows on ARM, emulated) | `acquisition-1.0.0-beta.1-x86_64-pc-windows-msvc.zip` |

On macOS and Linux, fetch and unpack it with your platform's name:

```sh
curl -L https://github.com/smallcatlabs/acquisition/releases/download/v1.0.0-beta.1/acquisition-1.0.0-beta.1-aarch64-apple-darwin.tar.gz | tar -xz
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
- **Windows**: download the `.zip` from the release page and extract
  it. The binaries are not signed; on the machine it was tried on,
  Windows ran them with no SmartScreen prompt, but yours may show one.
  Add the directory to your `PATH`; do not symlink `acq`.

On every platform, run `acq` at your desktop, not over ssh: over ssh
the keyring is out of reach, and on Windows the daemon ends with the
session. To build from source instead, `cargo build --release` puts the three
binaries in `target/release`.

## Log in

```sh
acq auth   # opens GGG's login page in your browser; --no-browser prints the link
acq auth status   # who is logged in, and whether the keyring holds the login
```

Logging in is a person's act at a terminal: `acq` refuses to start a
daemon against GGG without one. GGG is the provider unless
`ACQ_PROVIDER=mock` is set; with it, a built-in stand-in serves
made-up stashes and a login page that takes any name, for trying the
tool or an agent with no account. Its data is kept apart from the real.
Without a keyring, a login lasts as long as the daemon that holds it.

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

It shares the store and the daemon with `acq`, and its tools do what
the CLI does: search the store (`search`, `describe`, `show`,
`tabs`), price items and render the shop (`price_status`,
`price_set`, `shop_render` and four more), keep a league fresh
through a plan the agent reads first (`refresh_plan`, `apply_plan`),
and run jobs (`submit_job`). Each describes itself to the agent;
`MCP-REFERENCE.md` lists all of them.

The store and pricing tools need no daemon. The tools that fetch do,
and against GGG `acq-mcp` never starts one: it uses the daemon you
started, which exits after a minute with nothing to do. For a longer
session, start it with a longer idle time while none is running:

```sh
ACQ_IDLE_SHUTDOWN=3600 acq auth check   # a daemon that idles out after an hour, your login back from the keyring
ACQ_IDLE_SHUTDOWN=3600 acq auth   # the same without a keyring, where the login went with the last daemon
```

In PowerShell, set `$env:ACQ_IDLE_SHUTDOWN = 3600` first.

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
acq refresh --tabs a,b,c | --all [--deep]   # ad-hoc refresh of selected tabs
acq submit <kind> [--params JSON] [--detach]   # submit a job by kind
acq demo   # mock only: a burst of jobs with the limiter's ETAs
acq jobs [--watch | --all] | status <id> | result <id> [--wait]   # open jobs and the latest finished; one job; its payload, from disk if no daemon runs
acq cancel <id> | set-priority <id> <n>   # cancel a subtree; change priority

# keeping a league fresh: a policy, then a plan you read before it runs
acq policy [show]   # sync policy and revision
acq policy set '<json>'|-|@FILE [--if-revision N]   # {"version":3,"realms":{"pc":{"leagues":{…}}}}
acq refresh --plan [--realm R] [--league L] [--expand] [--json]   # compile offline; a running daemon adds its quote
acq refresh --apply[=plan.json] [--max-requests N]   # fetch exactly the plan; ends on what a new plan would still fetch

# pricing and the shop: offline, no daemon
acq reference currency [WORD] [--expand]   # the currency table
acq price status | list | show <target>   # listing state of a league; one target
acq price set <target> <type> [<amount> <currency>] [--if-revision N]   # set a price, with undo
acq price clear <target> [--if-revision N]   # remove a price, with undo
acq shop render [--size N] [--template FILE] [--page N] [--expand]   # forum pages to paste, omissions counted

# the store: no daemon, no network
acq tabs [--league L] [--realm R] [--type T] [--never-fetched] [--expand]   # counts, then the tabs; a large league by type unless --expand
acq store characters [--realm R] [--league L]   # character locations, ages and live items
acq search [--realm R] ['<query>'] [--sort V [--desc]] [--limit N] [--routes]   # item search; a store of two realms needs --realm (pc, poe2, all)
acq search ['<query>'] --count KEY,… | --cross KEY,KEY [--sum V]   # the matches counted by key; line[:text,…] is the vocabulary
acq search --describe [name,…|all]   # the query language: an index, or entries whole
acq search --trade <URL|ID> | --trade-file <FILE|->   # a trade site search (PoE1) translated to a query, no network
acq show <id> [--body]   # one item as the search derives it; the stored body on request
acq store status | events [--hours N] | refused [id]   # path and counts; ingest events; malformed bodies
acq store import <snapshot.json> | rebuild   # replay a snapshot from the retired `acq pull`; re-extract columns

acq daemon status | stop | reset-tripwire   # debugging: inspect, stop, reset rails
acq version [--json]   # build identity and sibling acqd
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
  well: `price set` and `policy set` write intent beside the facts,
  and `store import` replays a snapshot of facts.
- **Standard unless you name a league.** `stashes`, `stash`,
  `refresh --plan`, `tabs`, `price status` and `shop render` read or
  fetch Standard without `--league` (a saved plan carries its own); `price show` reads the league the facts
  hold the target in, `store characters` every league. `acq leagues`
  lists yours.
- **Your data stays on your machine**, under
  `~/Library/Application Support/com.smallcatlabs.acq` on macOS,
  `~/.local/share/acq` on Linux and `%APPDATA%\smallcatlabs\acq\data`
  on Windows; `acq store status` prints the path.
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
| `ACQ_KEYRING_ROUNDTRIP=1` | off | opt into the test against the OS keyring (CI uses gnome-keyring) | [auth.rs][auth] (tests) |
| `ACQ_CASES=full` | off | the generated search properties at full counts, as CI runs them; unset, about a quarter, fresh each run; a failure prints its seed (`PROPTEST_RNG_SEED`) | [generated.rs](crates/acquisition-search/tests/common/generated.rs) (tests) |
| `ACQ_CODESIGN_IDENTITY=<sha1\|name>` | unset | macOS shell-profile identity for signing the debug daemon after building | [sign-acqd.sh](tools/sign-acqd.sh) |
| `ACQ_NO_SPAWN=1` | off | never start or replace a daemon; for cron, ssh and headless shells | [client.rs][client]; keyring context: [auth.rs][auth] |
| `ACQ_IDLE_SHUTDOWN=<s>` | 60 | idle exit, held while the limiter's history is live | [daemon.rs][daemon] |
| `ACQ_JOB_RETENTION_DAYS`, `ACQ_FAILED_JOB_RETENTION_DAYS` | 7, 30 | finished-job retention for `acq result`; invalid values keep defaults | [daemon.rs][daemon] |
| `ACQ_TRIPWIRE=1` | off | rail 1: halt on 429/401/403/503 until reset, persisted | [rails.rs][rails] |
| `ACQ_MAX_SENDS=<n>` | off | rail 5: halt after `n` real sends this lifetime, not persisted | [rails.rs][rails] |
| `ACQ_JOURNAL=<path>` | `<log dir>/sends.jsonl`, bounded; `0` disables | rail 4: one JSON line per send, never a token or body | [rails.rs][rails] |
| `ACQ_MOCK_DEGRADED_HEAD=1` | off | the mock answers HEAD as GGG did in a December 2023 regression | [mockggg.rs][mock] |

[provider]: crates/acquisition-protocol/src/provider.rs
[cli]: crates/acquisition-cli/src/main.rs
[world]: crates/acquisition-store/src/world.rs
[auth]: crates/acquisition-daemon/src/auth.rs
[client]: crates/acquisition-client/src/client.rs
[daemon]: crates/acquisition-daemon/src/daemon.rs
[rails]: crates/acquisition-daemon/src/rails.rs
[mock]: crates/acquisition-daemon/src/mockggg.rs

A knob the daemon reads takes effect when it starts: set it on the
command that starts the daemon, or stop the daemon first. The rails are
for test runs against GGG (`LIVE-TESTING.md`, "Rails");
`acq daemon status` prints the paths and the rails' state.

## Known gaps

- **`acq policy set` replaces the whole policy**, every realm of it: a
  policy written for poe2 alone erases the pc one. Read it with
  `acq policy show` and write it back whole (`decisions/plans.md`,
  "Parked").
- **Path of Exile 2 is partial.** GGG offers no PoE2 stash tabs, so
  only characters are fetched, and the search's tables cover PoE1, so a
  PoE2 item's class and totals are undecided.
- **A character fetched with an empty body shows `0` items.** GGG
  sends the same empty arrays for a stripped character and for one that
  carries nothing; `acq store characters` counts them in its footer
  alone ("with empty bodies").
- **The mock is simpler than GGG**: it does not quantize its timing
  buckets, and after a 429 it marks every window of the rule as
  restricted; whether GGG marks only the violated one is not yet
  observed.
- Everything else not built yet is parked, with what would reopen it:
  `CONTEXT.md` and each `decisions/<area>.md`, "Parked".

## What it does not do

It does not say what an item is worth: a price is the one you set, or
your note or tab name in the game. `acq search --trade` translates a
trade search you paste, without fetching it. It does not touch the game
client, and there are no installers, packages or automatic updates.

## Reporting and contributing

A security problem: `SECURITY.md`, which asks for GitHub's private
reporting. A bug or a request: an issue on this repository. A change:
`CONTRIBUTING.md`.

## Licence

Licensed under MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`);
contributors keep their copyright and license their work under the same
terms. The Rust tree was audited against the GPL C++ app before the
declaration: `audits/relicense-2026-10/`.
