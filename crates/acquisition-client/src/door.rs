//! C157, a lifecycle that waits is a value: the client's door as a
//! machine. Its state is data, its [`Machine::step`] a pure function of
//! that state and one [`Event`], returning the [`Effect`]s the driver
//! performs next; time enters only as a named timer's event and leaves
//! only as a request to arm or consult one. The driver is `client.rs`'s
//! door loop (`Client::open_door`, `Client::observe_at`,
//! `Client::stop_any_at`): it holds the socket, the child process, the
//! lock file, the clock and every timer's length and end, performs each
//! effect with the functions it always used, and turns what it saw into
//! the next event. What the machine sees of a peer is the driver's
//! judgement of it (C10, the single-version handshake; C84, the two
//! identity values), delivered once per handshake as [`Event::Identified`].
//!
//! The criterion that draws the machine's edge (`4-0-design.md` §4, the
//! owner's ruling of 2026-10-31): a wait enters the machine when its
//! ordering against another event is a decision of the door, or when its
//! only pin asserts elapsed time; a wait whose interleavings are already
//! forced without relying on elapsed time, and whose outcome the door
//! takes as one value, stays a driver effect with its bound inside. So
//! the hello's deadline is a timer (it orders against the reply, and the
//! loser is the verdict: C86, the handshake deadline), the startup
//! deadline is a timer the machine consults after a spawn and after each
//! poll of the child (it orders against the child's exit), and the bind
//! poll and the settle after a replace are pauses the machine orders.
//! The stop's wait for the daemon to go ([`Effect::Stop`]) and the look
//! for a spawn race's winner ([`Effect::LookForWinner`]) are effects:
//! their own pins force their tie and their lateness, and the door takes
//! each as one value.
//!
//! The policy is read once, at the door's start: the caller's options,
//! `ACQ_NO_SPAWN` and whether a human is present (C88, a human starts a
//! real daemon). A change to `ACQ_NO_SPAWN` during a door is not seen.
//!
//! Pinned by the table of event scripts below, one row per transition, in
//! a synchronous test; the driver by the real-wiring tests of
//! `tests/contract.rs` and `client.rs`'s own.

use crate::client::{NotReplaced, NotSpawned, Stopped};

/// Which door this is (C10's policy tiers).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Door {
    /// A use verb, with the caller's options (`ConnectOptions`).
    Use { spawn: bool, replace: bool },
    /// An observation: never spawns, never replaces.
    Observe,
    /// `daemon stop`: stops whatever answers.
    Stop,
}

/// A timer the machine names; its length and its end are the driver's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Timer {
    /// The handshake's deadline (C86), raced against the reply.
    Hello,
    /// A spawned daemon's deadline to bind (C137), consulted, never raced.
    Startup,
    /// The pause between two looks at a socket a child has yet to bind.
    BindPoll,
    /// The pause after a stop sent to replace a daemon.
    Settle,
}

/// What the driver saw, one observation per event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Event {
    /// Something listens on the world's socket.
    Reached,
    /// Nothing does: no socket, or none listening.
    Nothing,
    /// The connect failed otherwise, as words.
    ConnectFailed(String),
    /// The handshake identified the peer, judged by the driver.
    Identified {
        ours: bool,
        same_world: bool,
        pid: u32,
    },
    /// The hello's deadline passed before a reply in time (C86).
    Unresponsive,
    /// The handshake failed otherwise, as words.
    HelloFailed(String),
    /// The child this door started, by pid.
    Spawned { pid: u32 },
    /// The spawn failed, as words.
    SpawnFailed(String),
    /// The child has not exited.
    ChildRunning,
    /// The child has exited; `holder` is the pid the world lock records,
    /// read at that moment.
    ChildExited { holder: Option<u32> },
    /// A consulted timer has passed (its end at or before now).
    Deadline(Timer),
    /// A consulted timer has not.
    Pending(Timer),
    /// The look found a winner listening; the driver holds its connection.
    Found,
    /// The look found nothing in its bound.
    NotFound,
    /// The look failed, as words.
    LookFailed(String),
    /// The stop was acknowledged, and what it came to.
    Stopped(Stopped),
    /// The stop was refused, or its conversation failed, as words.
    StopFailed(String),
}

/// What the driver does next.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Effect {
    /// Connect to the world's socket.
    Connect,
    /// Send hello over the connection in hand, under the armed `Hello`.
    SendHello,
    /// Start the `acqd` beside this executable.
    Spawn,
    /// Look whether the child has exited.
    PollChild,
    /// Say whether the timer has passed.
    Consult(Timer),
    /// Let the child go (reaped now or when it exits), and its deadline.
    DropChild,
    /// Look for the winner of a spawn race, the whole bounded look.
    LookForWinner,
    /// Ask the peer in hand to stop, and wait for it to go: the whole
    /// conversation, its bound inside.
    Stop,
    /// Arm the timer; a pause is delivered as its deadline.
    Arm(Timer),
    /// Forget the timer.
    Cancel(Timer),
    /// The door's answer.
    Report(Outcome),
}

/// How a spawn this door made came to nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Ended {
    /// The child exited during startup.
    Exited,
    /// It exited on the world lock `holder` records, and the look found
    /// no winner.
    Lost { holder: u32 },
    /// It did not bind before its deadline.
    NotBound,
}

/// The door's answer as data; the driver builds the typed value from it
/// and from what it holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    /// Use: the daemon is this client's; `started_here` when it is the
    /// child this door spawned.
    Opened { started_here: bool },
    /// Observe: the daemon is this client's.
    Compatible,
    /// Observe: a daemon that is not this client's.
    Incompatible,
    /// Observe, stop: nothing listens.
    Absent,
    /// Every door: the peer did not answer hello in time (C86).
    Unresponsive,
    /// Every door: the conversation failed, as words.
    Transport(String),
    /// Use: nothing listens, and this door may not spawn.
    NotSpawned(NotSpawned),
    /// Use: a daemon this door may not or did not replace.
    NotReplaced(NotReplaced),
    /// Use: a daemon on another world (C83), never replaced.
    OtherWorld,
    /// Use: the spawn failed, as words.
    SpawnFailed(String),
    /// Use: the spawn came to nothing.
    SpawnEnded(Ended),
    /// Stop: the stop was acknowledged.
    Stopped(Stopped),
    /// Stop: the stop failed, as words.
    StopFailed(String),
}

/// Why the door asked the peer in hand to stop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stopping {
    /// To replace a mismatch.
    ToReplace,
    /// The daemon this door spawned, judged another build after a
    /// respawn (#116).
    AfterRespawn,
    /// `daemon stop` asked.
    Asked,
}

/// Where a spawn this door made stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Starting {
    /// The spawn is under way.
    Spawning,
    /// The child is being looked at.
    Polling,
    /// The startup deadline is being consulted.
    Consulting,
    /// The bind poll's pause.
    Pausing,
    /// The look for the winner, after an exit on a lock `holder` holds.
    Looking { holder: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    /// A connect is under way.
    Reaching,
    /// A hello is under way, or the stop its verdict asked for.
    AwaitingHello { stopping: Option<Stopping> },
    /// A spawn this door made, on its way to a bound socket.
    Starting(Starting),
    /// The pause after a stop sent to replace.
    Settling,
    /// The door has answered.
    Done,
}

/// The door's lifecycle (C157), one per door.
#[derive(Debug)]
pub(crate) struct Machine {
    door: Door,
    no_spawn: bool,
    /// The effective policy: the caller's, under `ACQ_NO_SPAWN` and C88.
    spawn: bool,
    replace: bool,
    state: State,
    /// The pid of the child this door spawned.
    child: Option<u32>,
    /// A mismatch was stopped to be replaced.
    respawned: bool,
    /// The error of a stop sent to replace, reported if the next look
    /// finds that daemon still there (C147).
    stop_failed: Option<String>,
    /// A child that exited on the world lock has had its one look for
    /// the winner (C147).
    looked_for_winner: bool,
    /// The connection in hand is a winner the look found: handshaken
    /// directly, never judged against the startup deadline.
    found: bool,
}

impl Machine {
    /// A door's machine, from what is read once at its start: the door,
    /// `ACQ_NO_SPAWN`, whether a human is present (C88), and `child`, the
    /// pid of a child this door already spawned (the driver's seam: it
    /// holds that child and its deadline, and nothing is armed afresh).
    pub(crate) fn new(door: Door, no_spawn: bool, human: bool, child: Option<u32>) -> Machine {
        let (spawn, replace) = match door {
            Door::Use { spawn, replace } => {
                (spawn && !no_spawn && human, replace && !no_spawn && human)
            }
            Door::Observe | Door::Stop => (false, false),
        };
        Machine {
            door,
            no_spawn,
            spawn,
            replace,
            state: State::Reaching,
            child,
            respawned: false,
            stop_failed: None,
            looked_for_winner: false,
            found: false,
        }
    }

    /// Whether this door may spawn: the world is created first when it
    /// may (C83), observed otherwise.
    pub(crate) fn may_spawn(&self) -> bool {
        self.spawn
    }

    /// The first effects: every door begins at the socket.
    pub(crate) fn start(&mut self) -> Vec<Effect> {
        self.state = State::Reaching;
        vec![Effect::Connect]
    }

    /// One event in, the effects it asks for out. An event the state
    /// cannot receive is a broken driver (C47: a panic).
    pub(crate) fn step(&mut self, event: Event) -> Vec<Effect> {
        match (self.state, event) {
            (State::Reaching, Event::Reached) => self.hello(),
            (State::Reaching, Event::ConnectFailed(words)) => {
                self.report(Outcome::Transport(words))
            }
            (State::Reaching, Event::Nothing) => self.nothing(),
            (State::AwaitingHello { stopping: None }, Event::Unresponsive) => {
                self.report(Outcome::Unresponsive)
            }
            (State::AwaitingHello { stopping: None }, Event::HelloFailed(words)) => {
                self.report(Outcome::Transport(words))
            }
            (
                State::AwaitingHello { stopping: None },
                Event::Identified {
                    ours,
                    same_world,
                    pid,
                },
            ) => self.identified(ours, same_world, pid),
            (
                State::AwaitingHello {
                    stopping: Some(why),
                },
                Event::Stopped(stopped),
            ) => self.stopped(why, Ok(stopped)),
            (
                State::AwaitingHello {
                    stopping: Some(why),
                },
                Event::StopFailed(words),
            ) => self.stopped(why, Err(words)),
            (State::Starting(Starting::Spawning), Event::Spawned { pid }) => {
                self.child = Some(pid);
                self.state = State::Starting(Starting::Consulting);
                vec![Effect::Arm(Timer::Startup), Effect::Consult(Timer::Startup)]
            }
            (State::Starting(Starting::Spawning), Event::SpawnFailed(words)) => {
                self.report(Outcome::SpawnFailed(words))
            }
            (State::Starting(Starting::Polling), Event::ChildRunning) => {
                self.state = State::Starting(Starting::Consulting);
                vec![Effect::Consult(Timer::Startup)]
            }
            (State::Starting(Starting::Polling), Event::ChildExited { holder }) => {
                // An exited child will never bind: its refusal is the
                // answer now, unless it exited on a lock another daemon
                // holds (C147), whose winner gets one look first.
                match holder.filter(|pid| Some(*pid) != self.child) {
                    Some(holder) if !self.looked_for_winner => {
                        self.looked_for_winner = true;
                        self.state = State::Starting(Starting::Looking { holder });
                        vec![Effect::Cancel(Timer::Startup), Effect::LookForWinner]
                    }
                    _ => self.report(Outcome::SpawnEnded(Ended::Exited)),
                }
            }
            (State::Starting(Starting::Looking { .. }), Event::Found) => {
                self.child = None;
                self.found = true;
                let mut effects = vec![Effect::DropChild];
                effects.extend(self.hello());
                effects
            }
            (State::Starting(Starting::Looking { holder }), Event::NotFound) => {
                self.report(Outcome::SpawnEnded(Ended::Lost { holder }))
            }
            (State::Starting(Starting::Looking { .. }), Event::LookFailed(words)) => {
                self.report(Outcome::Transport(words))
            }
            (State::Starting(Starting::Consulting), Event::Deadline(Timer::Startup)) => {
                self.report(Outcome::SpawnEnded(Ended::NotBound))
            }
            (State::Starting(Starting::Consulting), Event::Pending(Timer::Startup)) => {
                self.state = State::Starting(Starting::Pausing);
                vec![Effect::Arm(Timer::BindPoll)]
            }
            (State::Starting(Starting::Pausing), Event::Deadline(Timer::BindPoll))
            | (State::Settling, Event::Deadline(Timer::Settle)) => self.start(),
            (state, event) => panic!("the door's machine received {event:?} in {state:?}"),
        }
    }

    fn report(&mut self, outcome: Outcome) -> Vec<Effect> {
        self.state = State::Done;
        vec![Effect::Report(outcome)]
    }

    fn hello(&mut self) -> Vec<Effect> {
        self.state = State::AwaitingHello { stopping: None };
        vec![Effect::Arm(Timer::Hello), Effect::SendHello]
    }

    fn stop(&mut self, why: Stopping) -> Vec<Effect> {
        self.state = State::AwaitingHello {
            stopping: Some(why),
        };
        vec![Effect::Stop]
    }

    /// Nothing listens.
    fn nothing(&mut self) -> Vec<Effect> {
        match self.door {
            Door::Observe | Door::Stop => self.report(Outcome::Absent),
            // Once a mismatch was stopped the door respawns even for a
            // `spawn: false` caller: leaving nothing would turn a read
            // into a stop.
            Door::Use { .. } if self.spawn || self.respawned => {
                // The stopped daemon is gone: its stop's error is not the
                // answer.
                self.stop_failed = None;
                if self.child.is_some() {
                    self.state = State::Starting(Starting::Polling);
                    vec![Effect::PollChild]
                } else {
                    self.state = State::Starting(Starting::Spawning);
                    vec![Effect::Spawn]
                }
            }
            // The caller's policy first: a door that never spawns is
            // absent by policy whatever the knob says.
            Door::Use { spawn, .. } => self.report(Outcome::NotSpawned(if !spawn {
                NotSpawned::Policy
            } else if self.no_spawn {
                NotSpawned::NoSpawnEnv
            } else {
                NotSpawned::NoTerminal
            })),
        }
    }

    /// The handshake's verdict, at each door's policy.
    fn identified(&mut self, ours: bool, same_world: bool, pid: u32) -> Vec<Effect> {
        let started_here = !self.found && self.child == Some(pid);
        self.found = false;
        let replace_asked = match self.door {
            Door::Observe if ours => return self.report(Outcome::Compatible),
            Door::Observe => return self.report(Outcome::Incompatible),
            Door::Stop => return self.stop(Stopping::Asked),
            Door::Use { replace, .. } => replace,
        };
        if ours {
            return self.report(Outcome::Opened { started_here });
        }
        // Another world is never replaced (C83): only the socket collided.
        if !same_world {
            return self.report(Outcome::OtherWorld);
        }
        if self.respawned {
            if let Some(words) = self.stop_failed.take() {
                return self.report(Outcome::NotReplaced(NotReplaced::StopFailed(words)));
            }
            // #116: the daemon this door spawned, still another build, is
            // stopped before the refusal; another process's is left.
            if started_here {
                return self.stop(Stopping::AfterRespawn);
            }
            return self.report(Outcome::NotReplaced(NotReplaced::StillAfterRespawn {
                stopped: None,
            }));
        }
        if !self.replace {
            return self.report(Outcome::NotReplaced(if !replace_asked {
                NotReplaced::NeverReplaces
            } else if self.no_spawn {
                NotReplaced::NoSpawn
            } else {
                NotReplaced::NoTerminal
            }));
        }
        self.stop(Stopping::ToReplace)
    }

    /// What the stop the door asked for came to.
    fn stopped(&mut self, why: Stopping, stopped: Result<Stopped, String>) -> Vec<Effect> {
        match why {
            Stopping::Asked => self.report(match stopped {
                Ok(stopped) => Outcome::Stopped(stopped),
                Err(words) => Outcome::StopFailed(words),
            }),
            Stopping::AfterRespawn => {
                self.report(Outcome::NotReplaced(NotReplaced::StillAfterRespawn {
                    stopped: Some(stopped),
                }))
            }
            // A failed stop is kept (C147): the next look decides. An
            // acknowledged one is no failure here (#116's review).
            Stopping::ToReplace => {
                self.stop_failed = stopped.err();
                self.respawned = true;
                self.child = None;
                self.state = State::Settling;
                vec![Effect::DropChild, Effect::Arm(Timer::Settle)]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The runner: a machine, a script of events, every effect asked for.
    fn drive(mut machine: Machine, script: &[Event]) -> Vec<Effect> {
        let mut effects = machine.start();
        for event in script {
            effects.extend(machine.step(event.clone()));
        }
        effects
    }

    /// One transition's row: the construction inputs, the script and the
    /// effects the rulings expect, written from them, never from a run.
    struct Row {
        name: &'static str,
        door: Door,
        no_spawn: bool,
        human: bool,
        child: Option<u32>,
        script: Vec<Event>,
        expect: Vec<Effect>,
    }

    const INTERACTIVE: Door = Door::Use {
        spawn: true,
        replace: true,
    };
    const AUTONOMOUS: Door = Door::Use {
        spawn: true,
        replace: false,
    };

    /// A use door at the interactive policy, a human present.
    fn row(name: &'static str, script: Vec<Event>, expect: Vec<Effect>) -> Row {
        Row {
            name,
            door: INTERACTIVE,
            no_spawn: false,
            human: true,
            child: None,
            script,
            expect,
        }
    }

    fn on(door: Door, row: Row) -> Row {
        Row { door, ..row }
    }

    use Effect::{
        Arm, Cancel, Connect, Consult, DropChild, LookForWinner, PollChild, Report, SendHello,
        Spawn, Stop,
    };
    use Event::{
        ChildExited, ChildRunning, Deadline, Found, Identified, LookFailed, NotFound, Nothing,
        Pending, Reached, Spawned, StopFailed, Unresponsive,
    };

    const OURS: Event = Identified {
        ours: true,
        same_world: true,
        pid: 42,
    };
    const STALE: Event = Identified {
        ours: false,
        same_world: true,
        pid: 42,
    };
    const ELSEWHERE: Event = Identified {
        ours: false,
        same_world: false,
        pid: 42,
    };
    /// The child this door spawned, and a winner another process started.
    const CHILD: u32 = 77;
    const WINNER: u32 = 99;

    fn ours(pid: u32) -> Event {
        Identified {
            ours: true,
            same_world: true,
            pid,
        }
    }

    fn stale(pid: u32) -> Event {
        Identified {
            ours: false,
            same_world: true,
            pid,
        }
    }

    fn still(stopped: Option<Result<Stopped, String>>) -> Effect {
        Report(Outcome::NotReplaced(NotReplaced::StillAfterRespawn {
            stopped,
        }))
    }

    /// The prefix of a replace: hello, a mismatch, its stop, the settle.
    fn replaced(stopped: Event) -> (Vec<Event>, Vec<Effect>) {
        (
            vec![Reached, STALE, stopped, Deadline(Timer::Settle)],
            vec![
                Connect,
                Arm(Timer::Hello),
                SendHello,
                Stop,
                DropChild,
                Arm(Timer::Settle),
                Connect,
            ],
        )
    }

    /// `replaced`, then a respawn whose child binds and answers `answer`.
    fn respawned(answer: Event) -> (Vec<Event>, Vec<Effect>) {
        let (mut script, mut expect) = replaced(Event::Stopped(Stopped::Gone));
        script.extend([
            Nothing,
            Spawned { pid: CHILD },
            Pending(Timer::Startup),
            Deadline(Timer::BindPoll),
            Reached,
            answer,
        ]);
        expect.extend([
            Spawn,
            Arm(Timer::Startup),
            Consult(Timer::Startup),
            Arm(Timer::BindPoll),
            Connect,
            Arm(Timer::Hello),
            SendHello,
        ]);
        (script, expect)
    }

    fn with(
        mut prefix: (Vec<Event>, Vec<Effect>),
        script: Vec<Event>,
        expect: Vec<Effect>,
    ) -> (Vec<Event>, Vec<Effect>) {
        prefix.0.extend(script);
        prefix.1.extend(expect);
        prefix
    }

    fn table() -> Vec<Row> {
        let hello = |then: Vec<Effect>| {
            let mut effects = vec![Connect, Arm(Timer::Hello), SendHello];
            effects.extend(then);
            effects
        };
        // From the seam: a child already spawned, a turn that found
        // nothing, its exit on a lock `WINNER` holds, the look.
        let lost = |then: Vec<Event>, effects: Vec<Effect>| {
            let mut script = vec![
                Nothing,
                ChildExited {
                    holder: Some(WINNER),
                },
            ];
            script.extend(then);
            let mut expect = vec![Connect, PollChild, Cancel(Timer::Startup), LookForWinner];
            expect.extend(effects);
            Row {
                child: Some(CHILD),
                ..row("", script, expect)
            }
        };
        let named = |name: &'static str, r: Row| Row { name, ..r };
        let (replace_script, replace_expect) = replaced(Event::Stopped(Stopped::Gone));
        let (failed_script, failed_expect) = replaced(StopFailed("refused".into()));
        let mut rows = vec![
            // Hello answered, ours, on each door.
            row(
                "use: a daemon of this client's opens, not started here",
                vec![Reached, OURS],
                hello(vec![Report(Outcome::Opened {
                    started_here: false,
                })]),
            ),
            on(
                Door::Observe,
                row(
                    "observe: a daemon of this client's is compatible",
                    vec![Reached, OURS],
                    hello(vec![Report(Outcome::Compatible)]),
                ),
            ),
            on(
                Door::Stop,
                row(
                    "stop: a daemon of this client's is asked to stop",
                    vec![Reached, OURS, Event::Stopped(Stopped::Gone)],
                    hello(vec![Stop, Report(Outcome::Stopped(Stopped::Gone))]),
                ),
            ),
            // The expiry rows (C86): unresponsive on each door, no spawn,
            // no stop.
            row(
                "use: the hello deadline is unresponsive, no spawn, no stop",
                vec![Reached, Unresponsive],
                hello(vec![Report(Outcome::Unresponsive)]),
            ),
            on(
                Door::Observe,
                row(
                    "observe: the hello deadline is unresponsive",
                    vec![Reached, Unresponsive],
                    hello(vec![Report(Outcome::Unresponsive)]),
                ),
            ),
            on(
                Door::Stop,
                row(
                    "stop: the hello deadline is unresponsive, no stop",
                    vec![Reached, Unresponsive],
                    hello(vec![Report(Outcome::Unresponsive)]),
                ),
            ),
            row(
                "use: a hello that failed is a transport failure",
                vec![Reached, Event::HelloFailed("closed".into())],
                hello(vec![Report(Outcome::Transport("closed".into()))]),
            ),
            row(
                "use: a connect that failed is a transport failure",
                vec![Event::ConnectFailed("denied".into())],
                vec![Connect, Report(Outcome::Transport("denied".into()))],
            ),
            // Another world (C83).
            row(
                "use: another world is refused and never replaced",
                vec![Reached, ELSEWHERE],
                hello(vec![Report(Outcome::OtherWorld)]),
            ),
            on(
                Door::Observe,
                row(
                    "observe: another build is incompatible",
                    vec![Reached, STALE],
                    hello(vec![Report(Outcome::Incompatible)]),
                ),
            ),
            on(
                Door::Observe,
                row(
                    "observe: another world is incompatible",
                    vec![Reached, ELSEWHERE],
                    hello(vec![Report(Outcome::Incompatible)]),
                ),
            ),
            on(
                Door::Stop,
                row(
                    "stop: another world is asked to stop",
                    vec![Reached, ELSEWHERE, Event::Stopped(Stopped::Gone)],
                    hello(vec![Stop, Report(Outcome::Stopped(Stopped::Gone))]),
                ),
            ),
            // A mismatch under replace: stop, drop, settle, look again.
            row(
                "use: a mismatch under replace is stopped, settled and looked at again",
                replace_script,
                replace_expect,
            ),
            // A mismatch the door may not replace, each reason.
            on(
                AUTONOMOUS,
                row(
                    "use: an autonomous door never replaces",
                    vec![Reached, STALE],
                    hello(vec![Report(Outcome::NotReplaced(
                        NotReplaced::NeverReplaces,
                    ))]),
                ),
            ),
            Row {
                no_spawn: true,
                ..row(
                    "use: ACQ_NO_SPAWN forbids the replace",
                    vec![Reached, STALE],
                    hello(vec![Report(Outcome::NotReplaced(NotReplaced::NoSpawn))]),
                )
            },
            Row {
                human: false,
                ..row(
                    "use: no terminal forbids a real-mode replace",
                    vec![Reached, STALE],
                    hello(vec![Report(Outcome::NotReplaced(NotReplaced::NoTerminal))]),
                )
            },
            // Nothing listening, on a door that may not spawn, each reason.
            on(
                Door::Use {
                    spawn: false,
                    replace: true,
                },
                row(
                    "use: a door that never spawns is absent by policy",
                    vec![Nothing],
                    vec![Connect, Report(Outcome::NotSpawned(NotSpawned::Policy))],
                ),
            ),
            Row {
                no_spawn: true,
                ..row(
                    "use: ACQ_NO_SPAWN forbids the spawn",
                    vec![Nothing],
                    vec![Connect, Report(Outcome::NotSpawned(NotSpawned::NoSpawnEnv))],
                )
            },
            Row {
                human: false,
                ..row(
                    "use: no terminal forbids a real-mode spawn",
                    vec![Nothing],
                    vec![Connect, Report(Outcome::NotSpawned(NotSpawned::NoTerminal))],
                )
            },
            on(
                Door::Observe,
                row(
                    "observe: nothing listening is absent",
                    vec![Nothing],
                    vec![Connect, Report(Outcome::Absent)],
                ),
            ),
            on(
                Door::Stop,
                row(
                    "stop: nothing listening is absent",
                    vec![Nothing],
                    vec![Connect, Report(Outcome::Absent)],
                ),
            ),
            // The spawn and the startup deadline (C137).
            row(
                "use: a spawn that failed is reported",
                vec![Nothing, Event::SpawnFailed("no acqd".into())],
                vec![
                    Connect,
                    Spawn,
                    Report(Outcome::SpawnFailed("no acqd".into())),
                ],
            ),
            row(
                "use: the startup deadline at zero is not bound at the first look, nothing more performed",
                vec![Nothing, Spawned { pid: CHILD }, Deadline(Timer::Startup)],
                vec![
                    Connect,
                    Spawn,
                    Arm(Timer::Startup),
                    Consult(Timer::Startup),
                    Report(Outcome::SpawnEnded(Ended::NotBound)),
                ],
            ),
            row(
                "use: the startup deadline after a poll is not bound",
                vec![
                    Nothing,
                    Spawned { pid: CHILD },
                    Pending(Timer::Startup),
                    Deadline(Timer::BindPoll),
                    Nothing,
                    ChildRunning,
                    Deadline(Timer::Startup),
                ],
                vec![
                    Connect,
                    Spawn,
                    Arm(Timer::Startup),
                    Consult(Timer::Startup),
                    Arm(Timer::BindPoll),
                    Connect,
                    PollChild,
                    Consult(Timer::Startup),
                    Report(Outcome::SpawnEnded(Ended::NotBound)),
                ],
            ),
            row(
                "use: the startup deadline pending after a poll is another bind poll",
                vec![
                    Nothing,
                    Spawned { pid: CHILD },
                    Pending(Timer::Startup),
                    Deadline(Timer::BindPoll),
                    Nothing,
                    ChildRunning,
                    Pending(Timer::Startup),
                ],
                vec![
                    Connect,
                    Spawn,
                    Arm(Timer::Startup),
                    Consult(Timer::Startup),
                    Arm(Timer::BindPoll),
                    Connect,
                    PollChild,
                    Consult(Timer::Startup),
                    Arm(Timer::BindPoll),
                ],
            ),
            row(
                "use: the child this door spawned binds and opens, started here",
                vec![
                    Nothing,
                    Spawned { pid: CHILD },
                    Pending(Timer::Startup),
                    Deadline(Timer::BindPoll),
                    Reached,
                    ours(CHILD),
                ],
                vec![
                    Connect,
                    Spawn,
                    Arm(Timer::Startup),
                    Consult(Timer::Startup),
                    Arm(Timer::BindPoll),
                    Connect,
                    Arm(Timer::Hello),
                    SendHello,
                    Report(Outcome::Opened { started_here: true }),
                ],
            ),
            Row {
                child: Some(CHILD),
                ..row(
                    "use: a machine built with a child polls it, no spawn, no fresh arming",
                    vec![Nothing, ChildRunning, Deadline(Timer::Startup)],
                    vec![
                        Connect,
                        PollChild,
                        Consult(Timer::Startup),
                        Report(Outcome::SpawnEnded(Ended::NotBound)),
                    ],
                )
            },
            // The spawn race (C147): one look for the winner.
            named(
                "use: a child that exited on another's lock looks, and the winner found is handshaken, not started here",
                lost(
                    vec![Found, ours(WINNER)],
                    vec![
                        DropChild,
                        Arm(Timer::Hello),
                        SendHello,
                        Report(Outcome::Opened {
                            started_here: false,
                        }),
                    ],
                ),
            ),
            named(
                "use: a child that exited on another's lock, its look finding nothing, lost the race",
                lost(
                    vec![NotFound],
                    vec![Report(Outcome::SpawnEnded(Ended::Lost { holder: WINNER }))],
                ),
            ),
            named(
                "use: a look that failed is a transport failure",
                lost(
                    vec![LookFailed("reset".into())],
                    vec![Report(Outcome::Transport("reset".into()))],
                ),
            ),
            Row {
                child: Some(CHILD),
                ..row(
                    "use: a child that exited on its own lock failed by exit",
                    vec![
                        Nothing,
                        ChildExited {
                            holder: Some(CHILD),
                        },
                    ],
                    vec![
                        Connect,
                        PollChild,
                        Report(Outcome::SpawnEnded(Ended::Exited)),
                    ],
                )
            },
            Row {
                child: Some(CHILD),
                ..row(
                    "use: a child that exited with no lock holder failed by exit",
                    vec![Nothing, ChildExited { holder: None }],
                    vec![
                        Connect,
                        PollChild,
                        Report(Outcome::SpawnEnded(Ended::Exited)),
                    ],
                )
            },
            named(
                "use: a second exit on another's lock, after the look, failed by exit",
                lost(
                    vec![
                        Found,
                        stale(WINNER),
                        Event::Stopped(Stopped::Gone),
                        Deadline(Timer::Settle),
                        Nothing,
                        Spawned { pid: 88 },
                        Pending(Timer::Startup),
                        Deadline(Timer::BindPoll),
                        Nothing,
                        ChildExited {
                            holder: Some(WINNER),
                        },
                    ],
                    vec![
                        DropChild,
                        Arm(Timer::Hello),
                        SendHello,
                        Stop,
                        DropChild,
                        Arm(Timer::Settle),
                        Connect,
                        Spawn,
                        Arm(Timer::Startup),
                        Consult(Timer::Startup),
                        Arm(Timer::BindPoll),
                        Connect,
                        PollChild,
                        Report(Outcome::SpawnEnded(Ended::Exited)),
                    ],
                ),
            ),
            // C147 and C137, a spawn begins with a fresh startup deadline:
            // the loser's look cancelled the first, the winner of another
            // build is stopped, and the replacement's spawn arms its own,
            // consulted before it is called unbound.
            named(
                "use: a replacement spawned after a lost race arms its own startup deadline and opens, started here",
                lost(
                    vec![
                        Found,
                        stale(WINNER),
                        Event::Stopped(Stopped::Gone),
                        Deadline(Timer::Settle),
                        Nothing,
                        Spawned { pid: 88 },
                        Pending(Timer::Startup),
                        Deadline(Timer::BindPoll),
                        Reached,
                        ours(88),
                    ],
                    vec![
                        DropChild,
                        Arm(Timer::Hello),
                        SendHello,
                        Stop,
                        DropChild,
                        Arm(Timer::Settle),
                        Connect,
                        Spawn,
                        Arm(Timer::Startup),
                        Consult(Timer::Startup),
                        Arm(Timer::BindPoll),
                        Connect,
                        Arm(Timer::Hello),
                        SendHello,
                        Report(Outcome::Opened { started_here: true }),
                    ],
                ),
            ),
            // The stop door's answers.
            on(
                Door::Stop,
                row(
                    "stop: a stop acknowledged and gone",
                    vec![Reached, STALE, Event::Stopped(Stopped::Gone)],
                    hello(vec![Stop, Report(Outcome::Stopped(Stopped::Gone))]),
                ),
            ),
            on(
                Door::Stop,
                row(
                    "stop: a stop acknowledged, not seen gone",
                    vec![Reached, STALE, Event::Stopped(Stopped::Acknowledged)],
                    hello(vec![Stop, Report(Outcome::Stopped(Stopped::Acknowledged))]),
                ),
            ),
            on(
                Door::Stop,
                row(
                    "stop: a stop that failed",
                    vec![Reached, STALE, StopFailed("refused".into())],
                    hello(vec![Stop, Report(Outcome::StopFailed("refused".into()))]),
                ),
            ),
        ];
        // A failed stop on the replace path: kept and reported when the
        // next look finds the daemon (C147) ...
        let (script, expect) = with(
            (failed_script.clone(), failed_expect.clone()),
            vec![Reached, STALE],
            vec![
                Arm(Timer::Hello),
                SendHello,
                Report(Outcome::NotReplaced(NotReplaced::StopFailed(
                    "refused".into(),
                ))),
            ],
        );
        rows.push(row("use: a stop that failed on the replace path is reported when the daemon is still there", script, expect));
        // ... and cleared when it finds nothing: the respawn goes ahead,
        // and its child judged another build is stopped as #116 says.
        let (script, expect) = with(
            (failed_script, failed_expect),
            vec![
                Nothing,
                Spawned { pid: CHILD },
                Pending(Timer::Startup),
                Deadline(Timer::BindPoll),
                Reached,
                stale(CHILD),
            ],
            vec![
                Spawn,
                Arm(Timer::Startup),
                Consult(Timer::Startup),
                Arm(Timer::BindPoll),
                Connect,
                Arm(Timer::Hello),
                SendHello,
                Stop,
            ],
        );
        rows.push(row("use: a stop that failed on the replace path is cleared when the daemon went, and the respawn goes ahead", script, expect));
        // A second mismatch after a respawn (#116).
        let (script, expect) = with(
            respawned(stale(CHILD)),
            vec![Event::Stopped(Stopped::Acknowledged)],
            vec![Stop, still(Some(Ok(Stopped::Acknowledged)))],
        );
        rows.push(row("use: a respawned child still another build is stopped by pid, its stop carried into the refusal", script, expect));
        let (script, expect) = with(
            respawned(stale(CHILD)),
            vec![StopFailed("refused".into())],
            vec![Stop, still(Some(Err("refused".into())))],
        );
        rows.push(row(
            "use: a respawned child whose stop failed carries the words into the refusal",
            script,
            expect,
        ));
        let (script, expect) = respawned(stale(WINNER));
        let mut expect = expect;
        expect.push(still(None));
        rows.push(row("use: a daemon another process started, still another build after a respawn, is not stopped", script, expect));
        let (script, mut expect) = respawned(ours(CHILD));
        expect.push(Report(Outcome::Opened { started_here: true }));
        rows.push(row(
            "use: a respawn that binds this client's daemon opens, started here",
            script,
            expect,
        ));
        // The respawn even for a caller that does not spawn.
        let (mut script, mut expect) = replaced(Event::Stopped(Stopped::Gone));
        script.push(Nothing);
        expect.push(Spawn);
        rows.push(on(
            Door::Use {
                spawn: false,
                replace: true,
            },
            row(
                "use: a door that never spawns respawns what it stopped",
                script,
                expect,
            ),
        ));
        rows
    }

    /// C157: the door's transitions, one row each, with no clock: its
    /// run time is the runner's report, asserted by nothing (C137).
    #[test]
    fn c157_the_door_is_a_table_of_event_scripts() {
        let rows = table();
        for row in &rows {
            let machine = Machine::new(row.door, row.no_spawn, row.human, row.child);
            assert_eq!(
                drive(machine, &row.script),
                row.expect,
                "the row {:?}",
                row.name
            );
        }
    }

    /// Every row has a name of its own, so a failure names one transition.
    #[test]
    fn every_row_of_the_door_table_is_named_once() {
        let rows = table();
        let mut names: Vec<_> = rows.iter().map(|r| r.name).collect();
        assert!(names.iter().all(|n| !n.is_empty()));
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), rows.len());
    }

    /// The policy is read once (Do 1): the effective spawn of each door.
    #[test]
    fn the_door_may_spawn_only_as_its_caller_and_the_policy_allow() {
        assert!(Machine::new(INTERACTIVE, false, true, None).may_spawn());
        assert!(!Machine::new(INTERACTIVE, true, true, None).may_spawn());
        assert!(!Machine::new(INTERACTIVE, false, false, None).may_spawn());
        assert!(!Machine::new(Door::Observe, false, true, None).may_spawn());
        assert!(!Machine::new(Door::Stop, false, true, None).may_spawn());
    }
}
