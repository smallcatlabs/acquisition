//! The dispatch: the context a verb runs in ([`Ctx`]), the daemon
//! connections it takes, and the one match from a parsed command to the
//! verb that answers it. A verb with a module of its own (`*_cmd.rs`,
//! `web/`) is called from here; the rest are answered inline.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use acquisition_client::client::{Client, DaemonError, Observed};
use acquisition_protocol::job::Outcome;
use acquisition_protocol::protocol::{ErrorKind, KeyringOutcome, Request, Response};
use acquisition_protocol::realm::Realm;
use acquisition_store::world::{Lock, RailsState, World};
use acquisition_verbs::jobs::result_live;
use acquisition_verbs::{Account, DaemonReport, Fault, Frontend, Job, Jobs};
use anyhow::{Result, bail};
use serde_json::json;

use crate::auth_cmd::{login, print_auth};
use crate::cli::{
    AccountsCmd, AuthCmd, Cli, Cmd, DaemonCmd, PolicyCmd, PriceCmd, ReferenceCmd, SavedCmd,
    ShopCmd, StoreCmd,
};
use crate::daemon_cmd::daemon_text;
use crate::jobs_cmd::{
    block_on_job, jobs_text, persisted_queue_text, print_outcome, print_persisted_job,
    print_result, print_table, submit, times_said, wait_for_job, watch_jobs,
    watch_table_until_done,
};
use crate::note_spawn;
use crate::terminal::AlreadyReported;
use crate::{
    dash, diagnose_cmd, leagues_cmd, plan_cmd, price_cmd, reference_cmd, saved_cmd, search_cmd,
    shop_cmd, store_cmd, web,
};

/// Who this process is and which account its flags select (C46: the
/// layer's values in place of process statics; C121). Built once in
/// `run`, after the environment is checked, and handed to every verb.
pub(crate) struct Ctx {
    pub frontend: Frontend,
    /// `--account` / `ACQ_ACCOUNT` as typed: a job verb passes it to the
    /// daemon unresolved (C3), a verb that opens a store resolves it.
    pub account: Option<String>,
}

impl Ctx {
    /// C121: the account the flags select, resolved against the index;
    /// C187: where it was taken as the main account, several being known
    /// and none named, a note on stderr says which ([`Ctx::note_main`]) —
    /// the search's answer says it itself, so the search resolves through
    /// the frontend.
    pub fn account(&self) -> Result<Account, Fault> {
        let account = self.frontend.account(self.account.as_deref())?;
        self.note_main(Some(&account));
        Ok(account)
    }

    /// The selector a job verb names to the daemon (C187): the flags',
    /// else the main account's uuid where it is taken, noted as
    /// [`Ctx::account`] notes it.
    pub fn daemon_selector(&self) -> Result<Option<String>, Fault> {
        let (selector, main) = self.frontend.daemon_selector(self.account.as_deref())?;
        self.note_main(main.as_ref());
        Ok(selector)
    }

    /// C187's one line, in text mode and under `--json` alike, where the
    /// answer's JSON does not carry `main_account` (the CLI prints a
    /// verb's JSON at many sites, so the field rides only the search's):
    /// on stderr, where the CLI's notes go.
    pub(crate) fn note_main(&self, account: Option<&Account>) {
        if let Some(account) = account.filter(|a| a.main) {
            note_taken(&account.entry.username);
        }
    }
}

/// C187's one line, the CLI's one wording of it: on stderr, where the
/// CLI's notes go.
pub(crate) fn note_taken(name: &str) {
    eprintln!("account {name}: the main account, taken since none was named");
}

/// C187: the one line for a verb whose answer says it took the main
/// account itself (the search's scope line and `main_account`), written
/// only where that answer never came: any failure after the account was
/// taken, so a refusal is never left without it (the second cold review
/// of #246, finding 3). Dropped unanswered, it writes the line.
pub(crate) struct SaidUnlessAnswered {
    name: Option<String>,
}

impl SaidUnlessAnswered {
    /// A guard over `taken`, the verbs' `main_account` or none.
    pub(crate) fn new(taken: Option<&serde_json::Value>) -> Self {
        SaidUnlessAnswered {
            name: taken.and_then(|t| t["name"].as_str()).map(str::to_string),
        }
    }

    /// The answer was printed and says it: no line.
    pub(crate) fn answered(mut self) {
        self.name = None;
    }
}

impl Drop for SaidUnlessAnswered {
    fn drop(&mut self) {
        if let Some(name) = &self.name {
            note_taken(name);
        }
    }
}

/// A use verb's connect (C10): the layer's human door — lazy spawn, and
/// a daemon of another contract, artifact or provider replaced (C84) —
/// noting a spawn for the refusal that follows one.
pub(crate) async fn connect(cx: &Ctx) -> Result<Client> {
    let client = cx.frontend.connect().await?;
    note_spawn(&client);
    Ok(client)
}

/// The running daemon, for a verb that observes it or acts on it (C10):
/// never spawns or replaces. Absence and a mismatch are faults the CLI
/// words; `daemon status` renders the same observation as states, and
/// `jobs` answers absence from the ledger on disk (C45).
pub(crate) async fn attach(cx: &Ctx) -> Result<Client> {
    Ok(cx.frontend.attach().await?)
}

pub(crate) async fn run(cli: Cli) -> Result<()> {
    // C88: a leftover `ACQ_GGG` or an unknown provider word is refused
    // before any verb reads the provider — loudly, never the wrong mode.
    let cx = Ctx {
        frontend: Frontend::human(plan_cmd::WRITTEN_VIA)?,
        account: cli.account.clone(),
    };
    let cx = &cx;
    match cli.cmd {
        Cmd::Auth { cmd, no_browser } => match cmd {
            None => login(cx, no_browser, cli.json).await,
            Some(AuthCmd::Status) => {
                let Some(mut client) = cx.frontend.observe().await? else {
                    return store_cmd::auth_status_absent(cx, cli.json);
                };
                let status = client.request(&Request::AuthStatus {}).await?;
                print_auth(&status, cli.json)?;
                if !cli.json {
                    store_cmd::print_other_accounts(
                        cx,
                        match &status {
                            Response::Auth { accounts, .. } => {
                                accounts.iter().map(|a| a.username.clone()).collect()
                            }
                            _ => Vec::new(),
                        },
                    )?;
                }
                Ok(())
            }
            Some(AuthCmd::Check) => {
                let mut client = connect(cx).await?;
                // C187: naming none among several, the main account
                let account = cx.daemon_selector()?;
                match client.request(&Request::AuthCheck { account }).await? {
                    Response::Error { kind, message } => {
                        Err(anyhow::Error::from(DaemonError { kind, message })
                            .context("auth check failed"))
                    }
                    status => {
                        if !cli.json {
                            println!("session verified (live token round-trip succeeded)");
                        }
                        print_auth(&status, cli.json)
                    }
                }
            }
            Some(AuthCmd::Logout) => {
                let mut client = attach(cx).await?;
                // C187: a logout takes no main account; ending a session
                // or forgetting a login is an act a person names (the
                // ruling's one changed command is a job; the second cold
                // review of #246, finding 2, declined)
                let account = cx.account.clone();
                let answer = client
                    .request(&Request::AuthLogout {
                        account: account.clone(),
                    })
                    .await?;
                match answer {
                    // C144: what the keyring did is the answer's field,
                    // never read from a sentence. An older daemon's bare
                    // ack carries none: unknown, and said so.
                    Response::Ack { keyring } => {
                        let (word, said) = match keyring {
                            Some(KeyringOutcome::Cleared) => ("cleared", "keyring entry cleared"),
                            Some(KeyringOutcome::Off) => ("off", "keyring off, nothing cleared"),
                            Some(KeyringOutcome::Failed) => {
                                ("failed", "the keyring entry could not be cleared")
                            }
                            None => ("unknown", "the daemon did not say what the keyring did"),
                        };
                        let failed = keyring == Some(KeyringOutcome::Failed);
                        if cli.json {
                            println!(
                                "{}",
                                json!({ "logged_out": true, "account": account, "keyring": word })
                            );
                        } else if failed {
                            eprintln!("logged out, but {said}");
                        } else {
                            println!("logged out ({said})");
                        }
                        if failed {
                            return Err(AlreadyReported.into());
                        }
                        Ok(())
                    }
                    // A failed clear or an index the daemon could not
                    // write: its sentence says what was and was not done.
                    Response::Error { kind, message } => {
                        Err(anyhow::Error::from(DaemonError { kind, message }))
                    }
                    other => anyhow::bail!("unexpected response: {other:?}"),
                }
            }
        },
        Cmd::Web { no_browser } => web::web(cli.account.clone(), no_browser, cli.json).await,
        Cmd::Profile => {
            let mut client = connect(cx).await?;
            let id = submit(cx, &mut client, "profile".into(), json!({}), 0).await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Characters { realm } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "characters".into(),
                json!({ "realm": realm }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Character { name, realm } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "character".into(),
                character_params(realm, &name),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Leagues {
            stored: true,
            expand,
        } => leagues_cmd::stored(cx, expand, cli.json),
        Cmd::Leagues { stored: false, .. } => {
            let mut client = connect(cx).await?;
            let id = submit(cx, &mut client, "leagues".into(), json!({}), 0).await?;
            if cli.json {
                return block_on_job(&mut client, id, true).await;
            }
            wait_for_job(&mut client, id, false).await?;
            let result = result_live(&mut client, id).await?;
            match &result.outcome {
                Outcome::Success { payload } => {
                    print!("{}", store_cmd::leagues_text(payload));
                    Ok(())
                }
                _ => print_outcome(&result, false, true),
            }
        }
        Cmd::Accounts { expand, cmd } => match cmd {
            None => store_cmd::accounts(cx, expand, cli.json),
            Some(AccountsCmd::Main {
                account,
                clear,
                if_revision,
            }) => store_cmd::main_account(cx, account.as_deref(), clear, if_revision, cli.json),
        },
        Cmd::Tabs {
            league,
            realm,
            limit,
            expand,
            r#type,
            never_fetched,
        } => store_cmd::tabs(
            cx,
            realm,
            &league,
            &acquisition_search::tabs::Selection {
                kind: r#type,
                never_fetched,
            },
            limit,
            expand,
            cli.json,
        ),
        Cmd::Search(args) => search_cmd::search(cx, *args, cli.json),
        Cmd::Show(args) => search_cmd::show_item(cx, args, cli.json),
        Cmd::Store { cmd } => match cmd {
            StoreCmd::Status => store_cmd::status(cx, cli.json),
            StoreCmd::Characters {
                realm,
                league,
                limit,
                expand,
            } => store_cmd::characters(cx, realm, league.as_deref(), limit, expand, cli.json),
            StoreCmd::Events {
                hours,
                limit,
                expand,
                summary,
            } => store_cmd::events(cx, hours, limit, expand, summary, cli.json),
            StoreCmd::Refused { id, limit } => store_cmd::refused(cx, id, limit, cli.json),
            StoreCmd::Rebuild => store_cmd::rebuild(cx, cli.json),
            StoreCmd::Import { path } => store_cmd::import(cx, &path, cli.json),
        },
        Cmd::Stashes { league, realm } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "stashes".into(),
                json!({ "realm": realm, "league": league }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Stash {
            id,
            sub,
            deep,
            league,
            realm,
        } => {
            let mut client = connect(cx).await?;
            let id = submit(
                cx,
                &mut client,
                "stash".into(),
                json!({ "realm": realm, "league": league, "id": id, "sub": sub, "deep": deep }),
                0,
            )
            .await?;
            block_on_job(&mut client, id, cli.json).await
        }
        Cmd::Refresh {
            all,
            tabs,
            characters,
            deep,
            plan,
            expand,
            apply,
            max_requests,
            league,
            realm,
        } => {
            if let Some(source) = plan {
                // Bare `--plan` compiles the stored policy; `=FILE` renders
                // a reviewed envelope.
                let source = (!source.is_empty()).then_some(source);
                return plan_cmd::refresh_plan(
                    cx,
                    realm.unwrap_or(Realm::DEFAULT),
                    league
                        .as_deref()
                        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE),
                    source.as_deref(),
                    cli.json,
                    expand,
                )
                .await;
            }
            if let Some(source) = apply {
                // Bare `--apply` (clap's empty default_missing_value)
                // means "compile the stored policy now and run that".
                let source = (!source.is_empty()).then_some(source);
                return plan_cmd::refresh_apply(
                    cx,
                    realm,
                    league.as_deref(),
                    source.as_deref(),
                    max_requests,
                    cli.json,
                )
                .await;
            }
            if !all && tabs.is_empty() && characters.is_empty() {
                bail!(
                    "refresh needs --all, --tabs <id,...>, --characters <id,...>, --plan, or --apply"
                );
            }
            let tabs = if all {
                Some(acquisition_plan::Selection::All)
            } else {
                (!tabs.is_empty()).then_some(acquisition_plan::Selection::Ids(tabs))
            };
            plan_cmd::refresh_selection(
                cx,
                realm.unwrap_or(Realm::DEFAULT),
                league
                    .as_deref()
                    .unwrap_or(acquisition_plan::DEFAULT_LEAGUE),
                acquisition_plan::PlanSelection {
                    tabs,
                    characters,
                    deep,
                },
                cli.json,
            )
            .await
        }
        Cmd::Price { cmd } => match cmd.unwrap_or(PriceCmd::Status {
            league: acquisition_plan::pricing::DEFAULT_LEAGUE.into(),
            realm: None,
            expand: false,
        }) {
            PriceCmd::Status {
                league,
                realm,
                expand,
            } => price_cmd::status(
                cx,
                realm.unwrap_or(Realm::DEFAULT),
                &league,
                expand,
                cli.json,
            ),
            PriceCmd::Show {
                target,
                league,
                realm,
                limit,
                expand,
            } => price_cmd::show(
                cx,
                &target,
                realm,
                league.as_deref(),
                acquisition_plan::pricing::Bound { limit, expand },
                cli.json,
            ),
            PriceCmd::List {
                league,
                realm,
                relation,
                effective,
                location,
                covered_by,
                game_text,
                limit,
                expand,
            } => price_cmd::list(
                cx,
                realm,
                league.as_deref(),
                &acquisition_plan::pricing::ListArgs {
                    relation,
                    effective,
                    location,
                    covered_by,
                    game_text,
                    limit,
                    expand,
                },
                expand,
                cli.json,
            ),
            PriceCmd::Set {
                target,
                kind,
                amount,
                currency,
                if_revision,
            } => price_cmd::set(
                cx,
                &target,
                &kind,
                amount.as_deref(),
                currency.as_deref(),
                if_revision,
                cli.json,
            ),
            PriceCmd::Clear {
                target,
                if_revision,
            } => price_cmd::clear(cx, &target, if_revision, cli.json),
        },
        Cmd::Reference { cmd } => match cmd {
            ReferenceCmd::Currency { word, expand } => {
                reference_cmd::currency(word.as_deref(), expand, cli.json)
            }
        },
        Cmd::Shop { cmd } => match cmd {
            ShopCmd::Render {
                league,
                realm,
                size,
                template,
                page,
                expand,
            } => shop_cmd::render_cmd(
                cx,
                realm.unwrap_or(Realm::DEFAULT),
                &league,
                &shop_cmd::RenderArgs {
                    size,
                    template: template.as_deref(),
                    page,
                    expand,
                    json: cli.json,
                },
            ),
        },
        Cmd::Saved { cmd } => match cmd.unwrap_or(SavedCmd::List {
            limit: None,
            expand: false,
        }) {
            SavedCmd::List { limit, expand } => saved_cmd::list(cx, limit, expand, cli.json),
            SavedCmd::Show { search } => saved_cmd::show(cx, &search, cli.json),
            SavedCmd::Set {
                id,
                if_revision,
                name,
                query,
                realm,
                membership,
            } => saved_cmd::set(
                cx,
                id,
                if_revision,
                name,
                query,
                realm,
                membership,
                cli.json,
            ),
            SavedCmd::Clear { id, if_revision } => saved_cmd::clear(cx, &id, if_revision, cli.json),
        },
        Cmd::Policy { cmd } => match cmd {
            None => plan_cmd::policy_show(cx, false, cli.json),
            Some(PolicyCmd::Show { expand }) => plan_cmd::policy_show(cx, expand, cli.json),
            Some(PolicyCmd::Set {
                value,
                if_revision,
                replace,
            }) => plan_cmd::policy_set(cx, &value, if_revision, replace, cli.json),
        },
        Cmd::Submit {
            kind,
            params,
            priority,
            detach,
        } => {
            let params: serde_json::Value = serde_json::from_str(&params)?;
            let mut client = connect(cx).await?;
            let id = submit(cx, &mut client, kind, params, priority).await?;
            if detach {
                if cli.json {
                    println!("{}", json!({ "job_id": id }));
                } else {
                    println!(
                        "job {id} submitted (`acq status {id}` reads its state, `acq result {id} --wait` collects it)"
                    );
                }
                Ok(())
            } else {
                block_on_job(&mut client, id, cli.json).await
            }
        }
        Cmd::Demo { count } => {
            let mut client = connect(cx).await?;
            let mut ids = Vec::new();
            for i in 0..count {
                let id = submit(
                    cx,
                    &mut client,
                    "fetch".into(),
                    json!({ "what": format!("demo stash tab {i}") }),
                    0,
                )
                .await?;
                ids.push(id);
            }
            println!("submitted {count} fetch jobs: {ids:?}");
            println!("(mock policy: 5 per 10s, then 30 per 300s — watch the ETAs)\n");
            watch_table_until_done(&mut client, &ids).await
        }
        Cmd::Dash => dash::run(cx, cli.json).await,
        Cmd::Diagnose { lines } => diagnose_cmd::run(cx, lines, cli.json).await,
        Cmd::Version => {
            // The candidate, not the daemon running (C84): the sibling
            // `acqd` as found, with no hash — the run record hashes it.
            if cli.json {
                println!("{}", acquisition_verbs::diagnose::version_document());
            } else {
                let sibling = acquisition_client::artifact::sibling();
                println!(
                    "acq {}, provider {}",
                    acquisition_protocol::VERSION_WITH_CONTRACT,
                    acquisition_protocol::provider::wanted()
                );
                match sibling {
                    Ok(s) => println!(
                        "acqd: {} ({} bytes) — what a job command would start; `acq daemon status` reports the daemon running",
                        s.identity.path, s.identity.len
                    ),
                    Err(e) => println!("acqd: none — {e}"),
                }
            }
            Ok(())
        }
        Cmd::Jobs { watch, all, state } => {
            if watch {
                return watch_jobs(cx, cli.json, state, all).await;
            }
            let jobs = cx.frontend.jobs(state, all).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&jobs)?);
                return Ok(());
            }
            match &jobs {
                Jobs::Live(listed) => print!("{}", jobs_text(listed)),
                // C45: no daemon, the persisted queue
                Jobs::Absent { persisted } => {
                    print!("{}", persisted_queue_text(persisted.as_ref())?)
                }
            }
            Ok(())
        }
        Cmd::Status { id } => {
            let job = match cx.frontend.job(id).await? {
                Job::Live(job) => job,
                absent => return print_persisted_job(id, &absent, cli.json),
            };
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&job)?);
            } else {
                print_table(std::slice::from_ref(&job));
                if let Some(words) = times_said(&job.times) {
                    println!("job {id}: {words}");
                }
            }
            Ok(())
        }
        Cmd::Result { id, wait } => {
            let Some(mut client) = cx.frontend.observe().await? else {
                // C45: a terminal job's outcome is final, so the ledger's
                // prints exactly as a daemon would answer it.
                return print_outcome(&cx.frontend.result_on_disk(id)?, cli.json, false);
            };
            // Only a job still in the queue is waited for: a previous
            // lifetime's result is served from retention (C45) and has no
            // status to wait on.
            let waited = wait
                && matches!(
                    client.request(&Request::Result { id }).await?,
                    Response::Error {
                        kind: ErrorKind::WrongState,
                        ..
                    }
                );
            if waited {
                wait_for_job(&mut client, id, cli.json).await?;
            }
            print_result(&mut client, id, cli.json, waited).await
        }
        Cmd::Cancel { id } => {
            let asked = cx.frontend.cancel(id).await?;
            if cli.json {
                println!("{}", serde_json::to_string(&asked)?);
            } else {
                println!(
                    "job {id} cancel requested — a running job stops at its next check; \
                     `acq result {id} --wait` returns once it has"
                );
            }
            Ok(())
        }
        Cmd::SetPriority { id, priority } => {
            let mut client = attach(cx).await?;
            client
                .expect_ack(&Request::SetPriority { id, priority })
                .await?;
            if cli.json {
                println!("{}", json!({ "job_id": id, "priority": priority }));
            } else {
                println!("job {id} priority -> {priority}");
            }
            Ok(())
        }
        Cmd::Daemon { cmd } => match cmd {
            DaemonCmd::Status => {
                // An observation (C10): absent, this build's, or another
                // daemon reported and left alone — never spawned or
                // replaced. The document is the layer's (C45, C84, C86).
                let report = cx.frontend.daemon().await?;
                if cli.json {
                    match &report {
                        DaemonReport::Absent { .. } => println!("{}", report.document()),
                        _ => println!("{}", serde_json::to_string_pretty(&report.document())?),
                    }
                    return Ok(());
                }
                print!("{}", daemon_text(&report));
                Ok(())
            }
            DaemonCmd::ResetTripwire => {
                match Client::observe_at(&cx.frontend.root).await? {
                    Observed::Compatible(mut client) => {
                        let resp = client.request(&Request::ResetTripwire {}).await?;
                        // A reset the daemon could not write is refused,
                        // the halt standing; never reported done.
                        if let Response::Error { kind, message } = resp {
                            return Err(DaemonError { kind, message }.into());
                        }
                        if cli.json {
                            println!("{}", serde_json::to_string_pretty(&resp)?);
                        } else {
                            println!(
                                "rails reset; the next send goes to GGG unless this daemon's send ceiling is met, which only a new daemon lifts"
                            );
                        }
                    }
                    // Another daemon holds its rails in memory; clearing the
                    // file under it would not reset anything. Stop it first.
                    Observed::Incompatible(found) => {
                        bail!("{found}; `acq daemon stop` first")
                    }
                    Observed::Unresponsive(found) => bail!("{found}"),
                    Observed::Absent => {
                        // The trip lives on disk, in the world (C83); clear
                        // it so the next spawned daemon is not still
                        // halted. No world, nothing persisted. The file
                        // has other owners: a readable one is edited, its
                        // trip and block mark cleared and its dead-grant
                        // marks kept (C24), in the store's one format
                        // (`RailsState`; C1, no frontend links the daemon).
                        let provider = cx.frontend.provider;
                        let world = World::observe_at(&cx.frontend.root).ok();
                        // The world lock (C83), held across the read and
                        // the write: no daemon can start between them,
                        // load the state and have its marks overwritten.
                        // Taken, it is also the proof no daemon runs here.
                        // Only a held lock means a daemon; a lock that
                        // could not be opened says what failed.
                        let _lock = match &world {
                            Some(world) => Some(Lock::acquire(&world.lock_path()).map_err(|e| {
                                if e.io.is_some() {
                                    anyhow::anyhow!("{e}; the reset cleared nothing")
                                } else {
                                    anyhow::anyhow!(
                                        "a daemon took this world while the reset ran: {e}; `acq daemon reset-tripwire` again asks it"
                                    )
                                }
                            })?),
                            None => None,
                        };
                        let cleared: Option<(std::path::PathBuf, bool)> = match world
                            .map(|world| world.rails_state_path(provider))
                        {
                            Some(state) => clear_rails_state(&state)?.map(|unread| (state, unread)),
                            None => None,
                        };
                        if cli.json {
                            println!(
                                "{}",
                                json!({
                                    "cleared": cleared.is_some(),
                                    "state": cleared.as_ref().map(|(state, _)| state),
                                    "unreadable": cleared.as_ref().is_some_and(|(_, unread)| *unread),
                                })
                            );
                        } else {
                            match cleared {
                                None => {
                                    println!("daemon is not running and no rails halt is persisted")
                                }
                                Some((state, true)) => println!(
                                    "daemon is not running; removed {}, which could not be read as rails state: whatever it held is gone, a halt or dead-grant marks alike (the next daemon starts clear)",
                                    state.display()
                                ),
                                Some((state, false)) => println!(
                                    "daemon is not running; cleared the rails halt persisted in {} (the next daemon's next send goes to GGG)",
                                    state.display()
                                ),
                            }
                        }
                    }
                }
                Ok(())
            }
            DaemonCmd::Stop => {
                // A session the keyring does not hold lives in the daemon
                // alone, so stopping it is a logout: asked of this build's
                // daemon before it goes, and said after. Best effort — a
                // daemon that will not answer is stopped all the same.
                let mut ended: Vec<String> = Vec::new();
                if let Ok(Observed::Compatible(mut client)) =
                    Client::observe_at(&cx.frontend.root).await
                    && let Ok(Response::Auth { accounts, .. }) =
                        client.request(&Request::AuthStatus {}).await
                {
                    ended = accounts
                        .into_iter()
                        .filter(|a| a.keyring != "ok")
                        .map(|a| a.username)
                        .collect();
                }
                // Stops this build's daemon or any other (C10): stopping is
                // how a mismatch is resolved by hand.
                match Client::stop_any_at(&cx.frontend.root).await? {
                    Some(found) => {
                        if cli.json {
                            println!(
                                "{}",
                                json!({
                                    "stopped": true,
                                    "pid": found.pid(),
                                    "version": found.version(),
                                    "provider": found.provider(),
                                    "compatible": found.is_ours(),
                                    "socket": found.socket(),
                                    "sessions_ended": ended,
                                })
                            );
                        } else if found.is_ours() {
                            println!("daemon stopped (pid {})", found.pid());
                            if !ended.is_empty() {
                                println!(
                                    "logged out with it: {} — not in the keyring, so the session \
                                     lived in that daemon alone; `acq auth` logs in again",
                                    ended.join(", ")
                                );
                            }
                        } else {
                            println!("stopped: {found}");
                        }
                    }
                    None => {
                        if cli.json {
                            println!("{}", json!({ "stopped": false, "running": false }));
                        } else {
                            println!("daemon is not running");
                        }
                    }
                }
                Ok(())
            }
        },
    }
}

/// The `character` job's params, as `acq character` submits them.
pub(crate) fn character_params(realm: Realm, name: &str) -> serde_json::Value {
    json!({ "realm": realm, "name": name })
}

/// `daemon reset-tripwire` with no daemon running: the rails state
/// persisted at `state` reset, or removed where it cannot be read. A
/// file it cannot read, or an empty directory in a file's place, is
/// removed (the daemon refuses to start over either shape, and names
/// this verb as the remedy); anything else is named for the hand. None
/// where there is nothing to clear (no file, or only the dead-grant
/// marks, which a reset keeps); else whether it was removed unread
/// rather than reset.
fn clear_rails_state(state: &std::path::Path) -> Result<Option<bool>> {
    let removed = match std::fs::symlink_metadata(state) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => Err(e),
        Ok(meta) if meta.is_file() => match std::fs::read_to_string(state)
            .ok()
            .and_then(|text| serde_json::from_str::<RailsState>(&text).ok())
        {
            Some(mut held) => {
                // Nothing a reset clears: the
                // dead-grant marks stay.
                if !held.reset() {
                    return Ok(None);
                }
                held.write(state).map(|()| false)
            }
            None => std::fs::remove_file(state).map(|()| true),
        },
        Ok(meta) if meta.is_dir() => std::fs::remove_dir(state).map(|()| true).map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!("it is a directory that is not empty ({e}); remove it by hand"),
            )
        }),
        Ok(_) => std::fs::remove_file(state).map(|()| true),
    };
    match removed {
        Ok(unread) => Ok(Some(unread)),
        Err(e) => bail!(
            "daemon is not running; could not clear {}: {e}",
            state.display()
        ),
    }
}
