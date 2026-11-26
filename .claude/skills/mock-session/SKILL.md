---
name: mock-session
description: Work against the mock daemon in an isolated session — store (its socket derives from it), keyring — with a scripted login. Use before running acq for anything that is not a live run.
---

# Mock session

Everything here talks to the in-process mock provider; nothing reaches
GGG. Isolation exists so a session never shares a world, its socket, or
the persisted job queue with the owner's real daemon or with another
session — a world is one daemon's (its lock refuses a second, C83), and
its socket is derived from it, so two worlds are two rendezvous and
nothing has to be named by hand.

## Set up

Build before the session, as before any daemon; see
[why build order matters](../session-close/SKILL.md#why-build-order-matters).

```sh
unset ACQ_GGG ACQ_TRIPWIRE ACQ_MAX_SENDS ACQ_IDLE_SHUTDOWN   # a live run may have left these exported; a leftover ACQ_GGG is refused (C88)
export ACQ_PROVIDER=mock                                      # the mock is opted into (C88): without this every command wants the real provider and the owner's store
export ACQ_NO_KEYRING=1                                       # sessions in memory only
export ACQ_STORE_DIR=<scratch>/store                          # the session's world (C83): never the real per-user data directory; its socket derives from it (`acq daemon status` prints it)
export ACQ_LOG_DIR=<scratch>/logs                             # the daemon log and journal, out of ~/Library/Logs
cargo build --workspace && ./target/debug/acq --version       # builds acq and its sibling acqd (C82)
alias acq=./target/debug/acq
acq daemon status                                             # a fresh socket: "daemon is not running" (status never spawns, C10); after the login below it says provider mock
```

## Read work: a seeded world, no daemon

For a read (a search, a tab list, prices, a page), skip the login: one
command writes an owner-shaped world (three accounts, two realms, three
leagues, prices, a sync policy, a saved search, a hostile tab name)
through the store's own ingest, and prints the directory it wrote.

```sh
cargo run -q -p acquisition-verbs --example seeded-world -- <scratch>/store
export ACQ_PROVIDER=mock ACQ_STORE_DIR=<scratch>/store ACQ_NO_SPAWN=1   # no daemon: any contact is an error
acq accounts                                                             # Alice#1234, Bob#9, Carol#77; name one with --account
```

It is for reads: a mock login over it orphans its prices and policy,
and an apply runs over the mock's own world. Its table is
`crates/acquisition-client/tests/harness/seeded.rs`.

## Log in without a browser

```sh
acq auth --no-browser --json > "$ACQ_STORE_DIR.auth.json" &   # writes {"authorize_url": …}; wait until it does
curl -sL "$(sed -n 's/.*"authorize_url":"\([^"]*\)".*/\1/p' "$ACQ_STORE_DIR.auth.json" | sed 's|/authorize?|/approve?|')"   # add &user=NAME for a second account
```

`-L` is required: approve 302-redirects to the daemon's callback, and
without following it the login never completes (the login's own
profile job must land the account uuid). Any username is accepted;
policies count per username, so two accounts are one login apart.

## Open the web UI

`acq web --no-browser &` serves this world's pages on loopback and prints their URL, which a browser opens; `kill %1` (or Ctrl-C in the foreground) stops it.

## Rehearse a rung

`tools/tracer-rung.sh --mock` and `tools/persist-check.sh --mock ...`
run the identical flow the live runs use, exact ceilings included, each
in a scratch world of its own under `runs/mock/` (unset this session's
`ACQ_STORE_DIR`, `ACQ_LOG_DIR` and `ACQ_NO_KEYRING` first: the drivers
refuse them). The tracer driver logs a mock account in and stores a
policy covering every tab and character of the league itself. After
touching it, run `tools/tracer-rung-breakers.sh` too: it stages what a
run must leave behind (the stored policy untouched, a failed apply's
words kept), some seconds of mock daemons that no gate stage runs.

## Tear down

`acq daemon stop`. Mock access tokens live 60 s, so a daemon left up
exercises silent refresh; it idles out on its own after an hour with no
connections and no live jobs (C3), so stop it when the session ends.
