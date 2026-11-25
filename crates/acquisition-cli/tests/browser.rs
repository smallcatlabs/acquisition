//! The browser-driven check (C187): `acq web`'s pages loaded in a
//! headless Chromium-family browser the harness starts on a profile under
//! the test's scratch (`harness::start_browser`), spoken to over its
//! debugging protocol by the loopback WebSocket client below, no crate
//! added. It holds the script rules no other test reaches: the count
//! beside a form's button follows that form's fields and no other
//! (`count.js`); the stat pane's rows and groups are added, picked by keys
//! from a finder, changed and removed, and a search of two groups is run
//! from them, the count following each step; a finder's list is busy
//! while an ask of it is in flight, and an ask in flight
//! past an Escape or a pick fills nothing (`lists.js`); a value picked by
//! name from "Add a column", on a place page and in the search form after
//! an unsent edit, lands ordered by it with its head marked; and the table's
//! window shows, wherever the page
//! is scrolled, the server's rows at their places and holds no more than
//! a bound the table's length does not move, takes the server's first
//! block without asking it again, and loads the page again when the
//! basis moves (`window.js`); and a place page's opener, on a page
//! several viewports long, leaves the reader at the opened head with
//! every tick kept; a table head's edge dragged resizes its column in
//! every row (`window.js`); and a site link's league select swaps the link its
//! option holds into the anchor, asking nothing (`link.js`). The tests type, click and scroll as a
//! user's keys and pointer would, and compare what the page shows with
//! what the server answers for the same form or window.
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
use crate::web::{USER, offline_web};

/// One WebSocket to a debugging target: text frames, masked out, as
/// RFC 6455 has a client send them, and the events read while waiting
/// for a call's reply kept for a later wait. Every request the page sends
/// is noted as it is read, by its URL, where `Network` is enabled.
struct Cdp {
    stream: TcpStream,
    next: u64,
    events: VecDeque<Value>,
    requests: Vec<String>,
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
            requests: Vec::new(),
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

    /// The next message, a request the page sends noted by its URL.
    fn receive(&mut self) -> Value {
        let message = self.frame();
        if message["method"] == "Network.requestWillBeSent"
            && let Some(url) = message["params"]["request"]["url"].as_str()
        {
            self.requests.push(url.to_string());
        }
        message
    }

    fn frame(&mut self) -> Value {
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
        let reply = self.reply(method, params);
        assert!(reply.get("error").is_none(), "{method}: {reply}");
        reply["result"].clone()
    }

    /// One call, its whole reply, an error among them; events read
    /// meanwhile are kept.
    fn reply(&mut self, method: &str, params: Value) -> Value {
        self.next += 1;
        let id = self.next;
        self.send(&json!({ "id": id, "method": method, "params": params }).to_string());
        loop {
            let reply = self.receive();
            if reply["id"] == id {
                return reply;
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
    /// text and a key up; `\u{8}` the backspace key, `\r` the Enter key.
    fn keys(&mut self, text: &str) {
        for c in text.chars() {
            let down = match c {
                '\u{8}' => {
                    json!({ "type": "rawKeyDown", "key": "Backspace", "code": "Backspace", "windowsVirtualKeyCode": 8 })
                }
                '\r' => {
                    json!({ "type": "keyDown", "key": "Enter", "code": "Enter", "windowsVirtualKeyCode": 13, "text": "\r" })
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

    /// A key that writes no text, as a user's would: `ArrowDown`,
    /// `ArrowUp`, `Escape` or `Tab`.
    fn press(&mut self, key: &str) {
        let code = match key {
            "ArrowDown" => 40,
            "ArrowUp" => 38,
            "Escape" => 27,
            "Tab" => 9,
            other => panic!("no key {other}"),
        };
        for kind in ["rawKeyDown", "keyUp"] {
            self.call(
                "Input.dispatchKeyEvent",
                json!({ "type": kind, "key": key, "code": key, "windowsVirtualKeyCode": code }),
            );
        }
    }

    /// A letter typed at a focused select, as a user's key would: the key
    /// down, its character, the key up; the select takes the first option
    /// its text begins with (its type-ahead, which a blur starts again).
    fn type_ahead(&mut self, letter: char) {
        let key = letter.to_string();
        let code = format!("Key{}", letter.to_ascii_uppercase());
        let vk = u32::from(letter.to_ascii_uppercase());
        self.call(
            "Input.dispatchKeyEvent",
            json!({ "type": "rawKeyDown", "key": key, "code": code, "windowsVirtualKeyCode": vk }),
        );
        self.call(
            "Input.dispatchKeyEvent",
            json!({ "type": "char", "key": key, "text": key }),
        );
        self.call(
            "Input.dispatchKeyEvent",
            json!({ "type": "keyUp", "key": key, "code": code, "windowsVirtualKeyCode": vk }),
        );
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
    /// The loader of the document the page holds.
    loader: Value,
}

impl Page {
    fn open(profile: &std::path::Path, url: &str) -> Page {
        let mut page = Page::blank(profile);
        page.navigate(url);
        page
    }

    /// A blank page, its lifecycle's events enabled, nothing loaded yet.
    fn blank(profile: &std::path::Path) -> Page {
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
        cdp.call("Page.setLifecycleEventsEnabled", json!({ "enabled": true }));
        Page {
            _browser: browser,
            cdp,
            loader: Value::Null,
        }
    }

    /// Load `url` and wait for its document's load.
    fn navigate(&mut self, url: &str) {
        let navigated = self.cdp.call("Page.navigate", json!({ "url": url }));
        let loader = navigated["loaderId"].clone();
        assert!(
            loader.is_string(),
            "the navigation names no loader: {navigated}"
        );
        // the load of this navigation's document, by its loader, whichever
        // side of the navigation's answer it was read on: a load of the
        // blank page carries another loader
        loop {
            let event = self.cdp.event("Page.lifecycleEvent");
            if event["params"]["name"] == "load" && event["params"]["loaderId"] == loader {
                break;
            }
        }
        self.loader = loader;
    }

    /// Wait for the load of a document the page loads itself, by a loader
    /// other than the one it holds, and hold that one.
    fn loaded_again(&mut self) {
        loop {
            let event = self.cdp.event("Page.lifecycleEvent");
            if event["params"]["name"] == "load" && event["params"]["loaderId"] != self.loader {
                self.loader = event["params"]["loaderId"].clone();
                return;
            }
        }
    }

    /// The number shown beside `form`'s button: its count's output among
    /// its elements, as `count.js` finds it (an output another form owns
    /// may stand inside it).
    fn shown(&mut self, form: &str) -> Value {
        self.cdp.eval(&format!(
            "Array.from({form}.elements).find((el) => el.matches('output[data-count=\"matched\"]')).value"
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

    /// Wait until the count's script asks the `count` route for `form`'s
    /// fields as they stand, among the requests noted since they were
    /// last cleared, then until `form` shows the server's total for them:
    /// an operation that changes the fields is seen to ask, whatever the
    /// total it leads to. `Network` must be enabled.
    fn asks(&mut self, web: &harness::WebUi, form: &str) -> String {
        let fields = self.cdp.eval(&format!(
            "new URLSearchParams(new FormData({form})).toString()"
        ));
        let wanted = format!("/count?{}", fields.as_str().expect("the form's fields"));
        while !self.cdp.requests.iter().any(|url| url.ends_with(&wanted)) {
            self.cdp.event("Network.requestWillBeSent");
        }
        self.follows(web, form)
    }

    /// The clauses not run shown beside `form`'s button: its count's
    /// `unrun` output's number, or null where its part is hidden.
    fn unrun(&mut self, form: &str) -> Value {
        self.cdp.eval(&format!(
            "(() => {{ const o = Array.from({form}.elements).find((el) => el.matches('output[data-count=\"unrun\"]')); \
             return o.parentElement.hidden ? null : o.value; }})()"
        ))
    }

    /// The clauses of `form`'s search as it stands that would not run,
    /// as the server's `count` route answers them (its remainder, none
    /// where the search is no site's), checked against what `form` shows
    /// once it shows the server's total: the number where there are any,
    /// the part hidden where there are none.
    fn marks(&mut self, web: &harness::WebUi, form: &str) -> usize {
        self.follows(web, form);
        let fields = self.cdp.eval(&format!(
            "new URLSearchParams(new FormData({form})).toString()"
        ));
        let fields = fields.as_str().expect("the form's fields");
        let answer = web.get_json(&format!("count?{fields}"));
        let left = answer["query"]["trade"]["remainder"]
            .as_array()
            .map_or(0, Vec::len);
        let shown = match left {
            0 => Value::Null,
            n => json!(n.to_string()),
        };
        assert_eq!(self.unrun(form), shown, "count?{fields}");
        left
    }

    /// Wait until `form` shows the total the server answers for it now.
    fn follows(&mut self, web: &harness::WebUi, form: &str) -> String {
        let expected = self.answered(web, form);
        self.cdp.until(&format!(
            "Array.from({form}.elements).find((el) => el.matches('output[data-count=\"matched\"]')).value === {}",
            json!(expected)
        ));
        expected
    }
}

const QUERY_BOX: &str = "document.getElementById('query-text').form";
const SEARCH_FORM: &str = "document.querySelector('input.item-search').form";
/// The site box's form, whose controls stand in the search form's bar
/// (`form=`).
const TRADE_BOX: &str = "document.getElementById('trade-text').form";

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
/// Then the site box, in the search form's bar: neither form's count
/// follows the other's edits, and the site search, run, fills the form.
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
    // a keystroke in a finder, a box no form sends, asks no count: the
    // number shown stays, never the dash a change puts in its place
    let finder = format!("{SEARCH_FORM}.querySelector('[data-finder]')");
    page.cdp.click(&format!("{finder}.querySelector('input')"));
    page.cdp.keys("li");
    assert_eq!(
        page.shown(SEARCH_FORM),
        json!(other),
        "a keystroke in a finder asked a count"
    );
    // the finder's answer, landing, scrolls the page to its first option:
    // the next click waits for it, or it is aimed before the scroll and
    // lands after it
    page.cdp.until(&at_rest(&finder, "li"));
    eprintln!("the search form {other}; the query box {totals:?}");
    assert_ne!(totals[0], totals[1], "the two queries must count apart");
    assert!(
        !totals.contains(&other),
        "the search form's total {other} must differ from the query box's {totals:?}"
    );
    // the site box, in the search form's bar and its own form's
    // (`4-full-1-form-rides.md`, Decision 5): the search form's edits
    // above asked nothing of it, and a site search typed there moves its
    // count and not the search form's; run, it fills the search form
    assert_eq!(
        page.shown(TRADE_BOX),
        json!("\u{2013}"),
        "the search form's edits moved the site box's count"
    );
    page.cdp.click("document.getElementById('trade-text')");
    let rings = r#"{"query": {"filters": {"type_filters": {"filters": {"category": {"option": "accessory.ring"}}}}}}"#;
    page.cdp.eval(&format!(
        "document.getElementById('trade-text').value = {}; true",
        json!(rings)
    ));
    page.cdp.keys(" ");
    let site = page.follows(&web, TRADE_BOX);
    assert_eq!(
        page.shown(SEARCH_FORM),
        json!(other),
        "the search form's number moved when the site box changed"
    );
    assert_ne!(
        site, other,
        "the site search must count apart from the form"
    );
    // its button, as its other controls, owned by its form through `form=`
    page.cdp.click(&format!(
        "Array.from({TRADE_BOX}.elements).find((el) => el.type === 'submit')"
    ));
    page.loaded_again();
    assert_eq!(
        page.cdp.eval(&format!(
            "{SEARCH_FORM}.elements['query.filters.type_filters.filters.category.option'].value"
        )),
        json!("accessory.ring"),
        "the site search ran did not fill the form"
    );
}

/// A number counted without part of the search never stands alone. The
/// count's rule (`count.js`) for the clauses not run: beside a form whose
/// search is the site's, the number of the search's clauses that would
/// not run (the `count` route's remainder) shows where it is not zero and
/// is hidden where it is, as the form's fields change. A line that runs
/// shows none; a weighted sum of a pseudo total with a minimum, which the
/// translation cannot run, shows its clauses; the group removed, none
/// again. The site box's count marks its own search, and the query box,
/// a query in the language, has no such part.
#[test]
fn the_count_beside_search_says_how_many_clauses_would_not_run() {
    let (scratch, web) = offline_web("browserunrun");
    let mut page = Page::open(&profile_of(&scratch), &web.url);
    let first = group(0);
    let second = group(1);
    let field = |of: &str, name: &str| format!("{of}.querySelector('[name=\"{name}\"]')");
    assert_eq!(
        page.cdp.eval(&format!(
            "Array.from({QUERY_BOX}.elements).some((el) => el.matches('output[data-count=\"unrun\"]'))"
        )),
        json!(false),
        "the query box has a part for site clauses"
    );
    assert_eq!(page.unrun(SEARCH_FORM), Value::Null, "the empty form");
    page.cdp.call("Network.enable", json!({}));

    // a line that runs: no clause left out, no mark
    page.cdp.requests.clear();
    pick(
        &mut page,
        &web,
        &first,
        "to maximum life",
        "explicit.stat_3299347043",
    );
    page.asks(&web, SEARCH_FORM);
    assert_eq!(page.marks(&web, SEARCH_FORM), 0);

    // a weighted sum of a pseudo total, its weight and its minimum: each
    // change's mark is the server's remainder, and the last is not none
    page.cdp.requests.clear();
    page.cdp
        .click("document.querySelector('.add-group > summary')");
    page.cdp.click(
        "Array.from(document.querySelectorAll('.add-group button')).find((b) => b.textContent === 'Weighted Sum')",
    );
    page.asks(&web, SEARCH_FORM);
    page.marks(&web, SEARCH_FORM);
    page.cdp.requests.clear();
    pick(
        &mut page,
        &web,
        &second,
        "maximum life",
        "pseudo.pseudo_total_life",
    );
    page.asks(&web, SEARCH_FORM);
    page.marks(&web, SEARCH_FORM);
    for (box_, typed) in [
        (field(&row(&second, 0), "stat.weight"), "1"),
        (field(&second, "group.min"), "100"),
    ] {
        page.cdp.requests.clear();
        page.cdp.click(&box_);
        page.cdp.keys(typed);
        page.asks(&web, SEARCH_FORM);
        page.marks(&web, SEARCH_FORM);
    }
    let left = page.marks(&web, SEARCH_FORM);
    assert!(left > 0, "the weighted pseudo group runs whole here");

    // the site box: its own search's clauses, the search form's untouched
    page.cdp.click("document.getElementById('trade-text')");
    let weighted = r#"{"query": {"stats": [{"type": "weight", "filters": [{"id": "pseudo.pseudo_total_life", "value": {"weight": 1}}], "value": {"min": 100}}]}}"#;
    page.cdp.eval(&format!(
        "document.getElementById('trade-text').value = {}; true",
        json!(weighted)
    ));
    page.cdp.keys(" ");
    page.follows(&web, TRADE_BOX);
    assert!(
        page.marks(&web, TRADE_BOX) > 0,
        "the site box's search runs whole"
    );
    assert_eq!(page.unrun(SEARCH_FORM), json!(left.to_string()));

    // the group removed: the search runs whole, and the mark goes
    page.cdp.requests.clear();
    page.cdp
        .click(&format!("{second}.querySelector('.filter [data-remove]')"));
    assert_eq!(
        page.cdp
            .eval("document.querySelectorAll('.stat-group').length"),
        json!(1)
    );
    page.asks(&web, SEARCH_FORM);
    assert_eq!(page.marks(&web, SEARCH_FORM), 0);
}

/// What the switch hides (`4-full-1-form-rides.md`, Decisions 3 and 4),
/// the stylesheet's alone: unticked, a control not run and empty, a whole
/// panel of them, and a type not run in a group's type select and in "Add
/// Stat Group" are not displayed; ticked, all are, the tick asking no
/// count; an empty control not run given a value while ticked stays
/// displayed once unticked; adding and removing a group still asks the
/// form's count. A page filled with a value in a control not run shows
/// it; one in a listing field shows no value there (#317: a listing
/// control showed a value the form would not send), so the field and its
/// panel stay hidden, the value named above the form.
#[test]
fn the_switch_shows_what_a_search_does_not_run_and_a_held_value_always() {
    let (scratch, web) = offline_web("browserswitch");
    let mut page = Page::open(&profile_of(&scratch), &web.url);
    let named = |name: &str| format!("document.querySelector('[name=\"{name}\"]')");
    let panel = |title: &str| {
        format!(
            "Array.from(document.querySelectorAll('details.panel')).find((d) => \
             d.querySelector('summary').textContent === '{title}')"
        )
    };
    let damage = named("query.filters.weapon_filters.filters.damage.min");
    let damage_control = format!("{damage}.closest('.filter')");
    let status = format!("{}.closest('.filter')", named("query.status.option"));
    let account = format!(
        "{}.closest('.filter')",
        named("query.filters.trade_filters.filters.account.option")
    );
    let trade = panel("Trade Filters");
    let ultimatum = panel("Ultimatum Filters");
    let crucible_type = format!(
        "{}.querySelector('option[value=\"crucible\"]')",
        named("group.type")
    );
    let crucible_add = "document.querySelector('button[data-add=\"stat-group-crucible\"]')";
    let switch = "document.getElementById('all-fields')";
    let unrun = [
        damage_control.as_str(),
        status.as_str(),
        trade.as_str(),
        ultimatum.as_str(),
        crucible_type.as_str(),
        crucible_add,
    ];
    for element in unrun {
        assert!(!shows(&mut page, element), "{element} shown unticked");
    }
    // a number beside Search, so a change counted would put the dash back
    page.cdp
        .click(&format!("{SEARCH_FORM}.querySelector('input.item-search')"));
    page.cdp.keys("Kaom's Heart");
    let number = page.follows(&web, SEARCH_FORM);
    page.cdp.click(switch);
    assert_eq!(
        page.shown(SEARCH_FORM),
        json!(number),
        "the tick asked a count"
    );
    for element in unrun {
        assert!(shows(&mut page, element), "{element} hidden ticked");
    }
    // a value typed, ticked, into an empty control not run, which stays
    // displayed once unticked; the rest hide again
    page.cdp.click(&format!(
        "{}.querySelector('summary')",
        panel("Weapon Filters")
    ));
    page.cdp.click(&damage);
    page.cdp.keys("10");
    page.follows(&web, SEARCH_FORM);
    page.cdp.click(switch);
    assert!(shows(&mut page, &damage_control), "a typed value hidden");
    for element in &unrun[1..] {
        assert!(!shows(&mut page, element), "{element} shown unticked");
    }
    // a group added and removed, unticked, each asking the form's count
    page.cdp.call("Network.enable", json!({}));
    page.cdp.requests.clear();
    page.cdp
        .click("document.querySelector('.add-group > summary')");
    page.cdp.click(
        "Array.from(document.querySelectorAll('.add-group button')).find((b) => b.textContent === 'And')",
    );
    page.asks(&web, SEARCH_FORM);
    page.cdp.requests.clear();
    page.cdp.click(
        "Array.from(document.querySelectorAll('.stat-group')).pop().querySelector('[data-remove]')",
    );
    page.asks(&web, SEARCH_FORM);
    // a page filled with a value in a control not run and in a listing
    // field: the first shown; the listing shows none, so it and its panel
    // stay hidden, the value named above the form; an empty panel not run
    // still hidden
    page.navigate(&format!(
        "{}search?realm=pc&query.filters.weapon_filters.filters.damage.min=10\
         &query.filters.trade_filters.filters.account.option=someone#answer",
        web.url
    ));
    assert_eq!(page.cdp.eval(&format!("{damage}.value")), json!("10"));
    assert!(
        shows(&mut page, &damage_control),
        "{damage_control} holding a value hidden"
    );
    assert_eq!(
        page.cdp
            .eval(&format!("{account}.querySelector('input').value")),
        json!("")
    );
    for element in [
        account.as_str(),
        trade.as_str(),
        status.as_str(),
        ultimatum.as_str(),
    ] {
        assert!(!shows(&mut page, element), "{element} shown empty");
    }
    assert_eq!(
        page.cdp.eval(
            "document.querySelector('main').textContent.includes('query.filters.trade_filters.filters.account.option')"
        ),
        json!(true)
    );
}

/// The one page in a browser (`4-full-1-form-design.md` §2): an answer
/// stands under its search form filled with the search it answered; a
/// bound edited there and the form sent by its own button lands at the
/// answer, the answer is the edit's, and the form stands filled with the
/// edit.
#[test]
fn an_answers_form_edited_and_sent_asks_the_edit_and_stands_refilled() {
    let (scratch, web) = offline_web("browseredit");
    let search = |bound: &str| {
        format!(
            "search?realm=pc&league=Standard\
             &query.filters.type_filters.filters.category.option=accessory.ring\
             &group.type=and&stat.id=explicit.stat_3299347043&stat.min={bound}&view.limit=20"
        )
    };
    let total = |path: &str| {
        web.get_json(path)["total"]["matched"]
            .as_u64()
            .expect("a matched total")
    };
    let (before, after) = (total(&search("70")), total(&search("90")));
    assert_ne!(before, after, "the two bounds must answer apart");
    let mut page = Page::open(
        &profile_of(&scratch),
        &format!("{}{}", web.url, search("70")),
    );
    let bound = format!("{SEARCH_FORM}.querySelector('input[name=\"stat.min\"]')");
    assert_eq!(page.cdp.eval(&format!("{bound}.value")), json!("70"));
    page.cdp.click(&bound);
    page.cdp.eval(&format!("{bound}.select(); true"));
    page.cdp.keys("90");
    page.cdp
        .click(&format!("{SEARCH_FORM}.querySelector('button.search-btn')"));
    page.loaded_again();
    assert_eq!(page.cdp.eval("location.hash"), json!("#answer"));
    assert_eq!(page.cdp.eval(&format!("{bound}.value")), json!("90"));
    let counted = match after {
        1 => "1 match".to_string(),
        n => format!("{n} matches"),
    };
    assert_eq!(
        page.cdp
            .eval("document.querySelector('#answer p.count').textContent"),
        json!(counted)
    );
}

/// The site link's league (`link.js`; C195, a link may carry a league a
/// person names for it alone): the anchor holds the link of the league
/// its select shows, at load and after the select is changed, and the
/// change asks the server nothing and leaves the page where it is.
#[test]
fn the_site_links_league_select_swaps_the_link_and_asks_nothing() {
    use crate::search_json::TRADE_ID;
    let (scratch, web) = offline_web("browserlink");
    let url = format!("https://www.pathofexile.com/trade/search/Standard/{TRADE_ID}");
    let mut page = Page::open(
        &profile_of(&scratch),
        &format!(
            "{}search?realm=pc&trade={}",
            web.url,
            url.replace(':', "%3A").replace('/', "%2F")
        ),
    );
    const SELECT: &str = "document.querySelector('select[data-link-league]')";
    const LINK: &str = "document.querySelector('a[data-link]')";
    let shown = |page: &mut Page| {
        (
            page.cdp.eval(&format!("{LINK}.getAttribute('href')")),
            page.cdp.eval(&format!("{SELECT}.value")),
        )
    };
    let (href, value) = shown(&mut page);
    assert_eq!(href, value);
    assert!(
        href.as_str()
            .is_some_and(|h| h.starts_with("https://www.pathofexile.com/trade/search/Standard/")),
        "{href}"
    );
    let href_at_load = page.cdp.eval("location.href");
    page.cdp.call("Network.enable", json!({}));
    page.cdp.requests.clear();
    // the link stands in the answer's details, below the rows: opened by
    // its summary, then the select reached by the pointer and a key
    page.cdp
        .click("document.querySelector('details.about > summary')");
    page.cdp.click(SELECT);
    page.cdp.press("Escape");
    page.cdp.type_ahead('H');
    page.cdp.until(&format!(
        "{LINK}.getAttribute('href') === {SELECT}.value && {SELECT}.value.includes('/Hardcore/')"
    ));
    // the table's window may fill as the page scrolls; nothing else is asked
    assert!(
        page.cdp.requests.iter().all(|url| url.contains("/rows?")),
        "{:?}",
        page.cdp.requests
    );
    assert_eq!(page.cdp.eval("location.href"), json!(href_at_load));
}

/// The table's columns (`window.js`; stop 6 of the owner's sitting): an
/// answer opens as the table, and a head's right edge, dragged by the
/// pointer, widens its column by the drag in the head and in every row
/// alike; the drag follows no link and loads nothing, and the widths are
/// the page's alone, gone at a reload.
#[test]
fn a_heads_edge_dragged_resizes_its_column_in_every_row() {
    let (scratch, web) = offline_web("browserresize");
    let mut page = Page::open(
        &profile_of(&scratch),
        &format!("{}search?realm=pc&text=class%3Arings#answer", web.url),
    );
    page.cdp.until("document.querySelector('.window .seg .r[data-id]') && document.querySelector('.window .r.head .edge')");
    let width = |page: &mut Page, row: &str| {
        page.cdp
            .eval(&format!(
                "document.querySelector('.window {row}').children[1].getBoundingClientRect().width"
            ))
            .as_f64()
            .expect("a width")
    };
    let (head, row) = (width(&mut page, ".r.head"), width(&mut page, ".r[data-id]"));
    assert!((head - row).abs() < 0.5, "{head} {row}");
    let at = page.cdp.eval(
        "(() => { const r = document.querySelector('.window .r.head').children[1].querySelector('.edge').getBoundingClientRect(); \
         return [r.x + r.width / 2, r.y + r.height / 2]; })()",
    );
    let (x, y) = (at[0].as_f64().unwrap(), at[1].as_f64().unwrap());
    let href = page.cdp.eval("location.href");
    page.cdp.call(
        "Input.dispatchMouseEvent",
        json!({ "type": "mouseMoved", "x": x, "y": y }),
    );
    page.cdp.call(
        "Input.dispatchMouseEvent",
        json!({ "type": "mousePressed", "x": x, "y": y, "button": "left", "clickCount": 1 }),
    );
    for step in 1..=10 {
        page.cdp.call(
            "Input.dispatchMouseEvent",
            json!({ "type": "mouseMoved", "x": x + f64::from(step) * 10.0, "y": y, "button": "left", "buttons": 1 }),
        );
    }
    page.cdp.call(
        "Input.dispatchMouseEvent",
        json!({ "type": "mouseReleased", "x": x + 100.0, "y": y, "button": "left", "clickCount": 1 }),
    );
    let (wider, row) = (width(&mut page, ".r.head"), width(&mut page, ".r[data-id]"));
    assert!((wider - head - 100.0).abs() < 2.0, "{head} → {wider}");
    assert!((wider - row).abs() < 0.5, "{wider} {row}");
    assert_eq!(page.cdp.eval("location.href"), href);
    // the page's own: a reload draws the columns as the server's
    page.cdp.eval("location.reload(); true");
    page.loaded_again();
    page.cdp
        .until("document.querySelector('.window .seg .r[data-id]')");
    page.cdp
        .until("document.querySelector('.window .r.head .edge')");
    assert!((width(&mut page, ".r.head") - head).abs() < 0.5);
}

/// The n-th stat group of the search form, as the page holds it.
fn group(n: usize) -> String {
    format!("document.querySelectorAll('.stat-group')[{n}]")
}

/// The m-th row of a group.
fn row(group: &str, m: usize) -> String {
    format!("{group}.querySelectorAll('.stat-row')[{m}]")
}

/// Whether an element shows: no rule of the stylesheet hides it.
fn shows(page: &mut Page, element: &str) -> bool {
    page.cdp
        .eval(&format!("getComputedStyle({element}).display !== 'none'"))
        .as_bool()
        .expect("a display")
}

/// The ids a finder's options carry, each its row's `stat.id`, joined.
fn listed_ids(finder: &str) -> String {
    format!(
        "Array.from({finder}.querySelectorAll('[role=option] > template'), \
         (t) => t.content.firstElementChild.querySelector('input').value).join(',')"
    )
}

/// The finder `finder` at rest on `words`: its box holds them and its
/// list is not busy, so no answer is on its way to it (`lists.js`). The
/// wait every pick makes before it reads the list or moves the mark: a
/// list that is busy may show an earlier ask's options, the very ones the
/// last ask will answer, and that answer, landing after an arrow, puts
/// the mark back on its first.
fn at_rest(finder: &str, words: &str) -> String {
    format!(
        "{finder}.querySelector('input').value === {} \
         && {finder}.querySelector('[role=listbox]').getAttribute('aria-busy') === null",
        json!(words)
    )
}

/// Whether the finder's list is busy: an ask of it is in flight.
fn busy(finder: &str) -> String {
    format!("{finder}.querySelector('[role=listbox]').getAttribute('aria-busy') === 'true'")
}

/// A stat picked into `group` from its finder as a user's keys would: the
/// finder's box clicked and `words` typed; once the finder is at rest its
/// list is the one the `stats` route answers for those words, the arrows
/// move the mark from the first option to the stat `id`'s, and Enter
/// picks it.
fn pick(page: &mut Page, web: &harness::WebUi, group: &str, words: &str, id: &str) {
    let finder = format!("{group}.querySelector('[data-finder]')");
    page.cdp.click(&format!("{finder}.querySelector('input')"));
    page.cdp.keys(words);
    let answered = web.get_json(&format!("stats?q={}", harness::form_encoded(words)));
    let ids: Vec<&str> = answered["entries"]
        .as_array()
        .expect("the entries")
        .iter()
        .map(|e| e["id"].as_str().expect("an id"))
        .collect();
    page.cdp.until(&at_rest(&finder, words));
    assert_eq!(page.cdp.eval(&listed_ids(&finder)), json!(ids.join(",")));
    let at = ids
        .iter()
        .position(|listed| *listed == id)
        .unwrap_or_else(|| panic!("{id} is not listed for {words:?}: {ids:?}"));
    assert_eq!(
        page.cdp.eval(&format!(
            "{finder}.querySelector('[aria-selected=true]') === {finder}.querySelector('[role=option]')"
        )),
        json!(true),
        "the first option is not marked"
    );
    for _ in 0..at {
        page.cdp.press("ArrowDown");
    }
    assert_eq!(
        page.cdp.eval(&format!(
            "{finder}.querySelector('[aria-selected=true] > template').content.firstElementChild.querySelector('input').value"
        )),
        json!(id)
    );
    page.cdp.keys("\r");
}

/// Rows and groups (`lists.js`, `4-full-1-form-rows.md`), by keys and
/// pointer, the count following each step with no other input. On an
/// empty form a stat is picked from the first group's finder with the
/// arrows and Enter, the form not submitted, and its min typed where the
/// focus stands; "Add Stat Group" adds a weighted sum, whose bound and
/// weight boxes show while the `and` group's do not; two stats are picked
/// into it with their weights and its min, a weighted sum the translation
/// runs. The first group's type is changed to a weighted sum and back, its
/// weight box showing then hiding. Search answers the server's total for
/// the same fields, which differs from the first group's alone; its report
/// drops neither group, and the form stands filled with both, their rows,
/// weights and bound. Then a row and a group are removed.
#[test]
fn two_stat_groups_one_weighted_are_made_edited_and_run_from_the_page() {
    let (scratch, web) = offline_web("browserrows");
    let mut page = Page::open(&profile_of(&scratch), &web.url);
    let first = group(0);
    let second = group(1);
    let life = "explicit.stat_3299347043";
    let implicit = "implicit.stat_3299347043";
    let field = |of: &str, name: &str| format!("{of}.querySelector('[name=\"{name}\"]')");
    page.cdp.eval(&format!(
        "window.submitted = false; {SEARCH_FORM}.addEventListener('submit', () => window.submitted = true); true"
    ));
    assert_eq!(
        page.cdp
            .eval("document.querySelectorAll('.stat-group').length"),
        json!(1)
    );
    assert_eq!(
        page.cdp
            .eval(&format!("{first}.querySelectorAll('.stat-row').length")),
        json!(0),
        "the empty form's group holds a row"
    );

    // each add, pick and remove below asks the count itself, seen by the
    // count's request for the fields it leaves (`Page::asks`)
    page.cdp.call("Network.enable", json!({}));

    // a pick by arrows and Enter, which submits nothing; the min typed
    // where the focus stands
    page.cdp.requests.clear();
    pick(&mut page, &web, &first, "to maximum life", life);
    page.asks(&web, SEARCH_FORM);
    assert_eq!(page.cdp.eval("window.submitted"), json!(false));
    let row0 = row(&first, 0);
    assert_eq!(
        page.cdp.eval(&format!("{}.value", field(&row0, "stat.id"))),
        json!(life)
    );
    page.cdp.until(&format!(
        "document.activeElement === {}",
        field(&row0, "stat.min")
    ));
    let finder = format!("{first}.querySelector('[data-finder]')");
    assert_eq!(
        page.cdp.eval(&format!(
            "[{finder}.querySelector('input').value, {finder}.querySelector('[role=listbox]').children.length]"
        )),
        json!(["", 0]),
        "the finder is not emptied by its pick"
    );
    page.cdp.keys("70");
    assert_eq!(
        page.cdp
            .eval(&format!("{}.value", field(&row0, "stat.min"))),
        json!("70")
    );
    assert!(!shows(&mut page, &field(&row0, "stat.weight")));
    assert!(!shows(&mut page, &field(&first, "group.min")));
    let alone = page.follows(&web, SEARCH_FORM);

    // a weighted sum added from the menu: its bound shows, the `and`
    // group's does not
    page.cdp.requests.clear();
    page.cdp
        .click("document.querySelector('.add-group > summary')");
    page.cdp.click(
        "Array.from(document.querySelectorAll('.add-group button')).find((b) => b.textContent === 'Weighted Sum')",
    );
    assert_eq!(
        page.cdp.eval("document.querySelector('.add-group').open"),
        json!(false)
    );
    assert_eq!(
        page.cdp
            .eval("document.querySelectorAll('.stat-group').length"),
        json!(2)
    );
    assert_eq!(
        page.cdp
            .eval(&format!("{}.value", field(&second, "group.type"))),
        json!("weight")
    );
    page.cdp.until(&format!(
        "document.activeElement === {second}.querySelector('[data-finder] input')"
    ));
    assert!(shows(&mut page, &field(&second, "group.min")));
    assert!(!shows(&mut page, &field(&first, "group.min")));
    page.asks(&web, SEARCH_FORM);

    // two stats picked into it, each with its weight, and its min
    for (m, id) in [life, implicit].into_iter().enumerate() {
        page.cdp.requests.clear();
        pick(&mut page, &web, &second, "to maximum life", id);
        let added = row(&second, m);
        page.cdp.until(&format!(
            "document.activeElement === {}",
            field(&added, "stat.min")
        ));
        page.asks(&web, SEARCH_FORM);
        assert!(shows(&mut page, &field(&added, "stat.weight")));
        page.cdp.click(&field(&added, "stat.weight"));
        page.cdp.keys("1");
        page.follows(&web, SEARCH_FORM);
    }
    page.cdp.click(&field(&second, "group.min"));
    page.cdp.keys("100");
    assert!(!shows(&mut page, &field(&row0, "stat.weight")));
    let both = page.follows(&web, SEARCH_FORM);
    assert_ne!(both, alone, "the weighted group must change the total");

    // the first group's type changed to a weighted sum and back, by the
    // keys at its select, which the pointer reaches from its title
    let title = format!("{first}.querySelector('.filter-title')");
    let kind = format!("{}.value", field(&first, "group.type"));
    for (letter, to, weighted) in [('W', "weight", true), ('A', "and", false)] {
        page.cdp.click(&title);
        page.cdp.press("Tab");
        page.cdp.type_ahead(letter);
        page.cdp.until(&format!("{kind} === {}", json!(to)));
        assert_eq!(
            shows(&mut page, &field(&row0, "stat.weight")),
            weighted,
            "{to}"
        );
        assert_eq!(
            shows(&mut page, &field(&first, "group.min")),
            weighted,
            "{to}"
        );
        // a weight left half typed while it shows, which the next type
        // hides: a field the type does not take never blocks the search
        if weighted {
            page.cdp.click(&field(&row0, "stat.weight"));
            page.cdp.keys("1e");
        }
        page.follows(&web, SEARCH_FORM);
    }

    // Search: the server's total for the same fields, the report dropping
    // neither group, the form refilled with both
    let fields = page.cdp.eval(&format!(
        "new URLSearchParams(new FormData({SEARCH_FORM})).toString()"
    ));
    let fields = fields.as_str().expect("the form's fields").to_string();
    let answer = web.get_json(&format!("search?{fields}"));
    let total = answer["total"]["matched"]
        .as_u64()
        .expect("a matched total");
    assert_eq!(total.to_string(), both);
    // the search's own request is held while its submit is read: the
    // flag is the document's that the search replaces, so read once the
    // answer has come it is the new page's, which has none
    page.cdp.call(
        "Fetch.enable",
        json!({ "patterns": [{ "urlPattern": "*/search?*", "requestStage": "Request" }] }),
    );
    page.cdp
        .click(&format!("{SEARCH_FORM}.querySelector('button.search-btn')"));
    assert_eq!(
        page.cdp.eval("window.submitted"),
        json!(true),
        "the browser held the search back"
    );
    let held = page.cdp.event("Fetch.requestPaused");
    page.cdp.call(
        "Fetch.continueRequest",
        json!({ "requestId": held["params"]["requestId"] }),
    );
    page.cdp.call("Fetch.disable", json!({}));
    page.loaded_again();
    let counted = match total {
        1 => "1 match".to_string(),
        n => format!("{n} matches"),
    };
    assert_eq!(
        page.cdp
            .eval("document.querySelector('#answer p.count').textContent"),
        json!(counted)
    );
    let report = page
        .cdp
        .eval("document.getElementById('answer').textContent");
    let report = report.as_str().expect("the answer's text");
    assert!(!report.contains("NOT RUN"), "{report}");
    assert_eq!(
        page.cdp.eval("document.querySelector('.left-out')"),
        Value::Null
    );
    let filled = page.cdp.eval(
        "Array.from(document.querySelectorAll('.stat-group'), (g) => ({ \
           type: g.querySelector('[name=\"group.type\"]').value, \
           min: g.querySelector('[name=\"group.min\"]').value, \
           rows: Array.from(g.querySelectorAll('.stat-row'), (r) => [ \
             r.querySelector('[name=\"stat.id\"]').value, \
             r.querySelector('[name=\"stat.min\"]').value, \
             r.querySelector('[name=\"stat.weight\"]').value]) }))",
    );
    assert_eq!(
        filled,
        json!([
            { "type": "and", "min": "", "rows": [[life, "70", ""]] },
            { "type": "weight", "min": "100", "rows": [[life, "", "1"], [implicit, "", "1"]] },
        ])
    );
    // the refilled form's fields are the ones searched
    assert_eq!(
        page.cdp.eval(&format!(
            "new URLSearchParams(new FormData({SEARCH_FORM})).toString()"
        )),
        json!(fields)
    );

    // a row removed, then a group: each asks the count
    page.cdp.requests.clear();
    page.cdp.click(&format!(
        "{}.querySelector('[data-remove]')",
        row(&second, 1)
    ));
    assert_eq!(
        page.cdp
            .eval(&format!("{second}.querySelectorAll('.stat-row').length")),
        json!(1)
    );
    let fewer = page.asks(&web, SEARCH_FORM);
    page.cdp.requests.clear();
    page.cdp
        .click(&format!("{first}.querySelector('.filter [data-remove]')"));
    assert_eq!(
        page.cdp
            .eval("document.querySelectorAll('.stat-group').length"),
        json!(1)
    );
    assert_eq!(
        page.cdp
            .eval(&format!("{}.value", field(&first, "group.type"))),
        json!("weight")
    );
    let last = page.asks(&web, SEARCH_FORM);
    eprintln!("first group alone {alone}; both {both}; a row less {fewer}; a group less {last}");
}

/// A finder's ask in flight fills nothing once the finder is emptied
/// (`lists.js`): an ask held by the debugging protocol's `Fetch` domain
/// and released after Escape, and one held and released after a pick
/// from the list already shown, leave the box and the list empty once
/// the page's own later ask of the same route is answered.
#[test]
fn a_finders_ask_held_past_an_escape_or_a_pick_fills_nothing() {
    let (scratch, web) = offline_web("browserfinder");
    let mut page = Page::open(&profile_of(&scratch), &web.url);
    let first = group(0);
    let finder = format!("{first}.querySelector('[data-finder]')");
    let emptied = format!(
        "[{finder}.querySelector('input').value, {finder}.querySelector('[role=listbox]').children.length]"
    );
    let hold = |page: &mut Page| {
        page.cdp.call(
            "Fetch.enable",
            json!({ "patterns": [{ "urlPattern": "*/stats?q=*", "requestStage": "Request" }] }),
        );
    };
    // released: the held ask goes on, or is refused as one the page
    // dropped; then the page's own ask of the route, unheld, is answered,
    // the held one's reply before it
    let release = |page: &mut Page, held: &Value| {
        page.cdp.reply(
            "Fetch.continueRequest",
            json!({ "requestId": held["params"]["requestId"] }),
        );
        page.cdp.call("Fetch.disable", json!({}));
        page.cdp
            .eval("fetch('stats?q=life').then((r) => r.text()).then(() => true)");
    };

    // Escape over an ask in flight
    hold(&mut page);
    page.cdp.click(&format!("{finder}.querySelector('input')"));
    page.cdp.keys("li");
    let held = page.cdp.event("Fetch.requestPaused");
    assert_eq!(
        page.cdp.eval(&busy(&finder)),
        json!(true),
        "a list whose ask is in flight is not busy"
    );
    page.cdp.press("Escape");
    assert_eq!(page.cdp.eval(&emptied), json!(["", 0]));
    assert_eq!(
        page.cdp.eval(&busy(&finder)),
        json!(false),
        "an emptied finder's list is busy"
    );
    release(&mut page, &held);
    assert_eq!(
        page.cdp.eval(&emptied),
        json!(["", 0]),
        "a reply after Escape filled the finder"
    );

    // a pick from the list shown, with a later ask in flight
    page.cdp.keys("life");
    page.cdp.until(&at_rest(&finder, "life"));
    assert_eq!(
        page.cdp.eval(&format!(
            "{finder}.querySelector('[aria-selected=true]') !== null && {}",
            listed_ids(&finder)
        )),
        json!(
            web.get_json("stats?q=life")["entries"]
                .as_array()
                .expect("the entries")
                .iter()
                .map(|e| e["id"].as_str().expect("an id"))
                .collect::<Vec<_>>()
                .join(",")
        )
    );
    hold(&mut page);
    page.cdp.keys(" max");
    let held = page.cdp.event("Fetch.requestPaused");
    assert_eq!(
        page.cdp.eval(&busy(&finder)),
        json!(true),
        "a list shown with a later ask in flight is not busy"
    );
    page.cdp.keys("\r");
    assert_eq!(
        page.cdp
            .eval(&format!("{first}.querySelectorAll('.stat-row').length")),
        json!(1),
        "Enter picked nothing"
    );
    assert_eq!(page.cdp.eval(&emptied), json!(["", 0]));
    release(&mut page, &held);
    assert_eq!(
        page.cdp.eval(&emptied),
        json!(["", 0]),
        "a reply after a pick filled the finder"
    );
}

/// The values a value finder's options send, joined.
fn listed_values(finder: &str) -> String {
    format!("Array.from({finder}.querySelectorAll('[role=option]'), (o) => o.value).join('\\n')")
}

/// A value picked from the value finder `finder` ("Add a column") as a
/// user's keys would: its box clicked and `words` typed; once the finder
/// is at rest its list is the one the `stats` route answers asked for
/// values, the arrows move the mark from the first option to `value`'s,
/// and Enter picks it, which sends the finder's form: the page loads
/// again.
fn pick_value(page: &mut Page, web: &harness::WebUi, finder: &str, words: &str, value: &str) {
    page.cdp.click(&format!("{finder}.querySelector('input')"));
    page.cdp.keys(words);
    let answered = web.get_json(&format!(
        "stats?values=true&q={}",
        harness::form_encoded(words)
    ));
    let values: Vec<&str> = answered["entries"]
        .as_array()
        .expect("the entries")
        .iter()
        .map(|e| e["value"].as_str().expect("a value"))
        .collect();
    page.cdp.until(&at_rest(finder, words));
    assert_eq!(
        page.cdp.eval(&listed_values(finder)),
        json!(values.join("\n"))
    );
    let at = values
        .iter()
        .position(|listed| *listed == value)
        .unwrap_or_else(|| panic!("{value} is not listed for {words:?}: {values:?}"));
    for _ in 0..at {
        page.cdp.press("ArrowDown");
    }
    assert_eq!(
        page.cdp.eval(&format!(
            "{finder}.querySelector('[aria-selected=true]').value"
        )),
        json!(value)
    );
    page.cdp.keys("\r");
    page.loaded_again();
}

/// The fields of the URL the page holds, in order.
const LOCATION_FIELDS: &str = "Array.from(new URLSearchParams(location.search))";

/// The value finder on a place page (`4-full-1-form-values.md`): a tab
/// opened with no query, "Add a column" focused, a value's name typed,
/// the arrows and Enter: the page lands at the URL its head's link would
/// write — the page's fields, the value added to the list and ordered by
/// it, largest first — its column added and its head marked, the rows in
/// that order.
#[test]
fn a_value_picked_by_name_orders_a_place_page() {
    let (scratch, web) = offline_web("browserplacevalue");
    let life = "sum(line(\"# to maximum Life\" source=explicit).arg1)";
    let mut page = Page::open(
        &profile_of(&scratch),
        &format!("{}tabs?realm=pc&league=Standard&open=t1", web.url),
    );
    let finder = "document.querySelector('.value-finder')";
    assert_eq!(
        page.cdp.eval(&format!(
            "{finder}.querySelector('input').getAttribute('aria-label')"
        )),
        json!("Add a column")
    );
    pick_value(&mut page, &web, finder, "to maximum life", life);
    assert_eq!(
        page.cdp.eval(LOCATION_FIELDS),
        json!([
            ["realm", "pc"],
            ["league", "Standard"],
            ["view.value", life],
            ["view.sort", life],
            ["view.desc", "true"],
            ["open", "t1"]
        ])
    );
    let head =
        "Array.from(document.querySelectorAll('a.head')).find((a) => a.querySelector('.mark'))";
    assert_eq!(
        page.cdp.eval(&format!(
            "{head}.textContent.replace('▼', '').replace(', ordered largest first', '')"
        )),
        json!(life)
    );
    page.cdp.until(
        "Array.from(document.querySelectorAll('[data-id]'), (r) => r.dataset.id).join(',') === 'kaom,r1,r2'",
    );
}

/// The value finder in the search form: an edit not yet sent, then a
/// value picked by name, lands with the form filled as it stood, the edit
/// among it, ordered by the value with its column; the ordered head's own
/// link then flips the order.
#[test]
fn a_value_picked_from_the_form_keeps_its_unsent_edit_and_its_head_flips() {
    let (scratch, web) = offline_web("browserformvalue");
    let life = "sum(line(\"# to maximum Life\" source=explicit).arg1)";
    let mut page = Page::open(
        &profile_of(&scratch),
        &format!(
            "{}search?realm=pc&league=Standard&group.type=and\
             &stat.id=explicit.stat_3299347043&stat.min=90&view.fields=icon",
            web.url
        ),
    );
    let bound = format!("{SEARCH_FORM}.querySelector('input[name=\"stat.min\"]')");
    page.cdp.click(&bound);
    page.cdp.eval(&format!("{bound}.select(); true"));
    page.cdp.keys("70");
    let finder = format!("{SEARCH_FORM}.querySelector('.value-finder')");
    pick_value(&mut page, &web, &finder, "to maximum life", life);
    assert_eq!(page.cdp.eval(&format!("{bound}.value")), json!("70"));
    let fields = page.cdp.eval(LOCATION_FIELDS);
    let has = |k: &str, v: &str| {
        fields
            .as_array()
            .expect("the fields")
            .contains(&json!([k, v]))
    };
    assert!(
        has("stat.min", "70") && has("view.sort", life) && has("view.desc", "true"),
        "{fields}"
    );
    assert!(!has("view.by", life), "{fields}");
    let ids = "Array.from(document.querySelectorAll('[data-id]'), (r) => r.dataset.id).join(',')";
    page.cdp.until(&format!("{ids} === 'kaom,r1,r2'"));
    // the ordered head's own link: the other way
    page.cdp.click(
        "Array.from(document.querySelectorAll('a.head')).find((a) => a.querySelector('.mark'))",
    );
    page.loaded_again();
    let fields = page.cdp.eval(LOCATION_FIELDS);
    assert!(
        fields
            .as_array()
            .expect("the fields")
            .contains(&json!(["view.sort", life]))
            && !fields
                .as_array()
                .expect("the fields")
                .contains(&json!(["view.desc", "true"])),
        "{fields}"
    );
    // the head's link carries no anchor: the table is scrolled to, as a
    // reader would, for the window to draw its rows
    page.cdp
        .eval("document.querySelector('.window').scrollIntoView(); true");
    page.cdp.until(&format!("{ids} === 'r2,r1,kaom'"));
    assert_eq!(page.cdp.eval(&format!("{bound}.value")), json!("70"));
}

/// What the table's window shows now, as drawn: the table's row height
/// and block's rows, the viewport's height, the segment's count and its
/// route, the rows whose index the viewport covers (each its index, its
/// id and where it is drawn from the segment's top) or null where one of
/// them is not drawn yet, and every row in the page by its index and id.
const LOOK: &str = "(() => { \
    const table = document.querySelector('.window[data-row-height]'); \
    const seg = table.querySelector('.seg'); \
    const h = Number(table.dataset.rowHeight); \
    const top = seg.getBoundingClientRect().top; \
    const vh = window.innerHeight; \
    const count = Number(seg.dataset.count); \
    const drawn = Array.from(seg.querySelectorAll('.r[data-i]'), (r) => \
      [Number(r.dataset.i), r.dataset.id, r.getBoundingClientRect().top - top]); \
    const at = new Map(drawn.map((row) => [row[0], row])); \
    const first = Math.max(0, Math.floor(-top / h)); \
    const last = Math.min(count, Math.ceil((vh - top) / h)) - 1; \
    const covered = []; \
    for (let i = first; i <= last; i++) { if (!at.has(i)) return null; covered.push(at.get(i)); } \
    return { h, block: Number(table.dataset.blockRows), vh, count, route: seg.dataset.route, \
             top: top + window.scrollY, covered, drawn }; })()";

/// The table's top in the page, where its start is scrolled to.
const TABLE_TOP: &str = "(document.querySelector('.window[data-row-height]').getBoundingClientRect().top + window.scrollY)";

/// Settled: two animation frames running in which no block of the table
/// waits for its rows, a block being asked in the frame after the last
/// block in view comes. A count of the script's own frames, no duration.
const SETTLED: &str = "new Promise((done) => { let quiet = 0; \
    const look = () => { \
      const empty = Array.from(document.querySelectorAll('.window .seg .block')) \
        .some((b) => b.firstElementChild === null); \
      quiet = empty ? 0 : quiet + 1; \
      if (quiet >= 2) done(true); else requestAnimationFrame(look); }; \
    look(); })";

/// The window's rule (`window.js`, the table's design): wherever the page
/// is scrolled — its start, its middle, its end and its start again — the
/// rows the viewport covers are, id for id and in order, the rows the
/// `rows` route answers for those offsets, each id once in the page, each
/// drawn at its index times the row height; and the page holds at most
/// the rows of the blocks three viewports span and one more, a bound the
/// table's length does not move. The table is several times longer than
/// that bound: a tab of 1,200 items recorded here, beside the seeded
/// world. The server's first block is never asked while the page holds
/// it; and once the world's basis moves, the next window asked loads the
/// page again, at the new basis.
#[test]
fn the_window_shows_the_servers_rows_in_place_and_follows_a_moved_basis() {
    let (scratch, web) = offline_web("browsertable");
    let mut store = acquisition_store::Store::open(&acquisition_store::account_path(
        &scratch.0.join("mock"),
        USER,
    ))
    .expect("the seeded store");
    let mut record = |ep: acquisition_store::Endpoint, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "pc", "league": "Standard" }),
                200,
                &body,
                at,
            )
            .expect("a recorded fetch");
    };
    let stash = |id: &str| acquisition_store::Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: None,
    };
    const ROWS: usize = 1200;
    let items: Vec<Value> = (0..ROWS)
        .map(|n| {
            json!({ "id": format!("b{n:04}"), "name": format!("Bulk {n:04}"), "typeLine": "Iron Ring",
                    "baseType": "Iron Ring", "rarity": "Rare", "frameTypeId": 2, "identified": true,
                    "ilvl": 60, "x": n % 24, "y": n / 24 })
        })
        .collect();
    record(
        acquisition_store::Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "stashes": [
            { "id": "t1", "name": "Rings", "type": "PremiumStash" },
            { "id": "t2", "name": "Launch", "type": "PremiumStash" },
            { "id": "b1", "name": "Bulk", "type": "PremiumStash" } ] }),
        60,
    );
    record(
        stash("b1"),
        json!({ "stash": { "id": "b1", "name": "Bulk", "type": "PremiumStash", "items": items } }),
        61,
    );
    let path = format!(
        "search?realm=pc&text={}&view.fields=icon",
        harness::form_encoded("name:bulk")
    );

    let mut page = Page::blank(&profile_of(&scratch));
    page.cdp.call("Network.enable", json!({}));
    page.cdp.call(
        "Emulation.setDeviceMetricsOverride",
        json!({ "width": 1000, "height": 600, "deviceScaleFactor": 1, "mobile": false }),
    );
    // landing at the answer, as every form that submits a search lands
    page.navigate(&format!("{}{path}#answer", web.url));
    let basis_before = page
        .cdp
        .eval("document.querySelector('meta[name=\"basis\"]').content");

    let whole = |page: &mut Page, scroll: &str, at: &str| -> Value {
        page.cdp
            .eval(&format!("window.scrollTo(0, {scroll}); true"));
        page.cdp.until(&format!("{LOOK} !== null"));
        let look = page.cdp.eval(LOOK);
        let (h, block, vh) = (
            look["h"].as_f64().unwrap(),
            look["block"].as_u64().unwrap() as usize,
            look["vh"].as_f64().unwrap(),
        );
        assert_eq!(look["count"], ROWS, "{at}: the segment's count");
        let covered = look["covered"].as_array().unwrap();
        assert!(!covered.is_empty(), "{at}: the viewport covers no row");
        let first = covered[0][0].as_u64().unwrap() as usize;
        let route = look["route"]
            .as_str()
            .unwrap()
            .trim_start_matches(&format!("/{}/", web.token))
            .to_string();
        let answered = web.get_json(&format!("{route}&offset={first}&count={}", covered.len()));
        let ids: Vec<&Value> = answered["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| &r["id"])
            .collect();
        let shown: Vec<&Value> = covered.iter().map(|r| &r[1]).collect();
        assert_eq!(shown, ids, "{at}: the rows the viewport covers");
        let drawn = look["drawn"].as_array().unwrap();
        let mut seen = std::collections::HashSet::new();
        for row in drawn {
            assert!(seen.insert(row[1].clone()), "{at}: {} drawn twice", row[1]);
            let (i, y) = (row[0].as_f64().unwrap(), row[2].as_f64().unwrap());
            assert!(
                (y - i * h).abs() <= 0.5,
                "{at}: row {i} drawn at {y}, not {}",
                i * h
            );
        }
        // three viewports of rows, and the row a part-shown row adds at
        // each edge, touch at most this many blocks with the one their
        // first row's place in its block adds
        let span = (3.0 * vh / h).ceil() as usize + 1;
        let bound = (span.div_ceil(block) + 1) * block;
        assert!(
            bound * 3 <= ROWS,
            "the table must be several times the bound {bound}"
        );
        assert!(
            drawn.len() <= bound,
            "{at}: {} rows in the page, past the bound {bound}",
            drawn.len()
        );
        // and once the window has settled, every block it asked for this
        // place held, none still on its way across two frames running
        page.cdp.eval(SETTLED);
        let settled = page
            .cdp
            .eval("document.querySelectorAll('.window .seg .r[data-i]').length");
        let settled = settled.as_u64().unwrap() as usize;
        assert!(
            settled <= bound,
            "{at}, settled: {settled} rows in the page, past the bound {bound}"
        );
        eprintln!(
            "{at}: rows {first}+{}, {} in the page, {settled} settled, bound {bound}",
            covered.len(),
            drawn.len()
        );
        look
    };

    // the start: the server's block shown and held; scrolled within it
    // until the next block is near, the window asks that block, never the
    // first, whose ask, had the script made one, it would have made first
    // the table stands under the page's editors: its start is its own top
    let start = whole(&mut page, TABLE_TOP, "the start");
    let block = start["block"].as_u64().unwrap();
    let near = format!("{} + {} * {}", start["top"], block / 2, start["h"]);
    whole(&mut page, &near, "the start, the next block near");
    page.cdp.until(&format!(
        "document.querySelector('.r[data-i=\"{block}\"]') !== null"
    ));
    let windows = |page: &Page| -> Vec<String> {
        page.cdp
            .requests
            .iter()
            .filter(|url| url.contains("/rows?"))
            .cloned()
            .collect()
    };
    let asked = windows(&page);
    assert!(
        asked
            .iter()
            .any(|url| url.contains(&format!("&offset={block}&"))),
        "the next block was never asked: {asked:?}"
    );
    assert!(
        !asked.iter().any(|url| url.contains("&offset=0&")),
        "the server's block was asked while the page held it: {asked:?}"
    );
    let middle = format!(
        "{} + {} * {} / 2 - innerHeight / 2",
        start["top"], ROWS, start["h"]
    );
    whole(&mut page, &middle, "the middle");
    // on through the middle half a block at a time, as a wheel scrolls:
    // the blocks behind are dropped as the ones ahead come, so the bound
    // holds at every step, not only after a jump
    for step in 1..=6 {
        whole(
            &mut page,
            &format!("{middle} + {step} * {} * {} / 2", block, start["h"]),
            &format!("the middle, scrolled on {step} half blocks"),
        );
    }
    let end = format!("{} + {} * {} - innerHeight", start["top"], ROWS, start["h"]);
    whole(&mut page, &end, "the end");
    whole(&mut page, TABLE_TOP, "the start again");

    // a window whose fetch fails is dropped and asked again at the next
    // scroll, never left pending: the next block's fetch is failed as a
    // reset connection would fail it, and a scroll of a pixel asks it
    // again
    let failed = format!("*offset={block}&*");
    page.cdp.call(
        "Fetch.enable",
        json!({ "patterns": [{ "urlPattern": failed }] }),
    );
    page.cdp.eval(&format!("window.scrollTo(0, {near}); true"));
    let paused = page.cdp.event("Fetch.requestPaused");
    page.cdp.call(
        "Fetch.failRequest",
        json!({ "requestId": paused["params"]["requestId"], "errorReason": "ConnectionReset" }),
    );
    page.cdp.call("Fetch.disable", json!({}));
    // the failure seen before the scroll that retries it: the failed
    // block's element gone, its place empty (a scroll read while the
    // block still waited would ask nothing, and the failure, coming after
    // it, would queue no fill of its own)
    let failed_top = format!("{}px", block as f64 * start["h"].as_f64().unwrap());
    page.cdp.until(&format!(
        "!Array.from(document.querySelectorAll('.window .seg .block')).some((b) => b.style.top === {})",
        json!(failed_top)
    ));
    let asks = |page: &Page| {
        windows(page)
            .iter()
            .filter(|url| url.contains(&format!("&offset={block}&")))
            .count()
    };
    let before = asks(&page);
    page.cdp
        .eval(&format!("window.scrollTo(0, {near} + 1); true"));
    page.cdp.until(&format!(
        "document.querySelector('.r[data-i=\"{block}\"]') !== null"
    ));
    assert!(asks(&page) > before, "the failed block was not asked again");
    whole(
        &mut page,
        &format!("{near} + 1"),
        "the start, the failed block asked again",
    );

    // the basis moves: a later fetch of a tab; the next window asked is
    // refused, and the page loads itself again, at the new basis
    record(
        stash("t2"),
        json!({ "stash": { "id": "t2", "name": "Launch", "type": "PremiumStash", "items": [] } }),
        70,
    );
    page.cdp
        .eval(&format!("window.scrollTo(0, {middle}); true"));
    page.loaded_again();
    let basis_after = page
        .cdp
        .eval("document.querySelector('meta[name=\"basis\"]').content");
    assert_ne!(
        basis_before, basis_after,
        "the page loaded again at the basis it had"
    );
    let now = web.get_json(&path)["basis"].clone();
    let shown: Value = serde_json::from_str(basis_after.as_str().unwrap()).unwrap();
    assert_eq!(
        shown["answer"], now,
        "the page loaded again, not at the basis now"
    );
    whole(&mut page, TABLE_TOP, "the start, at the new basis");
}

/// The place pages' opener keeps the reader's place: on a stash tabs page
/// several viewports long, a box ticked near the top, then a far map tab's
/// opener clicked, the page that loads stands at that tab's head, in the
/// viewport, its pages listed under it, and the box ticked before is still
/// ticked: the tick travelled through the open in the URL, and the answer's
/// anchor placed the reader with no script.
#[test]
fn opening_a_far_parent_keeps_its_head_in_view_and_every_tick() {
    let (scratch, web) = offline_web("browserplaces");
    let mut store = acquisition_store::Store::open(&acquisition_store::account_path(
        &scratch.0.join("mock"),
        USER,
    ))
    .expect("the seeded store");
    let mut listed: Vec<Value> = (0..80)
        .map(|n| json!({ "id": format!("t{n:02}"), "name": format!("Tab {n:02}"), "type": "PremiumStash" }))
        .collect();
    listed.push(json!({ "id": "far", "name": "Far maps", "type": "MapStash" }));
    listed.extend(
        (80..90)
            .map(|n| json!({ "id": format!("t{n:02}"), "name": format!("Tab {n:02}"), "type": "PremiumStash" })),
    );
    store
        .record(
            &acquisition_store::Endpoint::Stashes {
                realm: "pc".into(),
                league: "Standard".into(),
            },
            &json!({ "realm": "pc", "league": "Standard" }),
            200,
            &json!({ "stashes": listed }),
            70,
        )
        .expect("the listing");
    store
        .record(
            &acquisition_store::Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "far".into(),
                sub: None,
            },
            &json!({ "realm": "pc", "league": "Standard" }),
            200,
            &json!({ "stash": { "id": "far", "name": "Far maps", "type": "MapStash", "items": [], "children": [
                { "id": "fp1", "name": "Tier 1", "type": "MapStash", "parent": "far" },
                { "id": "fp2", "name": "Tier 2", "type": "MapStash", "parent": "far" } ] } }),
            71,
        )
        .expect("the far tab's fetch");
    drop(store);

    let mut page = Page::blank(&profile_of(&scratch));
    page.cdp.call(
        "Emulation.setDeviceMetricsOverride",
        json!({ "width": 1000, "height": 600, "deviceScaleFactor": 1, "mobile": false }),
    );
    page.navigate(&format!("{}tabs?realm=pc&league=Standard", web.url));
    let long = page
        .cdp
        .eval("document.documentElement.scrollHeight > 3 * window.innerHeight");
    assert_eq!(long, json!(true), "the page must be several viewports long");
    let tick = "document.querySelector('input[name=\"tab\"][value=\"t03\"]')";
    page.cdp.click(tick);
    assert_eq!(page.cdp.eval(&format!("{tick}.checked")), json!(true));
    page.cdp
        .click("document.querySelector('button[name=\"toggle\"][value=\"far\"]')");
    page.loaded_again();
    page.cdp.until("document.getElementById('p-fp2') !== null");
    let look = page.cdp.eval(
        "(() => { const head = document.getElementById('p-far').getBoundingClientRect(); \
         const pages = ['p-fp1', 'p-fp2'].map((id) => document.getElementById(id).getBoundingClientRect().top); \
         return { hash: location.hash, top: head.top, bottom: head.bottom, vh: window.innerHeight, \
                  scrolled: window.scrollY, pages, \
                  ticked: document.querySelector('input[name=\"tab\"][value=\"t03\"]').checked }; })()",
    );
    assert_eq!(look["hash"], "#p-far", "{look}");
    let (top, bottom, vh) = (
        look["top"].as_f64().unwrap(),
        look["bottom"].as_f64().unwrap(),
        look["vh"].as_f64().unwrap(),
    );
    assert!(
        top >= 0.0 && bottom <= vh,
        "the head is out of view: {look}"
    );
    assert!(
        look["scrolled"].as_f64().unwrap() > vh,
        "the page is at its top: {look}"
    );
    for page_top in look["pages"].as_array().unwrap() {
        assert!(
            page_top.as_f64().unwrap() > top,
            "a page above its tab: {look}"
        );
    }
    assert_eq!(look["ticked"], json!(true), "the tick was lost: {look}");
}

/// What one segment of a place page's window shows now, as
/// [`LOOK`] reads the table's one: the segment by its place's id.
fn look_at(place: &str) -> String {
    format!(
        "(() => {{ \
        const table = document.querySelector('.window[data-row-height]'); \
        let seg = document.getElementById('p-{place}').nextElementSibling; \
        while (seg && seg.tagName === 'NOSCRIPT') seg = seg.nextElementSibling; \
        if (!seg || !seg.classList.contains('seg')) return 'no segment'; \
        const h = Number(table.dataset.rowHeight); \
        const top = seg.getBoundingClientRect().top; \
        const vh = window.innerHeight; \
        const count = Number(seg.dataset.count); \
        const drawn = Array.from(seg.querySelectorAll('.r[data-i]'), (r) => \
          [Number(r.dataset.i), r.dataset.id, r.getBoundingClientRect().top - top]); \
        const at = new Map(drawn.map((row) => [row[0], row])); \
        const first = Math.max(0, Math.floor(-top / h)); \
        const last = Math.min(count, Math.ceil((vh - top) / h)) - 1; \
        const covered = []; \
        for (let i = first; i <= last; i++) {{ if (!at.has(i)) return null; covered.push(at.get(i)); }} \
        return {{ h, count, route: seg.dataset.route, top: top + window.scrollY, covered }}; }})()"
    )
}

/// The grouped table's window: on a stash tabs page, a query typed in its box
/// and sent by Enter lands on the place page, a tick and an open kept, never
/// the plan. Then, with a tab of several blocks open and a second tab's opener
/// clicked, the page that loads stands at the second's head, in view, though
/// the first's segment above it was sized after the anchor landed; and wherever
/// either segment is scrolled into view, the rows the viewport covers under its
/// head are, id for id, its route's JSON at those offsets, each drawn at its
/// index times the row height.
#[test]
fn a_grouped_page_shows_each_open_places_rows_and_keeps_its_place() {
    let (scratch, web) = offline_web("browsergrouped");
    let mut store = acquisition_store::Store::open(&acquisition_store::account_path(
        &scratch.0.join("mock"),
        USER,
    ))
    .expect("the seeded store");
    let mut record = |ep: acquisition_store::Endpoint, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "pc", "league": "Standard" }),
                200,
                &body,
                at,
            )
            .expect("a recorded fetch");
    };
    let small = |n: usize| json!({ "id": format!("t{n:02}"), "name": format!("Tab {n:02}"), "type": "PremiumStash" });
    let mut listed: Vec<Value> = (0..4).map(small).collect();
    listed.push(json!({ "id": "big1", "name": "Bulk one", "type": "PremiumStash" }));
    listed.extend((4..8).map(small));
    listed.push(json!({ "id": "big2", "name": "Bulk two", "type": "PremiumStash" }));
    listed.extend((8..12).map(small));
    record(
        acquisition_store::Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "stashes": listed }),
        60,
    );
    for (id, name, rows) in [("big1", "Bulk one", 330), ("big2", "Bulk two", 140)] {
        let items: Vec<Value> = (0..rows)
            .map(|n| {
                json!({ "id": format!("{id}-{n:04}"), "name": format!("Bulk {n:04}"), "typeLine": "Iron Ring",
                        "baseType": "Iron Ring", "rarity": "Rare", "frameTypeId": 2, "identified": true,
                        "ilvl": 60, "x": n % 24, "y": n / 24 })
            })
            .collect();
        record(
            acquisition_store::Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: id.into(),
                sub: None,
            },
            json!({ "stash": { "id": id, "name": name, "type": "PremiumStash", "items": items } }),
            61,
        );
    }
    drop(store);

    let mut page = Page::blank(&profile_of(&scratch));
    page.cdp.call(
        "Emulation.setDeviceMetricsOverride",
        json!({ "width": 1000, "height": 600, "deviceScaleFactor": 1, "mobile": false }),
    );
    // a query sent by Enter: the place page, the tick and the open kept
    page.navigate(&format!(
        "{}tabs?realm=pc&league=Standard&open=big1",
        web.url
    ));
    page.cdp
        .click("document.querySelector('input[name=\"tab\"][value=\"t01\"]')");
    page.cdp
        .click("document.querySelector('input[name=\"text\"]')");
    page.cdp.keys("name:bulk\r");
    page.loaded_again();
    let landed = page.cdp.eval(
        "({ path: location.pathname, fields: location.search, \
            matched: document.querySelector('p.matched') !== null })",
    );
    let fields = landed["fields"].as_str().unwrap_or_default().to_string();
    assert!(
        landed["path"]
            .as_str()
            .is_some_and(|p| p.ends_with("/tabs")),
        "{landed}"
    );
    for field in ["text=name%3Abulk", "open=big1", "tab=t01"] {
        assert!(fields.contains(field), "{field}: {landed}");
    }
    assert_eq!(landed["matched"], json!(true), "{landed}");

    // the second tab opened: its head in view, the first's segment above
    // it sized
    page.cdp
        .click("document.querySelector('button[name=\"toggle\"][value=\"big2\"]')");
    page.loaded_again();
    page.cdp
        .until("document.querySelector('.window[data-windowed]') !== null");
    let head = page.cdp.eval(
        "(() => { const head = document.getElementById('p-big2').getBoundingClientRect(); \
         return { hash: location.hash, top: head.top, bottom: head.bottom, vh: window.innerHeight, \
                  scrolled: window.scrollY }; })()",
    );
    assert_eq!(head["hash"], "#p-big2", "{head}");
    let (top, bottom, vh) = (
        head["top"].as_f64().unwrap(),
        head["bottom"].as_f64().unwrap(),
        head["vh"].as_f64().unwrap(),
    );
    assert!(
        top >= 0.0 && bottom <= vh,
        "the head is out of view: {head}"
    );
    assert!(
        head["scrolled"].as_f64().unwrap() > 330.0 * 24.0 / 2.0,
        "the first segment is not above the head: {head}"
    );

    // each segment's rows, wherever it is scrolled into view
    let seen = |page: &mut Page, place: &str, rows: usize, at: &str, scroll: &str| {
        page.cdp.eval(&format!(
            "(() => {{ let seg = document.getElementById('p-{place}').nextElementSibling; \
             while (seg.tagName === 'NOSCRIPT') seg = seg.nextElementSibling; \
             window.scrollTo(0, seg.getBoundingClientRect().top + window.scrollY + {scroll}); return true; }})()"
        ));
        page.cdp.until(&format!("{} !== null", look_at(place)));
        let look = page.cdp.eval(&look_at(place));
        assert_eq!(look["count"], rows, "{place} {at}: {look}");
        let covered = look["covered"].as_array().unwrap();
        assert!(!covered.is_empty(), "{place} {at}: no row in view");
        let first = covered[0][0].as_u64().unwrap();
        let route = look["route"]
            .as_str()
            .unwrap()
            .trim_start_matches(&format!("/{}/", web.token))
            .to_string();
        let answered = web.get_json(&format!("{route}&offset={first}&count={}", covered.len()));
        let ids: Vec<&Value> = answered["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| &r["id"])
            .collect();
        let shown: Vec<&Value> = covered.iter().map(|r| &r[1]).collect();
        assert_eq!(shown, ids, "{place} {at}: the rows the viewport covers");
        let h = look["h"].as_f64().unwrap();
        for row in covered {
            let (i, y) = (row[0].as_f64().unwrap(), row[2].as_f64().unwrap());
            assert!((y - i * h).abs() <= 0.5, "{place} {at}: row {i} at {y}");
        }
    };
    for (place, rows) in [("big1", 330), ("big2", 140)] {
        let h = 24;
        seen(&mut page, place, rows, "its start", "-100");
        seen(
            &mut page,
            place,
            rows,
            "its middle",
            &format!("{} - 300", rows * h / 2),
        );
        seen(
            &mut page,
            place,
            rows,
            "its end",
            &format!("{} - 500", rows * h),
        );
    }
}

/// An item opened under its row keeps the reader's place: on a stash tabs page
/// whose open tab is several blocks long, a box ticked near the top, then a far
/// row the script fetched opened by its name, the page that loads stands at
/// that row, in view, its card under it, the tick kept, and the rows under the
/// card are, id for id, the segment's route's JSON at those offsets. Closed by
/// the card's Close, the row is in view again, no card under it, the tick kept.
#[test]
fn opening_a_far_row_shows_its_card_under_it_and_keeps_every_tick() {
    let (scratch, web) = offline_web("browserrow");
    let mut store = acquisition_store::Store::open(&acquisition_store::account_path(
        &scratch.0.join("mock"),
        USER,
    ))
    .expect("the seeded store");
    let mut record = |ep: acquisition_store::Endpoint, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "pc", "league": "Standard" }),
                200,
                &body,
                at,
            )
            .expect("a recorded fetch");
    };
    let mut listed: Vec<Value> = (0..4)
        .map(|n| json!({ "id": format!("t{n:02}"), "name": format!("Tab {n:02}"), "type": "PremiumStash" }))
        .collect();
    listed.push(json!({ "id": "big", "name": "Bulk", "type": "PremiumStash" }));
    record(
        acquisition_store::Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "stashes": listed }),
        60,
    );
    let items: Vec<Value> = (0..330)
        .map(|n| {
            json!({ "id": format!("big-{n:04}"), "name": format!("Bulk {n:04}"), "typeLine": "Iron Ring",
                    "baseType": "Iron Ring", "rarity": "Rare", "frameTypeId": 2, "identified": true,
                    "ilvl": 60, "x": n % 24, "y": n / 24 })
        })
        .collect();
    record(
        acquisition_store::Endpoint::Stash {
            realm: "pc".into(),
            league: "Standard".into(),
            id: "big".into(),
            sub: None,
        },
        json!({ "stash": { "id": "big", "name": "Bulk", "type": "PremiumStash", "items": items } }),
        61,
    );
    drop(store);

    let mut page = Page::blank(&profile_of(&scratch));
    page.cdp.call(
        "Emulation.setDeviceMetricsOverride",
        json!({ "width": 1000, "height": 600, "deviceScaleFactor": 1, "mobile": false }),
    );
    page.navigate(&format!(
        "{}tabs?realm=pc&league=Standard&open=big",
        web.url
    ));
    page.cdp
        .until("document.querySelector('.window[data-windowed]') !== null");
    let tick = "document.querySelector('input[name=\"tab\"][value=\"t01\"]')";
    page.cdp.click(tick);
    assert_eq!(page.cdp.eval(&format!("{tick}.checked")), json!(true));
    // a far row, in a block the script fetched, opened by its name
    let row = "big-0200";
    page.cdp.eval(
        "(() => { const seg = document.querySelector('.seg'); \
         window.scrollTo(0, seg.getBoundingClientRect().top + window.scrollY + 200 * 24 - 200); return true; })()",
    );
    let name =
        format!("document.querySelector('.r button[name=\"toggle_item\"][value=\"{row}\"]')");
    page.cdp.until(&format!("{name} !== null"));
    page.cdp.click(&name);
    page.loaded_again();
    page.cdp
        .until("document.querySelector('.window[data-windowed]') !== null");
    let look = |page: &mut Page| {
        page.cdp.eval(&format!(
            "(() => {{ const row = document.getElementById('i-{row}'); \
             if (!row) return {{ hash: location.hash, row: null }}; \
             const box = row.getBoundingClientRect(); \
             const seg = row.closest('.seg'); \
             const after = seg.nextElementSibling; \
             const card = after && after.classList.contains('opened') ? after : null; \
             const heads = document.querySelector('.window .r.head').getBoundingClientRect(); \
             return {{ hash: location.hash, top: box.top, bottom: box.bottom, vh: window.innerHeight, \
                      heads: heads.bottom, \
                      card: card ? card.querySelector('.name').textContent : null, \
                      cardTop: card ? card.getBoundingClientRect().top : null, \
                      last: Number(seg.dataset.base) + Number(seg.dataset.count) - 1, \
                      ticked: document.querySelector('input[name=\"tab\"][value=\"t01\"]').checked }}; }})()"
        ))
    };
    let opened = look(&mut page);
    assert_eq!(opened["hash"], format!("#i-{row}"), "{opened}");
    let (top, bottom, vh) = (
        opened["top"].as_f64().unwrap(),
        opened["bottom"].as_f64().unwrap(),
        opened["vh"].as_f64().unwrap(),
    );
    // in view, below the column heads that stick at the top
    let heads = opened["heads"].as_f64().unwrap();
    assert!(
        top >= heads - 0.5 && bottom <= vh,
        "the row is out of view: {opened}"
    );
    assert_eq!(opened["last"], 200, "its segment ends at it: {opened}");
    assert!(
        opened["card"]
            .as_str()
            .is_some_and(|n| n.starts_with("Bulk 0200")),
        "no card under it: {opened}"
    );
    assert!(
        opened["cardTop"].as_f64().unwrap() >= bottom - 0.5,
        "{opened}"
    );
    assert_eq!(opened["ticked"], json!(true), "the tick was lost: {opened}");

    // the rows under the card: the next segment's, as its route answers
    let under = "(() => { const seg = document.querySelector('.opened').nextElementSibling; \
        const h = Number(document.querySelector('.window').dataset.rowHeight); \
        const top = seg.getBoundingClientRect().top; const base = Number(seg.dataset.base); \
        const drawn = Array.from(seg.querySelectorAll('.r[data-i]'), (r) => \
          [Number(r.dataset.i), r.dataset.id, r.getBoundingClientRect().top - top]); \
        const at = new Map(drawn.map((r) => [r[0], r])); \
        const first = Math.max(0, Math.floor(-top / h)); \
        const last = Math.min(Number(seg.dataset.count), Math.ceil((window.innerHeight - top) / h)) - 1; \
        const covered = []; \
        for (let i = first; i <= last; i++) { if (!at.has(base + i)) return null; covered.push(at.get(base + i)); } \
        return { h, base, route: seg.dataset.route, covered }; })()";
    page.cdp.until(&format!("{under} !== null"));
    let seen = page.cdp.eval(under);
    assert_eq!(seen["base"], 201, "{seen}");
    let covered = seen["covered"].as_array().unwrap();
    assert!(!covered.is_empty(), "no row under the card in view: {seen}");
    let first = covered[0][0].as_u64().unwrap();
    let route = seen["route"]
        .as_str()
        .unwrap()
        .trim_start_matches(&format!("/{}/", web.token))
        .to_string();
    let answered = web.get_json(&format!("{route}&offset={first}&count={}", covered.len()));
    let ids: Vec<&Value> = answered["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| &r["id"])
        .collect();
    let shown: Vec<&Value> = covered.iter().map(|r| &r[1]).collect();
    assert_eq!(shown, ids, "the rows under the card");
    let h = seen["h"].as_f64().unwrap();
    for row in covered {
        let (i, y) = (row[0].as_f64().unwrap() - 201.0, row[2].as_f64().unwrap());
        assert!((y - i * h).abs() <= 0.5, "row {i} at {y}");
    }

    // closed by its card's Close: the row in view, no card, the tick kept
    page.cdp
        .click("document.querySelector('.opened button[name=\"toggle_item\"]')");
    page.loaded_again();
    page.cdp
        .until("document.querySelector('.window[data-windowed]') !== null");
    let closed = look(&mut page);
    assert_eq!(closed["hash"], format!("#i-{row}"), "{closed}");
    let (top, bottom) = (
        closed["top"].as_f64().unwrap(),
        closed["bottom"].as_f64().unwrap(),
    );
    // in view, below the column heads that stick at the top
    let heads = closed["heads"].as_f64().unwrap();
    assert!(
        top >= heads - 0.5 && bottom <= vh,
        "the row is out of view: {closed}"
    );
    assert_eq!(closed["card"], Value::Null, "{closed}");
    assert!(page.cdp.eval("document.querySelector('.opened') === null") == json!(true));
    assert_eq!(closed["ticked"], json!(true), "the tick was lost: {closed}");
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
