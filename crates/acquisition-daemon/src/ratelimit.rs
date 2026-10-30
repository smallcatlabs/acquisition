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
//! `ChokePoint` is invariant 1 made structural: it privately owns the
//! workspace's only `reqwest::Client`, so nothing can send a request
//! without first asking the limiter, and every response is observed by it.
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
}
