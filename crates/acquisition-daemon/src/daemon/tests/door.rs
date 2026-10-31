//! C158, the door: a daemon scenario crosses the daemon's frontend
//! boundary only, and its time moves by its own hand or when nothing it
//! started can run. A scenario is a plain `#[test]` that opens a door on
//! a [`ParkingClock`] facing one fixture ([`Server`]) and runs through
//! [`DoorBuilder::run`], which owns the runtime: it builds a
//! current-thread runtime, constructs the fixture and the daemon inside
//! it, spawns the dispatcher, installs the park hook ([`IdleHook`]) and
//! awaits the scenario. Inside, the scenario sends protocol requests
//! ([`Door::request`], the path every frontend takes), awaits a job's end
//! or an auth answer on the daemon's events ([`Door::await_terminal`],
//! [`Door::await_auth`]), reads the journal ([`Door::journal`]), moves
//! the clock by its own hand where it means to ([`Door::advance`],
//! [`Door::laptop_sleep`]) and ends through the oracles
//! ([`Door::finish`]). It never polls and never yields: the runtime parks
//! whenever nothing can run, and the hook then moves the clock to its
//! next deadline unless a send or a blocking task is in flight.
//!
//! What stays off the door, each a component test that says which
//! boundary it needs (`roadmap/briefs/t5-design.md` §2.6): several
//! workers (`credentials.rs`'s keyring tests, `exit.rs`'s bound thread
//! against blocked workers and a signal's admission while `shared` is
//! held), since the hook fires per worker and never means all are idle;
//! a child process (`credentials.rs`, `exit.rs`); the daemon's own
//! frontend or callback socket, which no scripted GGG server stands in
//! for (`credentials.rs`'s protocol connections, the login's callback
//! tests, `exit.rs`'s listeners); a function or a table (`responses.rs`,
//! `idle.rs`'s verdict reads, the `Life` and `Door` tables, `queue.rs`'s
//! internal-input tests, fabricated queue and verdict states); and the
//! limiter's spec with `tests/main.rs`. A `#[tokio::test]` never uses the
//! door: its runtime would nest.
//!
//! What the door does not promise: a server that never answers. A silent
//! request holds its send permit through its real timeout, so the hook
//! leaves virtual time beside it and the scenario waits the real
//! timeout (the "Not doing" line of 2026-10-31).
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::harness::*;
use crate::daemon::*;
use crate::ratelimit::Clock;
use std::future::Future;
use std::sync::atomic::Ordering;
use tokio::sync::oneshot;

/// What the daemon faces: one of the harness's fixtures, each as it
/// stands, constructed inside the door's runtime.
pub(super) enum Server {
    /// [`scripted_server`]: one connection per response, in order.
    Script(Vec<ScriptedResponse>),
    /// [`account_server`], answering by account, holding the request
    /// `hold` names.
    #[allow(dead_code)] // the sessions' and login's scenarios, round 4
    Account((&'static str, &'static str, usize)),
    /// One of the two token servers.
    #[allow(dead_code)] // the refresh families, round 5
    Token(Token),
    /// The stateful mock ([`mockggg::start_with_clock`]) on the door's
    /// clock.
    #[allow(dead_code)] // the mock's scenarios, rounds 3 to 5
    Mock,
    /// A server the scenario built itself: its base URL. Its listener is
    /// bound before the runtime exists (a `std::net::TcpListener`, taken
    /// into the runtime with `TcpListener::from_std` inside the scenario).
    #[allow(dead_code)] // `sessions.rs`'s servers, round 4
    Own(String),
    /// No server: a base nothing listens on.
    #[allow(dead_code)] // the scenarios that send nothing, round 3
    None,
}

/// The two token servers.
#[allow(dead_code)] // the refresh families, round 5
pub(super) enum Token {
    /// [`token_server_answering`]: one `POST /token` per status, in order.
    Answering(Vec<&'static str>),
    /// [`delayed_token_server`]: one refresh, answered on release.
    Delayed,
}

/// The fixture as constructed: what the scenario reads from it or drives
/// it with ([`Door::fixture`]).
#[allow(dead_code)] // the fields the later rounds' scenarios read
pub(super) enum Fixture {
    Script {
        /// The methods the server received, in order.
        wire: Arc<Mutex<Vec<String>>>,
        server: tokio::task::JoinHandle<()>,
    },
    Account {
        sent: Arc<Mutex<Vec<Sent>>>,
        arrived: Option<oneshot::Receiver<()>>,
        release: Option<oneshot::Sender<()>>,
    },
    Token {
        /// The request bodies the server received, in order.
        bodies: Arc<Mutex<Vec<String>>>,
        server: tokio::task::JoinHandle<()>,
    },
    Delayed {
        arrived: Option<oneshot::Receiver<()>>,
        release: Option<oneshot::Sender<()>>,
        server: tokio::task::JoinHandle<()>,
    },
    Mock,
    Own,
    None,
}

/// What the park hook did over a run: readable after, asserted by
/// nothing (the measure of a scenario, not its property).
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Counters {
    /// Parks that moved the clock to its next deadline.
    pub(super) advances: usize,
    /// Parks that found a send in flight and left the clock.
    pub(super) held_by_send: usize,
    /// Parks that found a blocking task in flight and left the clock.
    pub(super) held_by_blocking: usize,
    /// Parks that found nothing parked on the clock.
    pub(super) nothing_parked: usize,
}

/// C158, the park hook: run at every park of the door's current-thread
/// runtime, which parks only when no task is runnable. It moves the
/// clock to its next deadline unless a send is in flight (the gate's
/// occupancy, from just before dispatch until the body completes) or a
/// blocking task is (`Daemon::blocking`'s count, a landing or the idle
/// verdict on its thread): then the park is the wire's or the blocking
/// thread's, and time must not move under it. Tokio's paused clock
/// advances at the same point; this is that, behind the daemon's
/// `Clock`, with no feature split (C84). The spike that found it:
/// `roadmap/briefs/4-0-spike.md`, "Answer".
struct IdleHook {
    clock: Arc<ParkingClock>,
    daemon: Arc<Daemon>,
    /// Shared with the door, which reads them ([`Door::counters`]).
    counters: Arc<Mutex<Counters>>,
}

impl IdleHook {
    fn on_idle(&mut self) {
        let mut counters = self.counters.lock().unwrap();
        if self.daemon.choke.actual_send_occupancy().0 > 0 {
            counters.held_by_send += 1;
        } else if self.daemon.blocking_in_flight.load(Ordering::SeqCst) > 0 {
            counters.held_by_blocking += 1;
        } else if self.clock.advance_to_next().is_some() {
            counters.advances += 1;
        } else {
            counters.nothing_parked += 1;
        }
    }
}

/// The slot the hook is installed in once the daemon exists.
type IdleSlot = Arc<Mutex<Option<IdleHook>>>;

/// A current-thread runtime whose park hook runs the slot's [`IdleHook`].
fn idle_runtime() -> (tokio::runtime::Runtime, IdleSlot) {
    let slot: IdleSlot = Arc::default();
    let hook = slot.clone();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .on_thread_park(move || {
            if let Some(hook) = hook.lock().unwrap().as_mut() {
                hook.on_idle();
            }
        })
        .build()
        .unwrap();
    (runtime, slot)
}

/// A door not yet opened: the fixture, the clock, the credential store
/// and what the scenario varies of the daemon, each an option of
/// `Daemon::builder`'s by the same name. Nothing is built until
/// [`DoorBuilder::run`].
#[must_use = "a door does nothing until it runs"]
pub(super) struct DoorBuilder {
    server: Server,
    clock: Arc<ParkingClock>,
    credentials: Arc<dyn CredentialStore>,
    rails: RailsConfig,
    jobs_db: Option<JobDb>,
    sessions: Sessions,
    restore_landed: bool,
    login_within: Option<Duration>,
    world: Option<(String, BoundSocket, PathBuf)>,
    events_capacity: Option<usize>,
}

/// The door a scenario holds: the daemon behind it, reached through the
/// protocol.
pub(super) struct Door {
    daemon: Arc<Daemon>,
    clock: Arc<ParkingClock>,
    /// The door's own events receiver, as a connection holds one.
    events: Option<broadcast::Receiver<JobInfo>>,
    log_path: TestPath,
    fixture: Fixture,
    dispatcher: Option<tokio::task::JoinHandle<()>>,
    /// The park hook's counters, which the hook writes.
    counters: Arc<Mutex<Counters>>,
    finished: bool,
}

/// `AuthStatus`'s answer, its fields as the protocol's `Response::Auth`
/// carries them, for [`Door::await_auth`]'s predicate.
#[derive(Debug, Clone)]
#[allow(dead_code)] // the login's scenarios read these, round 4
pub(super) struct AuthAnswer {
    pub(super) logged_in: bool,
    pub(super) pending: bool,
    pub(super) login_ok: Option<String>,
    pub(super) login_error: Option<String>,
    pub(super) username: Option<String>,
    pub(super) access_expires_in_seconds: Option<u64>,
    pub(super) keyring: String,
    pub(super) provider: String,
    pub(super) accounts: Vec<acquisition_protocol::protocol::SessionStatus>,
}

impl Door {
    /// A door facing `server` on `clock`, its daemon saving credentials
    /// into `credentials`. The daemon's options default to the harness's:
    /// the rails' default config (journaling always, to a scratch path
    /// the door supplies), an in-memory queue, one empty session, the
    /// restore landed.
    pub(super) fn open(
        server: Server,
        clock: Arc<ParkingClock>,
        credentials: Arc<dyn CredentialStore>,
    ) -> DoorBuilder {
        DoorBuilder {
            server,
            clock,
            credentials,
            rails: RailsConfig::default(),
            jobs_db: None,
            sessions: Sessions::with(AuthSession::default()),
            restore_landed: true,
            login_within: None,
            world: None,
            events_capacity: None,
        }
    }
}

#[allow(dead_code)] // the options the later rounds' scenarios vary
impl DoorBuilder {
    /// The rails' config; the door sets its `journal_path` to its own
    /// scratch journal whatever this names.
    pub(super) fn rails(mut self, config: RailsConfig) -> Self {
        self.rails = config;
        self
    }

    /// The persisted queue (C6); in memory by default.
    pub(super) fn jobs_db(mut self, jobs_db: JobDb) -> Self {
        self.jobs_db = Some(jobs_db);
        self
    }

    /// The sessions the daemon starts with; one empty session by default.
    pub(super) fn sessions(mut self, sessions: Sessions) -> Self {
        self.sessions = sessions;
        self
    }

    /// C148: whether the start's restore has landed; landed by default.
    pub(super) fn restore_landed(mut self, landed: bool) -> Self {
        self.restore_landed = landed;
        self
    }

    /// The login's browser wait (C137).
    pub(super) fn login_within(mut self, within: Duration) -> Self {
        self.login_within = Some(within);
        self
    }

    /// The world served (C83), its socket and its store directory.
    pub(super) fn world(mut self, world: String, socket: BoundSocket, store_dir: PathBuf) -> Self {
        self.world = Some((world, socket, store_dir));
        self
    }

    /// How many events a subscriber may fall behind (C85).
    pub(super) fn events_capacity(mut self, capacity: usize) -> Self {
        self.events_capacity = Some(capacity);
        self
    }

    /// The only way in, from a plain `#[test]`: the door's runtime built
    /// first, then inside it the fixture and the daemon, the dispatcher
    /// spawned and the park hook installed; then `scenario` awaited. What
    /// the hook did comes back, for a measure, never an assertion.
    pub(super) fn run<F, Fut>(self, scenario: F) -> Counters
    where
        F: FnOnce(Door) -> Fut,
        Fut: Future<Output = ()>,
    {
        let (runtime, slot) = idle_runtime();
        runtime.block_on(async {
            let door = self.build().await;
            let counters = door.counters.clone();
            *slot.lock().unwrap() = Some(IdleHook {
                clock: door.clock.clone(),
                daemon: door.daemon.clone(),
                counters: counters.clone(),
            });
            scenario(door).await;
            slot.lock().unwrap().take();
            *counters.lock().unwrap()
        })
    }

    /// The fixture, then the daemon on the clock, journaling to a scratch
    /// path, its dispatcher spawned. Inside the runtime: the servers bind
    /// their listeners on it.
    async fn build(self) -> Door {
        let (base, fixture) = match self.server {
            Server::Script(script) => {
                let (base, wire, server) = scripted_server(script).await;
                (base, Fixture::Script { wire, server })
            }
            Server::Account(hold) => {
                let (base, sent, arrived, release) = account_server(hold).await;
                let fixture = Fixture::Account {
                    sent,
                    arrived: Some(arrived),
                    release: Some(release),
                };
                (base, fixture)
            }
            Server::Token(Token::Answering(statuses)) => {
                let (base, bodies, server) = token_server_answering(statuses).await;
                (base, Fixture::Token { bodies, server })
            }
            Server::Token(Token::Delayed) => {
                let (base, arrived, release, server) = delayed_token_server().await;
                let fixture = Fixture::Delayed {
                    arrived: Some(arrived),
                    release: Some(release),
                    server,
                };
                (base, fixture)
            }
            Server::Mock => {
                let base = mockggg::start_with_clock(self.clock.clone())
                    .await
                    .expect("the mock starts");
                (base, Fixture::Mock)
            }
            Server::Own(base) => (base, Fixture::Own),
            Server::None => ("http://127.0.0.1:1".to_string(), Fixture::None),
        };
        let clock: Arc<dyn Clock> = self.clock.clone();
        let log_path = test_log_path();
        let journal = journal_of(&log_path);
        let rails = Arc::new(Rails::with_config_and_clock(
            RailsConfig {
                journal_path: Some(journal.clone()),
                ..self.rails
            },
            clock.clone(),
        ));
        assert!(
            journal.exists(),
            "the door's journal could not be opened at {}",
            journal.display()
        );
        let log = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&log_path)
            .unwrap();
        let mut builder = Daemon::builder(Provider::mock(&base), clock, self.credentials, log)
            .rails(rails)
            .sessions(self.sessions)
            .restore_landed(self.restore_landed)
            .log_path(log_path.to_str().map(str::to_string));
        if let Some(jobs_db) = self.jobs_db {
            builder = builder.jobs_db(jobs_db);
        }
        if let Some(within) = self.login_within {
            builder = builder.login_within(within);
        }
        if let Some((world, socket, store_dir)) = self.world {
            builder = builder.world(world, socket, store_dir);
        }
        if let Some(capacity) = self.events_capacity {
            builder = builder.events_capacity(capacity);
        }
        let daemon = builder.build();
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        Door {
            daemon,
            clock: self.clock,
            events: None,
            log_path,
            fixture,
            dispatcher: Some(dispatcher),
            counters: Arc::default(),
            finished: false,
        }
    }
}

impl Door {
    /// A protocol request through `handle_request`, the path every
    /// frontend's frame takes, with the door's own events receiver.
    pub(super) async fn request(&mut self, req: Request) -> Response {
        self.daemon.handle_request(req, &mut self.events).await
    }

    /// Job `id`'s end and outcome, as a client reads them: on the
    /// daemon's job events, subscribed before the first read, each event
    /// a hint and the job re-read through the protocol (`status`, then
    /// `result` once terminal). Never a poll or a yield, so the runtime
    /// parks whenever nothing can run.
    pub(super) async fn await_terminal(&mut self, id: JobId) -> (JobInfo, Outcome) {
        let mut events = self.daemon.events.subscribe();
        tokio::time::timeout(HANG, async {
            loop {
                match self.request(Request::Status { id }).await {
                    Response::Status { job } if job.state.is_terminal() => {
                        match self.request(Request::Result { id }).await {
                            Response::Result { outcome, .. } => return (job, outcome),
                            other => panic!("job {id} is terminal and its result reads {other:?}"),
                        }
                    }
                    Response::Status { .. } => {}
                    other => panic!("job {id}'s status reads {other:?}"),
                }
                match events.recv().await {
                    Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => {
                        panic!("the daemon's events closed")
                    }
                }
            }
        })
        .await
        .expect("the job ended")
    }

    /// The first `AuthStatus` answer `predicate` holds for: at once if
    /// it already does, else re-read after each auth-change notification
    /// (subscribed before the read), the notification a hint and the
    /// answer the truth. A login's end is `|s| !s.pending`.
    #[allow(dead_code)] // the login's scenarios, round 4
    pub(super) async fn await_auth(
        &mut self,
        predicate: impl Fn(&AuthAnswer) -> bool,
    ) -> AuthAnswer {
        let daemon = self.daemon.clone();
        tokio::time::timeout(HANG, async {
            loop {
                let changed = daemon.auth_changed.notified();
                tokio::pin!(changed);
                changed.as_mut().enable();
                let answer = match self.request(Request::AuthStatus).await {
                    Response::Auth {
                        logged_in,
                        pending,
                        login_ok,
                        login_error,
                        username,
                        access_expires_in_seconds,
                        keyring,
                        provider,
                        accounts,
                    } => AuthAnswer {
                        logged_in,
                        pending,
                        login_ok,
                        login_error,
                        username,
                        access_expires_in_seconds,
                        keyring,
                        provider,
                        accounts,
                    },
                    other => panic!("auth status reads {other:?}"),
                };
                if predicate(&answer) {
                    return answer;
                }
                changed.await;
            }
        })
        .await
        .expect("the auth answer came")
    }

    /// The journal's sends so far, header dropped, as the oracles read
    /// them.
    pub(super) fn journal(&self) -> Vec<Value> {
        journal_sends(&self.log_path)
    }

    /// Time passes by the scenario's hand: both faces move, every sleeper
    /// due wakes.
    #[allow(dead_code)] // a verdict read at a moment, rounds 2 and 5
    pub(super) fn advance(&self, duration: Duration) {
        self.clock.advance(duration);
    }

    /// The lid closes: the wall clock moves, the monotonic clock does not.
    #[allow(dead_code)] // the laptop sleep's rows, round 5
    pub(super) fn laptop_sleep(&self, duration: Duration) {
        self.clock.laptop_sleep(duration);
    }

    /// The fixture as constructed, for what a scenario reads from it or
    /// drives it with (a hold's release, a recorder).
    #[allow(dead_code)] // the later rounds' scenarios
    pub(super) fn fixture(&mut self) -> &mut Fixture {
        &mut self.fixture
    }

    /// The park hook's counters as they stand, shared: for a component
    /// test of the hook's rule, read from another thread while the
    /// runtime parks. A scenario asserts nothing of them.
    pub(super) fn counters(&self) -> Arc<Mutex<Counters>> {
        self.counters.clone()
    }

    /// The daemon itself, for the component tests that share a door. Not
    /// for a scenario's assertions: a scenario reads the protocol's
    /// answers and the journal (C158).
    pub(super) fn daemon(&self) -> &Arc<Daemon> {
        &self.daemon
    }

    /// The teardown, run always: a server that answers a script is
    /// awaited to its end (its assertions are the scenario's), the
    /// journal must exist, and the oracles the server kind selects run:
    /// the contract oracle with "never over the limit" against the mock,
    /// with "pacing follows responses" otherwise, exact since the door's
    /// clock is manual; a script's journal is held equal to its wire too.
    pub(super) async fn finish(mut self) {
        self.finished = true;
        let wire = match &mut self.fixture {
            Fixture::Script { wire, server } => {
                tokio::time::timeout(HANG, server)
                    .await
                    .expect("the script was answered whole")
                    .expect("the scripted server's assertions held");
                Some(wire.lock().unwrap().clone())
            }
            Fixture::Token { server, .. } | Fixture::Delayed { server, .. } => {
                tokio::time::timeout(HANG, server)
                    .await
                    .expect("the token server was answered whole")
                    .expect("the token server's assertions held");
                None
            }
            _ => None,
        };
        assert!(
            journal_of(&self.log_path).exists(),
            "the door's journal is missing at its finish"
        );
        if let Some(wire) = wire {
            assert_journal_matches_wire(&self.log_path, &wire);
        }
        if matches!(self.fixture, Fixture::Mock) {
            finish_harness_mock(self.dispatcher.take(), &self.log_path);
        } else {
            finish_harness(self.dispatcher.take(), &self.log_path);
        }
    }
}

impl Drop for Door {
    fn drop(&mut self) {
        if !self.finished && !std::thread::panicking() {
            panic!("a door scenario ends through `finish`, so its oracles run");
        }
    }
}
