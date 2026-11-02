//! C161 — the transport seam. The choke point prepares every request the
//! daemon sends and classifies every answer; a transport only carries the
//! prepared request to the provider and brings back what it answered.
//!
//! A [`Prepared`] request is a value: its method, its URL as the caller
//! gave it, its headers and, for a POST, its form. The choke point builds
//! it after admission and after the bearer is read (C87), C128's origin
//! check already passed, and writes on it the registration's user-agent
//! (`provider::USER_AGENT`, invariant 4) and the bearer, a header value
//! marked sensitive that `Secret::authorize` writes, so the token's text
//! is read in `auth.rs` alone.
//!
//! An [`Answer`] is the status, the response headers and a body that
//! completes later. The body is a future on purpose: the limiter observes
//! the headers before the body is read, and the gate's permit spans the
//! body's completion (C21), so the choke point awaits it under the same
//! permit, after the observation, as it awaited reqwest's body before the
//! seam.
//!
//! The production transport is this module's private `Production`: the one
//! reqwest client, owned privately, built and reachable by the choke point
//! alone (this module is `choke`'s child, its type and constructor visible
//! to `choke` and nothing above it). The user-agent has two readers of the
//! one constant: the prepared request, so every send is shaped with it
//! where it is shaped, and the client's default, kept because reqwest
//! sends a client's default on an HTTPS proxy's `CONNECT`, which no
//! request header reaches. The client follows no redirect (C130), keeps
//! both timeouts, and routes through the proxy variables as reqwest's
//! default does (C150). Its failures, the send's and the body's, are
//! reqwest's own words. `tools/arch-check.sh` check 13 holds the word
//! `reqwest` to this file among the daemon's production sources.
//!
//! What comes next: in-process transports for the daemon's tests, so a
//! door scenario's provider exchange is no real I/O and a request held at
//! its fixture holds the scenario's clock; then the request's deadline on
//! the daemon's clock in place of the client's timeouts.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use http::header::{HeaderMap, HeaderValue, USER_AGENT};
use http::{Method, StatusCode};
use std::future::Future;
use std::pin::Pin;

use super::{CONNECT_TIMEOUT, REQUEST_TIMEOUT};

/// A future the choke point awaits, boxed so [`Transport`] stays
/// object-safe.
pub(crate) type Pending<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A request as the choke point prepared it, for a transport to carry.
/// No `Debug`: its headers may carry a bearer and its form a token, and
/// a value that cannot be formatted names neither (#113).
pub(crate) struct Prepared {
    pub(crate) method: Method,
    /// The URL as the caller gave it; one that does not parse fails in
    /// the transport, as a send failure.
    pub(crate) url: String,
    pub(crate) headers: HeaderMap,
    /// A POST's form, its pairs in order.
    pub(crate) form: Option<Vec<(String, String)>>,
}

impl Prepared {
    /// `method` to `url`, carrying the registration's user-agent
    /// (invariant 4) and nothing else yet.
    pub(crate) fn new(method: Method, url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(crate::provider::USER_AGENT),
        );
        Prepared {
            method,
            url: url.to_string(),
            headers,
            form: None,
        }
    }
}

/// What a transport brought back: the status and headers as they landed,
/// and the body still to come, its text or its failure in words.
pub(crate) struct Answer {
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) body: Pending<'static, Result<String, String>>,
}

/// What carries a prepared request and answers. The choke point holds
/// one, privately, and awaits it under the send's permit.
pub(crate) trait Transport: Send + Sync {
    /// Carries `request`: its answer, or a transport failure in words.
    fn send(&self, request: Prepared) -> Pending<'_, Result<Answer, String>>;
}

/// The production transport: the one reqwest client.
pub(super) struct Production {
    client: reqwest::Client,
}

impl Production {
    pub(super) fn new() -> Self {
        Production {
            client: build_client(),
        }
    }
}

impl Transport for Production {
    fn send(&self, request: Prepared) -> Pending<'_, Result<Answer, String>> {
        Box::pin(async move {
            let Prepared {
                method,
                url,
                headers,
                form,
            } = request;
            let mut builder = self.client.request(method, url).headers(headers);
            if let Some(form) = &form {
                builder = builder.form(form);
            }
            let response = builder.send().await.map_err(|error| error.to_string())?;
            Ok(Answer {
                status: response.status(),
                headers: response.headers().clone(),
                body: Box::pin(
                    async move { response.text().await.map_err(|error| error.to_string()) },
                ),
            })
        })
    }
}

fn build_client() -> reqwest::Client {
    // The user-agent goes on the client too, from the same constant the
    // prepared request reads: reqwest sends the client's default on an
    // HTTPS proxy's `CONNECT`, which no request header reaches
    // (`AGENTS.md` invariant 4).
    // C130: no redirect is followed. A 3xx lands as the response it is,
    // observed and journaled with its `location`; a followed 307 would
    // re-send a token request's form body wherever `Location` points, as
    // a second send under this one's permit and journal line.
    reqwest::Client::builder()
        .user_agent(crate::provider::USER_AGENT)
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .expect("reqwest client builds")
}
