//! Frontend-side reads of the shared store: no daemon round-trip. The CLI
//! opens the same SQLite file the daemon writes; WAL makes that safe.

use std::path::Path;

use acquisition_protocol::realm::Realm;
use acquisition_store::{Endpoint, Store};
use acquisition_verbs::AccountRow;
use acquisition_verbs::events::LocationChanges;
use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

use crate::Ctx;

/// The selected account's facts (C12, C121): resolved and opened through
/// the layer, so no daemon is involved.
fn open(cx: &Ctx) -> Result<Store> {
    Ok(cx.account()?.facts()?)
}

/// After `auth status`: the other accounts the index knows.
pub fn print_other_accounts(cx: &Ctx, live: Vec<String>) -> Result<()> {
    let accounts = cx.frontend.accounts()?;
    let others: Vec<&AccountRow> = accounts
        .accounts
        .iter()
        .filter(|e| !live.contains(&e.username))
        .collect();
    if others.is_empty() {
        return Ok(());
    }
    println!(
        "other accounts: {}",
        others
            .iter()
            .map(|e| format!(
                "{}{}",
                e.username,
                if e.persisted { "" } else { " (not persisted)" }
            ))
            .collect::<Vec<_>>()
            .join(", ")
    );
    Ok(())
}

/// `auth status` with no daemon to ask: sessions live in a daemon, so
/// none is live, and what the index on disk says of each account is what
/// a daemon would start from — an account in the keyring is restored by
/// the next job command, one that is not needs `acq auth` again. An
/// observation: nothing is spawned (C10), and the answer is a state,
/// never an error.
pub fn auth_status_absent(cx: &Ctx, json: bool) -> Result<()> {
    let accounts = cx.frontend.accounts()?;
    if json {
        let rows: Vec<Value> = accounts
            .accounts
            .iter()
            .map(|e| {
                json!({ "username": e.username, "persisted": e.persisted,
                             "last_login": e.last_login, "uuid": e.uuid })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "running": false, "accounts": rows }))?
        );
        return Ok(());
    }
    println!("daemon is not running, so no session is live (it spawns on demand for job commands)");
    if accounts.accounts.is_empty() {
        println!(
            "no accounts known for {} — run `acq auth`",
            accounts.provider
        );
    }
    for e in &accounts.accounts {
        println!(
            "{}: {}",
            e.username,
            if e.persisted {
                "in the keyring — a job command restores the session"
            } else {
                "not persisted — `acq auth` logs in again"
            }
        );
    }
    Ok(())
}

/// Every account the index knows, with its store file (C12): the JSON is
/// the layer's rows, the array as it always was (C53).
pub fn accounts(cx: &Ctx, json: bool) -> Result<()> {
    let accounts = cx.frontend.accounts()?;
    if json {
        println!("{}", serde_json::to_string_pretty(&accounts.accounts)?);
        return Ok(());
    }
    if accounts.accounts.is_empty() {
        println!(
            "no accounts known for {} (run `acq auth`)",
            accounts.provider
        );
        return Ok(());
    }
    let now = acquisition_store::now();
    for e in &accounts.accounts {
        let size = std::fs::metadata(&e.store).map(|m| m.len()).unwrap_or(0);
        println!(
            "{:<24} last login {:<10} {:<14} store {:.1} MB{}",
            e.username,
            ago(now, Some(e.last_login)),
            if e.persisted {
                "in keyring"
            } else {
                "not persisted"
            },
            size as f64 / 1e6,
            e.uuid
                .as_deref()
                .map(|u| format!("  uuid {u}"))
                .unwrap_or_default()
        );
    }
    Ok(())
}

pub(crate) fn ago(now: i64, t: Option<i64>) -> String {
    match t {
        None => "never".into(),
        Some(t) => {
            let d = (now - t).max(0);
            if d < 90 {
                format!("{d}s ago")
            } else if d < 5400 {
                format!("{}m ago", d / 60)
            } else if d < 172_800 {
                format!("{}h ago", d / 3600)
            } else {
                format!("{}d ago", d / 86400)
            }
        }
    }
}

/// `xbox/` before a league in a message; nothing for pc, as on the wire.
pub(crate) fn realm_prefix(realm: Realm) -> String {
    match realm.segment() {
        Some(seg) => format!("{seg}/"),
        None => String::new(),
    }
}

/// A league with no tab known, and what lists its tabs: what `tabs` says
/// and the web UI's tabs page with it.
pub(crate) fn no_tabs_text(realm: Realm, league: &str, store: &Store) -> String {
    format!(
        "no tabs known for {}{league} in {} (run `acq stashes` or `acq refresh --all`)",
        realm_prefix(realm),
        store.path().display()
    )
}

/// `tabs`' first line: the league's counts, the answer first (C53).
pub(crate) fn tabs_head(
    listed: &acquisition_search::tabs::Tabs,
    realm: Realm,
    league: &str,
) -> String {
    let mut parts = vec![format!("{} fetched", listed.fetched)];
    if listed.never_fetched > 0 {
        parts.push(format!("{} never fetched", listed.never_fetched));
    }
    if listed.folders > 0 {
        parts.push(format!(
            "{} folder{} (never fetched)",
            listed.folders,
            if listed.folders == 1 { "" } else { "s" }
        ));
    }
    format!(
        "{} tabs in {}{league}: {}; {} items",
        listed.tabs,
        realm_prefix(realm),
        parts.join(", "),
        listed.items
    )
}

/// Cut a label to `max` characters with a marker, never silently.
pub(crate) fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max - 1).collect();
        out.push('…');
        out
    }
}

/// The items column: `-` when never fetched (nothing is known), the live
/// count otherwise — a fetched-empty body reads `0`, never the same as
/// unfetched (legibility ruling, 2026-09-02).
pub(crate) fn items_cell(fetched_at: Option<i64>, item_count: i64) -> String {
    match fetched_at {
        None => "-".into(),
        Some(_) => item_count.to_string(),
    }
}

/// C53: readable stash fields lead; the exact policy handle is the final
/// column, matching the character table and never clipped.
fn render_tabs_table(tabs: &[acquisition_store::TabRow], now: i64) -> String {
    let names: Vec<String> = tabs
        .iter()
        .map(|t| {
            let name = if t.name.is_empty() {
                "(unnamed)"
            } else {
                t.name.as_str()
            };
            let indent = if t.parent.is_some() { "  " } else { "" };
            clip(&format!("{indent}{name}"), 40)
        })
        .collect();
    let name_w = names
        .iter()
        .map(|n| n.chars().count())
        .max()
        .unwrap_or(4)
        .max(4);
    let type_w = tabs
        .iter()
        .map(|t| t.r#type.chars().count())
        .max()
        .unwrap_or(4)
        .max(4);
    let mut out = format!(
        "{:<name_w$} {:<type_w$} {:>6}  {:<12} {:<12} id\n",
        "name", "type", "items", "fetched", "parent"
    );
    for (t, name) in tabs.iter().zip(&names) {
        out.push_str(&format!(
            "{:<name_w$} {:<type_w$} {:>6}  {:<12} {:<12} {}\n",
            name,
            t.r#type,
            items_cell(t.fetched_at, t.item_count),
            ago(now, t.fetched_at),
            t.parent.as_deref().unwrap_or(""),
            t.id,
        ));
    }
    out
}

/// `acq tabs`: the league's tabs counted first and then listed — the ten
/// with the most items, `--limit` more or fewer, `--expand` every one,
/// whole where they fit (C53). The document is the search crate's, one maker for this
/// text, `--json` and the MCP's `tabs` (C115).
pub fn tabs(
    cx: &Ctx,
    realm: Realm,
    league: &str,
    select: &acquisition_search::tabs::Selection,
    limit: Option<usize>,
    expand: bool,
    json: bool,
) -> Result<()> {
    let store = open(cx)?;
    let listed = acquisition_search::tabs::Tabs::of(
        realm.as_str(),
        league,
        store.tabs(realm.as_str(), league)?,
        select,
        limit,
        expand,
    );
    if json {
        println!("{}", serde_json::to_string_pretty(&listed)?);
        return Ok(());
    }
    if listed.tabs == 0 {
        println!("{}", no_tabs_text(realm, league, &store));
        return Ok(());
    }
    let now = acquisition_store::now();
    // The answer first (C53), so it is never under a screen of rows.
    println!("{}", tabs_head(&listed, realm, league));
    // a selection is the answer under the league's counts: its rows,
    // whole or the ten, and the command that lists the rest (the re-sit:
    // every run asked which never-fetched tabs of a type to refresh)
    if let (Some(selected), Some(selection)) = (listed.selected, &listed.selection) {
        println!("{selected} {}", selection.words());
        print!("{}", render_tabs_table(&listed.rows, now));
        print!("{}", remove_only_said(&listed.rows));
        if listed.left_out > 0 {
            let (expand, _) = tabs_next(realm, league);
            let mut expand = expand;
            if let Some(kind) = &selection.kind {
                expand.push_str(&format!(" --type {kind}"));
            }
            if selection.never_fetched {
                expand.push_str(" --never-fetched");
            }
            println!(
                "{} of {selected} shown: `{expand}` lists all",
                listed.rows.len()
            );
        }
        return Ok(());
    }
    if listed.left_out == 0 {
        print!("{}", render_tabs_table(&listed.rows, now));
        print!("{}", remove_only_said(&listed.rows));
        return Ok(());
    }
    let width = listed
        .by_type
        .iter()
        .map(|t| t.kind.len())
        .max()
        .unwrap_or(4)
        .max(4);
    println!(
        "{:<width$}  {:>5}  {:>7}  {:>6}",
        "type", "tabs", "fetched", "items"
    );
    for t in &listed.by_type {
        println!(
            "{:<width$}  {:>5}  {:>7}  {:>6}",
            t.kind, t.tabs, t.fetched, t.items
        );
    }
    println!("the {} with the most items:", listed.rows.len());
    print!("{}", render_tabs_table(&listed.rows, now));
    print!("{}", remove_only_said(&listed.rows));
    let (expand, count) = tabs_next(realm, league);
    println!(
        "next: `{expand}` lists all {}; `{count}` counts what a query matches by tab",
        listed.tabs
    );
    Ok(())
}

/// What GGG's ` (Remove-only)` mark on a name is, said once under the
/// rows where a listed tab carries it (C117; the trade seat: cli-sonnet-2
/// never found out what it means, debrief 6).
fn remove_only_said(rows: &[acquisition_store::TabRow]) -> &'static str {
    if rows
        .iter()
        .any(|t| t.name.trim_end().ends_with("(Remove-only)"))
    {
        "a name ending (Remove-only) is GGG's mark on a tab whose items can only be taken out; a search asks it as tab.remove_only\n"
    } else {
        ""
    }
}

/// The large league's next commands, over the league and realm just
/// shown: `tabs` names them where they are not its defaults, and the
/// search, whose scope is a realm, takes the league as a term — printed
/// as every route is, so it runs as printed.
fn tabs_next(realm: Realm, league: &str) -> (String, String) {
    use acquisition_search::answer::{Counts, QueryInput, Request, Scope, Shape, shell_quoted};
    use acquisition_search::tree::{Node, Op, Value as Term};
    let mut expand = String::from("acq tabs --expand");
    if league != acquisition_plan::DEFAULT_LEAGUE {
        expand.push_str(&format!(" --league {}", shell_quoted(league)));
    }
    if realm != Realm::DEFAULT {
        expand.push_str(&format!(" --realm {}", realm.as_str()));
    }
    let term = Node::Test {
        field: "league".into(),
        op: Op::Eq,
        value: Term::Text(league.into()),
    };
    let count = Request {
        scope: Scope {
            realm: Some(acquisition_search::corpus::Realm::One(
                realm.as_str().into(),
            )),
            ..Scope::default()
        },
        query: QueryInput {
            text: Some(acquisition_search::print::print(&term)),
            tree: None,
            trade: None,
        },
        view: Shape::Counts(Counts {
            keys: vec!["tab".into()],
            sum: None,
            limit: None,
        })
        .into(),
        ..Request::default()
    }
    .command();
    (expand, count)
}

/// `acq store characters`: the characters on record, from the shared
/// store (no daemon, no network) — the CLI's twin of the MCP `characters`
/// tool, one document for both (C149): the counts over every character
/// the selection holds, then ten of them, `--limit` more or fewer,
/// `--expand` every one (C53). The readable columns first; the full id
/// last (policy ids match exactly, C55, so it is never cut).
pub fn characters(
    cx: &Ctx,
    realm: Option<Realm>,
    league: Option<&str>,
    limit: Option<usize>,
    expand: bool,
    json: bool,
) -> Result<()> {
    let account = cx.account()?;
    let store = account.facts()?;
    let listed = acquisition_verbs::Characters::of(
        realm.map(Realm::as_str),
        league,
        store.characters(realm.map(Realm::as_str), league)?,
        limit,
        expand,
    );
    if json {
        println!("{}", serde_json::to_string_pretty(&listed)?);
        return Ok(());
    }
    if listed.characters == 0 {
        // the league or realm asked about, as a line about a league names
        // it; the file when the whole store holds none
        let place = match (realm, league) {
            (None, None) => store.path().display().to_string(),
            (realm, Some(league)) => {
                format!("{}{league}", realm.map(realm_prefix).unwrap_or_default())
            }
            (Some(realm), None) => format!("realm {realm}"),
        };
        println!(
            "no characters known in {place} (run `acq characters` or cover them in the sync policy)"
        );
        return Ok(());
    }
    print!(
        "{}",
        characters_text(&listed, &account.entry.username, acquisition_store::now())
    );
    Ok(())
}

/// `store characters`' text, a function of its document (C53, C149): the
/// rows listed, the footer's counts read from the document, and where
/// some are left out the command that lists every one, carrying the
/// account and the selection it ran over so it runs as printed.
pub(crate) fn characters_text(
    listed: &acquisition_verbs::Characters,
    account: &str,
    now: i64,
) -> String {
    let rows = &listed.rows;
    let name_w = rows
        .iter()
        .map(|c| c.name.chars().count())
        .max()
        .unwrap_or(4)
        .clamp(4, 32);
    let league_w = rows
        .iter()
        .map(|c| c.league.as_deref().unwrap_or("-").chars().count())
        .max()
        .unwrap_or(6)
        .max(6);
    let mut out = String::new();
    if !rows.is_empty() {
        out.push_str(&format!(
            "{:<name_w$} {:<5} {:<league_w$} {:>5} {:>6}  {:<9} {:<9} id\n",
            "name", "realm", "league", "level", "items", "fetched", "listed"
        ));
    }
    for c in rows {
        out.push_str(&format!(
            "{:<name_w$} {:<5} {:<league_w$} {:>5} {:>6}  {:<9} {:<9} {}\n",
            clip(&c.name, 32),
            c.realm,
            c.league.as_deref().unwrap_or("-"),
            c.level.map(|l| l.to_string()).unwrap_or_default(),
            items_cell(c.fetched_at, c.item_count),
            ago(now, c.fetched_at),
            ago(now, c.listed_at),
            c.id,
        ));
    }
    let mut summary = format!("{} fetched", listed.fetched);
    if listed.empty_bodies > 0 {
        summary.push_str(&format!(" ({} with empty bodies)", listed.empty_bodies));
    }
    if listed.never_fetched > 0 {
        summary.push_str(&format!(", {} never fetched", listed.never_fetched));
    }
    out.push_str(&format!(
        "{} characters: {summary}; {} items\n",
        listed.characters, listed.items
    ));
    if listed.left_out > 0 {
        let mut every = format!(
            "acq store characters --account {}",
            acquisition_search::answer::shell_quoted(account)
        );
        if let Some(realm) = &listed.realm {
            every.push_str(&format!(" --realm {realm}"));
        }
        if let Some(league) = &listed.league {
            every.push_str(&format!(
                " --league {}",
                acquisition_search::answer::shell_quoted(league)
            ));
        }
        out.push_str(&format!(
            "{} of {} listed: `{every} --expand` lists every one\n",
            rows.len(),
            listed.characters
        ));
    }
    out
}

pub fn status(cx: &Ctx, json: bool) -> Result<()> {
    let store = open(cx)?;
    let st = store.status()?;
    if json {
        println!("{}", serde_json::to_string_pretty(&st)?);
    } else {
        print!("{}", status_text(&st));
    }
    Ok(())
}

/// What a granted-skill count counts: entries, not skills (the h2
/// review: one skill shown twice counted two, under the word "skills").
const SKILL_ENTRIES: &str = "GGG shows a granted skill once in each place it appears, the item's sockets and the character's skills, so one skill can be two entries";

fn counted(n: i64, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// `store status`'s text. Each count beyond the first line says what it
/// counted, where it came from and what the reader can do next (C146, a
/// count says what it counts): `withheld`, `unlifted` and `granted` named
/// mechanisms that four seats never resolved (the release seat's
/// debriefs, answer 8: cli-opus calls 3 and 78, cli-opus-2 call 75,
/// mcp-opus and mcp-opus-2 call 3).
pub(crate) fn status_text(st: &acquisition_store::Status) -> String {
    let mut out = format!("{}  ({:.1} MB)\n", st.path, st.bytes as f64 / 1e6);
    // the counts say what they count: `leagues 0` beside tabs in seven
    // leagues, and 3,444 tabs beside `tabs`'s 3,347 in one, read as a
    // contradiction (the trade seat: cli-opus calls 3, 5, its fault 7;
    // mcp-opus call 5)
    out.push_str(&format!(
        "responses {}  league list {}  characters {}  tabs {} (every realm and league)  items {} (+{} removed)  events {}\n",
        st.responses,
        match st.league_list.as_str() {
            "never fetched" => "never fetched (`acq leagues`)".to_string(),
            _ => format!("{} leagues", st.leagues),
        },
        st.characters,
        st.tabs,
        st.items,
        st.items_removed,
        st.events
    ));
    if st.withheld_responses > 0 {
        out.push_str(&format!(
            "withheld: {} of tabs or characters the latest list no longer named, carrying {}: kept as evidence, in no search or count; nothing to do unless a list names the place again\n",
            counted(st.withheld_responses, "fetch", "fetches"),
            counted(st.withheld_items, "item", "items"),
        ));
    }
    if st.unlifted_items > 0 {
        // The drift tripwire: an item array the store does not lift.
        out.push_str(&format!(
            "drift: {} in characters' data under a key this build does not read as items, so no search counts them: GGG added a kind of item list, which a newer acq reads (kept as `_unlifted` in the stored responses)\n",
            counted(st.unlifted_items, "item-shaped object", "item-shaped objects"),
        ));
    }
    if st.granted_skills > 0 {
        out.push_str(&format!(
            "granted: {} that PoE2 characters' items grant ({SKILL_ENTRIES}), kept with their characters and not counted as items (they have no id of their own); nothing to do\n",
            counted(st.granted_skills, "skill entry", "skill entries"),
        ));
    }
    if st.refused_bodies > 0 {
        out.push_str(&format!(
            "refused: {} the store would not ingest, kept verbatim as evidence (`acq store refused`)\n",
            counted(st.refused_bodies, "body", "bodies"),
        ));
    }
    out
}

/// Bodies the store refused as malformed: the list, or one body in full.
pub fn refused(cx: &Ctx, id: Option<i64>, limit: usize, json: bool) -> Result<()> {
    let store = open(cx)?;
    if let Some(id) = id {
        let Some(row) = store.refused(id)? else {
            anyhow::bail!("no refused body with id {id} (see `acq store refused`)");
        };
        if json {
            println!("{}", serde_json::to_string_pretty(&row)?);
        } else {
            println!(
                "refused {}  {} {}  fetched {}  status {}",
                row.id,
                row.endpoint,
                row.params,
                ago(acquisition_store::now(), Some(row.fetched_at)),
                row.status
            );
            println!("reason: {}", row.reason);
            println!("{}", serde_json::to_string_pretty(&row.body)?);
        }
        return Ok(());
    }
    let rows = store.refused_list(limit)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&rows)?);
        return Ok(());
    }
    let now = acquisition_store::now();
    for r in &rows {
        println!(
            "{:>5}  {:<10} {:<40} {:<9}  {}",
            r.id,
            r.endpoint,
            clip(&r.params.to_string(), 40),
            ago(now, Some(r.fetched_at)),
            r.reason
        );
    }
    println!(
        "{} refused body(ies){}",
        rows.len(),
        if rows.is_empty() {
            ""
        } else {
            " — `acq store refused <id>` prints one in full"
        }
    );
    Ok(())
}

/// One location's counts as text: nonzero kinds only.
fn counts(changes: &LocationChanges) -> String {
    let mut parts = Vec::new();
    if changes.added > 0 {
        parts.push(format!("+{} added", changes.added));
    }
    if changes.changed > 0 {
        parts.push(format!("~{} changed", changes.changed));
    }
    if changes.moved_in > 0 {
        parts.push(format!(">{} moved in", changes.moved_in));
    }
    if changes.moved_out > 0 {
        parts.push(format!("<{} moved out", changes.moved_out));
    }
    if changes.removed > 0 {
        parts.push(format!("-{} removed", changes.removed));
    }
    parts.join(", ")
}

/// `acq store events`: the text default is one line per location with
/// counts; the JSON default is the event list (what the driver reads).
/// `--expand` alone selects the event list in either mode, `--summary` the
/// locations, and `--summary --expand` every location (the legibility
/// ruling's one stated divergence). Each form is one document of the verbs
/// crate's (C149), ten by default (C53): the events' `limit`, the
/// locations' `expand`.
pub fn events(
    cx: &Ctx,
    since_hours: f64,
    limit: Option<usize>,
    expand: bool,
    summary: bool,
    json: bool,
) -> Result<()> {
    use acquisition_verbs::events::{EventSummary, Events};
    let account = cx.account()?;
    let store = account.facts()?;
    let now = acquisition_store::now();
    let ev = Events::read(&store, now, since_hours, limit)?;
    let summarize = summary || (!expand && !json);
    if summarize {
        let listed = EventSummary::of(
            &ev,
            acquisition_verbs::events::summary(&store, &ev.rows),
            summary && expand,
        );
        if json {
            println!("{}", serde_json::to_string_pretty(&listed)?);
            return Ok(());
        }
        print!(
            "{}",
            event_summary_text(&listed, &account.entry.username, now)
        );
        return Ok(());
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&ev)?);
        return Ok(());
    }
    for e in &ev.rows {
        let label = match (&e.name, &e.type_line) {
            (Some(n), Some(t)) if !n.is_empty() => format!("{n} {t}"),
            (_, Some(t)) => t.clone(),
            _ => "?".into(),
        };
        let loc = match e.kind.as_str() {
            "moved" => format!(
                "{} -> {}",
                e.from_location.as_deref().unwrap_or("?"),
                e.to_location.as_deref().unwrap_or("?")
            ),
            "removed" => format!("from {}", e.from_location.as_deref().unwrap_or("?")),
            _ => format!("at {}", e.to_location.as_deref().unwrap_or("?")),
        };
        println!(
            "{:<9} {:<8} {:<40} {loc}",
            ago(now, Some(e.at)),
            e.kind,
            clip(&label, 40)
        );
    }
    println!(
        "{} event(s){}",
        ev.returned,
        if ev.limit_reached {
            " (limit reached)"
        } else {
            ""
        }
    );
    Ok(())
}

/// The window's hours as the text says them: whole, or to a tenth.
fn hours_words(hours: f64) -> String {
    if hours == hours.trunc() {
        format!("{}", hours as i64)
    } else {
        format!("{hours:.1}")
    }
}

/// The summary's text, a function of its document (C53, C149): the line
/// of counts read from it, the locations listed, and where some are left
/// out the command that lists every one, carrying the account and the
/// window it ran over so it runs as printed.
pub(crate) fn event_summary_text(
    listed: &acquisition_verbs::events::EventSummary,
    account: &str,
    now: i64,
) -> String {
    let hours = hours_words(listed.hours);
    if listed.events == 0 {
        return format!("0 events in the last {hours} h\n");
    }
    let total = LocationChanges {
        added: listed.by_kind.added,
        changed: listed.by_kind.changed,
        moved_in: listed.by_kind.moved,
        removed: listed.by_kind.removed,
        ..Default::default()
    };
    let summed = counts(&total).replace(" moved in", " moved");
    let mut out = format!(
        "{} event{} at {} location{} in the last {hours} h: {summed}{}\n",
        listed.events,
        if listed.events == 1 { "" } else { "s" },
        listed.locations,
        if listed.locations == 1 { "" } else { "s" },
        if listed.limit_reached {
            " (limit reached)"
        } else {
            ""
        }
    );
    let rows = &listed.rows;
    let loc_w = rows
        .iter()
        .map(|r| r.location.chars().count())
        .max()
        .unwrap_or(0);
    let name_w = rows
        .iter()
        .map(|r| {
            r.name
                .as_deref()
                .map_or(0, |n| n.chars().count().max(7) + 2)
        })
        .max()
        .unwrap_or(0);
    for r in rows {
        let name = match r.name.as_deref() {
            Some("") => "(unnamed)".to_string(),
            Some(n) => format!("{n:?}"),
            None => String::new(),
        };
        out.push_str(&format!(
            "  {:<loc_w$}  {:<name_w$}  {:<28}  {}\n",
            r.location,
            name,
            counts(r),
            ago(now, Some(r.last))
        ));
    }
    if listed.left_out > 0 {
        out.push_str(&format!(
            "{} of {} locations listed: `acq store events --account {} --hours {hours} --limit {} --summary --expand` lists every one\n",
            rows.len(),
            listed.locations,
            acquisition_search::answer::shell_quoted(account),
            listed.limit
        ));
    }
    out.push_str("(--expand lists every event)\n");
    out
}

pub fn rebuild(cx: &Ctx, json: bool) -> Result<()> {
    let mut store = open(cx)?;
    let n = store.rebuild()?;
    if json {
        println!("{}", json!({ "reextracted": n }));
    } else {
        println!("re-extracted columns for {n} items");
    }
    Ok(())
}

/// Replay a snapshot from the retired `acq pull` (its format: `{league,
/// taken_at_unix, tabs: {id: {name, type, items: {id: item}}}}`) into the
/// store as if each tab had just been fetched: real-shape data, zero GGG
/// traffic. Those snapshots lost tab metadata and parents, so tabs land
/// flat. Kept as the real-data fixture path.
pub fn import(cx: &Ctx, path: &Path, json: bool) -> Result<()> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let snap: Value = serde_json::from_str(&text)?;
    let league = snap
        .get("league")
        .and_then(Value::as_str)
        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE)
        .to_string();
    let at = snap
        .get("taken_at_unix")
        .and_then(Value::as_i64)
        .unwrap_or_else(acquisition_store::now);
    let Some(tabs) = snap.get("tabs").and_then(Value::as_object) else {
        bail!("not a pull snapshot: no `tabs`")
    };
    let mut store = open(cx)?;
    let started = std::time::Instant::now();
    let mut total = acquisition_store::Ingest::default();
    let mut skipped_no_id = 0usize;
    for (id, tab) in tabs {
        let mut items: Vec<Value> = match tab.get("items") {
            Some(Value::Object(m)) => m.values().cloned().collect(),
            Some(Value::Array(a)) => a.clone(),
            _ => Vec::new(),
        };
        // Legacy tolerance lives here, at the import boundary, and is
        // reported: the store's network ingest refuses an id-less item as
        // malformed (it cannot be tracked, and dropped silently it would
        // poison removal), but an old pull snapshot is what it is.
        strip_idless_items(&mut items, &mut skipped_no_id);
        let body = json!({ "stash": { "id": id, "name": tab.get("name"), "type": tab.get("type"), "items": items } });
        // A retired-pull snapshot predates realms: pc, the only realm it
        // could have been taken on.
        let ep = Endpoint::Stash {
            realm: "pc".into(),
            league: league.clone(),
            id: id.clone(),
            sub: None,
        };
        let ing = store.record(
            &ep,
            &json!({ "realm": "pc", "league": league, "id": id }),
            200,
            &body,
            at,
        )?;
        total.items += ing.items;
        total.added += ing.added;
        total.moved += ing.moved;
        total.changed += ing.changed;
        total.removed += ing.removed;
    }
    let elapsed = started.elapsed();
    if json {
        println!(
            "{}",
            json!({ "tabs": tabs.len(), "ingest": total, "seconds": elapsed.as_secs_f64(),
                    "skipped_no_id": skipped_no_id })
        );
    } else {
        println!(
            "imported {} tabs, {} items (+{} added, ~{} changed, >{} moved, -{} removed) in {:.2}s",
            tabs.len(),
            total.items,
            total.added,
            total.changed,
            total.moved,
            total.removed,
            elapsed.as_secs_f64()
        );
        if skipped_no_id > 0 {
            println!("skipped {skipped_no_id} item(s) without ids (legacy snapshot tolerance)");
        }
    }
    Ok(())
}

/// Drop items (and socketed gems) that carry no `id`, counting them, so a
/// legacy snapshot imports with a report instead of failing — the store's
/// own ingest refuses id-less items as malformed.
fn strip_idless_items(items: &mut Vec<Value>, skipped: &mut usize) {
    items.retain(|item| {
        let ok = item.get("id").and_then(Value::as_str).is_some();
        if !ok {
            *skipped += 1;
        }
        ok
    });
    for item in items {
        if let Some(gems) = item.get_mut("socketedItems").and_then(Value::as_array_mut) {
            let before = gems.len();
            gems.retain(|gem| gem.get("id").and_then(Value::as_str).is_some());
            *skipped += before - gems.len();
        }
    }
}

/// `acq leagues` in text: the list GGG returned, one league a line with
/// its realm and category, ready to copy a name from; `--json` prints the
/// job's answer as it came. A payload of another shape prints whole.
pub fn leagues_text(payload: &Value) -> String {
    // the job's payload carries GGG's body under `body`
    let body = payload.get("body").unwrap_or(payload);
    let Some(leagues) = body.get("leagues").and_then(Value::as_array) else {
        return format!(
            "{}\n",
            serde_json::to_string_pretty(payload).unwrap_or_default()
        );
    };
    let field = |l: &Value, path: &[&str]| -> String {
        let mut v = l;
        for key in path {
            v = &v[*key];
        }
        v.as_str().unwrap_or("-").to_string()
    };
    let rows: Vec<[String; 3]> = leagues
        .iter()
        .map(|l| {
            [
                field(l, &["id"]),
                field(l, &["realm"]),
                field(l, &["category", "id"]),
            ]
        })
        .collect();
    let w = rows
        .iter()
        .map(|r| r[0].chars().count())
        .chain([6])
        .max()
        .unwrap_or(6);
    let mut out = format!(
        "{} {}\n{:<w$}  realm  category\n",
        rows.len(),
        if rows.len() == 1 { "league" } else { "leagues" },
        "league"
    );
    for [id, realm, category] in &rows {
        out.push_str(&format!("{id:<w$}  {realm:<5}  {category}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use acquisition_store::EventRow;

    fn ev(at: i64, kind: &str, from: Option<&str>, to: Option<&str>) -> EventRow {
        EventRow {
            at,
            item_id: "i".into(),
            kind: kind.into(),
            from_location: from.map(str::to_string),
            to_location: to.map(str::to_string),
            name: None,
            type_line: None,
        }
    }

    /// C146 — a count says what it counts: each of `store status`'s
    /// lines past the first names what it counted, where it came from and
    /// what to do, in words; the JSON's field names stay the contract.
    #[test]
    fn c146_store_status_lines_say_what_they_count() {
        let st = acquisition_store::Status {
            path: "/s/a.db".into(),
            bytes: 100_000,
            responses: 5,
            leagues: 0,
            league_list: "never fetched".into(),
            characters: 2,
            tabs: 3,
            items: 9,
            items_removed: 0,
            events: 9,
            unlifted_items: 2,
            withheld_responses: 1,
            withheld_items: 4,
            refused_bodies: 1,
            granted_skills: 8,
            last_event: 9,
        };
        let text = status_text(&st);
        for line in [
            "withheld: 1 fetch of tabs or characters the latest list no longer named, carrying 4 items: kept as evidence, in no search or count; nothing to do unless a list names the place again\n",
            "drift: 2 item-shaped objects in characters' data under a key this build does not read as items, so no search counts them: GGG added a kind of item list, which a newer acq reads (kept as `_unlifted` in the stored responses)\n",
            "granted: 8 skill entries that PoE2 characters' items grant (GGG shows a granted skill once in each place it appears, the item's sockets and the character's skills, so one skill can be two entries), kept with their characters and not counted as items (they have no id of their own); nothing to do\n",
            "refused: 1 body the store would not ingest, kept verbatim as evidence (`acq store refused`)\n",
        ] {
            assert!(text.contains(line), "{text}");
        }
        let json = serde_json::to_value(&st).unwrap();
        for field in [
            "unlifted_items",
            "withheld_responses",
            "withheld_items",
            "granted_skills",
        ] {
            assert!(json.get(field).is_some(), "{field}: {json}");
        }
    }

    #[test]
    fn events_group_by_location_newest_first_with_names() {
        let events = vec![
            ev(10, "added", None, Some("character/pc/c1")),
            ev(11, "added", None, Some("character/pc/c1")),
            ev(
                12,
                "moved",
                Some("stash/pc/Standard/t1"),
                Some("character/pc/c1"),
            ),
            ev(13, "removed", Some("stash/pc/Standard/t1"), None),
            ev(20, "changed", None, Some("stash/pc/Standard/t2")),
        ];
        let mut namer = |loc: &str| match loc {
            "character/pc/c1" => Some("Exile".to_string()),
            "stash/pc/Standard/t1" => Some("Dump".to_string()),
            _ => None,
        };
        let rows = acquisition_verbs::events::summarize(&events, &mut namer);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].location, "stash/pc/Standard/t2");
        assert_eq!(rows[0].changed, 1);
        assert!(rows[0].name.is_none());
        let t1 = rows.iter().find(|r| r.location.ends_with("t1")).unwrap();
        assert_eq!((t1.moved_out, t1.removed), (1, 1));
        assert_eq!(t1.name.as_deref(), Some("Dump"));
        let c1 = rows.iter().find(|r| r.location.ends_with("c1")).unwrap();
        assert_eq!((c1.added, c1.moved_in, c1.first, c1.last), (2, 1, 10, 12));
        assert_eq!(counts(c1), "+2 added, >1 moved in");
    }

    #[test]
    fn the_items_cell_tells_unfetched_from_empty() {
        assert_eq!(items_cell(None, 0), "-");
        assert_eq!(items_cell(Some(1), 0), "0");
        assert_eq!(items_cell(Some(1), 7), "7");
        assert_eq!(clip("abcdef", 4), "abc…");
        assert_eq!(clip("abc", 4), "abc");
    }

    #[test]
    fn c53_stash_tab_ids_are_the_final_column() {
        let tabs = [acquisition_store::TabRow {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "child-id".into(),
            parent: Some("parent-id".into()),
            name: "Child".into(),
            r#type: "PremiumStash".into(),
            idx: Some(1),
            listed_at: Some(90),
            fetched_at: Some(90),
            removed_at: None,
            item_count: 7,
            fetched_items: Some(7),
        }];
        let text = render_tabs_table(&tabs, 100);
        let mut lines = text.lines();
        assert!(lines.next().unwrap().ends_with(" id"), "{text}");
        let row = lines.next().unwrap();
        assert!(row.starts_with("  Child"), "{text}");
        assert!(row.contains("parent-id"), "{text}");
        assert!(row.ends_with("child-id"), "{text}");
    }
}
