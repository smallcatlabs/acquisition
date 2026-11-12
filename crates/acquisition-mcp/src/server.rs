//! The server: [`AcqMcp`], its openers, the tool table `tools/list`
//! shows and `MCP-REFERENCE.md` is generated from, and the handler that
//! serves each call and gives the `initialize` instructions.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use acquisition_plan::pricing::Expect;
use acquisition_plan::{PolicyWrite, RefreshPlan};
use acquisition_protocol::protocol::ListState;
use acquisition_protocol::realm::Realm;
use acquisition_search::answer::{Asked, QueryInput};
use acquisition_search::{Holder, SearchError};
use acquisition_store::{Annotations, Provenance, Store};
use acquisition_verbs::leagues::PolicyRead;
use acquisition_verbs::receipt;
use acquisition_verbs::refusal::Arguments;
use acquisition_verbs::saved::{self, Set};
use acquisition_verbs::{Account, Fault, Frontend, Held};
use anyhow::Result;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerConfig};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};
use serde_json::{Value, json};

use crate::describe::{Described, Entry, OWN_SECTIONS, own_listed, own_sections};
use crate::params::*;
use crate::told::{
    CALLED, Called, Refused, Spoken, Told, answer, failed, price_realm, priced, refusal, refused,
    refusing, settled, told,
};

/// The server: one frontend for its lifetime (C46), an agent's door
/// (C13), the account resolved per call.
pub(crate) struct AcqMcp {
    pub(crate) frontend: Frontend,
    /// The one corpus `search` answers over (C98).
    pub(crate) held: Holder,
}

impl AcqMcp {
    /// C121: the account a call's selector names.
    fn account(&self, selector: Option<&str>) -> Result<Account, Fault> {
        self.frontend.account(selector)
    }

    /// The selected account's facts, as the CLI's store verbs open them.
    fn facts(&self, selector: Option<&str>) -> Result<Store, Fault> {
        self.account(selector)?.facts()
    }

    /// The selected account's facts and its intent file, as `acq search`
    /// opens them: the search reads both and writes neither. A failure is
    /// a refusal document, `named` the argument that selects an account.
    fn open_search(
        &self,
        selector: Option<&str>,
        named: &str,
    ) -> std::result::Result<(Store, Annotations), Refused> {
        self.account(selector)
            .and_then(|account| account.open())
            .map(|opened| (opened.facts, opened.intent))
            .map_err(|e| Box::new(refusing(refusal(&e.into(), named))))
    }

    /// [`Self::open_search`] with the account's name, for a refusal that
    /// prints a command to run as that account (a second account known,
    /// a command naming none is ambiguous).
    fn open_priced(
        &self,
        selector: Option<&str>,
        named: &str,
    ) -> std::result::Result<(Store, Annotations, String), Refused> {
        self.account(selector)
            .and_then(|account| account.open())
            .map(|opened| (opened.facts, opened.intent, opened.account.entry.username))
            .map_err(|e| Box::new(refusing(refusal(&e.into(), named))))
    }

    /// `search`'s `saved`: taken out of the arguments, resolved by the
    /// verbs (`saved::request_of`, what `acq search --saved` runs), and the
    /// saved search's text, realm and membership set into the arguments as
    /// sent, so every other key reaches the reader as it came and is
    /// refused there as it is without `saved`; the search crate never sees
    /// the word. A word of a request beside it is refused, naming the
    /// word, in the kind and the sentence of the CLI's refusal of a flag
    /// beside `--saved`; `scope.account` is read where it is a string, and
    /// left for the reader otherwise.
    fn saved_into(
        &self,
        args: &mut serde_json::Map<String, Value>,
    ) -> std::result::Result<(), Refused> {
        let Some(saved) = args.shift_remove("saved") else {
            return Ok(());
        };
        let Some(saved) = saved.as_str() else {
            return Err(Box::new(refused(acquisition_search::LanguageError {
                kind: acquisition_search::ErrorKind::Tree,
                message: "`saved` is a saved search's id or its name, as a string".to_string(),
                span: None,
                readings: Vec::new(),
            })));
        };
        let scope = args.get("scope").and_then(Value::as_object);
        let in_scope = |key: &str| scope.is_some_and(|scope| scope.contains_key(key));
        let beside = [
            ("query", args.contains_key("query")),
            ("scope.realm", in_scope("realm")),
            ("scope.membership", in_scope("membership")),
            ("request", args.contains_key("request")),
            ("basis", args.contains_key("basis")),
            ("count", args.contains_key("count")),
            ("counted_at", args.contains_key("counted_at")),
        ];
        if let Some((word, _)) = beside.iter().find(|(_, sent)| *sent) {
            return Err(Box::new(refused(SearchError::Scope {
                kind: "view",
                message: format!(
                    "`saved` runs a saved search whole, its scope and its query: `{word}` has nowhere to stand beside it"
                ),
                offers: Vec::new(),
            })));
        }
        let account = scope
            .and_then(|scope| scope.get("account"))
            .and_then(Value::as_str)
            .map(str::to_string);
        let request = saved::request_of(&self.frontend, saved, account.as_deref())
            .map_err(|e| refusing(refusal(&e, "scope.account")))?;
        args.insert("query".into(), json!({ "text": request.query.text }));
        if let Some(scope) = args
            .entry("scope")
            .or_insert_with(|| json!({}))
            .as_object_mut()
        {
            if let Some(realm) = request.scope.realm {
                scope.insert("realm".into(), json!(realm.as_str()));
            }
            scope.insert(
                "membership".into(),
                json!(request.scope.membership.as_str()),
            );
        }
        Ok(())
    }
}

#[tool_router]
impl AcqMcp {
    // ---- store reads: no daemon, no GGG traffic ----

    #[tool(
        description = "Accounts this machine has logged into (from the store's non-secret index; no daemon), each with its store file and `last_login`, the last sign-in or token refresh: `total` counts every one, `accounts` lists ten, `expand` every one, and `left_out` counts the rest. `persisted` is true where the account's login is kept in the keyring, false where it lived in a daemon alone (no keyring, a failed save, or a logout). Each age is in words beside its time (`describe` with names [\"ages\"])."
    )]
    fn accounts(&self, Parameters(p): Parameters<Args<AccountsParams>>) -> Told {
        answer(|| {
            let expand = p.parse()?.expand.unwrap_or(false);
            Ok(self.frontend.accounts()?.bounded(expand))
        })
    }

    #[tool(
        description = "The stash tabs of a league, counted first, from the store (no daemon), then listed — the ten with the most items, `limit` more or fewer, `expand` every one — each with its type, item count and fetch time, its age in words beside it, and the league's `oldest_fetch`, `newest_fetch` and `list_seen`; `type` and `never_fetched` select the tabs listed; `leagues_held`, the realm's other leagues on disk. `describe` with names [\"store\"] says what each count counts, and [\"ages\"] what an age's words say. A name ending ` (Remove-only)` is GGG's mark on a tab whose items can only be taken out."
    )]
    fn tabs(&self, Parameters(p): Parameters<Args<TabsParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let realm = realm_param(p.realm.as_deref())?;
            // one account for the facts and the policy: a login landing
            // between two resolutions could answer each from another
            let account = self.account(p.account.as_deref())?;
            let store = account.facts()?;
            let league = p
                .league
                .as_deref()
                .unwrap_or(acquisition_plan::DEFAULT_LEAGUE);
            let expand = p.expand.unwrap_or(false);
            let (listed, holdings) = acquisition_search::tabs::Tabs::read(
                &store,
                realm.as_str(),
                league,
                &acquisition_search::tabs::Selection {
                    kind: p.r#type,
                    never_fetched: p.never_fetched.unwrap_or(false),
                },
                p.limit,
                expand,
                acquisition_store::now(),
            )?;
            let policy = PolicyRead::of_account(&account);
            Ok(Held::of(listed, holdings, &policy, realm, league, expand))
        })
    }

    #[tool(
        description = "Characters known to the store (no daemon), counted first — how many, fetched, fetched empty, never fetched, their items — then listed: ten, `limit` more or fewer, `expand` every one, each with class, level, league, `item_count` (the live items held at it now) and `fetched_items` (what the last full fetch carried, null until one), and when it was listed and fetched, each age in words beside its time. `describe` with names [\"store\"] says what each count counts, and [\"ages\"] what an age's words say. Fresh data is a `characters` or `character` job."
    )]
    fn characters(&self, Parameters(p): Parameters<Args<CharactersParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let realm = p
                .realm
                .as_deref()
                .map(|r| realm_param(Some(r)))
                .transpose()?;
            let store = self.facts(p.account.as_deref())?;
            let realm = realm.map(Realm::as_str);
            Ok(acquisition_verbs::Characters::of(
                realm,
                p.league.as_deref(),
                store.characters(realm, p.league.as_deref())?,
                p.limit,
                p.expand.unwrap_or(false),
                acquisition_store::now(),
            ))
        })
    }

    #[tool(
        description = "Item search over the store (no daemon): a query in the language `describe` prints, asked of every item in scope. The arguments are the request: {} is every item; {\"query\": {\"text\": \"class:ring rarity=rare\"}} is a query, and {\"query\": {\"trade\": a trade site URL}} a site search translated to one; {\"saved\": an id or a name} runs a saved search. The answer is one JSON document — `query`, `scope`, `total`, `rows`, `view`, `terms`, `basis`; `describe` with names [\"answer\"] says what each key holds and what each count counts. Any request an answer holds is sent back as printed."
    )]
    fn search(&self, Parameters(RequestParams(mut args)): Parameters<RequestParams>) -> Told {
        if let Err(refusal) = self.saved_into(&mut args) {
            return Ok(*refusal);
        }
        let asked = match Asked::read(&Value::Object(args)) {
            Ok(asked) => asked,
            Err(e) => return Ok(refused(e)),
        };
        // an authoring error is said before the store is read
        if let Err(e) = asked.request.check() {
            return Ok(refused(e));
        }
        let (store, intent) =
            match self.open_search(asked.request.scope.account.as_deref(), "scope.account") {
                Ok(opened) => opened,
                Err(refusal) => return Ok(*refusal),
            };
        match self
            .held
            .corpus(&asked.request, &store, &intent)
            .and_then(|corpus| asked.answer(&corpus))
        {
            Ok(answer) => told(&answer),
            Err(e) => Ok(refused(e)),
        }
    }

    #[tool(
        description = "What the tools answer and the language `search` takes (no store read). No names: the index, each entry's name and one line, some 38 KB. A name answers whole: a section (fields, what a query may ask; answer, a search answer's keys; trade, what a site search becomes; pricing; daemon; store), an entry, or all: over 150 KB, computed some 80 KB, fields and trade 20 KB each, categories 10 KB, pricing 8 KB, answer 7 KB, store 6 KB, daemon 5 KB, the rest under 4 KB. A pattern (\"~resist\") reads the language's names and definitions alone. An unknown name is refused with every name it knows."
    )]
    fn describe(&self, Parameters(p): Parameters<Args<DescribeParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let names = p.names.unwrap_or_default();
        let sections = own_sections();
        // an own section answers to its name, and each of its entries to
        // its own, as the language's entries do (C97: every word an entry
        // is named by can be asked for alone)
        let asked = |n: &str, section: usize, entry: &Entry| {
            n.eq_ignore_ascii_case(OWN_SECTIONS[section]) || n.eq_ignore_ascii_case(entry.name)
        };
        let own_name = |n: &str| {
            OWN_SECTIONS.iter().any(|o| o.eq_ignore_ascii_case(n))
                || sections
                    .iter()
                    .flatten()
                    .any(|e| e.name.eq_ignore_ascii_case(n))
        };
        let language: Vec<String> = names.iter().filter(|n| !own_name(n)).cloned().collect();
        // the whole, or the language's names alone: an own name asked
        // alone is never the whole language
        let language = if names.is_empty() || !language.is_empty() {
            match acquisition_search::describe(&language) {
                Ok(described) => Some(described),
                Err(mut e) => {
                    match &mut e {
                        // every name this tool knows, its own among them
                        SearchError::Language(e)
                            if e.kind == acquisition_search::error::ErrorKind::UnknownName =>
                        {
                            let own: Vec<&str> = OWN_SECTIONS
                                .iter()
                                .copied()
                                .chain(sections.iter().flatten().map(|e| e.name))
                                .collect();
                            e.message.push_str(&format!(", {}", own.join(", ")));
                        }
                        // a pattern reads the language alone: this tool's
                        // own sections are read by name
                        SearchError::Scope { message, .. } => {
                            message.push_str(&format!("; {} are read by name alone", own_listed()))
                        }
                        _ => {}
                    }
                    return Ok(refused(e));
                }
            }
        } else {
            None
        };
        // nothing named is the index, each own entry at the search
        // crate's one line beside the language's; `all` every one whole
        let all = names
            .iter()
            .any(|n| n.eq_ignore_ascii_case(acquisition_search::describe::ALL));
        let mut chosen = sections.into_iter().enumerate().map(|(section, entries)| {
            let kept: Vec<Entry> = entries
                .into_iter()
                .filter(|e| all || names.is_empty() || names.iter().any(|n| asked(n, section, e)))
                .map(|e| match names.is_empty() {
                    true => Entry {
                        what: acquisition_search::describe::one_line(&e.what).to_string(),
                        ..e
                    },
                    false => e,
                })
                .collect();
            (!kept.is_empty()).then_some(kept)
        });
        told(&Described {
            language,
            pricing: chosen.next().flatten(),
            daemon: chosen.next().flatten(),
            store: chosen.next().flatten(),
        })
    }

    #[tool(
        description = "One item as the search derives it, live or removed (no daemon): its place, its fields, class, category and price, and every line with its kind, template and numbers. A line's `value` and each `number_fields` key are values to sort by. `against` adds why the item does or does not match a query, each node's outcome (`describe` with names [\"against\"]); `body` adds the stored body, verbatim. `basis` is what the answer was read at, its `intent` the revision of your prices and sync policy (`describe` with names [\"answer\"]). Each age is in words beside its time (`describe` with names [\"ages\"])."
    )]
    fn show(&self, Parameters(p): Parameters<Args<ShowParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let against = match p.against.map(serde_json::from_value::<QueryInput>) {
            None => None,
            Some(Ok(query)) => Some(query),
            Some(Err(e)) => {
                // the language's own refusal of a tree it cannot read
                return Ok(refused(acquisition_search::LanguageError {
                    kind: acquisition_search::ErrorKind::Tree,
                    message: format!(
                        "`against` is a query as a request holds one, {{\"text\": …}} or {{\"tree\": …}}: {e}"
                    ),
                    span: None,
                    readings: Vec::new(),
                }));
            }
        };
        // an authoring error is said before the store is read
        if let Some(Err(e)) = against.as_ref().map(|q| q.read(None)) {
            return Ok(refused(e));
        }
        let (store, intent) = match self.open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(*refusal),
        };
        let body = p.body.unwrap_or(false);
        match match &against {
            Some(query) => acquisition_search::show::against(&store, &intent, &p.id, body, query),
            None => acquisition_search::show(&store, &intent, &p.id, body),
        } {
            Ok(shown) => told(&shown),
            Err(e) => Ok(refused(e)),
        }
    }

    #[tool(
        description = "Store file path, size, and row counts (no daemon), and `facts`: how old the facts are over every realm and league, the oldest and newest fetch with their locations; `describe` with names [\"store\"] says what each count counts."
    )]
    fn store_status(&self, Parameters(p): Parameters<Args<StoreParams>>) -> Told {
        answer(|| self.facts(p.parse()?.account.as_deref())?.status())
    }

    #[tool(
        description = "Item events (added/moved/changed/removed) from recent ingests (no daemon): the newest ten, `limit` more or fewer, beside how many were returned and whether the limit was reached; `summary: true` counts them by location instead, ten locations, `expand` every one. `describe` with names [\"store\"] says what each count counts, and [\"ages\"] what an age's words say."
    )]
    fn item_events(&self, Parameters(p): Parameters<Args<EventsParams>>) -> Told {
        use acquisition_verbs::events::{EventSummary, Events};
        answer(|| -> Result<Value> {
            let p = p.parse()?;
            let store = self.facts(p.account.as_deref())?;
            let now = acquisition_store::now();
            let events = Events::read(
                &store,
                now,
                p.hours.unwrap_or(acquisition_verbs::events::DEFAULT_HOURS),
                p.limit,
            )?;
            Ok(if p.summary.unwrap_or(false) {
                serde_json::to_value(EventSummary::read(
                    &store,
                    &events,
                    p.expand.unwrap_or(false),
                    now,
                )?)?
            } else {
                serde_json::to_value(events)?
            })
        })
    }

    #[tool(
        description = "Bodies the store refused as malformed, kept verbatim as evidence (no daemon): the newest ten, `limit` more or fewer, beside `returned`, `limit` and `total`, every body on record; `id`, a row's id from that list, reads that one body in full. `describe` with names [\"store\"] says what `refused_bodies` counts."
    )]
    fn store_refused(&self, Parameters(p): Parameters<Args<RefusedParams>>) -> Told {
        use acquisition_verbs::RefusedBodies;
        answer(|| -> Result<Value> {
            let p = p.parse()?;
            let account = self.account(p.account.as_deref())?;
            let store = account.facts()?;
            let now = acquisition_store::now();
            Ok(match p.id {
                Some(id) => serde_json::to_value(RefusedBodies::one(
                    &store,
                    id,
                    now,
                    &account.entry.username,
                )?)?,
                None => {
                    serde_json::to_value(RefusedBodies::read(&store, p.limit.unwrap_or(10), now)?)?
                }
            })
        })
    }

    // ---- the intent/plan slice (tracer step 8): policy → plan → apply ----

    #[tool(
        description = "The account's sync policy (declared coverage and freshness) with its revision — what refresh_plan compiles (no daemon). `policy` is null when none is set; `leagues_held`, each realm's leagues on disk with `named` where the policy names them, ten a realm, `expand` all (`describe` [\"store\"]). Each age is in words beside its time (`describe` with names [\"ages\"])."
    )]
    fn sync_policy(&self, Parameters(p): Parameters<Args<PolicyShowParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            Ok(self
                .account(p.account.as_deref())?
                .policy_shown(p.expand.unwrap_or(false))?)
        })
    }

    #[tool(
        description = "The league list last fetched, read from the store with no daemon (`stored: true`): each league with its realm, `started_at` (the newest start first), `named` (whether the sync policy names it) and `held` (its tabs and characters on disk), ten, `expand` every one, `leagues_total` all of them; `policy` says whether one is set. Never fetched: `fetched_at` null and no leagues; a `leagues` job (submit_job) fetches the list. `describe` with names [\"store\"] says what each count counts."
    )]
    fn leagues(&self, Parameters(p): Parameters<Args<LeaguesParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            if !p.stored {
                return Err(Arguments(
                    "this tool reads the stored league list and takes `stored: true`; a `leagues` job (submit_job) fetches it anew".into(),
                )
                .into());
            }
            Ok(self
                .account(p.account.as_deref())?
                .league_list(p.expand.unwrap_or(false))?)
        })
    }

    #[tool(
        description = "Write the account's sync policy, validated strictly, compare-and-swap on `if_revision`. Local only: it sends nothing. Each realm the value names replaces that realm alone, the others stay; `replace: true` writes the whole policy. Returns the stored row with the receipt — `target`, `written`, `prior`, `undo` (a request that puts `prior` back) or `no_undo` (why none) — and `league_notes` for a league the fetched league list lacks, with `list_fetched_at` (null if never fetched; a `leagues` job fetches it) and its age in words (`describe` [\"ages\"]). Nothing refuses on a note."
    )]
    fn set_sync_policy(&self, Parameters(p): Parameters<Args<SetPolicyParams>>) -> Told {
        let written = (|| -> Result<_> {
            let p = p.parse()?;
            let write = if p.replace.unwrap_or(false) {
                PolicyWrite::Replace
            } else {
                PolicyWrite::Merge
            };
            // C52: naming no revision creates, never replaces.
            let written = self.account(p.account.as_deref())?.set_policy(
                &p.value,
                Expect::or(p.if_revision, Expect::Create),
                write,
            )?;
            Ok(written)
        })();
        match written {
            Ok(w) => told(&receipt::policy(&w, &Spoken { named: "account" })),
            Err(e) => failed(e),
        }
    }

    #[tool(
        description = "Compile a refresh plan for one realm and league offline, sending nothing: from the stored sync policy, or from an explicit `selection`, which needs none. The plan is the bounded action set applying would execute, with reasons, skips, unknown ids and a wire estimate; a running daemon adds its read-only quote, never spawned for it (`quote_note` says why none). A policy plan's `notes` names a league the fetched list lacks or a covered league with no character listed; `uncovered_tabs`, the league's tabs outside the policy. Review it, then hand `plan` to apply_plan. Its ages: `describe` [\"ages\"]."
    )]
    async fn refresh_plan(&self, Parameters(p): Parameters<Args<PlanParams>>) -> Told {
        settled(
            async {
                let p = p.parse()?;
                let realm = realm_param(p.realm.as_deref())?;
                let mut opened = self.account(p.account.as_deref())?.open()?;
                let league = p
                    .league
                    .as_deref()
                    .unwrap_or(acquisition_plan::DEFAULT_LEAGUE);
                Ok(match &p.selection {
                    Some(selection) => opened.plan_selection(realm, league, selection).await?,
                    None => opened.plan(realm, league).await?,
                })
            }
            .await,
        )
    }

    #[tool(
        description = "Execute a reviewed plan, its actions and nothing else, as one `apply` parent job the daemon admits or refuses whole. Refused offline if the stored policy revision moved since the plan (replan with refresh_plan). A plan spends each request once: an action fetched since the plan, or held by the daemon, is listed in `spent` and not sent, so a replay answers `applied: false` with `remaining_requests`, what a plan compiled now would fetch. Returns `job_id`, `account`, `requests`, `spent` and `skipped`; poll job_status, then job_result. Ages in words: `describe` [\"ages\"]."
    )]
    async fn apply_plan(&self, Parameters(p): Parameters<Args<ApplyParams>>) -> Told {
        settled(
            async {
                let p = p.parse()?;
                let mut opened = self.account(p.account.as_deref())?.open()?;
                let plan = RefreshPlan::from_value(&p.plan).map_err(Fault::from)?;
                Ok(opened.apply(&plan, p.max_requests, &mut None).await?)
            }
            .await,
        )
    }

    // ---- pricing: adapters of acquisition-plan's pricing verbs ----

    #[tool(
        description = "The listing state of one league, counted (no daemon): what every item, tab and character is priced as — by hand, in game, how the two stand — as the counts and the row accounting, without the listings themselves, and in `facts` how old the facts are: the report's non-folder containers, the locations the facts were read from; `leagues_held`, the realm's other leagues on disk (`describe` [\"store\"]); `describe` with names [\"pricing\"] says what each count counts. price_list names the items; price_show is one target. Each age is in words beside its time (`describe` with names [\"ages\"])."
    )]
    fn price_status(&self, Parameters(p): Parameters<Args<PriceStatusParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm.unwrap_or(Realm::DEFAULT),
            Err(refusal) => return Ok(*refusal),
        };
        let (store, intent) = match self.open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(*refusal),
        };
        let league = p
            .league
            .as_deref()
            .unwrap_or(acquisition_plan::pricing::DEFAULT_LEAGUE);
        let expand = p.expand.unwrap_or(false);
        priced(
            acquisition_plan::pricing::report(&store, &intent, realm, league).and_then(|r| {
                Held::read(
                    r.status(expand, acquisition_store::now()),
                    &store,
                    &intent,
                    realm,
                    league,
                    expand,
                )
            }),
        )
    }

    #[tool(
        description = "The items of one league that are priced, each with both sides of its listing (no daemon): `items`, ten by default (`limit` more, `expand` all), each with its `subject`, the hand row that applies (`manual`), the game's reading (`game`), the `relation` and the `effective` price; `containers`, the tabs, substashes and characters they sit under; and the counts `describe` with names [\"pricing\"] explains. With neither `relation` nor `effective`, items nothing prices are left out, except unresolved ones."
    )]
    fn price_list(&self, Parameters(p): Parameters<Args<PriceListParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm,
            Err(refusal) => return Ok(*refusal),
        };
        let (store, intent, account) = match self.open_priced(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(*refusal),
        };
        priced(acquisition_plan::pricing::list(
            &store,
            &intent,
            realm,
            p.league.as_deref(),
            &acquisition_plan::pricing::ListArgs {
                relation: p.relation,
                effective: p.effective,
                location: p.location,
                covered_by: p.covered_by,
                game_text: p.game_text,
                limit: p.limit,
                expand: p.expand.unwrap_or(false),
            },
            &account,
        ))
    }

    #[tool(
        description = "One target's listing (no daemon): `listing`, both sides with their causes — the hand row that applies (`manual`, its `revision` and the target it is on, `from`), the note and the tab name verbatim beside their readings — the relation, the effective price and why; `container`, the one an item sits in; for a container, `items_here` (in it) and `items_covered_below` (through its children), ten each beside its counts, `limit` more, `expand` all (`describe` with names [\"pricing\"]; ages, [\"ages\"]). Read it before price_set or price_clear on a target with its own row, whose revision they name."
    )]
    fn price_show(&self, Parameters(p): Parameters<Args<PriceShowParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm,
            Err(refusal) => return Ok(*refusal),
        };
        let (store, intent, account) = match self.open_priced(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(*refusal),
        };
        priced(acquisition_plan::pricing::show(
            &store,
            &intent,
            &p.target,
            realm,
            p.league.as_deref(),
            acquisition_plan::pricing::Bound {
                limit: p.limit,
                expand: p.expand.unwrap_or(false),
            },
            &account,
        ))
    }

    #[tool(
        description = "The forum shop pages for one league, for a person to paste by hand; this server posts nothing and reads no forum (no daemon). One link code per hand-priced item under its price's spoiler, in pages of at most `size` characters: `pages` (each `text` is what to paste; with `page`, that page's fields instead), `posted`, `left_out`, `counts`, `policy` and `freshness`: whether the sync policy covers the posted items, never enforced, and in `facts` how old their locations' facts are. `describe` with names [\"pricing\"] says what each count counts, [\"store\"] what `leagues_held` holds, [\"ages\"] ages."
    )]
    fn shop_render(&self, Parameters(p): Parameters<Args<ShopRenderParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm.unwrap_or(Realm::DEFAULT),
            Err(refusal) => return Ok(*refusal),
        };
        let (store, intent) = match self.open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(*refusal),
        };
        let league = p
            .league
            .as_deref()
            .unwrap_or(acquisition_plan::pricing::DEFAULT_LEAGUE);
        let rendered = acquisition_plan::pricing::shop_render(
            &store,
            &intent,
            self.frontend.provider,
            realm,
            league,
            &acquisition_plan::pricing::ShopArgs {
                size: p.size.unwrap_or(acquisition_plan::shop::DEFAULT_PAGE_SIZE),
                template: p.template.as_deref(),
                now: acquisition_store::now(),
                expand: p.expand.unwrap_or(false),
            },
        );
        let expand = p.expand.unwrap_or(false);
        match p.page {
            Some(n) => priced(rendered.and_then(|r| {
                let page = acquisition_plan::pricing::shop_page_answer(&r, n)?;
                Held::read(page, &store, &intent, realm, league, expand)
            })),
            None => {
                priced(rendered.and_then(|r| Held::read(r, &store, &intent, realm, league, expand)))
            }
        }
    }

    #[tool(
        description = "The currency table this build ships (no store, no daemon): every row — `tag`, the word a price cites; `emit`, the word the game writes in a note; `aliases`, legacy words that resolve to it; `display`; `retired` where the game no longer writes it; `evidence` — with the table's version. `word` resolves one word instead."
    )]
    fn reference_currency(&self, Parameters(p): Parameters<Args<ReferenceCurrencyParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        priced(acquisition_plan::pricing::currency_document(
            p.word.as_deref(),
        ))
    }

    #[tool(
        description = "Set one target's price by hand: one row on an item, tab, substash or character, inherited by what it covers unless a nearer row says otherwise. Local: sends nothing, changes nothing in game. Over a row, `if_revision` names its revision; omit it to create one. Returns the receipt — `target`, `written`, `prior`, `revision`, `undo` (puts `prior` back) or `no_undo` (why none), after a replace `clear`, `in_facts`, and for a container's row price_show's `items_here_counts` and `items_covered_below_counts` after the write; `describe` with names [\"pricing\"] gives the revision rule and the undo."
    )]
    fn price_set(&self, Parameters(p): Parameters<Args<PriceSetParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let (store, mut intent, username) = match self.open_priced(p.account.as_deref(), "account")
        {
            Ok(opened) => opened,
            Err(refusal) => return Ok(*refusal),
        };
        let written = acquisition_plan::pricing::set(
            &store,
            &mut intent,
            &p.target,
            &p.r#type,
            p.amount.as_deref(),
            p.currency.as_deref(),
            acquisition_plan::pricing::Expect::or(
                p.if_revision,
                acquisition_plan::pricing::Expect::Create,
            ),
            &Provenance::via(self.frontend.via),
            &username,
        );
        match written {
            Ok(r) => told(&receipt::price_set(
                &r,
                &username,
                &Spoken { named: "account" },
            )),
            Err(e) => priced::<()>(Err(e)),
        }
    }

    #[tool(
        description = "Remove one target's own price row; what it covered falls back to the next row up, or to the game's price. Local only: it sends nothing. `if_revision` names the revision of the row it removes (price_show reads it). Returns the receipt — `target`, `written` null, `prior`, the row removed, `revision`, the one the clear left, and `undo`, the price_set that puts it back, a create naming no revision, sent as printed (or `no_undo`, why none); on a container's row, its items' counts after the clear, as price_set's."
    )]
    fn price_clear(&self, Parameters(p): Parameters<Args<PriceClearParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        // one account for the intent, the facts and the receipt: a login
        // landing between two resolutions could pair one account's intent
        // with another's facts
        let account = match self.account(p.account.as_deref()) {
            Ok(account) => account,
            Err(e) => return Ok(refusing(refusal(&e.into(), "account"))),
        };
        let (mut intent, username) = match account.intent() {
            Ok(intent) => (intent, account.entry.username.clone()),
            Err(e) => return Ok(refusing(refusal(&e.into(), "account"))),
        };
        // the facts where they can be read, never made: what a container's
        // row covered is counted from them (#171); a clear needs none
        let facts = account.facts_read();
        let written = acquisition_plan::pricing::clear(
            facts.as_ref().map_err(String::as_str),
            &mut intent,
            &p.target,
            acquisition_plan::pricing::Expect::or(
                p.if_revision,
                acquisition_plan::pricing::Expect::Create,
            ),
            &Provenance::via(self.frontend.via),
            &username,
        );
        match written {
            Ok(w) => told(&receipt::price_clear(
                &w,
                &username,
                &Spoken { named: "account" },
            )),
            Err(e) => priced::<()>(Err(e)),
        }
    }

    // ---- the saved search: adapters of acquisition-verbs' saved ----

    #[tool(
        description = "The saved searches, the world's and no one account's (no daemon): ten, `limit` more or fewer, `expand` every one, beside `total`, `left_out` and the file's `revision`; `saved`, an id or a name, is one row. A readable row carries `request`, the search it is; one this build cannot read is named by its id, revision and why. `search` with `saved` runs one. A name two rows share is refused, offering each by id. Each age is in words beside its time (`describe` with names [\"ages\"])."
    )]
    fn saved_searches(&self, Parameters(p): Parameters<Args<SavedSearchesParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        let Some(id_or_name) = p.saved else {
            return settled(saved::list(
                &self.frontend,
                p.limit,
                p.expand.unwrap_or(false),
            ));
        };
        // a row is never a list (C53): the bound of one has nowhere to
        // stand beside it
        if let Some(word) = [("limit", p.limit.is_some()), ("expand", p.expand.is_some())]
            .iter()
            .find_map(|(word, sent)| sent.then_some(word))
        {
            return failed(
                Arguments(format!(
                    "`saved` opens one saved search and `{word}` bounds the list of them: the two \
                     never stand together"
                ))
                .into(),
            );
        }
        settled(saved::show(&self.frontend, &id_or_name))
    }

    #[tool(
        description = "Save a search, or change one; local, it sends nothing (no daemon). No `id`: a create, `name` and `text` required; a create naming no realm takes the one the facts hold, the named `account`'s or every known account's. `id` and `if_revision`: a write over that row at exactly that revision, the fields not named kept (a rename is the two and `name`). `id` and no `if_revision`: a create at the id, the whole search, landing only where none stands (a clear's undo). Returns the receipt — `target`, `written`, `prior`, `revision`, `undo` (puts `prior` back) or `no_undo` (why none)."
    )]
    fn saved_search_set(&self, Parameters(p): Parameters<Args<SavedSearchSetParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        // C52: naming no revision creates, never replaces
        let set = Set {
            id: p.id,
            expect: Expect::or(p.if_revision, Expect::Create),
            name: p.name,
            text: p.text,
            realm: p.realm,
            membership: p.membership,
        };
        match saved::set(&self.frontend, p.account.as_deref(), &set, &mut saved::mint) {
            Ok(written) => told(&receipt::saved(&written, &Spoken { named: "account" })),
            Err(e) => failed(e),
        }
    }

    #[tool(
        description = "Clear one saved search at the revision named; local, it sends nothing (no daemon). Returns the receipt — `target`, `written` null, `prior`, the row cleared, `revision`, the one the clear left, and `undo`, the saved_search_set that puts it back, a create at its id, sent as printed (or `no_undo`, why none). A revision that moved is refused as conflict, naming the one that stands."
    )]
    fn saved_search_clear(&self, Parameters(p): Parameters<Args<SavedSearchClearParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        // a clear is no create: the revision named, exactly
        match saved::clear(&self.frontend, &p.id, Expect::Exactly(p.if_revision)) {
            Ok(written) => told(&receipt::saved(&written, &Spoken { named: "account" })),
            Err(e) => failed(e),
        }
    }

    // ---- daemon jobs ----

    #[tool(
        description = "Submit a job to the daemon and return its id at once; poll job_status, then job_result. A stash job for a tab the account's stored listing of its realm and league does not name is refused, nothing sent. `describe` with names [\"daemon\"] says what the job tools answer."
    )]
    async fn submit_job(&self, Parameters(p): Parameters<Args<SubmitParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        settled(
            async {
                // C51 (the owner's ruling on #246): the job's account chosen
                // before the daemon is reached, so several known and none
                // named is refused with none spawned
                let account = acquisition_verbs::jobs::job_account(
                    &self.frontend,
                    &p.kind,
                    p.account.as_deref(),
                )?;
                let mut client = self.frontend.connect().await?;
                let id = acquisition_verbs::jobs::submit(
                    &self.frontend,
                    &mut client,
                    &p.kind,
                    p.params.unwrap_or_else(|| json!({})),
                    p.priority.unwrap_or(0),
                    account.as_deref(),
                )
                .await?;
                Ok(json!({ "job_id": id }))
            }
            .await,
        )
    }

    #[tool(
        description = "Jobs the daemon lists, with states and ETAs, counted first over every job (`total`, `by_state`), then listed: the ten newest open and ten newest finished, `state` narrowing, `expand` every one. Observes only, never spawning a daemon: with none running it answers from the ledger (`running: false`, `persisted`); `describe` with names [\"daemon\"] says what each holds and counts, and [\"ages\"] what an age's words say."
    )]
    async fn list_jobs(&self, Parameters(p): Parameters<Args<ListJobsParams>>) -> Told {
        let read = p.parse().and_then(|p| {
            let state = match p.state.as_deref() {
                None => ListState::All,
                Some(word) => acquisition_verbs::list_state(word).map_err(Arguments)?,
            };
            Ok((state, p.expand.unwrap_or(false)))
        });
        let (state, expand) = match read {
            Ok(read) => read,
            Err(e) => return failed(e),
        };
        settled(async { Ok(self.frontend.jobs(state, expand).await?) }.await)
    }

    #[tool(
        description = "One job's state, ETA and retries, and its times (`submitted_at`, `started_at`, `finished_at`, each absent until it happens, its age in words beside it). A waiting job with a large ETA is the rate limiter holding, not a hang (holds can reach about 5 minutes). Observes only: a finished job, or any job with no daemon running, is answered from the ledger on disk (`describe` with names [\"daemon\"]; [\"ages\"] for an age's words)."
    )]
    async fn job_status(&self, Parameters(p): Parameters<Args<JobParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        settled(async { Ok(self.frontend.job(p.id).await?) }.await)
    }

    #[tool(
        description = "A finished job's outcome — payload or error — with its job_id, account and times (`submitted_at`, `started_at`, `finished_at`, each with its `_age`); a failure's `kind` and the `offers` it leaves (`describe` with names [\"daemon\"] lists the kinds), from the daemon or, with none running, the ledger on disk; an unfinished one is refused. A parent's failure carries `children` and `failed_children`. Payloads can be large — prefer the store tools for reading data."
    )]
    async fn job_result(&self, Parameters(p): Parameters<Args<JobParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        // C165 and C143: a failure's way on as this server's tool calls
        let spoken = Spoken { named: "account" };
        settled(async { Ok(self.frontend.result(p.id).await?.offered(&spoken)) }.await)
    }

    #[tool(description = "Cancel a waiting or running job; cascades to its descendants.")]
    async fn cancel_job(&self, Parameters(p): Parameters<Args<JobParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(*refusal),
        };
        settled(async { Ok(self.frontend.cancel(p.id).await?) }.await)
    }

    #[tool(
        description = "The daemon running: its identity — contract, executable and hash, the world it serves, its socket — which of the four match this server (`contract_matches`, `artifact_matches`, `provider_matches`, `world_matches`; `wanted` is this server's own) and, when compatible, its vitals. Observes only: `running: false` with no daemon, `compatible: false` for one of another contract, artifact, provider or world (never replaced here), `responsive: false` for a wedged one; `describe` with names [\"daemon\"] says what each holds, and [\"ages\"] what an age's words say."
    )]
    async fn daemon_status(&self, Parameters(p): Parameters<Args<NoParams>>) -> Told {
        if let Err(refusal) = p.read() {
            return Ok(*refusal);
        }
        settled(async { Ok(self.frontend.daemon().await?) }.await)
    }
}

#[tool_handler]
impl ServerHandler for AcqMcp {
    /// Every call, served with the call itself at hand ([`CALLED`]), so
    /// a refusal can offer it again as each account known.
    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResponse, ErrorData> {
        let called = Called {
            tool: request.name.to_string(),
            arguments: request.arguments.clone().unwrap_or_default(),
        };
        let call = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        CALLED.scope(called, Self::tool_router().call(call)).await
    }

    fn get_info(&self) -> ServerConfig {
        let mut info = ServerConfig::default();
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.instructions = Some(format!(
            "Acquisition (provider: {}).\n\
             - price_set, price_clear, set_sync_policy, saved_search_set and saved_search_clear \
             write locally and send nothing.\n\
             - submit_job, apply_plan and cancel_job talk to the daemon; in real mode a human \
             starts it and this server never spawns one.\n\
             - list_jobs, job_status, job_result and daemon_status observe, and answer without \
             one: the jobs from the ledger on disk.\n\
             - Every other tool reads locally, no daemon; refresh_plan adds a running daemon's \
             quote, never starting one.\n\
             - Begin with search and no arguments; where the store holds several realms, name \
             scope.realm (the refusal offers each).\n\
             - describe with no names is the index; names read entries whole, fields first; all \
             is over 150 KB.\n\
             - A refusal is marked isError: error, kind, and readings, next and offers where \
             it has them; any request an offer or an answer holds is sent back as printed, to \
             the tool it names or the same one.\n\
             - No tool says what an item is worth: a price is the owner's own listing.",
            self.frontend.provider,
        ));
        info
    }
}
