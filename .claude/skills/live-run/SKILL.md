---
name: live-run
description: Run the Rust daemon against the real GGG API — a first contact on a new endpoint, or the refresh loop under the rails. Human-run, from a terminal. Use before any command that talks to the real provider — the default since C88.
---

# Live run

Nothing here talks to GGG from an agent's shell: the real provider is
the default (C88), and a real-mode daemon is started only from a
terminal (the CLI refuses to spawn one without a terminal on stderr,
and a daemon spawned from cron or a background shell would have no
keychain and no session). A live run is the owner's, from a terminal,
under this procedure. The invariants in `AGENTS.md` apply. Every live
run is one row in `RUN-LEDGER.md`, its evidence under gitignored
`runs/`; a fact learned live is a numbered claim in
`docs/design/network-ground-truth.md`. The ladder that proved the
limiter (ten rungs, 2026-08-22 to 2026-08-27: about 1,450 sends, zero
429s, the daemon halts rather than floods) is `HISTORY.md`'s row; what
survives it is the standing rule and the rails below.

## The standing rule: first contact

- **The first live call on a new endpoint runs with the rails on,**
  `ACQ_TRIPWIRE=1 ACQ_MAX_SENDS=3`, from a terminal, on a fresh daemon
  (`acq daemon stop` first). Three, not one: a fresh daemon sends a
  token POST, then the HEAD probe, then the GET, and the ceiling counts
  every method (`rails.rs`,
  `the_ceiling_counts_every_method_and_halts_at_its_number`). Two if
  the daemon already holds a valid access token.
- **Read the journal before anything else.** The probe line must report
  0 hits on its policy; hits > 0 means something else is using this
  account (the C++ app, another CLI, another machine; counters are per
  account, N24): stop and find it. The probe learns the policy before
  the first counted send; a probe that fails or returns no rules closes
  the endpoint for 60 s (N20).
- **Record the observed policy** (name, windows, the state after the
  GET) as one `RUN-LEDGER.md` row and as a ground-truth claim; that row
  is what `acquisition-daemon/src/ratelimit/tests/spec.rs`'s test table cites. The row carries where the
  evidence is and where each lesson landed, never the lesson's text.
- **Post-violation: wait 360 s, then `reset-tripwire`; never
  reset-and-retry.** After any trip write the cause in the ledger, wait
  at least 360 s (the longest observed policy window plus the 60 s
  bucket), and only then `acq daemon reset-tripwire`. A trip is
  evidence to read, not a retry prompt.
- **A failed probe leaves only headers.** A HEAD has no body, so a
  probe's non-2xx is classified from its response headers, which the
  journal, the daemon log and the trip cause all carry. Routes known
  not to accept HEAD skip the probe (`route_probes` in `daemon/routes.rs`) and
  are taught by their first GET (C32).
- **A soak derives its ceiling from cadence times intended duration**
  (200 sends at one per 10 min is 33 h), and from the run's own
  listing, never another run's count.

Every job kind has had first contact (`RUN-LEDGER.md`); a new kind gets
the same treatment under this rule. What the samples found that headers
could not teach is declared route knowledge (C32; N38, N39).

## A new league on a contacted route

A new league is a new string on routes already contacted, so it is no
first contact (C59, an unobserved URL shape is never sent: the shape
is observed): a daemon that has already contacted a route probes it
no further for the new league, while a fresh daemon probes the stash
and character routes with a HEAD on their first use, and its ceiling
counts those probes. The league and profile routes are never probed
(`NO_PROBE_ROUTES` in `daemon/routes.rs`), so `acq leagues` costs one
GET, and a token refresh if the access token has expired, and no HEAD.
Run it first, under a ceiling of that count, and check the name it
lists before any policy names it: a policy naming a league the list
does not hold is noted, never refused, and an apply over a typo sends
a stash list for a league that does not exist (N27, the
invalid-request budget). Write the realm back with every league it
keeps (`policy set` replaces a realm whole), then take each cycle's
ceiling from its plan as the refresh loop's driver derives it. After,
the ledger row, and two claims nothing records yet: the real
`/account/leagues` answer's fields, and what GGG answers a stash list
for a league that does not exist, recorded if a run ever meets it and
never provoked.

## The rails

Knobs and defaults are the README's "Knobs" table; `acq daemon status`
prints their state; each rail's mechanism is `rails.rs`'s doc. The
tripwire (`ACQ_TRIPWIRE=1`: the first landed 429, 401, 403 or 503 halts
every later send until `reset-tripwire`, persisted per provider; queued
jobs wait on disk, C25) and the send ceiling (`ACQ_MAX_SENDS=<n>`: a
halt after n sends this lifetime, not persisted) are for first contacts
and the drivers. Permanent, no knob: the dead-grant stop (C24), the
HTTP timeouts (10 s connect, 60 s request; a send lost in transport is
paced as counted, C19), the send journal (`ACQ_JOURNAL`, the contract
surface, `TESTING-NOTES.md`), the mock-only kinds refused in real mode,
a keyring save failure surfaced in `daemon status` as a stop condition,
and `ACQ_IDLE_SHUTDOWN=<secs>` set per run. The gate admits two
concurrent sends, so one already-dispatched request may land after a
trip: a 2-send row is not a rail failure. Each rail has a deterministic
test against the mock with rails 1 and 5 forced on.

## Before

1. `unset ACQ_GGG ACQ_PROVIDER ACQ_TRIPWIRE ACQ_MAX_SENDS ACQ_IDLE_SHUTDOWN ACQ_STORE_DIR ACQ_LOG_DIR ACQ_NO_KEYRING`:
   the shell you were launched from may still export a previous run's
   rails or a mock session's isolation, and a leftover `ACQ_GGG` is
   refused by every binary (C88).
2. `acq daemon stop`, until it says "daemon is not running". Never
   rebuild under a live daemon.
3. `git status --porcelain` must print nothing (the drivers check the
   rung's own files, `tools/preflight.sh`'s list), then `cargo build
   --workspace` (`acq` and `acqd` beside it, C82), then
   `./target/debug/acq version`. The binaries carry no commit: two
   values say which code ran (C84), the shared-contract revision (a
   digest over the protocol and store crates' manifests and `src`
   trees, the root manifest and the lock, `build.rs`; `acq version
   --json` prints it as `contract`) and the SHA-256 of the `acqd` file,
   which the daemon reports in `hello`, its startup line and the
   journal header. A driver writes both into `provenance.json` before
   any wire phase and holds every journal header to the `acqd` hash; by
   hand, a clean tree, the build, then the run, and the ledger row
   names HEAD. After the build (the session-close skill, "Why build
   order matters"), `tools/sign-acqd.sh` (the daemon's code
   identity, from `ACQ_CODESIGN_IDENTITY` in your shell profile;
   without it macOS Keychain asks twice per login after every rebuild;
   sign after the build, since one that re-links `acqd` writes a fresh
   unsigned one). The drivers do all of this themselves and start
   `acqd` directly.
4. `ls -t runs/ | head` against `RUN-LEDGER.md`: a bundle newer than
   the last row is a run that happened off the record.

## The refresh loop (policy → plan → apply → replan)

```sh
tools/tracer-rung.sh --account <SEL> [--realm R] [--characters all|id,...] <tab1,...|all>
```

The driver sets the rails, derives each cycle's ceiling from the plan
exactly, gates every wire phase on an explicit enter, verifies the
journal (`tools/tracer-verify.py`), copies the evidence to
`runs/<date>-tracer/`, and drafts the ledger row. It refuses a stale
binary, working-tree changes to its own files, leftover env, and a
running daemon. Rehearse first with `--mock` (evidence goes to
`runs/mock/`). Expect holds: none while a cycle stays under 15 stash
GETs, about 15 s before the 16th, about 343 s at most above 30: the
limiter working, not a hang. macOS has no `timeout`. A Ctrl-C at a driver
now reaches its `acqd` (before, the shell ignored it there and the
daemon kept sending): it drains (sends in flight have 15 s to land, then
it exits), and a second Ctrl-C exits at once (C133).

## A first contact on a new endpoint

Fresh daemon, rails on, ceiling 3, or 2 if the daemon already holds a
valid access token:

```sh
ACQ_TRIPWIRE=1 ACQ_MAX_SENDS=3 acq <command>   # the real provider is the default (C88); no flag selects it
```

Read the journal before anything else (`acq daemon status` prints its
path: the world's `sends.jsonl` under the log directory; a driver's
daemons write the run directory's `sends.jsonl` instead, under
`ACQ_JOURNAL`): the probe line must report 0 hits on its policy.

## After

- The row in `RUN-LEDGER.md` (the driver's draft, or one written by
  hand for a first contact): date, tip, result, sends as POST/HEAD/GET,
  violations, the observed policy, the run directory, and where each
  lesson landed, never the lesson's text.
- A new fact about GGG is a numbered ground-truth claim, authored here;
  `acquisition-qt` takes copies, never the reverse.
- The owner's verdict is recorded verbatim from the conversation,
  marked as such.
- `acq daemon stop`. Evidence stays in `runs/` (gitignored); the ledger
  cites the directory.

## If something trips

A tripwire trip is evidence to read, not a retry prompt: the standing
rule's post-violation wait comes first. Never reset-and-retry. Either
halt leaves the queue waiting on disk, and clearing it (`acq daemon
reset-tripwire` after a trip, a respawn after a ceiling) wakes every
job in it: first `acq jobs` (it reads the queue on disk once the
halted daemon has idled out, C45) and `acq cancel` what should not go
out. An interrupted
driver leaves its `apply` parent persisted too: `acq cancel <id>`
before the driver runs again, or the parent's remaining children go out
during the next quote and the driver refuses the run.

Known costs, not stops: every lifetime starts with a token POST (the
access token is never persisted), so a by-hand ceiling of 2 is one POST
and one GET; the first probe of a lifetime queues a few seconds behind
that POST; an unsigned `acqd` is asked twice per login after every
rebuild. A signed one is asked once per keychain item on its first
start under the identity (one prompt per persisted account, "Always
Allow") and never again; the release archive's Developer ID `acqd` is
another identity, asked once per item too. That first start waits at
the prompts past the client's 5 s handshake deadline (C86) and the
command is lost; the report says to answer Keychain before it names
the pid. Do not kill it: answer the prompts, then run the command
again.

Sleep and wakes (soaks): a closed laptop on AC dark-wakes every 15 to
60 min (Power Nap) and cron runs during the wakes; on battery it sleeps
for hours; `pmset -g log` is the evidence for any sleep claim. The
first request after a wake can fail in transport before the network is
up; the timeout rail paces it as counted, and a consumer sees one
failed job.
