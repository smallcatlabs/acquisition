//! The crate's test support (C158, a daemon scenario's time moves by its
//! own hand or when nothing it started can run): the one test clock every
//! daemon test runs on, and the one hang ceiling, visible to the
//! daemon's tests and to the limiter's spec alike.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use crate::ratelimit::Clock;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};

/// Wall-clock origin for scenarios: 2000-01-01T00:00:00Z. Deliberately
/// synthetic so a journal written under this clock is recognizable as a
/// scenario even if someone strips the header line.
pub(crate) const MANUAL_WALL_ORIGIN: Duration = Duration::from_secs(946_684_800);

/// C158, the clock every daemon test runs on (found by the 4-0 spike,
/// `roadmap/briefs/4-0-spike.md`): a clock whose sleeps park, moved by
/// the door's park hook in a scenario and by the test's own hand in a
/// component test; nothing moves it by itself. `sleep`
/// and `sleep_until` park in a map ordered by deadline (then by arrival,
/// so equal deadlines wake first-come); `advance_to(t)` wakes
/// every sleeper due by `t`, in deadline order; `advance_to_next()` moves
/// to the earliest parked deadline and wakes the sleepers due there, so
/// each sleeper wakes at its own deadline, never a summed overshoot;
/// `sleepers()` is how many are parked now (a woken or dropped sleep is
/// not). Two faces, `now` and `wall`, from [`MANUAL_WALL_ORIGIN`], its
/// journal's clock `manual`. Tokio's paused clock behind the project's
/// own trait, without `test-util` (C84).
pub(crate) struct ParkingClock {
    inner: Mutex<Parked>,
}

struct Parked {
    now: Instant,
    wall: SystemTime,
    next_id: u64,
    parked: std::collections::BTreeMap<(Instant, u64), std::task::Waker>,
    woken: HashSet<u64>,
}

impl ParkingClock {
    pub(crate) fn new() -> Self {
        ParkingClock {
            inner: Mutex::new(Parked {
                now: Instant::now(),
                wall: SystemTime::UNIX_EPOCH + MANUAL_WALL_ORIGIN,
                next_id: 0,
                parked: Default::default(),
                woken: HashSet::new(),
            }),
        }
    }

    /// Time passes normally by `duration`: both faces move, and every
    /// sleeper due by then wakes ([`Self::advance_to`]).
    pub(crate) fn advance(&self, duration: Duration) {
        let to = self
            .now()
            .checked_add(duration)
            .expect("bounded test deadline");
        self.advance_to(to);
    }

    /// Move both faces to `t` (never back) and wake, in deadline order,
    /// every sleeper due by `t`. Waking only schedules: a woken task runs
    /// when the test yields, and reads `now` as `t`.
    pub(crate) fn advance_to(&self, t: Instant) {
        let wakers = {
            let mut p = self.inner.lock().unwrap();
            if t > p.now {
                let by = t - p.now;
                p.now = t;
                p.wall += by;
            }
            let now = p.now;
            let mut wakers = Vec::new();
            while let Some(entry) = p.parked.first_entry() {
                if entry.key().0 > now {
                    break;
                }
                let ((_, id), waker) = entry.remove_entry();
                p.woken.insert(id);
                wakers.push(waker);
            }
            wakers
        };
        for waker in wakers {
            waker.wake();
        }
    }

    /// Move to the earliest parked deadline and wake what is due there;
    /// the deadline moved to, or `None` when nothing is parked.
    pub(crate) fn advance_to_next(&self) -> Option<Instant> {
        let next = self.next_deadline()?;
        self.advance_to(next);
        Some(next)
    }

    /// How many sleeps are parked now.
    pub(crate) fn sleepers(&self) -> usize {
        self.inner.lock().unwrap().parked.len()
    }

    /// The earliest parked deadline, if any.
    pub(crate) fn next_deadline(&self) -> Option<Instant> {
        self.inner.lock().unwrap().parked.keys().next().map(|k| k.0)
    }

    /// The lid closes: the wall clock moves, the monotonic clock does not,
    /// and no sleeper wakes (nothing parked is due on the wall).
    pub(crate) fn laptop_sleep(&self, duration: Duration) {
        self.inner.lock().unwrap().wall += duration;
    }

    fn park(&self, deadline: Instant) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(ParkedSleep {
            clock: self,
            deadline,
            id: None,
        })
    }
}

/// One parked sleep: registered on its first poll, ready once woken,
/// unregistered when dropped (a cancelled sleep is never counted).
struct ParkedSleep<'a> {
    clock: &'a ParkingClock,
    deadline: Instant,
    id: Option<u64>,
}

impl Future for ParkedSleep<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        let mut p = self.clock.inner.lock().unwrap();
        match self.id {
            Some(id) => {
                if p.woken.remove(&id) {
                    drop(p);
                    self.id = None;
                    return std::task::Poll::Ready(());
                }
                p.parked.insert((self.deadline, id), cx.waker().clone());
                std::task::Poll::Pending
            }
            None => {
                if self.deadline <= p.now {
                    return std::task::Poll::Ready(());
                }
                let id = p.next_id;
                p.next_id += 1;
                p.parked.insert((self.deadline, id), cx.waker().clone());
                drop(p);
                self.id = Some(id);
                std::task::Poll::Pending
            }
        }
    }
}

impl Drop for ParkedSleep<'_> {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            let mut p = self.clock.inner.lock().unwrap();
            p.parked.remove(&(self.deadline, id));
            p.woken.remove(&id);
        }
    }
}

impl Clock for ParkingClock {
    fn now(&self) -> Instant {
        self.inner.lock().unwrap().now
    }

    fn wall(&self) -> SystemTime {
        self.inner.lock().unwrap().wall
    }

    fn kind(&self) -> &'static str {
        "manual"
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let deadline = self
            .now()
            .checked_add(duration)
            .expect("bounded test deadline");
        self.park(deadline)
    }

    fn sleep_until(&self, deadline: Instant) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        self.park(deadline)
    }
}

/// The hang ceiling: how long a test waits before it calls a wait hung,
/// never a measure of the wait (C137, the deadline's contract is its
/// expiry behaviour); the process harness's ceiling.
pub(crate) const HANG: Duration = Duration::from_secs(120);
