//! `acq` — the command-line frontend: clap and rendering over the
//! daemon protocol and the store's read API (C12).
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

mod dash;
mod plan_cmd;
mod price_cmd;
#[cfg(test)]
mod real_scale_fixture;
mod reference_cmd;
mod search_cmd;
mod shop_cmd;
mod store_cmd;

use std::io::{IsTerminal as _, Write as _};
use std::time::{Duration, Instant};

use acquisition_client::client::{Client, DaemonError, Observed, Signal, Subscription};
use acquisition_protocol::job::{JobInfo, JobState, Outcome};
use acquisition_protocol::protocol::{ErrorKind, Request, Response};
use acquisition_protocol::provider::GGG;
use acquisition_protocol::realm::Realm;
use acquisition_store::world::World;
use acquisition_verbs::jobs::{job_info, result_live};
use acquisition_verbs::{
    Account, ChildTally, DaemonReport, Fault, Frontend, Job, JobResult, Jobs, Progress,
};
use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use serde_json::json;

/// Who this process is and which account its flags select (C46: the
/// layer's values in place of process statics; C121). Built once in
/// `run`, after the environment is checked, and handed to every verb.
pub(crate) struct Ctx {
    pub frontend: Frontend,
    /// `--account` / `ACQ_ACCOUNT` as typed: a job verb passes it to the
    /// daemon unresolved (C3), a verb that opens a store resolves it.
    pub account: Option<String>,
}

impl Ctx {
    /// C121: the account the flags select, resolved against the index.
    pub fn account(&self) -> Result<Account, Fault> {
        self.frontend.account(self.account.as_deref())
    }
}

/// A use verb's connect (C10): the layer's human door — lazy spawn, and
/// a daemon of another contract, artifact or provider replaced (C84) —
/// noting a spawn for the refusal that follows one.
pub(crate) async fn connect(cx: &Ctx) -> Result<Client> {
    let client = cx.frontend.connect().await?;
    note_spawn(&client);
    Ok(client)
}

/// Remember that this command started the daemon it talks to (C3).
pub(crate) fn note_spawn(client: &Client) {
    if client.spawned() {
        SPAWNED.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// The running daemon, for a verb that observes it or acts on it (C10):
/// never spawns or replaces. Absence and a mismatch are faults the CLI
/// words; `daemon status` renders the same observation as states, and
/// `jobs` answers absence from the ledger on disk (C45).
pub(crate) async fn attach(cx: &Ctx) -> Result<Client> {
    Ok(cx.frontend.attach().await?)
}

/// `jobs` with no daemon (C45): `running: false` and the persisted queue
/// — the same table as the live list, without ETAs, under a line that
/// says how old the picture is and what restores it.
fn print_persisted_queue(jobs: &Jobs, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(jobs)?);
        return Ok(());
    }
    let Jobs::Absent { persisted } = jobs else {
        return Ok(());
    };
    let spawns = "a job command or `acq auth` starts one";
    match persisted {
        None => println!("daemon is not running; no queue on disk ({spawns})"),
        Some(p) if p.jobs.is_empty() => {
            println!("daemon is not running; nothing waits on disk ({spawns})")
        }
        Some(p) => {
            println!(
                "daemon is not running; on disk: {} waiting, {} recorded running, written {} — \
                 a job command spawns a successor that restores them, and `acq cancel <id>` \
                 then acts on them",
                p.waiting(),
                p.recorded_running(),
                store_cmd::ago(acquisition_store::now(), p.written_at),
            );
            let jobs = p
                .jobs
                .iter()
                .map(job_info)
                .collect::<Result<Vec<_>, Fault>>()?;
            print_table(&jobs);
        }
    }
    Ok(())
}

/// `status <id>` with no daemon: the row as recorded, in `jobs`' absent
/// shape — `running: false` beside what the ledger holds, and `next`.
fn print_persisted_job(id: u64, job: &Job, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(job)?);
        return Ok(());
    }
    let Job::Absent { persisted, .. } = job else {
        return Ok(());
    };
    println!(
        "daemon is not running; job {id} as the ledger on disk records it, written {}",
        store_cmd::ago(acquisition_store::now(), Some(persisted.updated_at)),
    );
    print_table(&[job_info(persisted)?]);
    Ok(())
}

/// What a human does about a daemon this client will not use: a
/// mismatch of contract, artifact or provider is replaced by a job
/// command; a daemon on another world (C83) is never replaced — stop it,
/// or point this shell at its world.
pub(crate) fn mismatch_remedy(found: &acquisition_client::client::DaemonId) -> &'static str {
    if found.world_matches() {
        "`acq daemon stop` stops it, a job command (`acq profile`, `acq refresh --apply`) replaces it"
    } else {
        "a job command from this shell refuses it too (another world is never replaced) — `acq daemon stop` stops it, or point ACQ_STORE_DIR at its world"
    }
}

/// The log the running daemon opened, as it reports it (`daemon_status`,
/// C83) — never recomputed from this shell's environment, which may
/// differ from the daemon's (review 2026-09-11).
pub(crate) async fn daemon_log_path(client: &mut Client) -> Option<String> {
    match client.request(&Request::DaemonStatus).await {
        Ok(Response::DaemonStatus { log, .. }) => log,
        _ => None,
    }
}

#[derive(Parser)]
#[command(
    name = "acq",
    // Usage lines name `acq`, never the file it was run as (`acq.exe`
    // on Windows): the printed commands and the reference are one text.
    bin_name = "acq",
    // `<pkg version> (contract <revision>)`: the contract half of the
    // handshake identity (C10, C84); `acq version --json` is the
    // structured form, with the sibling `acqd` beside it.
    version = acquisition_protocol::VERSION_WITH_CONTRACT,
    about = "Acquisition CLI: the real GGG API by default; ACQ_PROVIDER=mock talks to the in-process mock"
)]
struct Cli {
    /// Emit structured JSON instead of human-readable output: one
    /// document on stdout, a failure included (`{"error", "kind"}`). A
    /// job verb's is `{"outcome", "payload", "job_id", "account"}`, and a
    /// failed job's carries `error` in place of `payload`; without the
    /// flag it prints the payload alone.
    #[arg(long, global = true)]
    json: bool,
    /// Which account to act as: a username (with or without `#…`) or uuid.
    /// Defaults to `ACQ_ACCOUNT`. With neither, a verb that reads the
    /// store takes the sole account known on disk (`acq accounts`) and a
    /// job verb the sole account logged in (`acq auth status`); with more
    /// than one, the verb refuses and lists them. A job's progress line
    /// and its JSON name the account it ran as.
    #[arg(long, global = true, env = "ACQ_ACCOUNT")]
    account: Option<String>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Log in via OAuth (real GGG, or the mock under ACQ_PROVIDER=mock). The
    /// login completes only once its own profile job lands the account
    /// uuid; the mock's page accepts any username, so a second
    /// account is one login apart.
    Auth {
        #[command(subcommand)]
        cmd: Option<AuthCmd>,
        /// Print the login URL instead of opening a browser. With --json
        /// the URL is its own line first and the result follows when the
        /// login completes: two documents, since the URL must print
        /// before the login can finish.
        #[arg(long)]
        no_browser: bool,
    },
    /// The account profile (account:profile).
    Profile,
    /// List characters on the logged-in account. A route's first use
    /// queues a visible `probe` job — one HEAD that learns the policy and
    /// the account's current counters before anything real is sent.
    Characters {
        /// pc (default), xbox, sony, or poe2. pc is omitted on the wire;
        /// another realm is its own route segment, with its own probe.
        #[arg(long, default_value = "pc", value_parser = acquisition_verbs::realm)]
        realm: Realm,
    },
    /// Fetch one character with its equipment and inventory.
    Character {
        /// The character's name, as `acq characters` lists it. The sync
        /// policy and `acq price` name a character by its id instead
        /// (`acq store characters`).
        name: String,
        /// pc (default), xbox, sony, or poe2; pc is omitted on the wire.
        #[arg(long, default_value = "pc", value_parser = acquisition_verbs::realm)]
        realm: Realm,
    },
    /// List the account's leagues.
    Leagues,
    /// Accounts this machine has logged into (the store's index; no daemon).
    Accounts,
    /// Tabs of a league, from the shared store (no daemon round-trip):
    /// the counts first, then every tab with its type, items, age and id.
    /// A league of more than thirty tabs is counted by type with its ten
    /// largest tabs instead; --expand lists every one. --type and
    /// --never-fetched select the tabs listed — the never fetched of one
    /// type by name, which a refresh takes — under the league's counts.
    /// `--json` is the same document: the counts, `by_type`, and `rows`,
    /// the tabs listed, with `left_out` counting the rest.
    Tabs {
        /// List every tab of a large league, or of the selection.
        #[arg(long)]
        expand: bool,
        /// Only tabs of this type, as GGG names it (UniqueStash,
        /// MapStash, PremiumStash …; the counts by type list them).
        #[arg(long, value_name = "TYPE")]
        r#type: Option<String>,
        /// Only tabs never fetched.
        #[arg(long)]
        never_fetched: bool,
        #[arg(long, default_value = acquisition_plan::DEFAULT_LEAGUE)]
        league: String,
        /// pc (default), xbox, or sony — stashes are PoE1 only.
        #[arg(long, default_value = "pc", value_parser = acquisition_verbs::realm)]
        realm: Realm,
    },
    /// Search the items the store holds (no daemon): a query over every
    /// live item of one realm, answered with the canonical query, the
    /// scope searched, the total, and the rows with the lines the query
    /// touched — then each term's counts over the scope, matched, failed,
    /// lacked, undecided, and the basis. Every count has a route:
    /// the command that returns exactly its members, printed where a next
    /// step hangs on it and every one under `--routes`. `--json` is the
    /// answer whole, in the same order, the query as text and as tree, a
    /// count's request where its route is printed; a failure is `{"error",
    /// "kind", "readings"}`, with `offers` where a way on is a request
    /// `--request` reads. `--describe` prints the language, and what
    /// this build refuses by name.
    Search(Box<search_cmd::SearchArgs>),
    /// One item as the search derives it, live or removed (no daemon): its fields, its
    /// place, every line with its kind, template and numbers, and what
    /// could not be read; `--json` the same, structured. `--body` adds the
    /// body as the store holds it, which is still there when deriving
    /// could not read it.
    Show(search_cmd::ShowArgs),
    /// The shared store itself (what the daemon writes; every frontend reads).
    Store {
        #[command(subcommand)]
        cmd: StoreCmd,
    },
    /// List stash tabs for a league: a second rate-limit policy, paced in
    /// parallel with the character routes.
    Stashes {
        #[arg(long, default_value = acquisition_plan::DEFAULT_LEAGUE)]
        league: String,
        /// pc (default), xbox, or sony — the stash endpoints are PoE1 only;
        /// poe2 is refused at admission.
        #[arg(long, default_value = "pc", value_parser = acquisition_verbs::realm)]
        realm: Realm,
    },
    /// Fetch one stash tab (or one substash of a map/unique tab).
    Stash {
        id: String,
        #[arg(long)]
        sub: Option<String>,
        /// Follow a map/unique tab's substashes as child jobs. Opt-in per
        /// tab: one map tab can hold hundreds.
        #[arg(long)]
        deep: bool,
        #[arg(long, default_value = acquisition_plan::DEFAULT_LEAGUE)]
        league: String,
        /// pc (default), xbox, or sony — the stash endpoints are PoE1 only;
        /// poe2 is refused at admission.
        #[arg(long, default_value = "pc", value_parser = acquisition_verbs::realm)]
        realm: Realm,
    },
    /// Refresh tabs: one stash-list request, then one `stash` child job per
    /// selected tab. Selection is explicit — there is no default. (The
    /// ad-hoc `--tabs`/`--all` kind and the plan path are two doors to one
    /// task; the ad-hoc kind retires once nothing submits it.) Keeping a league up to date is the
    /// plan path: `acq policy set` once, then `--plan` and `--apply` until
    /// the plan says nothing to do — a map or unique tab's substashes are
    /// known only once the tab is fetched, so a first sync takes two
    /// rounds, and each apply ends by saying what is left.
    #[command(after_long_help = "\
Reading a plan as an agent: the text is a function of the envelope, so count \
with `jq` over `acq refresh --plan --json` rather than parsing prose —
  jq '.logical_requests'
  jq '[.actions[] | .action] | group_by(.) | map({(.[0]): length}) | add'          # requests by kind
  jq '[.actions[] | select(.action == \"fetch_substash\") | .parent] | group_by(.) | map({(.[0]): length}) | add'   # substashes per parent
  jq '[.skipped_tabs[], .skipped_characters[] | .reason.kind] | group_by(.) | map({(.[0]): length}) | add'   # skips by reason
`acq refresh --apply --json` adds `store_changes` beside the outcome; `acq store events --summary --json` \
is the per-location summary.")]
    Refresh {
        /// Every tab in the league (folder children included, folders not).
        #[arg(long, conflicts_with = "tabs")]
        all: bool,
        /// Comma-separated tab ids.
        #[arg(long, value_delimiter = ',')]
        tabs: Vec<String>,
        /// Also follow map/unique substashes (per tab, as child jobs).
        #[arg(long)]
        deep: bool,
        /// Compile the stored sync policy (`acq policy`) into the explicit
        /// action set and print it — nothing is submitted or sent. A
        /// running daemon adds its read-only quote. With --json, prints
        /// the serializable plan envelope. `--plan=FILE` (or `=-` for
        /// stdin) renders a reviewed envelope instead, through the same
        /// renderer, with the quote it carries.
        #[arg(long, value_name = "FILE", num_args = 0..=1, require_equals = true,
              default_missing_value = "", conflicts_with_all = ["all", "tabs", "deep"])]
        plan: Option<String>,
        /// With --plan: one line per action and every quote note, instead
        /// of the grouped view (groups over ten entities are counted).
        #[arg(long, requires = "plan")]
        expand: bool,
        /// Execute the plan: exactly its actions, as one `apply` parent
        /// job that never expands the set. Bare `--apply` compiles the
        /// stored policy now, so it never fetches what is fresh;
        /// `--apply=FILE` (or `--apply=-` for stdin) reads a reviewed plan
        /// envelope from `refresh --plan --json`. A plan spends each
        /// request once: a target fetched after the plan was compiled, or
        /// a request the daemon already holds, is named and not sent
        /// again, so applying a file twice, or retrying an apply that is
        /// still running, sends nothing (`applied: false` and `spent` in
        /// JSON, exit 0). To fetch what is fresh, name it: `acq stash`,
        /// `acq character`, `refresh --tabs`. Refused if the stored
        /// policy revision no longer matches the plan's.
        #[arg(long, value_name = "FILE", num_args = 0..=1, require_equals = true,
              default_missing_value = "", conflicts_with_all = ["all", "tabs", "deep", "plan"])]
        apply: Option<String>,
        /// Refuse the apply before anything runs if the plan authorizes
        /// more than this many requests (checked by the daemon at
        /// admission, before any child job exists).
        #[arg(long, requires = "apply")]
        max_requests: Option<u64>,
        /// Defaults to Standard. With `--apply=FILE`, the plan's own
        /// league governs; giving --league too asserts they agree.
        #[arg(long)]
        league: Option<String>,
        /// pc (default), xbox, sony, or poe2 (a poe2 policy entry can
        /// cover characters only). With `--apply=FILE`, the plan's own
        /// realm governs; giving --realm too asserts they agree.
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
    },
    /// The per-account sync policy: the declared coverage and freshness
    /// that `acq refresh --plan` compiles into requests.
    Policy {
        #[command(subcommand)]
        cmd: Option<PolicyCmd>,
    },
    /// Submit any job kind by hand.
    Submit {
        /// A network kind — profile, characters, character, leagues,
        /// stashes, stash, refresh, apply — or sleep, which never sends,
        /// or a mock-only one: fetch, whoami (refused in real mode).
        kind: String,
        /// JSON params, as the verb of the same name sends them:
        /// characters `{"realm"}`, character `{"name", "realm"}`, stashes
        /// `{"league", "realm"}`, stash `{"id", "sub", "deep", "league",
        /// "realm"}`, refresh `{"tabs": […] or "all": true, "deep",
        /// "league", "realm"}`, sleep `{"seconds"}`; realm defaults to pc and league
        /// to Standard. `acq status <id> --json` shows a job's params.
        #[arg(long, default_value = "{}")]
        params: String,
        /// Higher runs sooner. Default 0.
        #[arg(long, default_value_t = 0)]
        priority: u8,
        /// Return the job id immediately instead of blocking with progress.
        #[arg(long)]
        detach: bool,
    },
    /// Submit a burst of fetch jobs against the mock's 5-per-10 s policy
    /// and watch the rate limiter queue them (the ETAs are the limiter's
    /// prediction, corrected by headers).
    Demo {
        #[arg(long, default_value_t = 8)]
        count: u32,
    },
    /// Live dashboard (TUI): rate limiter state (enter expands a policy:
    /// bucket state, the observed X-Rate-Limit headers, per-endpoint
    /// sends), job queue, HTTP sends, recent errors, a rails halt in red.
    /// With --json, prints one snapshot and exits.
    Dash,
    /// What this build is: the package version, the shared-contract
    /// revision the daemon handshake compares — a digest over the
    /// protocol and store crates' manifests and `src` trees, the root
    /// manifest and the lock, never a git commit — and the sibling
    /// `acqd` a job command would start, as
    /// found on disk (path, length, modification time; no hash — the run
    /// record hashes it). This reports the candidate; `acq daemon status`
    /// reports the daemon running. `--json`: {"version", "contract",
    /// "provider", "acqd": {path, len, mtime_ns} | null}, plus
    /// "acqd_absent" (why) when acqd is null; `--version` is the human
    /// form of the first two.
    Version,
    /// The live jobs: id, parent, kind, target (from params), state
    /// (`↻n` counts 429 re-queues), priority, account, submitter, ETA.
    /// Every waiting and running job is listed, and the ten most recent
    /// finished ones; the last line counts them all, and says when the
    /// next waiting job starts and when the last one does. `--json` is
    /// every job the daemon holds.
    /// Observes only: with no daemon running it never spawns one, and
    /// answers from the ledger on disk instead — the non-terminal
    /// rows as the last daemon left them, without ETAs, and how long ago
    /// they were written; `--json`: `running: false`, `persisted` (`jobs`,
    /// `written_at`; `null` when this shell's world has no queue yet). A
    /// job command spawns a successor that restores them; `cancel` then
    /// acts on them.
    Jobs {
        /// Subscribe, print the queue, then every job-state change as it
        /// happens; the queue is printed again after a missed-events signal,
        /// and the watch ends when the daemon stops.
        #[arg(long)]
        watch: bool,
        /// List every finished job the daemon still holds, not the ten
        /// most recent. A daemon holds the jobs it ran: an earlier
        /// daemon's finished jobs are not listed, and `acq result <id>`
        /// still answers them from disk.
        #[arg(long, conflicts_with = "watch")]
        all: bool,
    },
    /// One job's state and ETA. A large ETA is the limiter holding, not a
    /// hang: holds can reach 300 s plus the timing bucket. With no daemon
    /// running, the job as the ledger on disk records it (never spawns).
    Status { id: u64 },
    /// A finished job's payload or error. Kept on disk (7 days done, 30
    /// failed, by default) and answered whether or not a daemon runs; a
    /// job still waiting or running is an error unless --wait. A failed
    /// or cancelled job exits 1. A failed fetch's refused body is in
    /// `acq store refused <id>`, not here.
    Result {
        id: u64,
        /// Block until the job finishes, printing progress on stderr,
        /// then print the result: how a `--detach`ed job is collected.
        /// Needs the daemon that is running the job.
        #[arg(long)]
        wait: bool,
    },
    /// Cancel a waiting or running job; cascades to every descendant
    /// still waiting. Needs the running daemon and never starts one.
    Cancel { id: u64 },
    /// Change a waiting job's priority; higher runs sooner and the queue
    /// reorders live. Needs the running daemon and never starts one.
    SetPriority { id: u64, priority: u8 },
    /// The listing state (no daemon): what every item, tab and character
    /// is priced as — by hand, in game, and how the two stand.
    Price {
        #[command(subcommand)]
        cmd: Option<PriceCmd>,
    },
    /// Reference data the binary ships (no store, no daemon): the currency
    /// table, by version.
    Reference {
        #[command(subcommand)]
        cmd: ReferenceCmd,
    },
    /// The forum shop (no daemon, sends nothing): render the page set a
    /// hand price would post, every omission counted with its reason.
    Shop {
        #[command(subcommand)]
        cmd: ShopCmd,
    },
    /// Inspect, stop or reset the daemon. A job command or `acq auth`
    /// starts one on demand; `stop` is for a login not in the keyring,
    /// which ends with the daemon, and for the README's longer-lived
    /// daemon recipe, which starts a new one while none runs.
    Daemon {
        #[command(subcommand)]
        cmd: DaemonCmd,
    },
}

#[derive(Subcommand)]
enum PriceCmd {
    /// One line on the league's listing state and the next action (the
    /// default); --expand adds the game side, the rows and the basis.
    Status {
        #[arg(long, default_value = acquisition_plan::DEFAULT_LEAGUE)]
        league: String,
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
        #[arg(long)]
        expand: bool,
    },
    /// One target's listing: `item/<id>`, `character/<id>`,
    /// `tab/<realm>/<id>` or `substash/<realm>/<parent>/<id>` — both
    /// sides with their causes, the raw note beside the parse; a
    /// container's items summarized under it (--expand lists them all).
    Show {
        target: String,
        /// The league to read the target in. Without it, the league the
        /// facts hold the target in: an address reads back as printed.
        #[arg(long)]
        league: Option<String>,
        /// The realm of an item or character target; a tab or substash
        /// address carries its own.
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
        #[arg(long)]
        expand: bool,
    },
    /// Listed items grouped by container, ten or fewer listed per group
    /// and more counted (--expand lists every one with its texts).
    /// With neither --relation nor --effective, items whose relation is
    /// `none` are left out — except unresolved ones (a row that cannot
    /// be read could decide), which are always listed.
    List {
        /// The league. Without it, the league of the container --in or
        /// --covered-by names, else Standard.
        #[arg(long)]
        league: Option<String>,
        /// The realm; a tab or substash address given to --in or
        /// --covered-by carries its own.
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
        /// none, manual_only, game_only, agree or conflict.
        #[arg(long)]
        relation: Option<String>,
        /// Who decides: game, manual, none or unresolved.
        #[arg(long)]
        effective: Option<String>,
        /// Only items physically in this container (a tab, substash or
        /// character address).
        #[arg(long = "in")]
        location: Option<String>,
        /// Only items a row on this target would cover: a folder's
        /// tabs' items, a map or unique tab's substashes' items too.
        #[arg(long)]
        covered_by: Option<String>,
        /// List at most N items, the first in listing order; the first
        /// line says N of how many.
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
        #[arg(long)]
        expand: bool,
    },
    /// Set one target's price by hand — a row on an item, tab, substash
    /// or character; what it covers inherits it. Prints what it
    /// replaced, how to put that back, and a note when the facts hold no
    /// such target (a tab not fetched yet, or a typo — never a refusal).
    /// A clear takes a revision too, so the next write's revision
    /// continues from it.
    Set {
        /// `item/<id>`, `character/<id>`, `tab/<realm>/<id>` or
        /// `substash/<realm>/<parent>/<id>`; `acq price show item/<id>`
        /// names the tab an item sits in.
        target: String,
        /// exact, negotiable, no_price or skip (the game's `price`, `b/o`
        /// and `~skip` work too).
        #[arg(value_name = "TYPE")]
        kind: String,
        /// For exact and negotiable: a decimal of up to four places
        /// (`12.5`), or a `wanted/lot` ratio (`1/5`).
        amount: Option<String>,
        /// For exact and negotiable: a tag of `acq reference currency`.
        currency: Option<String>,
        /// Only write over exactly this revision (what `acq price show`
        /// printed when you reviewed it); refused naming the current one
        /// otherwise. Without it the write replaces whatever is stored —
        /// though a write racing in between still conflicts rather than
        /// being clobbered.
        #[arg(long)]
        if_revision: Option<i64>,
    },
    /// Remove one target's own price row; what it covered falls back to
    /// the next row up. Prints what it removed and the command that
    /// puts it back. A clear takes a revision too, so the next write's
    /// revision continues from it.
    Clear {
        /// The address, as for `set`.
        target: String,
        /// Only clear exactly this revision; see `set`.
        #[arg(long)]
        if_revision: Option<i64>,
    },
}

#[derive(Subcommand)]
enum ShopCmd {
    /// Render the shop pages to stdout for pasting by hand: one
    /// link code per hand-priced item under its price's spoiler, one
    /// page spoiler labelled n of N, in pages of at most --size
    /// characters; what the
    /// game already lists is omitted and counted, an unobserved case is
    /// blocked and counted (an item in a substash, or socketed in
    /// another: the forum has no link form for it); the sync policy's
    /// coverage and freshness are reported, never enforced. A link's
    /// `Stash<n>` is the tab's rank among the tabs the website lists,
    /// folders and substashes left out, not the API's index.
    Render {
        #[arg(long, default_value = acquisition_plan::DEFAULT_LEAGUE)]
        league: String,
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
        /// Characters per page, the template included (the forum's hard
        /// limit).
        #[arg(long, default_value_t = acquisition_plan::shop::DEFAULT_PAGE_SIZE)]
        size: usize,
        /// A file whose `[items]` token each page replaces; without it
        /// the page is the items alone.
        #[arg(long)]
        template: Option<std::path::PathBuf>,
        /// Print one page alone (its text; with --json, its record), for
        /// the clipboard.
        #[arg(long)]
        page: Option<usize>,
        /// The whole policy table with each row's rule and count, and
        /// every item left off the page with its cell, those nothing
        /// applies to included. Without it they are counted: the --json
        /// document lists the first 50 left off for a reason, the blocked
        /// first, and `left_out_total` counts them all.
        #[arg(long)]
        expand: bool,
    },
}

#[derive(Subcommand)]
enum ReferenceCmd {
    /// The currency table: every row (tag, display name, the words a
    /// parser accepts, retired marks), or one word resolved. Exact and
    /// case-sensitive; `--json` is the whole table with its evidence.
    Currency {
        /// A tag, the game's word, or a legacy alias to resolve.
        word: Option<String>,
        /// Show each row's evidence and the table's sources.
        #[arg(long)]
        expand: bool,
    },
}

#[derive(Subcommand)]
enum StoreCmd {
    /// Path, size, and row counts.
    Status,
    /// Characters on record (no daemon): id, address, league, freshness,
    /// live item count. `acq characters` (the job) fetches the list anew.
    Characters {
        /// Restrict to one realm (pc, xbox, sony, poe2); every realm otherwise.
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
        /// Restrict to one league; every league otherwise.
        #[arg(long)]
        league: Option<String>,
    },
    /// Item events (added/moved/changed/removed) from recent ingests: by
    /// default one line per location with counts (text) or the event list
    /// (--json); --expand / --summary pick either form in both modes.
    Events {
        #[arg(long, default_value_t = 24.0)]
        hours: f64,
        #[arg(long, default_value_t = 200)]
        limit: usize,
        /// Every event, one per line (the JSON default).
        #[arg(long, conflicts_with = "summary")]
        expand: bool,
        /// One line per location with counts (the text default).
        #[arg(long)]
        summary: bool,
    },
    /// Bodies the store refused as malformed, kept verbatim as evidence:
    /// the list (newest first), or one body in full by its row id.
    Refused {
        /// Row id from the list (or from the failed job's error).
        id: Option<i64>,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// Re-extract derived columns from each item's own JSON.
    Rebuild,
    /// Replay a snapshot file from the retired `acq pull` into the store (no GGG traffic).
    Import { path: std::path::PathBuf },
}

#[derive(Subcommand)]
enum PolicyCmd {
    /// Print the stored policy value and its revision (the default).
    Show,
    /// Write the policy: inline JSON, `-` to read stdin, or `@<path>` to
    /// read a file. Validated before it is stored — a typo'd field or an
    /// unknown version is refused, never half-honored. Each realm the
    /// value names replaces that realm's policy, every league of it; a
    /// realm it does not name stays as stored, and a realm named with no
    /// leagues (`"poe2":{"leagues":{}}`) is removed. To add a league, read
    /// the policy with `acq policy show` and write its realm back with the
    /// league added; `--replace` writes the whole policy instead.
    #[command(after_long_help = "\
The policy, version 3:
  {\"version\":3,\"realms\":{\"pc\":{\"leagues\":{\"Standard\":{\"tabs\":\"all\",\"characters\":\"all\",\"max_age_seconds\":3600}}}}}
  realms            pc, xbox, sony or poe2, each with its leagues by name
  tabs              \"all\", or a list of tab ids (`acq stashes` or `acq tabs` print them); an id covers
                    the tab, a folder's children and a map or unique tab's substashes; not under poe2
  characters        \"all\", or a list of character ids (`acq characters`)
  max_age_seconds   required: what is older than this is fetched again; a plan made sooner says nothing to do
A league entry names tabs, characters or both. Nothing is fetched by setting a policy: \
`acq refresh --plan` shows what it asks for, `acq refresh --apply` fetches it.")]
    Set {
        /// The policy as JSON, `-` for stdin, or `@<path>` for a file.
        value: String,
        /// Only write if the stored policy is at exactly this revision
        /// (what `acq policy show` printed when you reviewed it); refused
        /// with the current revision otherwise. Without it the write
        /// replaces whatever revision is currently stored — though a write
        /// racing in between still conflicts rather than being clobbered.
        #[arg(long)]
        if_revision: Option<i64>,
        /// Replace the whole policy: every realm the value does not name
        /// is removed.
        #[arg(long)]
        replace: bool,
    },
}

#[derive(Subcommand)]
enum AuthCmd {
    /// Show session state (login, token expiry, keyring health).
    Status,
    /// Verify the session actually works: forces a token round-trip through
    /// the provider instead of trusting local state. Exit code 1 on failure.
    Check,
    /// Drop the session and clear the keyring entry. With --account naming
    /// another known account, clear only that account's keyring entry.
    Logout,
}

#[derive(Subcommand)]
enum DaemonCmd {
    /// The daemon running: pid, version, contract revision, the executable
    /// it runs from and its hash, provider, the world it serves
    /// (its canonical store root), uptime, the idle exit it was started with, connections, queue
    /// counts, policies learned, the socket, log and journal paths, the
    /// rails state, keyring health. Observes only: never spawns or
    /// replaces; with no daemon running, what the ledger on disk holds
    /// — `persisted`: `waiting`, `recorded_running`, `written_at`;
    /// `null` when this shell's world has no queue yet; a daemon of
    /// another contract, artifact, provider or
    /// world is reported by its identity alone — no vitals — and left
    /// running; something that accepts the connection and does not answer
    /// hello within 5s is reported unresponsive — `--json`: running,
    /// `responsive: false`, `socket`, `lock_holder` (the pid the world's
    /// lock records) — for a human to stop by hand.
    /// `--json`, in the two identified cases: running, compatible, `socket` (the
    /// one this shell reached it through: derived from the world), which of
    /// the four dimensions match (`contract_matches`, `artifact_matches`,
    /// `provider_matches`, `world_matches`), how the daemon's file relates to this
    /// client's sibling — `artifact_relation`: `same_file`; `same_bytes`,
    /// another copy; `different`; `unhashable`; `no_sibling`, no `acqd`
    /// beside this executable; `unreported`, a daemon from before the
    /// field — and under `wanted` this client's own version, contract,
    /// provider and world (`world_absent` says why when this shell's
    /// store root does not exist) with the sibling `acqd` a job command
    /// would start, all from the one look that judged the daemon.
    Status,
    /// Stop the daemon listening on this shell's world's socket,
    /// this build's — its contract and artifact — or another's.
    /// Queued jobs stay on disk and resume under the next one; a
    /// client's jobs are never cancelled by its leaving. `--json`:
    /// `stopped`, `pid`, `version`, `provider`, `compatible`, `socket`.
    Stop,
    /// Clear the live-test rails' tripwire/ceiling halt. After a trip,
    /// wait 360 s before clearing; a trip is evidence to read. With no
    /// daemon running, clears the persisted state in this shell's world
    /// (`<store root>/<provider>/rails.json`) so the next daemon
    /// starts clear.
    ResetTripwire,
}

/// The failure was already printed in the command's own output format (a
/// failed job's `--json` outcome); main only sets the exit code.
#[derive(Debug)]
struct AlreadyReported;

impl std::fmt::Display for AlreadyReported {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("already reported")
    }
}

impl std::error::Error for AlreadyReported {}

/// A command line the parser refuses, under `--json`: the same one
/// document on stdout as any other failure (C11), `kind: "usage"`, with
/// the parser's own exit 2 — the command never ran. Help and version are
/// not failures and print as they always do; so does a refusal without
/// `--json`. `--json` is read from the raw arguments, since nothing was
/// parsed: any argument spelled so before a bare `--`.
fn refuse_usage(e: clap::Error) -> ! {
    use clap::error::ErrorKind as Parse;
    let asked_json = std::env::args()
        .skip(1)
        .take_while(|a| a != "--")
        .any(|a| a == "--json");
    let informative = matches!(
        e.kind(),
        Parse::DisplayHelp
            | Parse::DisplayVersion
            | Parse::DisplayHelpOnMissingArgumentOrSubcommand
    );
    if !asked_json || informative {
        e.exit();
    }
    let message = e.render().to_string();
    let message = message.trim().trim_start_matches("error: ");
    println!("{}", json!({ "error": message, "kind": "usage" }));
    std::process::exit(2);
}

/// A layer fault in this frontend's words (C11, C116): the cause the
/// fault says, and the remedy in the CLI's own verbs — `--account`, `acq
/// auth`, `acq policy set`, `acq refresh --plan`. One arm per fault, so
/// a cause the layer adds cannot reach a person without its remedy.
fn fault_words(fault: &Fault) -> String {
    match fault {
        Fault::Environment(_)
        | Fault::Unopened(_)
        | Fault::Plan(_)
        | Fault::Policy(_)
        | Fault::Door(_)
        | Fault::Unresponsive(_)
        | Fault::Daemon(_)
        | Fault::Transport(_)
        | Fault::Other(_) => fault.to_string(),
        Fault::NoAccounts { .. } => format!("{fault} — log in with `acq auth`"),
        Fault::UnknownAccount { .. } => format!("{fault} (see `acq accounts`)"),
        Fault::AmbiguousAccount { .. } => {
            format!("{fault}; pick one with --account or ACQ_ACCOUNT")
        }
        Fault::NoUuid { .. } => format!("{fault}; log in once with `acq auth` to fix it"),
        Fault::NoSyncPolicy { .. } => format!(
            "{fault} — declare one first, e.g. `acq policy set '{}'`",
            plan_cmd::POLICY_EXAMPLE
        ),
        Fault::Spend(
            acquisition_plan::SpendError::PolicyGone { .. }
            | acquisition_plan::SpendError::PolicyMoved { .. },
        ) => format!("{fault} with `acq refresh --plan`"),
        Fault::Spend(_) => fault.to_string(),
        // A merge reads the stored row to merge into, so the retry is a
        // whole write naming the revision.
        Fault::UnreadablePolicy { revision, .. } => format!(
            "{fault} — `--replace --if-revision {revision}` writes a whole policy over it \
             deliberately; `acq policy show` names it"
        ),
        Fault::NoDaemon { because: None, .. } => {
            "daemon is not running (a job command or `acq auth` starts one)".into()
        }
        Fault::NoDaemon { .. } => fault.to_string(),
        Fault::NoJobOnDisk { .. } => {
            format!("{fault} (a job command or `acq auth` starts one)")
        }
        Fault::Unfinished { id, .. } => format!(
            "{fault}: a job command starts one that resumes it, and `acq result {id} --wait` \
             then waits for it"
        ),
        Fault::AnotherDaemon(found) => format!("{fault}; {}", mismatch_remedy(found)),
    }
}

/// The failure's words: its chain as anyhow prints it, a layer fault in
/// the CLI's words where the chain holds one.
fn failure_words(e: &anyhow::Error) -> String {
    let mut parts = Vec::new();
    for link in e.chain() {
        if let Some(fault) = link.downcast_ref::<Fault>() {
            parts.push(fault_words(fault));
            return parts.join(": ");
        }
        parts.push(link.to_string());
    }
    format!("{e:#}")
}

/// What a failure's `--json` document carries beside its words (C11): a
/// layer fault's kind and fields (`accounts`, `next`: C121, C45), a
/// pricing refusal's (`conflict` with `revision`, `not_found`), a daemon
/// refusal's closed kind (C85). Additive under C53; a failure with no
/// kind has none to branch on beyond its message.
fn error_report(e: &anyhow::Error) -> serde_json::Value {
    let mut report = serde_json::Map::new();
    report.insert("error".into(), json!(failure_words(e)));
    if let Some(fault) = e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        report.extend(fault.fields());
    } else if let Some(refused) = acquisition_plan::pricing::Refused::find(e) {
        report.extend(refused.fields());
    } else if let Some(refusal) = DaemonError::find(e) {
        report.insert("kind".into(), json!(refusal.kind));
    }
    serde_json::Value::Object(report)
}

/// Set when this command started the daemon it then talked to, so a
/// refusal for want of a login can say a daemon was started (C3 kept
/// after the release seat: cli-opus calls 100 to 101).
static SPAWNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The refusal of a job verb that started a daemon and was then refused
/// for want of a login, saying so: the daemon stays until idle, and the
/// line names what shows and what ends it. The kind is kept for the JSON
/// document.
fn noting_the_spawn(e: anyhow::Error) -> anyhow::Error {
    if !SPAWNED.load(std::sync::atomic::Ordering::Relaxed) {
        return e;
    }
    let refusal = match e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        Some(Fault::Daemon(refusal)) => Some(refusal.clone()),
        Some(_) => None,
        None => DaemonError::find(&e).cloned(),
    };
    match refusal {
        Some(refusal) if refusal.kind == ErrorKind::NotLoggedIn => {
            anyhow::Error::from(Fault::Daemon(DaemonError {
                kind: refusal.kind,
                message: format!(
                    "{}; a daemon was started for this command and runs until idle — \
                     `acq daemon status` shows it, `acq daemon stop` ends it",
                    failure_words(&e)
                ),
            }))
        }
        _ => e,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::try_parse().unwrap_or_else(|e| refuse_usage(e));
    let json = cli.json;
    if let Err(e) = run(cli).await.map_err(noting_the_spawn) {
        if e.downcast_ref::<AlreadyReported>().is_none() {
            if json {
                println!("{}", error_report(&e));
            } else {
                eprintln!("Error: {}", failure_words(&e));
            }
        }
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<()> {
    // C88: a leftover `ACQ_GGG` or an unknown provider word is refused
    // before any verb reads the provider — loudly, never the wrong mode.
    let cx = Ctx {
        frontend: Frontend::human(plan_cmd::WRITTEN_VIA)?,
        account: cli.account.clone(),
    };
    let cx = &cx;
    match cli.cmd {
        Cmd::Auth { cmd, no_browser } => match cmd {
            None => login(cx, no_browser, cli.json).await,
            Some(AuthCmd::Status) => {
                let Some(mut client) = cx.frontend.observe().await? else {
                    return store_cmd::auth_status_absent(cx, cli.json);
                };
                let status = client.request(&Request::AuthStatus).await?;
                print_auth(&status, cli.json)?;
                if !cli.json {
                    store_cmd::print_other_accounts(
                        cx,
                        match &status {
                            Response::Auth { accounts, .. } => {
                                accounts.iter().map(|a| a.username.clone()).collect()
                            }
                            _ => Vec::new(),
                        },
                    )?;
                }
                Ok(())
            }
            Some(AuthCmd::Check) => {
                let mut client = connect(cx).await?;
                let account = cx.account.clone();
                match client.request(&Request::AuthCheck { account }).await? {
                    Response::Error { kind, message } => {
                        Err(anyhow::Error::from(DaemonError { kind, message })
                            .context("auth check failed"))
                    }
                    status => {
                        if !cli.json {
                            println!("session verified (live token round-trip succeeded)");
                        }
                        print_auth(&status, cli.json)
                    }
                }
            }
            Some(AuthCmd::Logout) => {
                let mut client = attach(cx).await?;
                let account = cx.account.clone();
                client
                    .expect_ack(&Request::AuthLogout {
                        account: account.clone(),
                    })
                    .await?;
                if cli.json {
                    println!("{}", json!({ "logged_out": true, "account": account }));
                } else {
                    println!("logged out (session dropped, keyring cleared)");
                }
                Ok(())
            }
        },
        Cmd::Profile => {
            let mut client = connect(cx).await?;
            let id = submit(cx, &mut client, "profile".into(), json!({}), 0).await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Characters { realm } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "characters".into(),
                json!({ "realm": realm }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Character { name, realm } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "character".into(),
                json!({ "realm": realm, "name": name }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Leagues => {
            let mut client = connect(cx).await?;
            let id = submit(cx, &mut client, "leagues".into(), json!({}), 0).await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Accounts => store_cmd::accounts(cx, cli.json),
        Cmd::Tabs {
            league,
            realm,
            expand,
            r#type,
            never_fetched,
        } => store_cmd::tabs(
            cx,
            realm,
            &league,
            &acquisition_search::tabs::Selection {
                kind: r#type,
                never_fetched,
            },
            expand,
            cli.json,
        ),
        Cmd::Search(args) => search_cmd::search(cx, *args, cli.json),
        Cmd::Show(args) => search_cmd::show_item(cx, args, cli.json),
        Cmd::Store { cmd } => match cmd {
            StoreCmd::Status => store_cmd::status(cx, cli.json),
            StoreCmd::Characters { realm, league } => {
                store_cmd::characters(cx, realm, league.as_deref(), cli.json)
            }
            StoreCmd::Events {
                hours,
                limit,
                expand,
                summary,
            } => store_cmd::events(cx, hours, limit, expand, summary, cli.json),
            StoreCmd::Refused { id, limit } => store_cmd::refused(cx, id, limit, cli.json),
            StoreCmd::Rebuild => store_cmd::rebuild(cx, cli.json),
            StoreCmd::Import { path } => store_cmd::import(cx, &path, cli.json),
        },
        Cmd::Stashes { league, realm } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "stashes".into(),
                json!({ "realm": realm, "league": league }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Stash {
            id,
            sub,
            deep,
            league,
            realm,
        } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "stash".into(),
                json!({ "realm": realm, "league": league, "id": id, "sub": sub, "deep": deep }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Refresh {
            all,
            tabs,
            deep,
            plan,
            expand,
            apply,
            max_requests,
            league,
            realm,
        } => {
            if let Some(source) = plan {
                // Bare `--plan` compiles the stored policy; `=FILE` renders
                // a reviewed envelope.
                let source = (!source.is_empty()).then_some(source);
                return plan_cmd::refresh_plan(
                    cx,
                    realm.unwrap_or(Realm::DEFAULT),
                    league
                        .as_deref()
                        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE),
                    source.as_deref(),
                    cli.json,
                    expand,
                )
                .await;
            }
            if let Some(source) = apply {
                // Bare `--apply` (clap's empty default_missing_value)
                // means "compile the stored policy now and run that".
                let source = (!source.is_empty()).then_some(source);
                return plan_cmd::refresh_apply(
                    cx,
                    realm,
                    league.as_deref(),
                    source.as_deref(),
                    max_requests,
                    cli.json,
                )
                .await;
            }
            if !all && tabs.is_empty() {
                bail!("refresh needs --all, --tabs <id,...>, --plan, or --apply");
            }
            let league = league
                .as_deref()
                .unwrap_or(acquisition_plan::DEFAULT_LEAGUE);
            let realm = realm.unwrap_or(Realm::DEFAULT);
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "refresh".into(),
                json!({ "realm": realm, "league": league, "all": all, "tabs": tabs, "deep": deep }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Price { cmd } => match cmd.unwrap_or(PriceCmd::Status {
            league: acquisition_plan::pricing::DEFAULT_LEAGUE.into(),
            realm: None,
            expand: false,
        }) {
            PriceCmd::Status {
                league,
                realm,
                expand,
            } => price_cmd::status(
                cx,
                realm.unwrap_or(Realm::DEFAULT),
                &league,
                expand,
                cli.json,
            ),
            PriceCmd::Show {
                target,
                league,
                realm,
                expand,
            } => price_cmd::show(cx, &target, realm, league.as_deref(), expand, cli.json),
            PriceCmd::List {
                league,
                realm,
                relation,
                effective,
                location,
                covered_by,
                limit,
                expand,
            } => price_cmd::list(
                cx,
                realm,
                league.as_deref(),
                &acquisition_plan::pricing::ListArgs {
                    relation,
                    effective,
                    location,
                    covered_by,
                    limit,
                },
                expand,
                cli.json,
            ),
            PriceCmd::Set {
                target,
                kind,
                amount,
                currency,
                if_revision,
            } => price_cmd::set(
                cx,
                &target,
                &kind,
                amount.as_deref(),
                currency.as_deref(),
                if_revision,
                cli.json,
            ),
            PriceCmd::Clear {
                target,
                if_revision,
            } => price_cmd::clear(cx, &target, if_revision, cli.json),
        },
        Cmd::Reference { cmd } => match cmd {
            ReferenceCmd::Currency { word, expand } => {
                reference_cmd::currency(word.as_deref(), expand, cli.json)
            }
        },
        Cmd::Shop { cmd } => match cmd {
            ShopCmd::Render {
                league,
                realm,
                size,
                template,
                page,
                expand,
            } => shop_cmd::render_cmd(
                cx,
                realm.unwrap_or(Realm::DEFAULT),
                &league,
                &shop_cmd::RenderArgs {
                    size,
                    template: template.as_deref(),
                    page,
                    expand,
                    json: cli.json,
                },
            ),
        },
        Cmd::Policy { cmd } => match cmd {
            None | Some(PolicyCmd::Show) => plan_cmd::policy_show(cx, cli.json),
            Some(PolicyCmd::Set {
                value,
                if_revision,
                replace,
            }) => plan_cmd::policy_set(cx, &value, if_revision, replace, cli.json),
        },
        Cmd::Submit {
            kind,
            params,
            priority,
            detach,
        } => {
            let params: serde_json::Value = serde_json::from_str(&params)?;
            let mut client = connect(cx).await?;
            let id = submit(cx, &mut client, kind, params, priority).await?;
            if detach {
                if cli.json {
                    println!("{}", json!({ "job_id": id }));
                } else {
                    println!(
                        "job {id} submitted (`acq status {id}` reads its state, `acq result {id} --wait` collects it)"
                    );
                }
                Ok(())
            } else {
                block_on_job(&mut client, id, cli.json).await
            }
        }
        Cmd::Demo { count } => {
            let mut client = connect(cx).await?;
            let mut ids = Vec::new();
            for i in 0..count {
                let id = submit(
                    cx,
                    &mut client,
                    "fetch".into(),
                    json!({ "what": format!("demo stash tab {i}") }),
                    0,
                )
                .await?;
                ids.push(id);
            }
            println!("submitted {count} fetch jobs: {ids:?}");
            println!("(mock policy: 5 per 10s, then 30 per 300s — watch the ETAs)\n");
            watch_table_until_done(&mut client, &ids).await
        }
        Cmd::Dash => dash::run(cx, cli.json).await,
        Cmd::Version => {
            // The candidate, not the daemon running (C84): the sibling
            // `acqd` as found, with no hash — the run record hashes it.
            let sibling = acquisition_client::artifact::sibling();
            if cli.json {
                let mut report = json!({
                    "version": acquisition_protocol::VERSION,
                    "contract": acquisition_protocol::CONTRACT_REVISION,
                    "provider": acquisition_protocol::provider::wanted(),
                    "acqd": sibling.as_ref().ok().map(|s| json!({
                        "path": s.identity.path,
                        "len": s.identity.len,
                        "mtime_ns": s.identity.mtime_ns,
                    })),
                });
                if let Err(e) = &sibling {
                    report["acqd_absent"] = json!(e.to_string());
                }
                println!("{report}");
            } else {
                println!(
                    "acq {}, provider {}",
                    acquisition_protocol::VERSION_WITH_CONTRACT,
                    acquisition_protocol::provider::wanted()
                );
                match sibling {
                    Ok(s) => println!(
                        "acqd: {} ({} bytes) — what a job command would start; `acq daemon status` reports the daemon running",
                        s.identity.path, s.identity.len
                    ),
                    Err(e) => println!("acqd: none — {e}"),
                }
            }
            Ok(())
        }
        Cmd::Jobs { watch, all } => {
            if watch {
                return watch_jobs(cx, cli.json).await;
            }
            let jobs = match cx.frontend.jobs().await? {
                Jobs::Live(jobs) => jobs,
                absent => return print_persisted_queue(&absent, cli.json),
            };
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&jobs)?);
            } else {
                print_jobs(&jobs, all);
            }
            Ok(())
        }
        Cmd::Status { id } => {
            let job = match cx.frontend.job(id).await? {
                Job::Live(job) => job,
                absent => return print_persisted_job(id, &absent, cli.json),
            };
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&job)?);
            } else {
                print_table(std::slice::from_ref(&job));
            }
            Ok(())
        }
        Cmd::Result { id, wait } => {
            let Some(mut client) = cx.frontend.observe().await? else {
                // C45: a terminal job's outcome is final, so the ledger's
                // prints exactly as a daemon would answer it.
                return print_outcome(&cx.frontend.result_on_disk(id)?, cli.json);
            };
            // Only a job still in the queue is waited for: a previous
            // lifetime's result is served from retention (C45) and has no
            // status to wait on.
            if wait
                && matches!(
                    client.request(&Request::Result { id }).await?,
                    Response::Error {
                        kind: ErrorKind::WrongState,
                        ..
                    }
                )
            {
                wait_for_job(&mut client, id, cli.json).await?;
            }
            print_result(&mut client, id, cli.json).await
        }
        Cmd::Cancel { id } => {
            let mut client = attach(cx).await?;
            client.expect_ack(&Request::Cancel { id }).await?;
            if cli.json {
                println!("{}", json!({ "job_id": id, "cancel_requested": true }));
            } else {
                println!(
                    "job {id} cancel requested — a running job stops at its next check; \
                     `acq result {id} --wait` returns once it has"
                );
            }
            Ok(())
        }
        Cmd::SetPriority { id, priority } => {
            let mut client = attach(cx).await?;
            client
                .expect_ack(&Request::SetPriority { id, priority })
                .await?;
            if cli.json {
                println!("{}", json!({ "job_id": id, "priority": priority }));
            } else {
                println!("job {id} priority -> {priority}");
            }
            Ok(())
        }
        Cmd::Daemon { cmd } => match cmd {
            DaemonCmd::Status => {
                // An observation (C10): absent, this build's, or another
                // daemon reported and left alone — never spawned or
                // replaced. The document is the layer's (C45, C84, C86).
                let report = cx.frontend.daemon().await?;
                if cli.json {
                    match &report {
                        DaemonReport::Absent { .. } => println!("{}", report.document()),
                        _ => println!("{}", serde_json::to_string_pretty(&report.document())?),
                    }
                    return Ok(());
                }
                match report {
                    DaemonReport::Absent { persisted, log } => {
                        match persisted {
                            None => println!("daemon is not running; no queue on disk"),
                            Some(p) => println!(
                                "daemon is not running; on disk: {} waiting, {} recorded running, written {}",
                                p.waiting,
                                p.recorded_running,
                                store_cmd::ago(acquisition_store::now(), p.written_at),
                            ),
                        }
                        if let Some(log) = &log {
                            println!("last log: {}", log.display());
                        }
                    }
                    DaemonReport::Incompatible(found) => {
                        println!("{found} — running, not this client's");
                        println!("socket: {}", found.socket());
                        println!("next:   {}", mismatch_remedy(&found));
                    }
                    // Something accepted and never answered (C86): a
                    // state, like the others — not this verb's error.
                    DaemonReport::Unresponsive(found) => println!("{found}"),
                    DaemonReport::Running { status, identity } => {
                        let found = *identity;
                        if let Response::DaemonStatus {
                            pid,
                            version,
                            contract,
                            provider,
                            uptime_seconds,
                            idle_shutdown_seconds,
                            connections,
                            jobs_waiting,
                            jobs_running,
                            policies_known,
                            in_flight,
                            max_in_flight,
                            rails,
                            keyring,
                            log,
                        } = *status
                        {
                            println!(
                                "daemon {version} (contract {contract}) pid {pid}, up {uptime_seconds}s, provider {provider}, idle exit after {idle_shutdown_seconds}s"
                            );
                            println!("world:  {}", found.world());
                            match (found.artifact(), &found.verdict().artifact) {
                                (
                                    Some(a),
                                    acquisition_client::artifact::ArtifactVerdict::SameBytes {
                                        sibling,
                                    },
                                ) => {
                                    println!(
                                        "acqd:   {} (sha256 {}) — the same artifact as this client's sibling {}, another copy",
                                        a.file.path,
                                        a.short_hash(),
                                        sibling.path
                                    )
                                }
                                (Some(a), _) => println!(
                                    "acqd:   {} (sha256 {}) — this client's sibling, the file a job command would start",
                                    a.file.path,
                                    a.short_hash()
                                ),
                                (None, _) => println!("acqd:   not reported"),
                            }
                            println!(
                                "connections: {connections}  waiting: {jobs_waiting}  running: {jobs_running}  in flight: {in_flight}/{max_in_flight}  policies learned: {policies_known}"
                            );
                            println!("socket: {}", found.socket());
                            println!("log:    {}", log.as_deref().unwrap_or("not reported"));
                            println!(
                                "rails:  tripwire {} · sends {}{} · journal {}",
                                if rails.tripwire_enabled { "ON" } else { "off" },
                                rails.sends,
                                rails.max_sends.map_or(String::new(), |m| format!("/{m}")),
                                rails.journal.as_deref().unwrap_or("off"),
                            );
                            if let Some(cause) = &rails.halted {
                                println!("HALTED: {cause}");
                                // A ceiling is this lifetime's and never persisted
                                // (rails.rs): no violation, nothing to reset — the
                                // next daemon starts under a fresh one.
                                if cause.starts_with("ceiling:") {
                                    println!(
                                        "        the ceiling is this daemon's alone: `acq daemon stop`, and the next one starts fresh (`acq jobs` first: queued jobs resume under it)"
                                    );
                                } else {
                                    println!(
                                        "        clear with `acq daemon reset-tripwire` after the post-violation wait"
                                    );
                                }
                            }
                            if keyring != "ok" {
                                println!(
                                    "KEYRING: {keyring} — a rotated refresh token may be memory-only"
                                );
                            }
                            if let Some(cause) = &rails.refresh_failed {
                                println!("REFRESH DISABLED: {cause}");
                                println!("        re-login with `acq auth`");
                            }
                        }
                    }
                }
                Ok(())
            }
            DaemonCmd::ResetTripwire => {
                match Client::observe().await? {
                    Observed::Compatible(mut client) => {
                        let resp = client.request(&Request::ResetTripwire).await?;
                        if cli.json {
                            println!("{}", serde_json::to_string_pretty(&resp)?);
                        } else {
                            println!("rails reset");
                        }
                    }
                    // Another daemon holds its rails in memory; clearing the
                    // file under it would not reset anything. Stop it first.
                    Observed::Incompatible(found) => {
                        bail!("{found}; `acq daemon stop` first")
                    }
                    Observed::Unresponsive(found) => bail!("{found}"),
                    Observed::Absent => {
                        // The trip lives on disk, in the world (C83); clear
                        // it so the next spawned daemon is not still
                        // halted. No world, nothing persisted.
                        let provider = acquisition_protocol::provider::wanted();
                        let candidates: Vec<std::path::PathBuf> = World::observe()
                            .map(|world| vec![world.rails_state_path(provider)])
                            .unwrap_or_default();
                        // A file, or an empty directory in a file's place
                        // (the daemon refuses to start over either shape
                        // it cannot read, and names this verb as the
                        // remedy); anything else is named for the hand.
                        let mut cleared = Vec::new();
                        for state in &candidates {
                            let removed = match std::fs::symlink_metadata(state) {
                                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                                Err(e) => Err(e),
                                Ok(meta) if meta.is_dir() => std::fs::remove_dir(state).map_err(|e| {
                                    std::io::Error::new(
                                        e.kind(),
                                        format!("it is a directory that is not empty ({e}); remove it by hand"),
                                    )
                                }),
                                Ok(_) => std::fs::remove_file(state),
                            };
                            match removed {
                                Ok(()) => cleared.push(state.clone()),
                                Err(e) => bail!(
                                    "daemon is not running; could not clear {}: {e}",
                                    state.display()
                                ),
                            }
                        }
                        if cli.json {
                            println!(
                                "{}",
                                json!({ "cleared": !cleared.is_empty(), "state": cleared.first() })
                            );
                        } else if cleared.is_empty() {
                            println!("daemon is not running and no rails state is persisted");
                        } else {
                            for state in cleared {
                                println!(
                                    "daemon is not running; cleared persisted rails state {}",
                                    state.display()
                                );
                            }
                        }
                    }
                }
                Ok(())
            }
            DaemonCmd::Stop => {
                // A session the keyring does not hold lives in the daemon
                // alone, so stopping it is a logout: asked of this build's
                // daemon before it goes, and said after. Best effort — a
                // daemon that will not answer is stopped all the same.
                let mut ended: Vec<String> = Vec::new();
                if let Ok(Observed::Compatible(mut client)) = Client::observe().await
                    && let Ok(Response::Auth { accounts, .. }) =
                        client.request(&Request::AuthStatus).await
                {
                    ended = accounts
                        .into_iter()
                        .filter(|a| a.keyring != "ok")
                        .map(|a| a.username)
                        .collect();
                }
                // Stops this build's daemon or any other (C10): stopping is
                // how a mismatch is resolved by hand.
                match Client::stop_any().await? {
                    Some(found) => {
                        if cli.json {
                            println!(
                                "{}",
                                json!({
                                    "stopped": true,
                                    "pid": found.pid(),
                                    "version": found.version(),
                                    "provider": found.provider(),
                                    "compatible": found.is_ours(),
                                    "socket": found.socket(),
                                    "sessions_ended": ended,
                                })
                            );
                        } else if found.is_ours() {
                            println!("daemon stopped (pid {})", found.pid());
                            if !ended.is_empty() {
                                println!(
                                    "logged out with it: {} — not in the keyring, so the session \
                                     lived in that daemon alone; `acq auth` logs in again",
                                    ended.join(", ")
                                );
                            }
                        } else {
                            println!("stopped: {found}");
                        }
                    }
                    None => {
                        if cli.json {
                            println!("{}", json!({ "stopped": false, "running": false }));
                        } else {
                            println!("daemon is not running");
                        }
                    }
                }
                Ok(())
            }
        },
    }
}

/// The interactive login flow: ask the daemon to start OAuth, hand the URL to
/// the browser, then poll auth status until the flow resolves.
async fn login(cx: &Ctx, no_browser: bool, json: bool) -> Result<()> {
    let mut client = connect(cx).await?;
    let url = match client.request(&Request::AuthStart).await? {
        Response::AuthUrl { authorize_url } => authorize_url,
        Response::Error { kind, message } => return Err(DaemonError { kind, message }.into()),
        other => bail!("unexpected response: {other:?}"),
    };
    if json {
        // First of two JSON lines (the final auth status is the second), so
        // a scripted login can read the URL it must visit.
        println!("{}", json!({ "authorize_url": url }));
        std::io::stdout().flush().ok();
    } else {
        println!("To log in, open:\n\n  {url}\n");
    }
    if !no_browser && open_browser(&url) && !json {
        println!("(opened in your browser)");
    }
    if !json {
        println!("waiting for login to complete...");
    }
    let deadline = std::time::Instant::now() + Duration::from_secs(300);
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        let status = client.request(&Request::AuthStatus).await?;
        let Response::Auth {
            pending,
            ref login_ok,
            ref login_error,
            ref keyring,
            ..
        } = status
        else {
            bail!("unexpected response: {status:?}");
        };
        if pending {
            if std::time::Instant::now() > deadline {
                bail!("login did not complete within 5 minutes");
            }
            continue;
        }
        // Only the flow's own terminal result counts: `logged_in` is
        // aggregate state, and another account's live session must not be
        // mistaken for this login succeeding.
        if let Some(error) = login_error {
            bail!("login failed: {error}");
        }
        let Some(user) = login_ok else {
            bail!("login did not complete (see daemon log)");
        };
        if json {
            return print_auth(&status, true);
        }
        println!("logged in as {user}");
        if keyring != "ok" {
            println!("warning: keyring {keyring}; session will not survive a daemon restart");
        }
        return Ok(());
    }
}

fn print_auth(status: &Response, json: bool) -> Result<()> {
    let Response::Auth {
        logged_in,
        pending,
        keyring,
        provider,
        accounts,
        ..
    } = status
    else {
        bail!("unexpected response: {status:?}");
    };
    if json {
        println!("{}", serde_json::to_string_pretty(status)?);
        return Ok(());
    }
    if provider == GGG {
        println!("provider: real GGG");
    }
    match (logged_in, pending) {
        (_, true) => println!("login in progress (waiting on the browser)"),
        (false, _) => println!("not logged in — run `acq auth`"),
        (true, _) => {
            for a in accounts {
                let token = match a.access_expires_in_seconds {
                    Some(s) if s > 0 => {
                        format!(
                            "access token valid for ~{s}s (renewed on use; the login outlasts it)"
                        )
                    }
                    _ => "access token expired (renewed on next use; the login stands)".into(),
                };
                let kr = if a.keyring == "ok" {
                    String::new()
                } else {
                    format!("; keyring {}", a.keyring)
                };
                println!("logged in as {}: {token}{kr}", a.username);
            }
        }
    }
    println!("keyring: {keyring}");
    Ok(())
}

fn open_browser(url: &str) -> bool {
    let opener = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    std::process::Command::new(opener)
        .arg(url)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .is_ok()
}

/// C3 and C4: submit a job as this process, the account flag passed to
/// the daemon unresolved.
async fn submit(
    cx: &Ctx,
    client: &mut Client,
    kind: String,
    params: serde_json::Value,
    priority: u8,
) -> Result<u64> {
    Ok(acquisition_verbs::jobs::submit(
        &cx.frontend,
        client,
        &kind,
        params,
        priority,
        cx.account.as_deref(),
    )
    .await?)
}

async fn list(client: &mut Client) -> Result<Vec<JobInfo>> {
    match client.request(&Request::List).await? {
        Response::Jobs { jobs } => Ok(jobs),
        other => bail!("unexpected response: {other:?}"),
    }
}

/// `jobs --watch`: the subscriber's sequence under C85, as the reference
/// consumer. Subscribe first, then take the snapshot over a request
/// connection to the same daemon instance (a restart landing between the
/// two connections starts the sequence over); a job that changes between
/// subscribing and reading is an event, never a gap. Every event is an
/// invalidation, so the job is re-read over the request connection before
/// its line is printed — an event never stands in for the read, and a
/// read that fails restarts the sequence rather than trusting the hint.
/// On `resync_required` the lagged subscription is dropped — its queued
/// events predate any snapshot taken now — and the sequence starts over
/// with a fresh subscription and snapshot; when the daemon goes, observe
/// it again — an observer never spawns — and subscribe and snapshot
/// afresh if it is back. Pinned by `tests/watch_recovery.rs`.
async fn watch_jobs(cx: &Ctx, json: bool) -> Result<()> {
    let print_snapshot = |jobs: &[JobInfo]| -> Result<()> {
        if json {
            println!("{}", serde_json::to_string_pretty(jobs)?);
        } else {
            print_table(jobs);
        }
        Ok(())
    };
    loop {
        let mut subscription = match Subscription::observe().await? {
            Observed::Compatible(subscription) => subscription,
            Observed::Absent => bail!("daemon stopped (it spawns on demand for job commands)"),
            Observed::Incompatible(found) => bail!("{found}"),
            Observed::Unresponsive(found) => bail!("{found}"),
        };
        let mut client = attach(cx).await?;
        if client.daemon().pid() != subscription.daemon().pid() {
            // A restart between the two connections: the subscription is
            // to a daemon that is gone. Start over.
            continue;
        }
        print_snapshot(&list(&mut client).await?)?;
        loop {
            let signal = match subscription.next().await {
                Ok(signal) => signal,
                // A transport failure mid-stream is a disconnect; anything
                // else is a protocol violation worth stopping on.
                Err(e) if e.downcast_ref::<std::io::Error>().is_some() => None,
                Err(e) => return Err(e),
            };
            match signal {
                Some(Signal::Event(hint)) => {
                    // The read, never the hint, is what gets printed; a
                    // read that fails restarts the sequence.
                    let job = match client.status(hint.id).await {
                        Ok(job) => job,
                        Err(e) => {
                            if !json {
                                println!(
                                    "re-reading job {} failed ({e:#}); subscribing and reading again",
                                    hint.id
                                );
                            }
                            break;
                        }
                    };
                    if json {
                        println!("{}", serde_json::to_string(&job)?);
                    } else {
                        println!("job {:>3}  {:<8} -> {}", job.id, job.kind, job.state);
                    }
                }
                Some(Signal::ResyncRequired { missed }) => {
                    if !json {
                        println!("missed {missed} event(s); subscribing and reading again");
                    }
                    break;
                }
                None => {
                    if !json {
                        println!("daemon went away; watching for it");
                    }
                    break;
                }
            }
        }
    }
}

/// Default CLI mode: block with progress until the job finishes, then print
/// its result. This is where "rate limited, retrying in 4m37s..." UX lives.
pub(crate) async fn block_on_job(client: &mut Client, id: u64, json: bool) -> Result<()> {
    wait_for_job(client, id, json).await?;
    print_result(client, id, json).await
}

/// Progress is redrawn in place only on a tty; captured output (the
/// driver's `.out` files, an agent's shell) gets a plain line per change,
/// at most every this often — a 13-minute apply is not 1,500 lines.
const PROGRESS_EVERY: Duration = Duration::from_secs(10);

/// Block until the job is terminal and return its outcome, through the
/// layer's wait (C4). Progress goes to stderr: stdout is the answer and
/// nothing else, in text mode as in JSON, so what a caller captures is
/// what the job returned. `quiet` skips the progress line.
pub(crate) async fn wait_for_job(client: &mut Client, id: u64, quiet: bool) -> Result<Outcome> {
    let mut progress = Progressor::new();
    let mut report = |p: &Progress| progress.report(p);
    let outcome = acquisition_verbs::jobs::wait(
        client,
        id,
        (!quiet).then_some(&mut report as &mut (dyn FnMut(&Progress) + Send)),
    )
    .await?;
    Ok(outcome)
}

/// The progress line's state: redrawn in place on a tty, a plain line per
/// change elsewhere, at most every [`PROGRESS_EVERY`].
pub(crate) struct Progressor {
    tty: bool,
    last: String,
    last_print: Option<Instant>,
}

impl Progressor {
    pub(crate) fn new() -> Progressor {
        Progressor {
            tty: std::io::stderr().is_terminal(),
            last: String::new(),
            last_print: None,
        }
    }

    /// One reading of the job: printed when it changed and is due, and a
    /// tty's line ended once the job is terminal.
    pub(crate) fn report(&mut self, p: &Progress) {
        let terminal = p.job.state.is_terminal();
        let line = progress_line(p);
        let due = self
            .last_print
            .is_none_or(|t| t.elapsed() >= PROGRESS_EVERY);
        if line != self.last && (self.tty || terminal || due) {
            if self.tty {
                eprint!("\r\x1b[2K{line}");
                std::io::stderr().flush().ok();
            } else {
                eprintln!("{line}");
            }
            self.last = line;
            self.last_print = Some(Instant::now());
        }
        if terminal && self.tty {
            eprintln!();
        }
    }
}

/// One line of progress: a parent reports its children (`30/112 done, 82
/// waiting, next in ~343s (limiter hold on stash)`), any other job its
/// own state and ETA.
fn progress_line(p: &Progress) -> String {
    let job = &p.job;
    // The account the job runs as, on every line: with several accounts
    // known a job verb takes the one logged in (C51), and the caller
    // should see which that was before reading what came back.
    let id = match &job.account {
        Some(account) => format!("{} ({account})", job.id),
        None => job.id.to_string(),
    };
    if let Some(t) = p.children.as_ref().filter(|t| t.total > 0) {
        return children_line(&id, t);
    }
    match (job.state, job.eta_seconds) {
        (JobState::Waiting, Some(eta)) if eta > 0 && job.retries > 0 => {
            format!("job {id}: got a 429, retry {} in ~{eta}s...", job.retries)
        }
        (JobState::Waiting, Some(eta)) if eta > 0 => {
            format!("job {id}: rate limited, starting in ~{eta}s...")
        }
        (state, _) => format!("job {id}: {state}"),
    }
}

/// A parent's line: its children counted, the next limiter hold named.
fn children_line(id: &str, t: &ChildTally) -> String {
    let mut line = format!("job {id}: {}/{} done", t.done, t.total);
    if t.failed > 0 {
        line.push_str(&format!(", {} failed", t.failed));
    }
    if t.cancelled > 0 {
        line.push_str(&format!(", {} cancelled", t.cancelled));
    }
    if t.running > 0 {
        line.push_str(&format!(", {} running", t.running));
    }
    if t.waiting > 0 {
        line.push_str(&format!(", {} waiting", t.waiting));
    }
    if let Some((eta, kind)) = &t.next {
        line.push_str(&format!(", next in ~{eta}s (limiter hold on {kind})"));
    }
    line
}

/// At most this many failed children are expanded inline; the rest are
/// counted and `acq jobs` named.
const FAILED_CHILDREN_SHOWN: usize = 10;

/// The text report for an `apply` parent (rule 4 of the legibility
/// ruling): success as one line with the child range; a failure in the
/// daemon's own words, then, when its tally (C23) lists failed
/// children, one line per failed child — its id, kind, target, and the
/// error that names its evidence. The CLI composes no count of its own:
/// the sentence is the daemon's, the rows the tally's.
pub(crate) async fn report_apply(client: &mut Client, id: u64, outcome: &Outcome) -> String {
    let Outcome::Failure {
        children: Some(children),
        ..
    } = outcome
    else {
        return apply_report(id, outcome, &[]);
    };
    // The children as the daemon lists them this lifetime; after a
    // restart only their ids (from the tally) remain.
    let listed: Vec<JobInfo> = list(client)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|j| j.parent == Some(id) && j.state == JobState::Failed)
        .collect();
    let mut rows = Vec::new();
    for cid in children.failed_ids.iter().take(FAILED_CHILDREN_SHOWN) {
        let label = match listed.iter().find(|j| j.id == *cid) {
            Some(j) => format!("{} {}", j.kind, j.target()),
            None => String::new(),
        };
        let cause = match client.request(&Request::Result { id: *cid }).await {
            Ok(Response::Result {
                outcome: Outcome::Failure { error, .. },
                ..
            }) => error,
            _ => format!("(acq result {cid})"),
        };
        rows.push(FailedChild {
            id: *cid,
            label,
            cause,
        });
    }
    apply_report(id, outcome, &rows)
}

/// One failed child of a parent, as the report lists it.
pub(crate) struct FailedChild {
    pub id: u64,
    /// Its kind and target, empty when the daemon no longer lists it.
    pub label: String,
    /// Its own error, or where to read it.
    pub cause: String,
}

/// [`report_apply`]'s text from what the daemon said and the rows
/// fetched for the failed children: no daemon is asked here.
pub(crate) fn apply_report(id: u64, outcome: &Outcome, rows: &[FailedChild]) -> String {
    match outcome {
        Outcome::Success { payload } => {
            let requests = payload["requests"].as_u64().unwrap_or(0);
            let done = payload["children"]["done"].as_u64().unwrap_or(0);
            let ids: Vec<u64> = payload["child_jobs"]
                .as_array()
                .map(|a| a.iter().filter_map(serde_json::Value::as_u64).collect())
                .unwrap_or_default();
            let range = match (ids.iter().min(), ids.iter().max()) {
                (Some(a), Some(b)) if a == b => format!(" (job {a})"),
                (Some(a), Some(b)) => format!(" (jobs {a}–{b})"),
                _ => String::new(),
            };
            format!(
                "job {id} done: {requests} request{}, {done} done{range}\n",
                if requests == 1 { "" } else { "s" }
            )
        }
        Outcome::Failure { error, children } => {
            let mut out = format!("job {id} failed: {error}\n");
            let w1 = rows
                .iter()
                .map(|r| format!("job {}", r.id).len())
                .max()
                .unwrap_or(0);
            let w2 = rows
                .iter()
                .map(|r| r.label.chars().count())
                .max()
                .unwrap_or(0);
            for r in rows {
                let job = format!("job {}", r.id);
                out.push_str(&format!("  {job:<w1$}  {:<w2$}  {}\n", r.label, r.cause));
            }
            let failed = children.as_ref().map_or(0, |c| c.failed_ids.len());
            if failed > FAILED_CHILDREN_SHOWN {
                out.push_str(&format!(
                    "  and {} more failed: acq jobs\n",
                    failed - FAILED_CHILDREN_SHOWN
                ));
            }
            out
        }
        Outcome::Cancelled => format!("job {id} was cancelled\n"),
    }
}

async fn print_result(client: &mut Client, id: u64, json: bool) -> Result<()> {
    let result = result_live(client, id).await?;
    // An apply parent's payload is a child-id list and a tally; a person
    // wants the report, not the array. Status may be gone after a daemon
    // restart; then an apply prints without its report.
    if !json && client.status(id).await.is_ok_and(|job| job.kind == "apply") {
        print!("{}", report_apply(client, id, &result.outcome).await);
        return match result.outcome {
            Outcome::Success { .. } => Ok(()),
            _ => Err(AlreadyReported.into()),
        };
    }
    print_outcome(&result, json)
}

/// A terminal job's outcome, from the daemon or from the ledger on disk.
/// JSON is the layer's document, the outcome with the job's id and
/// account beside it (additive, C53): a blocking verb has no other way to
/// say which job it was or whose. Only a success exits 0 — a failed or
/// cancelled job left nothing on stdout to use, so the exit says so in
/// both modes (C11); the JSON outcome is the report, and main adds no
/// second message.
fn print_outcome(result: &JobResult, json: bool) -> Result<()> {
    let id = result.job_id;
    if json {
        println!("{}", serde_json::to_string_pretty(result)?);
        return match result.outcome {
            Outcome::Success { .. } => Ok(()),
            _ => Err(AlreadyReported.into()),
        };
    }
    match &result.outcome {
        Outcome::Success { payload } => {
            println!("{}", serde_json::to_string_pretty(payload)?);
            Ok(())
        }
        Outcome::Failure { error, .. } => bail!("job {id} failed: {error}"),
        Outcome::Cancelled => bail!("job {id} was cancelled"),
    }
}

/// Finished jobs `acq jobs` lists without `--all` (C53: more are
/// counted).
const FINISHED_LISTED: usize = 10;

/// `acq jobs`: every open job, the most recent finished ones, and the
/// summary over all of them — the answer to "what is running, what
/// waits, when is it done" is never below a screen of history.
fn print_jobs(jobs: &[JobInfo], all: bool) {
    let finished = jobs.iter().filter(|j| j.state.is_terminal()).count();
    let hidden = if all {
        0
    } else {
        finished.saturating_sub(FINISHED_LISTED)
    };
    // The daemon lists by id: the earliest finished are the ones left out.
    let mut skip = hidden;
    let shown: Vec<JobInfo> = jobs
        .iter()
        .filter(|j| {
            if j.state.is_terminal() && skip > 0 {
                skip -= 1;
                return false;
            }
            true
        })
        .cloned()
        .collect();
    if !jobs.is_empty() {
        print_rows(&shown);
    }
    print_summary(jobs);
    if hidden > 0 {
        println!("{hidden} earlier finished jobs not listed (`acq jobs --all`)");
    }
}

fn print_table(jobs: &[JobInfo]) {
    if jobs.is_empty() {
        println!("no jobs");
        return;
    }
    print_rows(jobs);
    print_summary(jobs);
}

fn print_rows(jobs: &[JobInfo]) {
    // Widths from the data: a route label or an account name must never
    // push the columns out of line or be cut.
    let rows: Vec<[String; 9]> = jobs
        .iter()
        .map(|job| {
            [
                job.id.to_string(),
                job.parent.map(|p| p.to_string()).unwrap_or_default(),
                job.kind.clone(),
                job.target(),
                if job.retries > 0 {
                    format!("{} ↻{}", job.state, job.retries)
                } else {
                    job.state.to_string()
                },
                job.priority.to_string(),
                job.account.clone().unwrap_or_else(|| "-".into()),
                job.submitted_by.clone(),
                job.eta_seconds
                    .map(|s| format!("~{s}s"))
                    .unwrap_or_default(),
            ]
        })
        .collect();
    let heads = [
        "id", "parent", "kind", "target", "state", "prio", "account", "by", "eta",
    ];
    let mut w = [0usize; 9];
    for (i, h) in heads.iter().enumerate() {
        w[i] = h.len();
    }
    for r in &rows {
        for (i, cell) in r.iter().enumerate() {
            w[i] = w[i].max(cell.chars().count());
        }
    }
    let line = |cells: [&str; 9]| {
        format!(
            "{:>w0$}  {:>w1$}  {:<w2$}  {:<w3$}  {:<w4$}  {:>w5$}  {:<w6$}  {:<w7$}  {}",
            cells[0],
            cells[1],
            cells[2],
            cells[3],
            cells[4],
            cells[5],
            cells[6],
            cells[7],
            cells[8],
            w0 = w[0],
            w1 = w[1],
            w2 = w[2],
            w3 = w[3],
            w4 = w[4],
            w5 = w[5],
            w6 = w[6],
            w7 = w[7],
        )
    };
    println!("{}", line(heads).trim_end());
    for r in &rows {
        println!(
            "{}",
            line([
                &r[0], &r[1], &r[2], &r[3], &r[4], &r[5], &r[6], &r[7], &r[8]
            ])
            .trim_end()
        );
    }
}

/// The last line of a job table: the counts by state, and for what waits
/// when the next starts and when the last does — a start, never an end:
/// how long a request then takes is not predicted.
fn print_summary(jobs: &[JobInfo]) {
    if jobs.is_empty() {
        println!("no jobs");
        return;
    }
    let count = |s: JobState| jobs.iter().filter(|j| j.state == s).count();
    let mut parts = vec![format!("{} running", count(JobState::Running))];
    let waiting = count(JobState::Waiting);
    let etas = || {
        jobs.iter()
            .filter(|j| j.state == JobState::Waiting)
            .filter_map(|j| j.eta_seconds)
            .filter(|e| *e > 0)
    };
    parts.push(match (etas().min(), etas().max()) {
        (Some(next), Some(last)) if last > next => {
            format!("{waiting} waiting (next starts in ~{next}s, the last in ~{last}s)")
        }
        (Some(next), _) => format!("{waiting} waiting (next starts in ~{next}s)"),
        _ => format!("{waiting} waiting"),
    });
    parts.push(format!("{} done", count(JobState::Done)));
    let failed = count(JobState::Failed);
    if failed > 0 {
        parts.push(format!("{failed} failed"));
    }
    let cancelled = count(JobState::Cancelled);
    if cancelled > 0 {
        parts.push(format!("{cancelled} cancelled"));
    }
    println!(
        "{} job{}: {}",
        jobs.len(),
        if jobs.len() == 1 { "" } else { "s" },
        parts.join(", ")
    );
}

/// Re-render the job table once a second until every listed job is terminal.
async fn watch_table_until_done(client: &mut Client, ids: &[u64]) -> Result<()> {
    let mut printed_lines = 0usize;
    loop {
        let jobs = list(client).await?;
        // Move the cursor up over the previous render and redraw in place.
        if printed_lines > 0 {
            print!("\x1b[{printed_lines}A");
        }
        let mine: Vec<JobInfo> = jobs.into_iter().filter(|j| ids.contains(&j.id)).collect();
        for _ in 0..printed_lines {
            println!("\x1b[2K");
        }
        if printed_lines > 0 {
            print!("\x1b[{printed_lines}A");
        }
        print_table(&mine);
        printed_lines = mine.len() + 1;
        if mine.iter().all(|j| j.state.is_terminal()) {
            let done = mine.iter().filter(|j| j.state == JobState::Done).count();
            println!("\nall jobs finished ({done}/{} done)", mine.len());
            return Ok(());
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_plan_conflicts_with_the_execute_selectors() {
        // `--plan` compiles the stored policy; --all/--tabs/--deep are the
        // ad-hoc execution path. Mixing them must be a parse error, not a
        // silently ignored flag.
        for bad in [
            vec!["acq", "refresh", "--plan", "--all"],
            vec!["acq", "refresh", "--plan", "--tabs", "a,b"],
            vec!["acq", "refresh", "--plan", "--deep"],
        ] {
            assert!(Cli::try_parse_from(&bad).is_err(), "{bad:?} must not parse");
        }
        assert!(Cli::try_parse_from(["acq", "refresh", "--plan"]).is_ok());
        assert!(Cli::try_parse_from(["acq", "refresh", "--plan", "--league", "Hardcore"]).is_ok());
        assert!(Cli::try_parse_from(["acq", "policy", "set", "{}", "--if-revision", "4"]).is_ok());
    }

    #[test]
    fn price_set_takes_a_type_then_an_optional_amount_and_currency() {
        for ok in [
            vec!["acq", "price", "set", "item/i1", "exact", "12.5", "chaos"],
            vec![
                "acq",
                "price",
                "set",
                "tab/pc/t1",
                "b/o",
                "1/5",
                "divine",
                "--if-revision",
                "2",
            ],
            vec!["acq", "price", "set", "item/i1", "skip"],
            vec!["acq", "price", "clear", "item/i1", "--if-revision", "3"],
        ] {
            assert!(Cli::try_parse_from(&ok).is_ok(), "{ok:?} must parse");
        }
        // The value's words are positional and at most three: a fourth is
        // a parse error, never a silently dropped word.
        assert!(
            Cli::try_parse_from([
                "acq", "price", "set", "item/i1", "exact", "1", "chaos", "extra"
            ])
            .is_err()
        );
        assert!(Cli::try_parse_from(["acq", "price", "set", "item/i1"]).is_err());
    }

    #[test]
    fn refresh_apply_conflicts_with_every_other_mode_and_owns_max_requests() {
        // `--apply` executes a plan; the ad-hoc selectors and `--plan`
        // are different modes, and `--max-requests` is meaningless
        // without an apply to budget.
        for bad in [
            vec!["acq", "refresh", "--apply", "--all"],
            vec!["acq", "refresh", "--apply", "--tabs", "a,b"],
            vec!["acq", "refresh", "--apply", "--deep"],
            vec!["acq", "refresh", "--apply", "--plan"],
            vec!["acq", "refresh", "--max-requests", "5"],
        ] {
            assert!(Cli::try_parse_from(&bad).is_err(), "{bad:?} must not parse");
        }
        assert!(Cli::try_parse_from(["acq", "refresh", "--apply"]).is_ok());
        assert!(Cli::try_parse_from(["acq", "refresh", "--apply=plan.json"]).is_ok());
        assert!(Cli::try_parse_from(["acq", "refresh", "--apply=-"]).is_ok());
        assert!(Cli::try_parse_from(["acq", "refresh", "--apply", "--max-requests", "5"]).is_ok());
        // The optional value takes `=` only, so a following flag can never
        // be swallowed as the FILE.
        let cli = Cli::try_parse_from(["acq", "refresh", "--apply", "--json"]).unwrap();
        match cli.cmd {
            Cmd::Refresh { apply, .. } => assert_eq!(apply.as_deref(), Some("")),
            other => panic!("parsed into the wrong command: {}", other_name(&other)),
        }
    }

    /// The unreadable policy's remedy names both flags the retry needs:
    /// the merge would read the row it cannot.
    #[test]
    fn the_unreadable_policy_remedy_names_a_whole_write_at_its_revision() {
        let words = fault_words(&Fault::UnreadablePolicy {
            revision: 3,
            detail: "newer".into(),
        });
        assert!(words.contains("`--replace --if-revision 3`"), "{words}");
        assert!(words.contains("`acq policy show`"), "{words}");
    }

    fn one_failed_child() -> (acquisition_protocol::job::Children, Vec<FailedChild>) {
        (
            acquisition_protocol::job::Children {
                done: 2,
                failed: 1,
                cancelled: 0,
                failed_ids: vec![7],
            },
            vec![FailedChild {
                id: 7,
                label: "stash Standard/t9".into(),
                cause: "GET /stash/Standard/t9 returned 404".into(),
            }],
        )
    }

    /// The review's case: a restart's deferred failure (the sentence the
    /// daemon installs mid fan-out) with a failed child keeps its warning
    /// and its remedy, and the child's row follows it.
    #[test]
    fn a_deferred_parent_failure_keeps_its_words_beside_a_failed_childs_row() {
        let (tally, rows) = one_failed_child();
        let error = "interrupted by a daemon restart mid fan-out: the children submitted \
                     before the restart ran (their responses are in the store), but the full \
                     child set is unknown — resubmit to complete it";
        let outcome = Outcome::Failure {
            error: error.into(),
            children: Some(tally),
        };
        let text = apply_report(4, &outcome, &rows);
        assert!(
            text.starts_with(&format!("job 4 failed: {error}\n")),
            "{text}"
        );
        assert!(text.contains("resubmit"), "{text}");
        assert!(
            text.contains("job 7  stash Standard/t9  GET /stash/Standard/t9 returned 404"),
            "{text}"
        );
    }

    /// The ordinary case: the daemon's count sentence once, then the rows;
    /// the CLI composes no count of its own.
    #[test]
    fn a_failed_apply_prints_the_daemons_sentence_once_and_the_rows() {
        let (tally, rows) = one_failed_child();
        let outcome = Outcome::Failure {
            error: "1 of 3 child jobs failed, 2 done".into(),
            children: Some(tally),
        };
        let text = apply_report(4, &outcome, &rows);
        assert_eq!(
            text,
            "job 4 failed: 1 of 3 child jobs failed, 2 done\n  \
             job 7  stash Standard/t9  GET /stash/Standard/t9 returned 404\n"
        );
        assert_eq!(text.matches("child jobs failed").count(), 1);
    }

    fn other_name(_: &Cmd) -> &'static str {
        "not refresh"
    }

    #[test]
    fn plan_takes_an_optional_file_and_expand_needs_it() {
        assert!(Cli::try_parse_from(["acq", "refresh", "--plan=plan.json"]).is_ok());
        assert!(Cli::try_parse_from(["acq", "refresh", "--plan", "--expand"]).is_ok());
        assert!(Cli::try_parse_from(["acq", "refresh", "--expand"]).is_err());
        let cli = Cli::try_parse_from(["acq", "refresh", "--plan", "--json"]).unwrap();
        match cli.cmd {
            Cmd::Refresh { plan, .. } => assert_eq!(plan.as_deref(), Some("")),
            other => panic!("parsed into the wrong command: {}", other_name(&other)),
        }
    }
}
