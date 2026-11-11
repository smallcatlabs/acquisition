//! Every tool's arguments as the server reads them: one struct per tool,
//! whose doc comments are the schema `tools/list` shows, read through
//! [`Args`] so an argument a tool does not take is refused.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use acquisition_protocol::realm::Realm;
use acquisition_verbs::refusal::Arguments;
use anyhow::Result;
use rmcp::schemars;
use serde::Deserialize;
use serde_json::Value;

use crate::told::{Refused, refusal, refusing};

/// A tool's arguments, read here and not by the SDK, whose refusal of
/// arguments it cannot read is a line of text and no document: what
/// the tool does not take is refused as any refusal is, and a key it
/// does not know is never read past. The schema is `T`'s.
pub(crate) struct Args<T>(serde_json::Map<String, Value>, std::marker::PhantomData<T>);

impl<'de, T> Deserialize<'de> for Args<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(deserializer).map(|map| Args(map, std::marker::PhantomData))
    }
}

impl<T: schemars::JsonSchema> schemars::JsonSchema for Args<T> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        T::schema_name()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        T::json_schema(generator)
    }
}

impl<T: serde::de::DeserializeOwned> Args<T> {
    /// The arguments, or [`Arguments`], refused as `arguments`.
    pub(crate) fn parse(self) -> Result<T> {
        serde_json::from_value(Value::Object(self.0))
            .map_err(|e| Arguments(format!("these are not arguments the tool takes: {e}")).into())
    }

    /// The arguments, or their refusal document.
    pub(crate) fn read(self) -> std::result::Result<T, Refused> {
        self.parse()
            .map_err(|e| Box::new(refusing(refusal(&e, "account"))))
    }
}

/// A `realm` tool parameter: pc when omitted (as on the wire), else one
/// of the documented realms in the layer's one sentence; anything else is
/// refused as an argument.
pub(crate) fn realm_param(realm: Option<&str>) -> Result<Realm> {
    match realm {
        None => Ok(Realm::DEFAULT),
        Some(s) => acquisition_verbs::realm(s).map_err(|why| Arguments(why).into()),
    }
}

/// Every tool's `account`, one sentence (#83: undescribed on seven).
/// True of every tool that carries it (4-full part 1, step 2):
/// `saved_search_set` reads it only for a create naming no realm, and in
/// a world with no account known saves that create over every realm, so
/// the refusals are said of a name read and of a tool that needs one,
/// within the bound every parameter's text is held to.
pub(crate) const ACCOUNT: &str = "A username, with or without its #discriminator, or a uuid; needed only when several are known. A name read that no account known holds is unknown_account (none known: not_logged_in if needed). A job tool also needs the account logged in now, else not_logged_in; a store read needs only its store.";

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct TabsParams {
    /// League name; defaults to "Standard".
    pub(crate) league: Option<String>,
    /// List this many tabs in place of ten: the most items first where
    /// some are left out, else in the listing's order.
    pub(crate) limit: Option<usize>,
    /// List every tab of the league, or of the selection.
    pub(crate) expand: Option<bool>,
    /// Only tabs of this type, as GGG names it (UniqueStash, MapStash,
    /// PremiumStash …; `by_type` lists them).
    pub(crate) r#type: Option<String>,
    /// Only tabs never fetched.
    pub(crate) never_fetched: Option<bool>,
    /// Realm: pc (default), xbox, or sony.
    pub(crate) realm: Option<String>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct CharactersParams {
    /// Restrict to one league; omitted lists every league.
    pub(crate) league: Option<String>,
    /// Restrict to one realm (pc, xbox, sony, poe2); omitted lists every realm.
    pub(crate) realm: Option<String>,
    /// List this many characters in place of ten, in the store's order.
    pub(crate) limit: Option<usize>,
    /// List every character of the selection.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

/// The arguments of `search`: the request, read by the search crate's
/// own reader and by nothing here (H1), but `saved`, which this adapter
/// takes out and resolves before the reader, setting the saved search's
/// text and scope into the arguments as sent
/// ([`AcqMcp::saved_into`](crate::server::AcqMcp::saved_into)).
/// The schema says what a request holds; the reader refuses what is
/// none.
#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct RequestParams(pub(crate) serde_json::Map<String, Value>);

impl schemars::JsonSchema for RequestParams {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "SearchRequest".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "properties": {
                "saved": {
                    "type": "string",
                    "description": "A saved search's id or name (saved_searches lists them), run whole: its query and its scope's realm and membership. Beside it only `scope.account`, `view` and `context`."
                },
                "scope": {
                    "type": "object",
                    "description": "What is searched: {\"account\": needed only where several could be meant, \"realm\": pc, xbox, sony, poe2 or all, needed only where the store holds several, \"membership\": \"live\" (the default), \"all\", the removed items too, each marked with when it left, or \"removed\"}."
                },
                "query": {
                    "type": "object",
                    "description": "{\"text\": a query in the language `describe` prints} or {\"tree\": the tree an answer returned}, never both; omitted, every item in scope. Or {\"trade\": a trade site search's URL, id or query JSON}, translated with no network; `describe` with names [\"trade\"] says what it becomes and reports."
                },
                "view": {
                    "type": "object",
                    "description": "{\"rows\": {\"limit\": N, \"sort\": a value, \"desc\": true}}, the default (ten rows); {\"counts\": {\"keys\": [a key, …], \"sum\": a value}}; {\"cross\": {\"keys\": [two keys]}}; or {\"explain\": {\"path\": a node's path}}; \"routes\": true beside any. `describe` with names [\"answer\"] says what each gives."
                },
                "context": {
                    "type": "string",
                    "description": "\"matches\" (the default), or \"corpus\": the view over every item in scope, the query set aside."
                },
                "basis": {
                    "type": "object",
                    "description": "Carried by a continuation, as an answer's `next` holds it; never written by hand."
                },
                "request": {
                    "type": "object",
                    "description": "In place of the fields above: a request as an answer holds one — a count's, `view.rows.next`, an offer — sent whole, as printed."
                },
                "count": {
                    "type": "integer",
                    "description": "Beside `request`, the count it was printed with: the answer then says under `before` where the count has moved."
                },
                "counted_at": {
                    "type": "object",
                    "description": "Beside `request` and `count`, the `basis` of the answer the count came from: the answer then says under `before` wherever the basis has moved."
                }
            }
        })
    }
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct DescribeParams {
    #[schemars(description = describe_names_doc())]
    pub(crate) names: Option<Vec<String>>,
}

/// `describe`'s `names`: what each form of the list answers. The
/// sections are named by the index, whose keys they are
/// (`describe::SECTIONS`, which the search crate pins against the
/// answer), so the doc names a few and the index every one.
pub(crate) fn describe_names_doc() -> String {
    "Omitted: the index, an entry a line. A name answers whole: a section (fields, computed, \
answer, trade, pricing), an entry (league, total_res), a category's id; all, everything. \
\"~resist\": the language's entries whose name or definition matches, whole if ten or fewer, \
else their lines."
        .to_string()
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ShowParams {
    /// An item's id, whole, as an answer printed it.
    pub(crate) id: String,
    /// Also return the body as the store holds it, verbatim.
    pub(crate) body: Option<bool>,
    /// Why the item does or does not match a query: `{"text": …}`,
    /// `{"tree": …}` or `{"trade": …}`, as a request's `query`.
    pub(crate) against: Option<Value>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct StoreParams {
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct RefusedParams {
    /// One refused body in full, by its row `id` from the list.
    pub(crate) id: Option<i64>,
    /// With no `id`, the most rows listed, the newest first (default ten).
    pub(crate) limit: Option<usize>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct PolicyShowParams {
    /// List every league on disk in `leagues_held`, not ten a realm.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct LeaguesParams {
    /// Must be true: this tool reads the league list last fetched, with no
    /// daemon. A `leagues` job (submit_job) fetches it anew.
    pub(crate) stored: bool,
    /// List every league of the list, not ten.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct EventsParams {
    /// How far back to look (default 24 hours).
    pub(crate) hours: Option<f64>,
    /// The most events read, the newest first (default ten).
    pub(crate) limit: Option<usize>,
    /// The events counted by location in place of listed: ten locations.
    pub(crate) summary: Option<bool>,
    /// With `summary`, every location.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

/// `submit_job`'s kind: the protocol's list, the words the daemon's
/// refusal of an unknown kind prints (C127), and what `apply` takes.
pub(crate) fn submit_kind_doc() -> String {
    use acquisition_protocol::job::{MOCK_ONLY_KINDS, client_kinds};
    format!(
        "Job kind, one a client may submit: {}. {} run against the mock daemon alone and are \
refused in real mode. apply_plan sends a reviewed plan, with the checks an apply here skips.",
        client_kinds(false),
        MOCK_ONLY_KINDS.join(", ")
    )
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SubmitParams {
    #[schemars(description = submit_kind_doc())]
    pub(crate) kind: String,
    /// Kind-specific params, e.g. {"league": "Standard", "id": "..."} for
    /// stash. Passed verbatim; visible to every connected client.
    pub(crate) params: Option<Value>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
    /// Higher runs sooner (default 0).
    pub(crate) priority: Option<u8>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct AccountsParams {
    /// List every account, not ten.
    pub(crate) expand: Option<bool>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ListJobsParams {
    /// List every job, open and finished, not ten of each.
    pub(crate) expand: Option<bool>,
    /// `open`, `finished` or `all` (both, the default).
    pub(crate) state: Option<String>,
}

/// The arguments of a tool that takes none: any key is refused.
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct NoParams {}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct JobParams {
    /// The job id, as returned by submit_job.
    pub(crate) id: u64,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SetPolicyParams {
    /// The policy, e.g.
    /// {"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}:
    /// per league `tabs` and/or `characters`, "all" or an id list; a realm
    /// named with no leagues is removed. Validated strictly, landing whole
    /// or not at all.
    pub(crate) value: Value,
    /// The revision this write replaces, from sync_policy: required when a
    /// policy exists, `replace` or not; omitted, or 0, only to create the
    /// first.
    pub(crate) if_revision: Option<i64>,
    /// true writes the whole policy: every realm `value` does not name is
    /// removed. Default false: the realms `value` names, the rest kept.
    pub(crate) replace: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct PlanParams {
    /// League name; defaults to "Standard".
    pub(crate) league: Option<String>,
    /// Realm: pc (default), xbox, sony, or poe2 (a poe2 plan can carry
    /// character work only).
    pub(crate) realm: Option<String>,
    /// An explicit selection, compiled in place of the stored policy:
    /// {"tabs": "all" or [tab ids], "characters": [character ids],
    /// "deep": true to fetch a named map or unique tab's pages on record
    /// with it}, any of them left out; ids as `tabs` and `characters` list
    /// them.
    #[schemars(with = "Option<Value>")]
    pub(crate) selection: Option<acquisition_plan::PlanSelection>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ApplyParams {
    /// The plan envelope exactly as refresh_plan returned it (the `plan`
    /// field). Re-validated here; a tampered or hand-built envelope will
    /// not parse.
    pub(crate) plan: Value,
    /// Refuse at admission — before any child job exists — if the plan
    /// authorizes more logical requests than this.
    pub(crate) max_requests: Option<u64>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct PriceStatusParams {
    /// The league; Standard when omitted.
    pub(crate) league: Option<String>,
    /// pc (the default), xbox, sony or poe2.
    pub(crate) realm: Option<String>,
    /// List every row of `rows.unmatched`, `rows.unreadable` and
    /// `rows.other_league`, past the first ten.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct PriceShowParams {
    /// One address: `item/<id>`, `character/<id>`, `tab/<realm>/<id>` or
    /// `substash/<realm>/<parent>/<id>`. An answer holds a target as an
    /// object, `{scope, realm, parent, id}`: the address is its parts in
    /// that order joined by `/`, the absent ones left out.
    pub(crate) target: String,
    /// The league to read the target in. With neither `league` nor
    /// `realm`, the realm and league the facts hold the target in, so an
    /// address an answer printed reads back as printed; held in several,
    /// it is refused with one request per place.
    pub(crate) league: Option<String>,
    /// The realm of an item or character target; a tab or substash
    /// address carries its own, and a realm given beside it must agree.
    /// Omitted, as `league` says (pc where the facts hold it nowhere).
    pub(crate) realm: Option<String>,
    /// List this many of each item set in place of ten, the first in
    /// listing order.
    pub(crate) limit: Option<usize>,
    /// List every item of both sets.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct PriceListParams {
    /// The league; omitted, the league of the container `in` or
    /// `covered_by` names, else Standard.
    pub(crate) league: Option<String>,
    /// pc (the default), xbox, sony or poe2; a tab or substash address
    /// given to `in` or `covered_by` carries its own.
    pub(crate) realm: Option<String>,
    /// Only items in this relation: none, manual_only, game_only, agree or
    /// conflict.
    pub(crate) relation: Option<String>,
    /// Only items whose price this side decides: game, manual, none or
    /// unresolved.
    pub(crate) effective: Option<String>,
    /// Only items physically in this container: a tab, substash or
    /// character address.
    #[serde(rename = "in")]
    pub(crate) location: Option<String>,
    /// Only items a row on this target would cover: a folder's tabs'
    /// items, a map or unique tab's substashes' items too.
    pub(crate) covered_by: Option<String>,
    /// Only the members of a count of price_status: residue, price text
    /// the trade site cannot see; invalid_note and invalid_tab_name, a
    /// note or an item's tab name that starts like a price and does not
    /// read as one; skip, a ~skip statement in a public tab.
    pub(crate) game_text: Option<String>,
    /// List this many items in place of ten, the first in listing order;
    /// `total` says how many the selection holds.
    pub(crate) limit: Option<usize>,
    /// List every item of the selection.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ShopRenderParams {
    /// The league; Standard when omitted.
    pub(crate) league: Option<String>,
    /// pc (the default), xbox, sony or poe2.
    pub(crate) realm: Option<String>,
    /// Characters per page, the template included: the forum's hard limit
    /// when omitted.
    pub(crate) size: Option<usize>,
    /// The page's text, holding `[items]` exactly once, which each page
    /// replaces with its items; omitted, a page is the items alone.
    pub(crate) template: Option<String>,
    /// One page, by its number from 1: that page's fields, and beside
    /// them every key of the whole render but `pages`.
    pub(crate) page: Option<usize>,
    /// List every entry of `posted`, `left_out` and `freshness`'s lists,
    /// past the first ten, the items nothing applies to included in
    /// `left_out` (a league holds thousands); they are counted either way.
    pub(crate) expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReferenceCurrencyParams {
    /// A tag, the game's word, or a legacy alias to resolve, exact and
    /// case-sensitive; omitted, the whole table.
    pub(crate) word: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct PriceSetParams {
    /// One address: `item/<id>`, `character/<id>`, `tab/<realm>/<id>` or
    /// `substash/<realm>/<parent>/<id>`. An answer holds a target as an
    /// object, `{scope, realm, parent, id}`: the address is its parts in
    /// that order joined by `/`, the absent ones left out.
    pub(crate) target: String,
    /// exact, negotiable, no_price or skip (the game's `price`, `b/o` and
    /// `~skip` work too).
    pub(crate) r#type: String,
    /// For exact and negotiable: a decimal of up to four places (`12.5`),
    /// or a `wanted/lot` ratio (`1/5`), as a string.
    pub(crate) amount: Option<String>,
    /// For exact and negotiable: a tag of reference_currency.
    pub(crate) currency: Option<String>,
    /// The revision of the target's own row this write replaces: price_show's
    /// `listing.manual.revision`, when `listing.manual.from` is the target
    /// itself, or the revision a refusal names. Required when the target
    /// has a row of its own; omit it, or give 0, only to create one.
    pub(crate) if_revision: Option<i64>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct PriceClearParams {
    /// The address, as for price_set.
    pub(crate) target: String,
    /// The revision of the target's own row this clear removes, as for
    /// price_set. Required: a clear without it is refused naming the
    /// current revision.
    pub(crate) if_revision: Option<i64>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SavedSearchesParams {
    /// One saved search, by its id or its name: its row. Never beside
    /// `limit` or `expand`, which list them.
    pub(crate) saved: Option<String>,
    /// List this many in place of ten.
    pub(crate) limit: Option<usize>,
    /// List every saved search.
    pub(crate) expand: Option<bool>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SavedSearchSetParams {
    /// The saved search written, by its id as listed; omitted, a new one
    /// under an id minted for it.
    pub(crate) id: Option<String>,
    /// The revision of the row at `id` this write replaces, from
    /// saved_searches or a refusal: the fields not named keep the row's
    /// values. Omitted, or 0, a create: at `id`, the whole search, landing
    /// only where none stands.
    pub(crate) if_revision: Option<i64>,
    /// A label, one to sixty-four characters; two searches may share one.
    pub(crate) name: Option<String>,
    /// The query, in the language `describe` prints; stored canonical.
    pub(crate) text: Option<String>,
    /// pc, xbox, sony, poe2 or all. A create naming none takes the
    /// account's one realm, and is refused offering each where it holds
    /// several.
    pub(crate) realm: Option<String>,
    /// live (a create's default), all or removed.
    pub(crate) membership: Option<String>,
    #[schemars(description = ACCOUNT)]
    pub(crate) account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct SavedSearchClearParams {
    /// The saved search's id, as listed.
    pub(crate) id: String,
    /// The revision it clears: its row's `revision`, or the one a refusal
    /// names.
    pub(crate) if_revision: i64,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct MainAccountSetParams {
    /// The account made the main one, as `accounts` lists it; null clears
    /// the main account, at `if_revision`.
    pub(crate) main: Option<String>,
    /// The revision of the main account replaced or cleared: `accounts`'
    /// `main.revision`. Omitted, or 0, a set landing only where none is.
    pub(crate) if_revision: Option<i64>,
}
