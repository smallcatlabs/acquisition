//! The pricing verbs as every frontend asks them (C46): `acq price
//! status | show | list | set | clear`, `acq shop render` and `acq
//! reference currency` on the CLI, and the MCP's `price_*`,
//! `shop_render` and `reference_currency` tools, call these functions and
//! print or return what they give back — the documents the CLI's `--json`
//! prints — so the two surfaces cannot answer one request two ways.
//! Nothing here prints, resolves an account or reads a file: a frontend
//! opens the selected account's facts and intent and hands them in.
//!
//! The one difference between the surfaces is a frontend's own and is
//! named here as [`Expect`]: what a write that names no revision means.
//! The CLI's is [`Expect::Current`], replace whatever is stored, read just
//! before the write; the MCP's is [`Expect::Create`], C52's create-only
//! form, so an agent never replaces or clears a row it has not read.
//!
//! # As built
//!
//! - The refusals are anyhow errors in the CLI's words, since they were
//!   the CLI's first: a frontend reports the message whole (C11). The
//!   ones an agent branches on carry a [`Refused`] — `kind: "conflict"`
//!   with the row's current `revision` for a write refused for its
//!   revision, `kind: "not_found"` for a target the facts do not hold —
//!   which both frontends add to the failure document (C11, amended
//!   2026-10-19). A refusal speaks the verb's words, never the store's
//!   (C116, amended): a price at a revision, a price that was cleared.
//! - A cleared row keeps counting: its tombstone holds a revision and the
//!   next write's continues from it, so a refusal over one names it.
//! - The league a verb reads with none given is the one the facts hold
//!   the named target in, so an address an answer printed reads back as
//!   printed; Standard otherwise ([`league_of`]).
//! - A tab or substash address carries its realm, and a realm given
//!   beside it must agree ([`realm_of`]).
//! - A row on a target the facts do not hold is listed whole by
//!   [`report`] (`rows.unmatched`, the manual side's shape), since no
//!   listing carries it (C35); [`show`] stays a subject of the facts and
//!   refuses such a target saying it has a row and where it is listed.
//! - A blind write never lands over a row this build cannot read
//!   ([`Expect::Current`]): the prior would be intent it cannot restore.
//! - The write is last: [`set`] reads everything its receipt reports,
//!   the facts included, before the write and nothing after it, so a
//!   read that fails refuses before anything changes and no write lands
//!   whose receipt was lost. A target the facts do not hold is noted,
//!   never refused (C64).

use std::str::FromStr;

use acquisition_protocol::realm::Realm;
use acquisition_store::{
    AnnotationError, Annotations, FactAddress, IntentValue, Provenance, SYNC_POLICY_KEY,
    SYNC_POLICY_KIND, SYNC_POLICY_SCOPE, Store, check_value,
};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::{Value, json};

use crate::currency::{Currency, CurrencyTable, table};
use crate::listing::{ListFilter, ListView, ListingReport, Relation, ShowView, resolve};
use crate::price::{
    BUYOUT_KIND, BUYOUT_VERSION, Buyout, PriceTarget, PriceWrite, PriceWriteError, clear_buyout,
    set_buyout,
};
use crate::shop::{ITEMS_TOKEN, Page, PolicySource, RenderOptions, ShopRender, render};
use crate::{SyncPolicy, plan_refresh};

/// A refusal an agent branches on, beside its words (C11): what kind it
/// is, and for a write refused for its revision the row's revision now
/// — a live row's, or the one a clear left — or none when no row was
/// ever written. Found in an error's chain by [`Refused::find`]; its
/// [`Refused::fields`] join the failure document on both surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    pub kind: RefusedKind,
    pub message: String,
}

/// The kinds of [`Refused`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusedKind {
    /// A write refused for its revision; `revision` is the row's now.
    Conflict { revision: Option<i64> },
    /// A target the facts do not hold.
    NotFound,
}

impl Refused {
    fn conflict(revision: Option<i64>, message: String) -> anyhow::Error {
        anyhow::Error::new(Refused {
            kind: RefusedKind::Conflict { revision },
            message,
        })
    }

    /// The refusal an error carries, if any.
    pub fn find(e: &anyhow::Error) -> Option<&Refused> {
        e.chain().find_map(|c| c.downcast_ref::<Refused>())
    }

    /// The fields a failure document adds beside `error`: `kind`, and
    /// for a conflict `revision` (null when no row was ever written).
    pub fn fields(&self) -> serde_json::Map<String, Value> {
        let mut out = serde_json::Map::new();
        match self.kind {
            RefusedKind::Conflict { revision } => {
                out.insert("kind".into(), json!("conflict"));
                out.insert("revision".into(), json!(revision));
            }
            RefusedKind::NotFound => {
                out.insert("kind".into(), json!("not_found"));
            }
        }
        out
    }
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Refused {}

/// The league `status`, `shop render`, and a `show` or `list` naming no
/// known target read when none is given.
pub const DEFAULT_LEAGUE: &str = "Standard";

/// The state of one (realm, league), from one account's facts and intent.
pub fn report(
    store: &Store,
    annotations: &Annotations,
    realm: Realm,
    league: &str,
) -> Result<ListingReport> {
    let snapshot = store.pricing_snapshot(realm.as_str(), league, annotations)?;
    let mut report = resolve(&snapshot).context("resolving the listing state")?;
    report.place_other_leagues(|target| {
        store
            .leagues_of(&FactAddress::from(target))
            .map(|leagues| leagues.iter().any(|l| l != league))
    })?;
    Ok(report)
}

/// The league a verb reads when none was given: the one the facts hold
/// the named target in; Standard when no target is named, the facts do
/// not hold it, or (a character the listing gave no league) hold it in
/// none. A target live in several leagues is refused with their names.
pub fn league_of(store: &Store, named: &[&PriceTarget], league: Option<&str>) -> Result<String> {
    if let Some(league) = league {
        return Ok(league.into());
    }
    let Some(target) = named.first() else {
        return Ok(DEFAULT_LEAGUE.into());
    };
    let leagues = store
        .leagues_of(&FactAddress::from(*target))
        .context("reading the facts for the target's league")?;
    match leagues.as_slice() {
        [] => Ok(DEFAULT_LEAGUE.into()),
        [one] => Ok(one.clone()),
        many => bail!(
            "{target} is in several leagues ({}); name one with --league",
            many.join(", ")
        ),
    }
}

/// A target address as one word, or the grammar to type — and, for a
/// bare id as `search` and `show` print one, the two readings it may be,
/// filled in: an item's and a character's ids are alike, so neither is
/// guessed (the trade seat: cli-opus call 31, its fault 6).
pub fn read_target(word: &str) -> Result<PriceTarget> {
    PriceTarget::from_str(word).map_err(|e| {
        let readings = if word.contains('/') || word.is_empty() {
            String::new()
        } else {
            format!("; as an item, item/{word}; as a character, character/{word}")
        };
        anyhow::anyhow!(
            "{e}; a target is one word: item/<id>, character/<id>, tab/<realm>/<id> or substash/<realm>/<parent>/<id>{readings}"
        )
    })
}

/// The realm a verb reads: a tab or substash address carries its own and
/// a realm given beside it must agree; an item or character address
/// takes the given realm, default pc.
pub fn realm_of(targets: &[&PriceTarget], given: Option<Realm>) -> Result<Realm> {
    let mut realm = given;
    for t in targets {
        if let PriceTarget::Tab { realm: r, .. } | PriceTarget::Substash { realm: r, .. } = t {
            match realm {
                Some(have) if have != *r => {
                    bail!("{t} is a {r} address; {have} was given (--realm or another address)")
                }
                _ => realm = Some(*r),
            }
        }
    }
    Ok(realm.unwrap_or(Realm::DEFAULT))
}

/// The refusal for a target these facts do not hold: where the facts do
/// hold it, when they do, so it names the league that reads it, never
/// only the one that does not. `own_row` is the revision of a row the
/// target has of its own, which no listing carries: the refusal says so
/// and names the command that prints the row, as typed — `status`
/// expanded, in the target's realm, since the bare verb counts rows and
/// reads pc.
fn not_on_record(
    store: &Store,
    r: &ListingReport,
    target: &PriceTarget,
    own_row: Option<i64>,
) -> anyhow::Error {
    let elsewhere = store
        .leagues_of(&FactAddress::from(target))
        .unwrap_or_default();
    let hint = match elsewhere.as_slice() {
        [] => "`acq search` finds an item's id, `acq tabs` a tab's".to_string(),
        leagues => format!(
            "the facts hold it in {}: `--league` reads it there",
            leagues.join(", ")
        ),
    };
    let realm = match r.header.realm.segment() {
        Some(seg) => format!("{seg}/"),
        None => String::new(),
    };
    let row = match own_row {
        Some(revision) => {
            let realm = match r.header.realm.segment() {
                Some(seg) => format!(" --realm {seg}"),
                None => String::new(),
            };
            format!(
                "; it has a price row of its own at revision {revision}: \
                 `acq price status --expand{realm}` lists it"
            )
        }
        None => String::new(),
    };
    anyhow::Error::new(Refused {
        kind: RefusedKind::NotFound,
        message: format!(
            "{target} is not in the facts for {realm}{} ({} items, {} tabs and characters on record); {hint}{row}",
            r.header.league, r.counts.items, r.counts.containers
        ),
    })
}

/// `price show <target>`: one target's view, in the league the facts hold
/// it in unless one is named.
pub fn show(
    store: &Store,
    annotations: &Annotations,
    target: &str,
    realm: Option<Realm>,
    league: Option<&str>,
) -> Result<ShowView> {
    let target = read_target(target)?;
    let realm = realm_of(&[&target], realm)?;
    let league = league_of(store, &[&target], league)?;
    let r = report(store, annotations, realm, &league)?;
    match r.show_view(&target) {
        Some(view) => Ok(view),
        None => {
            let (scope, key) = target.address()?;
            let own = annotations.get(scope, &key, BUYOUT_KIND)?;
            Err(not_on_record(
                store,
                &r,
                &target,
                own.map(|row| row.revision),
            ))
        }
    }
}

/// `list`'s selection as typed: relation and effective-side words,
/// container addresses, and how many to list at most — read here into a
/// [`ListFilter`].
#[derive(Debug, Default)]
pub struct ListArgs {
    pub relation: Option<String>,
    pub effective: Option<String>,
    pub location: Option<String>,
    pub covered_by: Option<String>,
    pub limit: Option<usize>,
}

/// `price list`.
pub fn list(
    store: &Store,
    annotations: &Annotations,
    realm: Option<Realm>,
    league: Option<&str>,
    args: &ListArgs,
) -> Result<ListView> {
    const SIDES: [&str; 4] = ["game", "manual", "none", "unresolved"];
    let effective = match args.effective.as_deref() {
        None => None,
        Some(word) if SIDES.contains(&word) => Some(word.to_string()),
        Some(word) => bail!(
            "{word:?} is not an effective side (one of {})",
            SIDES.join(", ")
        ),
    };
    let relation = match args.relation.as_deref() {
        None => None,
        Some(word) => Some(Relation::parse(word).ok_or_else(|| {
            anyhow::anyhow!(
                "{word:?} is not a relation (one of {})",
                Relation::ALL.map(Relation::as_str).join(", ")
            )
        })?),
    };
    let container = |flag: &str, word: Option<&str>| -> Result<Option<PriceTarget>> {
        let Some(word) = word else {
            return Ok(None);
        };
        let t = read_target(word)?;
        if matches!(t, PriceTarget::Item { .. }) {
            bail!("{flag} takes a container (tab, substash or character), not an item");
        }
        Ok(Some(t))
    };
    let location = container("--in", args.location.as_deref())?;
    let covered_by = container("--covered-by", args.covered_by.as_deref())?;
    let named: Vec<&PriceTarget> = [&location, &covered_by].into_iter().flatten().collect();
    let realm = realm_of(&named, realm)?;
    let league = league_of(store, &named, league)?;
    let r = report(store, annotations, realm, &league)?;
    let filter = ListFilter {
        relation,
        effective,
        r#in: location.clone(),
        covered_by: covered_by.clone(),
        limit: args.limit,
    };
    match r.list_view(filter) {
        Some(view) => Ok(view),
        None => {
            let missing = [&location, &covered_by]
                .into_iter()
                .flatten()
                .find(|t| r.find(t).is_none())
                .cloned()
                .unwrap_or(PriceTarget::Item { id: "?".into() });
            // a container with a row of its own is refused as `show`
            // refuses it, naming the command that prints the row
            // (`7571a570`; issue #11, C146)
            let (scope, key) = missing.address()?;
            let own = annotations.get(scope, &key, BUYOUT_KIND)?;
            Err(not_on_record(
                store,
                &r,
                &missing,
                own.map(|row| row.revision),
            ))
        }
    }
}

/// The value words `set` takes: the type word — or the game's own
/// (`price`, `b/o`, `~price`, `~b/o`, `~skip`) — then, for a priced
/// type, the amount and the currency tag. Built into the v1 wire shape
/// and read through the value's own strict parse, so no frontend has a
/// second grammar: a fifth digit, an alias that is not a tag, an amount
/// on `skip`, each refuses in the value's words.
pub fn read_buyout(kind: &str, amount: Option<&str>, currency: Option<&str>) -> Result<Buyout> {
    let kind = match kind {
        "exact" | "price" | "~price" => "exact",
        "negotiable" | "b/o" | "~b/o" => "negotiable",
        "no_price" | "no-price" => "no_price",
        "skip" | "~skip" => "skip",
        other => bail!(
            "{other:?} is not a price type: exact (the game's ~price), negotiable (~b/o), no_price or skip"
        ),
    };
    if matches!(kind, "exact" | "negotiable") && (amount.is_none() || currency.is_none()) {
        bail!(
            "{kind} takes an amount and a currency tag: `{kind} 12.5 chaos`, `{kind} 1/5 divine`; \
             `acq reference currency` lists the tags"
        );
    }
    let mut value = json!({ "version": BUYOUT_VERSION, "type": kind });
    if let Some(amount) = amount {
        value["amount"] = json!(amount);
    }
    if let Some(currency) = currency {
        value["currency"] = json!(currency);
    }
    Buyout::parse(&value).map_err(|e| anyhow::anyhow!("{e}"))
}

/// What a write expects to find: the compare-and-swap's revision, or
/// what naming none means — the one rule the frontends differ on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expect {
    /// Only over exactly this revision; refused naming the current one
    /// otherwise.
    Exactly(i64),
    /// Whatever is stored, read just before the write, so a write racing
    /// in between still conflicts rather than being clobbered (the CLI's
    /// default). A row this build cannot read is refused, never replaced
    /// blind.
    Current,
    /// A new row only: a row standing is refused naming its revision (the
    /// MCP's default, under the create-only form).
    Create,
}

impl Expect {
    /// A frontend's revision argument under its default.
    pub fn or(revision: Option<i64>, default: Expect) -> Expect {
        revision.map_or(default, Expect::Exactly)
    }
}

/// The blind default's read: the row's current revision, or none. A row
/// this build cannot read (a newer build's value, which the listing state
/// reports as unreadable) is refused here rather than replaced — a blind
/// write over it would destroy intent this build cannot restore, and the
/// receipt's undo would be a value it cannot write. A named revision is
/// the reviewed path past it.
fn current_revision(annotations: &Annotations, target: &PriceTarget) -> Result<Option<i64>> {
    let (scope, key) = target.address()?;
    let Some(row) = annotations.get(scope, &key, BUYOUT_KIND)? else {
        return Ok(None);
    };
    if let Err(e) = check_value::<Buyout>(&row.value) {
        bail!(
            "{target} holds a value this build cannot read (revision {}: {e}); \
             not replaced blind — `--if-revision {}` replaces it deliberately, \
             `acq price show {target}` names it",
            row.revision,
            row.revision
        );
    }
    Ok(Some(row.revision))
}

/// The create-only refusal: a row stands and no revision was named.
fn standing(target: &PriceTarget, revision: i64, verb: &str) -> anyhow::Error {
    Refused::conflict(
        Some(revision),
        format!(
            "{target} has a price row of its own at revision {revision}: {verb} names the \
             revision it replaces ({revision}), never a row it has not read — \
             `acq price show {target}` reads it"
        ),
    )
}

/// Which write a refusal speaks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verb {
    Set,
    Clear,
}

/// A write refused for the row's state, in the verb's words: a live row
/// at another revision; a row a clear left, at the revision its tombstone
/// holds; or no row ever written. `current` is the live row the store
/// reported, if any; the tombstone is read here.
fn refused_write(
    annotations: &Annotations,
    target: &PriceTarget,
    current: Option<i64>,
    verb: Verb,
) -> anyhow::Error {
    if let Some(revision) = current {
        return Refused::conflict(
            Some(revision),
            format!("{target} has a price at revision {revision} (re-read and retry)"),
        );
    }
    let cleared = target
        .address()
        .ok()
        .and_then(|(scope, key)| annotations.cleared(scope, &key, BUYOUT_KIND).ok())
        .flatten();
    let message = match (cleared, verb) {
        (Some(r), Verb::Set) => {
            format!("{target}'s price was cleared (revision {r}); a new price names no revision")
        }
        (Some(r), Verb::Clear) => format!(
            "{target}'s price was cleared (revision {r}): nothing to clear \
             (`acq price show {target}` reads what covers it)"
        ),
        (None, Verb::Set) => {
            format!("{target} has no price of its own; a new price names no revision")
        }
        (None, Verb::Clear) => format!(
            "nothing to clear: {target} has no price row of its own \
             (`acq price show {target}` reads what covers it)"
        ),
    };
    Refused::conflict(cleared, message)
}

/// The store's compare-and-swap refusal as [`refused_write`]'s; any other
/// error as it was.
fn on_conflict(
    annotations: &Annotations,
    target: &PriceTarget,
    verb: Verb,
    e: PriceWriteError,
) -> anyhow::Error {
    match e {
        PriceWriteError::Store(AnnotationError::Conflict { current }) => {
            refused_write(annotations, target, current.map(|row| row.revision), verb)
        }
        other => other.into(),
    }
}

/// `set --json`: the [`PriceWrite`] plus whether the facts hold the
/// target — the text's note, as JSON. Absent from `clear`, which only
/// ever removes a row that exists.
#[derive(Debug, Serialize)]
pub struct SetReceipt {
    #[serde(flatten)]
    pub write: PriceWrite,
    pub in_facts: bool,
}

/// `price set <target> <type> [<amount> <currency>]`: one row through
/// the shared write ([`set_buyout`]) under compare-and-swap.
#[allow(clippy::too_many_arguments)]
pub fn set(
    store: &Store,
    annotations: &mut Annotations,
    target: &str,
    kind: &str,
    amount: Option<&str>,
    currency: Option<&str>,
    expect: Expect,
    provenance: &Provenance,
) -> Result<SetReceipt> {
    let target = read_target(target)?;
    let value = read_buyout(kind, amount, currency)?;
    // Everything the receipt reports is read before the write and nothing
    // after it, so a read that fails refuses before anything changes: the
    // note, whether the facts hold the target at all, in any league.
    let in_facts = store
        .holds(&FactAddress::from(&target))
        .context("reading the facts for the target")?;
    let expected = match expect {
        Expect::Exactly(revision) => Some(revision),
        Expect::Current => current_revision(annotations, &target)?,
        Expect::Create => None,
    };
    let write = match set_buyout(annotations, &target, &value, expected, provenance) {
        Err(PriceWriteError::Store(AnnotationError::Conflict { current: Some(row) }))
            if expect == Expect::Create =>
        {
            return Err(standing(&target, row.revision, "a write over it"));
        }
        Err(e) => return Err(on_conflict(annotations, &target, Verb::Set, e)),
        Ok(write) => write,
    };
    Ok(SetReceipt { write, in_facts })
}

/// `price clear <target>`: remove the row, under the same compare-and-
/// swap. A target with no row is a refusal that says so — every nothing
/// says which nothing.
pub fn clear(
    annotations: &mut Annotations,
    target: &str,
    expect: Expect,
    provenance: &Provenance,
) -> Result<PriceWrite> {
    let target = read_target(target)?;
    let expected = match expect {
        Expect::Exactly(revision) => revision,
        Expect::Current => match current_revision(annotations, &target)? {
            Some(revision) => revision,
            None => return Err(refused_write(annotations, &target, None, Verb::Clear)),
        },
        Expect::Create => {
            let (scope, key) = target.address()?;
            return Err(match annotations.get(scope, &key, BUYOUT_KIND)? {
                Some(row) => standing(&target, row.revision, "a clear"),
                None => refused_write(annotations, &target, None, Verb::Clear),
            });
        }
    };
    clear_buyout(annotations, &target, expected, provenance)
        .map_err(|e| on_conflict(annotations, &target, Verb::Clear, e))
}

/// `shop render`'s request beside the facts: the page size, the template
/// (`[items]` alone when none is given), the clock, and whether the
/// items nothing applies to are listed as well as counted.
#[derive(Debug, Clone, Copy)]
pub struct ShopArgs<'a> {
    pub size: usize,
    pub template: Option<&'a str>,
    pub now: i64,
    pub expand: bool,
}

/// `shop render`: the listing state rendered through [`render`], the sync
/// policy read for the freshness report only — and the refresh the
/// staleness line cites compiled offline as `refresh --plan` compiles it,
/// so the count it prints is that command's. `provider` is the plan's.
pub fn shop_render(
    store: &Store,
    annotations: &Annotations,
    provider: &str,
    realm: Realm,
    league: &str,
    args: &ShopArgs<'_>,
) -> Result<ShopRender> {
    let snapshot = store.pricing_snapshot(realm.as_str(), league, annotations)?;
    let report = resolve(&snapshot).context("resolving the listing state")?;
    let row = annotations.get(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)?;
    let parsed = row
        .as_ref()
        .map(|row| (row.revision, SyncPolicy::from_value(&row.value)));
    let policy = match &parsed {
        None => PolicySource::NotSet,
        Some((revision, Err(e))) => PolicySource::Unreadable {
            revision: *revision,
            why: e.to_string(),
        },
        Some((revision, Ok(policy))) => {
            // The plan the staleness line cites, or why there is none: a
            // snapshot or compile failure is reported, never swallowed.
            let refresh = store
                .refresh_snapshot(realm.as_str(), league, annotations)
                .map_err(|e| e.to_string())
                .and_then(|s| {
                    plan_refresh(provider, &s, args.now)
                        .map(|plan| plan.logical_requests)
                        .map_err(|e| e.to_string())
                });
            PolicySource::Set {
                policy,
                revision: *revision,
                refresh,
            }
        }
    };
    render(
        &report,
        &RenderOptions {
            size: args.size,
            template: args.template.unwrap_or(ITEMS_TOKEN),
            now: args.now,
            policy,
            expand: args.expand,
        },
    )
    .map_err(|e| anyhow::anyhow!("{e}"))
}

/// One page of a render, for the clipboard; a page it does not have is
/// refused naming how many it has.
pub fn shop_page(rendered: &ShopRender, n: usize) -> Result<&Page> {
    rendered
        .pages
        .iter()
        .find(|p| p.number == n)
        .ok_or_else(|| {
            let pages = rendered.pages.len();
            anyhow::anyhow!(
                "no page {n}: the render has {pages} {} (`acq shop render` lists them)",
                if pages == 1 { "page" } else { "pages" }
            )
        })
}

/// The currency table this build ships, or why it is not usable.
pub fn currency_table() -> Result<&'static CurrencyTable> {
    table().context("the currency table this build ships is not usable")
}

/// One currency word resolved — a tag, the game's word or a legacy
/// alias, exact and case-sensitive — or the refusal naming the word and
/// the table's version.
pub fn currency_row(word: &str) -> Result<(&'static CurrencyTable, &'static Currency)> {
    let t = currency_table()?;
    let Some(row) = t.resolve(word) else {
        bail!(
            "{word:?} is not a currency word in currency table v{} (words are exact and case-sensitive; `acq reference currency` lists them)",
            t.version()
        );
    };
    Ok((t, row))
}

/// `reference currency [WORD]`'s document: the table whole, evidence
/// included, in its own order — or one word with the row it resolves to
/// and the table's version.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CurrencyDocument {
    Table(&'static CurrencyTable),
    Word(Value),
}

/// `reference currency [WORD]` as JSON.
pub fn currency_document(word: Option<&str>) -> Result<CurrencyDocument> {
    match word {
        None => Ok(CurrencyDocument::Table(currency_table()?)),
        Some(word) => {
            let (t, row) = currency_row(word)?;
            Ok(CurrencyDocument::Word(json!({
                "word": word,
                "version": t.version(),
                "currency": row,
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One account's facts in memory: pc Standard's tab `t1` fetched with
    /// two unpriced items, `i1` and `i2`; and its intent file, empty.
    fn priced() -> (Store, Annotations) {
        use acquisition_store::Endpoint;
        let mut store = Store::open_memory().unwrap();
        let item = |id: &str, x: i64| {
            json!({ "id": id, "name": "", "typeLine": "Iron Ring", "baseType": "Iron Ring",
                    "frameType": 2, "x": x, "y": 0, "w": 1, "h": 1, "inventoryId": "Stash1" })
        };
        let pc = json!({ "realm": "pc", "league": "Standard" });
        store
            .record(
                &Endpoint::Profile,
                &json!({}),
                200,
                &json!({ "uuid": "u-1", "name": "A" }),
                1,
            )
            .unwrap();
        store
            .record(
                &Endpoint::Stashes {
                    realm: "pc".into(),
                    league: "Standard".into(),
                },
                &pc,
                200,
                &json!({ "stashes": [{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }] }),
                10,
            )
            .unwrap();
        store
            .record(
                &Endpoint::Stash {
                    realm: "pc".into(),
                    league: "Standard".into(),
                    id: "t1".into(),
                    sub: None,
                },
                &pc,
                200,
                &json!({ "stash": { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0,
                                    "items": [item("i1", 0), item("i2", 1)] } }),
                20,
            )
            .unwrap();
        (store, Annotations::open_memory_for("u-1").unwrap())
    }

    /// `price list --limit N` holds the first N in listing order and says
    /// how many the selection holds and how many it lists; the containers
    /// are the listed items' alone (the release seat: a league's conflicts
    /// were 49 to 67 KB, cut by a harness after the first item).
    #[test]
    fn a_list_is_bounded_by_its_limit_and_counts_the_selection() {
        let (store, a) = priced();
        let args = |limit| ListArgs {
            effective: Some("none".into()),
            limit,
            ..ListArgs::default()
        };
        let whole = list(&store, &a, None, None, &args(None)).unwrap();
        assert_eq!((whole.total, whole.listed, whole.items.len()), (2, 2, 2));
        let limited = list(&store, &a, None, None, &args(Some(1))).unwrap();
        assert_eq!((limited.total, limited.listed), (2, 1));
        assert_eq!(limited.items, whole.items[..1].to_vec());
        assert_eq!(limited.filter.limit, Some(1));
        let chain = serde_json::to_value(&limited.items[0]).unwrap()["chain"].clone();
        for c in &limited.containers {
            let target = serde_json::to_value(c).unwrap()["subject"]["target"].clone();
            assert!(chain.as_array().unwrap().contains(&target), "{target}");
        }
    }

    /// A write refused for its revision is `conflict` with the row's
    /// revision now, in the verb's words, never the store's: a stale
    /// revision over a live row; a revision named over a cleared one; a
    /// clear of a cleared row. A target the facts do not hold is
    /// `not_found`.
    #[test]
    fn a_write_refused_for_its_revision_is_a_conflict_naming_it_in_the_verbs_words() {
        let (store, mut a) = priced();
        let via = Provenance::via("test");
        let refused = |e: anyhow::Error| {
            Refused::find(&e)
                .cloned()
                .unwrap_or_else(|| panic!("{e:#}"))
        };
        set(
            &store,
            &mut a,
            "item/i1",
            "exact",
            Some("2"),
            Some("divine"),
            Expect::Current,
            &via,
        )
        .unwrap();
        set(
            &store,
            &mut a,
            "item/i1",
            "exact",
            Some("3"),
            Some("divine"),
            Expect::Exactly(1),
            &via,
        )
        .unwrap();
        let r = refused(
            set(
                &store,
                &mut a,
                "item/i1",
                "exact",
                Some("7"),
                Some("divine"),
                Expect::Exactly(1),
                &via,
            )
            .unwrap_err(),
        );
        assert_eq!(r.kind, RefusedKind::Conflict { revision: Some(2) });
        assert_eq!(
            r.message,
            "item/i1 has a price at revision 2 (re-read and retry)"
        );
        let r = refused(clear(&mut a, "item/i1", Expect::Exactly(1), &via).unwrap_err());
        assert_eq!(r.kind, RefusedKind::Conflict { revision: Some(2) });

        clear(&mut a, "item/i1", Expect::Exactly(2), &via).unwrap();
        let r = refused(
            set(
                &store,
                &mut a,
                "item/i1",
                "exact",
                Some("1"),
                Some("divine"),
                Expect::Exactly(99),
                &via,
            )
            .unwrap_err(),
        );
        assert_eq!(r.kind, RefusedKind::Conflict { revision: Some(3) });
        assert_eq!(
            r.message,
            "item/i1's price was cleared (revision 3); a new price names no revision"
        );
        let r = refused(clear(&mut a, "item/i1", Expect::Current, &via).unwrap_err());
        assert_eq!(r.kind, RefusedKind::Conflict { revision: Some(3) });
        assert!(
            r.message
                .starts_with("item/i1's price was cleared (revision 3)"),
            "{}",
            r.message
        );
        for word in ["annotation", "buyout"] {
            assert!(!r.message.contains(word), "{}", r.message);
        }

        let e = show(&store, &a, "item/i-nowhere", None, None).unwrap_err();
        assert_eq!(refused(e).kind, RefusedKind::NotFound);
    }

    /// A row this build cannot read (a newer build's value) is never
    /// replaced or cleared blind: both refuse naming its revision and the
    /// reviewed path, and the row is untouched; a named revision replaces
    /// it deliberately.
    #[test]
    fn a_row_this_build_cannot_read_is_never_written_blind() {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct NewerBuyout(Value);
        impl IntentValue for NewerBuyout {
            const KIND: &'static str = "buyout";
            const VERSION: i64 = 99;
            fn parse(value: &Value) -> Result<Self, String> {
                Ok(NewerBuyout(value.clone()))
            }
        }
        let (store, mut a) = priced();
        let via = Provenance::via("test");
        let newer = json!({ "version": 2, "type": "auction", "reserve": "5 chaos" });
        a.put::<NewerBuyout>("item", "i1", &newer, None, &Provenance::via("future"))
            .unwrap();
        let said = [
            "item/i1 holds a value this build cannot read (revision 1: buyout declares version 2, newer than this build's v1)",
            "`--if-revision 1` replaces it deliberately",
        ];
        let blind = [
            set(
                &store,
                &mut a,
                "item/i1",
                "skip",
                None,
                None,
                Expect::Current,
                &via,
            )
            .map(|_| ())
            .unwrap_err(),
            clear(&mut a, "item/i1", Expect::Current, &via)
                .map(|_| ())
                .unwrap_err(),
        ];
        for e in blind {
            let e = format!("{e:#}");
            for words in said {
                assert!(e.contains(words), "{words}: {e}");
            }
        }
        let row = a.get("item", "i1", BUYOUT_KIND).unwrap().unwrap();
        assert_eq!((row.revision, &row.value), (1, &newer), "untouched");
        let written = set(
            &store,
            &mut a,
            "item/i1",
            "skip",
            None,
            None,
            Expect::Exactly(1),
            &via,
        )
        .unwrap();
        assert_eq!(written.write.prior.map(|r| r.revision), Some(1));
    }

    /// A tab or substash address carries its realm; a realm given beside
    /// it must agree; an item or character takes the given realm, default
    /// pc.
    #[test]
    fn the_realm_comes_from_the_address_and_a_disagreement_refuses() {
        let xbox = PriceTarget::Tab {
            realm: Realm::Xbox,
            id: "t".into(),
        };
        let item = PriceTarget::Item { id: "i".into() };
        assert_eq!(realm_of(&[&xbox], None).unwrap(), Realm::Xbox);
        assert_eq!(realm_of(&[&xbox], Some(Realm::Xbox)).unwrap(), Realm::Xbox);
        assert_eq!(realm_of(&[&item], None).unwrap(), Realm::Pc);
        assert_eq!(realm_of(&[&item], Some(Realm::Sony)).unwrap(), Realm::Sony);
        assert_eq!(realm_of(&[&item, &xbox], None).unwrap(), Realm::Xbox);
        let err = realm_of(&[&xbox], Some(Realm::Pc)).unwrap_err().to_string();
        assert!(
            err.contains("tab/xbox/t is a xbox address; pc was given"),
            "{err}"
        );
        let pc = PriceTarget::Tab {
            realm: Realm::Pc,
            id: "u".into(),
        };
        assert!(realm_of(&[&xbox, &pc], None).is_err());
    }

    /// C35 — `set` reads everything it reports before the write and
    /// nothing after it: a facts read that fails refuses naming the facts
    /// before anything changes, so no write lands whose receipt was lost.
    #[test]
    fn a_set_whose_facts_cannot_be_read_refuses_before_the_write() {
        let dir = std::env::temp_dir().join(format!(
            "acq-pricing-facts-{}-{}",
            std::process::id(),
            acquisition_store::now()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        let path = dir.join("facts.db");
        let store = Store::open(&path).unwrap();
        // The facts made unreadable under the open store: their item table
        // gone, through a second connection.
        rusqlite::Connection::open(&path)
            .unwrap()
            .execute_batch("DROP TABLE items")
            .unwrap();
        let mut a = Annotations::open_memory_for("u-1").unwrap();
        let via = Provenance::via("test");
        let first = set(
            &store,
            &mut a,
            "item/i1",
            "exact",
            Some("30"),
            Some("chaos"),
            Expect::Create,
            &via,
        );
        let err = first.unwrap_err();
        assert!(format!("{err:#}").contains("reading the facts"), "{err:#}");
        assert!(
            a.get("item", "i1", BUYOUT_KIND).unwrap().is_none(),
            "a create landed"
        );
        a.put::<Buyout>(
            "item",
            "i1",
            &read_buyout("exact", Some("30"), Some("chaos"))
                .unwrap()
                .to_value(),
            None,
            &via,
        )
        .unwrap();
        for expect in [Expect::Exactly(1), Expect::Current] {
            let err = set(&store, &mut a, "item/i1", "skip", None, None, expect, &via).unwrap_err();
            assert!(format!("{err:#}").contains("reading the facts"), "{err:#}");
            let row = a.get("item", "i1", BUYOUT_KIND).unwrap().unwrap();
            assert_eq!(row.revision, 1, "{expect:?}: a replace landed");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `set`'s value words are one grammar with the value's own: the
    /// C67 type words and the game's, an amount tolerated as a human
    /// types it and stored canonical, and every refusal in the value's
    /// words — a priced type without its two words, an amount on `skip`,
    /// an alias that is not a tag.
    #[test]
    fn set_reads_the_value_words_through_the_value_s_own_parse() {
        let exact = |amount: &str, currency: &str| {
            Buyout::Exact(crate::price::Price {
                amount: amount.parse().unwrap(),
                currency: currency.into(),
            })
        };
        assert_eq!(
            read_buyout("exact", Some("12.50"), Some("chaos")).unwrap(),
            exact("12.5", "chaos")
        );
        assert_eq!(
            read_buyout("~price", Some("1"), Some("divine")).unwrap(),
            exact("1", "divine")
        );
        assert_eq!(
            read_buyout("b/o", Some("1/5"), Some("divine"))
                .unwrap()
                .to_string(),
            "1/5 divine b/o"
        );
        assert_eq!(read_buyout("skip", None, None).unwrap(), Buyout::Skip);
        assert_eq!(read_buyout("~skip", None, None).unwrap(), Buyout::Skip);
        assert_eq!(
            read_buyout("no-price", None, None).unwrap(),
            Buyout::NoPrice
        );
        for (words, said) in [
            (("bogus", None, None), "is not a price type"),
            (
                ("exact", Some("1"), None),
                "takes an amount and a currency tag",
            ),
            (
                ("negotiable", None, None),
                "takes an amount and a currency tag",
            ),
            (("skip", Some("1"), Some("chaos")), "carries no amount"),
            (
                ("exact", Some("1"), Some("exa")),
                "resolves to tag \"exalted\"",
            ),
            (
                ("exact", Some("1"), Some("c")),
                "is not in currency table v1",
            ),
            (
                ("exact", Some("1.23456"), Some("chaos")),
                "four fractional digits",
            ),
            (("exact", Some("0"), Some("chaos")), "positive"),
        ] {
            let err = read_buyout(words.0, words.1, words.2).unwrap_err();
            assert!(err.to_string().contains(said), "{words:?}: {err}");
        }
    }
}
