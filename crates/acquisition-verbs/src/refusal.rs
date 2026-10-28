//! One refusal shape on every surface (C11, the failure document; C149,
//! every refusal one JSON object; round 4 of
//! `roadmap/briefs/2g-build.md`, the shape its design's Q5 delegated):
//! [`document`] is the one function every frontend's refusal passes
//! through — the MCP's tool refusals, the CLI's `--json` failure, the web
//! page's — so no surface sorts a refusal alone.
//!
//! The document is `error`, the words, and `kind`, one word from the
//! closed lists as they stand: the protocol's (C85), the layer's
//! [`Fault::kind`], the planner's [`Refused`], the search's
//! ([`SearchError::kind`]), and `arguments` and `refused` for what none
//! of them names; no new word (C121, kept). Beside them at most four
//! optional keys, each meaning one thing wherever it stands, and absent,
//! never null, when it has nothing to say:
//!
//! - `offers`: requests the reader sends back as printed, each
//!   `{request, says}`, naming `tool` where it is another tool's. An
//!   offer is made from what the refusal carries — the names known for an
//!   account that cannot be picked, the target and the scope a `not_found`
//!   was looked for in ([`Missing`]), a search's own ([`Offered`]) — never
//!   by reading scope again; the surface spells it ([`Surface::request`]).
//! - `readings`: how what was typed could be read, payload on every
//!   refusal.
//! - `next`: one sentence or command that is a person's act, never an
//!   agent's.
//! - the kind's own fields: `accounts` (C121); `revision`, with
//!   `current` beside it where a live row stands (a `revision: null`
//!   that stands today keeps its contract: C11); a language error's
//!   `span`.

use acquisition_client::client::DaemonError;
use acquisition_plan::PutPolicyError;
use acquisition_plan::pricing::{Missing, Refused, RefusedKind};
use acquisition_protocol::realm::Realm;
use acquisition_search::answer::{Request, Scope};
use acquisition_search::error::Offered;
use acquisition_search::{SearchError, corpus};
use acquisition_store::AnnotationError;
use serde_json::{Map, Value, json};

use crate::fault::Fault;

/// A way on from a refusal, in the shared words: what the request is,
/// before a surface spells it. Matched exhaustively by each surface, so
/// a way added here cannot be offered unspelled.
#[derive(Debug, Clone, PartialEq)]
pub enum Way {
    /// The refused request again, its account named `account`.
    Again { account: String },
    /// A search, as the search tool and `acq search --request` read one.
    Search(Box<Request>),
    /// The tabs of one league.
    Tabs {
        account: String,
        realm: Realm,
        league: String,
    },
    /// One price target in one league.
    PriceShow {
        target: String,
        account: String,
        realm: Realm,
        league: String,
    },
    /// One league's pricing state, every row listed.
    PriceStatus {
        account: String,
        realm: Realm,
        league: String,
    },
}

/// What a surface adds to the one shape: its words, the remedy in its
/// own terms (C143: the shared words first, its spelling after them),
/// and how it sends a [`Way`] back.
pub trait Surface {
    /// The failure's words on this surface, for a refusal that is not the
    /// search's (whose words are its message, the same on every surface).
    fn words(&self, e: &anyhow::Error) -> String;

    /// `way` as this surface sends it back: the request, and the tool
    /// it goes to where that is another tool than the one refused; none
    /// where this surface holds no request to send (a page has no
    /// command line to repeat).
    fn request(&self, way: &Way) -> Option<(Value, Option<&'static str>)>;

    /// A search's way on as this surface reads it: the shared reading,
    /// and this surface's spelling after it.
    fn reading(&self, offered: &Offered) -> String {
        offered.reading.clone()
    }

    /// Whether the request this surface repeats as another account is a
    /// continuation, whose place it drops ([`again`]): the offer then says
    /// it is the request from its start.
    fn repeats_from_its_start(&self) -> bool {
        false
    }
}

/// A search request read whole, as sent — a request, or the object that
/// holds one (a route with its `count` and `counted_at`, an offer) — as
/// `account`'s: the envelope as read with only the nested
/// `scope.account` changed, so what it holds beside the request stands
/// (round 4's review, finding 3, and its fix round's, finding 1, on both
/// surfaces). A continuation's place is another store's basis, so it is
/// dropped — `basis` and the rows' `after` — and the second value says
/// so: the request from its start (the fix round's finding 2). None for
/// what is no request `search` reads.
pub fn again(envelope: &Value, account: &str) -> Option<(Value, bool)> {
    acquisition_search::answer::Asked::read(envelope).ok()?;
    let mut envelope = envelope.clone();
    let request = match envelope.get("request").is_some() {
        true => envelope.get_mut("request")?,
        false => &mut envelope,
    };
    let request = request.as_object_mut()?;
    let scope = request
        .entry("scope")
        .or_insert_with(|| json!({}))
        .as_object_mut()?;
    scope.insert("account".into(), json!(account));
    let mut restarted = request.remove("basis").is_some();
    if let Some(rows) = request
        .get_mut("view")
        .and_then(|view| view.get_mut("rows"))
        .and_then(Value::as_object_mut)
    {
        restarted |= rows.remove("after").is_some();
    }
    Some((envelope, restarted))
}

/// The refusal `e` as `surface` prints it: the one shape.
pub fn document(e: &anyhow::Error, surface: &dyn Surface) -> Value {
    if let Some(search) = e.chain().find_map(|c| c.downcast_ref::<SearchError>()) {
        return of_search(search, surface);
    }
    let mut doc = Map::new();
    let mut offers = Vec::new();
    doc.insert("error".into(), json!(surface.words(e)));
    if let Some(fault) = e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        faulted(
            fault,
            surface.repeats_from_its_start(),
            &mut doc,
            &mut offers,
        );
    } else if let Some(refused) = Refused::find(e) {
        doc.extend(refused.fields());
        if let RefusedKind::NotFound(missing) = &refused.kind {
            offers.extend(finding(missing));
        }
    } else if let Some(refused) = DaemonError::find(e) {
        doc.insert("kind".into(), json!(refused.kind));
    } else if e.chain().any(|c| c.downcast_ref::<Arguments>().is_some()) {
        doc.insert("kind".into(), json!("arguments"));
    } else {
        doc.insert("kind".into(), json!("refused"));
    }
    offered(doc, offers, surface)
}

/// A search's refusal as `surface` prints it: the one shape, its words
/// the search's message on every surface.
pub fn of_search(e: &SearchError, surface: &dyn Surface) -> Value {
    let mut doc = Map::new();
    let mut offers = Vec::new();
    searched(e, surface, &mut doc, &mut offers);
    offered(doc, offers, surface)
}

/// `doc` with its `offers`, each spelled by `surface`, `{request, says}`
/// and `tool` where another tool's; the key absent where none is sent.
fn offered(
    mut doc: Map<String, Value>,
    offers: Vec<(Way, String)>,
    surface: &dyn Surface,
) -> Value {
    let sent: Vec<Value> = offers
        .into_iter()
        .filter_map(|(way, says)| {
            let (request, tool) = surface.request(&way)?;
            let mut offer = Map::new();
            offer.insert("request".into(), request);
            offer.insert("says".into(), json!(says));
            if let Some(tool) = tool {
                offer.insert("tool".into(), json!(tool));
            }
            Some(Value::Object(offer))
        })
        .collect();
    if !sent.is_empty() {
        doc.insert("offers".into(), Value::Array(sent));
    }
    Value::Object(doc)
}

/// Arguments a tool cannot read: refused as `arguments`, the MCP's word
/// for a parameter it does not take (the CLI's parser refuses a command
/// line as `usage` before any verb runs).
#[derive(Debug)]
pub struct Arguments(pub String);

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Arguments {}

/// A search's refusal: its message, its kind, its readings (payload: a
/// language error's are query texts, a scope refusal's its offers'
/// readings), a language error's `span`, and each offer that is a request.
fn searched(
    e: &SearchError,
    surface: &dyn Surface,
    doc: &mut Map<String, Value>,
    offers: &mut Vec<(Way, String)>,
) {
    let shared = e.to_json();
    doc.insert("error".into(), shared["error"].clone());
    doc.insert("kind".into(), shared["kind"].clone());
    match e {
        SearchError::Language(language) => {
            if !language.readings.is_empty() {
                doc.insert("readings".into(), json!(language.readings));
            }
            if let Some(span) = shared.get("span") {
                doc.insert("span".into(), span.clone());
            }
        }
        SearchError::Scope { offers: ways, .. } => {
            if !ways.is_empty() {
                let read: Vec<String> = ways.iter().map(|o| surface.reading(o)).collect();
                doc.insert("readings".into(), json!(read));
            }
            offers.extend(ways.iter().filter_map(|o| {
                let request = o.request.clone()?;
                Some((Way::Search(Box::new(request)), surface.reading(o)))
            }));
        }
        SearchError::Store(_) => {}
    }
}

/// A layer fault's kind and fields (C121, C45), the offers an account
/// that cannot be picked has — the refused request as each name known —
/// and a policy write refused for its revision as `conflict` (C11).
fn faulted(
    fault: &Fault,
    repeats_from_its_start: bool,
    doc: &mut Map<String, Value>,
    offers: &mut Vec<(Way, String)>,
) {
    if let Fault::Policy(PutPolicyError::Store(AnnotationError::Conflict { current })) = fault {
        doc.insert("kind".into(), json!("conflict"));
        doc.insert(
            "revision".into(),
            json!(current.as_ref().map(|row| row.revision)),
        );
        if let Some(row) = current {
            doc.insert("current".into(), json!(row));
        }
        return;
    }
    doc.extend(fault.fields());
    let names = match fault {
        Fault::UnknownAccount { known, .. } => known.as_slice(),
        Fault::AmbiguousAccount { candidates } => candidates.as_slice(),
        _ => &[],
    };
    offers.extend(names.iter().map(|name| {
        (
            Way::Again {
                account: name.clone(),
            },
            match repeats_from_its_start {
                true => format!("the same request from its start, as {name}"),
                false => format!("the same request as {name}"),
            },
        )
    }));
}

/// The ways that would find a target the facts do not hold, from the
/// scope it was looked for in: where other leagues hold it, its show in
/// each; else the status that lists a row of its own, a search of the
/// realm, whose rows carry their items' ids, and the league's tabs —
/// the commands the refusal's words print, as requests.
fn finding(m: &Missing) -> Vec<(Way, String)> {
    let target = m.target.to_string();
    if !m.elsewhere.is_empty() {
        return m
            .elsewhere
            .iter()
            .map(|(realm, league)| {
                (
                    Way::PriceShow {
                        target: target.clone(),
                        account: m.account.clone(),
                        realm: *realm,
                        league: league.clone(),
                    },
                    format!(
                        "{target} in {} {league}, where the facts hold it",
                        realm.as_str()
                    ),
                )
            })
            .collect();
    }
    let mut ways = Vec::new();
    if let Some(revision) = m.own_row {
        ways.push((
            Way::PriceStatus {
                account: m.account.clone(),
                realm: m.realm,
                league: m.league.clone(),
            },
            format!(
                "the pricing state of {} {}, every row listed: {target}'s own row, at revision {revision}, among them",
                m.realm.as_str(),
                m.league
            ),
        ));
    }
    ways.push((
        Way::Search(Box::new(Request {
            scope: Scope {
                account: Some(m.account.clone()),
                realm: Some(corpus::Realm::One(m.realm.as_str().to_string())),
                ..Scope::default()
            },
            ..Request::default()
        })),
        format!(
            "every item of {}, each row with its item's id",
            m.realm.as_str()
        ),
    ));
    ways.push((
        Way::Tabs {
            account: m.account.clone(),
            realm: m.realm,
            league: m.league.clone(),
        },
        format!(
            "the tabs of {} {}, each with its id",
            m.realm.as_str(),
            m.league
        ),
    ));
    ways
}
