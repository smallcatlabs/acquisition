//! C161 — the transport seam. The daemon's choke point prepares every
//! request the daemon sends and classifies every answer; a transport only
//! carries the prepared request to the provider and brings back what it
//! answered. This crate holds the seam's neutral types and the one
//! production transport, the one HTTP client in the workspace's
//! production graph; the daemon alone links it (C1 as amended, #160).
//!
//! A [`Prepared`] request is a value: its method, its URL as the caller
//! gave it, its headers and, for a POST, its form. The choke point builds
//! it after admission and after the bearer is read (C87), C128's origin
//! check already passed, and writes on it the registration's user-agent
//! (the daemon's `provider::USER_AGENT`, invariant 4, given to
//! [`Prepared::new`]) and the bearer, a header value marked sensitive
//! that the daemon's `Secret::authorize` writes, so the token's text is
//! read in the daemon's `auth.rs` alone.
//!
//! An [`Answer`] is the status, the response headers and a body that
//! completes later. The body is a future on purpose: the limiter observes
//! the headers before the body is read, and the gate's permit spans the
//! body's completion (C21), so the choke point awaits it under the same
//! permit, after the observation, as it awaited reqwest's body before the
//! seam.
//!
//! The production transport, [`Production`], owns the one reqwest client
//! privately. What holds the one client: the graph holds that no other
//! crate in the workspace's production build reaches reqwest
//! (`tools/arch-check.sh` check 5), and the compiler that the daemon, with
//! no dependency on reqwest of its own, can name it only through what this
//! crate exports. Two properties are held by review, by the owner's ruling
//! on #160, and by no check: the daemon's choke point is the one caller of
//! [`Production::new`]; and this crate's public API exports no reqwest
//! item, no re-export, alias or type naming it, so the client stays
//! reachable only through [`Transport`]. The public items, every one read
//! at any change to this crate (`REVIEW-CHECKLIST.md`, network): [`Pending`]
//! (a boxed future); [`Prepared`] with its fields `method`, `url`, `headers`
//! and `form` and [`Prepared::new`]; [`Answer`] with its fields `status`,
//! `headers` and `body`; [`Transport`] and its `send`; [`Production`] (no
//! public field) and [`Production::new`]. Each names `http`'s types and
//! std's, none of reqwest's.
//!
//! The user-agent has two readers of the one constant, both given by the
//! daemon: the prepared request, so every
//! send is shaped with it where it is shaped, and the client's default,
//! kept because reqwest sends a client's default on an HTTPS proxy's
//! `CONNECT`, which no request header reaches. The client follows no
//! redirect (C130), keeps no timeout, and routes through the proxy
//! variables as reqwest's default does (C150). A send's deadline is the
//! daemon's choke point's, on the daemon's clock, over the answer and the
//! body (C161): it drops the transport's future at the cut, which closes
//! the connection, so no timer here bounds a send. Its failures, the
//! send's and the body's, are reqwest's own words; a cut's are the
//! daemon's.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use http::header::{HeaderMap, HeaderValue, USER_AGENT};
use http::{Method, StatusCode};
use std::future::Future;
use std::pin::Pin;

/// A future the choke point awaits, boxed so [`Transport`] stays
/// object-safe.
pub type Pending<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A request as the choke point prepared it, for a transport to carry.
/// No `Debug`: its headers may carry a bearer and its form a token, and
/// a value that cannot be formatted names neither (#113).
pub struct Prepared {
    pub method: Method,
    /// The URL as the caller gave it; one that does not parse fails in
    /// the transport, as a send failure.
    pub url: String,
    pub headers: HeaderMap,
    /// A POST's form, its pairs in order.
    pub form: Option<Vec<(String, String)>>,
}

impl Prepared {
    /// `method` to `url`, carrying the registration's `user_agent`
    /// (invariant 4) and nothing else yet.
    pub fn new(method: Method, url: &str, user_agent: &'static str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));
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
pub struct Answer {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Pending<'static, Result<String, String>>,
}

/// What carries a prepared request and answers. The choke point holds
/// one, privately, and awaits it under the send's permit.
pub trait Transport: Send + Sync {
    /// Carries `request`: its answer, or a transport failure in words.
    fn send(&self, request: Prepared) -> Pending<'_, Result<Answer, String>>;
}

/// The production transport: the one reqwest client.
pub struct Production {
    client: reqwest::Client,
}

impl Production {
    /// The one client, carrying `user_agent` as its default, with no
    /// timeout of its own (the deadline is the choke point's, C161).
    pub fn new(user_agent: &'static str) -> Self {
        Production {
            client: build_client(user_agent),
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

fn build_client(user_agent: &'static str) -> reqwest::Client {
    // The user-agent goes on the client too, from the same constant the
    // prepared request reads: reqwest sends the client's default on an
    // HTTPS proxy's `CONNECT`, which no request header reaches
    // (`AGENTS.md` invariant 4).
    // C130: no redirect is followed. A 3xx lands as the response it is,
    // observed and journaled with its `location`; a followed 307 would
    // re-send a token request's form body wherever `Location` points, as
    // a second send under this one's permit and journal line.
    reqwest::Client::builder()
        .user_agent(user_agent)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("reqwest client builds")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    /// How long the test waits before it calls the exchange hung, never a
    /// measure of it (C137): the daemon's tests' hang ceiling, its value
    /// here since this crate links none of theirs. The client keeps no
    /// timeout, so nothing else bounds a listener that stops answering.
    const HANG: Duration = Duration::from_secs(120);

    /// The user-agent a request read off `stream` carries: its head read
    /// to the blank line, its header found case-blind.
    async fn user_agent_of(stream: &mut tokio::net::TcpStream) -> Option<String> {
        let mut head = Vec::new();
        let mut byte = [0u8; 1];
        while !head.ends_with(b"\r\n\r\n") {
            if stream.read(&mut byte).await.ok()? == 0 {
                return None;
            }
            head.push(byte[0]);
        }
        String::from_utf8(head)
            .ok()?
            .lines()
            .filter_map(|line| line.split_once(':'))
            .find(|(name, _)| name.eq_ignore_ascii_case("user-agent"))
            .map(|(_, value)| value.trim().to_string())
    }

    /// Invariant 4 through the seam: the user-agent the production
    /// transport sends is the prepared request's, not the client's
    /// default. The choke point writes the same constant in both places,
    /// so the spec's table cannot tell them apart on the wire; here the
    /// prepared value differs and the listener reads it. A real-wiring
    /// test of the production transport, which stays TCP for good (C161).
    #[tokio::test]
    async fn the_production_transport_sends_the_prepared_user_agent_not_its_default() {
        tokio::time::timeout(HANG, sends_the_prepared_user_agent())
            .await
            .expect("the exchange hung");
    }

    async fn sends_the_prepared_user_agent() {
        const AGENT: &str = "c161-distinct-agent";
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/agent", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let agent = user_agent_of(&mut stream).await;
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                .await
                .unwrap();
            agent
        });
        let mut request = Prepared::new(Method::GET, &url, "acquisition-test-default");
        request
            .headers
            .insert(USER_AGENT, HeaderValue::from_static(AGENT));
        let answer = Production::new("acquisition-test-default")
            .send(request)
            .await
            .unwrap();
        assert_eq!(answer.status, StatusCode::OK);
        assert_eq!(answer.body.await.unwrap(), "");
        assert_eq!(server.await.unwrap().as_deref(), Some(AGENT));
    }
}
