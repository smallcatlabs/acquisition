//! The gate's search (C21, one live send per policy, a HEAD exclusive):
//! every sequence of the gate's events over three tasks and three keys, to
//! a depth, against a model of its own.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! The events are acquiring a permit under a key, acquiring the HEAD
//! permit, a route learning its policy's name (a rekey), releasing a live
//! permit, cancelling a waiting one, and releasing one permit and then
//! cancelling a waiter the release may have granted before anyone polled
//! it. After each event every waiter is polled, as a woken task would be.
//!
//! The oracle never reads the gate's state: it keeps which task waits or
//! holds which key, and moves a key on a rekey as the route does. After
//! each event it holds five properties:
//!
//! - (a) at most [`MAX_ACTUAL_SENDS`] permits are live;
//! - (b) a newly granted permit never shares its key with a live one;
//! - (c) a live HEAD has nothing live beside it;
//! - (d) a waiting HEAD is granted once nothing is live;
//! - (e) with no HEAD live or waiting, no waiter whose key has nothing live
//!   is held while a slot is free.
//!
//! A rekey that joins two live permits on one key is not a failure: those
//! sends are out and cannot be recalled. What it may not do is let a third
//! out beside them, which (b) refuses.
//!
//! The search runs to depth 6 (about 1.2 million sequences). The variable
//! `ACQ_GATE_DEPTH` deepens it by hand; a smaller value is ignored, since a
//! shallower search would hold less than the gate's run does.

use std::fmt;
use std::future::Future;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use super::*;

const KEYS: [&str; 3] = ["a", "b", "c"];
const TASKS: usize = 3;
const DEPTH: usize = 6;

#[derive(Clone, Copy)]
enum Ev {
    Acquire(usize, usize),
    Head(usize),
    Rekey(usize, usize),
    Release(usize),
    Cancel(usize),
    ReleaseThenCancel(usize, usize),
}

impl fmt::Debug for Ev {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Ev::Acquire(t, k) => write!(f, "Acquire({t}, {})", KEYS[k]),
            Ev::Head(t) => write!(f, "Head({t})"),
            Ev::Rekey(r, p) => write!(f, "Rekey({} → {})", KEYS[r], KEYS[p]),
            Ev::Release(t) => write!(f, "Release({t})"),
            Ev::Cancel(t) => write!(f, "Cancel({t})"),
            Ev::ReleaseThenCancel(t, u) => write!(f, "ReleaseThenCancel({t}, {u})"),
        }
    }
}

type Grant = Pin<Box<dyn Future<Output = SendPermit>>>;

/// One task as the model sees it; `key` is `None` for a HEAD.
enum Task {
    Idle,
    Waiting {
        key: Option<usize>,
        grant: Grant,
    },
    Live {
        key: Option<usize>,
        _permit: SendPermit,
    },
}

impl Task {
    fn key(&self) -> Option<Option<usize>> {
        match self {
            Task::Idle => None,
            Task::Waiting { key, .. } | Task::Live { key, .. } => Some(*key),
        }
    }
}

struct World {
    gate: SendGate,
    tasks: Vec<Task>,
}

impl World {
    fn new() -> Self {
        Self {
            gate: SendGate::new(),
            tasks: (0..TASKS).map(|_| Task::Idle).collect(),
        }
    }
}

/// The events the model allows from here.
fn enabled(tasks: &[Task]) -> Vec<Ev> {
    let mut out = Vec::new();
    for (t, task) in tasks.iter().enumerate() {
        match task {
            Task::Idle => {
                out.extend((0..KEYS.len()).map(|k| Ev::Acquire(t, k)));
                out.push(Ev::Head(t));
            }
            Task::Live { .. } => {
                out.push(Ev::Release(t));
                for (u, other) in tasks.iter().enumerate() {
                    if matches!(other, Task::Waiting { .. }) {
                        out.push(Ev::ReleaseThenCancel(t, u));
                    }
                }
            }
            Task::Waiting { .. } => out.push(Ev::Cancel(t)),
        }
    }
    for r in 0..KEYS.len() {
        if tasks.iter().any(|t| t.key() == Some(Some(r))) {
            out.extend((0..KEYS.len()).filter(|&p| p != r).map(|p| Ev::Rekey(r, p)));
        }
    }
    out
}

/// What one step did beside passing: whether a rekey joined two live keys.
struct Stepped {
    joined: bool,
}

/// Apply one event, poll every waiter, then hold the oracle.
fn step(w: &mut World, ev: Ev) -> Result<Stepped, &'static str> {
    let mut joined = false;
    match ev {
        Ev::Acquire(t, k) => {
            let gate = w.gate.clone();
            let key = KEYS[k];
            w.tasks[t] = Task::Waiting {
                key: Some(k),
                grant: Box::pin(async move { gate.acquire(key).await }),
            };
        }
        Ev::Head(t) => {
            let gate = w.gate.clone();
            w.tasks[t] = Task::Waiting {
                key: None,
                grant: Box::pin(async move { gate.acquire_head().await }),
            };
        }
        Ev::Rekey(r, p) => {
            let live_on = |k: usize| {
                (w.tasks.iter()).any(|t| matches!(t, Task::Live { key: Some(x), .. } if *x == k))
            };
            joined = live_on(r) && live_on(p);
            w.gate.rekey_policy(KEYS[r], KEYS[p]);
            for task in &mut w.tasks {
                if let Task::Waiting { key, .. } | Task::Live { key, .. } = task
                    && *key == Some(r)
                {
                    *key = Some(p);
                }
            }
        }
        Ev::Release(t) | Ev::Cancel(t) => w.tasks[t] = Task::Idle,
        Ev::ReleaseThenCancel(t, u) => {
            w.tasks[t] = Task::Idle;
            w.tasks[u] = Task::Idle;
        }
    }

    let mut newly = Vec::new();
    for (i, task) in w.tasks.iter_mut().enumerate() {
        if let Task::Waiting { key, grant } = task
            && let Poll::Ready(permit) =
                grant.as_mut().poll(&mut Context::from_waker(Waker::noop()))
        {
            *task = Task::Live {
                key: *key,
                _permit: permit,
            };
            newly.push(i);
        }
    }

    let live: Vec<(usize, Option<usize>)> = (w.tasks.iter().enumerate())
        .filter_map(|(i, t)| match t {
            Task::Live { key, .. } => Some((i, *key)),
            _ => None,
        })
        .collect();
    let heads_live = live.iter().filter(|(_, k)| k.is_none()).count();
    if live.len() > MAX_ACTUAL_SENDS {
        return Err("(a) more permits are live than the gate's cap");
    }
    for &i in &newly {
        if let Some(Some(k)) = w.tasks[i].key()
            && live.iter().any(|&(j, kj)| j != i && kj == Some(k))
        {
            return Err("(b) a new permit was granted on a key that already has a live one");
        }
    }
    if heads_live > 0 && live.len() > 1 {
        return Err("(c) a HEAD is live beside another permit");
    }
    let head_waiting = (w.tasks.iter()).any(|t| matches!(t, Task::Waiting { key: None, .. }));
    if head_waiting && live.is_empty() {
        return Err("(d) a HEAD waits with nothing live");
    }
    if !head_waiting && heads_live == 0 && live.len() < MAX_ACTUAL_SENDS {
        let held = w.tasks.iter().any(|t| {
            matches!(t, Task::Waiting { key: Some(k), .. } if !live.iter().any(|(_, kl)| *kl == Some(*k)))
        });
        if held {
            return Err("(e) an eligible ordinary waiter is held with a slot free");
        }
    }
    Ok(Stepped { joined })
}

/// Replay `prefix` (known clean) and then `last` on a fresh world.
fn replay(prefix: &[Ev], last: Ev) -> Result<(Stepped, Vec<Ev>), String> {
    let mut world = World::new();
    let result = catch_unwind(AssertUnwindSafe(|| {
        for &ev in prefix {
            step(&mut world, ev).expect("the prefix was clean");
        }
        step(&mut world, last)
    }));
    match result {
        Ok(Ok(stepped)) => {
            let next = enabled(&world.tasks);
            Ok((stepped, next))
        }
        Ok(Err(violation)) => {
            std::mem::forget(world);
            Err(violation.to_string())
        }
        Err(panic) => {
            // The gate's mutex may be poisoned, and dropping the world would
            // panic again: leak it.
            std::mem::forget(world);
            let message = (panic.downcast_ref::<&str>().map(|s| s.to_string()))
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_default();
            Err(format!("the gate panicked: {message}"))
        }
    }
}

#[derive(Default)]
struct Search {
    sequences: u64,
    joins: u64,
    /// The shortest failing sequence found, and what it broke.
    shortest: Option<(Vec<Ev>, String)>,
}

fn search(prefix: &mut Vec<Ev>, next: &[Ev], depth: usize, found: &mut Search) {
    for &ev in next {
        found.sequences += 1;
        let outcome = replay(prefix, ev);
        prefix.push(ev);
        match outcome {
            Err(violation) => {
                if found
                    .shortest
                    .as_ref()
                    .is_none_or(|(s, _)| s.len() > prefix.len())
                {
                    found.shortest = Some((prefix.clone(), violation));
                }
            }
            Ok((stepped, after)) => {
                found.joins += u64::from(stepped.joined);
                if prefix.len() < depth {
                    search(prefix, &after, depth, found);
                }
            }
        }
        prefix.pop();
    }
}

#[test]
fn c21_every_sequence_of_the_gates_events_to_its_depth_holds_one_live_permit_per_policy() {
    let depth = std::env::var("ACQ_GATE_DEPTH")
        .ok()
        .and_then(|s| s.parse().ok())
        .map_or(DEPTH, |d: usize| d.max(DEPTH));

    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let started = std::time::Instant::now();
    let mut found = Search::default();
    search(
        &mut Vec::new(),
        &enabled(&World::new().tasks),
        depth,
        &mut found,
    );
    std::panic::set_hook(hook);
    println!(
        "depth {depth}: {} sequences in {:.2?}, {} rekeys joining two live permits",
        found.sequences,
        started.elapsed(),
        found.joins
    );

    if let Some((sequence, violation)) = found.shortest {
        panic!("{violation}, after {sequence:?}");
    }
    assert!(
        found.joins > 0,
        "the search never reached a rekey that joins two live permits, so it no longer covers \
         the state a learned policy name can leave"
    );
}
