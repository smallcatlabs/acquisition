//! The connection and the lifecycle: a connection's frames, each request,
//! the status documents, the rails reset, the idle verdict and watchdog,
//! and the exit. C148: `hello` is read and answered under no lock, the
//! connection count and the activity stamp being atomics; a logout's
//! landing runs on a blocking thread; the drain counts every landing
//! until its keyring and index I/O ends, and a login being recorded.
//! C148, the start's restore as the fifth landing: the socket serves
//! while it reads the keyring; `hello`, `daemon_stop`, the status
//! documents (their keyring word `restoring`), `list`, `status`,
//! `result`, `cancel`, `set_priority` and `subscribe` answer at once;
//! a submit that needs a session, a quote, `auth`, `auth check` and
//! `logout` wait for it, never refused; the idle verdict holds while any
//! landing is in flight (the restore, a save, a rejected token's clear).
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::life::{Effect, Life, Timer};
use super::*;

impl Daemon {
    fn session_statuses(&self, s: &Shared) -> Vec<SessionStatus> {
        let wall = self.choke.wall();
        let mut v: Vec<SessionStatus> = s
            .auth
            .by_account
            .values()
            .map(|session| SessionStatus {
                username: session.username.clone().unwrap_or_default(),
                access_expires_in_seconds: session
                    .access_expires_at
                    .map(|t| t.duration_since(wall).unwrap_or_default().as_secs()),
                keyring: session.keyring.clone(),
            })
            .collect();
        v.sort_by(|a, b| a.username.cmp(&b.username));
        v
    }

    /// The account reported as "the" username: the most recent login while
    /// it is live, else the sole session. Informational only.
    fn headline_session<'a>(&self, s: &'a Shared) -> Option<&'a AuthSession> {
        s.auth
            .last_login
            .as_ref()
            .and_then(|u| s.auth.find(u))
            .or_else(|| s.auth.get(None).ok())
    }

    /// The one keyring word the three status documents print: the
    /// headline session's mark, else the daemon's. C148: `restoring`
    /// while the start's restore reads the keyring, as `saving` is a
    /// refresh's: a field's word, the documents' shape unchanged.
    pub(super) fn keyring_summary(&self, s: &Shared) -> String {
        if !self.restore_landed() {
            return "restoring".to_string();
        }
        self.headline_session(s)
            .map(|session| session.keyring.clone())
            .unwrap_or_else(|| s.auth.keyring.clone())
    }

    pub(super) fn auth_status(&self) -> Response {
        let s = self.shared.lock().unwrap();
        let head = self.headline_session(&s);
        Response::Auth {
            logged_in: !s.auth.by_account.is_empty(),
            pending: s.auth.pending.is_some(),
            login_ok: s
                .auth
                .flow_result
                .as_ref()
                .and_then(|r| r.as_ref().ok().cloned()),
            login_error: s
                .auth
                .flow_result
                .as_ref()
                .and_then(|r| r.as_ref().err().cloned()),
            username: head.and_then(|h| h.username.clone()),
            access_expires_in_seconds: head.and_then(|h| h.access_expires_at).map(|t| {
                t.duration_since(self.choke.wall())
                    .unwrap_or_default()
                    .as_secs()
            }),
            keyring: self.keyring_summary(&s),
            provider: self.provider.name.to_string(),
            accounts: self.session_statuses(&s),
        }
    }

    // ---- connection handling -------------------------------------------

    /// One connection, frame by frame (C85). Every frame is read under
    /// the bound, then tried on the bootstrap plane first — `hello` and
    /// `daemon_stop` are answered on any connection in any state, so a
    /// client of another revision can identify this daemon and a human
    /// can stop it — and only then parsed as a versioned request. A frame
    /// that is oversize, not JSON, or not a request of this version is
    /// answered `bad_request` and the connection stays; so is a
    /// versioned request on a connection that has subscribed. Frames are
    /// answered in order, one at a time; a subscribed connection also
    /// carries `event` frames, and `resync_required` when the event
    /// channel overran it.
    /// C148: the count and the stamp are atomics, so `hello` is read
    /// and answered under no lock.
    pub(super) async fn handle_conn(self: Arc<Self>, stream: Stream) {
        self.connections.admit();
        self.mark_activity(self.choke.now());

        let (read, mut write) = tokio::io::split(stream);
        let mut reader = BufReader::new(read);
        let mut events: Option<broadcast::Receiver<JobInfo>> = None;
        // Every connection begins with the hello exchange (C85): a
        // versioned request before it is refused, so a peer that never
        // identified itself performs nothing (review round 1).
        let mut greeted = false;

        loop {
            tokio::select! {
                frame = read_frame(&mut reader, MAX_FRAME_BYTES) => {
                    let frame = match frame {
                        Ok(Frame::Closed) | Err(_) => break,
                        Ok(frame) => frame,
                    };
                    let bytes = match frame {
                        Frame::Oversize => {
                            let response = Refusal::new(
                                ErrorKind::BadRequest,
                                format!("frame exceeds {MAX_FRAME_BYTES} bytes; discarded through its newline"),
                            );
                            if write_line(&mut write, &Response::from(response)).await.is_err() {
                                break;
                            }
                            continue;
                        }
                        Frame::Line(bytes) => bytes,
                        Frame::Closed => unreachable!("handled above"),
                    };
                    if bytes.iter().all(u8::is_ascii_whitespace) {
                        continue;
                    }
                    match Bootstrap::read(&bytes) {
                        Some(Bootstrap::Hello {
                            version,
                            contract,
                            world,
                        }) => {
                            // The daemon names itself in every dimension
                            // (C84, C83) and decides nothing: the client
                            // judges staleness and replaces, refuses or
                            // reports (C10). What the client said about
                            // itself is logged when it differs, bounded
                            // (C127: one hello writes no 64 MiB line), so a
                            // mismatch is visible from this side too.
                            if contract != acquisition_protocol::CONTRACT_REVISION {
                                self.log(&format!(
                                    "contract mismatch: client {} (version {}), daemon {} (version {})",
                                    bounded_message(&contract, ""),
                                    bounded_message(&version, ""),
                                    acquisition_protocol::CONTRACT_REVISION,
                                    acquisition_protocol::VERSION
                                ));
                            }
                            let own_world = self
                                .world
                                .clone()
                                .unwrap_or_else(|| acquisition_protocol::protocol::UNKNOWN.to_string());
                            if world != own_world {
                                self.log(&format!(
                                    "world mismatch: client on {}, daemon on {own_world} — the client refuses this daemon",
                                    bounded_message(&world, "")
                                ));
                            }
                            let hello = BootstrapReply::Hello {
                                version: acquisition_protocol::VERSION.to_string(),
                                contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
                                artifact: self.artifact.clone(),
                                pid: std::process::id(),
                                provider: self.provider.name.to_string(),
                                world: own_world,
                            };
                            if write_line(&mut write, &hello).await.is_err() {
                                break;
                            }
                            greeted = true;
                            continue;
                        }
                        // C133: the stop connection stays open until the
                        // process ends, so a client's replace never
                        // reconnects to a draining daemon.
                        Some(Bootstrap::DaemonStop) => {
                            let _ = write_line(&mut write, &BootstrapReply::Stopping).await;
                            self.request_stop();
                            let never: std::convert::Infallible =
                                std::future::pending().await;
                            match never {}
                        }
                        None => {}
                    }
                    let response = if !greeted {
                        Refusal::new(
                            ErrorKind::BadRequest,
                            "the connection has not opened with hello; every connection begins with the hello exchange",
                        )
                        .into()
                    } else if events.is_some() {
                        Refusal::new(
                            ErrorKind::BadRequest,
                            "this connection is a subscription: after `subscribed` it carries events only; requests go on a connection of their own",
                        )
                        .into()
                    } else {
                        match serde_json::from_slice::<Request>(&bytes) {
                            Ok(req) => self.handle_request(req, &mut events).await,
                            // serde quotes a mistyped value whole: bounded,
                            // so the refusal fits the response frame (C127).
                            // A cut message loses serde's position at its
                            // end, so it is said again (a tagged request's
                            // fields have none: line 0).
                            Err(e) => {
                                let at = match e.line() {
                                    0 => String::new(),
                                    line => format!(" at line {line} column {}", e.column()),
                                };
                                Refusal::new(
                                    ErrorKind::BadRequest,
                                    format!("bad request: {}", bounded_message(&e, &at)),
                                )
                                .into()
                            }
                        }
                    };
                    if write_line(&mut write, &response).await.is_err() {
                        break;
                    }
                }
                signal = recv_event(&mut events) => {
                    let response = match signal {
                        Signal::Event(job) => Response::Event { job },
                        Signal::Lagged(missed) => Response::ResyncRequired { missed },
                    };
                    if write_line(&mut write, &response).await.is_err() {
                        break;
                    }
                }
            }
        }

        // Stamped before the count drops, connect's mirror: the idle
        // verdict never reads zero connections beside a stale stamp.
        self.mark_activity(self.choke.now());
        self.connections.close();
    }

    pub(super) async fn handle_request(
        self: &Arc<Self>,
        req: Request,
        events: &mut Option<broadcast::Receiver<JobInfo>>,
    ) -> Response {
        match req {
            Request::Submit {
                kind,
                params,
                priority,
                submitted_by,
                account,
            } => {
                let admitted = self
                    .refuse_while_stopping()
                    .and_then(|()| admit_client(&kind, &params, self.provider.is_real()))
                    .and_then(|()| admit_client_submitter(&submitted_by));
                // C148: a job that will need a session waits for the
                // start's restore before its account is resolved.
                let admitted =
                    if admitted.is_ok() && (account.is_some() || self.kind_needs_account(&kind)) {
                        self.restored().await.map(|_| ())
                    } else {
                        admitted
                    };
                match admitted
                    .and_then(|()| self.resolve_account(&kind, account.as_deref()))
                    .and_then(|account| self.submit(kind, params, priority, submitted_by, account))
                {
                    Ok(id) => Response::Submitted { id },
                    Err(refusal) => {
                        self.note_error(&format!("submit refused: {refusal}"));
                        refusal.into()
                    }
                }
            }
            Request::Status { id } => {
                let held = self.shared.lock().unwrap().snapshot(self, id);
                // Not in memory: concluded and evicted, or an earlier
                // daemon's; the table answers for it (C6).
                match held.map_or_else(|| self.stored_job(id), |job| Ok(Some(job))) {
                    Ok(Some(job)) => Response::Status { job },
                    Ok(None) => Refusal::no_job(id).into(),
                    Err(refusal) => {
                        self.note_error(&format!("status {id}: {refusal}"));
                        refusal.into()
                    }
                }
            }
            Request::Result { id } => {
                let held = {
                    let s = self.shared.lock().unwrap();
                    s.jobs.get(&id).map(|e| (e.info.state, e.outcome.clone()))
                };
                match held {
                    Some((_, Some(outcome))) => Response::Result { id, outcome },
                    Some((state, None)) => {
                        Refusal::new(ErrorKind::WrongState, format!("job {id} is still {state}"))
                            .into()
                    }
                    // Not in memory: concluded and evicted (C6), or a
                    // previous daemon's, if retention still has it.
                    None => match self.stored_outcome(id) {
                        Ok(Some(outcome)) => Response::Result { id, outcome },
                        Ok(None) => Refusal::no_job(id).into(),
                        Err(refusal) => {
                            self.note_error(&format!("result {id}: {refusal}"));
                            refusal.into()
                        }
                    },
                }
            }
            Request::Cancel { id } => match self.cancel(id) {
                Ok(()) => Response::Ack { keyring: None },
                Err(refusal) => refusal.into(),
            },
            Request::SetPriority { id, priority } => match self.set_priority(id, priority) {
                Ok(()) => {
                    self.work.notify_one();
                    Response::Ack { keyring: None }
                }
                Err(refusal) => refusal.into(),
            },
            Request::List => match self.job_list() {
                Ok(jobs) => Response::Jobs { jobs },
                Err(refusal) => {
                    self.note_error(&format!("list: {refusal}"));
                    refusal.into()
                }
            },
            Request::Subscribe => {
                *events = Some(self.events.subscribe());
                Response::Subscribed
            }
            // C148: the four requests below read the session table, so
            // each waits for the start's restore first, refused as
            // stopping before and after the wait; so does a quote, which
            // resolves its account as a submit does.
            Request::AuthStart => match self.restored().await {
                Err(refusal) => refusal.into(),
                Ok(daemon) => match daemon.auth_start().await {
                    Ok(authorize_url) => Response::AuthUrl { authorize_url },
                    Err(refusal) => refusal.into(),
                },
            },
            Request::AuthStatus => self.auth_status(),
            Request::AuthCheck { account } => {
                match self
                    .restored()
                    .await
                    .and_then(|daemon| daemon.canonical_account(account.as_deref()))
                {
                    Err(refusal) => refusal.into(),
                    Ok(account) => match self
                        .valid_access_token(account.as_deref(), true, None)
                        .await
                    {
                        Ok(_) => self.auth_status(),
                        Err(refusal) => {
                            self.note_error(&format!("auth check failed: {refusal}"));
                            refusal.into()
                        }
                    },
                }
            }
            // A live session (named, or the sole one) logs out; a known
            // but not live account only loses its keyring entries. C144:
            // the decision is the logout's own, ordered with the other
            // landings, and a clear that failed is the answer. C148: its
            // keyring and index I/O runs on a blocking thread.
            Request::AuthLogout { account } => match self.restored().await {
                Err(refusal) => refusal.into(),
                Ok(daemon) => match daemon.logout_blocking(account).await {
                    Ok(keyring) => Response::Ack {
                        keyring: Some(keyring),
                    },
                    Err(refusal) => refusal.into(),
                },
            },
            Request::Quote { jobs, account } => {
                match self
                    .restored()
                    .await
                    .and_then(|daemon| daemon.quote(&jobs, account.as_deref()))
                {
                    Ok(quote) => Response::Quote { quote },
                    Err(refusal) => refusal.into(),
                }
            }
            Request::DaemonStatus => {
                let s = self.shared.lock().unwrap();
                let (in_flight, max_in_flight) = self.choke.actual_send_occupancy();
                let (waiting, running) =
                    s.jobs
                        .values()
                        .fold((0, 0), |(w, r), e| match e.info.state {
                            JobState::Waiting => (w + 1, r),
                            JobState::Running => (w, r + 1),
                            _ => (w, r),
                        });
                Response::DaemonStatus {
                    pid: std::process::id(),
                    version: acquisition_protocol::VERSION.to_string(),
                    contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
                    provider: self.provider.name.to_string(),
                    uptime_seconds: self.uptime_seconds(),
                    idle_shutdown_seconds: idle_shutdown_from_env().as_secs(),
                    connections: self.connections.open(),
                    jobs_waiting: waiting,
                    jobs_running: running,
                    policies_known: self.choke.policy_statuses().len(),
                    in_flight,
                    max_in_flight,
                    rails: self.rails().status(),
                    keyring: self.keyring_summary(&s),
                    log: self.log_path.clone(),
                }
            }
            Request::ResetTripwire => match self.reset_rails() {
                Ok(()) => Response::Ack { keyring: None },
                Err(refusal) => refusal.into(),
            },
            Request::Dashboard => {
                let jobs = match self.job_list() {
                    Ok(jobs) => jobs,
                    Err(refusal) => {
                        self.note_error(&format!("dashboard: {refusal}"));
                        return refusal.into();
                    }
                };
                let now = self.choke.now();
                let s = self.shared.lock().unwrap();
                let (in_flight, max_in_flight) = self.choke.actual_send_occupancy();
                Response::Dashboard {
                    pid: std::process::id(),
                    version: acquisition_protocol::VERSION.to_string(),
                    contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
                    provider: self.provider.name.to_string(),
                    uptime_seconds: self.uptime_seconds(),
                    connections: self.connections.open(),
                    logged_in: !s.auth.by_account.is_empty(),
                    username: self.headline_session(&s).and_then(|h| h.username.clone()),
                    access_expires_in_seconds: self
                        .headline_session(&s)
                        .and_then(|h| h.access_expires_at)
                        .map(|t| {
                            t.duration_since(self.choke.wall())
                                .unwrap_or_default()
                                .as_secs()
                        }),
                    keyring: self.keyring_summary(&s),
                    in_flight,
                    max_in_flight,
                    policies: self.choke.policy_statuses(),
                    policyless_endpoints: self.choke.policyless_endpoints(),
                    degraded_endpoints: self.choke.degraded_endpoints(),
                    jobs,
                    sends: self.choke.recent_sends(),
                    rails: self.rails().status(),
                    errors: s
                        .errors
                        .iter()
                        .rev()
                        .map(|(at, message)| ErrorRecord {
                            seconds_ago: now.saturating_duration_since(*at).as_secs_f64(),
                            message: message.clone(),
                        })
                        .collect(),
                }
            }
        }
    }

    /// Clear the rails halt, the Cloudflare block mark and the lifetime's
    /// unreachable-origin halt (C129), and wake the jobs that were waiting
    /// on them. A reset the rails could not write is refused, every halt
    /// standing.
    pub(super) fn reset_rails(&self) -> Result<(), Refusal> {
        if let Err(error) = self.rails().reset_tripwire() {
            let message = format!("the halt stands, for the reset did not reach disk: {error}");
            self.note_error(&message);
            return Err(Refusal::new(ErrorKind::Internal, message));
        }
        self.log(
            "halt reset by request: the tripwire's trip, any Cloudflare block mark and any halt for an origin Cloudflare could not reach cleared",
        );
        self.work.notify_one();
        Ok(())
    }

    /// C148: this daemon, once the start's restore has landed, for a
    /// request that reads the session table. One rule for every request
    /// that waits: it is refused as stopping (C133) before the wait and
    /// after it, so an exit begun while the restore read the keyring
    /// starts no login, check, logout, quote or submit on a departing
    /// daemon.
    async fn restored(self: &Arc<Self>) -> Result<&Arc<Self>, Refusal> {
        self.refuse_while_stopping()?;
        self.await_restore().await;
        self.refuse_while_stopping()?;
        Ok(self)
    }

    /// C133: a submit after the exit began is refused, in words a frontend
    /// can act on; the queue is the next daemon's.
    pub(super) fn refuse_while_stopping(&self) -> Result<(), Refusal> {
        // C157: a connection admitted after the idle exit's claim, before
        // its swap, is refused as one after the swap would be.
        if self.exit.begun() || self.connections.claimed() {
            return Err(Refusal::new(
                ErrorKind::Refused,
                "the daemon is stopping; try again in a moment",
            ));
        }
        Ok(())
    }

    /// C133, `daemon stop`'s ingress: the exit begun here, its swap and,
    /// when won, its admission stop in this task, before the lifecycle's
    /// driver hears of it (C157); `true` when this stop began it. The
    /// stop's connection then pends until the process ends, so a second
    /// stop's connection stays open, as the first's.
    pub(super) fn request_stop(&self) -> bool {
        let won = self.begin_exit("stop requested; exiting");
        self.exit.hand(life::Event::StopRequested { won });
        won
    }

    /// C133: begin the exit, once; `false` if it had begun already. The
    /// bound is armed first ([`Exit::arm`]), before any lock is taken, so
    /// a cause can never wait behind one (a keyring save holds
    /// `credentials` and may be a prompt no one answers).
    pub(super) fn begin_exit(&self, why: &str) -> bool {
        if !self.exit.arm(why) {
            return false;
        }
        self.stop_admitting(why);
        true
    }

    /// C133: the lifetime's halt is set in the rails: the choke point
    /// admits nothing more, and a job parked before its send is turned
    /// away to waiting (C25). The dispatcher is woken so the parked jobs
    /// see it, and from here a submit is refused as stopping; `hello` is
    /// still answered.
    fn stop_admitting(&self, why: &str) {
        self.rails().begin_stopping();
        self.log(why);
        self.work.notify_one();
    }

    /// C133, the last of the exit. The daemon leaves by `process::exit`,
    /// so nothing is dropped: the store's and the queue's write-ahead logs
    /// are checkpointed here first (`Store::checkpoint`: an exit without
    /// it left every fact since the last automatic checkpoint in a `-wal`
    /// beside the file — the price-notes run, 2026-09-04; plan step 7,
    /// item 4), then the socket is removed, then the process exits. A
    /// checkpoint a reader keeps from finishing is logged, never waited
    /// on. Tests never call it. Pinned by `tests/daemon_stop_checkpoint.rs`
    /// (acquisition-cli).
    pub(super) fn leave(&self) -> ! {
        // Each checkpoint's result is bound before it is reported, so its
        // guard is dropped first: `note_error` takes `shared`, which comes
        // before `jobs_db` in the lock order (`struct Daemon`), and a
        // `match` keeps its scrutinee's guard alive through every arm.
        let store_checkpoint = self
            .store
            .lock()
            .unwrap()
            .as_ref()
            .map(|(account, store)| (account.clone(), store.checkpoint()));
        if let Some((account, checkpoint)) = store_checkpoint {
            match checkpoint {
                Ok(c) => self.log(&format!(
                    "store: {account}'s facts checkpointed on exit ({} of {} WAL pages{})",
                    c.checkpointed,
                    c.wal_pages,
                    if c.busy {
                        "; a reader kept it from finishing"
                    } else {
                        ""
                    }
                )),
                Err(e) => self.note_error(&format!(
                    "store: checkpoint of {account}'s facts on exit failed: {e:#}"
                )),
            }
        }
        let jobs_checkpoint = self.jobs_db.lock().unwrap().checkpoint();
        match jobs_checkpoint {
            Ok(c) => self.log(&format!(
                "jobs: queue checkpointed on exit ({} of {} WAL pages{})",
                c.checkpointed,
                c.wal_pages,
                if c.busy {
                    "; a reader kept it from finishing"
                } else {
                    ""
                }
            )),
            Err(e) => self.note_error(&format!("jobs: queue checkpoint on exit failed: {e:#}")),
        }
        if let Some(socket) = &self.socket_path {
            socket.remove_if_still_ours();
        }
        std::process::exit(0);
    }

    /// The idle verdict (C3; the halted arm is C25): no connection, no
    /// live job, the last activity at least `idle_shutdown` ago, and no
    /// limiter history worth keeping — a pending wait or counted hits
    /// inside a policy window, which a daemon respawned a minute later
    /// would otherwise have to assume the worst about. Every clause reads
    /// the daemon's clock, so a test advances a manual clock past the
    /// knob instead of waiting. Pinned: `tests/idle.rs`'s `idle_verdict_*`,
    /// through this boolean; the lifecycle's driver reads [`Self::judge`].
    #[cfg(test)]
    pub(super) fn is_idle(&self, idle_shutdown: Duration) -> bool {
        self.judge(idle_shutdown).idle
    }

    /// The idle verdict (`is_idle`'s clauses), and whether a clause
    /// other than time held the daemon, which the lifecycle's driver
    /// re-judges at [`IDLE_REJUDGE`] (C157). Takes `shared`.
    fn judge(&self, idle_shutdown: Duration) -> Judged {
        let now = self.choke.now();
        // C157: the connections' word judged, read once and judged as
        // read: the idle exit's commit claims exactly this word.
        let word = self.connections.word();
        let (held, timed) = {
            let s = self.shared.lock().unwrap();
            let parked = self.rails().halted().is_some() || self.queue_failed().is_some();
            let live_jobs = Self::has_live_jobs(&s, parked);
            // A login holds the daemon (C133): its browser step and code
            // exchange belong to no connection and no job. The count and
            // the stamp are read beside this hold, not under it (C148).
            let held = Connections::open_in(word) != 0
                || live_jobs
                || s.auth.pending.is_some()
                // C148: a keyring or index write in flight, the start's
                // restore among them, holds the daemon.
                || s.landings != 0;
            let timed = now.saturating_duration_since(self.last_activity()) >= idle_shutdown;
            (held, timed)
        };
        let live = self.choke.is_live();
        Judged {
            idle: !held && timed && !live,
            held: held || live,
            word,
        }
    }

    /// C157, the lifecycle's driver: one loop over [`Life`]'s `step`, the
    /// only place the lifecycle's instants, handles and durations live.
    /// It performs each effect with the functions that exist and turns
    /// what it saw into the next event, from its sources: the causes'
    /// events (`daemon stop`'s connection and the signal watcher, which
    /// each swapped the exit at their ingress, [`Exit::hand`]), the
    /// verdict's answer, the quiet wait's looks and the two timers. On
    /// tokio time, deliberately not on the daemon's `Clock`: the test
    /// `ManualClock` advances on every `sleep`, so a timer on it would
    /// fire at once. `knob` is the idle exit's, read once by the caller
    /// at the loop's start. The loop ends by `leave`, the effect `Leave`
    /// performed, given what the bound cut (`None` when the drain was
    /// clean): `run` installs [`Self::leave`], which ends the process; a
    /// test installs a recorder. Before that last act it never blocks on
    /// `shared`: the verdict is judged on a thread of its own and the
    /// drain looks through `try_lock`, so a signal's admission stop is
    /// never behind a held lock (C133).
    pub(super) async fn life(self: Arc<Self>, knob: Duration, leave: Leave) {
        let Some(mut ingress) = self.exit.take_ingress() else {
            // One driver per daemon: a second has nothing to drive.
            return;
        };
        let mut driver = Driver {
            daemon: self.clone(),
            knob,
            held: false,
            judged: None,
            idle: None,
            drain: None,
            judging: None,
            looking: None,
            seen: None,
            cut: None,
        };
        let mut machine = Life::new();
        let mut effects: VecDeque<Effect> = machine.start().into();
        let mut events: VecDeque<life::Event> = VecDeque::new();
        loop {
            while let Some(effect) = effects.pop_front() {
                if effect == Effect::Leave {
                    leave(&self, driver.cut.take());
                    return;
                }
                events.extend(match effect {
                    Effect::Begin => Some(driver.begin().await),
                    effect => driver.perform(effect),
                });
            }
            let event = match events.pop_front() {
                Some(event) => event,
                None => driver.next(&mut ingress).await,
            };
            // A signal that finds the exit begun is only logged, as the
            // watcher's line always was (C133).
            if let life::Event::Signal { won: false, why } = &event {
                self.log(why);
            }
            #[cfg(test)]
            if event == life::Event::Busy
                && let Some(probe) = self.life_probe.get()
            {
                probe.busy.notify_one();
            }
            effects.extend(machine.step(event));
        }
    }
}

/// C157: a test's view into the lifecycle's driver, at the points a pin
/// must order against: the judge's start, a busy verdict stepped, and an
/// idle verdict answered before its commit (the driver waits for `go`
/// when `hold_idle` is set), and the idle commit's claim made before its
/// swap (the driver waits for `go` when `hold_claim` is set).
#[cfg(test)]
#[derive(Default)]
pub(super) struct LifeProbe {
    pub(super) judging: tokio::sync::Notify,
    pub(super) busy: tokio::sync::Notify,
    pub(super) idle: tokio::sync::Notify,
    pub(super) hold_idle: bool,
    pub(super) claimed: tokio::sync::Notify,
    pub(super) hold_claim: bool,
    pub(super) go: tokio::sync::Notify,
}

/// C157: what the lifecycle's driver does at `Leave`, given what the
/// bound cut: [`Daemon::leave`] in `run`; a recorder in a test, which
/// reads the effect, never the process's end.
pub(super) type Leave = Box<dyn FnOnce(&Daemon, Option<InFlight>) + Send>;

/// The verdict as the driver reads it.
struct Judged {
    idle: bool,
    /// A clause other than time held the daemon.
    held: bool,
    /// The connections' word judged ([`Connections::word`]).
    word: u64,
}

/// The lifecycle's driver's own state: the armed timers, the verdict in
/// flight, the drain's looks.
struct Driver {
    daemon: Arc<Daemon>,
    knob: Duration,
    /// The last verdict was held by a clause other than time.
    held: bool,
    /// The connections' word the last verdict judged.
    judged: Option<u64>,
    idle: Option<tokio::time::Instant>,
    drain: Option<tokio::time::Instant>,
    judging: Option<tokio::task::JoinHandle<Judged>>,
    /// When the quiet wait looks again.
    looking: Option<tokio::time::Instant>,
    /// The last snapshot the drain's looks saw.
    seen: Option<InFlight>,
    /// What the bound cut, for `Leave`.
    cut: Option<InFlight>,
}

impl Driver {
    /// C157, the timers as the driver runs them: how long each is given.
    /// `Drain` is the exit's bound, which its absolute deadline was set
    /// from at the swap. `Idle` is the knob's remainder after the last
    /// activity, floored at the re-judge interval while a clause other
    /// than time held the last verdict, and capped at a day.
    fn duration(&self, timer: Timer) -> Duration {
        match timer {
            Timer::Drain => self.daemon.exit.bound,
            Timer::Idle => {
                let daemon = &self.daemon;
                let since = daemon
                    .choke
                    .now()
                    .saturating_duration_since(daemon.last_activity());
                let left = self.knob.saturating_sub(since);
                let left = if self.held {
                    left.max(IDLE_REJUDGE)
                } else {
                    left
                };
                left.min(IDLE_CAP)
            }
        }
    }

    /// C157, the idle cause's commit, by one rule: admission and the idle
    /// commit contend on one atomic, the connections' word, so no
    /// connection is admitted between the verdict confirmed and the
    /// exit's swap. The commit claims the exact word the verdict judged
    /// ([`Connections::claim`]): a connection admitted since, open or
    /// closed again, fails it, and the answer is `Busy` (the machine arms
    /// the idle deadline again); one admitted after it is refused as
    /// stopping. Then the swap, as the idle cause's `begin_exit` always
    /// was; a stop or a signal that swapped first makes the claim moot.
    /// The verdict's other clauses need no claim of their own: from an
    /// idle verdict, a live job, a pending login, a landing or a live
    /// limiter comes only from work a connection brings (a submit, an
    /// auth) or from work the verdict already counted (a live job's
    /// children and probes, a pending login's exchange, a landing); the
    /// start's restore is a landing counted before the driver starts, and
    /// the activity stamp moves only with these. The time clause moves
    /// only toward idle.
    async fn begin(&mut self) -> life::Event {
        if !self
            .judged
            .is_some_and(|judged| self.daemon.connections.claim(judged))
        {
            return life::Event::Busy;
        }
        #[cfg(test)]
        if let Some(probe) = self.daemon.life_probe.get() {
            probe.claimed.notify_one();
            if probe.hold_claim {
                probe.go.notified().await;
            }
        }
        life::Event::ExitArmed {
            won: self.daemon.begin_exit("idle timeout; exiting"),
        }
    }

    /// One effect performed; the event it answers at once, if any.
    fn perform(&mut self, effect: Effect) -> Option<life::Event> {
        match effect {
            Effect::Arm(Timer::Idle) => {
                let now = tokio::time::Instant::now();
                self.idle = Some(
                    now.checked_add(self.duration(Timer::Idle))
                        .unwrap_or_else(|| now + IDLE_CAP),
                );
                None
            }
            // The exit's own deadline, set by the swap at the ingress,
            // never read again from now.
            Effect::Arm(Timer::Drain) => {
                self.drain =
                    Some(self.daemon.exit.deadline.get().copied().unwrap_or_else(|| {
                        tokio::time::Instant::now() + self.duration(Timer::Drain)
                    }));
                None
            }
            Effect::Cancel(Timer::Idle) => {
                self.idle = None;
                None
            }
            Effect::Cancel(Timer::Drain) => {
                self.drain = None;
                self.looking = None;
                None
            }
            // The verdict takes `shared`: judged on a thread of its own,
            // answered when it returns, the loop's sources live meanwhile.
            Effect::Judge => {
                let daemon = self.daemon.clone();
                let knob = self.knob;
                self.judging = Some(tokio::task::spawn_blocking(move || {
                    #[cfg(test)]
                    if let Some(probe) = daemon.life_probe.get() {
                        probe.judging.notify_one();
                    }
                    daemon.judge(knob)
                }));
                None
            }
            Effect::Begin => unreachable!("the driver's loop performs the idle commit"),
            Effect::StopAdmitting(why) => {
                self.daemon.stop_admitting(&why);
                None
            }
            // The first look at once, as the drain always looked before
            // it paused.
            Effect::AwaitQuiet => self.look(),
            Effect::Cut => {
                let cut = self.seen.take().unwrap_or_default();
                self.daemon.log(&format!(
                    "exit: the bound of {}s cut {}",
                    self.daemon.exit.bound.as_secs_f64(),
                    cut.describe()
                ));
                self.cut = Some(cut);
                None
            }
            Effect::Leave => unreachable!("the driver's loop performs the leave"),
        }
    }

    /// One look at what is in flight, never blocking on `shared`: quiet
    /// is [`life::Event::Quiet`]; otherwise the snapshot is kept (a held
    /// lock keeps the last one seen) and the next look is armed.
    fn look(&mut self) -> Option<life::Event> {
        if let Ok(s) = self.daemon.shared.try_lock() {
            let left = InFlight::of(&s);
            drop(s);
            if left.is_empty() {
                self.daemon.log("exit: nothing in flight");
                self.looking = None;
                return Some(life::Event::Quiet);
            }
            self.seen = Some(left);
        }
        self.looking = Some(tokio::time::Instant::now() + DRAIN_POLL);
        None
    }

    /// The next event from the loop's sources, in one biased select: a
    /// cause first (a deadline is never a cause, so a stop or a signal
    /// ready beside one is delivered before it), then the verdict's
    /// answer, then the quiet wait's next look, then the drain's
    /// deadline, then the idle deadline. The drain's deadline abandons
    /// the quiet wait after one more look: quiet is `Quiet`, not quiet
    /// is the deadline, its snapshot kept for the cut.
    async fn next(
        &mut self,
        ingress: &mut tokio::sync::mpsc::UnboundedReceiver<life::Event>,
    ) -> life::Event {
        loop {
            let looking = until(self.looking);
            let drain = until(self.drain);
            let idle = until(self.idle);
            let judging = self.judging.as_mut();
            let judging = async move {
                match judging {
                    Some(judging) => judging.await,
                    None => std::future::pending().await,
                }
            };
            enum Turn {
                Cause(life::Event),
                Judged(Result<Judged, tokio::task::JoinError>),
                Look,
                Drain,
                Idle,
            }
            let turn = tokio::select! {
                biased;
                Some(event) = ingress.recv() => Turn::Cause(event),
                judged = judging => Turn::Judged(judged),
                () = looking => Turn::Look,
                () = drain => Turn::Drain,
                () = idle => Turn::Idle,
            };
            match turn {
                Turn::Cause(event) => return event,
                Turn::Judged(judged) => {
                    self.judging = None;
                    let judged = judged.expect("the idle verdict's judge ran to its end");
                    self.held = judged.held;
                    self.judged = Some(judged.word);
                    if !judged.idle {
                        return life::Event::Busy;
                    }
                    #[cfg(test)]
                    if let Some(probe) = self.daemon.life_probe.get() {
                        probe.idle.notify_one();
                        if probe.hold_idle {
                            probe.go.notified().await;
                        }
                    }
                    return life::Event::Idle;
                }
                Turn::Look => {
                    if let Some(quiet) = self.look() {
                        return quiet;
                    }
                }
                Turn::Drain => {
                    self.drain = None;
                    if let Some(quiet) = self.look() {
                        return quiet;
                    }
                    self.looking = None;
                    return life::Event::Deadline(Timer::Drain);
                }
                Turn::Idle => {
                    self.idle = None;
                    return life::Event::Deadline(Timer::Idle);
                }
            }
        }
    }
}

/// An armed instant's end; never, when nothing is armed.
async fn until(at: Option<tokio::time::Instant>) {
    match at {
        Some(at) => tokio::time::sleep_until(at).await,
        None => std::future::pending().await,
    }
}

impl Daemon {
    /// Whether any job keeps the daemon up. A halted daemon's waiting jobs
    /// do not: they are on disk, and its successor holds them until the
    /// reset (C25). A parent holding its result runs no task of its own.
    pub(super) fn has_live_jobs(s: &Shared, halted: bool) -> bool {
        s.jobs.values().any(|e| match e.info.state {
            JobState::Running => e.deferred.is_none() || !halted,
            JobState::Waiting => !halted,
            _ => false,
        })
    }
}

/// C133: what the exit's drain waits for, as one look saw it.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct InFlight {
    /// Jobs Running past admission, by id.
    pub(super) jobs: Vec<JobId>,
    /// Jobs picked but still waiting (at the limiter, before their send):
    /// waited for until their task gives the key back.
    pub(super) held: Vec<JobId>,
    /// Accounts whose token refresh is in flight.
    pub(super) refreshes: Vec<String>,
    /// Login code exchanges in flight.
    pub(super) exchanges: usize,
    /// C148: landings in flight, each a keyring or account-index write.
    pub(super) landings: usize,
    /// A login whose profile job has finished, or is about to be
    /// submitted, while its staged tokens are not yet registered or
    /// dropped: `complete_login` or `abort_login` is still to run; or a
    /// login being recorded, its keyring save and index write in flight
    /// (C148).
    pub(super) login: bool,
}

impl InFlight {
    pub(super) fn of(s: &Shared) -> InFlight {
        let (mut jobs, mut held): (Vec<JobId>, Vec<JobId>) =
            s.active_jobs.keys().copied().partition(|id| {
                s.jobs
                    .get(id)
                    .is_some_and(|e| e.info.state == JobState::Running)
            });
        jobs.sort_unstable();
        held.sort_unstable();
        let mut refreshes: Vec<String> = s
            .auth
            .by_account
            .iter()
            .filter(|(_, session)| session.refresh_flight.is_some())
            .map(|(account, _)| account.clone())
            .collect();
        refreshes.sort();
        // C133, issue #59: the exchange's count drops before the login's
        // profile job is submitted, and the job's active slot is freed
        // before `complete_login` takes the staging slot (from there its
        // landing mark is counted, C148). Between, the staging
        // slot is the login's only trace, and the drain counts it: while
        // the profile job is unsubmitted (the login task runs on to its
        // submit without a pause) or terminal (the task runs on to
        // complete or abort). A profile job still waiting is not counted:
        // the stop turned it away, and the login is lost and repeated.
        let login = s.auth.landing.is_some()
            || s.auth
                .staging
                .as_ref()
                .is_some_and(|st| match st.profile_job {
                    None => true,
                    // Absent is evicted, so terminal: the id was set
                    // where the job was inserted (C6).
                    Some(id) => s.jobs.get(&id).is_none_or(|e| e.info.state.is_terminal()),
                });
        InFlight {
            jobs,
            held,
            refreshes,
            exchanges: s.auth.exchanges,
            landings: s.landings,
            login,
        }
    }

    fn is_empty(&self) -> bool {
        self.jobs.is_empty()
            && self.held.is_empty()
            && self.refreshes.is_empty()
            && self.exchanges == 0
            && self.landings == 0
            && !self.login
    }

    fn describe(&self) -> String {
        if self.is_empty() {
            return "nothing it could see (the daemon's state was held throughout)".into();
        }
        let mut parts = Vec::new();
        if !self.jobs.is_empty() {
            let ids: Vec<String> = self.jobs.iter().map(JobId::to_string).collect();
            parts.push(format!(
                "job {} (left running on disk for the next daemon)",
                ids.join(", ")
            ));
        }
        if !self.held.is_empty() {
            let ids: Vec<String> = self.held.iter().map(JobId::to_string).collect();
            parts.push(format!("job {} (still waiting on disk)", ids.join(", ")));
        }
        if !self.refreshes.is_empty() {
            parts.push(format!(
                "the token refresh of {}",
                self.refreshes.join(", ")
            ));
        }
        if self.exchanges > 0 {
            parts.push("a login's code exchange".into());
        }
        if self.landings > 0 {
            parts.push("a keyring or account-index write".into());
        }
        if self.login {
            parts.push("a login landing its profile".into());
        }
        parts.join("; ")
    }
}

/// What a subscribed connection is told next: an event, or that it
/// missed some (C85).
enum Signal {
    Event(JobInfo),
    /// The broadcast channel overran this receiver; `missed` events are
    /// gone. The subscriber snapshots again.
    Lagged(u64),
}

async fn recv_event(rx: &mut Option<broadcast::Receiver<JobInfo>>) -> Signal {
    match rx {
        Some(rx) => match rx.recv().await {
            Ok(info) => Signal::Event(info),
            // A loss the client cannot observe cannot be recovered from,
            // so it is reported, never skipped (C85); the receiver has
            // already advanced past the gap. Closed can't happen while
            // the daemon holds the sender.
            Err(broadcast::error::RecvError::Lagged(missed)) => Signal::Lagged(missed),
            Err(broadcast::error::RecvError::Closed) => std::future::pending().await,
        },
        None => std::future::pending().await,
    }
}

async fn write_line<T: serde::Serialize>(
    write: &mut WriteHalf<Stream>,
    response: &T,
) -> std::io::Result<()> {
    let mut line = serde_json::to_string(response).expect("response serializes");
    line.push('\n');
    write.write_all(line.as_bytes()).await
}
