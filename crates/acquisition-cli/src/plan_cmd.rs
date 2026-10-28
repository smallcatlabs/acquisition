//! The intent layer's CLI surface: `acq policy` reads and writes the
//! per-account sync-policy annotation (through the store crate, under
//! compare-and-swap), and `acq refresh --plan` compiles it into a
//! [`RefreshPlan`] — offline, spending nothing (tracer step 6). A running
//! daemon enriches the plan with its read-only quote; the command never
//! spawns one just to decorate output the planner computed with the
//! daemon down.
//!
//! Text output follows the legibility ruling (C53): one verdict line
//! before detail, actions grouped by kind and parent and counted by
//! reason (`--expand` lists every one), a failure that names the job, its
//! target, the cause and where the evidence is, and the same renderer for
//! a fresh compile and a reviewed file (`--plan=FILE`) — the text is a
//! function of the envelope, which itself never changes for presentation.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/frontends.md` for this
//! area, `AGENTS.md` for the cross-cutting ones (`C<n>`); what follows is each
//! entry's full text as recorded there, moved here on 2026-09-02 because
//! the mechanism it describes is this module's. The registry is current;
//! this is the mechanism as decided, kept beside the code that implements it.
//!
//! ## C53 — Legible output for the refresh slice, as ruled (2026-09-02)
//!
//! **Density amendment (owner-approved 2026-09-02): default text is the
//! decision view, `--expand` is the audit view, and JSON is the contract.**
//! The default says repeated context once, removes explanatory prose and
//! provenance that cannot change the next decision, and keeps handles only
//! where they are useful. Audit text retains the complete basis, per-action
//! reasons, types, wire prerequisites, and quote evidence. The envelope is
//! unchanged. Owner verdict on the proposed model: "Yes, this is good."
//!
//! **Two consumers, read differently.** The owner at a terminal reads once,
//! top to bottom, and needs five answers in order: what will this do, what
//! did it do, what changed, what went wrong, what do I type next. An agent
//! reads the same surfaces through a shell and needs structure it can grep
//! and count, and `--json` it can `jq`, with text and JSON saying the same
//! thing. Designed for both at once; where they diverge it is said below
//! and decided.
//!
//! **Principles (binding for every surface in the slice):**
//!
//! 1. **One line before detail.** Each surface opens with the line that
//!    answers its reader's question — `112 requests`, `job 221 failed: 4 of
//!    5 requests failed`, `59 characters: 42 fetched (10 with empty bodies),
//!    17 never fetched` — and the detail under it is optional.
//! 2. **Grouping is presentation, never a change to what is authorized.**
//!    Text groups actions by kind and by parent and counts by reason; the
//!    envelope and every `--json` output keep every action explicit
//!    (`REFRESH_PLAN_SCHEMA` stays 6 — no field is added to the envelope
//!    for this). The rule for a group of entities: **ten or fewer are
//!    listed one per line; more are counted**, with a breakdown by reason
//!    and, for substashes, by parent. `--expand` lists every entity on the
//!    surfaces that group (`refresh --plan`, `store events`, the driver's
//!    summary); `--json` always carries everything.
//! 3. **Text is a function of the JSON.** `acq refresh --plan=FILE` renders
//!    a reviewed envelope through the same renderer as a fresh compile
//!    (re-validated by the planner's parse), so the driver's confirm step
//!    and a human's review of a saved plan read the same text, and the
//!    grouped text is derivable from the envelope by construction. Agents
//!    count with `jq` over the envelope; `acq refresh --help` carries the
//!    one-liners.
//! 4. **A failure line names the job, its target, the cause, and where the
//!    evidence is** — the child's result id, the `refused` row, the daemon
//!    log path — never a bare id list. A parent's failure expands its
//!    failed children inline (ten at most, then `and n more: acq jobs`),
//!    and `failed_ids` is sorted at the source.
//! 5. **Every nothing says which nothing.** Never fetched renders `-`; a
//!    fetched body that carried no items renders `0` and the footer counts
//!    those rows; an empty plan says what is fresh and for how long; a
//!    missing quote says `no daemon running` once, without an OS error
//!    string (the `ACQ_NO_SPAWN` distinction is dropped from that line —
//!    `--plan` never spawns either way).
//! 6. **Ages in text, epochs in JSON.** Column widths come from the data
//!    (capped, with `…` when cut), never silent truncation; ids stay full
//!    where they are the handle (character ids, last column).
//! 7. **Terminal escapes only on a tty.** Captured output — the driver's
//!    `.out` files, an agent's shell — gets plain lines; the in-place
//!    progress line is a tty convenience.
//! 8. **JSON changes are additive.** The plan envelope is untouched;
//!    `refresh --apply --json` gains `store_changes` beside the outcome;
//!    `CharacterRow` / `TabRow` gain `fetched_items` (the item facts the
//!    last landed body carried; `null` when never fetched); nothing that
//!    reads `--json` today (`tracer-rung.sh`, `tracer-verify.py`, `acq-mcp`)
//!    changes.
//! 9. **Stated divergence.** `acq store events`: the text default is the
//!    per-location summary (what a person asks: "what changed"), the JSON
//!    default stays the event list (what exists, what the driver reads);
//!    `--summary` and `--expand` select either form in both modes. `acq
//!    refresh --plan`: text default is grouped, JSON is the envelope.
//!
//! **Considered and rejected:**
//!
//! - A `summary` block inside the plan envelope: a schema bump and one
//!   more derived quantity to validate at parse, for presentation. The
//!   envelope is the authorization, not the report; `--plan=FILE` makes
//!   "text is a function of the JSON" literal without touching it.
//! - Counting everything (no threshold): a five-tab plan is more legible
//!   listed than counted, and the tracer's own plan was fine at six
//!   lines. The threshold is stated so the shape is predictable.
//! - Names without ids in the expanded character lines: ids are what a
//!   policy names (ruling above); the default view drops ids by counting,
//!   the expanded view keeps them whole.
//! - Changing the events JSON default to the summary: the driver reads
//!   the array and hits an explicit limit on it. Rule 9 instead.
//! - Colour or other decoration: captured output is the common case.
//! - A `--quiet` driver: its problem is repetition (the plan three times
//!   per cycle, the note twice), not volume.
//! - A store-side event summary function: the CLI can group the event
//!   list and resolve names with the reads it has; a shared function
//!   waits for the second consumer (MCP) to ask for it.

use std::collections::BTreeMap;

use acquisition_plan::pricing::Expect;
use acquisition_plan::{
    CharacterSkipReason, FetchReason, LeagueNote, ListingReason, PlanNotes, PolicyWrite,
    RefreshAction, RefreshPlan, SkipReason, Spent,
};
use acquisition_protocol::job::Outcome;
use acquisition_protocol::protocol::{Quote, QuoteScope};
use acquisition_protocol::realm::Realm;
use acquisition_verbs::refresh::{holding_jobs, spent_phrase};
use acquisition_verbs::{Apply, Planned, QuoteNote, StoreChanges};
use anyhow::{Context, Result, bail};
use serde_json::Value;

use crate::{Ctx, store_cmd};

/// The channel every intent write from this binary is stamped with (C65).
pub(crate) const WRITTEN_VIA: &str = "cli";

/// A shape hint for humans; the planner's strict parse is the authority.
pub(crate) const POLICY_EXAMPLE: &str = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}"#;

/// Groups of this many entities or fewer are listed one per line; larger
/// groups are counted (the ruling's threshold; `--expand` lists every one).
const LIST_UP_TO: usize = 10;

/// `acq policy show`: the stored row and its revision; `--json` the
/// MCP's `sync_policy` document, `{"policy": row}`, row null when none is
/// set (C149: a success is never a bare `null`).
pub fn policy_show(cx: &Ctx, json: bool) -> Result<()> {
    let shown = cx.account()?.policy_shown()?;
    if json {
        println!("{}", serde_json::to_string_pretty(&shown)?);
        return Ok(());
    }
    match shown.policy {
        None => {
            println!("no sync policy is set — write one with `acq policy set '{POLICY_EXAMPLE}'`")
        }
        Some(row) => {
            println!("{}", serde_json::to_string_pretty(&row.value)?);
            println!(
                "revision {}, updated {}",
                row.revision,
                store_cmd::ago(acquisition_store::now(), Some(row.updated_at))
            );
        }
    }
    Ok(())
}

/// The CLI's policy write, through the layer's (C52, C35): validate, then
/// compare-and-swap. `if_revision` is the CAS at the human boundary:
/// given, the write lands only over exactly that revision — what the
/// caller reviewed (`acq policy show` prints it) is what they replace.
/// Omitted, it replaces the revision read just before the put
/// ([`Expect::Current`], this frontend's default), still a CAS
/// underneath, and never over a stored policy this build cannot read.
/// `--replace` writes the whole policy instead of its realms (C36).
pub fn policy_set(
    cx: &Ctx,
    value: &str,
    if_revision: Option<i64>,
    replace: bool,
    json: bool,
) -> Result<()> {
    let text = match value {
        "-" => std::io::read_to_string(std::io::stdin()).context("reading policy from stdin")?,
        v if v.starts_with('@') => {
            let path = &v[1..];
            std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?
        }
        v => v.to_string(),
    };
    let value: Value = serde_json::from_str(&text).context("the policy must be JSON")?;
    let write = if replace {
        PolicyWrite::Replace
    } else {
        PolicyWrite::Merge
    };
    let written = cx
        .account()?
        .set_policy(&value, policy_expect(if_revision), write)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&written)?);
    } else {
        println!(
            "sync policy written (revision {}) — preview the work with `acq refresh --plan`",
            written.row.revision
        );
        print!(
            "{}",
            league_note_lines(&written.league_notes, acquisition_store::now())
        );
    }
    Ok(())
}

/// C126 (a league the list lacks is a note): one line for each league
/// the fetched league list does not hold, or one line naming every
/// league when no list was ever fetched. Nothing refuses on it.
fn league_note_lines(notes: &[LeagueNote], now: i64) -> String {
    let mut out = String::new();
    let unread: Vec<&str> = notes
        .iter()
        .filter(|n| n.list_fetched_at.is_none())
        .map(|n| n.league.as_str())
        .collect();
    if !unread.is_empty() {
        out.push_str(&format!(
            "note: the league list was never fetched, so {} {} not read against it (`acq leagues` fetches it)\n",
            unread.join(", "),
            if unread.len() == 1 { "is" } else { "are" }
        ));
    }
    for note in notes {
        if let Some(at) = note.list_fetched_at {
            out.push_str(&format!(
                "note: {} is not among the leagues fetched {} (`acq leagues`)\n",
                note.league,
                store_cmd::ago(now, Some(at))
            ));
        }
    }
    out
}

/// The compile's notes under a plan (C126; a covered league with no
/// character listed in it).
fn plan_note_lines(plan: &RefreshPlan, notes: &PlanNotes, now: i64) -> String {
    let mut out = league_note_lines(notes.league.as_slice(), now);
    if notes.no_characters_listed {
        let listed = plan
            .basis
            .character_listing
            .as_ref()
            .map(|l| format!(", listed {}", store_cmd::ago(now, Some(l.fetched_at))))
            .unwrap_or_default();
        out.push_str(&format!(
            "characters: none in {}{} (the realm's character listing{listed})\n",
            store_cmd::realm_prefix(plan.realm),
            plan.league
        ));
    }
    out
}

/// The CLI's `--if-revision` for a policy write: `0` is "only where no
/// policy stands" (revisions start at 1), the create-only write the MCP
/// takes by default; another number is that exact revision; none is the
/// revision read just before the write.
fn policy_expect(if_revision: Option<i64>) -> Expect {
    match if_revision {
        Some(0) => Expect::Create,
        revision => Expect::or(revision, Expect::Current),
    }
}

/// Read a plan envelope from a path or stdin (`-`) through the planner's
/// validating parse — the only way a serialized plan becomes a
/// [`RefreshPlan`] here.
fn read_plan(source: &str) -> Result<RefreshPlan> {
    let text = match source {
        "-" => std::io::read_to_string(std::io::stdin()).context("reading the plan from stdin")?,
        path => std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?,
    };
    let value: Value = serde_json::from_str(&text)
        .context("the plan must be JSON — the envelope `acq refresh --plan --json` prints")?;
    Ok(RefreshPlan::from_value(&value)?)
}

/// `acq refresh --plan`: the layer's compile of the stored sync policy,
/// quoted by a running daemon if one answers (C10), printed human or
/// JSON. Nothing is submitted and nothing is sent; the JSON form is the
/// `plan` member of the layer's [`Planned`] — the serialized envelope
/// itself (self-validating on parse), so it can be reviewed, stored, or
/// handed to `--apply` — and its note goes to stderr (C11's stdout rule).
/// `--plan=FILE` renders a reviewed envelope instead — the same
/// renderer, the quote it carries (none is asked for), no store read.
pub async fn refresh_plan(
    cx: &Ctx,
    realm: Realm,
    league: &str,
    source: Option<&str>,
    json: bool,
    expand: bool,
) -> Result<()> {
    let now = acquisition_store::now();
    let Planned {
        plan,
        quote_note,
        notes,
    } = match source {
        Some(source) => Planned {
            plan: read_plan(source)?,
            quote_note: None,
            notes: PlanNotes::default(),
        },
        None => cx.account()?.open()?.plan(realm, league).await?,
    };
    if json {
        // Stdout is the plan envelope, nothing else, so it can be piped;
        // why the quote is absent, and the compile's notes, are
        // diagnostics, not part of the plan.
        if let Some(note) = &quote_note {
            eprintln!("note: {note}");
        }
        eprint!("{}", plan_note_lines(&plan, &notes, now));
        println!("{}", serde_json::to_string_pretty(&plan)?);
        return Ok(());
    }
    print!(
        "{}",
        render_plan(&plan, quote_note.as_ref(), &notes, now, expand, source)
    );
    Ok(())
}

/// `acq refresh --apply`: execute a plan through the layer's apply — its
/// unspent actions (C111), as one `apply` parent job the daemon admits or
/// refuses whole (C38). With no source, the stored policy is compiled
/// right now (the normal one-keyboard loop); a source is a reviewed plan
/// envelope from `refresh --plan --json`, re-validated by the planner's
/// own parse. The CLI's own flag check runs first, then the layer's
/// staleness gate (C44), both before any daemon contact; an empty plan
/// never contacts one. After the parent finishes, the layer reads the
/// store back and says what landed; `--json` prints that one document.
pub async fn refresh_apply(
    cx: &Ctx,
    realm_flag: Option<Realm>,
    league_flag: Option<&str>,
    plan_source: Option<&str>,
    max_requests: Option<u64>,
    json: bool,
) -> Result<()> {
    let mut opened = cx.account()?.open()?;
    let plan = match plan_source {
        Some(source) => read_plan(source)?,
        None => opened.compile(
            realm_flag.unwrap_or(Realm::DEFAULT),
            league_flag.unwrap_or(acquisition_plan::DEFAULT_LEAGUE),
            acquisition_store::now(),
        )?,
    };
    check_flags(&plan, realm_flag, league_flag)?;
    apply_plan(&mut opened, &plan, max_requests, json).await
}

/// `acq refresh --tabs`, `--all` and `--characters` (C44 as amended, C76):
/// the selection compiled into a plan — quoted by a running daemon, printed as `--plan`
/// prints one — and applied through the one apply road `--apply` takes.
/// `--json` prints the apply's document alone, as `--apply --json` does.
pub async fn refresh_selection(
    cx: &Ctx,
    realm: Realm,
    league: &str,
    selection: acquisition_plan::PlanSelection,
    json: bool,
) -> Result<()> {
    let mut opened = cx.account()?.open()?;
    let Planned {
        plan,
        quote_note,
        notes,
    } = opened.plan_selection(realm, league, &selection).await?;
    if !json {
        print!(
            "{}",
            render_plan(
                &plan,
                quote_note.as_ref(),
                &notes,
                acquisition_store::now(),
                false,
                None
            )
        );
    }
    apply_plan(&mut opened, &plan, None, json).await
}

/// A plan applied through the layer (C38, C44, C111) and what landed
/// said: the one road every apply takes, a reviewed file's, the stored
/// policy's compiled now, and a selection's.
async fn apply_plan(
    opened: &mut acquisition_verbs::Opened,
    plan: &RefreshPlan,
    max_requests: Option<u64>,
    json: bool,
) -> Result<()> {
    let plan = plan.clone();
    let mut slot = None;
    let applied = opened.apply(&plan, max_requests, &mut slot).await;
    if let Some(client) = &slot {
        crate::note_spawn(client);
    }
    let submitted = match applied? {
        nothing @ Apply::Nothing { .. } => {
            // A strict subset of zero actions is satisfied by doing
            // nothing; "authorizes no requests" is the honest claim — a
            // plan can be empty for reasons besides freshness, and the
            // plan's own skipped/unknown reporting says which.
            if json {
                println!("{}", serde_json::to_string(&nothing)?);
            } else {
                print!("{}", render_nothing_to_do(&plan, acquisition_store::now()));
            }
            return Ok(());
        }
        spent @ Apply::Spent { .. } => {
            if json {
                println!("{}", serde_json::to_string(&spent)?);
                return Ok(());
            }
            if let Apply::Spent {
                note,
                spent,
                remaining_requests,
                ..
            } = &spent
            {
                println!("{note}{}", wait_hint(spent));
                if let Some(line) = remaining_line(*remaining_requests, &plan) {
                    println!("{line}");
                }
            }
            return Ok(());
        }
        Apply::Submitted(submitted) => submitted,
    };
    let Some(mut client) = slot else {
        bail!("the apply was submitted on no connection this command holds");
    };
    let id = submitted.job_id;
    if !json {
        let sending = requests(submitted.requests as u64);
        if submitted.spent.is_empty() {
            println!("applying {sending} as job {id}");
        } else {
            println!(
                "applying {sending} as job {id} ({} of the plan's {} already spent: {}{})",
                submitted.spent.len(),
                plan.actions.len(),
                spent_phrase(&submitted.spent, acquisition_store::now()),
                wait_hint(&submitted.spent)
            );
        }
    }
    let mut progress = crate::Progressor::new();
    let mut report = |p: &acquisition_verbs::Progress| progress.report(p);
    let landed = opened
        .landed(
            &mut client,
            &plan,
            submitted,
            (!json).then_some(&mut report as &mut (dyn FnMut(&acquisition_verbs::Progress) + Send)),
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&landed)?);
        return match landed.outcome {
            Outcome::Success { .. } => Ok(()),
            _ => Err(crate::AlreadyReported.into()),
        };
    }
    print!(
        "{}",
        crate::job_report(id, &landed.outcome, &landed.failed_children)
    );
    println!("{}", changes_line(&landed.store_changes));
    match landed.outcome {
        Outcome::Success { .. } => {
            if let Some(line) = remaining_line(landed.remaining_requests, &plan) {
                println!("{line}");
            }
            Ok(())
        }
        Outcome::Failure { .. } => {
            if let Some(log) = crate::daemon_log_path(&mut client).await {
                println!("daemon log: {log}");
            }
            Err(crate::AlreadyReported.into())
        }
        Outcome::Cancelled => Err(crate::AlreadyReported.into()),
    }
}

/// The CLI's way to wait for the job holding a spent request, after the
/// shared phrase names it (C111): the `acq result` of that job with `--wait`.
fn wait_hint(spent: &[Spent]) -> String {
    match holding_jobs(spent).first() {
        Some(first) => format!(" (`acq result {first} --wait`)"),
        None => String::new(),
    }
}

/// The apply's last line (C53: the next action): what a plan compiled
/// now would still fetch, or that nothing is left. The command carries
/// the plan's realm and league where they are not the defaults, so it
/// plans the league just applied as printed.
fn remaining_line(remaining: Option<u64>, plan: &RefreshPlan) -> Option<String> {
    Some(match remaining? {
        0 => "left: nothing — every covered entry is fresh".into(),
        n => format!(
            "next: `acq refresh --plan{}` — {} more are planned now (what this apply revealed, \
             such as a map or unique tab's substashes, or left undone); plan and apply until \
             the plan says nothing to do",
            scope_flags(plan),
            requests(n)
        ),
    })
}

/// ` --realm R --league L` for a plan off the defaults, each only where
/// it differs, the league quoted where a shell needs it.
fn scope_flags(plan: &RefreshPlan) -> String {
    let mut flags = String::new();
    if plan.realm != Realm::DEFAULT {
        flags.push_str(&format!(" --realm {}", plan.realm));
    }
    if plan.league != acquisition_plan::DEFAULT_LEAGUE {
        flags.push_str(&format!(
            " --league {}",
            acquisition_search::answer::shell_quoted(&plan.league)
        ));
    }
    flags
}

/// `changed: +98 items added, 3 changed at 5 locations; 112 responses
/// recorded (acq store events)` — nonzero kinds only.
fn changes_line(changes: &StoreChanges) -> String {
    let mut parts = Vec::new();
    if changes.added > 0 {
        parts.push(format!("+{} added", changes.added));
    }
    if changes.changed > 0 {
        parts.push(format!("~{} changed", changes.changed));
    }
    if changes.moved > 0 {
        parts.push(format!(">{} moved", changes.moved));
    }
    if changes.removed > 0 {
        parts.push(format!("-{} removed", changes.removed));
    }
    let responses = format!(
        "{} response{} recorded",
        changes.responses,
        if changes.responses == 1 { "" } else { "s" }
    );
    if parts.is_empty() {
        format!("changed: no items — {responses}, nothing added, changed, moved, or removed")
    } else {
        format!(
            "changed: {} at {} location{} since the apply started; {responses} (acq store events)",
            parts.join(", "),
            changes.locations,
            if changes.locations == 1 { "" } else { "s" }
        )
    }
}

/// The CLI's own check before the layer's staleness gate (C44): a
/// `--realm` or `--league` naming another than the reviewed envelope's is
/// caller confusion, refused before any spend.
fn check_flags(
    plan: &RefreshPlan,
    realm_flag: Option<Realm>,
    league_flag: Option<&str>,
) -> Result<()> {
    if let Some(realm) = realm_flag
        && realm != plan.realm
    {
        bail!(
            "--realm {realm} conflicts with the plan's realm {}",
            plan.realm
        );
    }
    if let Some(league) = league_flag
        && league != plan.league
    {
        bail!(
            "--league {league} conflicts with the plan's league {:?}",
            plan.league
        );
    }
    Ok(())
}

// ---- rendering --------------------------------------------------------------

fn requests(n: u64) -> String {
    format!("{n} request{}", if n == 1 { "" } else { "s" })
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// Two-column lines padded to the widest left column (capped), so reasons
/// line up under each other without truncating a name.
fn columns(rows: &[(usize, String, String)]) -> String {
    let width = rows
        .iter()
        .map(|(indent, left, _)| indent + left.chars().count())
        .max()
        .unwrap_or(0)
        .min(72);
    let mut out = String::new();
    for (indent, left, right) in rows {
        let pad = " ".repeat(*indent);
        if right.is_empty() {
            out.push_str(&format!("{pad}{left}\n"));
        } else {
            let used = indent + left.chars().count();
            let gap = " ".repeat(width.saturating_sub(used) + 2);
            out.push_str(&format!("{pad}{left}{gap}{right}\n"));
        }
    }
    out
}

/// The audit header keeps every basis handle. The decision view keeps only
/// context that can change whether the reader applies the plan.
/// What authorizes the plan (C44 as amended): the stored policy at a
/// revision, or the selection itself.
fn authority(plan: &RefreshPlan) -> String {
    match (&plan.basis.policy_revision, &plan.basis.selection) {
        (Some(revision), _) => format!("policy {revision}"),
        (None, Some(selection)) if selection.deep => {
            "a selection, with its map and unique tabs' pages".to_string()
        }
        _ => "a selection".to_string(),
    }
}

/// Under a deep selection, the map and unique tabs with no page to fetch
/// on record: pages not known yet arrive with the tab's fetch, so the plan
/// says to apply and select again for them.
fn pages_awaiting(plan: &RefreshPlan) -> String {
    let mut out = String::new();
    // C76: the selection's listing is refreshed beside its fetches, and a
    // tab or character it newly lists takes the next plan
    if plan.actions.iter().any(|a| {
        matches!(
            a,
            RefreshAction::ListStashes {
                reason: ListingReason::Selected,
                ..
            } | RefreshAction::ListCharacters {
                reason: ListingReason::Selected,
                ..
            }
        )
    }) {
        out.push_str("the listing is fetched beside the selection: what it newly lists takes the next plan\n");
    }
    if !plan.basis.selection.as_ref().is_some_and(|s| s.deep) {
        return out;
    }
    for action in &plan.actions {
        if let RefreshAction::FetchTab {
            id, name, tab_type, ..
        } = action
            && matches!(tab_type.as_str(), "MapStash" | "UniqueStash")
            && !plan
                .actions
                .iter()
                .any(|a| matches!(a, RefreshAction::FetchSubstash { parent, .. } if parent == id))
        {
            out.push_str(&format!(
                "pages of {}: none to fetch on record — pages not known yet arrive with its fetch; apply, then select it again for them\n",
                label(name)
            ));
        }
    }
    out
}

fn render_header(plan: &RefreshPlan, now: i64, audit: bool) -> String {
    let account = plan.account_name.as_deref().unwrap_or(&plan.account_uuid);
    if !audit {
        let plan_age = (now - plan.generated_at).max(0);
        let age = if plan_age < 90 {
            String::new()
        } else {
            format!(" (plan {} old)", dur(plan_age))
        };
        let mut out = format!(
            "refresh {}{} on {} as {} — {}{age}\n",
            store_cmd::realm_prefix(plan.realm),
            plan.league,
            plan.provider,
            account,
            authority(plan),
        );
        let has_tabs = plan.actions.iter().any(|a| {
            matches!(
                a,
                RefreshAction::ListStashes { .. }
                    | RefreshAction::FetchTab { .. }
                    | RefreshAction::FetchSubstash { .. }
            )
        }) || !plan.skipped_tabs.is_empty()
            || !plan.unknown_tabs.is_empty();
        let has_characters = plan.actions.iter().any(|a| {
            matches!(
                a,
                RefreshAction::ListCharacters { .. } | RefreshAction::FetchCharacter { .. }
            )
        }) || !plan.skipped_characters.is_empty()
            || !plan.unknown_characters.is_empty();
        let stash_age = plan
            .basis
            .stash_listing
            .as_ref()
            .map(|l| dur(plan.generated_at - l.fetched_at));
        let character_age = plan
            .basis
            .character_listing
            .as_ref()
            .map(|l| dur(plan.generated_at - l.fetched_at));
        let facts = match (has_tabs, has_characters, stash_age, character_age) {
            (true, true, Some(stash), Some(characters)) if stash == characters => {
                format!("stash and characters listed {stash} ago")
            }
            (true, true, stash, characters) => format!(
                "stash {}; characters {}",
                basis_age(stash),
                basis_age(characters)
            ),
            (true, false, stash, _) => format!("stash {}", basis_age(stash)),
            (false, true, _, characters) => {
                format!("characters {}", basis_age(characters))
            }
            (false, false, _, _) => "no covered facts".into(),
        };
        out.push_str(&format!("facts: {facts}\n\n"));
        return out;
    }
    let mut out = format!(
        "refresh plan: {}{} on {} as {} — {}\n",
        store_cmd::realm_prefix(plan.realm),
        plan.league,
        plan.provider,
        account,
        match plan.basis.policy_revision {
            Some(revision) => format!("policy revision {revision}"),
            None => authority(plan),
        },
    );
    // Both bases are the facts as of the read; which facets the policy
    // covers shows in the actions and skips, not here.
    let stash = match &plan.basis.stash_listing {
        Some(l) => format!(
            "stash listing {} old (response {})",
            dur(plan.generated_at - l.fetched_at),
            l.response_id
        ),
        None => "stashes never listed".into(),
    };
    let characters = match &plan.basis.character_listing {
        Some(l) => format!(
            "character listing {} old (response {})",
            dur(plan.generated_at - l.fetched_at),
            l.response_id
        ),
        None => "characters never listed".into(),
    };
    out.push_str(&format!("basis: {stash}, {characters}\n\n"));
    out
}

fn basis_age(age: Option<String>) -> String {
    age.map_or_else(|| "never listed".into(), |age| format!("listed {age} ago"))
}

/// The verdict line over the actions: the counts, the wire range, and the
/// prerequisites the envelope names (verbatim — they are the plan's).
fn render_verdict(plan: &RefreshPlan) -> String {
    let prerequisites = match plan.wire_sends.prerequisites.as_slice() {
        [] => String::new(),
        [one] => format!(" (plus {one})"),
        [rest @ .., last] => format!(" (plus {}, and {last})", rest.join(", ")),
    };
    format!(
        "{}, {}..{} wire sends{prerequisites}:\n",
        requests(plan.logical_requests),
        plan.wire_sends.min,
        plan.wire_sends.max,
    )
}

fn render_wire(plan: &RefreshPlan) -> String {
    let range = if plan.wire_sends.min == plan.wire_sends.max {
        plan.wire_sends.min.to_string()
    } else {
        format!("{}–{}", plan.wire_sends.min, plan.wire_sends.max)
    };
    let extras = if plan.wire_sends.prerequisites.is_empty() {
        ""
    } else {
        "; plus first-contact probes and token refresh if needed"
    };
    format!("wire: {range} sends{extras}\n")
}

/// The full text of `acq refresh --plan`: header, verdict, the grouped (or
/// expanded) actions, the skips, the quote or why there is none, and what
/// to type next.
pub(crate) fn render_plan(
    plan: &RefreshPlan,
    quote_note: Option<&QuoteNote>,
    notes: &PlanNotes,
    now: i64,
    expand: bool,
    source: Option<&str>,
) -> String {
    if plan.actions.is_empty() && !expand {
        return render_nothing_to_do(plan, now) + &plan_note_lines(plan, notes, now);
    }
    let mut out = render_header(plan, now, expand);
    if plan.actions.is_empty() {
        out.push_str(&render_nothing_to_do(plan, now));
        out.push_str(&plan_note_lines(plan, notes, now));
        return out;
    }
    out.push_str(&plan_note_lines(plan, notes, now));
    out.push_str(&pages_awaiting(plan));
    let common_reason = if expand {
        None
    } else {
        common_action_reason(&plan.actions)
    };
    if expand {
        out.push_str(&render_verdict(plan));
        for action in &plan.actions {
            out.push_str(&format!("  {}\n", describe_action(action)));
        }
    } else {
        out.push_str(&format!("{}:\n", requests(plan.logical_requests)));
        out.push_str(&render_grouped_actions(
            &plan.actions,
            common_reason.is_some(),
        ));
        if let Some(reason) = common_reason {
            out.push_str(&format!("reason: {reason}\n"));
        }
    }
    out.push_str(&render_skips(plan, expand));
    out.push('\n');
    match (&plan.quote, quote_note) {
        (Some(quote), _) if expand => out.push_str(&render_quote(quote, now, true)),
        (Some(quote), _) => out.push_str(&render_decision_quote(quote, now)),
        // The note's own sentence is self-describing ("no quote: …",
        // "daemon quote rejected: …"), so no prefix here.
        (None, Some(note)) if expand => out.push_str(&format!("{note}\n")),
        (None, Some(note)) => out.push_str(&render_decision_quote_note(note)),
        (None, None) => {}
    }
    if !expand {
        out.push_str(&render_wire(plan));
    }
    // a selection compiled here is applied by the command that compiled
    // it, or by the page's Apply: no command to name but a reviewed file's
    match (source, plan.basis.policy_revision) {
        (Some(path), _) => out.push_str(&format!("next: acq refresh --apply={path}\n")),
        (None, Some(_)) => {
            out.push_str(&format!("next: acq refresh --apply{}\n", scope_flags(plan)))
        }
        (None, None) => {}
    }
    out
}

/// The skip and unknown-id lines, one per facet — counts by reason; the
/// per-entity detail is in the envelope.
fn render_skips(plan: &RefreshPlan, audit: bool) -> String {
    let mut out = String::new();
    if !plan.skipped_tabs.is_empty() {
        out.push_str(&format!("{}\n", summarize_skipped_tabs(plan, audit)));
    }
    if !plan.unknown_tabs.is_empty() {
        if audit {
            out.push_str(&format!(
                "policy names {} tab id(s) the facts lack (reported, never fetched): {}\n",
                plan.unknown_tabs.len(),
                plan.unknown_tabs.join(", ")
            ));
        } else {
            out.push_str(&format!(
                "unknown tabs (not fetched): {}\n",
                plan.unknown_tabs.join(", ")
            ));
        }
    }
    if !plan.skipped_characters.is_empty() {
        out.push_str(&format!("{}\n", summarize_skipped_characters(plan, audit)));
    }
    if !plan.unknown_characters.is_empty() {
        if audit {
            out.push_str(&format!(
                "policy names {} character id(s) the facts lack (reported, never fetched): {}\n",
                plan.unknown_characters.len(),
                plan.unknown_characters.join(", ")
            ));
        } else {
            out.push_str(&format!(
                "unknown characters (not fetched): {}\n",
                plan.unknown_characters.join(", ")
            ));
        }
    }
    out
}

/// The no-op apply's text: what is fresh and for how long, then the claim.
fn render_nothing_to_do(plan: &RefreshPlan, now: i64) -> String {
    // the league the claim is about, as every line about its work names it
    let head = format!(
        "nothing to do in {}{}",
        store_cmd::realm_prefix(plan.realm),
        plan.league
    );
    let fresh_tabs = plan
        .skipped_tabs
        .iter()
        .filter(|t| matches!(t.reason, SkipReason::Fresh { .. }))
        .count();
    let fresh_characters = plan
        .skipped_characters
        .iter()
        .filter(|c| matches!(c.reason, CharacterSkipReason::Fresh { .. }))
        .count();
    let mut fresh = Vec::new();
    if fresh_tabs > 0 {
        fresh.push(plural(fresh_tabs, "tab", "tabs"));
    }
    if fresh_characters > 0 {
        fresh.push(plural(fresh_characters, "character", "characters"));
    }
    let only_fresh = fresh_tabs == plan.skipped_tabs.len()
        && fresh_characters == plan.skipped_characters.len()
        && plan.unknown_tabs.is_empty()
        && plan.unknown_characters.is_empty();
    if !fresh.is_empty() && only_fresh {
        let plan_age = (now - plan.generated_at).max(0);
        if plan_age >= 90 {
            return format!(
                "{head}: this {}-old plan found {} fresh (under {})\n",
                dur(plan_age),
                fresh.join(" and "),
                policy_window(plan.max_age_seconds)
            );
        }
        return format!(
            "{head}: {} {} fresh (under {})\n",
            fresh.join(" and "),
            if fresh_tabs + fresh_characters == 1 {
                "is"
            } else {
                "are"
            },
            policy_window(plan.max_age_seconds)
        );
    }
    let plan_age = (now - plan.generated_at).max(0);
    let has_covered_entries = !plan.skipped_tabs.is_empty() || !plan.skipped_characters.is_empty();
    let mut out = if has_covered_entries && plan_age < 90 {
        if fresh.is_empty() {
            format!("{head}: covered entries do not need fetching\n")
        } else {
            format!(
                "{head}: covered entries are fresh (under {}) or do not need fetching\n",
                policy_window(plan.max_age_seconds)
            )
        }
    } else if has_covered_entries {
        if fresh.is_empty() {
            format!(
                "{head}: this {}-old plan found covered entries that did not need fetching\n",
                dur(plan_age)
            )
        } else {
            format!(
                "{head}: this {}-old plan found covered entries fresh (under {}) or not needing a fetch\n",
                dur(plan_age),
                policy_window(plan.max_age_seconds)
            )
        }
    } else if plan_age < 90 {
        format!("{head}: no requests authorized\n")
    } else {
        format!(
            "{head}: this {}-old plan authorizes no requests\n",
            dur(plan_age)
        )
    };
    out.push_str(&render_skips(plan, false));
    out
}

/// One fetch in a group, for the grouped renderer.
struct Entity<'a> {
    /// The decision-view handle and name. Types stay in the audit view.
    label: String,
    reason: &'a FetchReason,
    /// A substash's parent tab id.
    parent: Option<&'a str>,
}

/// Actions grouped by kind and parent, counted by reason (the ruling's
/// rule 2): listings stay single lines; fetches of the same kind form a
/// group, listed one per line up to [`LIST_UP_TO`] and counted beyond,
/// substashes broken down by parent. The order is the planner's: the tab
/// facet's block, then the character facet's.
fn render_grouped_actions(actions: &[RefreshAction], suppress_reasons: bool) -> String {
    let mut rows: Vec<(usize, String, String)> = Vec::new();
    let mut tabs: Vec<Entity> = Vec::new();
    let mut substashes: Vec<Entity> = Vec::new();
    let mut characters: Vec<Entity> = Vec::new();
    // Substash counts per parent, for the parents' own lines.
    let mut under: BTreeMap<&str, usize> = BTreeMap::new();
    for action in actions {
        if let RefreshAction::FetchSubstash { parent, .. } = action {
            *under.entry(parent.as_str()).or_default() += 1;
        }
    }
    let flush_tabs = |rows: &mut Vec<(usize, String, String)>,
                      tabs: &mut Vec<Entity>,
                      substashes: &mut Vec<Entity>| {
        // The substash qualifier reads the tab group before it is emitted.
        // If every parent is named there, their child counts need not be
        // repeated below the substash total.
        let parents: std::collections::BTreeSet<&str> =
            substashes.iter().filter_map(|e| e.parent).collect();
        let named_above = parents
            .iter()
            .filter(|p| tabs.iter().any(|t| t.label.starts_with(&format!("{p} "))))
            .count();
        let qualifier = if !parents.is_empty() && named_above == parents.len() {
            format!(" under {} tabs", parents.len())
        } else {
            format!(" under {}", plural(parents.len(), "tab", "tabs"))
        };
        if !tabs.is_empty() {
            emit_group(
                rows,
                tabs,
                "tab",
                "tabs",
                "",
                Some(&under),
                suppress_reasons,
            );
        }
        if !substashes.is_empty() {
            emit_group(
                rows,
                substashes,
                "substash",
                "substashes",
                &qualifier,
                (named_above != parents.len()).then_some(&under),
                suppress_reasons,
            );
        }
        tabs.clear();
        substashes.clear();
    };
    for action in actions {
        match action {
            RefreshAction::ListStashes { reason, .. } => {
                flush_tabs(&mut rows, &mut tabs, &mut substashes);
                rows.push((
                    2,
                    "list stashes".into(),
                    if suppress_reasons {
                        String::new()
                    } else {
                        listing_why(reason)
                    },
                ));
            }
            RefreshAction::FetchTab {
                id, name, reason, ..
            } => tabs.push(Entity {
                label: format!("{id}  {}", compact_label(name)),
                reason,
                parent: None,
            }),
            RefreshAction::FetchSubstash {
                parent,
                id,
                name,
                reason,
                ..
            } => substashes.push(Entity {
                label: format!("{parent}/{id}  {}", compact_label(name)),
                reason,
                parent: Some(parent),
            }),
            RefreshAction::ListCharacters { reason, .. } => {
                flush_tabs(&mut rows, &mut tabs, &mut substashes);
                rows.push((
                    2,
                    "list characters".into(),
                    if suppress_reasons {
                        String::new()
                    } else {
                        listing_why(reason)
                    },
                ));
            }
            RefreshAction::FetchCharacter {
                id,
                name,
                league,
                reason,
                ..
            } => {
                flush_tabs(&mut rows, &mut tabs, &mut substashes);
                characters.push(Entity {
                    label: format!("{id}  {}", compact_label(name)),
                    reason,
                    parent: None,
                });
                // The league qualifier rides on the group; every character
                // action in one plan is in the envelope's league.
                let _ = league;
            }
        }
    }
    flush_tabs(&mut rows, &mut tabs, &mut substashes);
    if !characters.is_empty() {
        emit_group(
            &mut rows,
            &mut characters,
            "character",
            "characters",
            "",
            None,
            suppress_reasons,
        );
    }
    columns(&rows)
}

/// One group: its count line with the breakdown by reason, then either
/// every entity (small groups) or the per-parent breakdown (substashes).
fn emit_group(
    rows: &mut Vec<(usize, String, String)>,
    entities: &mut Vec<Entity>,
    one: &str,
    many: &str,
    qualifier: &str,
    under: Option<&BTreeMap<&str, usize>>,
    suppress_reasons: bool,
) {
    let n = entities.len();
    rows.push((
        2,
        format!("fetch {}{qualifier}", plural(n, one, many)),
        if suppress_reasons {
            String::new()
        } else {
            reason_breakdown(entities.iter().map(|e| e.reason))
        },
    ));
    if n <= LIST_UP_TO {
        for e in entities.iter() {
            let mut right = if suppress_reasons {
                String::new()
            } else {
                fetch_why(e.reason)
            };
            if e.parent.is_none()
                && let Some(id) = e.label.split(' ').next()
                && let Some(k) = under.and_then(|counts| counts.get(id))
            {
                right.push_str(&format!(
                    "  +{} below",
                    plural(*k, "substash", "substashes")
                ));
            }
            rows.push((4, e.label.clone(), right));
        }
    } else if under.is_some() && entities.iter().any(|e| e.parent.is_some()) {
        // Counted substashes: which parents, how many each — the two
        // facts a reviewer wants before spending sixty-four requests.
        let mut per_parent: Vec<(&str, usize)> = Vec::new();
        for e in entities.iter() {
            let Some(p) = e.parent else { continue };
            match per_parent.iter_mut().find(|(id, _)| *id == p) {
                Some((_, k)) => *k += 1,
                None => per_parent.push((p, 1)),
            }
        }
        let parts: Vec<String> = per_parent
            .iter()
            .map(|(p, k)| format!("{k} under {p}"))
            .collect();
        if parts.len() <= 2 {
            rows.push((4, parts.join(", "), String::new()));
        } else {
            for part in parts {
                rows.push((4, part, String::new()));
            }
        }
    }
    entities.clear();
}

/// A reason shared by every action belongs once below the action block, not
/// once on every group and entity. Comparisons use the rendered age because
/// the decision view intentionally speaks in those human-scale buckets.
fn common_action_reason(actions: &[RefreshAction]) -> Option<String> {
    let stale_ages: Option<Vec<String>> = actions
        .iter()
        .map(|action| match action {
            RefreshAction::ListStashes {
                reason: ListingReason::Stale { age_seconds },
                ..
            }
            | RefreshAction::ListCharacters {
                reason: ListingReason::Stale { age_seconds },
                ..
            }
            | RefreshAction::FetchTab {
                reason: FetchReason::Stale { age_seconds },
                ..
            }
            | RefreshAction::FetchSubstash {
                reason: FetchReason::Stale { age_seconds },
                ..
            }
            | RefreshAction::FetchCharacter {
                reason: FetchReason::Stale { age_seconds },
                ..
            } => Some(dur(*age_seconds)),
            _ => None,
        })
        .collect();
    if let Some(ages) = stale_ages
        && let Some(first) = ages.first()
        && ages.iter().all(|age| age == first)
    {
        return Some(format!("all facts are {first} old"));
    }

    let all_never_listed = actions.iter().all(|action| {
        matches!(
            action,
            RefreshAction::ListStashes {
                reason: ListingReason::NeverListed,
                ..
            } | RefreshAction::ListCharacters {
                reason: ListingReason::NeverListed,
                ..
            }
        )
    });
    if all_never_listed {
        let stash = actions
            .iter()
            .any(|a| matches!(a, RefreshAction::ListStashes { .. }));
        let characters = actions
            .iter()
            .any(|a| matches!(a, RefreshAction::ListCharacters { .. }));
        return Some(
            match (stash, characters) {
                (true, true) => "stash and characters never listed",
                (true, false) => "stash never listed",
                (false, true) => "characters never listed",
                (false, false) => return None,
            }
            .into(),
        );
    }

    let all_never_fetched = actions.iter().all(|action| {
        matches!(
            action,
            RefreshAction::FetchTab {
                reason: FetchReason::NeverFetched,
                ..
            } | RefreshAction::FetchSubstash {
                reason: FetchReason::NeverFetched,
                ..
            } | RefreshAction::FetchCharacter {
                reason: FetchReason::NeverFetched,
                ..
            }
        )
    });
    if all_never_fetched {
        let mut kinds = Vec::new();
        if actions
            .iter()
            .any(|a| matches!(a, RefreshAction::FetchTab { .. }))
        {
            kinds.push("tabs");
        }
        if actions
            .iter()
            .any(|a| matches!(a, RefreshAction::FetchSubstash { .. }))
        {
            kinds.push("substashes");
        }
        if actions
            .iter()
            .any(|a| matches!(a, RefreshAction::FetchCharacter { .. }))
        {
            kinds.push("characters");
        }
        let subjects = match kinds.as_slice() {
            [one] => (*one).to_string(),
            [one, two] => format!("{one} and {two}"),
            [one, two, three] => format!("{one}, {two}, and {three}"),
            _ => return None,
        };
        return Some(format!("selected {subjects} never fetched"));
    }
    None
}

/// `5 stale (13h)`, `64 stale (13h–14h)`, `41 never fetched`, `3 stale
/// (2d), 2 listing disagrees` — kinds in order of first appearance.
fn reason_breakdown<'a>(reasons: impl Iterator<Item = &'a FetchReason>) -> String {
    // (label, count, min age, max age)
    let mut kinds: Vec<(&'static str, usize, i64, i64)> = Vec::new();
    for r in reasons {
        let (label, age) = match r {
            FetchReason::NeverFetched => ("never fetched", None),
            FetchReason::Selected => ("selected", None),
            FetchReason::Stale { age_seconds } => ("stale", Some(*age_seconds)),
            FetchReason::ListedCountDisagrees { .. } => ("listing disagrees", None),
            FetchReason::ListedExperienceDisagrees { .. } => ("played since listed", None),
            FetchReason::ListedLeagueDisagrees { .. } => ("league changed", None),
        };
        match kinds.iter_mut().find(|(l, ..)| *l == label) {
            Some((_, n, lo, hi)) => {
                *n += 1;
                if let Some(a) = age {
                    *lo = (*lo).min(a);
                    *hi = (*hi).max(a);
                }
            }
            None => kinds.push((label, 1, age.unwrap_or(0), age.unwrap_or(0))),
        }
    }
    kinds
        .iter()
        .map(|(label, n, lo, hi)| {
            if *label == "stale" {
                let (lo, hi) = (dur(*lo), dur(*hi));
                if lo == hi {
                    format!("{n} stale ({lo})")
                } else {
                    format!("{n} stale ({lo}–{hi})")
                }
            } else {
                format!("{n} {label}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn listing_why(reason: &ListingReason) -> String {
    match reason {
        ListingReason::NeverListed => "never listed".into(),
        ListingReason::Stale { age_seconds } => stale(*age_seconds),
        ListingReason::Selected => "the selection is read from it".into(),
    }
}

/// The expanded form: one line per action, the verb first.
fn describe_action(action: &RefreshAction) -> String {
    match action {
        RefreshAction::ListStashes { league, reason, .. } => {
            format!("list stashes {league:<30} {}", listing_why(reason))
        }
        RefreshAction::FetchTab {
            id,
            name,
            tab_type,
            reason,
            ..
        } => format!(
            "fetch tab {:<34} {}",
            format!("{id} {} ({tab_type})", label(name)),
            fetch_why(reason)
        ),
        RefreshAction::FetchSubstash {
            parent,
            id,
            name,
            tab_type,
            reason,
            ..
        } => format!(
            "fetch substash {:<29} {}",
            format!("{parent}/{id} {} ({tab_type})", label(name)),
            fetch_why(reason)
        ),
        RefreshAction::ListCharacters { realm, reason } => format!(
            "list characters {:<27} {}",
            format!("({realm}, realm-wide)"),
            listing_why(reason)
        ),
        // The full id: matching is exact, and a prefix cannot be pasted
        // into a policy (C55).
        RefreshAction::FetchCharacter {
            id, name, reason, ..
        } => format!(
            "fetch character {:<28} {}",
            format!("{id} {}", label(name)),
            fetch_why(reason)
        ),
    }
}

fn label(name: &str) -> String {
    if name.is_empty() {
        "(unnamed)".into()
    } else {
        format!("{name:?}")
    }
}

fn compact_label(name: &str) -> &str {
    if name.is_empty() { "(unnamed)" } else { name }
}

/// A saturated age from the planner, human-shaped: "45s", "12m", "3h", "9d".
fn dur(seconds: i64) -> String {
    let s = seconds.max(0);
    if s < 90 {
        format!("{s}s")
    } else if s < 5400 {
        format!("{}m", s / 60)
    } else if s < 172_800 {
        format!("{}h", s / 3600)
    } else {
        format!("{}d", s / 86400)
    }
}

fn policy_window(seconds: u32) -> String {
    if seconds >= 86_400 && seconds.is_multiple_of(86_400) {
        format!("{}d", seconds / 86_400)
    } else if seconds >= 3_600 && seconds.is_multiple_of(3_600) {
        format!("{}h", seconds / 3_600)
    } else if seconds >= 60 && seconds.is_multiple_of(60) {
        format!("{}m", seconds / 60)
    } else {
        format!("{seconds}s")
    }
}

fn stale(age_seconds: i64) -> String {
    format!("stale for {}", dur(age_seconds))
}

fn fetch_why(reason: &FetchReason) -> String {
    match reason {
        FetchReason::NeverFetched => "never fetched".into(),
        FetchReason::Selected => "selected".into(),
        FetchReason::Stale { age_seconds } => stale(*age_seconds),
        FetchReason::ListedCountDisagrees { listed, held } => {
            format!("listing says {listed} item(s), store holds {held}")
        }
        FetchReason::ListedExperienceDisagrees { listed, held } => {
            format!("listing says experience {listed}, last fetch had {held} — played since")
        }
        FetchReason::ListedLeagueDisagrees { listed, held } => {
            format!("listing says league {listed}, last fetch said {held}")
        }
    }
}

/// One line of counts by reason — a real account skips dozens of fresh
/// characters, and the full per-character detail is in the JSON envelope.
fn summarize_skipped_characters(plan: &RefreshPlan, audit: bool) -> String {
    let (mut fresh, mut awaiting, mut no_league, mut deleted, mut expired) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    for c in &plan.skipped_characters {
        match c.reason {
            CharacterSkipReason::Fresh { .. } => fresh += 1,
            CharacterSkipReason::AwaitingListing => awaiting += 1,
            CharacterSkipReason::NoLeague => no_league += 1,
            CharacterSkipReason::Deleted => deleted += 1,
            CharacterSkipReason::Expired => expired += 1,
        }
    }
    let mut parts = Vec::new();
    if fresh > 0 {
        parts.push(format!("{fresh} fresh"));
    }
    if awaiting > 0 {
        parts.push(format!("{awaiting} awaiting the listing"));
    }
    if no_league > 0 {
        parts.push(format!(
            "{no_league} with no league — outside every league's coverage"
        ));
    }
    if deleted > 0 {
        parts.push(format!("{deleted} listed as deleted — never fetched"));
    }
    if expired > 0 {
        parts.push(format!("{expired} listed as expired — never fetched"));
    }
    let noun = if audit {
        "covered character(s)"
    } else if plan.skipped_characters.len() == 1 {
        "character"
    } else {
        "characters"
    };
    let detail = if audit {
        " (per-character reasons in --json)"
    } else {
        ""
    };
    format!(
        "skipped {} {noun}: {}{detail}",
        plan.skipped_characters.len(),
        parts.join(", ")
    )
}

/// One line of counts by reason — a real league skips hundreds of fresh
/// tabs, and the full per-tab detail is in the JSON envelope.
fn summarize_skipped_tabs(plan: &RefreshPlan, audit: bool) -> String {
    let (mut fresh, mut folders, mut awaiting, mut orphaned, mut empty) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    for tab in &plan.skipped_tabs {
        match tab.reason {
            SkipReason::Fresh { .. } => fresh += 1,
            SkipReason::Folder => folders += 1,
            SkipReason::AwaitingListing => awaiting += 1,
            SkipReason::OrphanedParent { .. } => orphaned += 1,
            SkipReason::EmptyStub => empty += 1,
        }
    }
    let mut parts = Vec::new();
    if fresh > 0 {
        parts.push(format!("{fresh} fresh"));
    }
    if folders > 0 {
        parts.push(format!(
            "{} — {} only",
            plural(folders, "folder", "folders"),
            if folders == 1 {
                "container"
            } else {
                "containers"
            }
        ));
    }
    if awaiting > 0 {
        parts.push(format!("{awaiting} awaiting the listing"));
    }
    if orphaned > 0 {
        parts.push(format!("{orphaned} with an off-record parent"));
    }
    if empty > 0 {
        parts.push(format!("{empty} empty substash stub(s) — nothing to fetch"));
    }
    let noun = if audit {
        "covered tab(s)"
    } else if plan.skipped_tabs.len() == 1 {
        "tab"
    } else {
        "tabs"
    };
    let detail = if audit {
        " (per-tab reasons in --json)"
    } else {
        ""
    };
    format!(
        "skipped {} {noun}: {}{detail}",
        plan.skipped_tabs.len(),
        parts.join(", ")
    )
}

/// The decision view's quote line: the cause, read from the note's
/// variant, in this view's words.
fn render_decision_quote_note(note: &QuoteNote) -> String {
    let cause = match note {
        QuoteNote::NoDaemon => "no daemon running".to_string(),
        QuoteNote::NoAccountName => "the facts record no account name to quote as".into(),
        QuoteNote::Timeout(limit) => format!("the daemon did not answer within {limit:?}"),
        QuoteNote::Rejected(e) => format!("daemon quote rejected: {e}"),
        QuoteNote::Other(why) => why.clone(),
    };
    format!("quote: unavailable — {cause}\n")
}

fn quote_scope_unlearned(scope: &QuoteScope) -> bool {
    scope
        .notes
        .iter()
        .any(|note| note.contains("not yet learned"))
}

/// The decision view reports only what the quote changes for this apply.
/// Rate windows, observations, exclusions and complete notes remain in the
/// audit view.
fn render_decision_quote(quote: &Quote, now: i64) -> String {
    if let Some(cause) = &quote.halted {
        return format!("quote: HALTED — {cause}\n");
    }
    if !quote.scopes.is_empty() && quote.scopes.iter().all(quote_scope_unlearned) {
        return format!(
            "quote: ETA unavailable until {} rate-limit {} learned\n",
            quote.scopes.len(),
            if quote.scopes.len() == 1 {
                "policy is"
            } else {
                "policies are"
            }
        );
    }
    if quote.scopes.is_empty() {
        return "quote: no rate-limited work\n".into();
    }

    let labels: Vec<&str> = quote
        .scopes
        .iter()
        .map(|scope| scope.key.split('@').next().unwrap_or(&scope.key))
        .collect();
    let width = labels
        .iter()
        .map(|label| label.chars().count())
        .max()
        .unwrap_or(0);
    let mut out = format!(
        "quote ({}):\n",
        store_cmd::ago(now, Some(quote.observed_at))
    );
    for (scope, label) in quote.scopes.iter().zip(labels) {
        let eta = match scope.eta_seconds {
            Some(0) => "ready".into(),
            Some(seconds) => format!("~{seconds}s"),
            None if quote_scope_unlearned(scope) => "ETA unavailable (unlearned)".into(),
            None => "ETA unavailable".into(),
        };
        let queued = if scope.queued_ahead == 0 {
            String::new()
        } else {
            format!(", {} queued", scope.queued_ahead)
        };
        out.push_str(&format!(
            "  {label:<width$}  {}{queued} — {eta}\n",
            requests(scope.requests)
        ));
    }
    out
}

/// The quote: one line per scope, the windows when the policy is known,
/// and what it does not cover on one line (the daemon's own phrases, cut at
/// their explanatory tail; `--expand` prints them whole, notes included).
fn render_quote(quote: &Quote, now: i64, expand: bool) -> String {
    let mut out = format!(
        "quote (the running daemon's estimate, observed {}):\n",
        store_cmd::ago(now, Some(quote.observed_at))
    );
    if let Some(cause) = &quote.halted {
        // Report the cause only: a tripwire halt has a 360 s post-violation
        // wait before `reset-tripwire` is appropriate, and a ceiling halt
        // cannot be reset mid-lifetime at all (the live-run skill).
        out.push_str(&format!(
            "  HALTED: {cause} — nothing sends while the halt stands, so every \
             estimate below is a floor (after a trip, wait 360 s before clearing; \
             a trip is evidence to read)\n"
        ));
    }
    let width = quote
        .scopes
        .iter()
        .map(|s| s.key.chars().count())
        .max()
        .unwrap_or(0);
    for scope in &quote.scopes {
        out.push_str(&render_scope(scope, width, expand));
    }
    if !quote.not_covered.is_empty() {
        if expand {
            for line in &quote.not_covered {
                out.push_str(&format!("  not covered: {line}\n"));
            }
        } else {
            let short: Vec<&str> = quote
                .not_covered
                .iter()
                .map(|l| l.split(" — ").next().unwrap_or(l))
                .collect();
            out.push_str(&format!("  not covered: {}\n", short.join("; ")));
        }
    }
    out
}

fn render_scope(scope: &QuoteScope, width: usize, expand: bool) -> String {
    let unlearned = scope.notes.iter().any(|n| n.contains("not yet learned"));
    let eta = match scope.eta_seconds {
        Some(0) => "ready now".into(),
        Some(s) => format!("last could dispatch in ~{s}s"),
        None if unlearned => "no ETA until its policy is learned (probe first)".into(),
        None => "no ETA".into(),
    };
    let count = format!(
        "{} request{},",
        scope.requests,
        if scope.requests == 1 { "" } else { "s" }
    );
    let mut out = format!(
        "  {:<width$}  {count:<12} {} queued ahead — {eta}\n",
        scope.key, scope.queued_ahead,
    );
    for rule in &scope.rules {
        for window in &rule.windows {
            let observed = scope
                .observed_seconds_ago
                .map(|s| format!(", seen {s}s ago"))
                .unwrap_or_default();
            let restricted = if window.restricted_secs > 0 {
                format!(", RESTRICTED {}s", window.restricted_secs)
            } else {
                String::new()
            };
            out.push_str(&format!(
                "      {} {}/{} hits per {}s{observed}{restricted}\n",
                rule.name, window.hits, window.max_hits, window.period_secs
            ));
        }
    }
    if expand || !unlearned {
        for note in &scope.notes {
            out.push_str(&format!("      note: {note}\n"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use acquisition_store::{
        AnnotationRow, RefreshSnapshot, SYNC_POLICY_KEY, SYNC_POLICY_KIND, SYNC_POLICY_SCOPE,
    };

    fn example() -> Value {
        serde_json::from_str(POLICY_EXAMPLE).unwrap()
    }

    /// A plan with no store behind it, compiled through the layer.
    fn tiny_plan() -> RefreshPlan {
        let snapshot = RefreshSnapshot {
            account_uuid: "u-cli".into(),
            account_name: Some("Alice#1234".into()),
            realm: "pc".into(),
            league: "Standard".into(),
            taken_at: 1_000,
            stash_listing: None,
            tabs: Vec::new(),
            character_listing: None,
            characters: Vec::new(),
            league_list: None,
            policy: Some(AnnotationRow {
                scope: SYNC_POLICY_SCOPE.into(),
                key: SYNC_POLICY_KEY.into(),
                kind: SYNC_POLICY_KIND.into(),
                value: example(),
                revision: 1,
                created_at: 1_000,
                updated_at: 1_000,
                written_via: "test".into(),
                actor: None,
            }),
            response_watermark: 0,
            fetches: Default::default(),
        };
        acquisition_verbs::refresh::compile("mock", &snapshot, "Alice#1234", 2_000).unwrap()
    }

    /// The CLI's own flag check, before the layer's staleness gate:
    /// flags that agree pass, ones that do not are refused naming both.
    #[test]
    fn apply_flags_that_contradict_the_plan_are_refused() {
        let plan = tiny_plan();
        check_flags(&plan, None, None).unwrap();
        check_flags(&plan, Some(Realm::Pc), Some("Standard")).unwrap();
        let err = check_flags(&plan, None, Some("Hardcore")).unwrap_err();
        assert!(err.to_string().contains("Standard"), "{err}");
        let err = check_flags(&plan, Some(Realm::Xbox), None).unwrap_err();
        assert!(err.to_string().contains("--realm xbox"), "{err}");
    }

    // ---- C53: decision, audit and contract views ----

    fn tab(id: &str, name: &str, reason: FetchReason) -> RefreshAction {
        RefreshAction::FetchTab {
            realm: Realm::Pc,
            league: "Standard".into(),
            id: id.into(),
            name: name.into(),
            tab_type: "MapStash".into(),
            reason,
        }
    }

    fn substash(parent: &str, id: &str, reason: FetchReason) -> RefreshAction {
        RefreshAction::FetchSubstash {
            realm: Realm::Pc,
            league: "Standard".into(),
            parent: parent.into(),
            id: id.into(),
            name: "1 (Remove-only)".into(),
            tab_type: "MapStash".into(),
            reason,
        }
    }

    fn character(i: usize) -> RefreshAction {
        RefreshAction::FetchCharacter {
            realm: Realm::Pc,
            league: "Standard".into(),
            id: format!("{i:0>64x}"),
            name: format!("Exile{i}"),
            reason: FetchReason::NeverFetched,
        }
    }

    /// The 2026-09-02 shape in miniature: a listing, tabs with substashes
    /// interleaved after their parents, the character listing, and more
    /// characters than the threshold.
    fn wall() -> Vec<RefreshAction> {
        let stale = |h: i64| FetchReason::Stale {
            age_seconds: h * 3600,
        };
        let mut actions = vec![
            RefreshAction::ListStashes {
                realm: Realm::Pc,
                league: "Standard".into(),
                reason: ListingReason::Stale {
                    age_seconds: 13 * 3600,
                },
            },
            tab("dump", "Dump Tab", stale(13)),
            tab("maps", "Maps (Remove-only)", stale(13)),
        ];
        for i in 0..8 {
            actions.push(substash("maps", &format!("m{i}"), stale(14)));
        }
        actions.push(tab("uniq", "Uniques 1 (Remove-only)", stale(13)));
        for i in 0..4 {
            actions.push(substash("uniq", &format!("u{i}"), stale(13)));
        }
        actions.push(RefreshAction::ListCharacters {
            realm: Realm::Pc,
            reason: ListingReason::Stale {
                age_seconds: 13 * 3600,
            },
        });
        for i in 0..12 {
            actions.push(character(i));
        }
        actions
    }

    #[test]
    fn grouping_counts_large_groups_and_lists_small_ones() {
        let text = render_grouped_actions(&wall(), false);
        // Listings stay single lines in the planner's order.
        assert!(text.starts_with("  list stashes"), "{text}");
        // Three tabs: under the threshold, so each is listed with its
        // reason, and the parents say how many substashes follow.
        assert!(text.contains("fetch 3 tabs"), "{text}");
        assert!(text.contains("3 stale (13h)"), "{text}");
        assert!(text.contains("maps  Maps (Remove-only)"), "{text}");
        assert!(!text.contains("MapStash"), "{text}");
        assert!(text.contains("+8 substashes below"), "{text}");
        assert!(text.contains("+4 substashes below"), "{text}");
        // Twelve substashes: over the threshold — counted, broken down by
        // parent and by reason with the age range, never listed.
        assert!(text.contains("fetch 12 substashes under 2 tabs"), "{text}");
        assert!(text.contains("12 stale (13h–14h)"), "{text}");
        assert!(!text.contains("8 under maps, 4 under uniq"), "{text}");
        assert!(!text.contains("maps/m0"), "{text}");
        // Twelve characters: counted, no ids in the default view.
        assert!(text.contains("list characters"), "{text}");
        assert!(text.contains("fetch 12 characters"), "{text}");
        assert!(text.contains("12 never fetched"), "{text}");
        assert!(!text.contains("Exile0"), "{text}");
        // Order: the tab facet's block, then the character facet's.
        let tabs_at = text.find("fetch 3 tabs").unwrap();
        let subs_at = text.find("fetch 12 substashes").unwrap();
        let chars_at = text.find("fetch 12 characters").unwrap();
        assert!(tabs_at < subs_at && subs_at < chars_at, "{text}");
    }

    #[test]
    fn small_groups_list_every_entity_with_its_reason() {
        let actions = vec![
            tab("a", "A", FetchReason::NeverFetched),
            tab(
                "b",
                "B",
                FetchReason::ListedCountDisagrees {
                    listed: 92,
                    held: 90,
                },
            ),
            character(1),
        ];
        let text = render_grouped_actions(&actions, false);
        assert!(text.contains("fetch 2 tabs"), "{text}");
        assert!(
            text.contains("1 never fetched, 1 listing disagrees"),
            "{text}"
        );
        assert!(text.contains("b  B"), "{text}");
        assert!(
            text.contains("listing says 92 item(s), store holds 90"),
            "{text}"
        );
        assert!(text.contains("fetch 1 character"), "{text}");
        assert!(text.contains("Exile1"), "{text}");
    }

    #[test]
    fn the_decision_view_states_shared_context_once() {
        let actions = vec![
            RefreshAction::ListStashes {
                realm: Realm::Pc,
                league: "Standard".into(),
                reason: ListingReason::Stale {
                    age_seconds: 3 * 3600,
                },
            },
            tab(
                "dump",
                "Dump Tab",
                FetchReason::Stale {
                    age_seconds: 3 * 3600,
                },
            ),
        ];
        assert_eq!(
            common_action_reason(&actions).as_deref(),
            Some("all facts are 3h old")
        );
        let text = render_grouped_actions(&actions, true);
        assert_eq!(text.matches("3h").count(), 0, "{text}");
        assert_eq!(text.matches("stale").count(), 0, "{text}");
        assert!(text.contains("list stashes"), "{text}");
        assert!(text.contains("dump  Dump Tab"), "{text}");
    }

    #[test]
    fn c53_never_reasons_distinguish_listing_from_fetches() {
        let listings = vec![
            RefreshAction::ListStashes {
                realm: Realm::Pc,
                league: "Standard".into(),
                reason: ListingReason::NeverListed,
            },
            RefreshAction::ListCharacters {
                realm: Realm::Pc,
                reason: ListingReason::NeverListed,
            },
        ];
        assert_eq!(
            common_action_reason(&listings).as_deref(),
            Some("stash and characters never listed")
        );

        let fetches = vec![
            tab("a", "A", FetchReason::NeverFetched),
            substash("a", "s", FetchReason::NeverFetched),
            character(1),
        ];
        assert_eq!(
            common_action_reason(&fetches).as_deref(),
            Some("selected tabs, substashes, and characters never fetched")
        );

        let mixed = vec![listings[0].clone(), fetches[0].clone()];
        assert_eq!(common_action_reason(&mixed), None);
    }

    #[test]
    fn the_decision_quote_collapses_repeated_unlearned_policies() {
        let scopes = [
            ("stash-list@Alice#1234", 1),
            ("stash@Alice#1234", 69),
            ("character-list@Alice#1234", 1),
            ("character@Alice#1234", 41),
        ]
        .into_iter()
        .map(|(key, requests)| QuoteScope {
            key: key.into(),
            endpoints: Vec::new(),
            requests,
            queued_ahead: 0,
            policy: None,
            rules: Vec::new(),
            observed_seconds_ago: None,
            eta_seconds: None,
            notes: vec!["policy not yet learned; probe first".into()],
        })
        .collect();
        let quote = Quote {
            observed_at: 2_000,
            provider: "ggg".into(),
            account: Some("Alice#1234".into()),
            halted: None,
            work: Vec::new(),
            scopes,
            not_covered: Vec::new(),
        };
        assert_eq!(
            render_decision_quote(&quote, 2_000),
            "quote: ETA unavailable until 4 rate-limit policies are learned\n"
        );
    }

    #[test]
    fn the_expanded_form_keeps_one_line_per_action_and_the_default_groups_them() {
        let mut plan = tiny_plan();
        plan.actions = wall();
        plan.logical_requests = plan.actions.len() as u64;
        let grouped = render_plan(
            &plan,
            Some(&QuoteNote::NoDaemon),
            &PlanNotes::default(),
            2_000,
            false,
            None,
        );
        let expanded = render_plan(
            &plan,
            Some(&QuoteNote::NoDaemon),
            &PlanNotes::default(),
            2_000,
            true,
            None,
        );
        assert_eq!(
            expanded.matches("\n  fetch ").count(),
            27,
            "one line per fetch:\n{expanded}"
        );
        assert!(grouped.lines().count() < expanded.lines().count());
        // Both end with the next step; the default is the decision view and
        // the expanded form retains the audit detail.
        for text in [&grouped, &expanded] {
            assert!(text.contains("next: acq refresh --apply"), "{text}");
            assert!(!text.contains("os error"), "{text}");
        }
        assert!(
            grouped.contains("quote: unavailable — no daemon running"),
            "{grouped}"
        );
        assert!(grouped.contains("wire:"), "{grouped}");
        assert!(!grouped.contains("response "), "{grouped}");
        assert!(!grouped.contains("MapStash"), "{grouped}");
        assert!(
            !grouped.contains("--expand lists every action"),
            "{grouped}"
        );
        assert_eq!(expanded.matches("no quote").count(), 1, "{expanded}");
        assert!(expanded.contains("basis:"), "{expanded}");
        assert!(expanded.contains("MapStash"), "{expanded}");
        assert!(expanded.contains("HEAD probe"), "{expanded}");
        // Rendered from a file, the next step names that file.
        let from_file = render_plan(
            &plan,
            None,
            &PlanNotes::default(),
            2_000,
            false,
            Some("/tmp/p.json"),
        );
        assert!(
            from_file.contains("next: acq refresh --apply=/tmp/p.json"),
            "{from_file}"
        );
        let old = render_plan(
            &plan,
            None,
            &PlanNotes::default(),
            9_200,
            false,
            Some("/tmp/p.json"),
        );
        assert!(old.contains("on mock as Alice#1234"), "{old}");
        assert!(old.contains("(plan 2h old)"), "{old}");
    }

    #[test]
    fn an_empty_plan_says_what_is_fresh() {
        let mut plan = tiny_plan();
        plan.skipped_tabs = (0..69)
            .map(|i| acquisition_plan::SkippedTab {
                id: format!("t{i}"),
                name: String::new(),
                reason: SkipReason::Fresh { age_seconds: 60 },
            })
            .collect();
        plan.skipped_characters = (0..41)
            .map(|i| acquisition_plan::SkippedCharacter {
                id: format!("c{i}"),
                name: String::new(),
                reason: CharacterSkipReason::Fresh { age_seconds: 60 },
            })
            .collect();
        plan.actions.clear();
        plan.logical_requests = 0;
        let text = render_nothing_to_do(&plan, plan.generated_at);
        assert_eq!(
            text,
            "nothing to do in Standard: 69 tabs and 41 characters are fresh (under 1h)\n"
        );
        assert_eq!(
            render_nothing_to_do(&plan, plan.generated_at + 7_200),
            "nothing to do in Standard: this 2h-old plan found 69 tabs and 41 characters fresh (under 1h)\n"
        );

        plan.skipped_tabs.push(acquisition_plan::SkippedTab {
            id: "folder".into(),
            name: "Container".into(),
            reason: SkipReason::Folder,
        });
        assert_eq!(
            render_nothing_to_do(&plan, plan.generated_at),
            concat!(
                "nothing to do in Standard: covered entries are fresh (under 1h) or do not need fetching\n",
                "skipped 70 tabs: 69 fresh, 1 folder — container only\n",
                "skipped 41 characters: 41 fresh\n",
            )
        );

        plan.skipped_tabs.retain(|tab| tab.id == "folder");
        plan.skipped_characters.clear();
        assert_eq!(
            render_nothing_to_do(&plan, plan.generated_at),
            concat!(
                "nothing to do in Standard: covered entries do not need fetching\n",
                "skipped 1 tab: 1 folder — container only\n",
            )
        );
        let rendered = render_plan(
            &plan,
            Some(&QuoteNote::NoDaemon),
            &PlanNotes::default(),
            2_000,
            false,
            None,
        );
        assert!(rendered.contains("nothing to do"), "{rendered}");
        assert!(!rendered.contains("next:"), "{rendered}");
        // Nothing to quote, so no quote line either way.
        assert!(!rendered.contains("quote"), "{rendered}");
    }

    #[test]
    fn store_changes_render_nonzero_kinds_only() {
        let none = StoreChanges {
            responses: 3,
            ..Default::default()
        };
        assert_eq!(
            changes_line(&none),
            "changed: no items — 3 responses recorded, nothing added, changed, moved, or removed"
        );
        let some = StoreChanges {
            responses: 112,
            added: 98,
            changed: 0,
            moved: 2,
            removed: 0,
            locations: 5,
        };
        assert_eq!(
            changes_line(&some),
            "changed: +98 added, >2 moved at 5 locations since the apply started; 112 responses recorded (acq store events)"
        );
    }
}
