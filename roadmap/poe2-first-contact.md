# PoE2 stashes: the first contacts and the merge

The day GGG documents the stash endpoints for Path of Exile 2, the
owner runs this from a terminal on the branch `prep/poe2-stash`, under
the live-run skill. The skill's standing rule, its "Before" and its
"After" apply unchanged; this file names only what differs, in the
order the day runs. The branch opens one arm of `Family::accepts`
(`crates/acquisition-protocol/src/realm.rs`): the stash family takes
`poe2`. The account-leagues family stays refused.

## 1. GGG's page

Read `https://www.pathofexile.com/developer/docs/reference`, the stash
section. The branch renders three URL shapes under `poe2`
(`route_for` in `crates/acquisition-daemon/src/daemon/routes.rs`):

| Shape | Route label | Job kind |
| --- | --- | --- |
| `GET /stash/poe2/{league}` | `stash-list/poe2` | `stashes` |
| `GET /stash/poe2/{league}/{stash_id}` | `stash/poe2` | `stash` |
| `GET /stash/poe2/{league}/{stash_id}/{substash_id}` | `stash/poe2` | `stash` with `sub` |

Each label's first use is preceded by a HEAD on its URL (the free
probe). The rule: **every one of the three shapes must be documented
for realm `poe2`**. Where only some are, stop here and merge nothing.
The arm opens the whole family, and `Family::accepts` cannot open part
of it. Narrowing it would take a per-kind check where `route_for` and
the mock read the family (a substash-less stash family, say), built
and reviewed as a step of its own before any contact.

Also note on the page whether the realm segment is still written
before the league (`/stash/poe2/{league}`, as `/character/poe2` is),
and whether `pc` is still meant when it is left out. A page that puts
it anywhere else is a stop too.

## 2. The build the contacts run from

1. The skill's "Before", steps 1 and 2: the environment unset, and
   `acq daemon stop` until it says the daemon is not running. Nothing
   builds or gates under a live daemon.
2. `git fetch`, then rebase `prep/poe2-stash` onto `origin/main`.
3. `tools/gate.sh` on the rebased tree, exit 0.
4. The skill's "Before", steps 3 and 4, on this branch: a clean tree,
   `cargo build --workspace`, `tools/sign-acqd.sh`, `./target/debug/acq
   version`, and `runs/` against the ledger. The contacts run from
   this build, never from `main`'s, which still refuses `--realm poe2`
   on `acq stashes` and `acq stash`.

## 3. The first contacts

Pick the league from the PoE2 characters already in the store:
`acq store characters --realm poe2` (`acq leagues` is realm-less and
lists PoE1's). `Standard` exists in both games (C58).

One fresh daemon per contact (`acq daemon stop` until it says the
daemon is not running), the listing first:

| # | Route label | Command | Ceiling |
| --- | --- | --- | --- |
| 1 | `stash-list/poe2` | `ACQ_TRIPWIRE=1 ACQ_MAX_SENDS=3 acq stashes --realm poe2 --league <L>` | 3 (token POST, HEAD, GET); 2 if `acq auth status` shows a valid access token |
| 2 | `stash/poe2` | `ACQ_TRIPWIRE=1 ACQ_MAX_SENDS=3 acq stash <id> --realm poe2 --league <L>` | the same |
| 3 | `stash/poe2`, a substash | `ACQ_TRIPWIRE=1 ACQ_MAX_SENDS=3 acq stash <id> --sub <sub> --realm poe2 --league <L>` | the same |

Contact 2's tab is a map or unique tab (`MapStash` or `UniqueStash`)
from contact 1's listing where the listing holds one, else an
ordinary tab. A substash's id is not in the listing: a map or unique
tab's substashes come as the `children` of that tab's own answer,
which the store files as tabs under it. So contact 3 runs only when
contact 2 fetched such a tab and its answer listed children. Its
`<sub>` is one of them, read after contact 2 with `acq tabs --realm
poe2 --league <L> --expand`, which lists a tab's substashes after it.
Otherwise contact 3 does not run, and the substash shape stays
unobserved: write that in the row and in the claims.

## 4. After each contact, the journal

`acq daemon status` prints its path. Read it before anything else:

- The probe line's hits on its policy. GGG keeps the counters, not
  the daemon: a fresh daemon's HEAD reports hits an earlier one spent
  (N37 for an `Account` rule; N6, one policy name shares its counters
  across routes). Contacts 2 and 3 share `stash/poe2`, and contact 1's
  listing shares its policy too if the names say so, so a later
  contact's HEAD counts the day's own earlier GETs while they are
  inside that policy's windows. Either wait out the longest window the
  earlier contact's headers reported, plus 60 s, before the next
  contact, after which the probe must report 0 hits; or reconcile, and
  the hits must equal the day's own GETs on that policy still inside
  each window, read from the earlier contacts' journals. A hit neither
  accounts for is the skill's stop: find what else uses the account.
- The policy's name and windows. PoE2's character routes carry pc's
  windows under a `-poe2` suffix (N41). Write down what the stash
  routes carry, whatever it is.
- The URL as sent: `/stash/poe2/<L>` and so on, byte for byte.

What an answer other than 200 means, and what is done. Never a
retry, never a reset-and-retry:

- **403**: possibly a Cloudflare block (invariant 3; N1 to N4, N28).
  The tripwire halts. Wait 360 s, read the response headers in the
  journal, write the row, and stop for the day.
- **404**: record the status, the URL as sent and the body, as
  observed. It counts toward the invalid-request budget (N27). Stop
  and write the row, with no retry. Its cause stays open: a resource
  that is gone (a tab removed between the listing and the fetch, a
  league the realm does not hold) and a shape GGG does not serve for
  `poe2` both answer 404, and the row names the cause only when
  evidence tells them apart. The branch is not merged while it is
  open.
- **400**: refused as invalid. It counts toward GGG's invalid-request
  budget (N27; N60 is a 400 on a shape answered 200 elsewhere). Stop,
  write the row and read the body before anything else is sent.
- **200 with a body the store refuses**: the wire passed and the
  ingest did not. `acq store refused` shows the body kept verbatim.
  Write the row, stop the contacts and bring the body to a session.
  The shape that blocks ingest is ruled and built before the next
  contact, as the PoE2 character bodies were (`RUN-LEDGER.md`, the rows
  of 2026-09-02).

## 5. The claims

New numbered claims in `docs/design/network-ground-truth.md`, in N41's
form (the observation, its source tag and confidence, the
consequence):

- The stash policies under `poe2`: names, rules, windows, and whether
  the HEAD is free.
- Whether a PoE2 tab or item body carries `realm: "poe2"` (N43 found
  it on every PoE2 character item), and the tab types the listing
  holds.
- Issue 45's three: what the realm segment of each endpoint's path
  means and whether `pc` is meant when it is left out; whether PoE2
  is still served on the character endpoints only (now: characters
  and stashes, not account leagues); what `inventoryId` holds on a
  stash item. A fact the day does not observe is named in the file as
  still unobserved.

## 6. The ledger row

One row per contact in `RUN-LEDGER.md`, in its columns:

| Date | Run | Daemon tip | Result | Sends (POST/HEAD/GET) | Violations | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `<date>` | first contact: `<shape>` under `poe2`, ceiling `<n>`, tripwire on | `<branch HEAD, short>` | `<pass, or the status and the stop>` | `<p>/<h>/<g> = <total>` | `<count>` | Contract `<acq version --json contract>`, `acqd` `<sha>`. Probe: `<status>`, `<hits>` hits, policy `<name>` `<rules>` `<windows>`. GET: `<status>`, state after `<state>`. Body: `<tabs or items, realm field, refused or not>`. Claims: `<N…>`. `runs/<dir>/` |

## 7. The merge

Only after the contacts it ran have passed:

1. The rows and the claims committed on the branch.
2. `tools/gate.sh` on that tree, the one holding the committed rows
   and claims, exit 0, before it is pushed or merged.
3. Merge `prep/poe2-stash` into `main`, a fast-forward from the
   rebased branch, and push.
4. On `main` after it: CI's two gates pass at the merged tip; `acq
   stashes --help` offers `poe2`; the README's "Known gaps" no longer
   says GGG offers no PoE2 stash tabs; a `poe2` policy entry naming
   `tabs` is accepted by `acq policy set`.
5. If contact 3 did not run, a substash under `poe2` is still
   unobserved. File it as an issue labelled `trigger:live`, so the
   first PoE2 map or unique tab gets its own contact.
