//! The connection and the lifecycle: a connection's frames, each request,
//! the status documents, the rails reset, the idle verdict and watchdog,
//! and the exit.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

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

    pub(super) fn keyring_summary(&self, s: &Shared) -> String {
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
    pub(super) async fn handle_conn(self: Arc<Self>, stream: Stream) {
        {
            let now = self.choke.now();
            let mut s = self.shared.lock().unwrap();
            s.connections += 1;
            s.last_activity = now;
        }

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
                            self.exit("stop requested; exiting").await;
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

        let now = self.choke.now();
        let mut s = self.shared.lock().unwrap();
        s.connections -= 1;
        s.last_activity = now;
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
            } => match self
                .refuse_while_stopping()
                .and_then(|()| admit_client(&kind, &params, self.provider.is_real()))
                .and_then(|()| self.resolve_account(&kind, account.as_deref()))
                .and_then(|account| self.submit(kind, params, priority, submitted_by, account))
            {
                Ok(id) => Response::Submitted { id },
                Err(refusal) => {
                    self.note_error(&format!("submit refused: {refusal}"));
                    refusal.into()
                }
            },
            Request::Status { id } => match self.shared.lock().unwrap().snapshot(self, id) {
                Some(job) => Response::Status { job },
                None => Refusal::no_job(id).into(),
            },
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
                    // Not this lifetime's: a previous daemon's result, if
                    // retention still has it.
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
                Ok(()) => Response::Ack,
                Err(refusal) => refusal.into(),
            },
            Request::SetPriority { id, priority } => match self.set_priority(id, priority) {
                Ok(()) => {
                    self.work.notify_one();
                    Response::Ack
                }
                Err(refusal) => refusal.into(),
            },
            Request::List => Response::Jobs {
                jobs: self.shared.lock().unwrap().list(self),
            },
            Request::Subscribe => {
                *events = Some(self.events.subscribe());
                Response::Subscribed
            }
            Request::AuthStart => match self.auth_start().await {
                Ok(authorize_url) => Response::AuthUrl { authorize_url },
                Err(refusal) => refusal.into(),
            },
            Request::AuthStatus => self.auth_status(),
            Request::AuthCheck { account } => match self.canonical_account(account.as_deref()) {
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
            },
            Request::AuthLogout { account } => {
                // A live session (named, or the sole one) logs out; a known
                // but not live account only loses its keyring entry.
                let live = {
                    let s = self.shared.lock().unwrap();
                    match &account {
                        None => s
                            .auth
                            .get(None)
                            .map(|x| x.username.clone().unwrap_or_default()),
                        Some(req) => s.auth.matching(req).and_then(|found| {
                            found
                                .map(|x| x.username.clone().unwrap_or_default())
                                .ok_or_else(|| no_session(req))
                        }),
                    }
                };
                match (live, &account) {
                    (Ok(username), _) => {
                        if let Err(e) = self.logout(Some(&username)) {
                            self.log(&format!("keyring clear failed: {e}"));
                        }
                        Response::Ack
                    }
                    (Err(_), Some(req)) => match self.forget_account(req) {
                        Ok(()) => Response::Ack,
                        Err(refusal) => refusal.into(),
                    },
                    (Err(refusal), None) => refusal.into(),
                }
            }
            Request::Quote { jobs, account } => match self.quote(&jobs, account.as_deref()) {
                Ok(quote) => Response::Quote { quote },
                Err(refusal) => refusal.into(),
            },
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
                    connections: s.connections,
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
                Ok(()) => Response::Ack,
                Err(refusal) => refusal.into(),
            },
            Request::Dashboard => {
                let now = self.choke.now();
                let s = self.shared.lock().unwrap();
                let (in_flight, max_in_flight) = self.choke.actual_send_occupancy();
                Response::Dashboard {
                    pid: std::process::id(),
                    version: acquisition_protocol::VERSION.to_string(),
                    contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
                    provider: self.provider.name.to_string(),
                    uptime_seconds: self.uptime_seconds(),
                    connections: s.connections,
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
                    jobs: s.list(self),
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

    /// C133: a submit after the exit began is refused, in words a frontend
    /// can act on; the queue is the next daemon's.
    pub(super) fn refuse_while_stopping(&self) -> Result<(), Refusal> {
        if self.exit.begun() {
            return Err(Refusal::new(
                ErrorKind::Refused,
                "the daemon is stopping; try again in a moment",
            ));
        }
        Ok(())
    }

    /// C133, the one exit every cause runs: begin, drain, leave. A cause
    /// that finds the exit already begun waits for the process to end
    /// with it (a second stop's connection stays open, as the first's).
    pub(super) async fn exit(&self, why: &str) -> ! {
        if !self.begin_exit(why) {
            std::future::pending::<()>().await;
        }
        self.drain().await;
        self.leave()
    }

    /// C133: begin the exit, once; `false` if it had begun already. The
    /// bound is armed first, on tokio time, before any lock is taken, so
    /// a cause can never wait behind one (a keyring save holds `shared`
    /// and may be a prompt no one answers). Then the lifetime's halt is
    /// set in the rails: the choke point admits nothing more, and a job
    /// parked before its send is turned away to waiting (C25). The
    /// dispatcher is woken so the parked jobs see it, and from here a
    /// submit is refused as stopping; `hello` is still answered.
    pub(super) fn begin_exit(&self, why: &str) -> bool {
        if self.exit.begun.swap(true, Ordering::SeqCst) {
            return false;
        }
        let _ = self
            .exit
            .deadline
            .set(tokio::time::Instant::now() + self.exit.bound);
        self.rails().begin_stopping();
        self.log(why);
        self.work.notify_one();
        true
    }

    /// C133: wait until no job task is past admission, no session's
    /// refresh is in flight and no login's code exchange is, or until the
    /// bound. Returns what the bound cut, `None` when nothing was. Never
    /// blocks on `shared`: a look that finds it held looks again, so the
    /// bound ends the wait whatever holds the lock. What is cut is left
    /// running on disk for the persisted queue to replay (C6).
    pub(super) async fn drain(&self) -> Option<InFlight> {
        let deadline = self
            .exit
            .deadline
            .get()
            .copied()
            .unwrap_or_else(tokio::time::Instant::now);
        let mut seen = None;
        loop {
            if let Ok(s) = self.shared.try_lock() {
                let left = InFlight::of(&s);
                drop(s);
                if left.is_empty() {
                    self.log("exit: nothing in flight");
                    return None;
                }
                seen = Some(left);
            }
            let now = tokio::time::Instant::now();
            if now >= deadline {
                let cut = seen.unwrap_or_default();
                self.log(&format!(
                    "exit: the bound of {}s cut {}",
                    self.exit.bound.as_secs_f64(),
                    cut.describe()
                ));
                return Some(cut);
            }
            tokio::time::sleep_until(deadline.min(now + DRAIN_POLL)).await;
        }
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
    fn leave(&self) -> ! {
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
    /// knob instead of waiting. Pinned: `tests/idle.rs`'s `idle_verdict_*`.
    pub(super) fn is_idle(&self, idle_shutdown: Duration) -> bool {
        let now = self.choke.now();
        let idle = {
            let s = self.shared.lock().unwrap();
            let parked = self.rails().halted().is_some() || self.queue_failed().is_some();
            let live_jobs = Self::has_live_jobs(&s, parked);
            // A login holds the daemon (C133): its browser step and code
            // exchange belong to no connection and no job.
            s.connections == 0
                && !live_jobs
                && s.auth.pending.is_none()
                && now.saturating_duration_since(s.last_activity) >= idle_shutdown
        };
        idle && !self.choke.is_live()
    }

    /// Polls the verdict on tokio time, deliberately not on the daemon's
    /// `Clock`: the test `ManualClock` advances on every `sleep`, so a
    /// poller on it would drive the clock forward `IDLE_POLL` per yield.
    /// The verdict is the tested part; this loop is never spawned under a
    /// manual clock (the harness constructors spawn nothing).
    pub(super) async fn idle_watchdog(self: Arc<Self>) {
        let idle_shutdown = idle_shutdown_from_env();
        loop {
            tokio::time::sleep(IDLE_POLL).await;
            if self.is_idle(idle_shutdown) {
                self.exit("idle timeout; exiting").await;
            }
        }
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
    /// Jobs whose task is past admission, by id.
    pub(super) jobs: Vec<JobId>,
    /// Accounts whose token refresh is in flight.
    pub(super) refreshes: Vec<String>,
    /// Login code exchanges in flight.
    pub(super) exchanges: usize,
}

impl InFlight {
    fn of(s: &Shared) -> InFlight {
        let mut jobs: Vec<JobId> = s.active_jobs.keys().copied().collect();
        jobs.sort_unstable();
        let mut refreshes: Vec<String> = s
            .auth
            .by_account
            .iter()
            .filter(|(_, session)| session.refresh_flight.is_some())
            .map(|(account, _)| account.clone())
            .collect();
        refreshes.sort();
        InFlight {
            jobs,
            refreshes,
            exchanges: s.auth.exchanges,
        }
    }

    fn is_empty(&self) -> bool {
        self.jobs.is_empty() && self.refreshes.is_empty() && self.exchanges == 0
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
        if !self.refreshes.is_empty() {
            parts.push(format!(
                "the token refresh of {}",
                self.refreshes.join(", ")
            ));
        }
        if self.exchanges > 0 {
            parts.push("a login's code exchange".into());
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
