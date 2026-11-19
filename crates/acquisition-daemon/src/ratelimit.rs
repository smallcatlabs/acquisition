//! The header-driven rate limiter (C17) and the choke point in front of it.
//!
//! Source of truth for every rule here is `docs/design/network-ground-truth.md`,
//! cited by claim number (N…/Q…/P-…). The limiter knows nothing except what
//! GGG's responses told it (invariant 2): the last `X-Rate-Limit-*` headers
//! per policy plus when recent responses arrived. There is no local token
//! counting. The tests at the bottom are the spec (C33) — a table of
//! "these headers + this history → wait this long". The frozen C++ design
//! is a property source, not a template (C22): its properties hold here
//! with the three adaptations that entry records.
//!
//! `ChokePoint` is invariant 1 made structural: it privately owns the one
//! transport, which privately owns the workspace's only HTTP client (C161),
//! so nothing can send a request without first asking the limiter, and
//! every response is observed by it.
//!
//! Endpoints are discovered by a HEAD probe before their first real send
//! (N16's sanctioned pattern; N24: HEADs report the policy without counting),
//! so server-side residue and other tools' hits are known before we add to
//! them. A degraded probe (N20) puts the endpoint in a cooldown.
//!
//! The send-lifetime gate bounds actual daemon-owned HTTP exchanges across
//! API GETs, HEAD probes, and OAuth token requests (P-B/N33). The dispatcher
//! cap remains separate job-scheduling machinery.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use serde::{Deserialize, Serialize};

use acquisition_protocol::status::{
    DegradedEndpoint, PolicyStatus, RuleStatus, SendRecord, WindowStatus,
};

use crate::gate::{SendGate, SendPermit};
use crate::rails::{Rails, SendReport, Verdict};

mod choke;
#[cfg(test)]
pub(crate) use choke::PACING_FLOOR;
mod limiter;
mod policy;
#[cfg(test)]
#[path = "ratelimit/tests/spec.rs"]
mod spec;

pub use choke::*;
pub use limiter::*;
pub use policy::*;

/// The URL path, for logs.
pub fn url_path(url: &str) -> String {
    url::Url::parse(url)
        .map(|u| u.path().to_string())
        .unwrap_or_else(|_| url.to_string())
}

/// The one source of time for the limiter, the daemon's token expiry, and
/// the send journal. Two faces on purpose: `now` is monotonic and pauses
/// while the machine sleeps; `wall` is the clock on the wall and does not.
/// R8 was the daemon measuring token expiry with the wrong face, and a fake
/// that can move the two apart is how that hazard becomes a cheap test.
pub(crate) trait Clock: Send + Sync {
    fn now(&self) -> Instant;

    fn wall(&self) -> SystemTime;

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    /// Sleep until `deadline` on this clock's `now` face. The default
    /// reads `now` once and sleeps the remainder; a clock whose own
    /// sleep reads its now later overrides it, so the deadline stays
    /// absolute (the system clock, through tokio's `sleep_until`), and so
    /// does a clock that parks by deadline (the tests' `ParkingClock`,
    /// C158).
    fn sleep_until(&self, deadline: Instant) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        self.sleep(deadline.saturating_duration_since(self.now()))
    }

    /// `"system"` or `"manual"`: written into the journal header so a
    /// fake-clock journal can never pass for live evidence.
    fn kind(&self) -> &'static str;
}

pub(crate) struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }

    fn wall(&self) -> SystemTime {
        SystemTime::now()
    }

    fn kind(&self) -> &'static str {
        "system"
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(tokio::time::sleep(duration))
    }

    /// The absolute deadline as armed: tokio's relative `sleep` reads its
    /// own now when it is built, which the default's remainder would
    /// carry past the instant the caller meant.
    fn sleep_until(&self, deadline: Instant) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(tokio::time::sleep_until(deadline.into()))
    }
}

/// A bound that expired before its future was ready ([`within_on`],
/// [`by_on`]): the daemon's own, since tokio's `Elapsed` is built only by
/// tokio.
#[derive(Debug)]
pub(crate) struct Expired;

/// C160 — every tokio timer in the daemon sleeps through its one `Clock`:
/// `future` raced against `bound` on `clock` ([`race`]). For a bound that
/// holds no daemon (the login's callback reads); a daemon's own is
/// `Daemon::within`.
pub(crate) async fn within_on<T>(
    clock: &dyn Clock,
    bound: Duration,
    future: impl Future<Output = T>,
) -> Result<T, Expired> {
    race(future, clock.sleep(bound)).await
}

/// C161 — `future` raced against the absolute `deadline` on `clock`
/// ([`race`]): a send's deadline, taken once and spent by its answer and
/// then its body, so the body has what the answer left.
pub(crate) async fn by_on<T>(
    clock: &dyn Clock,
    deadline: Instant,
    future: impl Future<Output = T>,
) -> Result<T, Expired> {
    race(future, clock.sleep_until(deadline)).await
}

/// The one race of a future against a bound's sleep, `biased` so the
/// future is polled first and one ready at its bound's instant wins, as
/// tokio's `timeout` has it. The sleep is polled outside the task's
/// cooperative budget, as tokio's `timeout` polls its timer, so a future
/// that spends the whole budget on every poll never holds an expired
/// bound back (the system clock's sleep spends that budget). The loser is
/// dropped when the race ends.
async fn race<T>(
    future: impl Future<Output = T>,
    bound: Pin<Box<dyn Future<Output = ()> + Send + '_>>,
) -> Result<T, Expired> {
    tokio::select! {
        biased;
        done = future => Ok(done),
        () = tokio::task::unconstrained(bound) => Err(Expired),
    }
}
