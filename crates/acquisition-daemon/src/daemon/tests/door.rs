//! C158, the door: a daemon scenario crosses the daemon's frontend
//! boundary only, and its time moves by its own hand or when nothing it
//! started can run. A scenario is a plain `#[test]` that opens a door on
//! a [`ParkingClock`] facing one fixture ([`Server`]), an in-process
//! transport of the test support's (C161, `crate::transport`): no
//! provider exchange in a scenario is real I/O, and a request the
//! fixture holds holds the clock. It runs through
//! [`DoorBuilder::run`], which owns the runtime: it builds a
//! current-thread runtime, constructs the fixture and the daemon inside
//! it, spawns the dispatcher (and, when the scenario asks, the
//! lifecycle's driver, [`DoorBuilder::life`]), installs the park hook
//! ([`IdleHook`]) and awaits the scenario. Inside, the scenario sends
//! protocol requests ([`Door::request`], the path every frontend takes)
//! and the stop a frontend's `acq daemon stop` sends ([`Door::stop`]),
//! awaits a job's state, its end or an auth answer on the daemon's events
//! ([`Door::await_job`], [`Door::await_terminal`], [`Door::await_auth`])
//! and the exit's leave ([`Door::await_leave`]), subscribes to the job
//! events as a frontend does ([`Door::subscribe`]), awaits the runtime
//! settled where a state must hold once the daemon has done all it can
//! ([`Door::settle`]), reads a second lifetime through a door opened
//! beside it ([`DoorBuilder::beside`]) or a restart's through a door
//! that restores as a start does ([`DoorBuilder::restore`]), serves a
//! world whose directory holds what a previous lifetime left
//! ([`DoorBuilder::world`]), reads the
//! journal ([`Door::journal`]) and what a scripted transport received
//! ([`Door::received`]), moves
//! the clock by its own hand where it means to ([`Door::advance`],
//! [`Door::laptop_sleep`]) and ends through the oracles
//! ([`Door::finish`]). It never polls and never yields: the runtime parks
//! whenever nothing can run, and the hook then moves the clock to its
//! next deadline unless a fixture's transport holds a request or a
//! blocking task is in flight. A scenario that waits on something
//! outside the daemon that neither count sees (a browser's callback to
//! the daemon's own listener) holds the clock across that wait
//! ([`Door::hold_clock`]), so the hook moves no deadline, the idle
//! deadline of [`DoorBuilder::life`] among them, under it. C160, the
//! verdict: a park at which nothing is parked on the clock and no held
//! request, blocking task or hold is pending is one nothing the hook accounts for
//! can wake, so a door wait pending there fails at that park, naming
//! what it waited for, with `HANG` the backstop behind it for a wait the
//! hook does not read; a door opened [`DoorBuilder::beside`] another
//! registers with the first door's hook, so its waits have the verdict
//! too.
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
use crate::daemon::server::InFlight;
use crate::daemon::*;
use crate::mockggg::HttpRequest;
use crate::ratelimit::Clock;
use crate::transport::{self, Ends, Hold, InProcess, Reply};
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::oneshot;

/// What the daemon faces (C161): one of the test support's in-process
/// transports (`crate::transport`), answering at its base, every request
/// it holds a hold the park hook sees.
pub(super) enum Server {
    /// The scripted transport: one response per request, in order.
    Script(Vec<ScriptedResponse>),
    /// The account transport, answering by account, holding the request
    /// `hold` names.
    Account((&'static str, &'static str, usize)),
    /// One of the token transports.
    Token(Token),
    /// The stateful mock's in-process face, on the door's clock.
    Mock,
    /// A transport the scenario builds itself ([`Server::own`]): its
    /// closure answers each request. What it records and the barriers it
    /// holds are the scenario's own, captured before (an `Arc`, a
    /// oneshot); the door reads nothing of it and awaits nothing of it
    /// at [`Door::finish`].
    Own(OwnServer),
    /// No fixture: the refusing transport.
    None,
}

/// [`Server::Own`]'s transport, built on the door's hold.
pub(super) type OwnServer = Box<dyn FnOnce(&Hold) -> InProcess>;

impl Server {
    /// [`Server::Own`] from a closure answering each request, as
    /// [`InProcess::new`] takes it: what it records before its first
    /// await is recorded in the send's first poll, and the hold spans
    /// the request it answers like any fixture's.
    pub(super) fn own<F, Fut>(respond: F) -> Server
    where
        F: Fn(HttpRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Reply> + Send + 'static,
    {
        Server::Own(Box::new(move |hold| InProcess::new(hold, respond)))
    }
}

/// A scenario's hold on the clock ([`Door::hold_clock`]), released when
/// dropped.
#[must_use = "the clock is held only while the guard stands"]
pub(super) struct ClockHold(Arc<AtomicUsize>);

impl Drop for ClockHold {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

/// The token transport a scenario holds a refresh at (a scripted token
/// answer is [`Server::Script`]'s, through [`token_answer`]).
pub(super) enum Token {
    /// [`transport::delayed_token`]: one refresh of `rt-old`, answered on
    /// release with the status and the body given (a grant's body is
    /// [`token_body`]'s).
    Delayed(&'static str, String),
}

/// The fixture as constructed: what the scenario reads from it or drives
/// it with ([`Door::fixture`]).
#[allow(dead_code)] // the fields the later rounds' scenarios read
pub(super) enum Fixture {
    Script {
        /// The methods the transport received, in order.
        wire: Arc<Mutex<Vec<String>>>,
        /// Each request whole, in order ([`Door::received`]).
        received: Arc<Mutex<Vec<Received>>>,
        /// Its faults and its end, the script answered whole.
        ends: Ends,
    },
    Account {
        sent: Arc<Mutex<Vec<Sent>>>,
        arrived: Option<oneshot::Receiver<()>>,
        release: Option<oneshot::Sender<()>>,
    },
    Delayed {
        arrived: Option<oneshot::Receiver<()>>,
        release: Option<oneshot::Sender<()>>,
        /// Its faults and its end, the refresh answered.
        ends: Ends,
    },
    /// The mock's state, which [`Door::browser`] reads its pages from.
    Mock(mockggg::Mock),
    Own,
    None,
}

/// What the park hook did over a run: readable after, a measure of a
/// scenario and never its property (asserted only by the hook's own pin,
/// `c160_a_wait_the_daemon_will_never_answer_fails_at_the_park`).
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Counters {
    /// Parks that moved the clock to its next deadline.
    pub(super) advances: usize,
    /// Parks that found a request a test transport holds and left the
    /// clock.
    pub(super) held_by_transport: usize,
    /// Parks that found a blocking task in flight and left the clock.
    pub(super) held_by_blocking: usize,
    /// Parks that found the scenario holding the clock
    /// ([`Door::hold_clock`]) and left it.
    pub(super) held_by_scenario: usize,
    /// Parks that found nothing parked on the clock.
    pub(super) nothing_parked: usize,
    /// Of those, the parks that found a door wait pending and failed it
    /// (C160, the verdict).
    pub(super) stuck: usize,
    /// Parks that found the scenario awaiting a settled runtime
    /// ([`Door::settle`]) and woke it in the clock's place.
    pub(super) settles: usize,
    /// Parks that found the scenario awaiting a quiet runtime
    /// ([`Door::quiet`]) and woke it, whatever held the clock.
    pub(super) quiets: usize,
}

/// C158, the park hook: run at every park of the door's current-thread
/// runtime, which parks only when no task is runnable. It reads every
/// door registered on the runtime, the first and any opened
/// [`DoorBuilder::beside`] it ([`Accounting`]). It moves the clock to
/// its next deadline unless a test transport holds a request (C160 as
/// amended: a door's [`Hold`], from the send's first poll until its body
/// completes or it is dropped) or a blocking task is
/// (`Daemon::blocking`'s count, a landing or the idle verdict on its
/// thread): then the park is the fixture's or the blocking thread's, and
/// time must not move under it; nor while a scenario holds the clock
/// across a wait of its own ([`Door::hold_clock`]). When a scenario
/// awaits a quiet runtime ([`Door::quiet`]), the park wakes it before
/// anything else is asked, whatever holds the clock, and moves nothing.
/// When a scenario awaits a settled runtime ([`Door::settle`]), a park
/// that would move the clock wakes the scenario instead and moves
/// nothing. C160: a park that finds none of these and nothing parked on
/// the clock is one nothing the hook accounts for can wake, and fails
/// every door wait pending there (the verdict, [`Door::await_job`] and
/// its siblings). Tokio's paused clock
/// advances at the same point; this is that, behind the daemon's
/// `Clock`, with no feature split (C84). The spike that found it:
/// `roadmap/briefs/4-0-spike.md`, "Answer".
struct IdleHook {
    clock: Arc<ParkingClock>,
    /// The doors the hook accounts for: the first, and any opened beside
    /// it, each registered whole before it waits on anything.
    doors: Doors,
    /// Shared with the doors, which read them ([`Door::counters`]).
    counters: Arc<Mutex<Counters>>,
    /// [`COUNTERS_VAR`] set: every park that did not advance the clock is
    /// printed.
    print: bool,
}

/// One door's accounting, as the park hook reads it (C160): its daemon's
/// blocking count, its transports' hold, its scenario's holds, and its
/// `quiet`, `settle` and stuck slots.
struct Accounting {
    /// Which door, so its `finish` unregisters it alone.
    door: usize,
    daemon: Arc<Daemon>,
    /// The requests the door's transports hold.
    hold: Hold,
    /// The scenario's holds ([`Door::hold_clock`]).
    holds: Arc<AtomicUsize>,
    /// The scenario's wait for a settled runtime ([`Door::settle`]).
    settle: SettleSlot,
    /// The scenario's wait for a quiet runtime ([`Door::quiet`]).
    quiet: SettleSlot,
    /// The door wait pending, which a stuck park fails (C160).
    stuck: SettleSlot,
}

/// The doors registered with a runtime's park hook.
type Doors = Arc<Mutex<Vec<Accounting>>>;

/// Test support, no knob: when set, the hook prints to stderr, at the
/// park itself, one line for every park that did not advance the clock,
/// and [`DoorBuilder::run`] prints the counters at its end, each line
/// naming the scenario, so a scenario killed mid-wait leaves its last
/// park's branch in the log (the measure of `briefs/151-stuck-verdict.md`).
const COUNTERS_VAR: &str = "ACQ_DOOR_COUNTERS";

/// The scenario's name, as the test harness names its thread.
fn scenario_name() -> String {
    std::thread::current()
        .name()
        .unwrap_or("unnamed")
        .to_string()
}

impl Counters {
    /// One line of [`COUNTERS_VAR`]'s output: `what` at the scenario,
    /// then every count.
    fn print(&self, what: &str) {
        let Counters {
            advances,
            held_by_transport,
            held_by_blocking,
            held_by_scenario,
            nothing_parked,
            stuck,
            settles,
            quiets,
        } = self;
        eprintln!(
            "door-counters {} {what} advances={advances} held_by_transport={held_by_transport} \
             held_by_blocking={held_by_blocking} held_by_scenario={held_by_scenario} \
             settles={settles} quiets={quiets} nothing_parked={nothing_parked} stuck={stuck}",
            scenario_name()
        );
    }
}

/// Where [`Door::settle`] and [`Door::quiet`] leave their wakers for the
/// park hook, and a door wait its verdict's.
type SettleSlot = Arc<Mutex<Option<oneshot::Sender<()>>>>;

impl IdleHook {
    fn on_idle(&mut self) {
        let doors = self.doors.lock().unwrap();
        let mut counters = self.counters.lock().unwrap();
        let branch = if let Some(quiet) = doors.iter().find_map(|d| d.quiet.lock().unwrap().take())
        {
            // Before the holds: a quiet park is the runtime's, whatever
            // holds the clock; it moves nothing.
            let _ = quiet.send(());
            counters.quiets += 1;
            "quiet"
        } else if doors.iter().any(|d| d.holds.load(Ordering::SeqCst) > 0) {
            counters.held_by_scenario += 1;
            "held_by_scenario"
        } else if doors.iter().any(|d| d.hold.count() > 0) {
            counters.held_by_transport += 1;
            "held_by_transport"
        } else if doors
            .iter()
            .any(|d| d.daemon.blocking_in_flight.load(Ordering::SeqCst) > 0)
        {
            counters.held_by_blocking += 1;
            "held_by_blocking"
        } else if let Some(settled) = doors.iter().find_map(|d| d.settle.lock().unwrap().take()) {
            // Tokio runs a task an `on_thread_park` hook wakes instead of
            // parking, so the scenario resumes with nothing else runnable.
            let _ = settled.send(());
            counters.settles += 1;
            "settle"
        } else if self.clock.advance_to_next().is_some() {
            counters.advances += 1;
            return;
        } else {
            // C160, the verdict: nothing the hook accounts for is pending,
            // so nothing it knows of can wake the daemon; a door wait
            // pending here will not be answered.
            counters.nothing_parked += 1;
            let stuck: Vec<_> = doors
                .iter()
                .filter_map(|d| d.stuck.lock().unwrap().take())
                .collect();
            if stuck.is_empty() {
                "nothing_parked"
            } else {
                for stuck in stuck {
                    let _ = stuck.send(());
                }
                counters.stuck += 1;
                "stuck"
            }
        };
        if self.print {
            counters.print(&format!("park={branch}"));
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
    /// The start's restore begun at build and awaited before the scenario
    /// ([`Self::restore`]).
    restore: bool,
    restore_jobs: Option<Retention>,
    login_within: Option<Duration>,
    /// The provider named as the real one on the fixture's base
    /// ([`Self::real_named`]).
    real_named: bool,
    /// The world served and its store directory ([`Self::world`]); its
    /// endpoint is bound inside the runtime.
    world: Option<(String, PathBuf)>,
    store_dir: Option<PathBuf>,
    events_capacity: Option<usize>,
    /// The lifecycle's driver's idle knob, when the scenario asks for
    /// the driver.
    life: Option<Duration>,
    exit_bound: Option<Duration>,
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
    /// The lifecycle's driver, when the scenario asked for it, and its
    /// recorded leave.
    life: Option<tokio::task::JoinHandle<()>>,
    leave: Option<oneshot::Receiver<Option<InFlight>>>,
    /// The park hook's counters, which the hook writes (the first door's,
    /// shared by a door beside it).
    counters: Arc<Mutex<Counters>>,
    /// This door's number among the doors the hook accounts for.
    id: usize,
    /// The doors the runtime's hook accounts for, this one among them
    /// from its build to its `finish`.
    doors: Doors,
    /// The scenario's holds on the clock, which the hook reads.
    holds: Arc<AtomicUsize>,
    /// The requests the door's transports hold, which the hook reads.
    hold: Hold,
    /// The scenario's wait for a settled runtime, which the hook wakes.
    settle: SettleSlot,
    /// The scenario's wait for a quiet runtime, which the hook wakes.
    quiet: SettleSlot,
    /// The door wait pending, which the hook fails at a stuck park.
    stuck: SettleSlot,
    finished: bool,
    /// The world's endpoint ([`DoorBuilder::world`]) and, on Unix, the
    /// folder its socket is in: held open while the door stands.
    _endpoint: Option<(Listener, Option<TestPath>)>,
}

/// `AuthStatus`'s answer, its fields as the protocol's `Response::Auth`
/// carries them, for [`Door::await_auth`]'s predicate.
#[derive(Debug, Clone)]
#[allow(dead_code)] // `provider`: read by no scenario yet
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
            restore: false,
            restore_jobs: None,
            login_within: None,
            real_named: false,
            world: None,
            store_dir: None,
            events_capacity: None,
            life: None,
            exit_bound: None,
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

    /// C148, the start's restore as a start runs it: the daemon built with
    /// its restore not landed, then `begin_restore` (a counted landing on
    /// a blocking thread, so the park hook holds the clock under it)
    /// before the dispatcher starts, reading the door's credential store
    /// and [`Self::store_dir`] on the door's clock's wall (no store
    /// directory, no session). [`Self::run`] awaits the landing, the
    /// keyring word no longer `restoring` (`AuthStatus`), before the
    /// scenario runs, so the scenario meets a restored daemon as a
    /// frontend does. The restore replaces the sessions [`Self::sessions`]
    /// gave.
    pub(super) fn restore(mut self) -> Self {
        self.restore = true;
        self.restore_landed = false;
        self
    }

    /// C6: the persisted queue restored before the dispatcher starts, as
    /// a start restores it (`Daemon::restore_jobs`, pruned by
    /// `retention`): for a second lifetime on the first's
    /// [`Self::jobs_db`], which alone installs the table and restores
    /// nothing. A restore that fails fails the door's construction.
    pub(super) fn restore_jobs(mut self, retention: Retention) -> Self {
        self.restore_jobs = Some(retention);
        self
    }

    /// The store directory the daemon records into (the profile's facts,
    /// the account index), without a world; none by default, so nothing
    /// is recorded.
    pub(super) fn store_dir(mut self, store_dir: PathBuf) -> Self {
        self.store_dir = Some(store_dir);
        self
    }

    /// The real provider's name on the fixture's base: `Provider { name:
    /// GGG, ..Provider::mock(&base) }`, so `is_real()` holds and the
    /// daemon judges what a client may submit as in real mode (C127, the
    /// mock-only kinds refused), while every URL it renders is the
    /// fixture's and every send reaches the fixture's record. The rule:
    /// never `Provider::ggg()`, which carries the production URLs, so a
    /// door on it would send to GGG and the fixture's empty record would
    /// prove nothing. Off by default: the mock's name on the same base.
    pub(super) fn real_named(mut self) -> Self {
        self.real_named = true;
        self
    }

    /// The login's browser wait (C137).
    pub(super) fn login_within(mut self, within: Duration) -> Self {
        self.login_within = Some(within);
        self
    }

    /// The world served (C83) and its store directory, as a start serves
    /// one: what a daemon with a world alone does follows (C134, the fault
    /// marker beside `daemon.db` read at the restore). Its endpoint is
    /// bound inside the door's runtime before the daemon is built, as
    /// [`Server::own`]'s closure runs there ([`door_endpoint`]: a socket
    /// in a folder of its own on Unix, a pipe named for the store
    /// directory on Windows); nothing connects to it, since a scenario
    /// enters through [`Door::request`], and it closes with the door.
    pub(super) fn world(mut self, world: String, store_dir: PathBuf) -> Self {
        self.world = Some((world, store_dir));
        self
    }

    /// How many events a subscriber may fall behind (C85).
    pub(super) fn events_capacity(mut self, capacity: usize) -> Self {
        self.events_capacity = Some(capacity);
        self
    }

    /// C157: the lifecycle's driver, spawned beside the dispatcher with
    /// the idle knob at `knob`, its leave recorded for
    /// [`Door::await_leave`] instead of ending the process. Off by
    /// default: a scenario that does not ask has no idle deadline on the
    /// clock, so the hook never moves time to it.
    pub(super) fn life(mut self, knob: Duration) -> Self {
        self.life = Some(knob);
        self
    }

    /// C133: the exit's bound, on the daemon's clock; the daemon's
    /// constant by default.
    pub(super) fn exit_bound(mut self, bound: Duration) -> Self {
        self.exit_bound = Some(bound);
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
        let restore = self.restore;
        let (runtime, slot) = idle_runtime();
        runtime.block_on(async {
            let mut door = self.build().await;
            let counters = door.counters.clone();
            door.register();
            *slot.lock().unwrap() = Some(IdleHook {
                clock: door.clock.clone(),
                doors: door.doors.clone(),
                counters: counters.clone(),
                print: std::env::var_os(COUNTERS_VAR).is_some(),
            });
            if restore {
                door.await_restored().await;
            }
            scenario(door).await;
            slot.lock().unwrap().take();
            let counters = *counters.lock().unwrap();
            if std::env::var_os(COUNTERS_VAR).is_some() {
                counters.print("end");
            }
            counters
        })
    }

    /// A second door, opened inside `first`'s scenario on the runtime
    /// that scenario runs on (a second `run` would nest one runtime in
    /// another): the fixture and the daemon built as [`Self::run`] builds
    /// them, the dispatcher spawned, on `first`'s clock ([`Door::clock`];
    /// a door opened on another is refused, before anything is built, in
    /// words that say so). It has no hook of its own: it registers its
    /// whole accounting with the hook `first` runs under before any
    /// restore wait (its daemon's blocking count, its transports' hold,
    /// its scenario's holds, its `quiet`, `settle` and stuck slots) and
    /// unregisters at its [`Door::finish`], so its waits gain the
    /// verdict (C160) like `first`'s, `HANG` the backstop behind it. For
    /// a second lifetime read while the first is live (a restart's view
    /// of the same rails state path, C129, or of the same queue file).
    pub(super) fn beside<'a>(self, first: &'a Door) -> impl Future<Output = Door> + 'a {
        assert!(
            Arc::ptr_eq(&self.clock, &first.clock),
            "a door beside another runs on the first door's clock (`Door::clock`), so the \
             hook that moves it accounts for both; this one was opened on another clock"
        );
        async move {
            let restore = self.restore;
            let mut door = self.build().await;
            door.counters = first.counters.clone();
            door.doors = first.doors.clone();
            door.register();
            if restore {
                door.await_restored().await;
            }
            door
        }
    }

    /// The fixture's transport on the door's hold, then the daemon on the
    /// clock over it, journaling to a scratch path, its dispatcher
    /// spawned.
    async fn build(self) -> Door {
        let hold = Hold::new();
        let (transport, fixture) = match self.server {
            Server::Script(script) => {
                let script = script.into_iter().map(Into::into).collect();
                let (transport, record) = transport::scripted(&hold, script);
                let fixture = Fixture::Script {
                    wire: record.wire,
                    received: record.received,
                    ends: record.ends,
                };
                (transport, fixture)
            }
            Server::Account(held) => {
                let (transport, record) = transport::account(&hold, held);
                let fixture = Fixture::Account {
                    sent: record.sent,
                    arrived: Some(record.arrived),
                    release: Some(record.release),
                };
                (transport, fixture)
            }
            Server::Token(Token::Delayed(status, body)) => {
                let (transport, record) = transport::delayed_token(&hold, status, body);
                let fixture = Fixture::Delayed {
                    arrived: Some(record.arrived),
                    release: Some(record.release),
                    ends: record.ends,
                };
                (transport, fixture)
            }
            Server::Mock => {
                let (transport, mock) = transport::mock(&hold, self.clock.clone());
                (transport, Fixture::Mock(mock))
            }
            Server::Own(build) => (build(&hold), Fixture::Own),
            Server::None => (transport::refusing(&hold), Fixture::None),
        };
        let base = transport::BASE;
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
        let provider = if self.real_named {
            Provider {
                name: acquisition_protocol::provider::GGG,
                ..Provider::mock(base)
            }
        } else {
            Provider::mock(base)
        };
        let mut builder = Daemon::builder(provider, clock, self.credentials, log)
            .transport(Box::new(transport))
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
        let mut endpoint = None;
        if let Some((world, store_dir)) = self.world {
            let (socket, folder) = door_endpoint(&store_dir);
            let (listener, bound) = Listener::bind(&socket)
                .await
                .expect("the door binds its world's endpoint");
            builder = builder.world(world, bound, store_dir);
            endpoint = Some((listener, folder));
        }
        if let Some(store_dir) = self.store_dir {
            builder = builder.store_dir(store_dir);
        }
        if let Some(capacity) = self.events_capacity {
            builder = builder.events_capacity(capacity);
        }
        if let Some(bound) = self.exit_bound {
            builder = builder.exit(Exit::with_bound(bound));
        }
        let daemon = builder.build();
        if let Some(retention) = self.restore_jobs {
            daemon
                .restore_jobs(retention)
                .expect("the door's daemon restores its queue");
        }
        if self.restore {
            daemon.begin_restore(None);
        }
        let dispatcher = tokio::spawn(daemon.clone().dispatcher());
        let (life, leave) = match self.life {
            Some(knob) => {
                let (life, leave) = spawn_life(&daemon, knob);
                (Some(life), Some(leave))
            }
            None => (None, None),
        };
        Door {
            daemon,
            clock: self.clock,
            events: None,
            log_path,
            fixture,
            dispatcher: Some(dispatcher),
            life,
            leave,
            counters: Arc::default(),
            id: NEXT_DOOR.fetch_add(1, Ordering::SeqCst),
            doors: Arc::default(),
            holds: Arc::default(),
            hold,
            settle: Arc::default(),
            quiet: Arc::default(),
            stuck: Arc::default(),
            finished: false,
            _endpoint: endpoint,
        }
    }
}

/// [`DoorBuilder::world`]'s endpoint: on Unix a socket `door.sock` in a
/// folder of its own, the folder held with the listener and removed with
/// the door (a path short enough for a socket, which a store directory
/// need not be).
#[cfg(unix)]
fn door_endpoint(_store_dir: &std::path::Path) -> (PathBuf, Option<TestPath>) {
    let socket = TestPath::file("door.sock");
    (socket.to_path_buf(), Some(socket))
}

/// [`DoorBuilder::world`]'s endpoint: on Windows a named pipe, which the
/// world names `\\.\pipe\acq-<SID>-<world id>` and `Listener::bind`
/// creates from the name given, so the door names its own from the
/// process, the store directory's folder name and a count, unique among
/// the tests that run at once and the doors one test opens in turn.
#[cfg(windows)]
fn door_endpoint(store_dir: &std::path::Path) -> (PathBuf, Option<TestPath>) {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let folder = store_dir
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let name = format!(
        r"\\.\pipe\acq-door-{}-{folder}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    );
    (PathBuf::from(name), None)
}

/// The doors' numbers, one per door built.
static NEXT_DOOR: AtomicUsize = AtomicUsize::new(0);

impl Door {
    /// This door's accounting, registered with the hook its runtime runs
    /// under.
    fn register(&self) {
        self.doors.lock().unwrap().push(Accounting {
            door: self.id,
            daemon: self.daemon.clone(),
            hold: self.hold.clone(),
            holds: self.holds.clone(),
            settle: self.settle.clone(),
            quiet: self.quiet.clone(),
            stuck: self.stuck.clone(),
        });
    }

    /// The door's clock, which a door opened beside it runs on
    /// ([`DoorBuilder::beside`]).
    pub(super) fn clock(&self) -> Arc<ParkingClock> {
        self.clock.clone()
    }

    /// A protocol request through `handle_request`, the path every
    /// frontend's frame takes, with the door's own events receiver.
    pub(super) async fn request(&mut self, req: Request) -> Response {
        self.daemon.handle_request(req, &mut self.events).await
    }

    /// C85, a frontend's subscription: `Subscribe` sent through
    /// [`Self::request`], answered `subscribed`, and the daemon's job
    /// events from then on, as the subscribed connection carries them.
    pub(super) async fn subscribe(&mut self) -> broadcast::Receiver<JobInfo> {
        match self.request(Request::Subscribe).await {
            Response::Subscribed => {}
            other => panic!("a subscribe answered {other:?}"),
        }
        self.events
            .take()
            .expect("the subscription's receiver, which `subscribe` installs")
    }

    /// C158: the runtime settled. Resolves at the first park at which the
    /// hook would move the clock or find nothing parked: no task can run,
    /// no transport holds a request, no blocking task is in flight, and
    /// the scenario does not hold the clock. That park moves nothing; the scenario resumes
    /// in its place. For a state that must hold once the daemon has done
    /// all it can without time moving (a halted daemon's sibling left
    /// waiting), never a sleep that stands for it. Under a request the
    /// scenario holds at its own fixture it never resolves (the held
    /// request leaves the clock, and settling waits on it): that wait is
    /// [`Self::quiet`]'s.
    pub(super) async fn settle(&self) {
        let (settled, wake) = oneshot::channel();
        let previous = self.settle.lock().unwrap().replace(settled);
        assert!(previous.is_none(), "one settle at a time");
        tokio::time::timeout(HANG, wake)
            .await
            .expect("the runtime settled")
            .expect("the park hook woke the settle");
    }

    /// C158: the runtime quiet. Resolves at the next park, at which no
    /// task can run, whatever holds the clock (a request a transport
    /// holds, a blocking task, a hold of the scenario's); that park moves
    /// nothing, and the scenario resumes in its place. The rule: a
    /// scenario whose assertion depends on a request having reached its
    /// fixture calls it only after that request's arrival barrier has
    /// fired (`Fixture::Account`'s or `Fixture::Delayed`'s `arrived`),
    /// never before: a park on the way to the transport (a pacing sleep,
    /// a credential read on its blocking thread) still resolves it first.
    /// Under that held request every task the daemon runs for a request
    /// has either answered or parked on what it waits for (the refresh
    /// flight's result, the gate's queue), so the park is the runtime's
    /// own word that the waiters joined and a parked job reached the
    /// gate, never a count of receivers. Where nothing is held and the
    /// state must hold once the daemon has done all it can, the wait is
    /// [`Self::settle`]'s.
    pub(super) async fn quiet(&self) {
        let (quiet, wake) = oneshot::channel();
        let previous = self.quiet.lock().unwrap().replace(quiet);
        assert!(previous.is_none(), "one quiet at a time");
        tokio::time::timeout(HANG, wake)
            .await
            .expect("the runtime went quiet")
            .expect("the park hook woke the quiet");
    }

    /// The browser, as the user's: a `GET` of `url`, following one
    /// redirect (the mock's `/approve` answers one, to the daemon's
    /// callback), its last answer read as the user reads the page: the
    /// status and the body. A page of the mock's (`url` at the fixture's
    /// base) is read from the door's mock in-process
    /// ([`transport::mock_page`]); a page of the daemon's callback
    /// listener is a real `GET`, under [`Self::hold_clock`], since that
    /// listener is a wait no count of the hook's sees.
    pub(super) async fn browser(&self, url: &str) -> (reqwest::StatusCode, String) {
        let mut url = url.to_string();
        let mut redirects = 1;
        if url.starts_with(transport::BASE) {
            let Fixture::Mock(mock) = &self.fixture else {
                panic!("the browser reads a page of the fixture's base from the mock alone")
            };
            let (status, headers, page) =
                transport::mock_page(mock, &url).expect("the mock's page reads");
            if !status.is_redirection() {
                return (status, page);
            }
            url = headers[reqwest::header::LOCATION]
                .to_str()
                .expect("the redirect's target reads")
                .to_string();
            redirects = 0;
        }
        let _hold = self.hold_clock();
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(redirects))
            .build()
            .expect("the browser builds");
        // A wait the scenario starts off the daemon is bounded whole:
        // the answer and its page under one `HANG`, never by halves.
        tokio::time::timeout(HANG, async {
            let answered = client
                .get(&url)
                .send()
                .await
                .expect("the browser's request went through");
            let status = answered.status();
            let page = answered.text().await.expect("the page reads");
            (status, page)
        })
        .await
        .expect("the browser was answered, its page read whole")
    }

    /// A protocol request on a connection of its own, spawned: a second
    /// frontend's, which pends while the daemon holds its answer (a
    /// refresh on the wire) and is answered whenever the daemon answers.
    pub(super) fn spawn_request(&self, req: Request) -> tokio::task::JoinHandle<Response> {
        let daemon = self.daemon.clone();
        tokio::spawn(async move { daemon.handle_request(req, &mut None).await })
    }

    /// C158: hold the clock until the guard drops: while it stands, the
    /// park hook moves no deadline. For a wait of the scenario's own that
    /// neither the transports' hold nor the blocking count sees: a real
    /// `GET` to the daemon's callback listener ([`Self::browser`]'s, or a
    /// scenario's hand-rolled one), so the time the scenario reads is the
    /// time it moved, not the time the hook found parked beside it; or a
    /// scenario's deliberate freeze of the clock while work runs beside
    /// parked waits.
    pub(super) fn hold_clock(&self) -> ClockHold {
        self.holds.fetch_add(1, Ordering::SeqCst);
        ClockHold(self.holds.clone())
    }

    /// C133, what a frontend's `acq daemon stop` asks on the bootstrap
    /// plane, which `Request` does not carry: the exit begun as the stop's
    /// connection begins it (`request_stop`), its event handed to the
    /// lifecycle's driver. The stop's connection then pends until the
    /// process ends; the scenario reads that end through
    /// [`Self::await_leave`].
    pub(super) fn stop(&self) {
        self.daemon.request_stop();
    }

    /// C133: the exit's leave, observed: the lifecycle's driver reached
    /// its last act, which a running daemon performs by ending the
    /// process. Nothing comes back: what the drain cut is read through
    /// the protocol (a cut job is still Running, a clean one is Done) and
    /// the journal. Needs [`DoorBuilder::life`]. Fails at the park where
    /// nothing the hook accounts for can bring the leave (C160), `HANG`
    /// the backstop behind that verdict.
    pub(super) async fn await_leave(&mut self) {
        let leave = self
            .leave
            .take()
            .expect("the door runs the lifecycle's driver (`DoorBuilder::life`), once");
        self.or_stuck("awaiting the exit's leave".into(), "the exit left", leave)
            .await
            .expect("the driver reached its leave");
    }

    /// Job `id` as `Status` reads it, the first time `predicate` holds:
    /// at once if it already does, else re-read after each of the
    /// daemon's job events (subscribed before the first read), the event
    /// a hint and the snapshot the truth. For a state that is not
    /// terminal (Running, a Waiting job with its retry spent); a job's
    /// end is [`Self::await_terminal`]'s. Fails at the park where nothing
    /// the hook accounts for can bring the state (C160), `HANG` the
    /// backstop behind that verdict.
    pub(super) async fn await_job(
        &mut self,
        id: JobId,
        predicate: impl Fn(&JobInfo) -> bool,
    ) -> JobInfo {
        let mut events = self.daemon.events.subscribe();
        let (stuck, counters) = (self.stuck.clone(), self.counters.clone());
        let ceiling = format!("job {id} reached the state awaited");
        or_stuck(
            stuck,
            counters,
            format!("awaiting job {id}'s state"),
            &ceiling,
            async {
                loop {
                    match self.request(Request::Status { id }).await {
                        Response::Status { job } if predicate(&job) => return job,
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
            },
        )
        .await
    }

    /// Job `id`'s end and outcome, as a client reads them: on the
    /// daemon's job events, subscribed before the first read, each event
    /// a hint and the job re-read through the protocol (`status`, then
    /// `result` once terminal). Never a poll or a yield, so the runtime
    /// parks whenever nothing can run. Fails at the park where nothing the
    /// hook accounts for can end the job (C160), `HANG` the backstop
    /// behind that verdict.
    pub(super) async fn await_terminal(&mut self, id: JobId) -> (JobInfo, Outcome) {
        let mut events = self.daemon.events.subscribe();
        let (stuck, counters) = (self.stuck.clone(), self.counters.clone());
        or_stuck(
            stuck,
            counters,
            format!("awaiting job {id}'s end"),
            "the job ended",
            async {
                loop {
                    match self.request(Request::Status { id }).await {
                        Response::Status { job } if job.state.is_terminal() => {
                            match self.request(Request::Result { id }).await {
                                Response::Result { outcome, .. } => return (job, outcome),
                                other => {
                                    panic!("job {id} is terminal and its result reads {other:?}")
                                }
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
            },
        )
        .await
    }

    /// The first `AuthStatus` answer `predicate` holds for: at once if
    /// it already does, else re-read after each auth-change notification
    /// (subscribed before the read), the notification a hint and the
    /// answer the truth. A login's end is `|s| !s.pending`. Fails at the
    /// park where nothing the hook accounts for can bring the answer
    /// (C160), `HANG` the backstop behind that verdict.
    pub(super) async fn await_auth(
        &mut self,
        predicate: impl Fn(&AuthAnswer) -> bool,
    ) -> AuthAnswer {
        let daemon = self.daemon.clone();
        let (stuck, counters) = (self.stuck.clone(), self.counters.clone());
        let awaiting = "awaiting an auth answer its predicate holds for";
        or_stuck(
            stuck,
            counters,
            awaiting.into(),
            "the auth answer came",
            async {
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
            },
        )
        .await
    }

    /// [`DoorBuilder::restore`]'s landing awaited: the keyring word
    /// `AuthStatus` answers is no longer `restoring`.
    async fn await_restored(&mut self) {
        self.await_auth(|answer| answer.keyring != "restoring")
            .await;
    }

    /// The journal's sends so far, header dropped, as the oracles read
    /// them.
    pub(super) fn journal(&self) -> Vec<Value> {
        journal_sends(&self.log_path)
    }

    /// The journal file the daemon wrote, its whole text as it stands,
    /// header included: what a fragment the journal must not keep is
    /// read against.
    pub(super) fn journal_text(&self) -> String {
        std::fs::read_to_string(journal_of(&self.log_path)).expect("the door's journal reads")
    }

    /// Time passes by the scenario's hand: both faces move, every sleeper
    /// due wakes.
    pub(super) fn advance(&self, duration: Duration) {
        self.clock.advance(duration);
    }

    /// The lid closes: the wall clock moves, the monotonic clock does not,
    /// and nothing parked wakes. What a scenario crosses this way is an
    /// expiry on the wall (an access token's), which nothing parks on;
    /// a deadline parked on the monotonic face (a hold) stays ahead.
    pub(super) fn laptop_sleep(&self, duration: Duration) {
        self.clock.laptop_sleep(duration);
    }

    /// The fixture as constructed, for what a scenario reads from it or
    /// drives it with (a hold's release, a recorder).
    pub(super) fn fixture(&mut self) -> &mut Fixture {
        &mut self.fixture
    }

    /// The held request's two barriers, taken once: `arrived` fires when
    /// the request the fixture holds is in, and the request is answered
    /// when `release` fires. The account transport's hold or the delayed
    /// token transport's refresh.
    pub(super) fn held(&mut self) -> (oneshot::Receiver<()>, oneshot::Sender<()>) {
        let (Fixture::Account {
            arrived, release, ..
        }
        | Fixture::Delayed {
            arrived, release, ..
        }) = &mut self.fixture
        else {
            panic!("only the account and the delayed token transports hold a request")
        };
        (
            arrived.take().expect("the arrival, taken once"),
            release.take().expect("the release, taken once"),
        )
    }

    /// What the account transport received so far, in order: each request's
    /// method and the account it named (a grant's, or a bearer's). The
    /// wire, read at the fixture, which the journal is held against.
    pub(super) fn sent(&self) -> Vec<Sent> {
        let Fixture::Account { sent, .. } = &self.fixture else {
            panic!("only the account transport records the accounts sent for")
        };
        sent.lock().unwrap().clone()
    }

    /// What the scripted transport received so far, in order, each request
    /// whole: its method, path, bearer and body ([`Received`]). The wire,
    /// read at the fixture: a token's value is a boundary property only
    /// through a send that carries it (a grant in a `POST /token`'s body,
    /// [`Received::grant`]; a bearer on a `GET`), which the journal never
    /// names.
    pub(super) fn received(&self) -> Vec<Received> {
        let Fixture::Script { received, .. } = &self.fixture else {
            panic!("only the scripted transport records each request whole")
        };
        received.lock().unwrap().clone()
    }

    /// The methods of the journal's sends, in order: what is held against
    /// a fixture's record of the wire.
    pub(super) fn journal_methods(&self) -> Vec<String> {
        self.journal()
            .iter()
            .map(|send| {
                send["method"]
                    .as_str()
                    .expect("a journaled method")
                    .to_string()
            })
            .collect()
    }

    /// A client's submit of `kind` with empty params as `account` (the
    /// sole session's when `None`), as a frontend sends it: the id the
    /// daemon answered.
    pub(super) async fn submit(&mut self, kind: &str, account: Option<&str>) -> JobId {
        let submit = Request::Submit {
            kind: kind.into(),
            params: json!({}),
            priority: 0,
            submitted_by: "test".into(),
            account: account.map(str::to_string),
        };
        match self.request(submit).await {
            Response::Submitted { id } => id,
            other => panic!("a submit of {kind} answered {other:?}"),
        }
    }

    /// [`or_stuck`] on this door's slot and counters.
    async fn or_stuck<T>(
        &self,
        awaiting: String,
        ceiling: &str,
        wait: impl Future<Output = T>,
    ) -> T {
        or_stuck(
            self.stuck.clone(),
            self.counters.clone(),
            awaiting,
            ceiling,
            wait,
        )
        .await
    }

    /// The requests this door's transports hold now, the hook's input
    /// (C160 as amended): for the hook's own pins, never a scenario's
    /// assertion.
    pub(super) fn transport_holds(&self) -> usize {
        self.hold.count()
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

    /// The teardown, run always: the lifecycle's driver, if it has not
    /// left, is aborted (and the dispatcher, as always, by the oracles'
    /// teardown); a scripted transport is awaited until its script has
    /// been asked whole, and the delayed token transport until its refresh
    /// was answered, each then found without a fault (a request it did
    /// not expect); the journal must exist, and the oracles the fixture
    /// kind selects run: the contract oracle with "never over the limit"
    /// against the mock, with "pacing follows responses" otherwise, exact
    /// since the door's clock is manual; a script's journal is held equal
    /// to its wire too. A fixture's end fails at the park where nothing
    /// the hook accounts for can bring it (C160), `HANG` the backstop
    /// behind that verdict.
    pub(super) async fn finish(mut self) {
        self.finished = true;
        if let Some(life) = self.life.take() {
            life.abort();
        }
        let (stuck, counters) = (self.stuck.clone(), self.counters.clone());
        let wire = match &mut self.fixture {
            Fixture::Script { wire, ends, .. } => {
                let awaiting = "awaiting the scripted transport's end, its script asked whole";
                let ceiling = "the script was asked whole";
                or_stuck(stuck, counters, awaiting.into(), ceiling, ends.answered()).await;
                assert_eq!(ends.faults(), Vec::<String>::new(), "the script's faults");
                Some(wire.lock().unwrap().clone())
            }
            Fixture::Delayed { ends, .. } => {
                let awaiting = "awaiting the token transport's end, its refresh answered";
                let ceiling = "the token transport was answered whole";
                or_stuck(stuck, counters, awaiting.into(), ceiling, ends.answered()).await;
                assert_eq!(
                    ends.faults(),
                    Vec::<String>::new(),
                    "the token fixture's faults"
                );
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
        if matches!(self.fixture, Fixture::Mock(_)) {
            finish_harness_mock(self.dispatcher.take(), &self.log_path);
        } else {
            finish_harness(self.dispatcher.take(), &self.log_path);
        }
        self.doors
            .lock()
            .unwrap()
            .retain(|door| door.door != self.id);
    }
}

impl Drop for Door {
    fn drop(&mut self) {
        if !self.finished && !std::thread::panicking() {
            panic!("a door scenario ends through `finish`, so its oracles run");
        }
    }
}

/// C160, a door wait: `wait` raced against the park hook's verdict, the
/// wait polled first (`biased`), so a wait the same park answers wins.
/// The verdict's arm panics naming what was awaited and the counters: the
/// park found nothing parked on the clock, no request a transport holds,
/// no blocking task and no hold, so nothing the hook accounts for can bring the answer. `HANG`
/// stays around both as the backstop a wait outside the accounting can
/// still reach, panicking with `ceiling`.
async fn or_stuck<T>(
    stuck: SettleSlot,
    counters: Arc<Mutex<Counters>>,
    awaiting: String,
    ceiling: &str,
    wait: impl Future<Output = T>,
) -> T {
    let (verdict, stuck_rx) = oneshot::channel();
    let previous = stuck.lock().unwrap().replace(verdict);
    assert!(previous.is_none(), "one door wait at a time");
    let raced = tokio::time::timeout(HANG, async {
        tokio::select! {
            biased;
            done = wait => Some(done),
            Ok(()) = stuck_rx => None,
        }
    })
    .await;
    stuck.lock().unwrap().take();
    match raced {
        Ok(Some(done)) => done,
        Ok(None) => {
            let counters = *counters.lock().unwrap();
            panic!(
                "the daemon can be woken by nothing the hook accounts for: nothing parked on \
                 its clock, no request a transport holds, no blocking task, no hold; {awaiting} \
                 ({counters:?})"
            )
        }
        Err(_) => panic!("{ceiling}"),
    }
}

/// The methods of a record of the account transport's wire ([`Door::sent`]),
/// for the journal's comparison ([`Door::journal_methods`]).
pub(super) fn wire_methods_of(wire: &[Sent]) -> Vec<String> {
    wire.iter().map(|(method, _)| method.clone()).collect()
}

/// The authorize URL's callback, as the browser carries it back: the
/// daemon's redirect target and the flow's state.
pub(super) fn callback_of(authorize_url: &str) -> (String, String) {
    let authorize = url::Url::parse(authorize_url).expect("the authorize URL parses");
    let query: HashMap<String, String> = authorize.query_pairs().into_owned().collect();
    (query["redirect_uri"].clone(), query["state"].clone())
}

/// The daemon's callback URL for the authorize URL's flow, its query the
/// pairs given, in order, each encoded.
fn callback_with(authorize_url: &str, pairs: &[(&str, &str)]) -> String {
    let (redirect_uri, _) = callback_of(authorize_url);
    let mut callback = url::Url::parse(&redirect_uri).expect("the redirect target parses");
    callback.query_pairs_mut().extend_pairs(pairs);
    callback.to_string()
}

/// The provider's redirect on approval: `code` with the flow's state.
pub(super) fn code_callback(authorize_url: &str, code: &str) -> String {
    let (_, state) = callback_of(authorize_url);
    callback_with(authorize_url, &[("code", code), ("state", &state)])
}

/// The provider's redirect on a denial: `error=access_denied` with a
/// free-text description and the flow's state.
pub(super) fn denied_callback(authorize_url: &str) -> String {
    let (_, state) = callback_of(authorize_url);
    callback_with(
        authorize_url,
        &[
            ("error", "access_denied"),
            ("error_description", "The user denied the request"),
            ("state", &state),
        ],
    )
}

/// Requests on the callback path that are not the flow's answer: a code
/// under another state, a code and an error with no state, and the
/// flow's state with neither a code nor an error.
pub(super) fn stray_callbacks(authorize_url: &str) -> [String; 4] {
    let (_, state) = callback_of(authorize_url);
    [
        callback_with(authorize_url, &[("code", "code-x"), ("state", "not-ours")]),
        callback_with(authorize_url, &[("code", "code-x")]),
        callback_with(authorize_url, &[("error", "access_denied")]),
        callback_with(authorize_url, &[("state", &state)]),
    ]
}

/// C160, the verdict: a door wait the daemon will never answer fails at
/// the park where nothing the hook accounts for is pending, never at
/// `HANG`. The halted daemon as `rails.rs` stages it: a tripwire's 429
/// halts it, the first job waiting with its retry spent; a second job
/// submitted after the halt waits in the queue, the pick passing it over
/// while halted, and the scenario awaits that job's end. Nothing is parked
/// on the clock and no held request, blocking task or hold is pending, so the
/// wait's verdict arm panics out of `run` (the scenario is awaited inside
/// `block_on`): its words name the verdict and the job, and the counters
/// show the one stuck park and no advance since the wait began. No
/// duration is asserted (C137): that the wait ended by the verdict and
/// not by `HANG` is the verdict's own words.
#[test]
fn c160_a_wait_the_daemon_will_never_answer_fails_at_the_park() {
    let script = vec![
        ScriptedResponse::full("HEAD", "204 No Content", None, ""),
        ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
    ];
    type Seen = (JobId, Arc<Mutex<Counters>>, Counters);
    let seen: Arc<Mutex<Option<Seen>>> = Arc::default();
    let reads = seen.clone();
    let ended = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Door::open(
            Server::Script(script),
            Arc::new(ParkingClock::new()),
            Arc::new(RecordingCredentialStore::default()),
        )
        .rails(tripwire_config())
        .run(|mut door| async move {
            let first = door.submit("fetch", None).await;
            door.await_job(first, |job| {
                job.retries == 1 && job.state == JobState::Waiting
            })
            .await;
            let parked = door.submit("fetch", None).await;
            let counters = door.counters();
            let before = *counters.lock().unwrap();
            *reads.lock().unwrap() = Some((parked, counters, before));
            door.await_terminal(parked).await;
            panic!("the halted daemon ended job {parked}");
        });
    }));
    let panic = ended.expect_err("the wait on a halted daemon's job failed");
    let words = panic.downcast_ref::<String>().cloned().unwrap_or_default();
    let (parked, counters, before) = seen.lock().unwrap().take().expect("the scenario ran");
    assert!(
        words.contains("the daemon can be woken by nothing the hook accounts for"),
        "{words}"
    );
    assert!(
        words.contains(&format!("awaiting job {parked}'s end")),
        "{words}"
    );
    let after = *counters.lock().unwrap();
    assert_eq!(
        (after.stuck, after.advances),
        (before.stuck + 1, before.advances),
        "one stuck park and no advance since the wait began: {before:?} then {after:?}"
    );
}

/// C160 through a door opened beside another: its waits have the verdict
/// too, since it registers its accounting with the hook the first door's
/// runtime runs under. The halted daemon of the pin above, staged on the
/// door beside (its tripwire's 429 halts it, its second job waits in the
/// queue), the first door facing nothing: the beside door's wait on that
/// job fails at the park where nothing either door accounts for is
/// pending, in the verdict's words naming the job, never at `HANG`. And a
/// door opened beside another on a clock of its own is refused before
/// anything is built, in words that say why.
#[test]
fn c160_a_wait_on_a_door_beside_another_fails_at_the_park() {
    let clock = Arc::new(ParkingClock::new());
    let shared = clock.clone();
    let parked_job: Arc<Mutex<Option<JobId>>> = Arc::default();
    let reads = parked_job.clone();
    let ended = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Door::open(
            Server::None,
            clock,
            Arc::new(RecordingCredentialStore::default()),
        )
        .run(|door| async move {
            let elsewhere = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                drop(
                    Door::open(
                        Server::None,
                        Arc::new(ParkingClock::new()),
                        Arc::new(RecordingCredentialStore::default()),
                    )
                    .beside(&door),
                )
            }));
            let refused = elsewhere.expect_err("a door beside another on its own clock is refused");
            let words = refused.downcast_ref::<&str>().copied().unwrap_or_default();
            assert!(words.contains("runs on the first door's clock"), "{words}");

            let script = vec![
                ScriptedResponse::full("HEAD", "204 No Content", None, ""),
                ScriptedResponse::full("GET", "429 Too Many Requests", Some(0), "{}"),
            ];
            let mut beside = Door::open(
                Server::Script(script),
                shared,
                Arc::new(RecordingCredentialStore::default()),
            )
            .rails(tripwire_config())
            .beside(&door)
            .await;
            let first = beside.submit("fetch", None).await;
            beside
                .await_job(first, |job| {
                    job.retries == 1 && job.state == JobState::Waiting
                })
                .await;
            let parked = beside.submit("fetch", None).await;
            *reads.lock().unwrap() = Some(parked);
            beside.await_terminal(parked).await;
            panic!("the halted daemon beside ended job {parked}");
        });
    }));
    let panic = ended.expect_err("the wait on the halted daemon beside failed");
    let words = panic.downcast_ref::<String>().cloned().unwrap_or_default();
    let parked = parked_job.lock().unwrap().expect("the scenario ran");
    assert!(
        words.contains("the daemon can be woken by nothing the hook accounts for"),
        "{words}"
    );
    assert!(
        words.contains(&format!("awaiting job {parked}'s end")),
        "{words}"
    );
}

/// How a row's GET is answered by [`hold_rows_server`].
enum Shape {
    /// Held before its headers until its release.
    Whole(oneshot::Receiver<()>),
    /// Its headers at once, its body withheld until its release.
    Body(oneshot::Receiver<()>),
}

/// The rows' own transport: every `HEAD` a 204 naming a policy of its
/// path's own, and each path's `GET` answered as its [`Shape`] says, its
/// arrival announced on `arrived` in the send's first poll.
fn hold_rows_server(
    shapes: Vec<(&'static str, Shape)>,
    arrived: tokio::sync::mpsc::UnboundedSender<&'static str>,
) -> Server {
    let shapes = Mutex::new(shapes.into_iter().collect::<HashMap<_, _>>());
    Server::own(move |request: HttpRequest| {
        let rate = |path: &str| {
            format!(
                "X-Rate-Limit-Policy: row{}\r\nX-Rate-Limit-Rules: Account\r\n\
                 X-Rate-Limit-Account: 100:1:60\r\nX-Rate-Limit-Account-State: 0:1:0\r\n",
                path.replace('/', "-")
            )
        };
        let headers = rate(&request.path);
        let taken = (request.method == "GET")
            .then(|| shapes.lock().unwrap().remove_entry(request.path.as_str()))
            .flatten();
        let (shape, path) = match taken {
            Some((path, shape)) => (Some(shape), path),
            None => (None, ""),
        };
        if shape.is_some() {
            arrived.send(path).unwrap();
        }
        let bytes = crate::mockggg::response_text("200 OK", "application/json", &headers, "{}");
        async move {
            match shape {
                None => transport::reply("204 No Content", &headers, ""),
                Some(Shape::Whole(release)) => {
                    let _ = release.await;
                    Reply::Bytes(bytes)
                }
                Some(Shape::Body(release)) => Reply::Held(bytes, release),
            }
        }
    })
}

/// A row's `GET`s, by path, each with its shape, and their releases in
/// the order the row fires them.
type Shapes = (Vec<(&'static str, Shape)>, Vec<oneshot::Sender<()>>);

/// One row of [`c160_the_transports_hold_holds_the_clock_through_the_door`].
struct HoldRow {
    name: &'static str,
    /// The `GET`s sent, made fresh for the row's door.
    shapes: fn() -> Shapes,
    /// The requests held once every held `GET` has arrived.
    held: usize,
}

/// C160 as amended, the hold's boundary rows through the door: a request a
/// test transport holds holds the scenario's clock, from the send's first
/// poll to its body's end. Each row probes its routes (`HEAD`), sends its
/// `GET`s on the daemon's choke point, and, once every held one has
/// arrived, parks a 30 s sleep on the clock: at the next park (`quiet`)
/// the clock has not moved, the door's transports hold the row's count,
/// and a `settle` begun before stays pending; each release lets one go,
/// and only when the last body ends does `settle` resolve, the clock
/// still where it was, then the sleep's deadline is reached. Two requests
/// held at once (headers withheld) hold the clock until both end; a
/// release fired before the request's first poll leaks no hold and
/// nothing waits; a body withheld after its headers holds the clock and
/// keeps `settle` pending until its release. A dropped send or body is
/// the transports' component case
/// (`the_hold_spans_a_withheld_body_and_goes_on_a_drop`): no frontend
/// request drops one.
#[test]
fn c160_the_transports_hold_holds_the_clock_through_the_door() {
    let rows = [
        HoldRow {
            name: "two requests held at once",
            shapes: || {
                let (a, a_rx) = oneshot::channel();
                let (b, b_rx) = oneshot::channel();
                (
                    vec![("/a", Shape::Whole(a_rx)), ("/b", Shape::Whole(b_rx))],
                    vec![a, b],
                )
            },
            held: 2,
        },
        HoldRow {
            name: "a release fired before the request's first poll",
            shapes: || {
                let (early, early_rx) = oneshot::channel();
                early.send(()).unwrap();
                (vec![("/a", Shape::Whole(early_rx))], Vec::new())
            },
            held: 0,
        },
        HoldRow {
            name: "a body withheld after its headers",
            shapes: || {
                let (body, body_rx) = oneshot::channel();
                (vec![("/a", Shape::Body(body_rx))], vec![body])
            },
            held: 1,
        },
    ];
    for row in rows {
        let name = row.name;
        let (shapes, releases) = (row.shapes)();
        let paths: Vec<&'static str> = shapes.iter().map(|(path, _)| *path).collect();
        let (arrived_tx, mut arrived) = tokio::sync::mpsc::unbounded_channel();
        Door::open(
            hold_rows_server(shapes, arrived_tx),
            Arc::new(ParkingClock::new()),
            Arc::new(RecordingCredentialStore::default()),
        )
        .run(|door| async move {
            let (daemon, clock) = (door.daemon().clone(), door.clock());
            for path in &paths {
                let (route, url) = (
                    format!("row{}", path.replace('/', "-")),
                    format!("{}{path}", transport::BASE),
                );
                daemon
                    .choke
                    .head(&route, &url, None, daemon.choke.now(), ALWAYS_WANTED)
                    .await
                    .expect("the row's probe");
            }
            let sends: Vec<_> = paths
                .iter()
                .map(|path| {
                    let daemon = daemon.clone();
                    let (route, url) = (
                        format!("row{}", path.replace('/', "-")),
                        format!("{}{path}", transport::BASE),
                    );
                    tokio::spawn(async move {
                        daemon
                            .choke
                            .get(&route, &url, daemon.choke.now(), ALWAYS_WANTED)
                            .await
                            .map(|response| response.status.as_u16())
                    })
                })
                .collect();
            for _ in 0..paths.len() {
                tokio::time::timeout(HANG, arrived.recv())
                    .await
                    .expect("the row's GET arrived")
                    .expect("the row's transport announces each GET");
            }
            let origin = clock.now();
            let sleeper = tokio::spawn({
                let clock = clock.clone();
                async move { clock.sleep(Duration::from_secs(30)).await }
            });
            {
                let settle = door.settle();
                tokio::pin!(settle);
                let mut releases = releases.into_iter();
                loop {
                    tokio::select! {
                        biased;
                        () = &mut settle => panic!("{name}: settled under a held request"),
                        () = door.quiet() => {}
                    }
                    let held = door.transport_holds();
                    assert_eq!(
                        clock.now(),
                        origin,
                        "{name}: the clock moved under {held} held"
                    );
                    assert_eq!(clock.sleepers(), 1, "{name}: the sleep is parked");
                    let Some(release) = releases.next() else {
                        assert_eq!(
                            held, 0,
                            "{name}: no hold leaks once every request is answered"
                        );
                        break;
                    };
                    assert!(
                        (1..=row.held).contains(&held),
                        "{name}: {held} held, the row holds {}",
                        row.held
                    );
                    release.send(()).unwrap();
                }
                settle.await;
            }
            assert_eq!(clock.now(), origin, "{name}: settled, not advanced");
            assert_eq!(door.transport_holds(), 0, "{name}: every hold released");
            tokio::time::timeout(HANG, sleeper)
                .await
                .expect("the sleep's deadline was reached")
                .unwrap();
            assert_eq!(clock.now() - origin, Duration::from_secs(30), "{name}");
            for send in sends {
                assert_eq!(send.await.unwrap().ok(), Some(200), "{name}");
            }
            door.finish().await;
        });
    }
}
