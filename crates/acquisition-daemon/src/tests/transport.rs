//! C161, the in-process transports: the daemon's tests' fixtures behind
//! the transport seam (`acquisition_transport::Transport`), so a provider
//! exchange in a door scenario or a component daemon is no real I/O. Each
//! is an [`InProcess`] transport over a fixture's answer: the scripted
//! one ([`scripted`]: the full, raw and silent forms), the account
//! fixture ([`account`]), the two token fixtures ([`delayed_token`],
//! [`answering_token`]), the stateful mock's in-process face ([`mock`]),
//! the refusing one ([`refusing`]) and a scenario's own
//! ([`InProcess::new`] over its closure). Every answer is the bytes a
//! TCP fixture would have written (`mockggg::response_text`), read back
//! as a client reads them: the status line, the headers, and a body cut
//! at its `Content-Length`; a body shorter than its length fails in this
//! support's own words ([`SHORT_BODY`]), never reqwest's.
//!
//! The hold (C160 as amended, "a request a test transport holds"): one
//! count, [`Hold`], shared by every transport a door or a component
//! daemon serves. A send takes it in its first poll, where the fixture
//! records the request before anything returns `Pending`, and keeps it
//! until the answer's body completes or the future or the body is
//! dropped (a guard, so a cancel releases it): the span the gate's permit
//! covers (C21). A fixture that answers its headers and holds its body
//! behind a release ([`Reply::Held`]) keeps the hold through the body. A
//! silent reply, before its headers or after them, holds nothing and
//! never completes.
//!
//! Every transport answers at [`BASE`]; a request to another origin is
//! refused as a connect nothing answers would be, recorded nowhere.
//!
//! Rulings: `decisions/network.md`, `decisions/daemon.md`; the
//! cross-cutting ones are `AGENTS.md`'s.

use crate::mockggg::{self, HttpRequest};
use crate::ratelimit::Clock;
use acquisition_transport::{Answer, Pending, Prepared, Transport};
use http::StatusCode;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::json;
use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::{Notify, oneshot};

/// The origin every in-process fixture answers at: a name no resolver
/// reaches, so a send that escaped the seam would fail, not land.
pub(crate) const BASE: &str = "http://fixture.test";

/// A send nothing answers: [`refusing`]'s every send, and a send to an
/// origin other than [`BASE`].
pub(crate) const REFUSED: &str = "refused by the test transport: nothing answers at this address";

/// A body cut short of its `Content-Length`, in this support's words.
pub(crate) const SHORT_BODY: &str = "the test transport's body ended short of its Content-Length";

/// A response whose head does not read as one.
pub(crate) const UNREADABLE: &str = "the test transport could not read the response's head";

/// The token route's rate headers as GGG answers a grant (N33).
pub(crate) const TOKEN_HEADERS: &str = "X-Rate-Limit-Policy: token-request-limit\r\nX-Rate-Limit-Rules: Ip\r\nX-Rate-Limit-Ip: 60:30:30\r\nX-Rate-Limit-Ip-State: 1:30:0\r\n";

/// The hold: how many requests the transports sharing it hold now, from a
/// send's first poll to its body's end or its drop. The door's park hook
/// reads it (C160 as amended); a component test reads it or not.
#[derive(Clone, Default)]
pub(crate) struct Hold(Arc<AtomicUsize>);

impl Hold {
    pub(crate) fn new() -> Self {
        Hold::default()
    }

    /// The requests held now.
    pub(crate) fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    fn take(&self) -> Held {
        self.0.fetch_add(1, Ordering::SeqCst);
        Held(self.0.clone())
    }
}

/// One request's share of the [`Hold`], released when dropped.
struct Held(Arc<AtomicUsize>);

impl Drop for Held {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

/// What a fixture answers a request with.
pub(crate) enum Reply {
    /// The response as its bytes: status line, headers, body.
    Bytes(String),
    /// The response's head and body as bytes, its body withheld until
    /// the release fires (or its sender is dropped): the hold spans it.
    Held(String, oneshot::Receiver<()>),
    /// A transport failure, in words.
    Failed(String),
    /// Silent before its headers: holds nothing, never completes.
    Silent,
    /// The head as bytes, then a body that never completes and holds
    /// nothing.
    SilentBody(String),
}

/// A response as the mock writes it: the status, `extra_headers` (zero or
/// more whole `Name: value\r\n` lines) and a JSON body.
pub(crate) fn reply(status: &str, extra_headers: &str, body: &str) -> Reply {
    Reply::Bytes(mockggg::response_text(
        status,
        "application/json",
        extra_headers,
        body,
    ))
}

type Respond = Box<dyn Fn(HttpRequest) -> Pending<'static, Reply> + Send + Sync>;

/// A transport answering in-process: each request, at [`BASE`], handed
/// to the fixture's `respond` as the request the wire would carry, under
/// the [`Hold`].
pub(crate) struct InProcess {
    hold: Hold,
    respond: Respond,
}

impl InProcess {
    /// A transport over `respond`, sharing `hold`: a scenario's own (its
    /// barriers its own), and every fixture below. `respond` runs in the
    /// send's first poll, so what it records before its first await is
    /// recorded before the send first returns `Pending`.
    pub(crate) fn new<F, Fut>(hold: &Hold, respond: F) -> Self
    where
        F: Fn(HttpRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Reply> + Send + 'static,
    {
        InProcess {
            hold: hold.clone(),
            respond: Box::new(move |request| Box::pin(respond(request))),
        }
    }
}

impl Transport for InProcess {
    fn send(&self, request: Prepared) -> Pending<'_, Result<Answer, String>> {
        Box::pin(async move {
            let held = self.hold.take();
            let head = request.method == http::Method::HEAD;
            let request = http_request(request)?;
            let reply = (self.respond)(request).await;
            answered(reply, head, held).await
        })
    }
}

/// The prepared request as a fixture reads it off the wire: the method,
/// the path and query, the headers by lowercased name, the form encoded
/// as the body; refused unless it is for [`BASE`].
fn http_request(request: Prepared) -> Result<HttpRequest, String> {
    let url = url::Url::parse(&request.url).map_err(|_| REFUSED.to_string())?;
    let base = url::Url::parse(BASE).expect("the fixture base parses");
    if url.origin() != base.origin() {
        return Err(REFUSED.into());
    }
    let mut headers: HashMap<String, String> = request
        .headers
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    let body = match &request.form {
        Some(form) => {
            headers.insert(
                "content-type".into(),
                "application/x-www-form-urlencoded".into(),
            );
            url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(form)
                .finish()
        }
        None => String::new(),
    };
    let raw_query = url.query().unwrap_or_default().to_string();
    Ok(HttpRequest {
        method: request.method.as_str().to_string(),
        path: url.path().to_string(),
        query: url::form_urlencoded::parse(raw_query.as_bytes())
            .into_owned()
            .collect(),
        raw_query,
        headers,
        body,
    })
}

/// The reply as an answer, the hold moved into its body; a silent or
/// failed reply lets the hold go at once.
async fn answered(reply: Reply, head: bool, held: Held) -> Result<Answer, String> {
    let (bytes, release) = match reply {
        Reply::Bytes(bytes) => (bytes, None),
        Reply::Held(bytes, release) => (bytes, Some(release)),
        Reply::Failed(words) => return Err(words),
        Reply::Silent => {
            drop(held);
            return std::future::pending().await;
        }
        Reply::SilentBody(bytes) => {
            drop(held);
            let (status, headers, _) = read_response(&bytes, head)?;
            return Ok(Answer {
                status,
                headers,
                body: Box::pin(std::future::pending()),
            });
        }
    };
    let (status, headers, body) = read_response(&bytes, head)?;
    Ok(Answer {
        status,
        headers,
        body: Box::pin(async move {
            if let Some(release) = release {
                let _ = release.await;
            }
            drop(held);
            body
        }),
    })
}

/// A response's bytes read as a client reads them: the status line, the
/// headers, and the body cut at its `Content-Length` (none under a HEAD,
/// to the end without a length); a body shorter than its length is a
/// body failure in [`SHORT_BODY`]'s words, the head landed.
fn read_response(
    bytes: &str,
    head: bool,
) -> Result<(StatusCode, HeaderMap, Result<String, String>), String> {
    let (top, rest) = bytes.split_once("\r\n\r\n").ok_or(UNREADABLE)?;
    let mut lines = top.split("\r\n");
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .and_then(|code| StatusCode::from_u16(code).ok())
        .ok_or(UNREADABLE)?;
    let mut headers = HeaderMap::new();
    for line in lines {
        let (name, value) = line.split_once(':').ok_or(UNREADABLE)?;
        let name = HeaderName::from_bytes(name.trim().as_bytes()).map_err(|_| UNREADABLE)?;
        let value = HeaderValue::from_str(value.trim()).map_err(|_| UNREADABLE)?;
        headers.append(name, value);
    }
    let length = headers
        .get(http::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok());
    let body = match length {
        _ if head => Ok(String::new()),
        Some(length) if rest.len() < length => Err(SHORT_BODY.to_string()),
        Some(length) => Ok(rest[..length].to_string()),
        None => Ok(rest.to_string()),
    };
    Ok((status, headers, body))
}

/// A transport every send to fails as a connect nothing answers would, in
/// [`REFUSED`]'s words; nothing is recorded.
pub(crate) fn refusing(hold: &Hold) -> InProcess {
    InProcess::new(hold, |_| async { Reply::Failed(REFUSED.into()) })
}

/// One answer of a [`scripted`] transport. A response whose `status` is
/// empty is answered as its `body` gives it, the whole response as given
/// ([`ScriptedResponse::raw`]).
pub(crate) struct ScriptedResponse {
    pub(crate) method: &'static str,
    /// The request path this response answers; the transport asserts it.
    pub(crate) path: &'static str,
    pub(crate) status: &'static str,
    pub(crate) headers: String,
    pub(crate) body: String,
}

impl ScriptedResponse {
    /// A response to a `GET /fetch` written exactly as given, status
    /// line, headers and body: a body cut short of its `Content-Length`,
    /// a body that is not JSON (`roadmap/briefs/t5-design.md` §3, the
    /// wire twins of round 3).
    pub(crate) fn raw(response: &str) -> Self {
        ScriptedResponse {
            method: "GET",
            path: "/fetch",
            status: "",
            headers: String::new(),
            body: response.into(),
        }
    }

    /// The bytes this response answers with.
    fn bytes(&self) -> String {
        if self.status.is_empty() {
            return self.body.clone();
        }
        mockggg::response_text(self.status, "application/json", &self.headers, &self.body)
    }
}

/// One step of a [`scripted`] transport's script: a response, or one of
/// the silent forms for a request that never completes.
pub(crate) enum Scripted {
    Response(ScriptedResponse),
    /// The request on `method` and `path`, silent before its headers.
    Silent(&'static str, &'static str),
    /// The response's head, then a silent body.
    SilentBody(ScriptedResponse),
}

impl From<ScriptedResponse> for Scripted {
    fn from(response: ScriptedResponse) -> Self {
        Scripted::Response(response)
    }
}

/// One request as a scripted fixture received it: the method, the path,
/// the bearer its `Authorization` header carried (without `Bearer `) and
/// the body. What a send carried, which the journal never keeps (it
/// names no token and no grant).
#[derive(Debug, Clone)]
pub(crate) struct Received {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) bearer: Option<String>,
    pub(crate) body: String,
}

impl Received {
    /// The refresh grant a token `POST` carried: its form body's
    /// `refresh_token`, or `None` for any other request.
    pub(crate) fn grant(&self) -> Option<String> {
        url::form_urlencoded::parse(self.body.as_bytes())
            .find(|(name, _)| name == "refresh_token")
            .map(|(_, token)| token.into_owned())
    }
}

/// A fixture's faults and its end: what it found wrong with a request (a
/// method or path it did not expect, a request past its script), each
/// failing that send, and a wake when it has answered whole.
#[derive(Clone, Default)]
pub(crate) struct Ends {
    faults: Arc<Mutex<Vec<String>>>,
    left: Arc<Mutex<usize>>,
    done: Arc<Notify>,
}

impl Ends {
    fn expecting(count: usize) -> Self {
        let ends = Ends::default();
        *ends.left.lock().unwrap() = count;
        ends
    }

    /// A fault, recorded, as the failure the send meets.
    fn fault(&self, words: String) -> Reply {
        self.faults.lock().unwrap().push(words.clone());
        Reply::Failed(words)
    }

    /// One expected request taken.
    fn took(&self) {
        let mut left = self.left.lock().unwrap();
        *left = left.saturating_sub(1);
        if *left == 0 {
            self.done.notify_waiters();
        }
    }

    /// Resolves once every expected request has been taken.
    pub(crate) async fn answered(&self) {
        loop {
            let done = self.done.notified();
            tokio::pin!(done);
            done.as_mut().enable();
            if *self.left.lock().unwrap() == 0 {
                return;
            }
            done.await;
        }
    }

    /// The faults found, in order; none on a fixture used as expected.
    pub(crate) fn faults(&self) -> Vec<String> {
        self.faults.lock().unwrap().clone()
    }

    /// Whether every expected request has been taken.
    pub(crate) fn whole(&self) -> bool {
        *self.left.lock().unwrap() == 0
    }
}

/// What a [`scripted`] transport received: the methods (what the journal
/// is held equal to), each request whole, and its ends.
pub(crate) struct ScriptRecord {
    pub(crate) wire: Arc<Mutex<Vec<String>>>,
    pub(crate) received: Arc<Mutex<Vec<Received>>>,
    pub(crate) ends: Ends,
}

/// A transport answering `script` in order, one request each, each
/// request's method and path asserted against its step's (a mismatch, or
/// a request past the script, a fault that fails the send); it records the
/// methods and each request whole ([`Received`]), in order.
pub(crate) fn scripted(hold: &Hold, script: Vec<Scripted>) -> (InProcess, ScriptRecord) {
    let ends = Ends::expecting(script.len());
    let record = ScriptRecord {
        wire: Arc::default(),
        received: Arc::default(),
        ends: ends.clone(),
    };
    let (wire, received) = (record.wire.clone(), record.received.clone());
    let script = Arc::new(Mutex::new(VecDeque::from(script)));
    let transport = InProcess::new(hold, move |request: HttpRequest| {
        received.lock().unwrap().push(Received {
            method: request.method.clone(),
            path: request.path.clone(),
            bearer: request
                .headers
                .get("authorization")
                .and_then(|value| value.strip_prefix("Bearer "))
                .map(str::to_string),
            body: request.body.clone(),
        });
        wire.lock().unwrap().push(request.method.clone());
        let step = script.lock().unwrap().pop_front();
        let reply = match step {
            None => ends.fault(format!(
                "the script was answered whole, and {} {} came after it",
                request.method, request.path
            )),
            Some(step) => {
                let (method, path) = match &step {
                    Scripted::Response(response) | Scripted::SilentBody(response) => {
                        (response.method, response.path)
                    }
                    Scripted::Silent(method, path) => (*method, *path),
                };
                ends.took();
                if (request.method.as_str(), request.path.as_str()) != (method, path) {
                    ends.fault(format!(
                        "the script expected {method} {path}, and {} {} came",
                        request.method, request.path
                    ))
                } else {
                    match step {
                        Scripted::Response(response) => Reply::Bytes(response.bytes()),
                        Scripted::Silent(..) => Reply::Silent,
                        Scripted::SilentBody(response) => Reply::SilentBody(response.bytes()),
                    }
                }
            }
        };
        async move { reply }
    });
    (transport, record)
}

/// The account [`account`] records for a token request whose grant it
/// cannot read (no `grant_type`, or no `code` or `refresh_token` for it):
/// the request is on the record all the same.
pub(crate) const UNREAD_ACCOUNT: &str = "?";

/// One send as the account fixture saw it: the method, and the account it
/// was for (the token POST's grant, or the GET's bearer).
pub(crate) type Sent = (String, String);

/// What an [`account`] transport received, and the barriers of the one
/// request it holds.
pub(crate) struct AccountRecord {
    pub(crate) sent: Arc<Mutex<Vec<Sent>>>,
    /// Fires when the held request is in.
    pub(crate) arrived: oneshot::Receiver<()>,
    /// Answers the held request.
    pub(crate) release: oneshot::Sender<()>,
}

/// The account a request names: a token `POST`'s grant's (a refresh
/// grant's is its token's, `rt-old` being `test-user`'s and `rt-<name>`
/// `<name>`'s; a code grant's is its code), or a `GET`'s bearer's
/// (`at-<name>`); [`UNREAD_ACCOUNT`] for a grant it cannot read. Read
/// without indexing, so a request the daemon should never send is on the
/// record, never a panic.
fn account_of(request: &HttpRequest) -> String {
    if request.path == "/token" {
        let grant: HashMap<String, String> = url::form_urlencoded::parse(request.body.as_bytes())
            .into_owned()
            .collect();
        let field = |name: &str| grant.get(name).map(String::as_str);
        match (field("grant_type"), field("code"), field("refresh_token")) {
            (Some("authorization_code"), Some(code), _) => code.to_string(),
            (Some("refresh_token"), _, Some("rt-old")) => "test-user".to_string(),
            (Some("refresh_token"), _, Some(token)) => token.trim_start_matches("rt-").to_string(),
            _ => UNREAD_ACCOUNT.to_string(),
        }
    } else {
        request
            .headers
            .get("authorization")
            .map(|bearer| bearer.trim_start_matches("Bearer at-").to_string())
            .unwrap_or_default()
    }
}

/// A transport for the cancel scenarios and the logins that answers by
/// what was sent: `POST /token` with the grant's account's tokens
/// (`at-<name>`, `rt-<name>`, [`account_of`]), and `GET /profile` with the
/// bearer's account, `uuid-<name>`, without rate headers as GGG serves
/// it. Every request is recorded before it is answered. Of the requests
/// matching `hold`'s method and path, the one numbered `hold.2` (from 0)
/// is held: `arrived` fires when it is in, and it is answered when
/// `release` fires, the [`Hold`] held across.
pub(crate) fn account(
    hold: &Hold,
    held: (&'static str, &'static str, usize),
) -> (InProcess, AccountRecord) {
    let sent: Arc<Mutex<Vec<Sent>>> = Arc::default();
    let (arrived_tx, arrived) = oneshot::channel();
    let (release, release_rx) = oneshot::channel::<()>();
    let barriers = Arc::new(Mutex::new(Some((arrived_tx, release_rx))));
    let matched = Arc::new(AtomicUsize::new(0));
    let recorded = sent.clone();
    let transport = InProcess::new(hold, move |request: HttpRequest| {
        let account = account_of(&request);
        recorded
            .lock()
            .unwrap()
            .push((request.method.clone(), account.clone()));
        let barriers = ((request.method.as_str(), request.path.as_str()) == (held.0, held.1)
            && matched.fetch_add(1, Ordering::SeqCst) == held.2)
            .then(|| barriers.lock().unwrap().take())
            .flatten();
        let answer = if request.path == "/token" {
            let body = json!({
                "access_token": format!("at-{account}"),
                "refresh_token": format!("rt-{account}"),
                "expires_in": 3600,
                "username": account,
            });
            reply("200 OK", TOKEN_HEADERS, &body.to_string())
        } else {
            let body = json!({ "uuid": format!("uuid-{account}"), "name": account });
            reply("200 OK", "", &body.to_string())
        };
        async move {
            if let Some((arrived, release)) = barriers {
                let _ = arrived.send(());
                let _ = release.await;
            }
            answer
        }
    });
    let record = AccountRecord {
        sent,
        arrived,
        release,
    };
    (transport, record)
}

/// A delayed token transport's barriers and ends.
pub(crate) struct DelayedRecord {
    /// Fires when the refresh is in, its grant and token checked.
    pub(crate) arrived: oneshot::Receiver<()>,
    /// Answers the refresh.
    pub(crate) release: oneshot::Sender<()>,
    pub(crate) ends: Ends,
}

/// One refresh of `rt-old`, held: `arrived` fires when its `POST /token`
/// is in (the grant and the token checked, a fault otherwise), and it is
/// answered `status` with `body` and the token route's rate headers when
/// `release` fires, the [`Hold`] held across. A second request is a
/// fault.
pub(crate) fn delayed_token(
    hold: &Hold,
    status: &'static str,
    body: String,
) -> (InProcess, DelayedRecord) {
    let ends = Ends::expecting(1);
    let (arrived_tx, arrived) = oneshot::channel();
    let (release, release_rx) = oneshot::channel::<()>();
    let barriers = Arc::new(Mutex::new(Some((arrived_tx, release_rx))));
    let faults = ends.clone();
    let transport = InProcess::new(hold, move |request: HttpRequest| {
        let taken = barriers.lock().unwrap().take();
        let refresh = request.method == "POST"
            && request.path == "/token"
            && request.body.contains("grant_type=refresh_token")
            && request.body.contains("refresh_token=rt-old");
        let fault = match (&taken, refresh) {
            (None, _) => Some(faults.fault(format!(
                "the delayed token fixture holds one refresh, and {} {} came after it",
                request.method, request.path
            ))),
            (Some(_), false) => Some(faults.fault(format!(
                "the delayed token fixture expected a refresh of rt-old, and {} {} came",
                request.method, request.path
            ))),
            (Some(_), true) => None,
        };
        let (answer, ends) = (reply(status, TOKEN_HEADERS, &body), faults.clone());
        async move {
            if let Some(fault) = fault {
                return fault;
            }
            let (arrived, release) = taken.expect("a refresh's barriers");
            let _ = arrived.send(());
            let _ = release.await;
            ends.took();
            answer
        }
    });
    let record = DelayedRecord {
        arrived,
        release,
        ends,
    };
    (transport, record)
}

/// What an [`answering_token`] transport received: each request's form
/// body, and its ends.
pub(crate) struct TokenRecord {
    pub(crate) bodies: Arc<Mutex<Vec<String>>>,
    pub(crate) ends: Ends,
}

/// A token transport answering one request per status in `statuses`, in
/// order, each on `/token` (a fault otherwise): a `200` with the rotated
/// grant (`at-new`, `rt-rotated` for `test-user`), anything else with
/// `invalid_grant`; each request's body recorded.
pub(crate) fn answering_token(
    hold: &Hold,
    statuses: Vec<&'static str>,
) -> (InProcess, TokenRecord) {
    let ends = Ends::expecting(statuses.len());
    let record = TokenRecord {
        bodies: Arc::default(),
        ends: ends.clone(),
    };
    let bodies = record.bodies.clone();
    let statuses = Arc::new(Mutex::new(VecDeque::from(statuses)));
    let transport = InProcess::new(hold, move |request: HttpRequest| {
        bodies.lock().unwrap().push(request.body.clone());
        let status = statuses.lock().unwrap().pop_front();
        let answer = match status {
            _ if request.path != "/token" => ends.fault(format!(
                "the token fixture answers /token, and {} came",
                request.path
            )),
            None => ends.fault("the token fixture's statuses were answered whole".into()),
            Some(status) => {
                ends.took();
                let body = if status.starts_with("200") {
                    json!({
                        "access_token": "at-new",
                        "refresh_token": "rt-rotated",
                        "expires_in": 3600,
                        "username": "test-user",
                    })
                    .to_string()
                } else {
                    r#"{"error":"invalid_grant"}"#.to_string()
                };
                reply(status, TOKEN_HEADERS, &body)
            }
        };
        async move { answer }
    });
    (transport, record)
}

/// The stateful mock's in-process face: the request handed to
/// `mockggg::Mock::answer`, the one function its TCP face answers through
/// too (the registration's user-agent checked there, so both faces run
/// it), its rate-limit counters on `clock` and its windows GGG's (the
/// divisor 1, the test's). The answer comes back whole, its body ready.
/// The mock's state comes back beside it, for the pages a browser reads
/// ([`mock_page`]).
pub(crate) fn mock(hold: &Hold, clock: Arc<dyn Clock>) -> (InProcess, mockggg::Mock) {
    let mock = mockggg::Mock::new(clock, 1);
    let state = mock.clone();
    let transport = InProcess::new(hold, move |request: HttpRequest| {
        let answer = Reply::Bytes(mock.answer(&request).text());
        async move { answer }
    });
    (transport, state)
}

/// A page of the mock's, as a browser reads it in-process: `url` (at
/// [`BASE`]) asked of `mock` with no user-agent of the registration's,
/// its status, headers and body.
pub(crate) fn mock_page(
    mock: &mockggg::Mock,
    url: &str,
) -> Result<(StatusCode, HeaderMap, String), String> {
    let request = http_request(Prepared::new(http::Method::GET, url, "the-browser"))?;
    let (status, headers, body) = read_response(&mock.answer(&request).text(), false)?;
    Ok((status, headers, body?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::{Context, Poll, Waker};

    /// A prepared request on `method` to `path` at [`BASE`].
    fn prepared(method: http::Method, path: &str) -> Prepared {
        Prepared::new(method, &format!("{BASE}{path}"), "test-agent")
    }

    /// One poll of `future` with a waker that does nothing.
    fn poll_once<T>(future: &mut Pending<'_, T>) -> Poll<T> {
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
    }

    /// The record is made and the hold taken in the send's first poll,
    /// before it returns `Pending`; the hold spans the headers and the
    /// body, and goes when the body completes.
    #[tokio::test]
    async fn a_send_is_recorded_and_held_in_its_first_poll_until_its_body_ends() {
        let hold = Hold::new();
        let seen: Arc<Mutex<Vec<String>>> = Arc::default();
        let (release, released) = oneshot::channel::<()>();
        let released = Arc::new(Mutex::new(Some(released)));
        let recorder = seen.clone();
        let transport = InProcess::new(&hold, move |request: HttpRequest| {
            recorder.lock().unwrap().push(request.path.clone());
            let released = released.lock().unwrap().take().expect("one request");
            async move {
                let _ = released.await;
                reply("200 OK", "", "{}")
            }
        });
        let mut send = transport.send(prepared(http::Method::GET, "/held"));
        assert!(poll_once(&mut send).is_pending());
        assert_eq!(
            *seen.lock().unwrap(),
            ["/held"],
            "recorded in the first poll"
        );
        assert_eq!(hold.count(), 1, "held from the first poll");
        release.send(()).unwrap();
        let answer = send.await.unwrap();
        assert_eq!(hold.count(), 1, "held through the answered headers");
        assert_eq!(answer.body.await.unwrap(), "{}");
        assert_eq!(hold.count(), 0, "released at the body's end");
    }

    /// A body withheld behind a release keeps the hold until the release;
    /// a dropped send and a dropped body each release it.
    #[tokio::test]
    async fn the_hold_spans_a_withheld_body_and_goes_on_a_drop() {
        let hold = Hold::new();
        let (release, released) = oneshot::channel::<()>();
        let released = Arc::new(Mutex::new(Some(released)));
        let held_body = InProcess::new(&hold, move |_| {
            let released = released.lock().unwrap().take().expect("one request");
            async move {
                Reply::Held(
                    mockggg::response_text("200 OK", "application/json", "", "late"),
                    released,
                )
            }
        });
        let answer = held_body
            .send(prepared(http::Method::GET, "/body"))
            .await
            .unwrap();
        let mut body = answer.body;
        assert!(poll_once(&mut body).is_pending());
        assert_eq!(hold.count(), 1, "the withheld body holds");
        release.send(()).unwrap();
        assert_eq!(body.await.unwrap(), "late");
        assert_eq!(hold.count(), 0);

        let never = InProcess::new(&hold, |_| async {
            std::future::pending::<()>().await;
            Reply::Failed("unreached".into())
        });
        let mut send = never.send(prepared(http::Method::GET, "/never"));
        assert!(poll_once(&mut send).is_pending());
        assert_eq!(hold.count(), 1);
        drop(send);
        assert_eq!(hold.count(), 0, "a dropped send releases its hold");

        let (_keep, withheld) = oneshot::channel::<()>();
        let withheld = Arc::new(Mutex::new(Some(withheld)));
        let unread = InProcess::new(&hold, move |_| {
            let withheld = withheld.lock().unwrap().take().expect("one request");
            async move {
                Reply::Held(
                    mockggg::response_text("200 OK", "text/plain", "", "x"),
                    withheld,
                )
            }
        });
        let answer = unread
            .send(prepared(http::Method::GET, "/unread"))
            .await
            .unwrap();
        assert_eq!(hold.count(), 1);
        drop(answer);
        assert_eq!(hold.count(), 0, "a dropped body releases its hold");
    }

    /// The scripted transport: the full form as the mock writes it, the
    /// raw form read from its bytes (a body cut short of its length fails
    /// in the support's words, the status landed), the silent forms
    /// holding nothing, a mismatch and a request past the script each a
    /// fault, and the record as the scripted server kept it.
    #[tokio::test]
    async fn the_scripted_transport_answers_its_forms_and_records_each_request() {
        let hold = Hold::new();
        let (transport, record) = scripted(
            &hold,
            vec![
                ScriptedResponse {
                    method: "HEAD",
                    path: "/fetch",
                    status: "204 No Content",
                    headers: "X-Rate-Limit-Policy: p\r\n".into(),
                    body: String::new(),
                }
                .into(),
                ScriptedResponse::raw(
                    "HTTP/1.1 500 Internal Server Error\r\nRetry-After: 3\r\nContent-Length: 64\r\n\r\n{\"cut",
                )
                .into(),
                Scripted::Silent("GET", "/silent"),
                Scripted::SilentBody(ScriptedResponse {
                    method: "GET",
                    path: "/silent-body",
                    status: "200 OK",
                    headers: String::new(),
                    body: "{}".into(),
                }),
                ScriptedResponse::raw("HTTP/1.1 200 OK\r\n\r\n").into(),
            ],
        );
        let answer = transport
            .send(prepared(http::Method::HEAD, "/fetch"))
            .await
            .unwrap();
        assert_eq!(answer.status, StatusCode::NO_CONTENT);
        assert_eq!(answer.headers["x-rate-limit-policy"], "p");
        assert_eq!(
            answer.headers["content-type"],
            "application/json; charset=utf-8"
        );
        assert_eq!(answer.body.await.unwrap(), "");

        let mut post = prepared(http::Method::GET, "/fetch?a=b");
        post.headers.insert(
            http::header::AUTHORIZATION,
            HeaderValue::from_static("Bearer at-1"),
        );
        let answer = transport.send(post).await.unwrap();
        assert_eq!(answer.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(answer.headers["retry-after"], "3");
        assert_eq!(answer.body.await.unwrap_err(), SHORT_BODY);
        assert_eq!(hold.count(), 0);

        let mut silent = transport.send(prepared(http::Method::GET, "/silent"));
        assert!(poll_once(&mut silent).is_pending());
        assert_eq!(hold.count(), 0, "silent before its headers, nothing held");
        let answer = transport
            .send(prepared(http::Method::GET, "/silent-body"))
            .await
            .unwrap();
        assert_eq!(answer.status, StatusCode::OK);
        let mut body = answer.body;
        assert!(poll_once(&mut body).is_pending());
        assert_eq!(hold.count(), 0, "a silent body holds nothing");

        let words = transport
            .send(prepared(http::Method::POST, "/token"))
            .await
            .err();
        assert_eq!(
            words.as_deref(),
            Some("the script expected GET /fetch, and POST /token came")
        );
        let words = transport
            .send(prepared(http::Method::GET, "/fetch"))
            .await
            .err();
        assert_eq!(
            words.as_deref(),
            Some("the script was answered whole, and GET /fetch came after it")
        );
        assert!(record.ends.whole());
        assert_eq!(record.ends.faults().len(), 2);
        assert_eq!(
            *record.wire.lock().unwrap(),
            ["HEAD", "GET", "GET", "GET", "POST", "GET"]
        );
        let received = record.received.lock().unwrap();
        assert_eq!(received[1].path, "/fetch");
        assert_eq!(received[1].bearer.as_deref(), Some("at-1"));
        assert_eq!(received[0].bearer, None);
    }

    /// A form travels as the wire carries it, so `Received::grant` reads
    /// it; a send to another origin, and every send of the refusing
    /// transport, fails in [`REFUSED`]'s words with nothing recorded.
    #[tokio::test]
    async fn a_form_is_received_as_encoded_and_a_refusal_records_nothing() {
        let hold = Hold::new();
        let (transport, record) = scripted(
            &hold,
            vec![
                ScriptedResponse {
                    method: "POST",
                    path: "/token",
                    status: "200 OK",
                    headers: String::new(),
                    body: "{}".into(),
                }
                .into(),
            ],
        );
        let mut post = prepared(http::Method::POST, "/token");
        post.form = Some(vec![
            ("grant_type".into(), "refresh_token".into()),
            ("refresh_token".into(), "rt a&b".into()),
        ]);
        transport.send(post).await.unwrap();
        let received = record.received.lock().unwrap().clone();
        assert_eq!(
            received[0].body,
            "grant_type=refresh_token&refresh_token=rt+a%26b"
        );
        assert_eq!(received[0].grant().as_deref(), Some("rt a&b"));

        let elsewhere = Prepared::new(http::Method::GET, "http://127.0.0.1:1/fetch", "test-agent");
        assert_eq!(
            transport.send(elsewhere).await.err().as_deref(),
            Some(REFUSED)
        );
        assert_eq!(record.wire.lock().unwrap().len(), 1, "nothing recorded");
        let refused = refusing(&hold);
        let words = refused
            .send(prepared(http::Method::GET, "/fetch"))
            .await
            .err();
        assert_eq!(words.as_deref(), Some(REFUSED));
        assert_eq!(hold.count(), 0);
    }

    /// The account transport answers by the account a request names,
    /// records each before answering, and holds the one request its hold
    /// names behind its barriers, the [`Hold`] held across; a release
    /// fired before that request arrives lets it through at once.
    #[tokio::test]
    async fn the_account_transport_answers_by_account_and_holds_the_named_request() {
        let hold = Hold::new();
        let (transport, record) = account(&hold, ("GET", "/profile", 1));
        let get = |token: &'static str| {
            let mut get = prepared(http::Method::GET, "/profile");
            get.headers
                .insert(http::header::AUTHORIZATION, HeaderValue::from_static(token));
            get
        };
        let answer = transport.send(get("Bearer at-alice")).await.unwrap();
        assert_eq!(
            answer.body.await.unwrap(),
            r#"{"name":"alice","uuid":"uuid-alice"}"#
        );
        let mut held = transport.send(get("Bearer at-bob"));
        assert!(poll_once(&mut held).is_pending());
        assert_eq!(hold.count(), 1);
        let AccountRecord {
            sent,
            arrived,
            release,
        } = record;
        arrived.await.unwrap();
        release.send(()).unwrap();
        let answer = held.await.unwrap();
        assert_eq!(
            answer.body.await.unwrap(),
            r#"{"name":"bob","uuid":"uuid-bob"}"#
        );
        assert_eq!(hold.count(), 0);
        let mut grant = prepared(http::Method::POST, "/token");
        grant.form = Some(vec![("grant_type".into(), "refresh_token".into())]);
        let answer = transport.send(grant).await.unwrap();
        assert_eq!(answer.headers["x-rate-limit-policy"], "token-request-limit");
        answer.body.await.unwrap();
        assert_eq!(
            *sent.lock().unwrap(),
            [
                ("GET".to_string(), "alice".to_string()),
                ("GET".into(), "bob".into()),
                ("POST".into(), UNREAD_ACCOUNT.into()),
            ]
        );

        let (early, record) = account(&hold, ("GET", "/profile", 0));
        record.release.send(()).unwrap();
        let answer = early.send(get("Bearer at-carol")).await.unwrap();
        assert_eq!(answer.status, StatusCode::OK);
        record.arrived.await.unwrap();
        assert_eq!(hold.count(), 1, "the body is still owed");
        answer.body.await.unwrap();
        assert_eq!(hold.count(), 0, "no hold leaks from an early release");
    }

    /// The delayed token transport holds its one refresh behind its
    /// barriers and ends whole once answered; a second request, or a
    /// request that is not the refresh, is a fault. The answering one
    /// answers its statuses in order, recording each body.
    #[tokio::test]
    async fn the_token_transports_hold_or_answer_their_refreshes() {
        let hold = Hold::new();
        let refresh = || {
            let mut post = prepared(http::Method::POST, "/token");
            post.form = Some(vec![
                ("grant_type".into(), "refresh_token".into()),
                ("refresh_token".into(), "rt-old".into()),
            ]);
            post
        };
        let (transport, record) = delayed_token(&hold, "200 OK", "{}".into());
        let mut held = transport.send(refresh());
        assert!(poll_once(&mut held).is_pending());
        let DelayedRecord {
            arrived,
            release,
            ends,
        } = record;
        arrived.await.unwrap();
        assert_eq!(hold.count(), 1);
        assert!(!ends.whole());
        release.send(()).unwrap();
        let answer = held.await.unwrap();
        assert_eq!(answer.status, StatusCode::OK);
        answer.body.await.unwrap();
        ends.answered().await;
        assert_eq!(hold.count(), 0);
        let words = transport.send(refresh()).await.err().unwrap();
        assert!(words.contains("holds one refresh"), "{words}");
        assert_eq!(ends.faults(), [words]);
        let (other, record) = delayed_token(&hold, "200 OK", "{}".into());
        let words = other
            .send(prepared(http::Method::GET, "/fetch"))
            .await
            .err()
            .unwrap();
        assert!(words.contains("expected a refresh of rt-old"), "{words}");
        assert_eq!(record.ends.faults(), [words]);

        let (answering, record) = answering_token(&hold, vec!["200 OK", "400 Bad Request"]);
        let first = answering.send(refresh()).await.unwrap();
        assert!(first.body.await.unwrap().contains("rt-rotated"));
        let second = answering.send(refresh()).await.unwrap();
        assert_eq!(second.status, StatusCode::BAD_REQUEST);
        record.ends.answered().await;
        assert_eq!(record.bodies.lock().unwrap().len(), 2);
        assert!(record.ends.faults().is_empty());
    }

    /// The mock's in-process face answers as its TCP face does, through
    /// the one `answer`: a request without the registration's user-agent
    /// is answered 400 in-process as on the wire, and a route's answer
    /// (`GET /fetch`, its status, every header and its body) is the same
    /// on both faces of two mocks at the same state.
    #[tokio::test]
    async fn the_mocks_in_process_face_answers_as_its_tcp_face() {
        let hold = Hold::new();
        let clock: Arc<dyn Clock> = Arc::new(crate::testing::ParkingClock::new());
        let (in_process, _) = mock(&hold, clock.clone());
        let bare = Prepared::new(
            http::Method::GET,
            &format!("{BASE}/fetch"),
            "not-the-registration",
        );
        let answer = in_process.send(bare).await.unwrap();
        assert_eq!(answer.status, StatusCode::BAD_REQUEST);
        assert!(
            answer
                .body
                .await
                .unwrap()
                .contains("the registration's user-agent"),
        );
        let base = mockggg::start_with_clock(clock, 1).await.unwrap();
        let wire = reqwest::Client::new()
            .get(format!("{base}/fetch"))
            .header("user-agent", "not-the-registration")
            .send()
            .await
            .unwrap();
        assert_eq!(wire.status().as_u16(), 400);

        let fresh = || -> Arc<dyn Clock> { Arc::new(crate::testing::ParkingClock::new()) };
        let (in_process, _) = mock(&hold, fresh());
        let answer = in_process
            .send(Prepared::new(
                http::Method::GET,
                &format!("{BASE}/fetch"),
                crate::provider::USER_AGENT,
            ))
            .await
            .unwrap();
        let base = mockggg::start_with_clock(fresh(), 1).await.unwrap();
        let wire = reqwest::Client::new()
            .get(format!("{base}/fetch"))
            .header("user-agent", crate::provider::USER_AGENT)
            .send()
            .await
            .unwrap();
        let sorted = |headers: &HeaderMap| {
            let mut pairs: Vec<(String, String)> = headers
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
                .collect();
            pairs.sort();
            pairs
        };
        assert_eq!(answer.status.as_u16(), wire.status().as_u16());
        assert_eq!(sorted(&answer.headers), sorted(wire.headers()));
        assert!(answer.headers.contains_key("x-rate-limit-account"));
        assert_eq!(answer.body.await.unwrap(), wire.text().await.unwrap());
    }
}
