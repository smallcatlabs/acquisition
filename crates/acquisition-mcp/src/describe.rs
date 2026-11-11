//! The server's own sections of `describe`'s index: the daemon's and
//! the store's tools, and pricing's, listed beside the search crate's
//! entries.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

/// What the job tools answer, read once by name (`describe` with names
/// `["daemon"]`, #83): the words their texts carried, cut. The document is
/// the verbs crate's; the words are this adapter's, since what this
/// server does about a daemon (never spawning one in real mode, never
/// replacing one) is its own (C13).
const DAEMON: [(&str, &str); 8] = [
    (
        "real mode",
        "in real-GGG mode the daemon must already be running: a human starts it from the CLI, which also does the login, and this server never spawns or replaces one there — it submits, applies and quotes only through one a human started; in mock mode it starts one into an empty socket",
    ),
    (
        "jobs",
        "an API request is a job: submit_job returns its id at once, and apply_plan the parent's when it submits one (an empty or fully spent plan submits none, `applied: false`); poll job_status, then read job_result. A waiting job with a large ETA is the rate limiter holding, not a hang: holds can reach about 5 minutes. A stash job for a tab the account's stored listing of its realm and league does not name, or names as gone, is refused at submit, the listing's age said and nothing sent; a `stashes` job fetches the listing again. With none of that league stored it is admitted. An `account` that is none of the accounts known is refused as unknown_account (with none known, not_logged_in), and one known with no session live as not_logged_in",
    ),
    (
        "no daemon",
        "list_jobs, job_status, job_result and daemon_status observe only: with no daemon running none of them spawns one, and each answers from the ledger on disk; list_jobs, job_status and daemon_status say `running: false` — list_jobs `persisted`, the open rows as the last daemon left them (without ETAs), and `written_at`, how old that picture is, `written_at_age` in words; job_status the job's row, with `updated_at_age` when it was written, and `next`, the command a person runs to start a daemon; daemon_status `persisted` (waiting, recorded_running, written_at and written_at_age) and `log`, the last daemon's log file in this world. `persisted` is null when this world has no queue yet; job_result answers a finished job's outcome as a running daemon would",
    ),
    (
        "job counts",
        "list_jobs with a daemon running: `running: true`, `total` every job this daemon holds (restored ones too), `by_state` them by state (running, waiting, done, failed, cancelled, a zero included), and `next_starts_in_seconds` and `last_starts_in_seconds` the soonest and the latest predicted start among the waiting (a start, never an end; absent when none is predicted); then `rows` in id order, the ten newest open jobs by id (newest by submission, not by when they run: priority decides that, so a running job can be left out) and the ten newest finished, or those of the one `state` names, `open_left_out` and `finished_left_out` counting the earlier ones, which `expand` lists. The counts are over every job whatever `state` names. With none running, `persisted` holds ten of the ledger's open rows as `jobs`, with `total`, `waiting`, `recorded_running` and `left_out` over every one",
    ),
    (
        "ledger",
        "a finished job is answered from the ledger on disk while retention keeps it, whichever daemon ran it, across restarts; its outcome there is final, and an unfinished one is refused as wrong_state with `next`; a job the ledger does not hold is refused as unknown_job with `next`. A job command from the CLI spawns a successor that restores the open ones",
    ),
    (
        "outcome",
        "job_result holds the payload or the error, with `job_id`, `account` and the job's times beside it (`submitted_at`, `started_at` and `finished_at`, each absent until it happens, a 429's re-queue keeping the first start, each with its `_age`), as job_status and list_jobs' rows carry them; a failure's `kind` is one of not_found (GGG has nothing there; the offer is the listing that names it: a tab's league `stashes`, a substash's parent `stash`), upstream (GGG's answer could not be used; the same job, later), rate_limited (the same job once the limiter's hold has passed), blocked (Cloudflare answered in GGG's place: nothing is offered and nothing should be sent again), not_retried (GGG's 403 or 503, never retried: nothing is offered; a human looks first, a login where it names an authorization error), not_recorded (what it fetched is in `payload`), children (each failed child's job_result), not_logged_in (a login, a human's), interrupted (the same job again) and internal, with its `offers`, each sent back as printed; a parent's failure carries its children's tally as `children` (done, failed, cancelled, failed_ids) and the first ten failed children with their kind, target and error as `failed_children`. Payloads can be large: the store tools read the same data",
    ),
    (
        "another daemon",
        "daemon_status `running: true, compatible: false`: a daemon of another contract, artifact, provider or world, reported by its identity alone — contract_matches, artifact_matches, provider_matches, world_matches; artifact_relation same_file, same_bytes for another copy, different, unhashable, no_sibling or unreported, against the acqd beside this server; `wanted` this server's own — and never replaced by this server. A compatible one adds its vitals: provider, uptime, idle_shutdown_seconds, connections, queue counts, rate-limit policies learned and `rails.sends` since this daemon started, the rails state with the journal path, keyring health and `log`, the file it opened",
    ),
    (
        "wedged",
        "daemon_status `running: true, responsive: false`, with `socket` and `lock_holder`, the pid the world's lock records: something accepted the connection and did not answer hello within 5 s — a wedged daemon, for a human to stop by hand",
    ),
];

/// What `store_status`, `tabs`, `characters` and `item_events` count, and
/// what an age's words beside a time say on every tool that carries them,
/// read once by name (`describe` with names `["store"]`, or `["ages"]`
/// alone, the store's own words): C146, amended 2026-10-27 — on the
/// MCP a count's gloss stands in the `describe` entry the tool's
/// description names. The words are the ones those descriptions carried,
/// whole, and the CLI footers' (C149: the counts a text prints stand in
/// its JSON under one set of words).
const STORE: [(&str, &str); 7] = [
    (
        "store counts",
        "store_status: the store file's path, size and row counts. `leagues` counts every league id seen across every league fetch, a union that never retires one, and `league_list` says whether a list was ever fetched (a `leagues` job fetches it); `league_list_latest` is the latest fetch alone: its `count`, `fetched_at` and that age in words, null and `never` before one; characters and tabs carry their league names whether or not it was. `refused_bodies` counts bodies the store would not take, which `store_refused` lists, `refused_latest_at` when the newest was fetched and `refused_latest_at_age` that age in words. `facts` says how old the facts are over every realm and league the store holds, in the keys a search's `scope` names them by: `fetched` and `never_fetched` locations (by type and league), `folders`, the `oldest_fetch` and `newest_fetch` with their `_age` and `_location`, and `list_seen`, when the oldest location list was seen. `withheld_responses` counts fetches of tabs or characters the latest list no longer named and `withheld_items` the items they carried: kept as evidence, in no search or count, nothing to do unless a list names the place again. `unlifted_items` counts item-shaped objects in characters' data under a key this build does not read as items, so no search counts them: GGG added a kind of item list, which a newer build reads. `granted_skills` counts the skill entries PoE2 characters' items grant — GGG shows a granted skill once in each place it appears, the item's sockets and the character's skills, so one skill can be two entries — kept with their characters and not counted as items, having no id of their own",
    ),
    (
        "tab counts",
        "tabs, counted first: how many tabs, fetched, never fetched and folders, how many items in tabs (characters apart: the `characters` tool with the league counts theirs), and the same by tab type (`by_type`, the most items first, each type's `never` its never fetched); then `rows`, the tabs listed — the ten with the most items, `limit` more or fewer, `expand` every one, in the listing's order where none is left out (`listed`: `all` or `most_items`). `left_out` counts the tabs not listed. `oldest_fetch` and `newest_fetch`, each with its `_age` and `_location`, are the league's tabs' extremes and `list_seen` when its stash listing was seen (`never` where it was not). `type` and `never_fetched: true` select the tabs listed — the never fetched of one type by name, which a refresh takes — under the league's counts, `selected` counting them",
    ),
    (
        "character counts",
        "characters, counted first over every character the selection holds (`realm` and `league` as asked, absent when not): `characters` how many; `fetched` those a full fetch (equipment and inventory) has filled; `empty_bodies` the fetched whose last full fetch carried no items (`fetched_items` 0); `never_fetched` those listed and never fetched, whose items no answer knows; `items` the live items held at every one. Then `rows`, the characters listed — ten in the store's order (by realm, league, then level, highest first), `limit` more or fewer, `expand` every one — and `left_out`, the characters not listed. On a row, `item_count` is the live items the store holds at the character now and `fetched_items` what the last full fetch's body carried (equipment and inventory, socketed gems not counted), null until one",
    ),
    (
        "event counts",
        "item_events reads the newest events of the last `hours`, at most `limit` (ten when not given): `returned` counts the events in `rows`, and `limit_reached` says it returned as many as `limit`, so the window may hold more (a larger `limit` reads them); the window's whole is not counted. With `summary: true` the same events by location: `events` counts the events summarized, the returned ones, `locations` every location they touched (a move touches two), `by_kind` the added, changed, moved and removed over every location; `rows` lists ten locations, the newest activity first, `expand` every one, and `left_out` counts the locations not listed. A summary of a window with no event (not one a `limit` of 0 cut) carries `newest_on_record`, the newest event the store holds at all (`at`, its words `at_age`; `at` null and `never` where none is on record): a larger `hours` reaches it",
    ),
    (
        "leagues held",
        "price_status, shop_render and tabs carry `leagues_held` beside the league they read, sync_policy one per realm the store holds: the realm's leagues with tabs or characters on disk, `leagues_total` all, `leagues` ten (the one read first; `expand` all), `left_out` the rest, each with `tabs`, `characters` and `named`, whether the policy names the league in that realm whatever it covers; false under `policy` `none` or `unreadable` and for characters with no league (`league` null); `policy_names_realm` false where the policy names other realms alone; sync_policy's `leagues_held_unavailable` says why where the facts file could not be read",
    ),
    (
        "league list",
        "leagues (`stored: true`): the list last fetched, `fetched_at` with its age (null and no leagues if never), `leagues_total` all, `leagues` ten, the newest `started_at` first, then those with none in the list's order (`expand` all), each with `realm` (null where it names none), `started_at` with its age, GGG's start (null where it gives none), `named` (the policy names it in that realm, or any realm where none; false under `policy` `none` or `unreadable`) and `held`, its tabs and characters on disk in that realm, summed where none",
    ),
    ("ages", acquisition_store::age::SAID),
];

/// The names `describe` answers beside the search crate's sections: what
/// the pricing tools, the job tools and the store's counts answer, which
/// are not the search's (C1: the search links no plan).
pub(crate) const OWN_SECTIONS: [&str; 3] = ["pricing", "daemon", "store"];

/// The own sections as words: `pricing, daemon and store`.
pub(crate) fn own_listed() -> String {
    match OWN_SECTIONS.split_last() {
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
        None => String::new(),
    }
}

/// The own sections' entries, in [`OWN_SECTIONS`]' order.
pub(crate) fn own_sections() -> [Vec<Entry>; 3] {
    let entries = |table: &[(&'static str, &'static str)]| {
        table
            .iter()
            .map(|&(name, what)| Entry {
                name,
                what: what.to_string(),
            })
            .collect()
    };
    [
        acquisition_plan::pricing::described()
            .into_iter()
            .map(|d| Entry {
                name: d.name,
                what: d.what,
            })
            .collect(),
        entries(&DAEMON),
        entries(&STORE),
    ]
}

/// One entry of an own section: a name and what it says.
#[derive(serde::Serialize)]
pub(crate) struct Entry {
    pub(crate) name: &'static str,
    pub(crate) what: String,
}

/// `describe`'s answer: the search crate's document for the names it
/// knows, beside the pricing model and the daemon entries where named or
/// where nothing is named. Flattened, so the language's keys keep the
/// order the crate declares (C115).
#[derive(serde::Serialize)]
pub(crate) struct Described {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub(crate) language: Option<acquisition_search::describe::Describe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pricing: Option<Vec<Entry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) daemon: Option<Vec<Entry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) store: Option<Vec<Entry>>,
}
