//! The command tree: every verb, flag and help string `acq` takes, as
//! clap parses them; `CLI-REFERENCE.md` is generated from it. Parsing
//! only: `run.rs` dispatches what this parses.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use acquisition_protocol::protocol::ListState;
use acquisition_protocol::realm::Realm;
use clap::{Parser, Subcommand};

use crate::search_cmd;

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
pub(crate) struct Cli {
    /// Emit structured JSON instead of human-readable output: one
    /// document on stdout, a failure included: `{"error", "kind"}`, and
    /// where it has them `offers` (each `{request, says}`, a command line
    /// to run as printed, its words unquoted beside it as `argv` for a
    /// program to run with no shell, or a request `acq search --request`
    /// reads),
    /// `readings`, `next` (a person's act) and the kind's own fields. A
    /// job verb's is `{"outcome", "payload", "job_id", "account"}`, and a
    /// failed job's carries `error` in place of `payload`; without the
    /// flag it prints the payload alone.
    #[arg(long, global = true)]
    pub(crate) json: bool,
    /// Which account to act as: a username (with or without `#…`) or uuid,
    /// named each time: nothing remembers it. With none, a verb takes the
    /// sole account known on disk (`acq accounts`); with more than one, it
    /// refuses and lists them (`ambiguous_account`), a job verb before any
    /// daemon is reached (a login names none, and a logout ends the one
    /// session logged in).
    /// With accounts known, a name that is none of them is refused as
    /// `unknown_account`; with none known, as `not_logged_in`. A job verb
    /// also needs the account logged in now (`not_logged_in` where it is
    /// not); a verb that reads the store needs only its store. A job's
    /// progress line and its JSON name the account it ran as. The saved
    /// searches are the world's, not an account's: `acq saved` needs no
    /// login and uses an account only to resolve a new search's realm.
    #[arg(long, global = true)]
    pub(crate) account: Option<String>,
    #[command(subcommand)]
    pub(crate) cmd: Cmd,
}

#[derive(Subcommand)]
pub(crate) enum Cmd {
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
    /// The web UI: a page over loopback that renders the documents these
    /// verbs answer — the search as a form shaped like the trade site's, or a
    /// trade site search's URL, id or query JSON run as `search --trade`
    /// runs it, its answer as cards with each item's art or a table on
    /// asking, one item, and the refresh
    /// loop (a league's tabs and characters to tick, the plan with its
    /// quote, apply, the jobs) — and the saved searches, the world's,
    /// whoever saved them: kept from any answer under its query, listed,
    /// opened, renamed and cleared, as `acq saved` writes them. The pages
    /// read the store as `search`, `show` and `tabs` do; a refresh or an
    /// apply reaches the daemon as `refresh` does. Binds 127.0.0.1 on a random port; the URL it
    /// prints carries a key no other page knows, and the server answers
    /// nothing without it. Runs until Ctrl-C. The item art is fetched by
    /// your browser from GGG's image host; a page reads whole without it.
    /// With no account on disk and none named (no --account) it starts
    /// all the same, every page saying to log in
    /// with `acq auth`; the first account found after is the one it
    /// serves until it stops. An account named that cannot be picked
    /// refuses before it starts.
    Web {
        /// Print the URL alone and open no browser. With --json the URL
        /// is `{"url"}`, one line, either way.
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
    /// List the account's leagues: one a line, with its realm, its category
    /// and when it started (`--json`: GGG's answer whole). A new league is
    /// listed here the day it opens. `--stored` reads the list last fetched
    /// instead, with no daemon: the newest start first, each league beside
    /// when it started, whether the sync policy names it and what the store
    /// holds of it, ten unless `--expand`.
    Leagues {
        /// Read the stored list (the latest fetch) with no daemon, the
        /// newest start first, each league beside its start, the policy's
        /// word and its tabs and characters on disk.
        #[arg(long)]
        stored: bool,
        /// With `--stored`, list every league, not ten.
        #[arg(long, requires = "stored")]
        expand: bool,
    },
    /// Accounts this machine has logged into (the store's index; no
    /// daemon): ten, --expand every one, the count of all of them beside.
    /// `--json` is one document: `provider`, `total`, `accounts` (the
    /// listed) and `left_out`.
    Accounts {
        /// List every account.
        #[arg(long)]
        expand: bool,
    },
    /// Tabs of a league, from the shared store (no daemon round-trip):
    /// the counts first, then every tab with its type, items, age and id.
    /// A league of more than ten tabs is counted by type with its ten
    /// largest tabs instead; --limit lists more or fewer, --expand every
    /// one. --type and
    /// --never-fetched select the tabs listed — the never fetched of one
    /// type by name, which a refresh takes — under the league's counts.
    /// `--json` is the same document: the counts, `by_type`, and `rows`,
    /// the tabs listed, with `left_out` counting the rest.
    Tabs {
        /// List this many tabs in place of ten: the most items first
        /// where some are left out, else in the listing's order.
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
        /// List every tab of the league, or of the selection.
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
    /// "kind"}`, with `readings` where the query could be read otherwise
    /// and `offers`, each `{request, says}`, where a way on is a request
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
    /// Refresh tabs and characters. `--tabs`, `--all` and `--characters`
    /// name a selection, compiled into a plan, printed and applied at
    /// once: each named tab or character fetched however fresh, under
    /// `--deep` a map or unique tab's pages on record with it.
    /// Selection is explicit — there is no default. Keeping a league up to date is the
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
`acq refresh --apply --json` is the apply job's result as `acq result --json` gives it (its times, a failure's \
`kind` and `offers`) with `store_changes` beside it; `acq store events --summary --json` is the per-location \
summary.")]
    Refresh {
        /// Every tab in the league (folder children included, folders
        /// not), and no character: `--characters` names those.
        #[arg(long, conflicts_with = "tabs")]
        all: bool,
        /// Comma-separated tab ids.
        #[arg(long, value_delimiter = ',')]
        tabs: Vec<String>,
        /// Comma-separated character ids, as `acq store characters`
        /// lists them, with the realm's character listing fetched beside
        /// them; alone, or with --tabs or --all.
        #[arg(long, value_delimiter = ',')]
        characters: Vec<String>,
        /// With --tabs or --all: a named map or unique tab's pages on record
        /// are fetched with it.
        #[arg(long)]
        deep: bool,
        /// Compile the stored sync policy (`acq policy`) into the explicit
        /// action set and print it — nothing is submitted or sent. A
        /// running daemon adds its read-only quote. With --json, prints
        /// the serializable plan envelope. `--plan=FILE` (or `=-` for
        /// stdin) renders a reviewed envelope instead, through the same
        /// renderer, with the quote it carries.
        #[arg(long, value_name = "FILE", num_args = 0..=1, require_equals = true,
              default_missing_value = "", conflicts_with_all = ["all", "tabs", "characters", "deep"])]
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
              default_missing_value = "", conflicts_with_all = ["all", "tabs", "characters", "deep", "plan"])]
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
        #[arg(help = submit_kind_help())]
        kind: String,
        /// JSON params, as the verb of the same name sends them:
        /// characters `{"realm"}`, character `{"name", "realm"}`, stashes
        /// `{"league", "realm"}`, stash `{"id", "sub", "deep", "league",
        /// "realm"}`, sleep `{"seconds"}` (0 to 3600); realm defaults to pc
        /// and league to Standard. A name, league or id is one path segment of at most
        /// 255 bytes: a slash, a control character, or only dots and spaces before any
        /// `;` is refused; a stash needs its id. `acq status <id> --json` shows a job's
        /// params.
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
    /// Everything a bug report needs, in one document to paste into the
    /// issue: the build and its platform, the daemon as `daemon status`
    /// reports it (with none running, what the queue on disk holds), each
    /// account's store counts as `store status` prints them, the rails,
    /// then the last lines of the send journal and of the daemon log.
    /// Account names and uuids, the store and log directories, the home
    /// directory and anything shaped like a token are replaced by
    /// placeholders that still tell two accounts apart (`<account 1>`,
    /// `<store>`, `~`, `<token 1>`); character, league and tab names are
    /// kept. Observes only: never starts a daemon or changes the store.
    /// Exits 0 whatever it finds; a part it could not read says why in
    /// its place. `--json` is the whole document, with `redactions`
    /// counting what was replaced.
    Diagnose {
        /// How many of the journal's and the log's last lines to carry
        /// (default 50 of the journal, 100 of the log).
        #[arg(long, value_name = "N")]
        lines: Option<usize>,
    },
    /// The live jobs: id, parent, kind, target (from params), state
    /// (`↻n` counts 429 re-queues), priority, account, submitter, when
    /// (the latest of submitted, started and finished, as an age), ETA.
    /// Each row's JSON carries `submitted_at`, `started_at` and
    /// `finished_at`, each absent until it happens and with its age in
    /// words beside it (`submitted_at_age`, …).
    /// The ten newest waiting and running jobs are listed, and the ten
    /// newest finished ones (newest by id: the order they were submitted
    /// in, not the order they run, which priority decides); `--state`
    /// narrows the list to one of the two. The last line counts every job
    /// the daemon lists, whatever `--state` says, and says when the next
    /// waiting job starts and when the last one does, and what was left
    /// out; `--all` lists every one.
    /// `--json` is the same document: `running: true`, the counts
    /// (`total`, `by_state`, the starts), `rows`, and `open_left_out` and
    /// `finished_left_out`; the finished ones are of this daemon's work
    /// and of what it restored from the daemon before it, read from disk.
    /// Observes only: with no daemon running it never spawns one, and
    /// answers from the ledger on disk instead — the non-terminal
    /// rows as the last daemon left them, ten unless `--all`, without
    /// ETAs, and how long ago they were written; `--json`: `running:
    /// false`, `persisted` (`jobs`, `written_at`, `total`, `waiting`,
    /// `recorded_running`, `left_out`; `null` when this shell's world has
    /// no queue yet). A job command spawns a successor that restores them;
    /// `cancel` then acts on them.
    Jobs {
        /// Subscribe, print the queue, then every job-state change as it
        /// happens; the queue is printed again after a missed-events signal,
        /// and the watch ends when the daemon stops.
        #[arg(long)]
        watch: bool,
        /// List every job: every open one, not the newest ten, and every
        /// finished job of this daemon's work and of what it restored from
        /// the daemon before it, read from disk, not the newest ten; with
        /// --watch, in every snapshot. An earlier daemon's other finished
        /// jobs are not listed; `acq status <id>` and `acq result <id>`
        /// still answer them from disk while retention keeps them.
        #[arg(long)]
        all: bool,
        /// Which jobs to list: `open` (waiting or running), `finished`
        /// (done, failed or cancelled) or `all`, both, the default. With
        /// no daemon running, the ledger's open rows whatever it says.
        #[arg(long, default_value = "all", value_parser = acquisition_verbs::list_state)]
        state: ListState,
    },
    /// One job's state and ETA, and when it was submitted, started and
    /// finished, as ages. A large ETA is the limiter holding, not a
    /// hang: holds can reach 300 s plus the timing bucket. A finished job
    /// is answered from disk while retention keeps it, whichever daemon
    /// ran it. With no daemon running, the job as the ledger on disk
    /// records it (never spawns).
    Status { id: u64 },
    /// A finished job's payload or error, and on stderr when it was
    /// submitted, started and finished (`--json`: `submitted_at`,
    /// `started_at`, `finished_at`, each with its `_age`). A failure
    /// says its `kind` — not_found (GGG has nothing there: the listing
    /// that names it is fetched again), upstream (GGG's answer could not
    /// be used: the same job, later), rate_limited (the same job once the
    /// hold has passed), blocked (Cloudflare in GGG's place: submit
    /// nothing again), not_retried (GGG's 403 or 503: a person looks
    /// before anything like it is sent again),
    /// not_recorded (what it fetched is kept with the result), children
    /// (each failed child's result), not_logged_in (a login, a person's),
    /// interrupted (the same job again) or internal — and one line of the
    /// way on, `--json`'s `offers` the commands it names. Kept on disk
    /// (7 days done, 30 failed, by default) and answered whether or not a
    /// daemon runs; a
    /// job still waiting or running is an error unless --wait. A failed
    /// or cancelled job exits 1. A failed fetch's refused body is in
    /// `acq store refused <id>`, not here.
    Result {
        id: u64,
        /// Block until the job finishes, printing progress on stderr, its
        /// last line the job's times, then print the result: how a
        /// `--detach`ed job is collected.
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
    /// The saved searches (no daemon, no login): a name, a query's text
    /// and its scope, kept in the world's own file, not an account's, so
    /// any account runs one (`acq search --saved`). Each has an id, the
    /// identity, printed last; the name is a label two may share. A new
    /// search naming no realm takes the one realm the facts hold: the
    /// account's named with `--account`, else every known account's.
    /// `--json` is the document whole; a write's is its receipt, the
    /// row before it and the command that puts it back.
    Saved {
        #[command(subcommand)]
        cmd: Option<SavedCmd>,
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
pub(crate) enum PriceCmd {
    /// One line on the league's listing state and the next action (the
    /// default); --expand adds the game side, the rows and the basis.
    /// `--json`: the rows naming nothing, unreadable and of other leagues
    /// ten each beside their counts, every one under --expand.
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
    /// container's items summarized under it, each set counted and
    /// listed when ten or fewer (--limit lists that many of each,
    /// --expand them all). `--json`: each set ten by default beside its
    /// counts.
    Show {
        target: String,
        /// The league to read the target in. With neither --league nor
        /// --realm, the realm and league the facts hold the target in, so
        /// an address reads back as printed; held in several, it is
        /// refused with one command per place.
        #[arg(long)]
        league: Option<String>,
        /// The realm of an item or character target; a tab or substash
        /// address carries its own. Without it, as --league says.
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
        /// List this many of each item set, the first in listing order.
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
        #[arg(long)]
        expand: bool,
    },
    /// Listed items grouped by container: the first ten of the selection,
    /// counted whole on the first line (--limit lists N, --expand every
    /// one with its texts).
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
        /// Only the items of a game-side count `acq price status` prints:
        /// residue, a note or tab name that reads as a price where the
        /// trade site cannot see it (a character's item, a tab that is not
        /// public); invalid_note, a note that starts like a price and does
        /// not read as one; invalid_tab_name, an item whose tab's name
        /// starts like a price and does not read as one; skip, a game-side
        /// ~skip statement in a public tab.
        #[arg(long, value_name = "TEXT")]
        game_text: Option<String>,
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
        /// otherwise. 0 writes only where the target has no price of its
        /// own, and is refused naming the stored revision where it has.
        /// Without it the write replaces whatever is stored — though a
        /// write racing in between still conflicts rather than being
        /// clobbered.
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
pub(crate) enum SavedCmd {
    /// List the saved searches (the default): ten by name, each with its
    /// scope, its query, its revision and its id, the count of all beside;
    /// --limit lists N, --expand every one. A row this build cannot read
    /// is listed by its id and why.
    List {
        /// List this many, the first by name.
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
        /// List every saved search.
        #[arg(long)]
        expand: bool,
    },
    /// One saved search, by its id or its name: its scope, its query, its
    /// revision and who wrote it. A name several share is refused, each
    /// offered by its id.
    Show {
        #[arg(value_name = "ID|NAME")]
        search: String,
    },
    /// Save a search, or change one. A new one is `--name` and the query,
    /// with no `--id`: its realm is `--realm`, else the one the facts
    /// hold, the account's named with `--account` or else every known
    /// account's (`all` where they hold none; over several it is
    /// required), its membership `--membership`, else live. A change is
    /// `--id` and what it changes — a query, `--name` (a rename),
    /// `--realm`, `--membership` — the rest kept as the row stands. The
    /// query is stored as the language prints it, and refused before
    /// anything is written where it does not read. Prints what it
    /// replaced and the command that puts it back.
    Set {
        /// The saved search to change, by its id (`acq saved list`).
        #[arg(long, value_name = "ID")]
        id: Option<String>,
        /// With --id, only write over exactly this revision (what `acq
        /// saved show` printed when you reviewed it); refused naming the
        /// current one otherwise. 0 writes the whole search at the id only
        /// where none stands there, as an undo of a clear does. Without it
        /// the write replaces the revision that stands — though a write
        /// racing in between still conflicts rather than being clobbered.
        #[arg(long)]
        if_revision: Option<i64>,
        /// The name: one to sixty-four characters, no line break. A label,
        /// not an identity: two searches may share one.
        #[arg(long)]
        name: Option<String>,
        /// The query, as `acq search` takes it. One that starts with `-`,
        /// the language's not, takes a space inside its quotes.
        query: Option<String>,
        /// pc, xbox, sony, poe2, or all.
        #[arg(long)]
        realm: Option<String>,
        /// live, all or removed, as `acq search --membership` reads them.
        #[arg(long, value_name = "live|all|removed")]
        membership: Option<String>,
    },
    /// Remove one saved search. Prints what it removed and the command
    /// that puts it back.
    Clear {
        /// The saved search's id (`acq saved list`).
        #[arg(value_name = "ID")]
        id: String,
        /// Only clear exactly this revision; see `set`.
        #[arg(long)]
        if_revision: Option<i64>,
    },
}

#[derive(Subcommand)]
pub(crate) enum ShopCmd {
    /// Render the shop pages to stdout for pasting by hand: one
    /// link code per hand-priced item under its price's spoiler, one
    /// page spoiler labelled n of N, in pages of at most --size
    /// characters; what the
    /// game already lists is omitted and counted, an unobserved case is
    /// blocked and counted (an item in a substash, or socketed in
    /// another: the forum has no link form for it); the sync policy's
    /// coverage and freshness are reported, never enforced. A link's
    /// `Stash<n>` is the tab's rank among the tabs the website lists,
    /// folders and substashes left out, not the API's index; under each
    /// page's label, each `Stash<n>` its links write is named by its
    /// tab's name.
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
        /// Print one page's text alone on stdout, for the clipboard, and
        /// the render's lines (counts, facts, coverage, the page's label)
        /// on stderr; with --json, the page's own fields and beside them
        /// every key of the whole render but `pages`.
        #[arg(long)]
        page: Option<usize>,
        /// The whole policy table with each row's rule and count, and
        /// every item left off the page with its cell, those nothing
        /// applies to included. Without it they are counted: the --json
        /// document lists the first ten left off for a reason, the blocked
        /// first, and `left_out_total` counts them all; `posted` and each
        /// freshness list hold ten beside their counts the same way.
        #[arg(long)]
        expand: bool,
    },
}

#[derive(Subcommand)]
pub(crate) enum ReferenceCmd {
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
pub(crate) enum StoreCmd {
    /// Path, size, and row counts.
    Status,
    /// Characters on record (no daemon): id, address, league, freshness,
    /// live item count. The counts over every character first (the
    /// footer), then ten of them in the store's order, the highest level
    /// first in each league; --limit lists more or fewer, --expand every
    /// one. `--json` is the same document: the counts, and `rows`, the
    /// characters listed, with `left_out` counting the rest. `acq
    /// characters` (the job) fetches the list anew.
    Characters {
        /// Restrict to one realm (pc, xbox, sony, poe2); every realm otherwise.
        #[arg(long, value_parser = acquisition_verbs::realm)]
        realm: Option<Realm>,
        /// Restrict to one league; every league otherwise.
        #[arg(long)]
        league: Option<String>,
        /// List this many characters in place of ten.
        #[arg(long)]
        limit: Option<usize>,
        /// List every character of the selection.
        #[arg(long)]
        expand: bool,
    },
    /// Item events (added/moved/changed/removed) from recent ingests, the
    /// newest ten (--limit more or fewer): by default one line per
    /// location with counts (text) or the event list (--json); --expand
    /// alone picks the event list and --summary the locations in both
    /// modes, ten locations, --summary --expand every one. `--json` counts
    /// what it lists: the events `returned` and whether the limit was
    /// reached; the summary the events, the locations and each kind.
    Events {
        #[arg(long, default_value_t = acquisition_verbs::events::DEFAULT_HOURS)]
        hours: f64,
        /// The most events read, the newest first (ten when not given).
        #[arg(long)]
        limit: Option<usize>,
        /// Every event read, one per line (the JSON default); with
        /// --summary, every location.
        #[arg(long)]
        expand: bool,
        /// One line per location with counts (the text default), ten
        /// locations.
        #[arg(long)]
        summary: bool,
    },
    /// Bodies the store refused as malformed, kept verbatim as evidence:
    /// the list (newest first), or one body in full by its row id.
    /// `--json` lists them under `rows`, beside `returned` and `limit`.
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
pub(crate) enum PolicyCmd {
    /// Print the stored policy value and its revision (the default), and
    /// each realm's leagues on disk with whether the policy names them,
    /// ten a realm.
    Show {
        /// List every league on disk, not ten a realm.
        #[arg(long)]
        expand: bool,
    },
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
        /// with the current revision otherwise. 0 writes only where no
        /// policy is stored yet, and is refused with the stored revision
        /// where one is. Without it the write replaces whatever revision is
        /// currently stored — though a write racing in between still
        /// conflicts rather than being clobbered.
        #[arg(long)]
        if_revision: Option<i64>,
        /// Replace the whole policy: every realm the value does not name
        /// is removed.
        #[arg(long)]
        replace: bool,
    },
}

/// `acq submit`'s kind: the protocol's list, the words the daemon's
/// refusal of an unknown kind prints (C127), and what `apply` takes.
fn submit_kind_help() -> String {
    use acquisition_protocol::job::{MOCK_ONLY_KINDS, client_kinds};
    format!(
        "One a client may submit: {}. {} run against the mock alone and are refused in real \
mode. apply takes `{{\"jobs\": [{{\"kind\", \"params\"}}, …]}}`, the single requests a plan \
authorizes (stashes, stash, characters, character); `acq refresh --apply` sends a reviewed \
plan after the checks this skips",
        client_kinds(false),
        MOCK_ONLY_KINDS.join(", ")
    )
}

#[derive(Subcommand)]
pub(crate) enum AuthCmd {
    /// Show session state (login, token expiry, keyring health).
    Status,
    /// Verify the session actually works: forces a token round-trip through
    /// the provider instead of trusting local state. Exit code 1 on failure.
    Check,
    /// Drop the session and clear the keyring entry. With --account naming
    /// another known account, clear only that account's keyring entry. A
    /// keyring turned off (`ACQ_NO_KEYRING`) clears nothing and says so;
    /// the logout still succeeds. A clear the keyring refuses, or an
    /// account index that cannot be written, is a failure that says the
    /// session was dropped and what was not done, and the command that
    /// clears the entry.
    /// `--json`: `logged_out`, `account`, `keyring` — `cleared`; `off`,
    /// the keyring turned off and nothing cleared; `failed`, the session
    /// dropped but the keyring refused the clear, so the entry may
    /// remain, exit 1 (this build's daemon answers a refused clear with
    /// the error document instead, in words that say the same); or
    /// `unknown`, an older daemon that did not say. On a failure, the
    /// error document.
    Logout,
}

#[derive(Subcommand)]
pub(crate) enum DaemonCmd {
    /// The daemon running: pid, version, contract revision, the executable
    /// it runs from and its hash, provider, the world it serves
    /// (its canonical store root), uptime, the idle exit it was started with, connections, queue
    /// counts, policies learned since this daemon started, the socket,
    /// log and journal paths, the rails state (its sends since this
    /// daemon started), keyring health. Observes only: never spawns or
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
    /// The daemon waits up to 15 seconds for requests already sent to
    /// GGG to land and be recorded, sending nothing new, and this
    /// command waits with it until the daemon is gone. Queued
    /// jobs stay on disk and resume under the next one; a client's
    /// jobs are never cancelled by its leaving. `--json`:
    /// `stopped`, `pid`, `version`, `provider`, `compatible`, `socket`.
    Stop,
    /// Clear the rails halt: the live-test tripwire's, and the mark a
    /// Cloudflare answer in GGG's place leaves, which holds every daemon
    /// until cleared, and the halt this daemon keeps after Cloudflare
    /// could not reach GGG. A met send ceiling is not cleared: it belongs
    /// to the daemon's lifetime, and only a new daemon counts from zero
    /// again. After a trip, wait 360 s before clearing; a trip or a mark
    /// is evidence to read, and the next send goes to GGG unless the
    /// ceiling is met. With no daemon running, clears the halt persisted in
    /// this shell's world (`<store root>/<provider>/rails.json`) so the
    /// next daemon starts clear; a rejected sign-in's mark stays until
    /// `acq auth`.
    ResetTripwire,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_plan_conflicts_with_the_execute_selectors() {
        // `--plan` compiles the stored policy; --all/--tabs/--deep name a
        // selection, compiled and applied at once. Mixing them must be a parse error, not a
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
        // `--apply` executes a plan; the selection's flags and `--plan`
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
