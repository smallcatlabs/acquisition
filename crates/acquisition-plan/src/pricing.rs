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
    AnnotationError, AnnotationRow, Annotations, FactAddress, IntentValue, Provenance, Store,
    check_value,
};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::{Value, json};

use crate::currency::{Currency, CurrencyTable, table};
use crate::listing::{
    GameText, ListFilter, ListView, ListingReport, ManualSide, Relation, SetCounts, ShowView,
    resolve,
};
use crate::price::{
    BUYOUT_KIND, BUYOUT_VERSION, Buyout, PriceTarget, PriceWrite, PriceWriteError, clear_buyout,
    set_buyout,
};
use crate::shop::{ITEMS_TOKEN, Page, PolicySource, RenderOptions, ShopRender, render};
use crate::{StoredPolicy, plan_refresh, stored_policy};

/// A refusal an agent branches on, beside its words (C11): what kind it
/// is, and for a write refused for its revision the row's revision now
/// — a live row's, or the one a clear left — or none when no row was
/// ever written, with the live row itself; for a target the facts do not
/// hold, the target and where it was looked for ([`Missing`]). Found in
/// an error's chain by [`Refused::find`]; its [`Refused::fields`] join
/// the failure document on both surfaces, and the verbs' one refusal
/// shape makes its offers from [`Missing`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    pub kind: RefusedKind,
    pub message: String,
}

/// The kinds of [`Refused`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusedKind {
    /// A write refused for its revision; `revision` is the row's now,
    /// and `current` the stored row as it stands — `show`'s `manual`, its
    /// `updated_at` in words at the refusal's time (C163) — where
    /// a live row stands and this build reads its value: none where a
    /// clear left the revision or no row was ever written.
    Conflict {
        revision: Option<i64>,
        current: Option<Box<ManualSide>>,
        /// The write asked, re-aimed at the revision a write over the
        /// target names now: a live row's, which this build reads, or 0,
        /// the create, where no row stands and a price was asked. None
        /// for a clear with nothing to clear and for a row this build
        /// cannot read, which is never offered blind (C151). The one
        /// refusal shape offers it (the league seat, fault 3).
        retry: Option<Box<Retry>>,
    },
    /// A target the facts do not hold.
    NotFound(Box<Missing>),
}

/// A refused write as it would land now: its target, the account it ran
/// as, the price it asked (none: a clear), and the revision to name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Retry {
    pub target: PriceTarget,
    pub account: String,
    pub value: Option<Buyout>,
    pub if_revision: i64,
}

/// What a `not_found` refusal knows of the request it refused (round 4
/// of `roadmap/briefs/2g-build.md`): the target, and the scope it was
/// looked for in as the verb resolved it — the account it ran as, the
/// realm and the league — with where else the facts hold the target and
/// the row of its own it has. The offers that would find it are made
/// from these, carried from where the refusal is made, never by reading
/// the scope again: a target names no league, an item or a character no
/// realm, and a request may have named neither.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Missing {
    pub target: PriceTarget,
    pub account: String,
    pub realm: Realm,
    pub league: String,
    /// The realms and leagues the facts hold the target in, if any, this
    /// scope apart: an item's or a character's address names no realm, so
    /// one held in another realm is found there too.
    pub elsewhere: Vec<(Realm, String)>,
    /// The revision of a row of the target's own, which no listing
    /// carries.
    pub own_row: Option<i64>,
}

impl Refused {
    fn conflict(
        current: Conflicting,
        target: &PriceTarget,
        asked: &Asked,
        message: String,
    ) -> anyhow::Error {
        let retry_at = match (&current, asked.verb) {
            (Conflicting::Live(row), _) if check_value::<Buyout>(&row.value).is_ok() => {
                Some(row.revision)
            }
            (Conflicting::Cleared(_), Verb::Set(_)) => Some(0),
            _ => None,
        };
        let retry = retry_at.map(|if_revision| {
            Box::new(Retry {
                target: target.clone(),
                account: asked.account.to_string(),
                value: match asked.verb {
                    Verb::Set(value) => Some(value.clone()),
                    Verb::Clear => None,
                },
                if_revision,
            })
        });
        let (revision, current) = match current {
            Conflicting::Live(row) => (
                Some(row.revision),
                check_value::<Buyout>(&row.value).ok().map(|value| {
                    Box::new(ManualSide {
                        value,
                        from: target.clone(),
                        inherited: false,
                        revision: row.revision,
                        updated_at: row.updated_at,
                        updated_at_age: Some(acquisition_store::age::ago(
                            acquisition_store::now(),
                            Some(row.updated_at),
                        )),
                        written_via: row.written_via.clone(),
                        actor: row.actor.clone(),
                    })
                }),
            ),
            Conflicting::Cleared(revision) => (revision, None),
            Conflicting::Unreadable(revision) => (Some(revision), None),
        };
        anyhow::Error::new(Refused {
            kind: RefusedKind::Conflict {
                revision,
                current,
                retry,
            },
            message,
        })
    }

    /// The refusal an error carries, if any.
    pub fn find(e: &anyhow::Error) -> Option<&Refused> {
        e.chain().find_map(|c| c.downcast_ref::<Refused>())
    }

    /// The fields a failure document adds beside `error`: `kind`, and
    /// for a conflict `revision` (null when no row was ever written:
    /// C11's field, kept) and `current`, the stored row as it stands,
    /// `show`'s `manual` with its age words (C163) — absent, never null,
    /// where none stands.
    pub fn fields(&self) -> serde_json::Map<String, Value> {
        let mut out = serde_json::Map::new();
        match &self.kind {
            RefusedKind::Conflict {
                revision, current, ..
            } => {
                out.insert("kind".into(), json!("conflict"));
                out.insert("revision".into(), json!(revision));
                if let Some(current) = current {
                    out.insert("current".into(), json!(current));
                }
            }
            RefusedKind::NotFound(_) => {
                out.insert("kind".into(), json!("not_found"));
            }
        }
        out
    }
}

/// The row a write was refused over: the live row the store reported,
/// or the revision a clear left (none when no row was ever written).
enum Conflicting<'a> {
    Live(&'a AnnotationRow),
    Cleared(Option<i64>),
    /// A live row this build cannot read (C151): its revision, and no
    /// `current`, since no value can be shown.
    Unreadable(i64),
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
/// and names the command that displays the row, as typed and as the
/// account the frontend resolved (with a second account known a command
/// naming none is refused as ambiguous: C121, an account that cannot be
/// picked is refused). For a row the facts hold nowhere, an orphan, that
/// is `status` expanded in the target's realm, since the bare verb counts
/// rows and reads pc. For a row the facts hold in another league it is
/// `show` in that league: that league's `status` counts the row under
/// "other league" and never displays it (the h2 fix round's review).
/// Every command it prints carries the scope the verb resolved, the
/// account, the realm and the league, the defaults included (round 2 of
/// `roadmap/briefs/2g-build.md`: a printed read runs as printed with a
/// second account known and a target live in two leagues); and the
/// refusal carries that scope beside the target ([`Missing`]), so the
/// offers made from it are the commands it prints. `quote` makes one
/// shell word of the account, a league and the target; the frontend
/// supplies it (`acquisition_search::answer::shell_quoted`), since this
/// crate does not link the search (C1).
fn not_on_record(
    store: &Store,
    r: &ListingReport,
    target: &PriceTarget,
    own_row: Option<i64>,
    account: &str,
    quote: fn(&str) -> String,
) -> anyhow::Error {
    let here_realm = r.header.realm;
    let elsewhere: Vec<(Realm, String)> = store
        .places_of(&FactAddress::from(target))
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(realm, league)| Some((Realm::parse(&realm)?, league)))
        .filter(|(realm, league)| (*realm, league.as_str()) != (here_realm, &r.header.league))
        .collect();
    let realm = match here_realm.segment() {
        Some(seg) => format!("{seg}/"),
        None => String::new(),
    };
    let scope = |realm: Realm, league: &str| {
        format!(
            "{} {} {}",
            option("--account", account, quote),
            option("--realm", realm.as_str(), quote),
            option("--league", league, quote)
        )
    };
    let here = scope(here_realm, &r.header.league);
    // a place in this realm is its league; in another, realm/league
    let places = elsewhere
        .iter()
        .map(|(realm, league)| match *realm == here_realm {
            true => league.clone(),
            false => format!("{}/{league}", realm.as_str()),
        })
        .collect::<Vec<_>>()
        .join(", ");
    let hint = match elsewhere.as_slice() {
        [] => format!(
            "`acq search {} {}` finds an item's id, `acq tabs {here}` a tab's",
            option("--account", account, quote),
            option("--realm", here_realm.as_str(), quote)
        ),
        held if held.iter().all(|(realm, _)| *realm == here_realm) => {
            format!("the facts hold it in {places}: `--league` reads it there")
        }
        _ => format!("the facts hold it in {places}: `--realm` and `--league` read it there"),
    };
    let row = match (own_row, elsewhere.as_slice()) {
        (None, _) => String::new(),
        (Some(revision), []) => format!(
            "; it has a price row of its own at revision {revision}: \
             `acq price status --expand {here}` lists it"
        ),
        (Some(revision), held) => {
            let shown = held
                .iter()
                .map(|(realm, league)| {
                    format!(
                        "`acq price show {} {}`",
                        quote(&target.to_string()),
                        scope(*realm, league)
                    )
                })
                .collect::<Vec<_>>()
                .join(" or ");
            format!(
                "; it has a price row of its own at revision {revision} in {places}: \
                 {shown} shows it"
            )
        }
    };
    let message = format!(
        "{target} is not in the facts for {realm}{} ({} items, {} tabs and characters on record); {hint}{row}",
        r.header.league, r.counts.items, r.counts.containers
    );
    anyhow::Error::new(Refused {
        kind: RefusedKind::NotFound(Box::new(Missing {
            target: target.clone(),
            account: account.to_string(),
            realm: here_realm,
            league: r.header.league.clone(),
            elsewhere,
            own_row,
        })),
        message,
    })
}

/// How many of a set a view lists (C53, the read economy): `limit` names
/// a number, `expand` the whole; neither, ten.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Bound {
    pub limit: Option<usize>,
    pub expand: bool,
}

/// `price show <target>`: one target's view, in the league the facts hold
/// it in unless one is named, each item set under `bound`, the ages its
/// text says in words at the answer's time ([`ShowView::aged`]). `account` is
/// the resolved account's name and `quote` the frontend's shell quoting,
/// for the command a refusal names (`not_on_record`).
#[allow(clippy::too_many_arguments)]
pub fn show(
    store: &Store,
    annotations: &Annotations,
    target: &str,
    realm: Option<Realm>,
    league: Option<&str>,
    bound: Bound,
    account: &str,
    quote: fn(&str) -> String,
) -> Result<ShowView> {
    let target = read_target(target)?;
    let realm = realm_of(&[&target], realm)?;
    let league = league_of(store, &[&target], league)?;
    let r = report(store, annotations, realm, &league)?;
    match r.show_view(&target) {
        Some(view) => Ok(view
            .bounded(bound.limit, bound.expand)
            .aged(acquisition_store::now())),
        None => {
            let (scope, key) = target.address()?;
            let own = annotations.get(scope, &key, BUYOUT_KIND)?;
            Err(not_on_record(
                store,
                &r,
                &target,
                own.map(|row| row.revision),
                account,
                quote,
            ))
        }
    }
}

/// `list`'s selection as typed: relation and effective-side words,
/// container addresses, how many to list (ten by default, C53) and
/// whether every one (`expand`) — read here into a [`ListFilter`].
#[derive(Debug, Default)]
pub struct ListArgs {
    pub relation: Option<String>,
    pub effective: Option<String>,
    pub location: Option<String>,
    pub covered_by: Option<String>,
    pub game_text: Option<String>,
    pub limit: Option<usize>,
    pub expand: bool,
}

/// `price list`, its ages in words at the answer's time
/// ([`ListView::aged`]). `account` and `quote` as [`show`]'s.
pub fn list(
    store: &Store,
    annotations: &Annotations,
    realm: Option<Realm>,
    league: Option<&str>,
    args: &ListArgs,
    account: &str,
    quote: fn(&str) -> String,
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
    let game_text = match args.game_text.as_deref() {
        None => None,
        Some(word) => Some(GameText::parse(word).ok_or_else(|| {
            anyhow::anyhow!(
                "{word:?} is not a game-side text the list selects (one of {})",
                GameText::ALL.map(GameText::as_str).join(", ")
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
        game_text,
        limit: args.limit,
        expand: args.expand,
    };
    match r.list_view(filter) {
        Some(view) => Ok(view.aged(acquisition_store::now())),
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
                account,
                quote,
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
/// what naming none means — the one rule the frontends differ on. A
/// revision of 0, which no row carries (revisions start at 1), is the
/// create-only form on every surface and both write verbs, so a command
/// one surface prints reads the same on the other (#101).
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
    /// A frontend's revision argument under its default: none is the
    /// default, 0 is [`Expect::Create`], another is exactly that one.
    pub fn or(revision: Option<i64>, default: Expect) -> Expect {
        match revision {
            None => default,
            Some(0) => Expect::Create,
            Some(revision) => Expect::Exactly(revision),
        }
    }
}

/// The blind default's read: the row's current revision, or none. A row
/// this build cannot read (a newer build's value, which the listing state
/// reports as unreadable) is refused here rather than replaced — a blind
/// write over it would destroy intent this build cannot restore, and the
/// receipt's undo would be a value it cannot write. A named revision is
/// the reviewed path past it; `shown` the command that reads the row.
fn current_revision(
    annotations: &Annotations,
    target: &PriceTarget,
    shown: &str,
) -> Result<Option<i64>> {
    let (scope, key) = target.address()?;
    let Some(row) = annotations.get(scope, &key, BUYOUT_KIND)? else {
        return Ok(None);
    };
    if let Err(e) = check_value::<Buyout>(&row.value) {
        bail!(
            "{target} holds a value this build cannot read (revision {}: {e}); \
             not replaced blind — `--if-revision {}` replaces it deliberately, \
             `{shown}` names it",
            row.revision,
            row.revision
        );
    }
    Ok(Some(row.revision))
}

/// The create-only refusal: a row stands and no revision was named.
fn standing(
    target: &PriceTarget,
    row: &AnnotationRow,
    asked: &Asked,
    verb: &str,
    shown: &str,
) -> anyhow::Error {
    let revision = row.revision;
    Refused::conflict(
        Conflicting::Live(row),
        target,
        asked,
        format!(
            "{target} has a price row of its own at revision {revision}: {verb} names the \
             revision it replaces ({revision}) with `--if-revision {revision}`, never a row it \
             has not read — `{shown}` reads it"
        ),
    )
}

/// Which write a refusal speaks for: a set, with the price it asked, or
/// a clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verb<'a> {
    Set(&'a Buyout),
    Clear,
}

/// The write a refusal speaks for, as the account it ran as: what its
/// offer repeats ([`Retry`]).
struct Asked<'a> {
    verb: Verb<'a>,
    account: &'a str,
}

/// A write refused for the row's state, in the verb's words: a live row
/// at another revision; a row a clear left, at the revision its tombstone
/// holds; or no row ever written. `current` is the live row the store
/// reported, if any, carried whole into the refusal; the tombstone is
/// read here. `shown` is the command that reads what covers the target.
fn refused_write(
    annotations: &Annotations,
    target: &PriceTarget,
    current: Option<&AnnotationRow>,
    asked: &Asked,
    shown: &str,
) -> anyhow::Error {
    if let Some(row) = current {
        let revision = row.revision;
        let does = match asked.verb {
            Verb::Set(_) => "writes over it",
            Verb::Clear => "clears it",
        };
        return Refused::conflict(
            Conflicting::Live(row),
            target,
            asked,
            format!(
                "{target} has a price at revision {revision}, not the revision named: \
                 `--if-revision {revision}` {does} as it stands (`{shown}` reads it)"
            ),
        );
    }
    let cleared = target
        .address()
        .ok()
        .and_then(|(scope, key)| annotations.cleared(scope, &key, BUYOUT_KIND).ok())
        .flatten();
    let message = match (cleared, asked.verb) {
        (Some(r), Verb::Set(_)) => {
            format!(
                "{target}'s price was cleared (revision {r}); a new price names revision 0, or none"
            )
        }
        (Some(r), Verb::Clear) => format!(
            "{target}'s price was cleared (revision {r}): nothing to clear \
             (`{shown}` reads what covers it)"
        ),
        (None, Verb::Set(_)) => {
            format!("{target} has no price of its own; a new price names revision 0, or none")
        }
        (None, Verb::Clear) => format!(
            "nothing to clear: {target} has no price row of its own \
             (`{shown}` reads what covers it)"
        ),
    };
    Refused::conflict(Conflicting::Cleared(cleared), target, asked, message)
}

/// The store's compare-and-swap refusal as [`refused_write`]'s; any other
/// error as it was.
fn on_conflict(
    annotations: &Annotations,
    target: &PriceTarget,
    asked: &Asked,
    e: PriceWriteError,
    shown: &str,
) -> anyhow::Error {
    match e {
        PriceWriteError::Store(AnnotationError::Conflict { current }) => {
            refused_write(annotations, target, current.as_deref(), asked, shown)
        }
        // C151: a stale write over a row this build cannot read is the
        // same conflict shape — kind conflict, the row's revision — in the
        // verb's words, with the revision the write must name to go
        // through (the second review of #63, its fourth finding).
        PriceWriteError::Store(AnnotationError::Unreadable(row)) => {
            let does = match asked.verb {
                Verb::Set(_) => "replaces",
                Verb::Clear => "clears",
            };
            Refused::conflict(
                Conflicting::Unreadable(row.revision),
                target,
                asked,
                format!(
                    "{target} holds a value this build cannot read (revision {}: {}); \
                     `--if-revision {}` {does} it deliberately, `{shown}` names it",
                    row.revision, row.fault, row.revision
                ),
            )
        }
        // the price's words, never the store's (C116)
        PriceWriteError::Store(AnnotationError::RevisionExhausted { revision, .. }) => {
            anyhow::anyhow!(
                "{target}'s price is at revision {revision}, the last a row can take: nothing was written"
            )
        }
        other => other.into(),
    }
}

/// `set --json`: the [`PriceWrite`] plus whether the facts hold the
/// target — the text's note, as JSON — and, where the target is or sits
/// in a map or unique substash, `note`, the shop's rule that blocks what
/// it prices ([`substash_note`]; absent otherwise). Absent from `clear`,
/// which only ever removes a row that exists.
#[derive(Debug, Serialize)]
pub struct SetReceipt {
    #[serde(flatten)]
    pub write: PriceWrite,
    /// Whether the address exists in these facts in any league, removed
    /// included — wider than a status's "name nothing in this league's
    /// facts" (C146: two extents, two phrases).
    pub in_facts: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// A container's row: what its items resolve to after the write.
    #[serde(flatten)]
    pub covered: Option<Covered>,
}

/// `clear --json`: the [`PriceWrite`], and for a container's row what its
/// items resolve to after the clear ([`Covered`]).
#[derive(Debug, Serialize)]
pub struct ClearReceipt {
    #[serde(flatten)]
    pub write: PriceWrite,
    #[serde(flatten)]
    pub covered: Option<Covered>,
}

/// What a row on a tab, a folder, a substash or a character covers once
/// the write lands (#171; the answer-facts seat, CLI run 1 call 79
/// against 84: the receipt said the tab's price and only `price show`
/// how many items take it): the two sets `price show` counts, by the
/// function it counts them with ([`ListingReport::show_view`]), so the
/// words are its own. A set's `by_relation` says which of its items take
/// a price by hand alone, which the game decides and which agree; none
/// is listed here (`returned` 0, `left_out` every one), `price show`
/// lists them. Absent for an item's row and for a target the facts do
/// not hold.
#[derive(Debug, Clone, Serialize)]
pub struct Covered {
    pub items_here_counts: SetCounts,
    pub items_covered_below_counts: SetCounts,
}

/// [`Covered`] for `target` as the store and intent now stand: its realm
/// and league read as `show` reads them.
fn covered(
    store: &Store,
    annotations: &Annotations,
    target: &PriceTarget,
) -> Result<Option<Covered>> {
    if matches!(target, PriceTarget::Item { .. }) || !store.holds(&FactAddress::from(target))? {
        return Ok(None);
    }
    let realm = realm_of(&[target], None)?;
    let league = league_of(store, &[target], None)?;
    let view = report(store, annotations, realm, &league)?.show_view(target);
    Ok(view.map(|view| {
        let view = view.bounded(Some(0), false);
        Covered {
            items_here_counts: view.items_here_counts,
            items_covered_below_counts: view.items_covered_below_counts,
        }
    }))
}

/// [`covered`] after a write that landed: a read that fails then says the
/// write stands, so no one retries what is done.
fn covered_after(
    store: &Store,
    annotations: &Annotations,
    write: &PriceWrite,
) -> Result<Option<Covered>> {
    covered(store, annotations, &write.target).with_context(|| {
        format!(
            "{} was written (revision {}); reading the items it covers failed",
            write.target, write.revision
        )
    })
}

/// The note a set's receipt carries where the shop cannot post what the
/// row prices (the league seat, fault 4): the row stands (C69), and the
/// shop's own rule says why, in its words (`Cell::Substash`).
pub fn substash_note() -> String {
    format!(
        "the shop blocks what this row prices (substash): {}",
        crate::shop::Cell::Substash.why()
    )
}

/// Whether `target` is a substash, or an item the facts hold in one, as
/// the listing reads an item's place (its `location`, the shop's reading
/// too): in any realm and league the facts hold it in.
fn in_substash(store: &Store, annotations: &Annotations, target: &PriceTarget) -> Result<bool> {
    match target {
        PriceTarget::Substash { .. } => Ok(true),
        PriceTarget::Item { .. } => {
            for (realm, league) in store.places_of(&FactAddress::from(target))? {
                let Some(realm) = Realm::parse(&realm) else {
                    continue;
                };
                let report = report(store, annotations, realm, &league)?;
                if report.find(target).is_some_and(|l| {
                    matches!(l.subject.location, Some(PriceTarget::Substash { .. }))
                }) {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        PriceTarget::Tab { .. } | PriceTarget::Character { .. } => Ok(false),
    }
}

/// The read-back a write's refusal prints: `price show` of the target as
/// the account the frontend resolved, and nothing else, since a write
/// resolves no realm or league (round 5 of the 2g build: a printed
/// command carries the scope its verb resolved, here the account alone).
/// `quote` is the frontend's shell quoting, as for [`show`].
fn read_back(target: &PriceTarget, account: &str, quote: fn(&str) -> String) -> String {
    format!(
        "acq price show {} {}",
        option("--account", account, quote),
        quote(&target.to_string())
    )
}

/// A scope option and its value as a printed command spells them, quoted
/// by the frontend's `quote`: attached, `--league=-L`, where the value
/// starts with `-`, which the parser would otherwise read as an option
/// (the review of the league seat's fix round, finding 3; the CLI's
/// `option_words` is the same rule for the lines it spells itself).
fn option(name: &str, value: &str, quote: fn(&str) -> String) -> String {
    match value.starts_with('-') {
        true => quote(&format!("{name}={value}")),
        false => format!("{name} {}", quote(value)),
    }
}

/// `price set <target> <type> [<amount> <currency>]`: one row through
/// the shared write ([`set_buyout`]) under compare-and-swap. `account`
/// and `quote` are as for [`show`]: the command a refusal prints.
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
    account: &str,
    quote: fn(&str) -> String,
) -> Result<SetReceipt> {
    let target = read_target(target)?;
    let shown = read_back(&target, account, quote);
    let value = read_buyout(kind, amount, currency)?;
    // Everything the receipt reports is read before the write and nothing
    // after it, so a read that fails refuses before anything changes: the
    // note, whether the facts hold the target at all, in any league,
    // removed included.
    let in_facts = store
        .holds(&FactAddress::from(&target))
        .context("reading the facts for the target")?;
    let note = in_substash(store, annotations, &target)
        .context("reading the facts for the target's place")?
        .then(substash_note);
    let expected = match expect {
        Expect::Exactly(revision) => Some(revision),
        Expect::Current => current_revision(annotations, &target, &shown)?,
        Expect::Create => None,
    };
    let asked = Asked {
        verb: Verb::Set(&value),
        account,
    };
    let write = match set_buyout(annotations, &target, &value, expected, provenance) {
        Err(PriceWriteError::Store(AnnotationError::Conflict { current: Some(row) }))
            if expect == Expect::Create =>
        {
            return Err(standing(&target, &row, &asked, "a write over it", &shown));
        }
        Err(e) => return Err(on_conflict(annotations, &target, &asked, e, &shown)),
        Ok(write) => write.aged(acquisition_store::now()),
    };
    let covered = covered_after(store, annotations, &write)?;
    Ok(SetReceipt {
        write,
        in_facts,
        note,
        covered,
    })
}

/// `price clear <target>`: remove the row, under the same compare-and-
/// swap. A target with no row is a refusal that says so — every nothing
/// says which nothing. `account` and `quote` are as for [`set`]. `store`
/// is the facts where a frontend could open them without making them:
/// what a container's row covered is counted from them after the clear,
/// and a clear needs no facts (C46).
pub fn clear(
    store: Option<&Store>,
    annotations: &mut Annotations,
    target: &str,
    expect: Expect,
    provenance: &Provenance,
    account: &str,
    quote: fn(&str) -> String,
) -> Result<ClearReceipt> {
    let target = read_target(target)?;
    let shown = read_back(&target, account, quote);
    let asked = Asked {
        verb: Verb::Clear,
        account,
    };
    let expected = match expect {
        Expect::Exactly(revision) => revision,
        Expect::Current => match current_revision(annotations, &target, &shown)? {
            Some(revision) => revision,
            None => {
                return Err(refused_write(annotations, &target, None, &asked, &shown));
            }
        },
        Expect::Create => {
            let (scope, key) = target.address()?;
            return Err(match annotations.get(scope, &key, BUYOUT_KIND)? {
                Some(row) => standing(&target, &row, &asked, "a clear", &shown),
                None => refused_write(annotations, &target, None, &asked, &shown),
            });
        }
    };
    let write = clear_buyout(annotations, &target, expected, provenance)
        .map(|write| write.aged(acquisition_store::now()))
        .map_err(|e| on_conflict(annotations, &target, &asked, e, &shown))?;
    let covered = match store {
        Some(store) => covered_after(store, annotations, &write)?,
        None => None,
    };
    Ok(ClearReceipt { write, covered })
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
    let stored = stored_policy(annotations)?;
    let policy = match &stored {
        StoredPolicy::NotSet => PolicySource::NotSet,
        StoredPolicy::Unreadable { revision, why } => PolicySource::Unreadable {
            revision: *revision,
            why: why.clone(),
        },
        StoredPolicy::Set { revision, policy } => {
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

/// One page of a render as an answer (#169; the answer-facts seat, MCP
/// run 1 call 50 against 51: a page alone dropped the facts' age, what
/// was left out and the policy): the page's own fields, as a page alone
/// answered them, and beside them the render's document with `pages`
/// narrowed to that page, so `counts`, `left_out`, `policy` and
/// `freshness` stand as for the whole render.
#[derive(Debug, Clone, Serialize)]
pub struct ShopPage {
    #[serde(flatten)]
    pub page: Page,
    #[serde(flatten)]
    pub render: ShopRender,
}

/// [`shop_page`] as an answer: the render, its `pages` the one asked.
pub fn shop_page_answer(mut rendered: ShopRender, n: usize) -> Result<ShopPage> {
    let page = shop_page(&rendered, n)?.clone();
    rendered.pages = vec![page.clone()];
    Ok(ShopPage {
        page,
        render: rendered,
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

/// One entry of the pricing model as a frontend prints it on demand: a
/// name and what it says (the MCP's `describe` with names
/// `["pricing"]`, #83). The words are the ones the MCP's tool texts carried,
/// cut, beside the functions that make what they describe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Described {
    pub name: &'static str,
    pub what: String,
}

/// The listing model, read once rather than in every tool's text: the
/// hand row and what inherits it, the game's reading, the relation and
/// the effective side, the revision rule and the undo, and the counts a
/// listing state, a list and a shop render carry (C146: on the MCP a
/// count's gloss stands here, named by the tool's description).
pub fn described() -> Vec<Described> {
    let entry = |name, what: &str| Described {
        name,
        what: what.to_string(),
    };
    vec![
        entry(
            "hand row",
            "a price set by hand is one row on an item, tab, substash or character, which what it covers inherits (a tab's items, a folder's tabs, a parent tab's substashes, a character's items) unless a row nearer the item says otherwise; a listing's `manual` is the row that applies, its own target in `from`, `inherited` when that is a container's, and its `revision`. A row is local: writing it sends nothing and changes nothing in game",
        ),
        entry(
            "game",
            "the game's reading: an item's note, then its tab's name, read only where the tab is public; a listing's `game` holds the note and the tab name verbatim beside what each reads as, the tab and whether it is public",
        ),
        entry(
            "relation",
            "how the two sides stand: none, manual_only, game_only, agree or conflict, so the items priced by hand are manual_only, agree and conflict together",
        ),
        entry(
            "effective",
            "who decides an item's price: game, manual, none, or unresolved — a row that cannot be read could decide. The most specific statement decides, the game's on a tie: an item's note before a hand row on its tab, a hand row on the item before its tab's name",
        ),
        entry(
            "revision",
            "a write over a row the target has of its own names that row's revision — a listing's `manual.revision` where `manual.from` is the target itself, a write's receipt's `revision`, or the `revision` a `conflict` refusal names, beside `current`, the stored row as it stands in `manual`'s shape, its age words with it, absent over a cleared row — and with no row names none, or 0, to create one (0, which no row carries, creates only where no row stands, on every surface); a write naming a revision the target's own row does not stand at is refused as a `conflict` naming the target's own revision, null where it has none — a container's (`manual.revision` where `manual.inherited`) among them, unless the target's own row stands at that same number, which a revision cannot tell apart: revisions count per target. A clear takes a revision too, so the next write's revision continues from it: a receipt's `revision` is the one now current, the written row's, or after a clear its tombstone's — the generation the row sequence has reached, which a later conflict names — and the next write after a clear is a create, naming none",
        ),
        Described {
            name: "undo",
            what: format!(
                "a write's receipt, a set's or a clear's, is one shape, the sync policy's write's too: `target`; `written`, the row now standing, null after a clear; `prior`, the row before, null on a create; `revision`, the one the next write names; and `undo`, the request that puts `prior` back, sent back as printed (`request` and `says`, `tool` where it is another tool's, and beside a command line `argv`, its words unquoted for a program to run with no shell): after a create, a clear naming the written revision; after a replace, a set with the prior's value naming the written revision; after a clear, a set with the prior's value naming no revision, a create. Beside a replace's undo stands `clear`, the clear of the row at the written revision, in the same shape (\"clears it: no price by hand\"): a replace's undo puts the prior back, and a create's clear goes stale once a replace moves the row; a create and a clear carry none. Where nothing puts it back `undo` is absent and `no_undo` says why, in words: a prior this build cannot read (a newer build's, replaced only by naming its revision), one naming a retired currency, which a new price cannot name, or a policy's first write, since no verb clears a policy. `in_facts` is whether the address exists in these facts in any league, removed included (false: not fetched yet, or a typo; the row stands); on a set or a clear of a tab's, folder's, substash's or character's row, `items_here_counts` and `items_covered_below_counts` count what its items resolve to after the write, as a show counts its sets, listing none; `note`, on a set whose target is a map or unique tab's substash or an item the facts hold in one, where the row stands all the same, says: {}",
                substash_note()
            ),
        },
        entry(
            "status counts",
            "a league's listing state, counted: `counts.by_effective` says who decides each item's price (game, manual, none, unresolved — a row that cannot be read could decide), `counts.by_relation` how the two sides stand (none, manual_only, game_only, agree, conflict), so the items priced by hand are its manual_only, agree and conflict together; `counts.game_priced` the items the game prices; `counts.by_game_statement` the items by the game side's statement in a public tab (exact, negotiable, skip, none; a hand row nearer the item still applies over a skip); `counts.residue` the items carrying price text the trade site cannot see — a note on a character's item, which decides no price until the item is moved into a public stash tab, or a note or tab name in a tab that is not public, which decides none until the tab is made public in the game; `counts.invalid_notes` the notes that start like a price and do not read as one (no effect: a hand row or its tab's name applies, where either does); `counts.invalid_tab_names` the tabs whose name starts like a price and does not read as one, and `counts.invalid_tab_name_items` the items in them (no effect: a note or a hand row applies, where either does); price_list with `game_text` `residue`, `invalid_note`, `invalid_tab_name` or `skip` lists each count's items; `rows` accounts for every hand-written row: how many apply here, which name nothing in this league's facts (`unmatched`, each row whole — `value`, `from`, `revision`, `updated_at` with its words, `written_via` — since no listing carries it), which cannot be read (`unreadable`), which belong to other leagues (`other_league`) or realms (`other_realm`, a count); each of the three lists holds ten unless `expand`, its whole count beside it (`unmatched_total`, `unreadable_total`, `other_league_total`); `facts`, how old the facts are, is over the report's non-folder containers, the locations the facts were read from: `places` (`tabs`, `characters`), `fetched`, the oldest and newest last fetch each with its location, and `list_seen` the oldest of the lists naming them, null where none",
        ),
        entry(
            "show counts",
            "a container's show, counted: `items_here` (physically in it) and `items_covered_below` (covered through its children) each list ten by default, as many as `limit` names, every one under `expand`; beside each, `items_here_counts` and `items_covered_below_counts` count the whole set: `total`, every item it holds; `returned`, how many it lists; `left_out`, how many it does not; `by_relation`, the whole set by relation word, an item whose price is unresolved never counted as none; and `unresolved`, the set's items a row that cannot be read could decide",
        ),
        entry(
            "list counts",
            "a list of the priced items, counted: `items_on_record`, how many items the league holds, so an empty answer says which nothing; `total`, how many the selection holds, and `listed`, how many `items` lists — the first ten in listing order, as many as `limit` names, every one under `expand`, `containers` being theirs alone; `counts` counts the whole selection as a show counts a set: `total`, `returned`, `left_out`, `by_relation` and `unresolved`",
        ),
        entry(
            "shop counts",
            "a shop render, counted: each page's `tabs` name each `Stash<n>` its links write — the tab's rank among the tabs the website lists — by the tab's own `name`, with how many `items` link to it; `posted` lists the first ten items on the pages and `posted_total` counts them all; `left_out` lists the first ten items left off for a reason, the blocked first, each with its `cell`, and `left_out_total` counts them all; `freshness`'s `uncovered`, `stale` and `position_before_listing` list ten each beside `uncovered_total`, `stale_total` and `position_before_listing_total`, `position_before_listing` the posted items fetched before the stash listing whose tab ranks their links use, so a tab reindexed between would move a link, which a refresh re-reads; `freshness.facts`, whatever the policy and absent when nothing is posted, is a status's `facts` over the posted items' locations, each once (`oldest_stale_seconds` is another set's: the oldest covered stale item's age, now minus its sighting, `oldest_stale_seconds_age` that age's words); `expand` lists every one; `counts` counts every item by its cell, the items nothing applies to in `off_page`; `policy` gives each cell's rule and count",
        ),
    ]
}

#[cfg(test)]
#[path = "../unit/tests/pricing.rs"]
mod tests;
