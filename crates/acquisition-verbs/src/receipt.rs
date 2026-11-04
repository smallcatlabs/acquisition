//! One receipt shape for every write (round 5 of `roadmap/briefs/2g-build.md`,
//! the shape its design's Q6 delegated; C78, a single write returns the
//! prior so a hand can undo it): `price_set`'s, `price_clear`'s and the
//! sync policy's write's answer is one document on every surface, made
//! here so no frontend shapes a receipt alone:
//!
//! - `target`: what was written — a price's target; a policy's account
//!   and the realms the write replaced;
//! - `written`, the row now standing, null after a clear; `prior`, the
//!   row before, null on a create;
//! - `revision`, the revision now current, which the next write over the
//!   target names: the written row's, or after a clear its tombstone's,
//!   returned by the delete itself;
//! - `undo`, the request that puts `prior` back, an offer in the refusal's
//!   shape (`{request, says}`, `tool` where it is another tool's; each
//!   surface spells it, [`Surface::request`]); or, where nothing puts it
//!   back, `no_undo` beside its absence, the reason in words — never both,
//!   never neither;
//! - `clear`, beside a replace's `undo` or `no_undo`: the clear of the
//!   row at the revision it wrote, an offer in the same shape, so a
//!   reader who wants no price by hand has the request that stays
//!   current (#167: after a set then a replace, the create's clear named
//!   the revision the replace moved past, and the replace's undo puts the
//!   prior back). Absent where the undo is that clear (a create) and
//!   where no row stands (a clear), and on a policy's write, which no
//!   verb clears;
//! - the write's own facts: `in_facts` on a price set, `league_notes` on
//!   a policy write (C126), and on a price set or clear over a container's
//!   row `items_here_counts` and `items_covered_below_counts`, what its
//!   items resolve to after it, as `price show` counts them (#171), or
//!   `counts_unavailable` where they cannot be counted: `why`, and
//!   `reads`, the bare `price show` that counts them where the facts
//!   hold the target, the write standing whole all the same (the reviews
//!   of round B and of its fix round).

use acquisition_plan::SyncPolicy;
use acquisition_plan::price::{PriceWrite, Undo};
use acquisition_plan::pricing::{ClearReceipt, SetReceipt, Uncounted};
use acquisition_store::{IntentValue as _, check_value};
use serde::Serialize;
use serde_json::Value;

use crate::frontend::PolicyWritten;
use crate::refusal::{Surface, Way, offer};

/// `price set`'s receipt, as the account it ran as: the planner's
/// document, then its `undo` or `no_undo`.
pub fn price_set<'a>(
    receipt: &'a SetReceipt,
    account: &str,
    surface: &dyn Surface,
) -> Receipt<'a, SetReceipt> {
    let mut shaped = shaped(receipt, price_undo(&receipt.write, account), surface);
    shaped.clear = price_clear_offer(&receipt.write, account)
        .and_then(|(way, says)| offer(&way, &says, surface));
    shaped.counts_unavailable =
        unavailable(receipt.uncounted.as_ref(), &receipt.write, account, surface);
    shaped
}

/// Where a container's counts are read when a receipt could not count
/// them: the bare `price show` of the target, which reads where the
/// facts hold it when it runs, and names each place where they hold it
/// in several; the receipt never guesses a scope (the review of round
/// B's fix round). One read: `reads` keeps its list, now always of one.
pub fn uncounted_reads(_: &Uncounted, write: &PriceWrite, account: &str) -> Vec<(Way, String)> {
    vec![(
        Way::PriceShow {
            target: write.target.to_string(),
            account: account.to_string(),
            realm: None,
            league: None,
        },
        "counts its items where the facts hold it".to_string(),
    )]
}

/// `counts_unavailable`: why, and [`uncounted_reads`] spelled by the
/// surface.
fn unavailable(
    u: Option<&Uncounted>,
    write: &PriceWrite,
    account: &str,
    surface: &dyn Surface,
) -> Option<Value> {
    let u = u?;
    let reads: Vec<Value> = uncounted_reads(u, write, account)
        .iter()
        .filter_map(|(way, says)| offer(way, says, surface))
        .collect();
    Some(serde_json::json!({ "why": u.why, "reads": reads }))
}

/// The clear of the row a write left standing, at the revision it wrote,
/// where its undo is not that clear (#167): after a replace, whatever its
/// undo; none after a create, whose undo it is, or after a clear.
pub fn price_clear_offer(write: &PriceWrite, account: &str) -> Option<(Way, String)> {
    let row = write.written.as_ref()?;
    if matches!(write.undo(), Ok(Undo::Clear { .. })) {
        return None;
    }
    Some((
        Way::PriceClear {
            target: write.target.to_string(),
            account: account.to_string(),
            if_revision: row.revision,
        },
        "clears it: no price by hand".to_string(),
    ))
}

/// `price clear`'s receipt, as [`price_set`]'s.
pub fn price_clear<'a>(
    receipt: &'a ClearReceipt,
    account: &str,
    surface: &dyn Surface,
) -> Receipt<'a, ClearReceipt> {
    let mut shaped = shaped(receipt, price_undo(&receipt.write, account), surface);
    shaped.counts_unavailable =
        unavailable(receipt.uncounted.as_ref(), &receipt.write, account, surface);
    shaped
}

/// What puts a price write's prior back, as the account it ran as, with
/// what the offer says; or why nothing does, in words. The planner's rule
/// ([`PriceWrite::undo`]), the same line on every surface: the CLI's text
/// receipt prints it too.
pub fn price_undo(write: &PriceWrite, account: &str) -> Result<(Way, String), String> {
    let target = write.target.to_string();
    let undo = write
        .undo()
        .map_err(|why| match (&write.written, &write.prior) {
            (Some(_), _) => format!("the prior cannot be put back: {why}"),
            (None, Some(_)) => format!("the cleared value cannot be put back: {why}"),
            (None, None) => why,
        })?;
    Ok(match undo {
        Undo::Clear { if_revision } => (
            Way::PriceClear {
                target,
                account: account.to_string(),
                if_revision,
            },
            "undoes it".to_string(),
        ),
        Undo::Set { value, if_revision } => {
            let way = price_set_way(target, account, &value, if_revision);
            let says = match if_revision {
                Some(_) => "puts the prior back".to_string(),
                None => "puts it back (a create, which lands only where no price stands; the row's count continues past the clear)".to_string(),
            };
            (way, says)
        }
    })
}

/// `value` written on `target` as `account`, over `if_revision` (none: a
/// create), as `price set` takes it: the `type` word, and its amount and
/// currency where it has a price. A receipt's undo, and a conflict's
/// offer ([`crate::refusal`]).
pub fn price_set_way(
    target: String,
    account: &str,
    value: &acquisition_plan::price::Buyout,
    if_revision: Option<i64>,
) -> Way {
    let price = value.price();
    Way::PriceSet {
        target,
        account: account.to_string(),
        kind: value.kind().to_string(),
        amount: price.map(|p| p.amount.to_string()),
        currency: price.map(|p| p.currency.clone()),
        if_revision,
    }
}

/// The sync policy's write's receipt: [`PolicyWritten`], then its `undo`
/// or `no_undo`.
pub fn policy<'a>(written: &'a PolicyWritten, surface: &dyn Surface) -> Receipt<'a, PolicyWritten> {
    shaped(written, policy_undo(written), surface)
}

/// What puts a policy write's prior back: the prior's value written whole
/// (`replace`), naming the revision this write left, so it undoes this
/// write and not a later one; or why nothing does. A first write has no
/// prior to put back, and no verb clears a policy (round 5 of the 2g
/// build names the missing undo rather than adding one); a prior this
/// build cannot read is no value its door would write.
pub fn policy_undo(written: &PolicyWritten) -> Result<(Way, String), String> {
    let Some(prior) = &written.prior else {
        return Err(
            "a first policy has nothing before it to put back, and no verb clears a policy"
                .to_string(),
        );
    };
    check_value::<SyncPolicy>(&prior.value)
        .map_err(|_| ())
        .and_then(|policy| policy.check_write().map_err(|_| ()))
        .map_err(|()| {
            "the prior policy cannot be put back: this build cannot read it".to_string()
        })?;
    Ok((
        Way::PolicySet {
            account: written.target.account.clone(),
            value: prior.value.clone(),
            if_revision: written.row.revision,
        },
        "puts the prior policy back, whole".to_string(),
    ))
}

/// A write's document with its `undo`, spelled by a surface, or its
/// `no_undo` beside that absence: serialized as one object, the
/// document's keys in the order it declares them and the undo's after
/// them, so the receipt reads as the document did (C115: an answer leads
/// with what bears on the question).
#[derive(Serialize)]
pub struct Receipt<'a, T: Serialize> {
    #[serde(flatten)]
    pub document: &'a T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_undo: Option<String>,
    /// A price write's [`price_clear_offer`], spelled as `undo` is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<Value>,
    /// A container's row whose counts could not be read: `why`, and
    /// `reads`, each `price show` that counts them, spelled as `undo` is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_unavailable: Option<Value>,
}

/// `document` with `undo` spelled by `surface`, or `no_undo` beside its
/// absence: one of the two, never both, never neither.
fn shaped<'a, T: Serialize>(
    document: &'a T,
    undo: Result<(Way, String), String>,
    surface: &dyn Surface,
) -> Receipt<'a, T> {
    let undo = undo.and_then(|(way, says)| {
        offer(&way, &says, surface).ok_or_else(|| "this surface sends no request back".to_string())
    });
    let (undo, no_undo) = match undo {
        Ok(offer) => (Some(offer), None),
        Err(why) => (None, Some(why)),
    };
    Receipt {
        document,
        undo,
        no_undo,
        clear: None,
        counts_unavailable: None,
    }
}
