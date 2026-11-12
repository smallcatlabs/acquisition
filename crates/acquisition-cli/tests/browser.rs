//! The browser-driven check (C187): `acq web`'s pages loaded in a
//! headless Chromium-family browser the harness starts on a profile under
//! the test's scratch (`harness::start_browser`), spoken to over its
//! debugging protocol by the loopback WebSocket client below, no crate
//! added. It holds the script rules no other test reaches: the count
//! beside a form's button follows that form and no other (`count.js`),
//! and the stat picker fills the first empty stat row and announces the
//! write (`picker.js`). The tests type and click as a user's keys and
//! pointer would, and compare what the page shows with what the server
//! answers for the same form.
//!
//! No wait has a bound of its own: each waits in the page for what it
//! expects, and a page that never shows it is a test the runner ends at
//! its hang ceiling (C137).

use std::collections::VecDeque;
use std::io::{Read as _, Write as _};
use std::net::TcpStream;
use std::path::PathBuf;

use serde_json::{Value, json};

use crate::harness;
use crate::web::offline_web;

/// One WebSocket to a debugging target: text frames, masked out, as
/// RFC 6455 has a client send them, and the events read while waiting
/// for a call's reply kept for a later wait.
struct Cdp {
    stream: TcpStream,
    next: u64,
    events: VecDeque<Value>,
}

impl Cdp {
    fn connect(ws_url: &str) -> Cdp {
        let rest = ws_url.strip_prefix("ws://").expect("a ws:// URL");
        let (addr, path) = rest.split_once('/').expect("a path");
        let mut stream = TcpStream::connect(addr).expect("connecting to the browser");
        write!(
            stream,
            "GET /{path} HTTP/1.1\r\nHost: {addr}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\
             Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n"
        )
        .expect("the handshake");
        let mut head = Vec::new();
        while !head.ends_with(b"\r\n\r\n") {
            let mut b = [0u8];
            stream.read_exact(&mut b).expect("the handshake's answer");
            head.push(b[0]);
        }
        let head = String::from_utf8_lossy(&head);
        assert!(head.starts_with("HTTP/1.1 101"), "no upgrade: {head}");
        Cdp {
            stream,
            next: 0,
            events: VecDeque::new(),
        }
    }

    fn send(&mut self, text: &str) {
        let data = text.as_bytes();
        let mask = [0x12u8, 0x34, 0x56, 0x78];
        let mut frame = vec![0x81u8];
        match data.len() {
            n if n < 126 => frame.push(0x80 | n as u8),
            n if n < 65536 => {
                frame.push(0x80 | 126);
                frame.extend_from_slice(&(n as u16).to_be_bytes());
            }
            n => {
                frame.push(0x80 | 127);
                frame.extend_from_slice(&(n as u64).to_be_bytes());
            }
        }
        frame.extend_from_slice(&mask);
        frame.extend(data.iter().enumerate().map(|(i, b)| b ^ mask[i % 4]));
        self.stream.write_all(&frame).expect("sending a frame");
    }

    fn receive(&mut self) -> Value {
        let mut message = Vec::new();
        loop {
            let mut head = [0u8; 2];
            self.stream.read_exact(&mut head).expect("a frame's head");
            let mut n = u64::from(head[1] & 0x7f);
            if n == 126 {
                let mut len = [0u8; 2];
                self.stream.read_exact(&mut len).expect("a frame's length");
                n = u64::from(u16::from_be_bytes(len));
            } else if n == 127 {
                let mut len = [0u8; 8];
                self.stream.read_exact(&mut len).expect("a frame's length");
                n = u64::from_be_bytes(len);
            }
            let mut payload = vec![0u8; n as usize];
            self.stream
                .read_exact(&mut payload)
                .expect("a frame's body");
            match head[0] & 0x0f {
                0x9 | 0xa => continue,
                0x8 => panic!("the browser closed the socket"),
                _ => message.extend(payload),
            }
            if head[0] & 0x80 != 0 {
                return serde_json::from_slice(&message).expect("a JSON message");
            }
        }
    }

    /// One call, its result; events read meanwhile are kept.
    fn call(&mut self, method: &str, params: Value) -> Value {
        self.next += 1;
        let id = self.next;
        self.send(&json!({ "id": id, "method": method, "params": params }).to_string());
        loop {
            let reply = self.receive();
            if reply["id"] == id {
                assert!(reply.get("error").is_none(), "{method}: {reply}");
                return reply["result"].clone();
            }
            if reply.get("method").is_some() {
                self.events.push_back(reply);
            }
        }
    }

    /// The next event named `method`, those before it passed over.
    fn event(&mut self, method: &str) -> Value {
        loop {
            let event = match self.events.pop_front() {
                Some(event) => event,
                None => self.receive(),
            };
            if event["method"] == method {
                return event;
            }
        }
    }

    /// An expression's value in the page, a promise's once it settles.
    fn eval(&mut self, expression: &str) -> Value {
        let got = self.call(
            "Runtime.evaluate",
            json!({ "expression": expression, "returnByValue": true, "awaitPromise": true }),
        );
        assert!(
            got.get("exceptionDetails").is_none(),
            "{expression}: {}",
            got["exceptionDetails"]
        );
        got["result"]["value"].clone()
    }

    /// Wait in the page, a frame at a time, until `condition` holds: no
    /// bound but the runner's hang ceiling (C137).
    fn until(&mut self, condition: &str) {
        self.eval(&format!(
            "new Promise((done) => {{ const look = () => {{ let held = false; \
             try {{ held = !!({condition}); }} catch (e) {{}} \
             if (held) done(true); else requestAnimationFrame(look); }}; look(); }})"
        ));
    }

    /// Keys as a user's would: each character a key down carrying its
    /// text and a key up; `\u{8}` the backspace key.
    fn keys(&mut self, text: &str) {
        for c in text.chars() {
            let down = match c {
                '\u{8}' => {
                    json!({ "type": "rawKeyDown", "key": "Backspace", "code": "Backspace", "windowsVirtualKeyCode": 8 })
                }
                c => json!({ "type": "keyDown", "key": c.to_string(), "text": c.to_string() }),
            };
            let key = down["key"].clone();
            self.call("Input.dispatchKeyEvent", down);
            self.call(
                "Input.dispatchKeyEvent",
                json!({ "type": "keyUp", "key": key }),
            );
        }
    }

    /// A click as a user's pointer would: on the middle of the element
    /// `element` evaluates to, scrolled into view first.
    fn click(&mut self, element: &str) {
        let at = self.eval(&format!(
            "(() => {{ const el = {element}; el.scrollIntoView({{ block: 'center' }}); \
             const r = el.getBoundingClientRect(); return [r.x + r.width / 2, r.y + r.height / 2]; }})()"
        ));
        let (x, y) = (at[0].clone(), at[1].clone());
        self.call(
            "Input.dispatchMouseEvent",
            json!({ "type": "mouseMoved", "x": x, "y": y }),
        );
        for kind in ["mousePressed", "mouseReleased"] {
            self.call(
                "Input.dispatchMouseEvent",
                json!({ "type": kind, "x": x, "y": y, "button": "left", "clickCount": 1 }),
            );
        }
    }
}

/// A page the test drives: the browser it runs in, killed when the test
/// ends, and the page's socket, loaded with `url`.
struct Page {
    _browser: harness::Browser,
    cdp: Cdp,
}

impl Page {
    fn open(profile: &std::path::Path, url: &str) -> Page {
        let browser = harness::start_browser(profile);
        let page_ws = {
            let mut root = Cdp::connect(&browser.endpoint);
            let target = root.call("Target.createTarget", json!({ "url": "about:blank" }));
            let id = target["targetId"]
                .as_str()
                .expect("a target id")
                .to_string();
            let (base, _) = browser
                .endpoint
                .split_once("/devtools/browser/")
                .expect("a browser endpoint");
            format!("{base}/devtools/page/{id}")
        };
        let mut cdp = Cdp::connect(&page_ws);
        cdp.call("Page.enable", json!({}));
        cdp.call("Page.navigate", json!({ "url": url }));
        // a load before the navigation's answer is the blank page's
        cdp.events.clear();
        cdp.event("Page.loadEventFired");
        Page {
            _browser: browser,
            cdp,
        }
    }

    /// The number shown beside `form`'s button.
    fn shown(&mut self, form: &str) -> Value {
        self.cdp.eval(&format!(
            "{form}.querySelector('output[data-count=\"matched\"]').value"
        ))
    }

    /// The matched total the server's `count` route answers for `form`
    /// as it stands, its fields as the count's script sends them.
    fn answered(&mut self, web: &harness::WebUi, form: &str) -> String {
        let fields = self.cdp.eval(&format!(
            "new URLSearchParams(new FormData({form})).toString()"
        ));
        let fields = fields.as_str().expect("the form's fields");
        let answer = web.get(&format!("count?{fields}"));
        assert_eq!(answer.status, 200, "count?{fields}: {}", answer.body);
        serde_json::from_str::<Value>(&answer.body).expect("the count's JSON")["total"]["matched"]
            .as_u64()
            .expect("a matched total")
            .to_string()
    }

    /// Wait until `form` shows the total the server answers for it now.
    fn follows(&mut self, web: &harness::WebUi, form: &str) -> String {
        let expected = self.answered(web, form);
        self.cdp.until(&format!(
            "{form}.querySelector('output[data-count=\"matched\"]').value === {}",
            json!(expected)
        ));
        expected
    }
}

const QUERY_BOX: &str = "document.getElementById('query-text').form";
const SEARCH_FORM: &str = "document.getElementById('stat-find').form";

fn profile_of(scratch: &harness::Scratch) -> PathBuf {
    let profile = scratch.0.join("browser-profile");
    std::fs::create_dir_all(&profile).expect("the profile directory");
    profile
}

/// The count's rule (`count.js`): after a form changes, the number beside
/// its button comes to be the one the `count` route answers for the form
/// as it stands, and no other form's number moves. The search form's
/// item box is typed first, for a number of its own; then two queries in
/// the query box, the second in the first's place: their totals differ,
/// the number shown follows each, and the search form's stays its own.
#[test]
fn the_count_beside_the_button_follows_its_own_form() {
    let (scratch, web) = offline_web("browsercount");
    let mut page = Page::open(&profile_of(&scratch), &web.url);

    page.cdp
        .click(&format!("{SEARCH_FORM}.querySelector('input.item-search')"));
    page.cdp.keys("Kaom's Heart");
    page.cdp.click("document.getElementById('query-text')");
    // leaving the item box is a `change`, which asks once more: the
    // number is read once that ask is answered
    let other = page.follows(&web, SEARCH_FORM);
    let mut typed = String::new();
    let mut totals = Vec::new();
    for query in ["class:rings", "ilvl>=84"] {
        let erase: String = std::iter::repeat_n('\u{8}', typed.chars().count()).collect();
        page.cdp.keys(&format!("{erase}{query}"));
        typed = query.to_string();
        let fields = page.cdp.eval(&format!(
            "new URLSearchParams(new FormData({QUERY_BOX})).toString()"
        ));
        assert!(
            fields
                .as_str()
                .is_some_and(|f| f.contains(&harness::form_encoded(query))),
            "{fields}"
        );
        totals.push(page.follows(&web, QUERY_BOX));
        assert_eq!(
            page.shown(SEARCH_FORM),
            json!(other),
            "the search form's number moved when the query box changed"
        );
    }
    eprintln!("the search form {other}; the query box {totals:?}");
    assert_ne!(totals[0], totals[1], "the two queries must count apart");
    assert!(
        !totals.contains(&other),
        "the search form's total {other} must differ from the query box's {totals:?}"
    );
}

/// The picker's rule (`picker.js`): a pick fills the first empty stat
/// row, never a row already written, and announces the write as typing
/// would, so the count follows the pick with no other input. The query
/// box is typed first, for a number of its own that no pick moves. The
/// search form's item box is typed, and its total shown, before the
/// first pick; the pick changes the server's total, and the new one is
/// shown. A second pick takes the second row and leaves the first; with
/// the last rows written by hand, a third pick writes nothing.
#[test]
fn a_pick_fills_the_first_empty_stat_row_and_the_count_follows() {
    let (scratch, web) = offline_web("browserpicker");
    let mut page = Page::open(&profile_of(&scratch), &web.url);
    let row = |n: usize| format!("document.getElementById('stat-{n}').value");

    page.cdp.click("document.getElementById('query-text')");
    page.cdp.keys("rarity=unique");
    page.cdp
        .click(&format!("{SEARCH_FORM}.querySelector('input.item-search')"));
    // leaving a field is a `change`, which asks once more: each number is
    // read once that ask is answered, so no ask is pending at the pick
    // and only the pick's own announcement can move the number after it
    let other = page.follows(&web, QUERY_BOX);
    page.cdp.keys("Two-Stone Ring");
    page.cdp.click("document.getElementById('stat-find')");
    let before = page.follows(&web, SEARCH_FORM);

    page.cdp.keys("maximum Life");
    page.cdp
        .until("document.querySelector('#stat-list button') !== null");
    let pick = "document.querySelector('#stat-list button')";

    page.cdp.click(pick);
    page.cdp.until(&format!("{} !== ''", row(0)));
    let picked = page.cdp.eval(&row(0));
    for n in 1..4 {
        assert_eq!(
            page.cdp.eval(&row(n)),
            json!(""),
            "row {n} after the first pick"
        );
    }
    let after = page.answered(&web, SEARCH_FORM);
    assert_ne!(
        before, after,
        "the pick must change the search form's total"
    );
    // no input but the pick's own: the number follows the announced write
    page.follows(&web, SEARCH_FORM);
    assert_eq!(
        page.shown(QUERY_BOX),
        json!(other),
        "the query box's number moved when the search form changed"
    );
    assert!(
        ![before.as_str(), after.as_str()].contains(&other.as_str()),
        "the query box's total {other} must differ from the search form's {before} and {after}"
    );

    eprintln!("the query box {other}; the search form {before}, then {after} after the pick");

    page.cdp.click(pick);
    page.cdp.until(&format!("{} !== ''", row(1)));
    assert_eq!(
        page.cdp.eval(&row(0)),
        picked,
        "the second pick moved the first row"
    );

    for n in 2..4 {
        page.cdp
            .click(&format!("document.getElementById('stat-{n}')"));
        page.cdp.keys(&format!("written-{n}"));
    }
    let written: Vec<Value> = (0..4).map(|n| page.cdp.eval(&row(n))).collect();
    page.cdp.click(pick);
    page.cdp
        .until("document.getElementById('stat-said').textContent !== ''");
    let now: Vec<Value> = (0..4).map(|n| page.cdp.eval(&row(n))).collect();
    assert_eq!(now, written, "a pick with every row written changed a row");
}

/// The harness refuses a browser that exits before its debugging
/// protocol listens, at once and naming how it exited. `acq` stands in
/// for such a browser: it refuses the browser's flags and exits.
#[test]
fn a_browser_that_exits_before_it_listens_is_refused_at_once() {
    let scratch = harness::scratch("browserexit");
    let profile = profile_of(&scratch);
    let refused = match harness::start_browser_at(harness::acq_exe(), &profile) {
        Ok(_) => panic!("acq was taken for a listening browser"),
        Err(e) => e,
    };
    assert!(
        refused.contains("exited before its debugging protocol listened"),
        "{refused}"
    );
    // `exit status: 2` on Unix, `exit code: 2` on Windows: acq's usage refusal
    assert!(refused.contains(": 2)"), "{refused}");
}

/// The locator fails, never skips: with no browser where it looks, and
/// none named, it refuses naming `ACQ_TEST_BROWSER` and every path it
/// tried; a named one that is no file is refused naming it.
#[test]
fn no_browser_found_is_a_refusal_naming_the_variable_and_each_path() {
    let scratch = harness::scratch("browserlocate");
    let known = vec![scratch.0.join("no-chrome"), scratch.0.join("no-edge.exe")];
    let refused = harness::locate_browser(None, &known).expect_err("nothing to find");
    assert!(refused.contains("ACQ_TEST_BROWSER"), "{refused}");
    for path in &known {
        assert!(refused.contains(&path.display().to_string()), "{refused}");
    }
    let named = scratch.0.join("not-a-browser");
    let refused = harness::locate_browser(Some(named.clone().into()), &known)
        .expect_err("a name that is no file");
    assert!(refused.contains(&named.display().to_string()), "{refused}");

    std::fs::write(&known[1], b"").expect("a stand-in file");
    assert_eq!(harness::locate_browser(None, &known), Ok(known[1].clone()));
}
