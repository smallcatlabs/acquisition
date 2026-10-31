//! C157, a lifecycle that waits is a value: the daemon's lifecycle as a
//! machine. Its state is data, its [`Life::step`] a pure function of that
//! state and one [`Event`], returning the [`Effect`]s the driver performs
//! next; time enters only as a named timer's [`Event::Deadline`] and
//! leaves only as a request to arm or cancel one. The driver is
//! `server.rs`'s lifecycle loop (`Daemon::life`): it holds the daemon,
//! the exit (`Exit`), the stop connection's and the signal watcher's
//! events, the in-flight snapshot, the log, every instant and every
//! duration, performs each effect with the functions it always used
//! (`is_idle`, `begin_exit`, `InFlight::of`, `leave`), and turns what it
//! saw into the next event.
//!
//! The criterion that draws the machine's edge (`4-0-design.md` §4, the
//! owner's ruling of 2026-10-31, applied per wait in step 2's brief): a
//! wait enters the machine when its ordering against another event is a
//! decision of the lifecycle, or when its only pin asserts elapsed time;
//! a wait whose interleavings are already forced without relying on
//! elapsed time, and whose outcome the lifecycle takes as one value,
//! stays a driver effect with its bound inside. So the idle deadline is a
//! timer (its expiry is the lifecycle's decision, the exit, and it had no
//! pin at all), and the drain's deadline is a timer (it orders against
//! the quiet: cut or clean; its only pin asserted elapsed time). The
//! first cause's race to arm the exit is decided where it always was, at
//! each ingress by one atomic swap (`Exit::arm`), and the machine learns
//! the winner as data (`won`). The verdict itself (`is_idle`, pure,
//! pinned by `idle.rs` on the manual clock), the quiet wait (`InFlight`
//! empty, its pause inside), the exit bound's OS thread (C133, real by
//! design) and the accept backoff are driver effects. The second signal
//! is the watcher's own forced end, a backstop that must stay available
//! while the loop checkpoints (C133), so it is no transition.
//!
//! An event a state cannot use is dropped by a transition that does
//! nothing, never a panic: the driver's sources race (a late verdict, a
//! second cause, a deadline beside a quiet).
//!
//! Pinned by the table of event scripts in `tests/exit.rs`, one row per
//! transition, in a synchronous test; the driver by the real-wiring
//! tests beside it (the `c137_` and `c133_` tests) and the cross-process
//! ones of `acquisition-cli`.

/// A timer the machine names; its length and its end are the driver's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Timer {
    /// When the idle verdict is next judged (C3).
    Idle,
    /// The exit's bound on the drain (C133).
    Drain,
}

/// What began the exit: the first cause, whose swap won.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Cause {
    /// The idle verdict (C3).
    Idle,
    /// `daemon stop`.
    Stop,
    /// A terminating signal (Unix).
    Signal,
}

/// What the driver saw, one observation per event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Event {
    /// The timer has passed.
    Deadline(Timer),
    /// The verdict, asked by [`Effect::Judge`]: idle.
    Idle,
    /// The verdict: not idle.
    Busy,
    /// The idle cause's own swap, asked by [`Effect::Begin`]; `won` when
    /// it began the exit.
    ExitArmed { won: bool },
    /// `daemon stop` was read and its connection swapped the exit;
    /// `won` when it began it.
    StopRequested { won: bool },
    /// A first signal; the watcher swapped the exit on its own thread,
    /// `won` when it began it; `why` the words it built.
    #[cfg_attr(not(unix), allow(dead_code))] // the watcher is Unix's alone
    Signal { won: bool, why: String },
    /// Nothing is in flight (`InFlight` empty).
    Quiet,
}

/// What the driver does next.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Effect {
    /// Arm the timer.
    Arm(Timer),
    /// Forget the timer.
    Cancel(Timer),
    /// Judge the idle verdict, answered [`Event::Idle`] or
    /// [`Event::Busy`].
    Judge,
    /// Begin the exit for the idle cause, its swap and, when won, its
    /// admission stop: answered [`Event::ExitArmed`].
    Begin,
    /// Stop admitting, for a cause that swapped and did no more (the
    /// signal's winner): the rails stop, the words logged, the
    /// dispatcher woken.
    StopAdmitting(String),
    /// Wait for nothing to be in flight, answered [`Event::Quiet`].
    AwaitQuiet,
    /// Log what the bound cut, from the last look the driver took.
    Cut,
    /// Leave: the checkpoints, the socket, the process's end.
    Leave,
}

/// Where the lifecycle stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum State {
    /// The daemon serves; the idle verdict is judged at its deadline.
    Serving,
    /// The exit has begun: the drain against its bound.
    Draining,
    /// The leave has been asked for: nothing more.
    Done,
}

/// The daemon's lifecycle (C157), one per daemon.
#[derive(Debug)]
pub(super) struct Life {
    state: State,
    /// The first cause, set from the event that reports its swap won.
    cause: Option<Cause>,
}

impl Life {
    /// A daemon's lifecycle, serving. Nothing is read: the knob and the
    /// bound are the driver's durations.
    pub(super) fn new() -> Life {
        Life {
            state: State::Serving,
            cause: None,
        }
    }

    /// Where the lifecycle stands.
    #[cfg_attr(not(test), allow(dead_code))] // read by the table
    pub(super) fn state(&self) -> State {
        self.state
    }

    /// The first cause, once the exit has begun.
    #[cfg_attr(not(test), allow(dead_code))] // read by the table
    pub(super) fn cause(&self) -> Option<Cause> {
        self.cause
    }

    /// The first effects: the idle deadline armed.
    pub(super) fn start(&mut self) -> Vec<Effect> {
        vec![Effect::Arm(Timer::Idle)]
    }

    /// One event in, the effects it asks for out. An event the state
    /// cannot use asks for nothing.
    pub(super) fn step(&mut self, event: Event) -> Vec<Effect> {
        match (self.state, event) {
            (State::Serving, Event::Deadline(Timer::Idle)) => vec![Effect::Judge],
            (State::Serving, Event::Busy) => vec![Effect::Arm(Timer::Idle)],
            (State::Serving, Event::Idle) => vec![Effect::Begin],
            (State::Serving, Event::ExitArmed { won: true }) => self.begun(Cause::Idle, vec![]),
            (State::Serving, Event::StopRequested { won: true }) => self.begun(Cause::Stop, vec![]),
            // The watcher swapped and did no more: the admission stop is
            // the loop's first act.
            (State::Serving, Event::Signal { won: true, why }) => {
                self.begun(Cause::Signal, vec![Effect::StopAdmitting(why)])
            }
            (State::Draining, Event::Quiet) => {
                self.state = State::Done;
                vec![Effect::Cancel(Timer::Drain), Effect::Leave]
            }
            // The drain's return was always discarded: cut or clean differ
            // only in the log.
            (State::Draining, Event::Deadline(Timer::Drain)) => {
                self.state = State::Done;
                vec![Effect::Cut, Effect::Leave]
            }
            // A lost swap waits for its winner's own event; in the drain a
            // second cause, a late verdict or a late deadline is nothing;
            // after the leave everything is.
            _ => vec![],
        }
    }

    /// The first cause's swap won: the idle deadline forgotten, the drain
    /// against its bound.
    fn begun(&mut self, cause: Cause, mut effects: Vec<Effect>) -> Vec<Effect> {
        self.cause = Some(cause);
        self.state = State::Draining;
        effects.extend([
            Effect::Cancel(Timer::Idle),
            Effect::Arm(Timer::Drain),
            Effect::AwaitQuiet,
        ]);
        effects
    }
}
