//! The request and the answer (C100, C96, C98): a query bound to a scope
//! and a view in, one answer shape out — the canonical query, the scope,
//! the total, the rows and how to go on, then the terms block and the
//! basis, in that order on both surfaces (C115).
//!
//! The shapes are shown once, in *The worked example* below; the
//! invariants cited by number are the crate's doc's. This is the crate's
//! boundary: tests pin a
//! request in and an answer out, as JSON, and a frontend's text is a
//! function of that JSON (C53).
//!
//! # The worked example
//!
//! Seven pc items: rare rings r1 (life 95, res 65), r2 (20 implicit + 75
//! explicit, 60), r3 (100, 55), r6 (readable 95, implicit array unread, res
//! subtotal 65 incomplete), r7 (no life line, 65); r4 a *magic* ring (95,
//! 70); r5 rare, life 95, res 65, its base missing from the class table.
//! Two fetched locations, one never fetched. Every count below is worked by
//! hand from those facts.
//!
//! **The request** (shape, not the Rust layout; a request sends `text` *or*
//! `tree`, an answer returns both):
//!
//! ```json
//! { "scope":  { "account": "A", "realm": "pc", "membership": "live" },
//!   "query":  { "tree": { "all": [
//!       { "field": "league", "op": "=", "value": "Standard" },
//!       { "field": "class",  "op": "=", "value": "Rings" },
//!       { "field": "rarity", "op": "=", "value": "rare" },
//!       { "exists": "lines", "where": { "all": [
//!           { "attr": "template", "op": "=",  "value": "# to maximum Life" },
//!           { "attr": "arg1",     "op": ">=", "value": 90 } ] } },
//!       { "value": { "pseudo": "total_res" }, "op": ">=", "number": 60 } ] } },
//!   "view":   { "rows": { "limit": 20 } } }
//! ```
//!
//! **The answer at a terminal:**
//!
//! ```text
//! query   league=Standard class=Rings rarity=rare line("# to maximum Life" arg1>=90) pseudo.total_res>=60
//! scope   account A · pc · live · 7 items · 2 locations fetched (oldest 3d, newest 2h)
//!         1 location never fetched: PremiumStash 1 of 3 · location list seen 2h ago · each tab: acq tabs --account A --realm pc --league Standard --never-fetched
//! total   1 match · 2 undecided
//! rows    r1  Two-Stone Ring · rare · Standard / Rings · fetched 2h ago
//!             +95 to maximum Life (explicit) · pseudo.total_res 65
//! next    1 of 1
//! terms   each term counted alone over all live pc items in all leagues, never over the matches
//!   0    league=Standard                        7 matched
//!   1    class=Rings                            6 matched · 1 undecided
//!   2    rarity=rare                            6 matched · 1 failed
//!   3    line("# to maximum Life" arg1>=90)     5 matched · 1 failed · 1 lacked · 1 reaches 90 only together
//!   4    pseudo.total_res>=60                   5 matched · 1 failed · 1 undecided
//! routes  (--routes prints all; one shown)
//!   term 3 failed, over the scope:
//!     acq search --account A --realm pc 'line("# to maximum Life") -line("# to maximum Life" arg1>=90)'
//! basis   store 3f9a1c0be27d · snapshot 41 · prices and policy revision 12 · totals v1 · classes v3
//! ```
//!
//! The two undecided items say why and what might help: r5 — *class: base
//! not in the class table; a refresh will not help, a reference update
//! may*; r6 — *pseudo.total_res: implicit lines unread; a refresh may
//! help*. These are hints, never guarantees.
//!
//! **The same route under `--json`** — directly resubmittable, carrying its
//! count and its denominator; the basis it was counted at is the answer's,
//! stated once (B3, 2026-10-10). Run where its count has moved it reports
//! both counts, and a caller who sends the answer's `basis` beside it as
//! `counted_at` is told wherever the basis has moved:
//!
//! ```json
//! { "count": 1, "denominator": "scope",
//!   "request": { "scope": { "account": "A", "realm": "pc", "membership": "live" },
//!                "query": { "text": "line(\"# to maximum Life\") -line(\"# to maximum Life\" arg1>=90)" },
//!                "view":  { "rows": {} } } }
//! ```
//!
//! It returns r2. Appending it to the original query instead would return
//! nothing, which is why a route is a request and never a fragment. A
//! bucket of a count view routes *under the query*: counting every item by
//! class gives `ring 6 · undecided 1`, and the undecided bucket's request
//! has the query `undecided(class)` and returns r5, which no root-undecided
//! list would ever contain; a `none` bucket routes by `-has:<key>` (C105).
//! A crossed cell's route carries both keys. Paths (`0`…`4`, `3.1`) belong
//! to this exact returned query; after an edit, use the next answer's.
//!
//! # Rulings held here
//!
//! - **C113** (a site URL is a request): a bare id or the query JSON
//!   fills no scope; `--realm` beside a URL is a conflict; a sort, which
//!   only the JSON envelope carries, is reported, never applied.
//!
//! # As built
//!
//! - **A site search is a query's third spelling** (C113; the brief's
//!   step 5): `query.trade` holds it — a URL, an id or the query JSON as
//!   a string, or the JSON as an object — and every read translates it
//!   again ([`trade::translate`]), so the report an answer holds is this
//!   build's and no caller's. The query is the translation's text, bound;
//!   a text or a tree sent beside it, as a printed request and a
//!   continuation send one, must be that text, and one that is not —
//!   edited, or translated by another build's tables — is refused with
//!   the translation as its reading, never run under the site's label
//!   (the owner, 2026-10-16, choosing this over the report carried as a
//!   label: "Site search, re-translated"). The answer's query carries the
//!   report, `trade`: the source, the fidelity, the remainder, every
//!   clause, and the versions of the catalogue and the stat table. They
//!   are the report's, not the basis's, which is the corpus's: a route's
//!   `counted_at` would otherwise move with no corpus moving, and the
//!   build the basis names digests both tables (`build.rs`). The URL's
//!   realm is the scope, and is in no term of the text, so a site search
//!   is read in a realm or not at all: [`QueryInput::read`] takes the
//!   realm its caller asks in — a request's scope, the realm of the item
//!   `show` holds — and refuses a URL that names another; a scope beside
//!   a URL is the resolved form an answer prints. It is the one reader of
//!   a site search, so none decodes one for itself and leaves the realm
//!   behind (the outside review of step 5: `show` read the query alone
//!   and showed a pc item matching an xbox URL). A
//!   continuation and the start offered across a changed basis carry the
//!   site search; a route, an offer to explain and a forced query are
//!   queries of their own and carry none. A command made of a request
//!   that carries one is `--trade` and that search
//!   ([`Request::command`]), with no `--realm` where its URL names one.
//!
//! - **The request** — `{account, realm | all, membership}`, the query, a
//!   view, and a basis when it continues (C98, C100; a route carries none, B3). A
//!   receiver whose selected account differs refuses it; rebinding is
//!   deliberate and shown. A request always carries its realm resolved: at
//!   a terminal an omitted `--realm` over a store holding one realm
//!   resolves to it and the answer prints it; over a store holding more
//!   than one it is C96's error.
//! - **Every term is counted over the whole scope** — matched, failed,
//!   lacked, undecided — before composition and before the limit, with no
//!   short circuit (C93). A count that is not zero carries its **route**:
//!   the request that returns exactly its members, with its
//!   denominator, the scope (invariant 4); the basis it was counted at
//!   is the answer's, stated once (B3, `search/LEDGER.md`). A route is
//!   a request and never a fragment: a term's failed members are asked of
//!   the scope, not appended to the query that counted them.
//! - **The routes of a term**: matched is the term; undecided is
//!   `undecided(term)`; lacked is `-has:<field>`, or `-line(<selector>)`;
//!   failed is `has:<field> -term` — `-term` alone for `class`, which
//!   never lacks (`class.rs`) — or `line(<selector>) -term` — with
//!   `or undecided(line(<selector>))` where the selector asks a flag, which
//!   an occurrence with unread flags leaves open; a `linked( … )` lacks
//!   with `-has:links`, an item with no link group, and fails with
//!   `has:links -term` (`sockets.rs`); a term
//!   that cannot lack fails by `-term`. The together count's is the failed
//!   route and the selector's sum against the bound.
//! - **The routes an answer carries** (C115): unasked, the ones a next
//!   step hangs on — what is undecided, at the root and of each term;
//!   what holds only together; what each term matched where nothing
//!   matched; what a count could not place, a key's `none` and
//!   `undecided` and a crossed cell holding either; an explained node's
//!   forced counts; what an ambiguous trade clause's other reading adds
//!   and drops, the translation's own undecided (C119) — and the requests
//!   that are no count's: a value list's
//!   rest, an id that left, a zero answer's offers, the rows' `next`.
//!   Every other count keeps its number and drops its request; the view's
//!   `routes` asks for every one, a terminal's `--routes`, and a
//!   continuation and the corpus offer keep it. A count with no request
//!   is still a count of the members its route would return: the answer
//!   weighed 31 KB by default with every request in it, over every seat
//!   runner's harness cut (the trade seat), and the terminal already
//!   printed these alone.
//! - **What is at stake in an ambiguous clause** (C119, `TRADE-SLICE.md` at `b6197a82`
//!   row 8c): where the clause carries its other reading (`translate.rs`,
//!   which says where one applies), the query with the conjunct the
//!   reading stands in for replaced by it is asked of every item by the
//!   one evaluator, and the clause's `other` counts the items it matches
//!   that the query as run fails — `adds` — and those the query matches
//!   that it fails — `drops`. Each count's route is the query with that
//!   conjunct replaced by what tells the two apart: the reading and not
//!   the conjunct, or the conjunct and not the reading, the shared part
//!   said once where the reading writes the conjunct inside it (`{term}
//!   or …` adds alone, `{term} -…` drops alone); it returns exactly the
//!   members counted, an item undecided either way in neither. A count
//!   of none is a number with no route, and a clause with no reading has
//!   no `other` (C93). Only where the site's search is the query run:
//!   under the corpus context it is set aside, and nothing is staked.
//! - **The report's clauses** (C100): an exact clause with a term of its
//!   own carries `at`, `status`, `term` and `path` alone, the text
//!   stating the rest; every other — not exact, or with no term of its
//!   own, a weighted sum's filter or a group — keeps the site's filter,
//!   its catalogue rows and what they say ([`ClauseOut`]).
//! - **A `:` or `~` selector is printed as authored with what it resolved
//!   to beside it** (invariant 2): the values its matched items carry,
//!   or — for a line's group, a `sum`'s too — the templates its selector
//!   picks over the scope, whatever its comparisons then make of them; the
//!   ten most carried listed and the rest counted. A quoted template
//!   resolves to itself and lists nothing, unless it found two spellings
//!   of one line, which any-case `=` can, and then both are listed — the
//!   spelling of the row, where it named a mod by a row (C90). The
//!   route to the rest is a count: the term alone over the scope — never
//!   the query, whose other terms would drop values the selector resolved
//!   to — counted by the field, or the vocabulary narrowed by the group's
//!   one template test (`counts.rs`); a `tab` selector has none, since a
//!   tab is counted by the tab and it resolved to names (outside audit,
//!   2026-09-22).
//! - **What a row shows of one term is bounded** and says how many it left
//!   out; the item whole is `show <id>` (invariant 5). Of lines and
//!   strings, six, a sum's value beside them. Of why an item is undecided,
//!   six of the item's unread parts in the item's order, each with every
//!   term that rests on it — so a block may hold more entries than six,
//!   and what it leaves out is counted in parts, never in entries
//!   (`eval::why`).
//! - **The undecided block is by reason, never by item** (C93's contract
//!   detail, V8, 2026-09-26; taken here at step 9b): an answer shows its
//!   undecided items as one line per distinct reason — by what was
//!   unread, the kind a count tallies (C105) — each with one example
//!   item, the first met in the store's order, then the one route to
//!   them all; after the rows, before the routes; the first ten listed
//!   and the rest counted (invariant 5). No item list and no count per
//!   reason: no term selects a reason, so such a count would have no
//!   route. The JSON has the same shape. A per-reason count and its
//!   selector wait for someone missing the number. The seat's first shape
//!   — ten items, each with every term's reason — printed one reason
//!   eighteen times under eighteen `class:` terms (F9).
//! - **Rows** are the matching items in the store's stable order, or by
//!   the sort scalar with items that have none last either way; past the
//!   limit they are counted, and the way on is the answer's `next`. A
//!   counts view shows none: the matches are handed to `counts.rs`
//!   instead, with the one maker of a route (`Router`, private), so a
//!   bucket's route is made as a term's is.
//! - **A continuation is a request with its basis and its place** (C98,
//!   C100; the build plan, step 10): the answer's own request, scope resolved and
//!   query canonical, with the basis it was counted at and how many rows
//!   of the order came before. The place is what makes a request a
//!   continuation: with one, a basis that differs in any part from the
//!   corpus's is refused, both named part by part (`Basis::against`),
//!   and the offer is the request from its start; without one a basis
//!   is compared with nothing, since a route runs on any basis. No
//!   answer is held between asks: the order is a function of the basis,
//!   the store's stable order or a stable sort over it, so a place in
//!   it needs nothing kept; a facts file deleted and refetched, or
//!   restored from a copy, to an id the basis already names is another
//!   basis by when that response was recorded (`corpus.rs`). Rows alone
//!   continue; a table's values past
//!   its limit are a larger limit's. The token a terminal carries is
//!   that request and nothing else, encoded ([`Request::token`]).
//! - **A route read back** ([`Asked`]) is the request inside it; the
//!   answer carries what it said when it was printed beside what it says
//!   now (*The worked example*, above) — where the count has
//!   moved, or, for a caller who sent the answer's basis with it,
//!   wherever the basis has, each part named.
//! - **A zero total** prints, in place of rows — the coverage being the
//!   scope block's, as in every answer: the selectors that
//!   resolved to nothing in this scope — a group's bound selector picked
//!   no occurrence, a field's term matched no item — each with the values
//!   sharing its words — a suggestion to type, never a match made for the author
//!   (S107: never fuzzy-matched); an `id:` that matched nothing, with the
//!   note that an id is whole (F7); the root's undecided route; and the
//!   offers — never their output: explain of each
//!   part of the query — a child of the root, a root that is no and its
//!   one part — the part fewest items match first and by path where two
//!   are equal, ten listed and the rest counted (invariant 5), then the
//!   corpus context beside the view the request had, where a query was
//!   typed. An offer is a request, made by the one maker of a route's
//!   (`Router::request`), and a command only as every route is one
//!   ([`Request::command`]): the block computes no forced count.
//! - **Explain is a view** (the build plan, step 10): it travels in the request as the rows and the counts do,
//!   so it stands beside neither, and an adapter's flag is an encoding
//!   of it. The path names a node of the query as returned
//!   (`tree::nodes`, the one walk that numbers them); the substitution
//!   is that walk's too, the one node replaced by `true()` or `false()`
//!   and nothing around it touched (invariant 1). Each transformed query
//!   is then bound and answered as any query is — this module asks
//!   itself, as the request it is, and reads the total — so a forced
//!   count is a count the build gives a query a user could type, by the
//!   one evaluator, and its route is that query. A path the query lacks
//!   is an authoring error whose readings list the nodes with their
//!   paths; a substitution the binder refuses is the binder's error;
//!   both are said before any item is read. The answer keeps its terms
//!   block and its total, shows no rows, and no zero block: its three
//!   pairs of counts stand where the rows would. Inside a member group
//!   the substitution is per member: it never creates a member, and an
//!   unread collection is not a population. Opt-in, one node at a time.
//! - **The corpus context**: the answer is the empty query's under whatever view the
//!   request has, and the query typed is carried as set aside — read
//!   first, so that one that does not bind is the error it is in any
//!   context, and counted nowhere: no term, no route and no bucket holds
//!   it. Nothing typed, nothing set aside.
//! - **A row says what it sorts by** (F6, the first seat: a sort on a line
//!   the query did not name showed the number alone): the field by its
//!   name, the occurrence whose slot is the largest, a sum's
//!   contributors, a computed value's inputs — shown as a matched term's
//!   evidence is, bounded the same way (`eval::sorted_by`).
//! - **Membership is the corpus's**, a scope value beside realm and
//!   never a term — `is:removed` is refused toward it (`bind.rs`)
//!   (`corpus.rs`; C108;
//!   the build plan, steps 10 and 11d): `live`; `all`, where the store's
//!   read hands over every item at the place it was last seen; or
//!   `removed`, the removed of that read alone. A request naming another
//!   than the corpus holds is refused, as a realm is. Under `all` and
//!   `removed` a removed item is a row like any other, marked with when
//!   it left and counted by every term; the scope block counts the live
//!   and the removed apart, whichever of them was searched; its price is
//!   lacked (`price.rs`); every route and every command carries the
//!   membership. No count of an answer under `all` says how many of its
//!   items are removed: the same request under `removed` does. Under
//!   `live` an `id:` term that matches nothing says whether the id is among what
//!   the read left out — an item removed, a tab or a character retired
//!   — with what it was, when it left and the route under `all`,
//!   whatever the total; under `all` and `removed` the same is said of
//!   a retired tab or character that held no item, with no route, there
//!   being nothing more to find. Never of an id that is also a live
//!   location's, and never of a live item's asked under `removed`.

use std::collections::HashMap;

use acquisition_store::age;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::bind::{self, Atom, Query, Term, Thing};
use crate::corpus::{Basis, Corpus, Coverage, Held, Place, Realm};
use crate::counts;
use crate::error::{LanguageError, SearchError};
use crate::eval::{self, Outcome, Scalar, Truth};
pub use crate::eval::{Evidence, Reason};
use crate::group;
use crate::trade::translate::{Clause, Fidelity, Reading, Status, Translation};
use crate::trade::{self, Decoded, Source};
use crate::tree::{Collection, Node, Op, Probe, Value, ValueRef};
use crate::{json, parse, print};

/// The register's wording for a line the search cannot name (C102, S107).
pub const S107: &str = "a line the search cannot name is shown as unknown, with what it knows of it, and never fuzzy-matched";
/// The zero answer's wording where a term's near values are offered: a
/// suggestion stands beside "never fuzzy-matched" and is said to be one
/// (the re-sit, mcp-opus fault 4: "Fenumus' Weave" under a promise of
/// no guessing, "a wrong lead").
pub const SUGGESTED_SAID: &str = "a line the search cannot name is shown as unknown, with what it knows of it, and never fuzzy-matched; a value that shares a word with the term is offered beside it as a suggestion with its own count, never as a match";

/// What a zero answer says of an `id:` term that matched nothing (F7, the
/// first seat): the help's rule, where the reader is.
pub const ID_WHOLE: &str =
    "an id is matched whole, as an answer printed it: a part of one finds nothing";

// How many rows an answer returns when the request names no limit, and
// how many of a bounded block's entries are listed before the rest are
// counted, is the store's one number (C53, the read economy's bound:
// `acquisition_store::bound::LISTED`), never a second one here.
use acquisition_store::bound::{LISTED, bounded};
/// How many near values a zero answer offers beside a term: offers, not a
/// list of entities, so not the read economy's bound.
const SUGGESTED: usize = 5;

// ---- the request ------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub query: QueryInput,
    #[serde(default)]
    pub view: View,
    /// What the answer is of: the query's matches, or the corpus — the
    /// empty query over the same scope, the query typed set aside.
    #[serde(default, skip_serializing_if = "Context::is_matches")]
    pub context: Context,
    /// The basis a continuation was counted at (C98, C100): what its
    /// place in the rows is a place in. Beside no place it is carried
    /// and compared with nothing — a route runs on any basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basis: Option<Basis>,
}

/// What an answer is of (the clap help):
/// `matches`, what every answer is unless it says otherwise, or `corpus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Context {
    #[default]
    Matches,
    Corpus,
}

impl Context {
    fn is_matches(&self) -> bool {
        *self == Context::Matches
    }

    /// The word an adapter's flag carries.
    pub fn parse(word: &str) -> Result<Context, LanguageError> {
        match word {
            _ if word.eq_ignore_ascii_case("matches") => Ok(Context::Matches),
            _ if word.eq_ignore_ascii_case("corpus") => Ok(Context::Corpus),
            _ => Err(LanguageError::new(
                crate::error::ErrorKind::View,
                format!(
                    "`{word}` is no context: `matches`, what every answer is of, or `corpus`, the empty query over the same scope with the query typed set aside"
                ),
            )
            .with_readings(vec!["matches".to_string(), "corpus".to_string()])),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    /// The account's uuid or name; a receiver holding another refuses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Resolved in every answer and every route; omitted only by an
    /// author over a store holding one realm (C96).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<Realm>,
    #[serde(default)]
    pub membership: Membership,
}

/// Read from a request by its word ([`Membership::parse`]), so a word
/// that is none of the three is refused in [`Membership::unknown`]'s
/// words on every surface, not serde's (C143, the request's words).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Membership {
    #[default]
    Live,
    /// Removed items too, each row marked with when it left.
    All,
    /// The removed items alone: what has left, and nothing live.
    Removed,
}

impl Membership {
    /// The word a request, an answer and a terminal's flag spell it by.
    pub fn as_str(self) -> &'static str {
        match self {
            Membership::Live => "live",
            Membership::All => "all",
            Membership::Removed => "removed",
        }
    }

    /// A request's or a terminal's word for one, or none where it is no
    /// membership.
    pub fn parse(word: &str) -> Option<Membership> {
        [Membership::Live, Membership::All, Membership::Removed]
            .into_iter()
            .find(|m| m.as_str() == word)
    }

    /// What a word that is no membership is refused in, wherever it was
    /// sent: the three, each with what it searches.
    pub fn unknown(word: &str) -> String {
        format!(
            "`{word}` is no membership: live, the items in the stash; all, the removed beside them; or removed, the removed alone"
        )
    }

    /// The store's read that hands its items over: the removed alone are
    /// the removed of the read that holds every item.
    pub(crate) fn read(self) -> acquisition_store::Membership {
        match self {
            Membership::Live => acquisition_store::Membership::Live,
            Membership::All | Membership::Removed => acquisition_store::Membership::All,
        }
    }
}

impl<'de> Deserialize<'de> for Membership {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Membership, D::Error> {
        let word = String::deserialize(d)?;
        Membership::parse(&word).ok_or_else(|| serde::de::Error::custom(Membership::unknown(&word)))
    }
}

/// The query as text or as a tree; an answer returns both (C104). Or a
/// trade site search (C113), its third spelling: a URL, an id or the
/// query JSON as a string, or the query JSON as an object, translated at
/// every read — and a text or a tree beside it is held to be the one the
/// site's search translates to (the module doc).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<Json>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade: Option<Json>,
}

/// What an answer shows of its matches, and whether every count it holds
/// carries its route (C115). On the wire it is the shape's one key —
/// `{"rows": …}`, `{"counts": …}` — with `"routes": true` beside it where
/// every route is asked: a terminal's `--routes`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct View {
    pub shape: Shape,
    /// Every count's route. Unasked, an answer carries the routes a next
    /// step hangs on and no other (the module doc, "The routes an answer
    /// carries").
    pub routes: bool,
}

/// The shape of a view: rows, or counts of them — one table for each key,
/// or one crossed table of two (C95) — or, of one node of the query, the
/// root's counts with it forced true and forced false, and no rows (the
/// module doc). One of the four.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Shape {
    Rows(Rows),
    Counts(Counts),
    Cross(Counts),
    Explain(Explain),
}

impl From<Shape> for View {
    fn from(shape: Shape) -> View {
        View {
            shape,
            routes: false,
        }
    }
}

impl Serialize for View {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(flatten)]
            shape: &'a Shape,
            #[serde(skip_serializing_if = "std::ops::Not::not")]
            routes: bool,
        }
        Wire {
            shape: &self.shape,
            routes: self.routes,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for View {
    /// Strict, as the request's every part is: `routes` a boolean, and
    /// beside it one shape or none — none being the default, rows.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<View, D::Error> {
        use serde::de::Error as _;
        let mut map = serde_json::Map::<String, Json>::deserialize(deserializer)?;
        let routes = match map.remove("routes") {
            None => false,
            Some(Json::Bool(routes)) => routes,
            Some(other) => {
                return Err(D::Error::custom(format!(
                    "`routes` is true or false, not {other}"
                )));
            }
        };
        let shape = if map.is_empty() {
            Shape::default()
        } else {
            Shape::deserialize(Json::Object(map)).map_err(D::Error::custom)?
        };
        Ok(View { shape, routes })
    }
}

/// The node explained, by its path in the query as an answer returns it
/// (`tree::nodes`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Explain {
    pub path: String,
}

impl Default for Shape {
    fn default() -> Shape {
        Shape::Rows(Rows::default())
    }
}

/// A counts view: the keys, as `counts.rs` binds them — a field, `line`,
/// `line:<text>`, `line~<pattern>` — and the one value summed beside each
/// count, a value's text as `--sort` takes one.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Counts {
    pub keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum: Option<String>,
    /// How many values a table lists, or cells a crossed one; the rest are
    /// counted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rows {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// A value's text: a number field, `line(P).<slot>`, `sum( … )`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub desc: bool,
    /// A continuation's place: how many rows of this order the answers
    /// before it returned. It is what makes a request a continuation,
    /// refused across a changed basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<usize>,
    /// What each row carries past its compact fields, by name (C100, a
    /// caller may name fields): `icon` alone, the URL of the item's art.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<RowField>,
}

/// A field a rows view may ask of each row beyond the compact ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowField {
    /// The URL of the item's art on GGG's CDN, as its body gives it: a
    /// page names it and the user's browser fetches it (C140).
    Icon,
}

/// A query input read in a realm ([`QueryInput::read`]): the query, the
/// realm it is asked in, and the translation where it was a site search.
#[derive(Debug, Clone)]
pub struct Read {
    pub query: Query,
    /// The realm asked in, or the one the site search's URL names where
    /// none was: none where neither names one.
    pub realm: Option<Realm>,
    pub translation: Option<Translation>,
}

impl QueryInput {
    /// The query read in a realm, and the translation it is where the
    /// input carries a site search: the translation's text, bound, with a
    /// text or a tree beside it held to be that text. One that is not was
    /// edited after the translation, or translated by another build: it
    /// is refused, never run under the site's label.
    ///
    /// A site search is a request, its URL's realm its scope (K6), and
    /// the realm is in no term of its text: so it is read in a realm or
    /// not at all. `realm` is the one the caller asks in — a request's
    /// scope, the realm of the item `show` holds — and none where the
    /// caller states none; a URL that names another is refused here, the
    /// one place a site search's realm is read, so no reader can leave
    /// it behind.
    pub fn read(&self, realm: Option<&Realm>) -> Result<Read, LanguageError> {
        let given = match (&self.text, &self.tree) {
            (Some(_), Some(_)) => {
                return Err(LanguageError::new(
                    crate::error::ErrorKind::Tree,
                    "a request sends `text` or `tree`, never both: an answer returns both",
                ));
            }
            (Some(text), None) => Some(bind::parse_query(text)?),
            (None, Some(tree)) => Some(bind::bind(&json::from_json(tree)?)?),
            (None, None) => None,
        };
        let Some(decoded) = self.decoded()? else {
            return Ok(Read {
                query: match given {
                    Some(query) => query,
                    None => bind::parse_query("")?,
                },
                realm: realm.cloned(),
                translation: None,
            });
        };
        let realm = match (realm, &decoded.realm) {
            (Some(asked), Some(url)) if asked.as_str() != url => {
                return Err(LanguageError::new(
                    crate::error::ErrorKind::Trade,
                    format!(
                        "the site search's URL names the realm `{url}`, and this is asked in `{}`: a URL's realm is the search's scope, which nothing outside it is in — ask in `{url}`, or send the search's id, which names no realm",
                        asked.as_str()
                    ),
                ));
            }
            (Some(asked), _) => Some(asked.clone()),
            (None, url) => url.clone().map(Realm::One),
        };
        let translation = trade::translate::translate(&decoded)?;
        if let Some(given) = given
            && given.text() != translation.text
        {
            return Err(LanguageError::new(
                crate::error::ErrorKind::Trade,
                format!(
                    "the query beside `trade` is not the text this build translates the site's search to — `{}`: a request that carries `trade` runs the site's search, so drop `trade` to run the edit as a query of your own, labelled as yours",
                    translation.text
                ),
            )
            .with_readings(vec![translation.text.clone()]));
        }
        Ok(Read {
            query: bind::parse_query(&translation.text)?,
            realm,
            translation: Some(translation),
        })
    }

    /// The site search the query carries, decoded with no network (C113):
    /// a string is a URL, an id or the query JSON, an object the query
    /// JSON. Read by [`QueryInput::read`] alone.
    fn decoded(&self) -> Result<Option<Decoded>, LanguageError> {
        match &self.trade {
            None => Ok(None),
            Some(Json::String(input)) => trade::decode(input).map(Some),
            Some(value @ Json::Object(_)) => trade::decode_json(value).map(Some),
            Some(_) => Err(LanguageError::new(
                crate::error::ErrorKind::Trade,
                "`trade` is a site search URL, its id or its query JSON as a string, or the query JSON as an object",
            )),
        }
    }

    /// The site search as a terminal types it after `--trade`.
    fn trade_word(&self) -> Option<String> {
        match self.trade.as_ref()? {
            Json::String(input) => Some(input.clone()),
            other => Some(other.to_string()),
        }
    }
}

/// The empty query: every item in scope.
fn is_empty(root: &Node) -> bool {
    matches!(root, Node::All(children) if children.is_empty())
}

impl Request {
    /// The request's query, read in its scope ([`QueryInput::read`]).
    pub fn read(&self) -> Result<Read, LanguageError> {
        self.query.read(self.scope.realm.as_ref())
    }

    /// The query of a request as typed, parsed or read, and bound.
    pub fn bind(&self) -> Result<Query, LanguageError> {
        self.read().map(|read| read.query)
    }

    /// The realm the request is asked in: the scope's, or the one its
    /// site search's URL names, which is the search's scope (K6) — a
    /// scope beside a URL being the resolved form an answer prints, and
    /// one that names another realm, or all, refused.
    pub fn realm(&self) -> Result<Option<Realm>, LanguageError> {
        self.read().map(|read| read.realm)
    }

    /// The query the answer is of, and the one set aside: under `corpus`
    /// the empty query, the typed one beside it — still read, so that one
    /// that does not bind is the error it is in any context. Explain is
    /// of the query typed, so beside `corpus` it is the view's error.
    /// The site search's translation, where the query is one, is the
    /// typed query's, and stands with it.
    fn asked(&self) -> Result<Asking, LanguageError> {
        if self.context == Context::Corpus && matches!(self.view.shape, Shape::Explain(_)) {
            return Err(LanguageError::new(
                crate::error::ErrorKind::View,
                "`explain` is of the query typed, and the `corpus` context sets it aside: ask one or the other",
            ));
        }
        let Read {
            query: typed,
            translation,
            ..
        } = self.read()?;
        Ok(match self.context {
            Context::Matches => Asking {
                query: typed,
                set_aside: None,
                translation,
            },
            Context::Corpus => Asking {
                query: bind::parse_query("")?,
                set_aside: (!is_empty(&typed.root) || translation.is_some()).then_some(typed),
                translation,
            },
        })
    }

    /// A continuation's place, where the request is one: a place names
    /// the basis it is a place in.
    fn place(&self) -> Result<Option<(usize, &Basis)>, LanguageError> {
        match (&self.view.shape, &self.basis) {
            (
                Shape::Rows(Rows {
                    after: Some(after), ..
                }),
                Some(basis),
            ) => Ok(Some((*after, basis))),
            (Shape::Rows(Rows { after: Some(_), .. }), None) => Err(LanguageError::new(
                crate::error::ErrorKind::View,
                "a place in the rows is a place at a basis: a continuation names the basis its answer was counted at, as an answer's `next` does",
            )),
            _ => Ok(None),
        }
    }

    /// The request as a token a terminal carries: `--next` takes it.
    pub fn token(&self) -> String {
        use base64::Engine as _;
        let json = serde_json::to_vec(self).unwrap_or_default();
        format!(
            "{TOKEN_PREFIX}{}",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json)
        )
    }

    /// The continuation a token is: a request with its basis and its
    /// place, and nothing that is not one.
    pub fn from_token(token: &str) -> Result<Request, LanguageError> {
        use base64::Engine as _;
        let no_token = || {
            LanguageError::new(
                crate::error::ErrorKind::Token,
                "this is no token an answer printed: a continuation takes one whole, as an answer's `next` gives it",
            )
        };
        let encoded = token
            .trim()
            .strip_prefix(TOKEN_PREFIX)
            .ok_or_else(no_token)?;
        let json = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|_| no_token())?;
        let request: Request = serde_json::from_slice(&json).map_err(|_| no_token())?;
        match request.place() {
            Ok(Some(_)) => Ok(request),
            _ => Err(no_token()),
        }
    }

    /// The corpus the request is asked of: its realm's, under its
    /// membership. A refusal that offers a realm offers this request in
    /// it (`error::Offered`).
    pub fn corpus(
        &self,
        store: &acquisition_store::Store,
        intent: &acquisition_store::Annotations,
    ) -> Result<Corpus, SearchError> {
        let realm = self.realm()?;
        Corpus::load_of(store, intent, realm.as_ref(), self.scope.membership)
            .map_err(|e| e.asked_by(self))
    }

    /// Every authoring error the request holds — the query's, the sort's,
    /// the view's, a path the query lacks, a realm its site search's URL
    /// contradicts — with no store read: what an adapter asks before it
    /// opens one.
    pub fn check(&self) -> Result<(), LanguageError> {
        self.place()?;
        self.realm()?;
        let asking = self.asked()?;
        self.view.shape.bound()?.of(&asking.query).map(|_| ())
    }
}

/// What a request asks: the query answered, the one set aside under the
/// corpus context, and the typed query's translation where it was a
/// site search.
struct Asking {
    query: Query,
    set_aside: Option<Query>,
    translation: Option<Translation>,
}

/// What a token starts with: the format, so that another is told apart.
const TOKEN_PREFIX: &str = "acq1.";

/// What `--request` reads: a request, or what an answer holds one in —
/// a count's route, the request with its count, and beside them the
/// basis it was counted at where the caller sends the answer's
/// (the module doc); a `next`; an offer.
#[derive(Debug, Clone, PartialEq)]
pub struct Asked {
    pub request: Request,
    pub before: Option<Before>,
}

/// What a route said when it was printed: its count, and the basis
/// where the caller sent one with it — an answer states its basis once,
/// and a route cut out of one carries none (B3, `search/LEDGER.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Before {
    pub count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counted_at: Option<Basis>,
}

impl Asked {
    /// A request, or an object with the request inside it, beside
    /// which nothing of a request stands — and where
    /// that object says a count it is a route, with the basis it was
    /// counted at where one is sent; a basis beside no count labels
    /// nothing, and what is neither is the tree's error, as a request
    /// that does not read is.
    pub fn read(value: &Json) -> Result<Asked, LanguageError> {
        let unread = |e: serde_json::Error| {
            LanguageError::new(
                crate::error::ErrorKind::Tree,
                format!("this is no request and no route an answer printed: {e}"),
            )
        };
        if !value.is_object() {
            return Err(LanguageError::new(
                crate::error::ErrorKind::Tree,
                "this is no request and no route an answer printed: a request is an object",
            ));
        }
        // what is read from inside its holder is whole (K7): a part of
        // a request written beside it would be dropped unread
        if value.get("request").is_some()
            && let Some(beside) = ["scope", "query", "view", "context", "basis"]
                .into_iter()
                .find(|key| value.get(key).is_some())
        {
            return Err(LanguageError::new(
                crate::error::ErrorKind::Tree,
                format!(
                    "`{beside}` has nowhere to stand beside `request`: a request an answer held is whole, its scope, its query and its view — write `{beside}` inside it, or send the request alone"
                ),
            ));
        }
        match value.get("request") {
            Some(request) => Ok(Asked {
                request: serde_json::from_value(request.clone()).map_err(unread)?,
                before: match (value.get("count"), value.get("counted_at")) {
                    (Some(count), None) => Some(
                        serde_json::from_value(serde_json::json!({ "count": count }))
                            .map_err(unread)?,
                    ),
                    (Some(count), Some(counted_at)) => Some(
                        serde_json::from_value(serde_json::json!({
                            "count": count,
                            "counted_at": counted_at,
                        }))
                        .map_err(unread)?,
                    ),
                    (None, Some(_)) => {
                        return Err(LanguageError::new(
                            crate::error::ErrorKind::Tree,
                            "this is no request and no route an answer printed: a basis beside no count labels nothing",
                        ));
                    }
                    (None, None) => None,
                },
            }),
            None if value.get("count").is_some() || value.get("counted_at").is_some() => {
                Err(LanguageError::new(
                    crate::error::ErrorKind::Tree,
                    "this is no request and no route an answer printed: a route holds its request",
                ))
            }
            None => Ok(Asked {
                request: serde_json::from_value(value.clone()).map_err(unread)?,
                before: None,
            }),
        }
    }

    /// The answer, and beside it what the route said when it was
    /// printed: where its basis was sent, wherever the basis has moved
    /// since, the count the same or not; where none was, wherever the
    /// count has.
    pub fn answer(&self, corpus: &Corpus) -> Result<Answer, SearchError> {
        let mut answered = answer(corpus, &self.request)?;
        let matched = answered.total.matched;
        answered.before = self
            .before
            .clone()
            .filter(|before| match &before.counted_at {
                Some(basis) => *basis != corpus.basis,
                None => before.count != matched,
            });
        Ok(answered)
    }
}

/// A view with its names bound.
enum BoundView {
    Rows {
        sort: Option<(String, bind::SortKey)>,
        desc: bool,
        limit: usize,
        fields: Vec<RowField>,
    },
    Counts(counts::BoundCounts),
    Cross(counts::BoundCounts),
    Explain(String),
}

/// One node of a query, and the query with it forced each way.
struct Explained {
    node: crate::tree::Pathed,
    forced: [Node; 2],
}

impl BoundView {
    /// The view of one query: for explain, the node its path names and
    /// the two transformed queries, each bound as any query is — a path
    /// the query lacks is an authoring error whose readings list the
    /// query's nodes with their paths, and a substitution the binder
    /// refuses is refused with the binder's own error.
    fn of(&self, query: &Query) -> Result<Option<Explained>, LanguageError> {
        let BoundView::Explain(path) = self else {
            return Ok(None);
        };
        let nodes = crate::tree::nodes(&query.root);
        let lacking = |nodes: Vec<crate::tree::Pathed>| {
            let message = if nodes.is_empty() {
                format!("the query has no node at `{path}`: the empty query has none")
            } else {
                format!(
                    "the query has no node at `{path}`: a path names a term, a group of terms, or a condition inside line( … ) or linked( … ), as this query numbers them"
                )
            };
            LanguageError::new(crate::error::ErrorKind::NoSuchPath, message).with_readings(
                nodes
                    .into_iter()
                    .map(|node| format!("{}  {}", node.path, node.text))
                    .collect(),
            )
        };
        let (Some(forced_true), Some(forced_false)) = (
            crate::tree::forced(&query.root, path, true),
            crate::tree::forced(&query.root, path, false),
        ) else {
            return Err(lacking(nodes));
        };
        let Some(node) = nodes.iter().find(|node| node.path == *path).cloned() else {
            return Err(lacking(nodes));
        };
        bind::bind(&forced_true)?;
        bind::bind(&forced_false)?;
        Ok(Some(Explained {
            node,
            forced: [forced_true, forced_false],
        }))
    }
}

impl Shape {
    /// The view an adapter's flat arguments name: rows unless a count, a
    /// crossed table or an explained path is asked for, and never two of
    /// them. A sort orders rows and a sum stands beside a count, so either
    /// with the other view is an authoring error and not a flag ignored;
    /// explain shows counts and no rows, so beside a count, a crossed
    /// table or a sort it is one too. A limit bounds what a view lists,
    /// and explain lists nothing: the one an adapter always sends is no
    /// conflict.
    pub fn of(
        count: Option<Vec<String>>,
        cross: Option<Vec<String>>,
        sum: Option<String>,
        explain: Option<String>,
        rows: Rows,
    ) -> Result<Shape, LanguageError> {
        let view_error =
            |message: &str| Err(LanguageError::new(crate::error::ErrorKind::View, message));
        if let Some(path) = explain {
            let beside = [
                ("count", count.is_some()),
                ("cross", cross.is_some()),
                ("sum", sum.is_some()),
                ("sort", rows.sort.is_some()),
            ];
            if let Some((flag, _)) = beside.iter().find(|(_, typed)| *typed) {
                return view_error(&format!(
                    "`explain` shows the root's counts with one node forced true and forced false, and no rows: `{flag}` has nothing to stand beside — an answer has one view"
                ));
            }
            return Ok(Shape::Explain(Explain { path }));
        }
        let limit = rows.limit;
        let keys = match (count, cross) {
            (Some(_), Some(_)) => {
                return view_error(
                    "`count` is a table for each key and `cross` one table of two: an answer has one view",
                );
            }
            (None, None) if sum.is_some() => {
                return view_error(
                    "`sum` stands beside a count: name what to count by, a `count` key or two `cross` keys",
                );
            }
            (None, None) => return Ok(Shape::Rows(rows)),
            (Some(keys), None) => (keys, false),
            (None, Some(keys)) => (keys, true),
        };
        if rows.sort.is_some() {
            return view_error(
                "`sort` orders rows, and a count shows none: a table is ranked by its counts",
            );
        }
        let counts = Counts {
            keys: keys.0,
            sum,
            limit,
        };
        Ok(if keys.1 {
            Shape::Cross(counts)
        } else {
            Shape::Counts(counts)
        })
    }

    fn bound(&self) -> Result<BoundView, LanguageError> {
        Ok(match self {
            Shape::Rows(rows) => BoundView::Rows {
                sort: match &rows.sort {
                    Some(text) => {
                        let value = parse::parse_value(text)?;
                        Some((print::print_value(&value), bind::bind_sort(&value)?))
                    }
                    None => None,
                },
                desc: rows.desc,
                limit: rows.limit.unwrap_or(LISTED),
                fields: rows.fields.clone(),
            },
            Shape::Counts(c) => {
                BoundView::Counts(counts::bind(&c.keys, false, c.sum.as_deref(), c.limit)?)
            }
            Shape::Cross(c) => {
                BoundView::Cross(counts::bind(&c.keys, true, c.sum.as_deref(), c.limit)?)
            }
            Shape::Explain(explain) => BoundView::Explain(explain.path.clone()),
        })
    }
}

// ---- the answer ---------------------------------------------------------------------------------

/// The answer, its fields in the order a reader needs them (C115): what
/// was asked and over what, the total, the view and how to go on, and
/// only then the terms block, each term counted alone over the scope, and
/// the basis. An agent reads the JSON as text, so the order is the
/// serialized order too (the trade seat: "I learned to scroll for
/// `total`").
#[derive(Debug, Clone, Serialize)]
pub struct Answer {
    pub query: QueryOut,
    /// Under the corpus context, the query typed beside it: read, set
    /// aside, and counted nowhere.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_aside: Option<QueryOut>,
    pub scope: ScopeOut,
    pub total: Total,
    /// What a route said when it was printed, where it is run on a later
    /// basis (`Asked::answer`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Before>,
    pub rows: Vec<Row>,
    pub view: ViewOut,
    /// In place of rows, when the total is zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero: Option<Zero>,
    pub terms: Vec<TermOut>,
    pub basis: Basis,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryOut {
    /// Where the query is a site search translated (C113): the report,
    /// first, as a terminal opens with it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade: Option<TradeOut>,
    pub text: String,
    pub tree: Json,
}

/// A site search's translation as the answer reports it (C113, K6): what
/// the input was, every clause with its status, the remainder and the
/// fidelity, and the versions of the three tables it was read by — the
/// catalogue, the stat table and the forms table, which the build named
/// in the basis digests (`build.rs`). The text and the tree are the query's own.
#[derive(Debug, Clone, Serialize)]
pub struct TradeOut {
    pub source: Source,
    pub fidelity: Fidelity,
    /// The `at` of every unsupported clause: what the text does not say.
    pub remainder: Vec<String>,
    pub clauses: Vec<ClauseOut>,
    pub catalogue: u32,
    pub stats: String,
    pub forms: u32,
}

impl From<Translation> for TradeOut {
    fn from(t: Translation) -> TradeOut {
        TradeOut {
            source: t.source,
            fidelity: t.fidelity,
            remainder: t.remainder,
            clauses: t.clauses.into_iter().map(ClauseOut::from).collect(),
            catalogue: t.catalogue,
            stats: t.stats,
            forms: t.forms,
        }
    }
}

/// One clause of a site search as the answer reports it (C113, K6): where
/// it stands in the site's request, its status, and its term and path —
/// and, where it is not exact, what the site wrote, the catalogue's rows
/// read for it and what they say; for an ambiguous clause whose row
/// carries its other reading, what is at stake (C119).
///
/// An exact clause with a term of its own is that term (C100: one
/// shape, its evidence once): the text states it, the terms block counts
/// it by its path, and its `site`, `rows` and `notes` would say again
/// what the term says, a pseudo stat's sentence once a filter (the trade
/// seat's ring URL: its report 2.7 KB of a 7.9 KB answer of three rows).
/// A clause with no term of its own — a filter inside a weighted sum,
/// whose sum is its group's term, or a group that writes none — is not
/// stated by the text, and keeps its whole row, as every clause that
/// differs from the text does; the catalogue's sentences stay in
/// `describe trade`.
#[derive(Debug, Clone, Serialize)]
pub struct ClauseOut {
    pub at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Json>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<String>,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    /// What the clause's other reading would change, counted over the
    /// scope (C119): absent where its row carries none (C93).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Stake>,
    /// The reading the stake is counted from; the answer's alone.
    #[serde(skip)]
    reading: Option<Reading>,
}

impl From<Clause> for ClauseOut {
    fn from(c: Clause) -> ClauseOut {
        // slimmed only where the text states it: exact, with a term of
        // its own — a filter inside a weighted sum, or a group, has none
        let exact = c.status == Status::Exact && c.term.is_some();
        ClauseOut {
            at: c.at,
            site: (!exact).then_some(c.site),
            rows: if exact { Vec::new() } else { c.rows },
            status: c.status,
            term: c.term,
            path: c.path,
            notes: if exact { Vec::new() } else { c.notes },
            other: None,
            reading: c.other,
        }
    }
}

/// What an ambiguous clause's other reading would change (C119): the
/// reading, and the items in scope it would add to the answer and drop
/// from it, against the query as run — each a count with its route.
#[derive(Debug, Clone, Serialize)]
pub struct Stake {
    pub reading: String,
    pub adds: Count,
    pub drops: Count,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScopeOut {
    pub account: Account,
    pub realm: Realm,
    pub membership: Membership,
    #[serde(flatten)]
    pub coverage: Coverage,
    /// A named realm this store has nothing under.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub realm_not_held: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Account {
    pub uuid: String,
    pub name: Option<String>,
}

/// A count, and — when it is not zero — the request that returns exactly
/// its members (invariant 4).
#[derive(Debug, Clone, Serialize)]
pub struct Count {
    pub count: usize,
    #[serde(flatten)]
    pub route: Option<Route>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Route {
    /// What the count is a part of: always the scope.
    pub denominator: &'static str,
    pub request: Request,
}

#[derive(Debug, Clone, Serialize)]
pub struct TermOut {
    pub path: String,
    pub term: String,
    pub matched: Count,
    pub failed: Count,
    pub lacked: Count,
    pub undecided: Count,
    /// C92: items no occurrence of which meets the lower bound, and whose
    /// occurrences do together. Absent where it does not apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub together: Option<Together>,
    /// What a `:` or `~` selector resolved to (invariant 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Resolved>,
    /// Of an `id:` term that matches nothing live: that the id is among
    /// what left, when, and the route to it under membership `all`.
    /// Under `all` and `removed`, of a retired place's that held nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Removed>,
}

/// An id the scope held and holds no longer: a handle from an earlier
/// answer never reads as an item that did not exist.
#[derive(Debug, Clone, Serialize)]
pub struct Removed {
    /// `item`, `tab` or `character`.
    pub what: &'static str,
    pub removed_at: i64,
    /// C53: `removed_at` in words at the answer's time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_at_age: Option<String>,
    /// Under `live`, the request that finds it under `all`. None under
    /// `all`, where a retired tab or character that matches nothing held
    /// no item.
    #[serde(flatten)]
    pub route: Option<Route>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Together {
    #[serde(flatten)]
    pub count: Count,
    pub slot: String,
    pub bound: Json,
}

#[derive(Debug, Clone, Serialize)]
pub struct Resolved {
    /// `template`, or the field's name.
    pub of: String,
    pub values: Vec<Carried>,
    /// Values past the listed ones.
    pub more: usize,
    /// The count that lists every value, the listed ones too: none where
    /// no count does (the module doc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest: Option<Route>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Carried {
    pub value: String,
    pub items: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Total {
    pub matched: usize,
    pub undecided: Count,
    /// Why items are undecided at the root: one entry per distinct reason
    /// — by what was unread, the kind a count tallies (C105) — with one
    /// example item, the first met; the first ten, the rest counted (the
    /// module doc, V8).
    pub undecided_reasons: Vec<UndecidedReason>,
    #[serde(skip_serializing_if = "is_zero")]
    pub reasons_left_out: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct UndecidedReason {
    /// The example's own: its problem names the item's part.
    #[serde(flatten)]
    pub reason: Reason,
    pub example: Example,
}

/// An item named where a whole row is too much.
#[derive(Debug, Clone, Serialize)]
pub struct Example {
    pub id: String,
    pub name: Option<String>,
}

/// The view as answered: the rows' bounds, the tables (`counts.rs`), or
/// the node explained.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewOut {
    /// Boxed, as the explained node is: the continuation it carries holds
    /// a request, the basis among it.
    Rows(Box<RowsOut>),
    Counts(counts::CountsOut),
    Cross(counts::CrossOut),
    Explain(Box<ExplainOut>),
}

/// One node explained (the module doc): the root's matched and
/// undecided counts as given, with the node forced true and with it
/// forced false, each forced pair beside its transformed query. The
/// words are literal: forced, never removed, never a contribution.
#[derive(Debug, Clone, Serialize)]
pub struct ExplainOut {
    pub path: String,
    /// The node, as the canonical text prints it.
    pub node: String,
    pub given: Given,
    pub forced_true: Forced,
    pub forced_false: Forced,
}

/// The root's counts as the query was given: the answer's own total.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Given {
    pub matched: usize,
    pub undecided: usize,
}

/// The root's counts of the transformed query, each with its route: the
/// transformed query, asked as a request of its own.
#[derive(Debug, Clone, Serialize)]
pub struct Forced {
    pub query: QueryOut,
    pub matched: Count,
    pub undecided: Count,
}

/// A continuation (C100): the request that returns the rows after
/// these, naming the basis, the query, the view and the order it
/// continues, and refused across a change (C98). The token a terminal
/// carries is that request encoded ([`Request::token`]), made where a
/// command is printed and never a second copy in the JSON (C115).
#[derive(Debug, Clone, Serialize)]
pub struct Next {
    pub request: Request,
}

#[derive(Debug, Clone, Serialize)]
pub struct RowsOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub desc: bool,
    pub limit: usize,
    pub returned: usize,
    /// Where this answer's rows begin in the order: a continuation's
    /// place, absent on a first answer.
    #[serde(skip_serializing_if = "is_zero")]
    pub after: usize,
    /// Matching items past these rows.
    pub left_out: usize,
    /// How to go on, where rows are left out: the continuation as a
    /// request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<Next>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Row {
    pub id: String,
    pub name: Option<String>,
    pub typeline: Option<String>,
    pub base: Option<String>,
    pub rarity: Option<String>,
    /// The URL of the item's art, where the view asked for it: null where
    /// the body gives none, absent where it was not asked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    pub place: Place,
    /// When the item left, under membership `all` or `removed`: a fetch
    /// of its place no longer had it, or the place was retired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_at: Option<i64>,
    /// C53: `removed_at` in words at the answer's time, where the item
    /// left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_at_age: Option<String>,
    pub matched: Vec<Touched>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sorted>,
}

/// What a row shows of one term it matched: the term by its path in the
/// terms block, and what it touched. The term's text is the terms
/// block's, said once there (C53, C115: a row of the trade seat's ring
/// search carried each of its four terms whole, every row).
#[derive(Debug, Clone, Serialize)]
pub struct Touched {
    pub path: String,
    pub shows: Vec<Evidence>,
    /// What the term touched past what is shown — lines or strings, or,
    /// of reasons, unread parts of the item: `show <id>` has the item whole
    /// (invariant 5).
    #[serde(skip_serializing_if = "is_zero")]
    pub left_out: usize,
}

fn is_zero(n: &usize) -> bool {
    *n == 0
}

/// An item's sort scalar, or why it has none (C92), and what it sorts by
/// (F6, the first seat): the field with its name, the occurrence whose
/// slot is the largest, a sum's contributors, a computed value's inputs —
/// as a matched term's `shows` are, bounded the same way.
#[derive(Debug, Clone, Serialize)]
pub struct Sorted {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Json>,
    /// `no satisfying occurrence`, or `incomplete` beside what was readable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'static str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shows: Vec<Evidence>,
    #[serde(skip_serializing_if = "is_zero")]
    pub left_out: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Zero {
    /// Selectors nothing in this scope carries, with what shares their
    /// words.
    pub resolved_to_nothing: Vec<Nothing>,
    pub said: &'static str,
    pub undecided: Count,
    /// What a zero answer offers (the module doc, "A zero total"): each a
    /// request, never its output.
    pub offers: Offers,
}

/// What a zero answer offers: explain of each part of the query — a
/// child of the root — the part fewest items match first and by path
/// where two are equal, the first ten and the rest counted (invariant
/// 5); then the corpus context, where a query was typed.
#[derive(Debug, Clone, Serialize)]
pub struct Offers {
    pub explain: Vec<ExplainOffer>,
    #[serde(skip_serializing_if = "is_zero")]
    pub explain_left_out: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Offer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExplainOffer {
    pub path: String,
    pub node: String,
    pub request: Request,
}

#[derive(Debug, Clone, Serialize)]
pub struct Offer {
    pub request: Request,
}

#[derive(Debug, Clone, Serialize)]
pub struct Nothing {
    pub path: String,
    pub term: String,
    pub of: String,
    pub suggestions: Vec<Suggestion>,
    /// Why nothing could carry it, where that is not S107's: an `id:`
    /// that is no whole id ([`ID_WHOLE`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Suggestion {
    pub value: String,
    pub items: usize,
    /// The exact term that selects it.
    pub term: String,
}

// ---- answering -----------------------------------------------------------------------------------

/// Answer a request from a held corpus. The corpus is the scope: a
/// request naming another realm or account than it holds is refused.
/// Every age the text prints is said beside its epoch at the answer's
/// own time, the clock read now ([`Answer::aged`]): a corpus is held
/// across answers, so its places are aged by each answer, never by the
/// load.
pub fn answer(corpus: &Corpus, request: &Request) -> Result<Answer, SearchError> {
    unaged(corpus, request).map(|answered| answered.aged(acquisition_store::now()))
}

impl Answer {
    /// C53 (an age the text prints stands in words beside its epoch,
    /// made at the answer's own time): the scope's fetch and list ages,
    /// each row's place and removal, and each term's removed id, said
    /// at `now`. Said again at another `now`, the words are that time's.
    pub fn aged(mut self, now: i64) -> Answer {
        self.scope.coverage.aged(now);
        for row in &mut self.rows {
            row.place.aged(now);
            row.removed_at_age = row.removed_at.map(|at| age::ago(now, Some(at)));
        }
        for term in &mut self.terms {
            if let Some(removed) = &mut term.removed {
                removed.removed_at_age = Some(age::ago(now, Some(removed.removed_at)));
            }
        }
        self
    }
}

fn unaged(corpus: &Corpus, request: &Request) -> Result<Answer, SearchError> {
    if let Some(account) = &request.scope.account
        && !account_is(corpus, account)
    {
        return Err(SearchError::scope(
            "account_mismatch",
            format!(
                "this request is bound to account `{account}` and the store selected is {}'s: rebinding is deliberate (not built: it waits for a consumer that asks)",
                corpus
                    .account_name
                    .as_deref()
                    .unwrap_or(&corpus.basis.account)
            ),
        ));
    }
    if request.scope.membership != corpus.membership {
        return Err(SearchError::scope(
            "membership_mismatch",
            format!(
                "this corpus holds {} and the request names membership `{}`",
                match corpus.membership {
                    Membership::Live => "live items",
                    Membership::All => "all items",
                    Membership::Removed => "the removed items alone",
                },
                request.scope.membership.as_str(),
            ),
        ));
    }
    if let Some(realm) = &request.realm()?
        && *realm != corpus.realm
    {
        return Err(SearchError::scope(
            "realm_mismatch",
            format!(
                "this corpus holds `{}` and the request names `{}`",
                corpus.realm.as_str(),
                realm.as_str()
            ),
        ));
    }
    let Asking {
        query,
        set_aside,
        translation,
    } = request.asked()?;
    let view = request.view.shape.bound()?;
    let explained = view.of(&query)?;
    let typed = set_aside.as_ref().unwrap_or(&query).text();
    let resolved = Scope {
        account: Some(
            corpus
                .account_name
                .clone()
                .unwrap_or_else(|| corpus.basis.account.clone()),
        ),
        realm: Some(corpus.realm.clone()),
        membership: corpus.membership,
    };
    // a continuation is a place in one basis's order: across a change it
    // is refused, both named, and the way on is the request from its start
    let after = match request.place()? {
        Some((_, basis)) if *basis != corpus.basis => {
            let mut start = request.clone();
            start.scope = resolved;
            start.query = QueryInput {
                text: Some(typed),
                tree: None,
                trade: request.query.trade.clone(),
            };
            start.basis = None;
            if let Shape::Rows(rows) = &mut start.view.shape {
                rows.after = None;
            }
            return Err(SearchError::scope(
                "basis_changed",
                format!(
                    "this continues an answer counted at another basis, and its place is no place in this one — {}: ask again from the start",
                    basis.against(&corpus.basis)
                ),
            )
            .offering(vec![start]));
        }
        Some((after, _)) => after,
        None => 0,
    };

    let router = Router {
        scope: Scope {
            account: Some(
                corpus
                    .account_name
                    .clone()
                    .unwrap_or_else(|| corpus.basis.account.clone()),
            ),
            realm: Some(corpus.realm.clone()),
            membership: corpus.membership,
        },
        root: &query.root,
    };
    let count = |n: usize, node: Option<&Node>| router.count(n, node);

    // every term over every item, then the tree
    let n_terms = query.terms.len();
    let mut tallies = vec![[0usize; 4]; n_terms];
    let mut together = vec![0usize; n_terms];
    // whether a group's selector picked any occurrence in the scope: what
    // the zero block means by a selector that resolved to nothing
    let mut picked = vec![false; n_terms];
    let mut carried: Vec<Option<HashMap<String, usize>>> = query
        .terms
        .iter()
        .map(|t| resolves(t).map(|_| HashMap::new()))
        .collect();
    let mut matches: Vec<(usize, Vec<Outcome>)> = Vec::new();
    let mut reasons: Vec<UndecidedReason> = Vec::new();
    let mut n_undecided = 0usize;
    let mut outcomes: Vec<Outcome> = Vec::with_capacity(n_terms);
    // each item's outcome at the root, for what an ambiguous clause's
    // other reading would change (C119)
    let mut truths: Vec<Truth> = Vec::with_capacity(corpus.items.len());
    // the query's parts — the children of the root, a root that is no and
    // its one part — and how many items each holds on: what a zero answer
    // orders its offers by, counted while nothing has matched
    let parts = eval::parts(&query.bound);
    let mut part_matched = vec![0usize; parts.len()];
    for (at, held) in corpus.items.iter().enumerate() {
        eval::asked(&query.terms, held, &mut outcomes);
        if matches.is_empty() {
            for (n, part) in parts.iter().enumerate() {
                if eval::truth(part, &outcomes) == Truth::True {
                    part_matched[n] += 1;
                }
            }
        }
        for (i, term) in query.terms.iter().enumerate() {
            let outcome = outcomes[i];
            tallies[i][outcome as usize] += 1;
            if let (Atom::Lines(group), Outcome::Failed) = (&term.atom, outcome)
                && eval::together(held, group)
            {
                together[i] += 1;
            }
            if !picked[i]
                && let Atom::Lines(group) | Atom::Sum { group, .. } = &term.atom
            {
                picked[i] = eval::selected(held, group).next().is_some();
            }
            if let Some(carried) = &mut carried[i] {
                for value in resolved_values(term, held, outcome) {
                    *carried.entry(value).or_default() += 1;
                }
            }
        }
        let truth = eval::truth(&query.bound, &outcomes);
        truths.push(truth);
        match truth {
            Truth::True => matches.push((at, outcomes.clone())),
            Truth::Undecided => {
                n_undecided += 1;
                // one entry per kind of reason, the first item met its
                // example (V8): the whole list, so the rest can be counted
                let mut blamed = Vec::new();
                eval::blame(&query.bound, &outcomes, &mut blamed);
                for reason in eval::distinct_reasons(&query.terms, &blamed, held) {
                    if !reasons.iter().any(|r| r.reason.unread == reason.unread) {
                        reasons.push(UndecidedReason {
                            reason,
                            example: Example {
                                id: held.item.facts.id.clone(),
                                name: label(held),
                            },
                        });
                    }
                }
            }
            Truth::False => {}
        }
    }

    let terms: Vec<TermOut> = query
        .terms
        .iter()
        .enumerate()
        .map(|(i, term)| {
            let routes = routes(term);
            let tally = tallies[i];
            TermOut {
                removed: match &term.atom {
                    Atom::Id(id) if tally[Outcome::Matched as usize] == 0 => {
                        corpus.gone.get(id).map(|left| Removed {
                            what: match left.kind.as_str() {
                                "item" => "item",
                                "stash" => "tab",
                                _ => "character",
                            },
                            removed_at: left.removed_at,
                            removed_at_age: None,
                            route: (corpus.membership == Membership::Live).then(|| {
                                let mut route = router.route(&term.node, None);
                                route.request.scope.membership = Membership::All;
                                route
                            }),
                        })
                    }
                    _ => None,
                },
                path: term.path.clone(),
                term: print::print(&term.node),
                matched: count(tally[Outcome::Matched as usize], Some(&term.node)),
                failed: count(tally[Outcome::Failed as usize], routes.failed.as_ref()),
                lacked: count(tally[Outcome::Lacked as usize], routes.lacked.as_ref()),
                undecided: count(tally[Outcome::Undecided as usize], Some(&routes.undecided)),
                together: routes.together.as_ref().map(|(node, lower)| Together {
                    count: count(together[i], Some(node)),
                    slot: lower.slot.clone(),
                    bound: eval::number_json(lower.bound.as_f64()),
                }),
                resolved: carried[i]
                    .take()
                    .filter(|carried| !exact_only(term) || carried.len() > 1)
                    .zip(resolves(term))
                    .map(|(carried, of)| {
                        let mut values: Vec<Carried> = carried
                            .into_iter()
                            .map(|(value, items)| Carried { value, items })
                            .collect();
                        values.sort_by(|a, b| {
                            b.items.cmp(&a.items).then_with(|| a.value.cmp(&b.value))
                        });
                        // every pick of a category term is printed: the
                        // ids are a closed set, and no count lists the rest
                        let whole = matches!(
                            &term.atom,
                            Atom::Closed {
                                thing: Thing::Category,
                                ..
                            }
                        );
                        let (values, more) = bounded(values, None, whole);
                        let rest = (more > 0)
                            .then(|| rest_of(term))
                            .flatten()
                            .map(|(node, key)| router.continuation(&node, key));
                        Resolved {
                            of,
                            values,
                            more,
                            rest,
                        }
                    }),
            }
        })
        .collect();

    // the empty query has no term for `undecided( … )` to ask about, and
    // nothing on an item can leave it open
    let root_undecided = (!is_empty(&query.root))
        .then(|| Node::Undecided(Probe::Term(Box::new(query.root.clone()))));
    let (reasons, reasons_left_out) = bounded(reasons, None, false);
    let total = Total {
        matched: matches.len(),
        undecided: count(n_undecided, root_undecided.as_ref()),
        undecided_reasons: reasons,
        reasons_left_out,
    };

    // the view: counts of the matches, or the rows — the order, then the
    // limit
    let at: Vec<usize> = matches.iter().map(|(at, _)| *at).collect();
    let counted = counts::Matches {
        corpus,
        at: &at,
        router: &router,
    };
    let (sort, desc, limit, fields) = match &view {
        BoundView::Rows {
            sort,
            desc,
            limit,
            fields,
        } => (sort.as_ref(), *desc, *limit, fields.as_slice()),
        // a count shows no rows (AQ1), and explain none
        BoundView::Counts(_) | BoundView::Cross(_) | BoundView::Explain(_) => {
            (None, false, 0, &[][..])
        }
    };
    let icon = fields.contains(&RowField::Icon);
    let scalars: Option<Vec<Scalar>> = sort.map(|(_, key)| {
        matches
            .iter()
            .map(|(at, _)| eval::scalar(key, &corpus.items[*at]))
            .collect()
    });
    let mut order: Vec<usize> = (0..matches.len()).collect();
    if let Some(scalars) = &scalars {
        order.sort_by(|a, b| match (scalars[*a], scalars[*b]) {
            (Scalar::Value(x), Scalar::Value(y)) => {
                let by = x.cmp(&y);
                if desc { by.reverse() } else { by }
            }
            (Scalar::Value(_), _) => std::cmp::Ordering::Less,
            (_, Scalar::Value(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        });
    }
    let rows: Vec<Row> = order
        .iter()
        .skip(after)
        .take(limit)
        .map(|&m| {
            let (at, outcomes) = &matches[m];
            let held = &corpus.items[*at];
            let mut touched = Vec::new();
            eval::touched(&query.bound, outcomes, true, &mut touched);
            Row {
                id: held.item.facts.id.clone(),
                name: held.item.name.clone(),
                typeline: held.item.typeline.clone(),
                base: held.item.base.clone(),
                rarity: held.item.rarity.clone(),
                icon: icon.then(|| held.item.icon.clone()),
                place: held.place.clone(),
                removed_at: held.item.facts.removed_at,
                removed_at_age: None,
                matched: touched
                    .into_iter()
                    .map(|i| {
                        let term = &query.terms[i];
                        let (shows, left_out) = eval::evidence(&query.terms, term, held, outcomes);
                        Touched {
                            path: term.path.clone(),
                            shows,
                            left_out,
                        }
                    })
                    .filter(|t| !t.shows.is_empty())
                    .collect(),
                sort: scalars.as_ref().zip(sort).map(|(s, (_, key))| {
                    let (shows, left_out) = eval::sorted_by(key, held);
                    let (value, status) = match s[m] {
                        Scalar::Value(n) => (Some(eval::number_json(n.as_f64())), None),
                        Scalar::None => (None, Some("no satisfying occurrence")),
                        Scalar::Incomplete(n) => {
                            (n.map(|n| eval::number_json(n.as_f64())), Some("incomplete"))
                        }
                    };
                    Sorted {
                        value,
                        status,
                        shows,
                        left_out,
                    }
                }),
            }
        })
        .collect();

    // explain shows counts in place of rows, a zero among them: the zero
    // block is the other views'
    let zero = (total.matched == 0 && explained.is_none()).then(|| {
        let resolved_to_nothing: Vec<Nothing> = query
            .terms
            .iter()
            .enumerate()
            .filter_map(|(i, term)| {
                nothing(
                    corpus,
                    term,
                    picked[i],
                    tallies[i][Outcome::Matched as usize],
                )
            })
            .collect();
        let suggested = resolved_to_nothing
            .iter()
            .any(|n| !n.suggestions.is_empty());
        Zero {
            resolved_to_nothing,
            said: if suggested { SUGGESTED_SAID } else { S107 },
            undecided: count(n_undecided, root_undecided.as_ref()),
            offers: offers(&router, &query, &part_matched, request),
        }
    });
    let left_out = total.matched.saturating_sub(after + rows.len());
    let view = match &view {
        BoundView::Rows { .. } => ViewOut::Rows(Box::new(RowsOut {
            limit,
            returned: rows.len(),
            after,
            left_out,
            // a limit of nothing returns nothing from any place: the way
            // on is a larger limit, never a continuation that stands still
            next: (left_out > 0 && limit > 0).then(|| {
                let request = Request {
                    scope: resolved.clone(),
                    query: QueryInput {
                        text: Some(typed.clone()),
                        tree: None,
                        trade: request.query.trade.clone(),
                    },
                    // a continuation goes on as the answer it continues:
                    // the routes asked of it too
                    view: View {
                        shape: Shape::Rows(Rows {
                            limit: Some(limit),
                            sort: sort.map(|(text, _)| text.clone()),
                            desc,
                            after: Some(after + rows.len()),
                            fields: fields.to_vec(),
                        }),
                        routes: request.view.routes,
                    },
                    context: request.context,
                    basis: Some(corpus.basis.clone()),
                };
                Next { request }
            }),
            sort: sort.map(|(text, _)| text.clone()),
            desc,
        })),
        BoundView::Counts(bound) => ViewOut::Counts(counts::tables(bound, &counted)),
        BoundView::Cross(bound) => ViewOut::Cross(counts::crossed(bound, &counted)),
        BoundView::Explain(_) => {
            let Some(Explained {
                node,
                forced: [forced_true, forced_false],
            }) = explained
            else {
                // `BoundView::of` gives an explain view its node or its error
                return Err(SearchError::Language(LanguageError::new(
                    crate::error::ErrorKind::NoSuchPath,
                    "the explained node was not resolved",
                )));
            };
            ViewOut::Explain(Box::new(ExplainOut {
                path: node.path,
                node: node.text,
                given: Given {
                    matched: total.matched,
                    undecided: total.undecided.count,
                },
                forced_true: forced(corpus, &router, request, forced_true)?,
                forced_false: forced(corpus, &router, request, forced_false)?,
            }))
        }
    };
    // the site's search is the typed query's: its report stands with it,
    // and what each ambiguous clause's other reading would change, where
    // that search is the query run (C119)
    let mut trade = translation.map(TradeOut::from);
    if let (Some(report), None) = (&mut trade, &set_aside) {
        for clause in &mut report.clauses {
            if let Some(reading) = &clause.reading {
                clause.other = Some(stake(corpus, &router, &query.root, &truths, reading)?);
            }
        }
    }
    let (trade, set_aside_trade) = match set_aside {
        Some(_) => (None, trade),
        None => (trade, None),
    };
    // the lists the query reads: the league it fixes and the characters',
    // else the realm's — the age the scope line says (C118)
    let tree = json::to_json(&query.root);
    let mut coverage = corpus.coverage.clone();
    if let Some(seen) = fixed_league(&tree).and_then(|league| corpus.list_seen_in(league)) {
        coverage.list_seen = Some(seen);
    }
    let mut answer = Answer {
        before: None,
        query: QueryOut {
            text: query.text(),
            tree,
            trade,
        },
        set_aside: set_aside.map(|typed| QueryOut {
            text: typed.text(),
            tree: json::to_json(&typed.root),
            trade: set_aside_trade,
        }),
        scope: ScopeOut {
            account: Account {
                uuid: corpus.basis.account.clone(),
                name: corpus.account_name.clone(),
            },
            realm: corpus.realm.clone(),
            membership: corpus.membership,
            realm_not_held: match &corpus.realm {
                Realm::One(realm) => !corpus.coverage.realms_held.contains(realm),
                Realm::All => false,
            },
            coverage,
        },
        basis: corpus.basis.clone(),
        terms,
        total,
        view,
        rows,
        zero,
    };
    if !request.view.routes {
        answer.shed_routes();
    }
    Ok(answer)
}

/// What an ambiguous clause's other reading would change (C119): the
/// query with the conjunct it stands in for replaced by it, asked of
/// every item by the one evaluator; an item it matches that the query as
/// run fails is one it adds, one the query matches that it fails one it
/// drops. Each count's route is the query with that conjunct replaced by
/// what tells the two apart — the reading and not the conjunct, or the
/// conjunct and not the reading, the shared part said once where the
/// reading writes the conjunct inside it — which returns exactly the
/// members counted (invariant 4).
fn stake(
    corpus: &Corpus,
    router: &Router<'_>,
    root: &Node,
    truths: &[Truth],
    reading: &Reading,
) -> Result<Stake, SearchError> {
    let defect = |what: &str| {
        SearchError::Language(LanguageError::new(
            crate::error::ErrorKind::Trade,
            format!("the other reading `{}` stands in for {what}", reading.text),
        ))
    };
    let conjunct = conjunct_at(root, &reading.path)
        .ok_or_else(|| defect(&format!("`{}`, no conjunct of the query", reading.path)))?;
    let other = bind::bind(&replaced(root, &reading.path, vec![reading.node.clone()]))?;
    let (mut adds, mut drops) = (0usize, 0usize);
    let mut outcomes = Vec::with_capacity(other.terms.len());
    for (held, truth) in corpus.items.iter().zip(truths) {
        eval::asked(&other.terms, held, &mut outcomes);
        match (eval::truth(&other.bound, &outcomes), truth) {
            (Truth::True, Truth::False) => adds += 1,
            (Truth::False, Truth::True) => drops += 1,
            _ => {}
        }
    }
    let (added, dropped) = parted(conjunct, &reading.node);
    let route = |n: usize, part: Option<Vec<Node>>| match part {
        Some(part) => router.count(n, Some(&replaced(root, &reading.path, part))),
        None => router.count(n, None),
    };
    Ok(Stake {
        reading: reading.text.clone(),
        adds: route(adds, added),
        drops: route(drops, dropped),
    })
}

/// The conjunct of the query at a path: the root's child, or the root
/// itself, `""` or the one path a root that is no and has.
fn conjunct_at<'a>(root: &'a Node, path: &str) -> Option<&'a Node> {
    match root {
        _ if path.is_empty() => Some(root),
        Node::All(children) => children.get(path.parse::<usize>().ok()?),
        other => (path == crate::tree::child_path("", 0)).then_some(other),
    }
}

/// The query with the conjunct at a path replaced by the nodes given,
/// each a conjunct in its place (invariant 7: the grouping alone moves).
fn replaced(root: &Node, path: &str, with: Vec<Node>) -> Node {
    let one = |mut with: Vec<Node>| match with.len() {
        1 => with.remove(0),
        _ => Node::All(with),
    };
    match root {
        Node::All(children) if !path.is_empty() => {
            let at = path.parse::<usize>().unwrap_or(usize::MAX);
            Node::All(
                children
                    .iter()
                    .enumerate()
                    .flat_map(|(n, child)| match n == at {
                        true => with.clone(),
                        false => vec![child.clone()],
                    })
                    .collect(),
            )
        }
        _ => one(with),
    }
}

/// What tells a conjunct and its other reading apart, as the conjuncts
/// that stand in for it: the items the reading adds, then those it drops;
/// none where the reading cannot change it that way — a reading that is
/// the conjunct or more adds alone, one that is the conjunct and more
/// drops alone, and the shared conjunct is said once.
fn parted(conjunct: &Node, reading: &Node) -> (Option<Vec<Node>>, Option<Vec<Node>>) {
    let not = |n: &Node| match n {
        Node::Not(inner) => (**inner).clone(),
        other => Node::Not(Box::new(other.clone())),
    };
    let rest = |members: &[Node], or: bool| {
        let mut rest: Vec<Node> = members.iter().filter(|m| *m != conjunct).cloned().collect();
        match (rest.len(), or) {
            (1, _) => rest.remove(0),
            (_, true) => Node::Any(rest),
            (_, false) => Node::All(rest),
        }
    };
    match reading {
        Node::Any(members) if members.contains(conjunct) => {
            (Some(vec![rest(members, true), not(conjunct)]), None)
        }
        Node::All(members) if members.contains(conjunct) => (
            None,
            Some(vec![conjunct.clone(), not(&rest(members, false))]),
        ),
        _ => (
            Some(vec![reading.clone(), not(conjunct)]),
            Some(vec![conjunct.clone(), not(reading)]),
        ),
    }
}

/// The root's counts of a transformed query: the query bound and answered
/// as any query is, by the one evaluator — this function asks it as the
/// request it is, and reads its total (the module doc, "Explain is a view").
fn forced(
    corpus: &Corpus,
    router: &Router<'_>,
    request: &Request,
    transformed: Node,
) -> Result<Forced, SearchError> {
    let tree = json::to_json(&transformed);
    let asked = answer(
        corpus,
        &Request {
            scope: request.scope.clone(),
            query: QueryInput {
                text: None,
                tree: Some(tree.clone()),
                trade: None,
            },
            view: Shape::Rows(Rows {
                limit: Some(0),
                ..Rows::default()
            })
            .into(),
            context: Context::Matches,
            basis: None,
        },
    )?;
    let undecided = Node::Undecided(Probe::Term(Box::new(transformed.clone())));
    Ok(Forced {
        query: QueryOut {
            text: print::print(&transformed),
            tree,
            trade: None,
        },
        matched: router.count(asked.total.matched, Some(&transformed)),
        undecided: router.count(asked.total.undecided.count, Some(&undecided)),
    })
}

/// What a zero answer offers (the module doc): explain of each part, in
/// the order of what each matched and then of its path, cut after the
/// order; then the corpus context beside the view the request had —
/// rows, where it had explain, which the empty query has no path for.
fn offers(router: &Router<'_>, query: &Query, part_matched: &[usize], request: &Request) -> Offers {
    let nodes: Vec<&Node> = match &query.root {
        Node::All(children) => children.iter().collect(),
        root => vec![root],
    };
    let mut parts: Vec<(usize, usize, &Node)> = nodes
        .into_iter()
        .enumerate()
        .map(|(n, node)| (part_matched.get(n).copied().unwrap_or(0), n, node))
        .collect();
    parts.sort_by_key(|(matched, n, _)| (*matched, *n));
    let (parts, explain_left_out) = bounded(parts, None, false);
    Offers {
        explain: parts
            .into_iter()
            .map(|(_, n, node)| {
                let path = crate::tree::child_path("", n);
                ExplainOffer {
                    node: print::print(node),
                    request: router.request(
                        router.root,
                        None,
                        Shape::Explain(Explain { path: path.clone() }).into(),
                        Context::Matches,
                    ),
                    path,
                }
            })
            .collect(),
        explain_left_out,
        context: (!is_empty(&query.root)).then(|| Offer {
            request: router.request(
                router.root,
                None,
                View {
                    shape: match &request.view.shape {
                        Shape::Explain(_) => Shape::default(),
                        // a place is a place in the answer continued, never
                        // in the corpus's
                        Shape::Rows(rows) => Shape::Rows(Rows {
                            after: None,
                            ..rows.clone()
                        }),
                        shape => shape.clone(),
                    },
                    routes: request.view.routes,
                },
                Context::Corpus,
            ),
        }),
    }
}

fn account_is(corpus: &Corpus, account: &str) -> bool {
    let name = corpus.account_name.as_deref().unwrap_or_default();
    account.eq_ignore_ascii_case(&corpus.basis.account)
        || account.eq_ignore_ascii_case(name)
        // a username may carry its discriminator: `name#1234`
        || name.split('#').next().is_some_and(|n| account.eq_ignore_ascii_case(n))
        || account.split('#').next().is_some_and(|a| a.eq_ignore_ascii_case(name))
}

/// What an item is called where a whole row is too much.
fn label(held: &Held) -> Option<String> {
    match (&held.item.name, &held.item.typeline) {
        (Some(name), Some(typeline)) => Some(format!("{name} {typeline}")),
        (name, typeline) => name.clone().or_else(|| typeline.clone()),
    }
}

// ---- routes ------------------------------------------------------------------------------------------

/// The one maker of a route (invariant 4): a request over the answer's
/// scope. The basis it was counted at is the answer's, stated once.
pub(crate) struct Router<'a> {
    scope: Scope,
    /// The query answered: what a bucket's route is a request under.
    root: &'a Node,
}

impl Router<'_> {
    /// A request over the answer's scope: what a route carries, and what
    /// a zero answer offers.
    fn request(&self, node: &Node, realm: Option<&str>, view: View, context: Context) -> Request {
        let mut scope = self.scope.clone();
        if let Some(realm) = realm {
            scope.realm = Some(Realm::One(realm.to_string()));
        }
        Request {
            scope,
            query: QueryInput {
                text: Some(print::print(node)),
                tree: None,
                trade: None,
            },
            view,
            context,
            basis: None,
        }
    }

    fn route(&self, node: &Node, realm: Option<&str>) -> Route {
        Route {
            denominator: "scope",
            request: self.request(node, realm, View::default(), Context::Matches),
        }
    }

    /// The count of `node`'s matches by `key`, over the scope.
    fn continuation(&self, node: &Node, key: String) -> Route {
        let mut route = self.route(node, None);
        route.request.view = View::from(Shape::Counts(Counts {
            keys: vec![key],
            sum: None,
            limit: None,
        }));
        route
    }

    /// A count over the scope, with its route when it is not zero.
    pub(crate) fn count(&self, n: usize, node: Option<&Node>) -> Count {
        Count {
            count: n,
            route: node.filter(|_| n > 0).map(|node| self.route(node, None)),
        }
    }

    /// A count of the answer's matches: its route is the query and the
    /// terms that select it — the old query, parenthesised, and new terms
    /// (C91) — never a fragment, and under one realm where a vocabulary row
    /// is one realm's (C97).
    pub(crate) fn under(&self, n: usize, terms: Vec<Node>, realm: Option<&str>) -> Count {
        let mut all = match self.root {
            Node::All(children) if children.is_empty() => Vec::new(),
            root => vec![root.clone()],
        };
        all.extend(terms);
        let node = if all.len() == 1 {
            all.remove(0)
        } else {
            Node::All(all)
        };
        Count {
            count: n,
            route: (n > 0).then(|| self.route(&node, realm)),
        }
    }
}

struct Routes {
    failed: Option<Node>,
    lacked: Option<Node>,
    undecided: Node,
    together: Option<(Node, group::LowerBound)>,
}

fn not(node: Node) -> Node {
    Node::Not(Box::new(node))
}

fn routes(term: &Term) -> Routes {
    let node = term.node.clone();
    let undecided = Node::Undecided(Probe::Term(Box::new(node.clone())));
    let (failed, lacked, together) = match (&term.atom, &term.node) {
        (Atom::Lines(group), _) if group.selects_only => (None, Some(not(node.clone())), None),
        (Atom::Lines(group), _) => {
            let selected = Node::Members {
                of: Collection::Lines,
                where_: Box::new(group.selector_tree.clone()),
            };
            // a selector that asks a flag may be open on an occurrence whose
            // flags are unread: such an item did not lack the line, so the
            // failed route admits it, and stays as short as the worked example's
            // wherever no flag is asked
            let picked = if group.selector_asks_a_flag {
                Node::Any(vec![
                    selected.clone(),
                    Node::Undecided(Probe::Term(Box::new(selected.clone()))),
                ])
            } else {
                selected.clone()
            };
            let failed = Node::All(vec![picked, not(node.clone())]);
            let together = group.together.clone().map(|lower| {
                let sum = Node::Compare {
                    value: ValueRef::Sum {
                        lines: Box::new(group.selector_tree.clone()),
                        slot: lower.slot.clone(),
                    },
                    op: lower.op,
                    rhs: Value::Number(lower.bound),
                };
                (
                    Node::All(vec![selected.clone(), not(node.clone()), sum]),
                    lower,
                )
            });
            (Some(failed), Some(not(selected)), together)
        }
        (Atom::Has(_) | Atom::HasComputed(_), _) => (None, Some(not(node.clone())), None),
        // a category term failed on an item that has a category or whose
        // category is open (`category.rs`): what it lacked is `-has:` alone
        (
            Atom::Closed {
                thing: Thing::Category,
                ..
            },
            Node::Test { field, .. },
        ) => {
            let has = Node::Has(field.clone());
            let had_or_open = Node::Any(vec![
                has.clone(),
                Node::Undecided(Probe::Thing(ValueRef::Field(field.clone()))),
            ]);
            (
                Some(Node::All(vec![had_or_open, not(node.clone())])),
                Some(not(has)),
                None,
            )
        }
        // a class never lacks (`class.rs`), and `has:class` is open on an
        // item whose base is under several classes while `class:X` on it
        // may be false (F2): the failed route is the not alone
        (
            Atom::Closed {
                thing: Thing::Class,
                ..
            },
            _,
        ) => (Some(not(node.clone())), None, None),
        // a link group's member is one of the item's link groups: an item
        // with none lacks it
        (Atom::Links(_), _) => {
            let has = Node::Has("links".to_string());
            (
                Some(Node::All(vec![has.clone(), not(node.clone())])),
                Some(not(has)),
                None,
            )
        }
        // a computed value lacked is `-has:` of it: a derived field (T2),
        // a total of nothing (C94) — of the value, never of a number of
        // it: `has:` takes no slot word, and a ranged total asked by one
        // is lacked where the total is (J31)
        (Atom::Pseudo { name, slot, .. }, _) => {
            let whole = slot
                .as_deref()
                .and_then(|slot| name.strip_suffix(slot)?.strip_suffix('.'))
                .unwrap_or(name);
            let has = Node::Has(whole.to_string());
            (
                Some(Node::All(vec![has.clone(), not(node.clone())])),
                Some(not(has)),
                None,
            )
        }
        (
            Atom::Text { thing, .. } | Atom::Closed { thing, .. } | Atom::Number { thing, .. },
            Node::Test { field, .. },
        ) if *thing != Thing::Text => {
            let has = Node::Has(field.clone());
            (
                Some(Node::All(vec![has.clone(), not(node.clone())])),
                Some(not(has)),
                None,
            )
        }
        _ => (Some(not(node.clone())), None, None),
    };
    Routes {
        failed,
        lacked,
        undecided,
        together,
    }
}

// ---- what a selector resolved to ------------------------------------------------------------

/// What a term's `:` or `~` selector ranges over, when it has one: a
/// field, or the templates of a line's group — a `sum`'s group too.
/// Whether every template test of the term's group is a quoted `"T"`. Such
/// a term resolves to itself and lists nothing — unless it found more than
/// one spelling, which any-case `=` can (owner, 2026-09-20: six pairs of
/// the census's templates differ only by capitals), and then it says so.
fn exact_only(term: &Term) -> bool {
    match &term.atom {
        Atom::Lines(group) | Atom::Sum { group, .. } => group.quoted_only(),
        _ => false,
    }
}

fn resolves(term: &Term) -> Option<String> {
    // a quoted template resolves too: `=` is any-case, and GGG has
    // spelled some lines two ways (`exact_only`, above)
    if let Atom::Lines(group) | Atom::Sum { group, .. } = &term.atom {
        return group.names_a_template().then(|| "template".to_string());
    }
    match &term.node {
        Node::Test {
            field,
            op: Op::Contains | Op::Match,
            ..
        } if !matches!(
            &term.atom,
            Atom::Text {
                thing: Thing::Text,
                ..
            } | Atom::Id(_)
        ) =>
        {
            Some(field.clone())
        }
        _ => None,
    }
}

/// The count that lists everything a term's selector resolved to: the
/// term alone — a group's selector alone — counted by its field, or the
/// vocabulary narrowed by the group's one template test. None for `tab`
/// (E1: counted by the tab, resolved to names) and for a group whose
/// selector is more than one template test.
fn rest_of(term: &Term) -> Option<(Node, String)> {
    match &term.atom {
        Atom::Lines(group) | Atom::Sum { group, .. } => {
            let (op, text) = group.sole_template_test()?;
            let key = match op {
                Op::Match => format!("line~{text}"),
                _ => format!("line:{text}"),
            };
            Some((
                Node::Members {
                    of: Collection::Lines,
                    where_: Box::new(group.selector_tree.clone()),
                },
                key,
            ))
        }
        // no count lists every id a category term picked: a count by
        // category lists an item under its most specific id alone
        Atom::Closed {
            thing: Thing::Category,
            ..
        } => None,
        Atom::Text { thing, .. } | Atom::Closed { thing, .. } if *thing != Thing::Tab => {
            match &term.node {
                Node::Test { field, .. } => Some((term.node.clone(), field.clone())),
                _ => None,
            }
        }
        _ => None,
    }
}

/// What the term's selector resolved to on this item. A group's is what
/// its selector picks, whatever its comparisons then make of it: `20%
/// to Fire Resistance` is a line `template:resistance arg1>=60` resolved
/// to, and its value failed — by its whole text, or by the name a quoted
/// template found it by, a row's among them (`Group::resolved`). A
/// field's is the value that matched.
fn resolved_values(term: &Term, held: &Held, outcome: Outcome) -> Vec<String> {
    let mut values: Vec<String> = match &term.atom {
        Atom::Lines(group) | Atom::Sum { group, .. } => eval::selected(held, group)
            .flat_map(|l| group.resolved(l))
            .map(str::to_string)
            .collect(),
        Atom::Text { thing, test } if outcome == Outcome::Matched => eval::texts(held, *thing)
            .into_iter()
            .filter(|v| test.holds(v))
            .map(str::to_string)
            .collect(),
        // the picked ids that hold on the item (`category.rs`)
        Atom::Closed {
            thing: Thing::Category,
            ids,
            ..
        } if outcome == Outcome::Matched => crate::category::tables()
            .map(|tables| tables.holding(&held.category, *ids))
            .unwrap_or_default()
            .into_iter()
            .map(str::to_string)
            .collect(),
        // what the closed set's selector picked of the item's values — an
        // item that is each of two classes carries one the term did not
        // pick (the rule of 2026-09-26; `class.rs`)
        Atom::Closed { thing, values, .. } if outcome == Outcome::Matched => {
            eval::texts(held, *thing)
                .into_iter()
                .filter(|v| values.iter().any(|legal| legal.eq_ignore_ascii_case(v)))
                .map(str::to_string)
                .collect()
        }
        _ => Vec::new(),
    };
    values.sort();
    values.dedup();
    values
}

// ---- a selector nothing carries ------------------------------------------------------------------

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.chars().count() >= 3)
        .map(str::to_lowercase)
        .collect()
}

/// A term whose selector nothing in scope carries, with the values
/// sharing its words. Exact execution, tolerant suggestion: nothing here
/// is matched for the author (S107). A group is read as the rest of the
/// answer reads it — by its bound selector, a `sum`'s too — so one that
/// picked any occurrence resolved to something, whatever a template test
/// inside it would find alone; a field's term carried nothing when no item
/// matched it. A pattern that resolved to nothing is listed like any other
/// and has no words to suggest by.
fn nothing(corpus: &Corpus, term: &Term, picked: bool, matched: usize) -> Option<Nothing> {
    // what the selector ranges over; the words a suggestion is scored
    // against, which a pattern has none of and is listed all the same; and
    // the field its values are read from, a group's being its templates
    let (of, wanted, thing): (String, Option<String>, Option<Thing>) =
        match (&term.node, &term.atom) {
            (_, Atom::Lines(group) | Atom::Sum { group, .. })
                if !picked && group.names_a_template() =>
            {
                ("template".to_string(), group.wanted(), None)
            }
            (
                Node::Test {
                    field, op, value, ..
                },
                Atom::Text { thing, .. },
            ) if *thing != Thing::Text && matched == 0 => {
                let wanted = match (op, value) {
                    (Op::Contains | Op::Eq, Value::Text(wanted)) => Some(wanted.clone()),
                    _ => None,
                };
                (field.clone(), wanted, Some(*thing))
            }
            // a closed set's `:` or `~` picks among legal values, and the
            // answer says what it resolved to: nothing, here
            (
                Node::Test {
                    field,
                    op: Op::Contains | Op::Match,
                    ..
                },
                Atom::Closed { thing, .. },
            ) if matched == 0 => (field.clone(), None, Some(*thing)),
            // an id is whole, and one that matched nothing says so (F7):
            // no value shares words with it
            (_, Atom::Id(_)) if matched == 0 => {
                return Some(Nothing {
                    path: term.path.clone(),
                    term: print::print(&term.node),
                    of: "id".to_string(),
                    suggestions: Vec::new(),
                    // an id that left is whole, and the term says where
                    note: match &term.atom {
                        Atom::Id(id) if corpus.gone.contains_key(id) => None,
                        _ => Some(ID_WHOLE),
                    },
                });
            }
            _ => return None,
        };
    let values_of = |held: &'_ Held| -> Vec<String> {
        let mut values: Vec<String> = match thing {
            Some(thing) => eval::texts(held, thing)
                .into_iter()
                .map(str::to_string)
                .collect(),
            None => held.item.lines.iter().map(|l| l.template.clone()).collect(),
        };
        values.sort_unstable();
        values.dedup();
        values
    };
    let mut suggestions: Vec<Suggestion> = Vec::new();
    if let Some(wanted) = wanted {
        let mut carried: HashMap<String, usize> = HashMap::new();
        for held in &corpus.items {
            for value in values_of(held) {
                *carried.entry(value).or_default() += 1;
            }
        }
        let wanted_words = words(&wanted);
        let mut scored: Vec<(usize, Carried)> = carried
            .into_iter()
            .filter_map(|(value, items)| {
                let has = words(&value);
                // a plural typed for a singular shares the word too
                let shared = wanted_words
                    .iter()
                    .filter(|w| has.iter().any(|h| h.contains(*w) || w.contains(h)))
                    .count();
                (shared > 0).then_some((shared, Carried { value, items }))
            })
            .collect();
        scored.sort_by(|(sa, a), (sb, b)| {
            sb.cmp(sa)
                .then_with(|| b.items.cmp(&a.items))
                .then_with(|| a.value.cmp(&b.value))
        });
        // a suggestion's count is its term's (invariant 4): the term is an
        // any-case `=`, so it is asked of the scope as it will be when
        // typed, and a second spelling of a value already offered is the
        // same term and is not offered again
        let mut offered: Vec<bind::TextTest> = Vec::new();
        for (_, candidate) in scored {
            if suggestions.len() == SUGGESTED {
                break;
            }
            if offered.iter().any(|test| test.holds(&candidate.value)) {
                continue;
            }
            let exact = Value::Text(candidate.value.clone());
            let Ok(test) = bind::text_test(&of, Op::Eq, &exact) else {
                continue;
            };
            // a template's term names a mod by a row too (C90)
            let carries = |held: &Held| match thing {
                Some(_) => values_of(held).iter().any(|v| test.holds(v)),
                None => held
                    .item
                    .lines
                    .iter()
                    .any(|l| l.names().any(|name| test.holds(name))),
            };
            suggestions.push(Suggestion {
                term: print::print(&exactly(&of, &candidate.value)),
                value: candidate.value,
                items: corpus.items.iter().filter(|held| carries(held)).count(),
            });
            offered.push(test);
        }
    }
    Some(Nothing {
        path: term.path.clone(),
        term: print::print(&term.node),
        suggestions,
        of,
        note: None,
    })
}

/// The exact term that selects one value: `line("T")`, `base="…"`.
fn exactly(of: &str, value: &str) -> Node {
    if of == "template" {
        return group::line_of(value);
    }
    Node::Test {
        field: of.to_string(),
        op: Op::Eq,
        value: Value::Text(value.to_string()),
    }
}

/// A command a terminal takes: the one way the search prints one (the
/// build plan's rule 5 — a command printed is a command that runs). It
/// names the account it was answered for, since a second account makes a
/// command without one an error, and a last word that starts with `-` —
/// the language's not, a terminal's flag — goes after `--`, or `-has:note`
/// is read as `-h`.
pub fn command(verb: &str, account: Option<&str>, realm: Option<&str>, last: &str) -> String {
    command_with(verb, account, realm, last, "")
}

/// [`command`] with the flags a view and a context add: after the query,
/// as a reader types them — and before it where the query goes after
/// `--`, past which a terminal reads no flag.
fn command_with(
    verb: &str,
    account: Option<&str>,
    realm: Option<&str>,
    last: &str,
    flags: &str,
) -> String {
    let mut out = opening(verb, account, realm);
    if last.starts_with('-') {
        out.push_str(flags);
        out.push_str(" -- ");
        out.push_str(&shell_quoted(last));
    } else {
        out.push(' ');
        out.push_str(&shell_quoted(last));
        out.push_str(flags);
    }
    out
}

/// Where an answer's scope points for the tabs it never fetched (C118):
/// `tabs` over one league, as a command a terminal takes, and how many
/// other leagues hold a never-fetched tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsPointer {
    pub command: String,
    pub other_leagues: usize,
}

impl Answer {
    /// The `tabs` command for the tabs this answer's scope never fetched,
    /// a function of the answer's JSON: under one realm, the league the
    /// query fixes where it holds a never-fetched tab, or, where the query
    /// fixes none, the league holding the most; none where no league
    /// holds one. Made by the one opening every command has.
    pub fn tabs_pointer(&self) -> Option<TabsPointer> {
        let Realm::One(realm) = &self.scope.realm else {
            return None;
        };
        let leagues = &self.scope.coverage.never_fetched_leagues;
        let (league, other_leagues) = match fixed_league(&self.query.tree) {
            // `=` is any-case: the command carries the store's spelling
            Some(asked) => (
                leagues
                    .iter()
                    .find(|l| bind::folded(l) == bind::folded(asked))?,
                0,
            ),
            None => (leagues.first()?, leagues.len() - 1),
        };
        let mut command = opening(
            "tabs",
            Some(
                self.scope
                    .account
                    .name
                    .as_deref()
                    .unwrap_or(&self.scope.account.uuid),
            ),
            Some(realm),
        );
        command.push_str(&format!(
            " --league {} --never-fetched",
            shell_quoted(league)
        ));
        Some(TabsPointer {
            command,
            other_leagues,
        })
    }
}

/// The one league a query's tree fixes: the one `league=` term among the
/// root and its and-ed descendants, through every nested conjunction — a
/// refinement parenthesizes the old text and appends a term, and means
/// the same (review 2 of the re-sit: `(league=Standard rarity=rare)
/// ilvl>=1` had lost its league's list age).
fn fixed_league(tree: &Json) -> Option<&str> {
    fn conjoined<'a>(t: &'a Json, found: &mut Vec<&'a str>) {
        if t["field"] == "league" && t["op"] == "=" {
            found.extend(t["value"].as_str());
        }
        if let Some(all) = t["all"].as_array() {
            all.iter().for_each(|child| conjoined(child, found));
        }
    }
    let mut found = Vec::new();
    conjoined(tree, &mut found);
    match found.as_slice() {
        [one] => Some(one),
        _ => None,
    }
}

/// How every command the search prints opens — `show`'s `price show`
/// pointer too: the verb, the account it was answered for, the realm.
pub fn opening(verb: &str, account: Option<&str>, realm: Option<&str>) -> String {
    let mut out = format!("acq {verb}");
    if let Some(account) = account {
        out.push_str(&format!(" --account {}", shell_quoted(account)));
    }
    if let Some(realm) = realm {
        out.push_str(&format!(" --realm {realm}"));
    }
    out
}

impl Next {
    /// The continuation as a command a terminal takes: the token whole,
    /// and the account it was answered for (`command`).
    pub fn command(&self) -> String {
        // the realm is the token's to carry
        let mut out = opening("search", self.request.scope.account.as_deref(), None);
        out.push_str(&format!(" --next {}", shell_quoted(&self.request.token())));
        out
    }
}

impl Route {
    /// The route as a command a terminal takes ([`Request::command`]).
    pub fn command(&self) -> String {
        self.request.command()
    }
}

impl Request {
    /// The request as a command a terminal takes: the query, then the
    /// view where it is not the default rows, then the context where it
    /// is not the matches. A request that carries a tree and no text has
    /// none of its own: an answer returns the text. One that carries a
    /// site search is `--trade` and that search, and no realm where its
    /// URL names one: `--realm` beside a URL is a conflict (K6).
    pub fn command(&self) -> String {
        let scope = &self.scope;
        let mut out = String::new();
        if scope.membership != Membership::Live {
            out.push_str(" --membership ");
            out.push_str(scope.membership.as_str());
        }
        self.view_flags(&mut out);
        if self.context == Context::Corpus {
            out.push_str(" --context corpus");
        }
        if let Some(word) = self.query.trade_word() {
            // the realm the site search names by itself, read where every
            // reader reads it
            let url = self.query.read(None).ok().and_then(|read| read.realm);
            let mut command = opening(
                "search",
                scope.account.as_deref(),
                match url {
                    Some(_) => None,
                    None => scope.realm.as_ref().map(Realm::as_str),
                },
            );
            command.push_str(&format!(" --trade {}", shell_quoted(&word)));
            command.push_str(&out);
            return command;
        }
        command_with(
            "search",
            scope.account.as_deref(),
            scope.realm.as_ref().map(Realm::as_str),
            self.query.text.as_deref().unwrap_or_default(),
            &out,
        )
    }

    fn view_flags(&self, out: &mut String) {
        self.shape_flags(out);
        if self.view.routes {
            out.push_str(" --routes");
        }
    }

    fn shape_flags(&self, out: &mut String) {
        let (flag, counts) = match &self.view.shape {
            Shape::Rows(rows) => {
                if let Some(sort) = &rows.sort {
                    out.push_str(&format!(" --sort {}", shell_quoted(sort)));
                    if rows.desc {
                        out.push_str(" --desc");
                    }
                }
                if let Some(limit) = rows.limit {
                    out.push_str(&format!(" --limit {limit}"));
                }
                return;
            }
            Shape::Explain(explain) => {
                out.push_str(&format!(" --explain {}", shell_quoted(&explain.path)));
                return;
            }
            Shape::Counts(c) => ("--count", c),
            Shape::Cross(c) => ("--cross", c),
        };
        for list in counts.lists() {
            out.push_str(&format!(" {flag} {}", shell_quoted(&list)));
        }
        if let Some(sum) = &counts.sum {
            out.push_str(&format!(" --sum {}", shell_quoted(sum)));
        }
        if let Some(limit) = counts.limit {
            out.push_str(&format!(" --limit {limit}"));
        }
    }
}

impl Answer {
    /// The routes an answer carries unasked (C115): the module doc, "The
    /// routes an answer carries". Every other count keeps its number and
    /// drops its request.
    fn shed_routes(&mut self) {
        let matched_something = self.total.matched > 0;
        for term in &mut self.terms {
            if matched_something {
                term.matched.route = None;
            }
            term.failed.route = None;
            term.lacked.route = None;
        }
        // `none` and `undecided` are what a key could not place; every
        // other bucket — a value, a computed value — is placed
        let placed = |label: &counts::Label| !matches!(label.bucket, "none" | "undecided");
        match &mut self.view {
            ViewOut::Counts(counts) => {
                for bucket in counts.tables.iter_mut().flat_map(|t| &mut t.buckets) {
                    if placed(&bucket.label) {
                        bucket.count.route = None;
                    }
                    for kind in bucket.sources.iter_mut().chain(&mut bucket.flags) {
                        kind.count.route = None;
                    }
                }
            }
            ViewOut::Cross(cross) => {
                for cell in &mut cross.cells {
                    if cell.of.iter().all(placed) {
                        cell.count.route = None;
                    }
                }
            }
            ViewOut::Rows(_) | ViewOut::Explain(_) => {}
        }
    }

    /// The command that shows one of this answer's items whole: where a
    /// bounded block of a row sends its reader (invariant 5).
    pub fn show_command(&self, id: &str) -> String {
        let account = &self.scope.account;
        command(
            "show",
            Some(account.name.as_deref().unwrap_or(&account.uuid)),
            None,
            id,
        )
    }
}

impl Counts {
    /// The keys as the lists a terminal takes, one for each `--count` (or
    /// the one `--cross`): what a printed command spells, before the
    /// shell's own quoting. Each key is read by the one reader of a key
    /// (`bind::bind_key`; rule 10) and spelled from what it read, never
    /// from the key's own letters — so a prefix in another case, or a
    /// text that looks like a key, is spelled as what it is. A narrowing
    /// stands alone in its list, `line:<text>` or `line~<pattern>`, the
    /// text quoted in the list's grammar where the list would read part
    /// of it as syntax: in a list of several, the rest after `line:` are
    /// texts, which a field could never follow and a text that begins
    /// `line:` could never be told from. Fields, and `line` alone, run
    /// together in one list until a narrowing ends it. (Outside audit,
    /// 2026-10-10, twice: a spelling made case by case kept one case
    /// behind.)
    pub fn lists(&self) -> Vec<String> {
        let mut lists: Vec<String> = Vec::new();
        let mut run: Vec<String> = Vec::new();
        for key in &self.keys {
            let alone = match bind::bind_key(key) {
                Ok(bind::Key::Field(def)) => {
                    run.push(def.name.to_string());
                    continue;
                }
                Ok(bind::Key::Line(None)) => {
                    run.push("line".to_string());
                    continue;
                }
                Ok(bind::Key::Line(Some((Op::Match, pattern)))) => {
                    format!("line~{}", listed_text(&pattern))
                }
                Ok(bind::Key::Line(Some((_, words)))) => {
                    format!("line:{}", listed_text(&words))
                }
                // a key the binder refuses is in no answer's request: as
                // written, alone, for the refusal to name
                Err(_) => key.clone(),
            };
            if !run.is_empty() {
                lists.push(std::mem::take(&mut run).join(","));
            }
            lists.push(alone);
        }
        if !run.is_empty() {
            lists.push(run.join(","));
        }
        lists
    }
}

/// A text of the count list, quoted in the list's grammar when the list
/// would read part of it as syntax, with the language's three escapes.
fn listed_text(text: &str) -> String {
    let plain = !text.is_empty()
        && !text.contains([',', '"', '\\', '\n'])
        && !text.starts_with('~')
        && text.trim() == text;
    if plain {
        return text.to_string();
    }
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// One shell word, in single quotes when it needs them; an apostrophe
/// inside is `'\''` (the build plan, gap 2).
pub fn shell_quoted(text: &str) -> String {
    let plain = !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_./@:=".contains(c));
    if plain {
        text.to_string()
    } else {
        format!("'{}'", text.replace('\'', "'\\''"))
    }
}
