# The stored row, shared words and the crate edges

The plan's D4. Investigator's report: `reports/investigators/stored-row.md`. Portfolio: proposal 10.

## What was found

"Unreadable" means three unrelated things, and the plan merged them: a body from GGG that ingest cannot file (kept as evidence in `refused`); a file another build wrote (a schema stamp); and an item line the game tables do not cover (the search's three-valued logic, which is a different concept and stays). The finding class is a fourth thing: an intent row a newer build wrote, handled again and again above the store.

On the owner's store none of it has happened: 0 unreadable intent rows of 21, 0 withheld responses of 4,946, 4 refused bodies from one day in September and one cause.

The typed read is already in the store and nothing calls it.

## Checked by the orchestrator

- `Annotations::get_as` has no production caller: `git grep -n 'get_as\b' -- crates` finds it in its own file and in tests.
- `check_value` is called above the store in `plan` (`lib.rs`, `listing.rs`, `price.rs`, `pricing.rs`), `verbs` (`receipt.rs`, `saved.rs`) and the CLI (`price_cmd.rs`). The investigator counts 32 sites by a wider rule; the orchestrator's narrower grep finds about 15.
- The daemon's comment above `KINDS` (`daemon/queue.rs:1437-1447`) lists the places a new job kind is added by hand; with the table itself, eleven.
- `grep keyring tools/arch-check.sh` finds nothing: invariant 5 has no edge check.
- Not checked: the 129 match lines in 18 files, and the investigator's reading of `record_in`.

## Where this parts from the plan

- The history scout said C1's edges partly force the scattered defaults. They do not. No `Realm::DEFAULT` or `DEFAULT_LEAGUE` site is in the store; they are the frontends' defaults for an unnamed scope, and go only with proposal 3.
- The search's own realm type is not forced by C1 either; nothing forbids the search linking the protocol today.
- The smallest set of edges that keeps invariants 1, 3 and 5 structural is four: the HTTP client in the transport crate only, the transport in the daemon only, the daemon linked by nobody, the keyring in the daemon only. The fourth is unchecked. No other crate boundary holds an invariant; the rest hold C34, C41 and C89.

## Position

**One typed intent read in the store.** `read::<K>()` returns a typed value or the one refusal the store already has; "a newer build wrote it" becomes a flag on that refusal. About ten types and variants above the store go. Smallest input that changes: a policy row whose text is not JSON gets the same refusal as a row from a newer build, where today the two are different faults.

**The store links the protocol crate, and only it.** Job kind and realm become types with one home. Eleven places become one enum with exhaustive matches; five lists of realm words become one. The wire stays the same strings, and the store keeps reading its realm column as text so a newer build's fifth realm is not a new unreadable row.

**`record_in` in two cases.** Its true statement is short: a body is refused whole or lands whole; a listing stamps its members and retires what it did not stamp; a fetch replaces its location's contents. The code interleaves validation with writes and writes one retirement rule three times.

The `refused` table stays: four rows, and the only evidence behind two ground-truth claims.

## Route

Proposal 10, with the one sentence for the owner: "The store links the protocol crate, and only it; the protocol still links serde alone." Issues drafted: `an-unparsable-realm-becomes-pc`, `c1-omits-the-client-store-edge`, `invariant-5-has-no-edge`.

## After the skeptic

Narrowed (`dissent/theses-6-10.md`, thesis 10), in ways the investigator's own caveats mostly anticipated.

- `Realm` is already a type in the protocol crate (`realm.rs:75`). What is missing is the store's use of it, not the type.
- A typed read must keep the row's address and revision when the value cannot be read. `get_as` as it stands does not, so it cannot simply be substituted: a future-version price at revision 7 has to stay nameable, clearable and replaceable, and pricing stops inheritance at it (`listing.rs:1452`).
- An unknown realm string in a stored row must stay observable, not become unreadable.
- That none of the unreadable states occurs in the owner's data says how rare they are, not that the recovery they allow is expendable.

What stands: eleven hand-kept places per job kind; a typed read with no production caller; one condition given two different faults.
