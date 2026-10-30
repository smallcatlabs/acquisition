//! The typed price: what a `buyout` row says ([`Buyout`], C67) and what
//! it is about ([`PriceTarget`]). Built by the pricing slice's plan step 2
//! (`PRICING-SLICE.md` at `b6197a82`, 2026-09-05). Nothing here reads facts or a
//! daemon; this module is the value's shape, its parse, its address, and
//! — since plan step 5 (2026-09-06) — the one write path every frontend's
//! `set` and `clear` use ([`set_buyout`], [`clear_buyout`]), through the
//! intent file's own API and nothing else.
//!
//! # Rulings held here
//!
//! - **C67** (the `buyout` and its target): a league-less target means a
//!   merge keeps the row; `amount`'s four fractional digits are the game's
//!   (T10); canonical text, structural equality; more digits are refused.
//!
//! # As built
//!
//! **The target** is the row's `(scope, key)`, rendered and parsed by one
//! type so the realm-bearing keys were defined before the first
//! tab-scoped row landed (2b constraint (6)): `item/<id>`,
//! `character/<id>`, `tab/<realm>/<id>`, `substash/<realm>/<parent>/<id>`.
//! Ids are GGG's, opaque; a component is never empty and never holds a
//! `/`; the realm is [`Realm`]'s own word (`pc`, `xbox`, `sony`, `poe2`),
//! spelled out even for pc — the wire's pc-by-omission (C58) is a URL
//! rule, not a key rule. No league: a tab id is stable across a league
//! merge, and the row should be too.
//!
//! **The value** is stamped `version: 1` and is one of four shapes. In
//! JSON: `{"version":1,"type":"exact","amount":"12.5","currency":"chaos"}`
//! (or `negotiable`), `{"version":1,"type":"no_price"}`,
//! `{"version":1,"type":"skip"}`. `amount` is text, never a JSON
//! number, so `2.50` and `2.5` cannot silently become one float: as text
//! the canonical spelling is the shortest — no leading zeros, no trailing
//! fraction zeros, no trailing dot — and a ratio is `wanted/lot` of two
//! positive integers kept exactly as written (`22/10` is not `11/5`, and
//! `3/1` is not `3`: the lot is a lot). [`Amount`] compares structurally
//! on those parts, so `"12.50"` typed by a human ([`Amount::from_str`])
//! equals `"12.5"`, while the JSON door holds a v1 value to its canonical
//! text through the store's exact round-trip (C66) — a non-canonical
//! spelling in a value is refused there naming the path, not rewritten.
//! A decimal has at most **four** fractional digits — what the game
//! itself writes (T10: `999.1234` in the fixture) — held as
//! ten-thousandths, so the same type carries an observed note's amount
//! at plan step 3 and a manual price, and the listing state (step 4,
//! `listing.rs`) compares them structurally; a fifth digit is refused. `0`, `0.00` and `0/5` are
//! refused (a price is positive); `10/0` is refused. (v1 was ruled at
//! two places on 2026-09-04 and widened to four on 2026-09-06 after an
//! outside review; widening is compatible, every stored two-place value
//! still parses.)
//!
//! `currency` is a **tag** of the shipped currency table
//! ([`crate::currency`]) — the immutable identity intent cites (C68),
//! never an alias or a display name. A word that resolves to a row by
//! alias or emit but is not that row's tag is refused naming the tag, so
//! the frontend can offer it; a word the table does not know is refused
//! naming the table version. A **retired** tag parses: a stored row may
//! cite it forever, and whether a *new* price may name one is the
//! writer's rule ([`Buyout::check_write`], run at the store's door on
//! every write and never on a read), not the parse's. `no_price`
//! and `skip` carry no amount and no currency; a value that supplies
//! either is refused (the 0.18 userstore's `[ignore]` rows carried a
//! non-semantic 4321 `blessed`, which is exactly the shape this refuses).
//!
//! [`Buyout`] is an [`IntentValue`], so the store's write door runs the
//! version gate, this parse and the exact round-trip before the
//! compare-and-swap; nothing lands that this module would not read back.
//!
//! **The write** ([`set_buyout`], [`clear_buyout`]) is a single-row
//! compare-and-swap through that door, built once for every frontend the
//! way `put_sync_policy` is: the caller passes the revision it reviewed
//! (or `None` to create), the frontend decides whether "whatever is
//! stored" is an acceptable default, and what comes back is a
//! [`PriceWrite`] — the row written and the row it replaced, raw with
//! provenance — which is C78's clause until receipts exist: a hand can
//! undo what the command printed. The one writer's rule C67 leaves
//! open, that a *new* price never names a retired tag, is the value's
//! [`Buyout::check_write`], which the store's door runs on every write
//! and never on a read (2026-09-07, after an outside review found the
//! rule bypassable through `Annotations::put`): a stored row citing one
//! still parses, and no frontend can write past it.

use std::fmt;
use std::str::FromStr;

use acquisition_protocol::realm::Realm;
use acquisition_store::{
    AnnotationError, AnnotationRow, Annotations, FactAddress, IntentValue, Provenance, check_value,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::currency;

/// The row kind a price is stored under.
pub const BUYOUT_KIND: &str = "buyout";

/// The `buyout` value schema this build writes ([`Buyout`]).
pub const BUYOUT_VERSION: i64 = 1;

/// The scopes a price may sit on, as stored; the key grammar is
/// [`PriceTarget::address`].
pub const ITEM_SCOPE: &str = "item";
pub const CHARACTER_SCOPE: &str = "character";
pub const TAB_SCOPE: &str = "tab";
pub const SUBSTASH_SCOPE: &str = "substash";

/// What a price is about: the row's address, typed. Identity mirrors the
/// store's (C54, C55, C58): an item or character by its GGG id, a tab by
/// realm and id, a substash by realm, parent tab and its own id. No
/// league.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case", deny_unknown_fields)]
pub enum PriceTarget {
    Item {
        id: String,
    },
    Character {
        id: String,
    },
    Tab {
        realm: Realm,
        id: String,
    },
    Substash {
        realm: Realm,
        parent: String,
        id: String,
    },
}

/// The store's address for the same identity (C67 mirrors C54 and C58),
/// so a frontend can ask the facts whether they hold the target.
impl<'a> From<&'a PriceTarget> for FactAddress<'a> {
    fn from(target: &'a PriceTarget) -> Self {
        match target {
            PriceTarget::Item { id } => FactAddress::Item { id },
            PriceTarget::Character { id } => FactAddress::Character { id },
            PriceTarget::Tab { realm, id } => FactAddress::Tab {
                realm: realm.as_str(),
                id,
            },
            PriceTarget::Substash { realm, parent, id } => FactAddress::Substash {
                realm: realm.as_str(),
                parent,
                id,
            },
        }
    }
}

/// Why a `(scope, key)` is not a [`PriceTarget`], or a component cannot
/// be one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetError {
    /// Not one of the four price scopes.
    UnknownScope { scope: String },
    /// A key with the wrong number of `/`-separated parts for its scope.
    MalformedKey { scope: &'static str, key: String },
    /// A realm word [`Realm`] does not know.
    UnknownRealm { realm: String },
    /// An id, parent or realm component that is empty or holds a `/`.
    MalformedComponent { component: String },
}

impl fmt::Display for TargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetError::UnknownScope { scope } => write!(
                f,
                "{scope:?} is not a price scope (item, character, tab, substash)"
            ),
            TargetError::MalformedKey { scope, key } => {
                write!(f, "{key:?} is not a {scope} key")
            }
            TargetError::UnknownRealm { realm } => write!(f, "{realm:?} is not a realm"),
            TargetError::MalformedComponent { component } => write!(
                f,
                "{component:?}: a target component is a non-empty id without `/`"
            ),
        }
    }
}

impl std::error::Error for TargetError {}

fn component(s: &str) -> Result<(), TargetError> {
    if s.is_empty() || s.contains('/') {
        return Err(TargetError::MalformedComponent {
            component: s.into(),
        });
    }
    Ok(())
}

fn realm(word: &str) -> Result<Realm, TargetError> {
    Realm::parse(word).ok_or_else(|| TargetError::UnknownRealm { realm: word.into() })
}

impl PriceTarget {
    /// The stored address: the scope and the key under it. Refuses a
    /// target whose components could not be read back (an empty id, a
    /// `/` inside one), so every address this renders parses again.
    pub fn address(&self) -> Result<(&'static str, String), TargetError> {
        Ok(match self {
            PriceTarget::Item { id } => {
                component(id)?;
                (ITEM_SCOPE, id.clone())
            }
            PriceTarget::Character { id } => {
                component(id)?;
                (CHARACTER_SCOPE, id.clone())
            }
            PriceTarget::Tab { realm, id } => {
                component(id)?;
                (TAB_SCOPE, format!("{}/{id}", realm.as_str()))
            }
            PriceTarget::Substash { realm, parent, id } => {
                component(parent)?;
                component(id)?;
                (SUBSTASH_SCOPE, format!("{}/{parent}/{id}", realm.as_str()))
            }
        })
    }

    /// The target a stored row's `(scope, key)` names — the inverse of
    /// [`PriceTarget::address`].
    pub fn from_address(scope: &str, key: &str) -> Result<PriceTarget, TargetError> {
        let parts: Vec<&str> = key.split('/').collect();
        let malformed = |scope: &'static str| TargetError::MalformedKey {
            scope,
            key: key.into(),
        };
        let target = match (scope, parts.as_slice()) {
            (ITEM_SCOPE, [id]) => PriceTarget::Item { id: (*id).into() },
            (ITEM_SCOPE, _) => return Err(malformed(ITEM_SCOPE)),
            (CHARACTER_SCOPE, [id]) => PriceTarget::Character { id: (*id).into() },
            (CHARACTER_SCOPE, _) => return Err(malformed(CHARACTER_SCOPE)),
            (TAB_SCOPE, [r, id]) => PriceTarget::Tab {
                realm: realm(r)?,
                id: (*id).into(),
            },
            (TAB_SCOPE, _) => return Err(malformed(TAB_SCOPE)),
            (SUBSTASH_SCOPE, [r, parent, id]) => PriceTarget::Substash {
                realm: realm(r)?,
                parent: (*parent).into(),
                id: (*id).into(),
            },
            (SUBSTASH_SCOPE, _) => return Err(malformed(SUBSTASH_SCOPE)),
            (other, _) => {
                return Err(TargetError::UnknownScope {
                    scope: other.into(),
                });
            }
        };
        // A split never yields a `/` inside a part; emptiness it can.
        target.address()?;
        Ok(target)
    }
}

impl FromStr for PriceTarget {
    type Err = TargetError;

    /// The address as one word, `scope/key` — what [`PriceTarget`]'s
    /// `Display` writes: `item/<id>`, `character/<id>`, `tab/<realm>/<id>`,
    /// `substash/<realm>/<parent>/<id>`.
    fn from_str(word: &str) -> Result<PriceTarget, TargetError> {
        match word.split_once('/') {
            Some((scope, key)) => PriceTarget::from_address(scope, key),
            None => Err(TargetError::UnknownScope { scope: word.into() }),
        }
    }
}

impl fmt::Display for PriceTarget {
    /// `scope/key`, the address as one word.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.address() {
            Ok((scope, key)) => write!(f, "{scope}/{key}"),
            Err(e) => write!(f, "<invalid target: {e}>"),
        }
    }
}

/// How much: a decimal of at most four fractional digits, held as
/// ten-thousandths, or a bulk ratio `wanted/lot` (T2) kept unreduced.
/// Equality is structural on those parts — `2.50` and `2.5` are one
/// amount, `22/10` and `11/5` are two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Amount {
    /// A positive decimal, in ten-thousandths: `12.5` is `125000`.
    Decimal { ten_thousandths: u64 },
    /// `wanted` of the currency for a `lot` of the item; both positive.
    Ratio { wanted: u64, lot: u64 },
}

/// Why a text is not an [`Amount`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmountError {
    pub text: String,
    pub why: &'static str,
}

impl fmt::Display for AmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "amount {:?}: {}", self.text, self.why)
    }
}

impl std::error::Error for AmountError {}

/// A positive integer with no leading zero, within `u64`.
fn positive_integer(s: &str) -> Result<u64, &'static str> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) || s.starts_with('0') {
        return Err("a positive integer has digits only and no leading zero");
    }
    s.parse().map_err(|_| "too large")
}

impl FromStr for Amount {
    type Err = AmountError;

    /// The human spelling: `12`, `12.5`, `12.50` (a trailing zero is
    /// tolerated and dropped), `999.1234`, `22/10`. Refused: a fifth
    /// fractional digit (never rounded), a leading zero, a sign,
    /// whitespace, zero, a zero lot, and anything that is not digits, one
    /// `.` or one `/`.
    fn from_str(text: &str) -> Result<Amount, AmountError> {
        let err = |why: &'static str| AmountError {
            text: text.into(),
            why,
        };
        if let Some((wanted, lot)) = text.split_once('/') {
            let part = |s: &str| match positive_integer(s) {
                Ok(n) => Ok(n),
                Err("too large") => Err(err("too large")),
                Err(_) => Err(err("`wanted/lot` needs two positive integers")),
            };
            return Ok(Amount::Ratio {
                wanted: part(wanted)?,
                lot: part(lot)?,
            });
        }
        let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
        if text.contains('.') && fraction.is_empty() {
            return Err(err("a decimal does not end in `.`"));
        }
        if fraction.len() > 4 {
            return Err(err("at most four fractional digits (refused, not rounded)"));
        }
        if !fraction.bytes().all(|b| b.is_ascii_digit()) {
            return Err(err(
                "a decimal is digits, optionally `.` and one to four digits",
            ));
        }
        let units: u64 = if whole == "0" {
            0
        } else {
            match positive_integer(whole) {
                Ok(n) => n,
                Err("too large") => return Err(err("too large")),
                Err(_) => {
                    return Err(err(
                        "a decimal is digits, optionally `.` and one to four digits",
                    ));
                }
            }
        };
        // The fraction, right-padded to four places: `5` → 5000, `1234` → 1234.
        let fraction_part: u64 = if fraction.is_empty() {
            0
        } else {
            let digits = fraction.parse::<u64>().map_err(|_| err("not a decimal"))?;
            digits * 10u64.pow(4 - fraction.len() as u32)
        };
        let ten_thousandths = units
            .checked_mul(10_000)
            .and_then(|t| t.checked_add(fraction_part))
            .ok_or_else(|| err("too large"))?;
        if ten_thousandths == 0 {
            return Err(err("a price is positive"));
        }
        Ok(Amount::Decimal { ten_thousandths })
    }
}

impl fmt::Display for Amount {
    /// The canonical text: the shortest decimal, or `wanted/lot` verbatim.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Amount::Decimal { ten_thousandths } => {
                let (units, mut frac) = (ten_thousandths / 10_000, ten_thousandths % 10_000);
                if frac == 0 {
                    return write!(f, "{units}");
                }
                let mut places = 4;
                while frac % 10 == 0 {
                    frac /= 10;
                    places -= 1;
                }
                write!(f, "{units}.{frac:0places$}")
            }
            Amount::Ratio { wanted, lot } => write!(f, "{wanted}/{lot}"),
        }
    }
}

impl Serialize for Amount {
    /// The canonical text — never a JSON number (the module doc's reason).
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Amount, D::Error> {
        let text = String::deserialize(d)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

/// An amount in a currency: the tag intent cites (C68).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Price {
    pub amount: Amount,
    /// A tag of the shipped currency table; retired tags included.
    pub currency: String,
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

/// The `buyout` value, v1 (C67): four shapes, two with a price.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Buyout {
    /// `~price`: this amount.
    Exact(Price),
    /// `~b/o`: this amount, open to offers.
    Negotiable(Price),
    /// Listed with no price.
    NoPrice,
    /// `~skip`'s word: leave this out of the shop — the forum analogue of
    /// the game's "Do not index". Renamed from `ignore` on 2026-09-06
    /// (C67), before any row existed.
    Skip,
}

impl Buyout {
    /// The price, for the two shapes that carry one.
    pub fn price(&self) -> Option<&Price> {
        match self {
            Buyout::Exact(p) | Buyout::Negotiable(p) => Some(p),
            Buyout::NoPrice | Buyout::Skip => None,
        }
    }

    /// The `type` word.
    pub fn kind(&self) -> &'static str {
        match self {
            Buyout::Exact(_) => "exact",
            Buyout::Negotiable(_) => "negotiable",
            Buyout::NoPrice => "no_price",
            Buyout::Skip => "skip",
        }
    }

    /// The value as it is written: the v1 wire shape, canonical text.
    pub fn to_value(&self) -> Value {
        // A wire struct of strings and options serializes without failing.
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}

impl fmt::Display for Buyout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Buyout::Exact(p) => write!(f, "{p}"),
            Buyout::Negotiable(p) => write!(f, "{p} b/o"),
            Buyout::NoPrice => write!(f, "no price"),
            Buyout::Skip => write!(f, "skip"),
        }
    }
}

/// The v1 wire shape: every field explicit, unknown fields refused. The
/// combination rules (a price iff the type carries one) are checked in
/// [`Buyout::parse`], where the message can name what is wrong.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuyoutWireV1 {
    version: i64,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
}

impl Serialize for Buyout {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let price = self.price();
        BuyoutWireV1 {
            version: BUYOUT_VERSION,
            kind: self.kind().into(),
            amount: price.map(|p| p.amount.to_string()),
            currency: price.map(|p| p.currency.clone()),
        }
        .serialize(s)
    }
}

impl<'de> Deserialize<'de> for Buyout {
    /// The same strict parse as the write door, so a report or a plan
    /// that carries a value re-reads it exactly.
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Buyout, D::Error> {
        let value = Value::deserialize(d)?;
        Buyout::parse(&value).map_err(serde::de::Error::custom)
    }
}

impl IntentValue for Buyout {
    const KIND: &'static str = BUYOUT_KIND;
    const VERSION: i64 = BUYOUT_VERSION;

    /// The strict parse: the v1 shape, the type word, the amount's
    /// grammar, and a currency that is a tag of the shipped table.
    fn parse(value: &Value) -> Result<Buyout, String> {
        let wire: BuyoutWireV1 =
            serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        if wire.version != BUYOUT_VERSION {
            return Err(format!(
                "declares version {}, not this build's v{BUYOUT_VERSION}",
                wire.version
            ));
        }
        let carries_price = match wire.kind.as_str() {
            "exact" | "negotiable" => true,
            "no_price" | "skip" => false,
            other => {
                return Err(format!(
                    "type {other:?} is not one of exact, negotiable, no_price, skip"
                ));
            }
        };
        if !carries_price {
            if wire.amount.is_some() || wire.currency.is_some() {
                return Err(format!(
                    "type {:?} carries no amount and no currency",
                    wire.kind
                ));
            }
            return Ok(match wire.kind.as_str() {
                "no_price" => Buyout::NoPrice,
                _ => Buyout::Skip,
            });
        }
        let amount = wire
            .amount
            .as_deref()
            .ok_or_else(|| format!("type {:?} needs an amount", wire.kind))?
            .parse::<Amount>()
            .map_err(|e| e.to_string())?;
        let word = wire
            .currency
            .as_deref()
            .ok_or_else(|| format!("type {:?} needs a currency", wire.kind))?;
        let table = currency::table().map_err(|e| e.to_string())?;
        let currency = match table.by_tag(word) {
            Some(row) => row.tag.clone(),
            None => match table.resolve(word) {
                Some(row) => {
                    return Err(format!(
                        "currency {word:?} is not a tag; it resolves to tag {:?} (currency table v{}) — a price row names the tag",
                        row.tag,
                        table.version()
                    ));
                }
                None => {
                    return Err(format!(
                        "currency {word:?} is not in currency table v{}",
                        table.version()
                    ));
                }
            },
        };
        let price = Price { amount, currency };
        Ok(match wire.kind.as_str() {
            "exact" => Buyout::Exact(price),
            _ => Buyout::Negotiable(price),
        })
    }

    /// The writer's rule (C67): a new price never names a retired tag,
    /// though a stored row may cite one forever. Run by the store's door
    /// on every write, through every frontend, and never on a read.
    fn check_write(&self) -> Result<(), String> {
        let Some(price) = self.price() else {
            return Ok(());
        };
        let table = currency::table().map_err(|e| e.to_string())?;
        match table
            .by_tag(&price.currency)
            .and_then(|row| row.retired.as_deref())
        {
            Some(retired) => Err(format!(
                "currency {:?} is retired ({retired}): a new price cannot name it, though a stored row may",
                price.currency
            )),
            None => Ok(()),
        }
    }
}

/// The outcome of one write, until receipts exist (C78): the row as
/// written — `None` after a clear — and the row it replaced — `None` on a
/// create — so what a command printed is enough to undo it by hand. Both
/// are the store's rows, raw, with their provenance; on success `prior`
/// is exactly the row the write replaced — the row at the revision the
/// write named, a live row that moved between the read and the write
/// conflicting (C35) — and a create, naming none, replaces nothing: its
/// `prior` is `None` even when a row stood until a racing clear.
/// `prior.value` may be one this build cannot read (a newer build's,
/// written past by an explicit revision, or a value not in its canonical
/// text): a frontend reads it through the store's door check,
/// `check_value::<Buyout>`, shows the JSON when that fails, and takes
/// what puts it back from [`PriceWrite::undo`], never by parsing the
/// prior itself.
///
/// `revision` is the revision now current on the target, the one the next
/// write over it names: the written row's, or after a clear its
/// tombstone's, as the delete's own transaction returned it — the
/// generation the row sequence has reached, which a later conflict names;
/// the next write after a clear is a create and names none (round 5 of
/// the 2g build).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PriceWrite {
    pub target: PriceTarget,
    pub written: Option<AnnotationRow>,
    pub prior: Option<AnnotationRow>,
    /// The row this write replaced or cleared when this build could not
    /// read it (C151): named by its revision and fault, since `prior`
    /// cannot carry a value nothing can parse. `None` beside a `prior`
    /// or when nothing stood.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_unreadable: Option<acquisition_store::UnreadableRow>,
    pub revision: i64,
}

/// The write that puts a [`PriceWrite`]'s prior back (C78), before any
/// surface spells it: after a create, the clear naming the revision
/// written; after a replace, the prior's value naming the revision
/// written, so it undoes this write and not a later one; after a clear, a
/// create of the prior's value, which names no revision (the store lands
/// it over the tombstone, [`Annotations::put`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Undo {
    Set {
        value: Buyout,
        if_revision: Option<i64>,
    },
    Clear {
        if_revision: i64,
    },
}

impl PriceWrite {
    /// What undoes this write, or why nothing does, in words: a prior
    /// this build cannot read (a newer build's, written past by naming
    /// its revision, or a value not in its canonical text), or one a new
    /// price cannot name (a retired currency, the writer's rule, C67) — a
    /// receipt never promises a write the door refuses, nor a value other
    /// than the one stored. The prior is read through the store's own
    /// door check ([`check_value`]: the version, the strict parse, the
    /// exact round-trip), the read every reader of a stored row takes.
    pub fn undo(&self) -> Result<Undo, String> {
        let put_back = |prior: &AnnotationRow| -> Result<Buyout, String> {
            let value = check_value::<Buyout>(&prior.value)
                .map_err(|_| "this build cannot read it".to_string())?;
            value.check_write()?;
            Ok(value)
        };
        match (&self.written, &self.prior) {
            (Some(row), None) => Ok(Undo::Clear {
                if_revision: row.revision,
            }),
            (Some(row), Some(prior)) => Ok(Undo::Set {
                value: put_back(prior)?,
                if_revision: Some(row.revision),
            }),
            (None, Some(prior)) => Ok(Undo::Set {
                value: put_back(prior)?,
                if_revision: None,
            }),
            (None, None) => Err(match &self.prior_unreadable {
                Some(row) => format!(
                    "the cleared row (revision {}) could not be read, so nothing puts it back",
                    row.revision
                ),
                None => "nothing was cleared".into(),
            }),
        }
    }
}

/// Why a price write did not land. The value's own refusals — the strict
/// parse and the writer's rule ([`Buyout::check_write`], C67) — arrive
/// as [`AnnotationError::Invalid`] under `Store`.
#[derive(Debug)]
pub enum PriceWriteError {
    /// The target's components cannot be an address.
    Target(TargetError),
    /// The compare-and-swap, the busy timeout, the file: the store's own.
    Store(AnnotationError),
}

impl fmt::Display for PriceWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceWriteError::Target(e) => write!(f, "{e}"),
            PriceWriteError::Store(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for PriceWriteError {}

impl From<TargetError> for PriceWriteError {
    fn from(e: TargetError) -> PriceWriteError {
        PriceWriteError::Target(e)
    }
}

impl From<AnnotationError> for PriceWriteError {
    fn from(e: AnnotationError) -> PriceWriteError {
        PriceWriteError::Store(e)
    }
}

/// The row a write naming `expected` replaces: none for a create, which
/// names no revision and replaces nothing — a row read before its put
/// may be one a racing clear removes before the put lands; for a named
/// revision, the row read now, which must stand at exactly that revision.
/// A row at any other is refused here as the store's compare-and-swap
/// would refuse it, so a write that lands replaced the very row returned:
/// revisions only grow, and the swap holds the row at it until the put.
/// The store's one checked read, [`Annotations::at_revision`].
fn replaced(
    annotations: &Annotations,
    scope: &str,
    key: &str,
    expected: Option<i64>,
) -> Result<
    (
        Option<AnnotationRow>,
        Option<acquisition_store::UnreadableRow>,
    ),
    PriceWriteError,
> {
    let Some(expected) = expected else {
        return Ok((None, None));
    };
    match annotations.at_revision(scope, key, BUYOUT_KIND, expected) {
        Ok(row) => Ok((Some(row), None)),
        // C151: the row at exactly that revision cannot be read; the write
        // naming it replaces it anyway, and the receipt names what stood.
        Err(AnnotationError::Unreadable(row)) if row.revision == expected => Ok((None, Some(row))),
        Err(e) => Err(e.into()),
    }
}

/// Write one price under compare-and-swap (C35) — the one write path
/// every frontend's `set` uses: the value goes through the store's typed
/// door as its canonical text (C66), where the writer's rule runs too
/// ([`Buyout::check_write`]: a new price never names a retired tag, C67
/// — the door's, so `Annotations::put` cannot be used to write past it),
/// and the row it replaced comes back (C78). No daemon, no quote, no job
/// (C64).
///
/// `expected_revision` is the store's compare-and-swap, verbatim:
/// `Some(r)` replaces exactly the revision the caller reviewed, and a row
/// that moved conflicts; the receipt's prior is the row at `r`, read and
/// checked before the put. `None` creates, its prior none, refused if a
/// live row exists — a
/// tombstone of any generation does not count, so a `clear` then `set`
/// works without the caller knowing it is there, and so a create cannot
/// tell which clear it follows. A frontend that wants "replace whatever
/// is stored" reads the current row itself and passes its revision here
/// — the blind form is a frontend policy, not this function's, the same
/// rule as [`crate::put_sync_policy`]; what the frontend owes that read
/// is to refuse a row it cannot parse (a newer build's), since a blind
/// replacement of one destroys intent this build cannot restore.
pub fn set_buyout(
    annotations: &mut Annotations,
    target: &PriceTarget,
    value: &Buyout,
    expected_revision: Option<i64>,
    provenance: &Provenance,
) -> Result<PriceWrite, PriceWriteError> {
    let (scope, key) = target.address()?;
    let (prior, prior_unreadable) = replaced(annotations, scope, &key, expected_revision)?;
    let written = annotations.put::<Buyout>(
        scope,
        &key,
        &value.to_value(),
        expected_revision,
        provenance,
    )?;
    Ok(PriceWrite {
        target: target.clone(),
        revision: written.revision,
        written: Some(written),
        prior,
        prior_unreadable,
    })
}

/// Remove one price row under the same compare-and-swap, returning the
/// row it removed (C78), the row at `expected_revision`, read and checked
/// before the delete, and the tombstone's revision the delete returned. `expected_revision` is exact: what the caller
/// reviewed; the store tombstones the row, so the revision sequence
/// carries on and a stale writer still conflicts. Whether a target with
/// no row is a refusal is the frontend's call — the store reports it as a
/// conflict carrying nothing.
pub fn clear_buyout(
    annotations: &mut Annotations,
    target: &PriceTarget,
    expected_revision: i64,
    provenance: &Provenance,
) -> Result<PriceWrite, PriceWriteError> {
    let (scope, key) = target.address()?;
    let (prior, prior_unreadable) = replaced(annotations, scope, &key, Some(expected_revision))?;
    let revision = annotations.delete(scope, &key, BUYOUT_KIND, expected_revision, provenance)?;
    Ok(PriceWrite {
        target: target.clone(),
        written: None,
        prior,
        prior_unreadable,
        revision,
    })
}

#[cfg(test)]
#[path = "../unit/tests/price.rs"]
mod tests;
