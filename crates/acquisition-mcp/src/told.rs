//! How a tool's answer and its refusal are told: the result a tool
//! returns ([`Told`]), a refusal's words and its one document shape
//! with this server's requests in its offers ([`Spoken`]), the call
//! being served ([`CALLED`]), and the JSON-RPC errors for a call the
//! server could not serve.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use acquisition_client::client::{ConnectError, NotSpawned};
use acquisition_plan::pricing::Expect;
use acquisition_protocol::protocol::ErrorKind;
use acquisition_protocol::realm::Realm;
use acquisition_search::SearchError;
use acquisition_verbs::Fault;
use acquisition_verbs::fault::door_word;
use acquisition_verbs::refusal::{self, Arguments, Way};
use anyhow::Result;
use rmcp::ErrorData;
use rmcp::model::CallToolResult;
use serde_json::{Value, json};

use crate::prose::{Prose, spelled, spoken};

/// A call the server could not serve, as a JSON-RPC error with the full
/// context chain.
fn err(e: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(format!("{e:#}"), None)
}

/// A layer fault in this adapter's words (C11, C116): the cause the fault
/// says, and the remedy in the MCP's own verbs — the argument that
/// selects an account (`named`: `account`, `scope.account` on search),
/// `set_sync_policy`, `refresh_plan`, the person's command for a daemon.
/// One arm per fault, so a cause the layer adds cannot reach an agent
/// without its remedy.
fn fault_words(fault: &Fault, named: &str) -> String {
    match fault {
        Fault::Environment(_)
        | Fault::Unopened(_)
        | Fault::Plan(_)
        | Fault::Policy(_)
        | Fault::Door(_)
        | Fault::Unresponsive(_)
        | Fault::Daemon(_)
        | Fault::Transport(_)
        | Fault::Other(_)
        | Fault::Spend(_)
        | Fault::Replan { .. } => fault.to_string(),
        // No login for the caller to run (the agent's seat, F10: a
        // server pointed at the wrong root answered every call with
        // `acq auth`, the one act an agent cannot perform).
        Fault::NoAccounts { .. } => format!(
            "{fault}; ACQ_STORE_DIR sets the root this server looks under, and a store is made \
             by a login, which is a human's, from a terminal"
        ),
        Fault::UnknownAccount { .. } => format!(
            "{fault}: `{named}` takes a username, with or without its #discriminator, or a uuid"
        ),
        // C187: why this account, and the two ways on; a login is a
        // person's, from a terminal
        Fault::MainNotLoggedIn(_) => {
            format!("{fault}: a person logs it in with `acq auth`, or `{named}` names another")
        }
        Fault::AmbiguousAccount { .. } => format!(
            "{fault}; `{named}` selects one, or main_account_set makes one the main account, \
             taken where none is named"
        ),
        Fault::NoUuid { .. } => format!("{fault}; a human can fix it with one `acq auth` login"),
        Fault::NoSyncPolicy { .. } => {
            format!("{fault} — declare one with set_sync_policy first")
        }
        // a read-back names the account it was refused for (#97)
        Fault::NotCovered {
            covered, username, ..
        } => match covered.len() {
            0 => format!("{fault}: set_sync_policy covers a league"),
            n if n <= acquisition_store::bound::LISTED => {
                format!("{fault}: `league` names one")
            }
            _ => format!(
                "{fault}: `league` names one; sync_policy with `{named}` {username} lists them all"
            ),
        },
        // A merge reads the stored row to merge into, so the retry is a
        // whole write naming the revision.
        Fault::UnreadablePolicy {
            username, revision, ..
        } => format!(
            "{fault} — `replace: true` with `if_revision: {revision}` writes a whole policy over \
             it deliberately; sync_policy with `{named}` {username} reads it"
        ),
        // Absent by this server's own policy in real mode: the remedy is
        // the human's, and the words say so rather than promising a spawn.
        Fault::NoDaemon {
            because: Some(NotSpawned::Policy),
            next,
        } if acquisition_protocol::provider::ggg_mode() => format!(
            "no daemon running in real-GGG mode, and this server never starts one there — \
             {next}: {fault}"
        ),
        Fault::NoDaemon {
            because: Some(_),
            next,
        } => format!("no daemon running — {next}: {fault}"),
        Fault::NoDaemon {
            because: None,
            next,
        } => format!("no daemon running — {next}"),
        Fault::NoJobOnDisk { next, .. } => format!("{fault} — {next}"),
        Fault::Unfinished { next, .. } => {
            format!("{fault}: a daemon resumes it — {next}")
        }
        Fault::AnotherDaemon(found) => format!(
            "{found}; this server never replaces a daemon — resolve it with the CLI (`acq daemon stop`{})",
            if found.world_matches() {
                ""
            } else {
                ", or point ACQ_STORE_DIR at its world"
            }
        ),
    }
}

/// The failure's words: its chain as anyhow prints it, a layer fault in
/// this adapter's words where the chain holds one.
fn failure_words(e: &anyhow::Error, named: &str) -> String {
    let mut parts = Vec::new();
    for link in e.chain() {
        if let Some(fault) = link.downcast_ref::<Fault>() {
            parts.push(fault_words(fault, named));
            return parts.join(": ");
        }
        parts.push(link.to_string());
    }
    format!("{e:#}")
}

/// A tool's failure, sorted by the one rule every tool shares, so no
/// tool decides it alone (one match over the layer's fault). A call the
/// server could not serve is a JSON-RPC error: a daemon's own refusal
/// with its closed kind ([`daemon_error`], C85), a door that did not
/// open ([`connect_error`]), another or an unresponsive daemon, the
/// conversation with the daemon failing. Everything else is a refusal —
/// the request was wrong, or the store or policy cannot serve it, or no
/// daemon runs — a result marked isError holding the document
/// [`refusal()`] words. The release seat's rerun met an unknown account
/// and an uncovered league as JSON-RPC errors where the instructions
/// promise results (mcp-opus-2 calls 50, 46).
pub(crate) fn failed(e: anyhow::Error) -> Told {
    let words = spelled(&failure_words(&e, "account"));
    match e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        Some(Fault::Daemon(refused)) => Err(daemon_error(refused.kind, words)),
        Some(Fault::Door(door)) => Err(connect_error(door, words)),
        Some(Fault::AnotherDaemon(_) | Fault::Unresponsive(_) | Fault::Transport(_)) => {
            Err(ErrorData::internal_error(words, None))
        }
        _ => Ok(refusing(refusal(&e, "account"))),
    }
}

/// A refusal's document (C11, C121; round 4 of the 2g build, the one
/// refusal shape): the verbs' one sorter
/// (`acquisition_verbs::refusal::document`), its words this server's,
/// `named` the argument that selects an account, and its offers spelled
/// as this server's requests ([`Spoken`]).
pub(crate) fn refusal(e: &anyhow::Error, named: &str) -> Value {
    refusal::document(e, &Spoken { named })
}

tokio::task_local! {
    /// The call being served, as sent: what a refusal's offer to send it
    /// again as another account is made from ([`Spoken`]), and where the
    /// call took the main account, what its answer adds ([`took_main`]).
    /// Set around every call by `call_tool`.
    pub(crate) static CALLED: Called;
}

/// A tool's name and its arguments, as the call sent them; and the
/// account the call took as the main one, where it took it (C187).
#[derive(Clone)]
pub(crate) struct Called {
    pub(crate) tool: String,
    pub(crate) arguments: serde_json::Map<String, Value>,
    /// Set by the server's account door ([`took_main`]) when this call
    /// named no account and the main account was taken: what the answer
    /// adds as `main_account`.
    pub(crate) main: std::sync::Arc<std::sync::Mutex<Option<Value>>>,
}

/// C187: note that the call being served took `account` as the main
/// account, several being known and none named; its answer says so
/// ([`told_as`]). Outside a call, nothing.
pub(crate) fn took_main(account: &acquisition_verbs::Account) {
    let Some(taken) = acquisition_verbs::main_account::taken(account) else {
        return;
    };
    let _ = CALLED.try_with(|c| {
        if let Ok(mut main) = c.main.lock() {
            *main = Some(taken);
        }
    });
}

/// C187: `document` with the call's `main_account`, where the call took
/// the main account, added by the verbs' one function: every answer and
/// every refusal this server serves passes here.
fn marked(document: &mut Value) {
    let main = CALLED
        .try_with(|c| c.main.lock().ok().and_then(|m| m.clone()))
        .ok()
        .flatten();
    acquisition_verbs::main_account::mark(document, main.as_ref());
}

/// This server's side of the one refusal shape: its words
/// ([`failure_words`]) and its requests — an offer is the arguments a
/// tool takes, naming the tool where it is another than the one refused;
/// the same call again is its arguments as sent with the account alone
/// changed (`scope.account` for `search`, inside the object that holds
/// the request where it came in one).
pub(crate) struct Spoken<'a> {
    pub(crate) named: &'a str,
}

impl refusal::Surface for Spoken<'_> {
    fn words(&self, e: &anyhow::Error) -> String {
        failure_words(e, self.named)
    }

    fn repeats_from_its_start(&self) -> bool {
        CALLED
            .try_with(|c| {
                c.tool == "search"
                    && refusal::again(&Value::Object(c.arguments.clone()), "")
                        .is_some_and(|(_, restarted)| restarted)
            })
            .unwrap_or(false)
    }

    fn request(&self, way: &Way) -> Option<(Value, Option<&'static str>)> {
        let called = CALLED.try_with(Called::clone).ok();
        let tool = called.as_ref().map(|c| c.tool.clone());
        let other = |name: &'static str| (tool.as_deref() != Some(name)).then_some(name);
        match way {
            // the arguments as sent, only the account changed: for
            // `search` the verbs' one rule over a request read whole
            // (`refusal::again`), a continuation from its start
            Way::Again { account } => {
                let mut arguments = called?.arguments;
                // a saved search run is no request the reader reads: its
                // arguments as sent, `scope.account` set, nothing restarted
                if tool.as_deref() == Some("search") && arguments.contains_key("saved") {
                    let scope = arguments
                        .entry("scope")
                        .or_insert_with(|| json!({}))
                        .as_object_mut()?;
                    scope.insert("account".into(), json!(account));
                    return Some((Value::Object(arguments), None));
                }
                if tool.as_deref() == Some("search") {
                    let (envelope, _) = refusal::again(&Value::Object(arguments), account)?;
                    return Some((envelope, None));
                }
                arguments.insert("account".into(), json!(account));
                Some((Value::Object(arguments), None))
            }
            Way::Search(request) => Some((serde_json::to_value(request).ok()?, other("search"))),
            // the index where no names are asked: `{}`
            Way::Describe { names } => Some((
                match names.is_empty() {
                    true => json!({}),
                    false => json!({ "names": names }),
                },
                other("describe"),
            )),
            Way::Tabs {
                account,
                realm,
                league,
            } => Some((
                json!({ "account": account, "realm": realm.as_str(), "league": league }),
                other("tabs"),
            )),
            Way::RefreshPlan {
                account,
                realm,
                league,
            } => Some((
                json!({ "account": account, "realm": realm.as_str(), "league": league }),
                other("refresh_plan"),
            )),
            // apply_plan takes the envelope refresh_plan returns, never a
            // scope it compiles: no request here applies one by its league
            Way::RefreshApply { .. } => None,
            Way::PriceShow {
                target,
                account,
                realm,
                league,
            } => {
                let mut request = json!({ "target": target, "account": account });
                if let Some(realm) = realm {
                    request["realm"] = json!(realm.as_str());
                }
                if let Some(league) = league {
                    request["league"] = json!(league);
                }
                Some((request, other("price_show")))
            }
            Way::PriceStatus {
                account,
                realm,
                league,
            } => Some((
                json!({
                    "account": account, "realm": realm.as_str(), "league": league, "expand": true,
                }),
                other("price_status"),
            )),
            // a receipt's undo: the write tool's own arguments, the
            // amount and currency only where the type carries a price,
            // the revision only where it names one (a create names none)
            Way::PriceSet {
                target,
                account,
                kind,
                amount,
                currency,
                if_revision,
            } => {
                let mut arguments = serde_json::Map::new();
                arguments.insert("target".into(), json!(target));
                arguments.insert("type".into(), json!(kind));
                if let Some(amount) = amount {
                    arguments.insert("amount".into(), json!(amount));
                }
                if let Some(currency) = currency {
                    arguments.insert("currency".into(), json!(currency));
                }
                if let Some(revision) = if_revision {
                    arguments.insert("if_revision".into(), json!(revision));
                }
                arguments.insert("account".into(), json!(account));
                Some((Value::Object(arguments), other("price_set")))
            }
            Way::PriceClear {
                target,
                account,
                if_revision,
            } => Some((
                json!({ "target": target, "if_revision": if_revision, "account": account }),
                other("price_clear"),
            )),
            Way::PolicySet {
                account,
                value,
                if_revision,
            } => Some((
                json!({
                    "value": value, "if_revision": if_revision, "replace": true, "account": account,
                }),
                other("set_sync_policy"),
            )),
            // a failed job's way on (C165): submit_job's own arguments
            Way::Submit {
                account,
                kind,
                params,
            } => {
                let mut arguments = json!({ "kind": kind, "params": params });
                if let Some(account) = account {
                    arguments["account"] = json!(account);
                }
                Some((arguments, other("submit_job")))
            }
            Way::Result { id } => Some((json!({ "id": id }), other("job_result"))),
            Way::Currencies => Some((json!({}), other("reference_currency"))),
            Way::RefusedBodies { account } => {
                Some((json!({ "account": account }), other("store_refused")))
            }
            // the saved search's ways as its three tools' arguments, and
            // a run as `search`'s `saved`
            Way::SavedSearches { expand } => Some((
                match expand {
                    true => json!({ "expand": true }),
                    false => json!({}),
                },
                other("saved_searches"),
            )),
            Way::SavedSearchShow { id } => Some((json!({ "saved": id }), other("saved_searches"))),
            Way::SavedSearchRun { id, account } => {
                let mut request = json!({ "saved": id });
                if let Some(account) = account {
                    request["scope"] = json!({ "account": account });
                }
                Some((request, other("search")))
            }
            // the write's own arguments: the revision under `Exactly`,
            // none under `Create` (this server's create, at the id where
            // one is named); a write at the revision read is no form this
            // server sends (C52)
            Way::SavedSearchSet(set) => {
                let mut arguments = serde_json::Map::new();
                if let Some(id) = &set.id {
                    arguments.insert("id".into(), json!(id));
                }
                match set.expect {
                    Expect::Exactly(revision) => {
                        arguments.insert("if_revision".into(), json!(revision));
                    }
                    Expect::Create => {}
                    Expect::Current => return None,
                }
                for (key, value) in [
                    ("name", &set.name),
                    ("text", &set.text),
                    ("realm", &set.realm),
                    ("membership", &set.membership),
                ] {
                    if let Some(value) = value {
                        arguments.insert(key.into(), json!(value));
                    }
                }
                Some((Value::Object(arguments), other("saved_search_set")))
            }
            Way::SavedSearchClear { id, if_revision } => Some((
                json!({ "id": id, "if_revision": if_revision }),
                other("saved_search_clear"),
            )),
            Way::MainAccountSet {
                account,
                if_revision,
            } => Some((
                json!({ "main": account, "if_revision": if_revision }),
                other("main_account_set"),
            )),
            Way::MainAccountClear { if_revision } => Some((
                json!({ "main": null, "if_revision": if_revision }),
                other("main_account_set"),
            )),
        }
    }
}

/// A tool's answer: the document, or its failure as [`failed`] sorts it.
pub(crate) fn settled<T: serde::Serialize>(answer: Result<T>) -> Told {
    match answer {
        Ok(document) => told(&document),
        Err(e) => failed(e),
    }
}

/// [`settled`] over a tool's body, for a tool that never awaits.
pub(crate) fn answer<T: serde::Serialize>(body: impl FnOnce() -> Result<T>) -> Told {
    settled(body())
}

/// A door that did not open, as a JSON-RPC error: the server could not
/// serve the call (`internal_error`), and `data.connect` names which
/// door failed — `absent`, `incompatible`, `other_world`,
/// `spawn_failed`, `unresponsive`, `transport` — so an agent can branch
/// on it the way it branches on a daemon error's kind.
fn connect_error(door: &ConnectError, message: String) -> ErrorData {
    ErrorData::internal_error(message, Some(json!({ "connect": door_word(door) })))
}

/// A daemon refusal as a JSON-RPC error (C85): the code follows the kind
/// — the request was wrong (`invalid_request`), its parameters were
/// (`invalid_params`), it named nothing (`resource_not_found`), or the
/// daemon could not serve it (`internal_error`) — and the kind itself
/// rides in `data` so an agent can branch on the closed set.
fn daemon_error(kind: ErrorKind, message: String) -> ErrorData {
    let data = Some(json!({ "kind": kind }));
    match kind {
        ErrorKind::BadRequest => ErrorData::invalid_request(message, data),
        ErrorKind::Refused
        | ErrorKind::AmbiguousAccount
        | ErrorKind::UnknownAccount
        | ErrorKind::WrongState => ErrorData::invalid_params(message, data),
        ErrorKind::UnknownJob => ErrorData::resource_not_found(message, data),
        ErrorKind::NotLoggedIn
        | ErrorKind::QueueFailed
        | ErrorKind::Upstream
        | ErrorKind::Internal => ErrorData::internal_error(message, data),
    }
}

// ---- the item search: adapters of `acquisition-search` ----

/// What a tool of the search returns: the document the CLI's `--json`
/// prints — an answer, or a failure in a result marked an error.
pub(crate) type Told = Result<CallToolResult, ErrorData>;

/// The document as a tool's result: one text block, serialized from the
/// document itself, so its keys come in the order the document declares
/// them (C115), and no structured content ([`told_as`]).
pub(crate) fn told<T: serde::Serialize>(document: &T) -> Told {
    told_as(document, Prose { all: false })
}

/// [`told`], the document's prose as [`Prose`] says: each string
/// [`spoken`] for this server.
///
/// The text block alone, by measurement (step 2g's round 7): in the
/// stream `claude` 2.1.312 logged for two successes of one tool, the
/// result it passed on was the structured copy alone, its keys sorted,
/// when one was sent, and the text block when none was (that the model
/// reads what the stream logs is inferred; a refusal was not measured).
/// So the text block alone carries the declared order to the model: the
/// answer leads with what bears on the question (C115), and on the MCP
/// the JSON is the text, with no second rendering (C53).
pub(crate) fn told_as<T: serde::Serialize>(document: &T, prose: Prose) -> Told {
    let mut value = serde_json::to_value(document).map_err(|e| err(e.into()))?;
    // C187: an answer whose call took the main account says which, and
    // that it was the default, under one additive key
    marked(&mut value);
    let text = spoken(
        &serde_json::to_string(&value).map_err(|e| err(e.into()))?,
        prose,
    );
    // C149: every answer is one JSON object; a bare array, a bare null or
    // a scalar is this server's fault, never served (held here, the one
    // door every answer passes, so no tool can regress alone)
    if !text.starts_with('{') {
        return Err(err(anyhow::anyhow!(
            "an answer must be one JSON object, and this tool's is not"
        )));
    }
    // the text is one JSON document, whatever `spoken` did to it
    serde_json::from_str::<serde::de::IgnoredAny>(&text).map_err(|e| err(e.into()))?;
    Ok(CallToolResult::success(vec![
        rmcp::model::ContentBlock::text(text),
    ]))
}

/// A refusal the search crate made (C11), in the one shape: `error`,
/// `kind`, a language refusal's `readings`, payload, and `offers`, each
/// saying in the request's words what it is.
pub(crate) fn refused(e: impl Into<SearchError>) -> CallToolResult {
    refusing(refusal::of_search(&e.into(), &Spoken { named: "account" }))
}

/// A refusal's document as a tool's result, marked an error: its prose
/// [`spoken`] for this server, its readings and offers payload; the text
/// block alone, as an answer's is ([`told_as`]).
pub(crate) fn refusing(mut doc: Value) -> CallToolResult {
    // C187: a refusal whose call took the main account says so too
    marked(&mut doc);
    let text = spoken(&doc.to_string(), Prose { all: true });
    CallToolResult::error(vec![rmcp::model::ContentBlock::text(text)])
}

/// A pricing verb's answer as a tool's result: the document the CLI's
/// `--json` prints, or its failure document, `{"error": …}`, in a result
/// marked an error, its words [`spelled`] for this server.
pub(crate) fn priced<T: serde::Serialize>(answer: Result<T>) -> Told {
    match answer {
        Ok(document) => told(&document),
        Err(e) => Ok(refusing(refusal(&e, "account"))),
    }
}

/// A pricing tool's `realm`, read as the CLI's `--realm` is; a word that
/// is no realm is refused as any argument the tool does not take.
pub(crate) fn price_realm(realm: Option<&str>) -> std::result::Result<Option<Realm>, Refused> {
    realm
        .map(|s| {
            acquisition_verbs::realm(s)
                .map_err(|why| Box::new(refusing(refusal(&Arguments(why).into(), "account"))))
        })
        .transpose()
}

/// A refusal a tool's helper hands back for the tool to return as its
/// result ([`refusing`]), boxed: `CallToolResult` carries JSON values,
/// which `preserve_order` makes the size of an insertion-ordered map
/// (#200), too large to pass by value in every helper's `Err`.
pub(crate) type Refused = Box<CallToolResult>;

#[cfg(test)]
mod tests {
    use super::*;

    /// The unreadable policy's remedy names both arguments the retry
    /// needs: the merge would read the row it cannot; and its read-back
    /// names the account it was refused for (#97).
    #[test]
    fn the_unreadable_policy_remedy_names_a_whole_write_at_its_revision() {
        let words = fault_words(
            &Fault::UnreadablePolicy {
                username: "Alice#1234".into(),
                revision: 3,
                detail: "newer".into(),
            },
            "account",
        );
        assert!(
            words.contains("`replace: true` with `if_revision: 3`"),
            "{words}"
        );
        assert!(
            words.contains("sync_policy with `account` Alice#1234 reads it"),
            "{words}"
        );
    }
}
