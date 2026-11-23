//! `acq web` at process level (C138, C139): the listener answers only
//! its own pages, at its own address, with the
//! token, mutating by POST alone and sending no CORS header; a form
//! submission's answer is `acq search --trade`'s for the same envelope; a
//! ticked selection submits what `refresh --tabs` submits; the plan loop
//! runs through the pages, a replay sending nothing and a moved policy
//! refused; every page is a wording golden. Facts are seeded through the
//! store crate for the search pages, which open no door (`ACQ_NO_SPAWN=1`
//! makes any daemon contact an error); the refresh pages run over the
//! mock daemon. No wait but the hang ceiling (C137): a page is read when
//! it comes.

use std::path::Path;

use acquisition_store::{Endpoint, Index, Store, account_path};
use serde_json::{Value, json};

use crate::harness;

use harness::{
    Pages, WebUi, offline_acq, offline_command, page_golden, panes_golden, sole_json, start_web,
    submitted::submitted,
};

pub(crate) const USER: &str = "Alice#1234";
const ART: &str = "https://web.poecdn.com/gen/image/ring.png";

/// The row fields the answer page asks by default, as `acq search
/// --fields` names them: what each JSON comparison of a page with the CLI
/// asks of the CLI, so the two compare whole.
const PAGE_FIELDS: &str = "icon,item";

/// The league list (Standard, Hardcore, and Settlers with no tab on
/// disk), and one league of two tabs: `Rings` with two life rings and Kaom's Heart,
/// each with its art; `Launch` with what a new league brings first — a
/// base the class table does not hold, and a mechanic's item whose line
/// carries a flag that is no yes or no.
fn seed(world: &Path) {
    let mock = world.join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let mut index = Index::load(&mock).unwrap();
    index.record_login(USER, "u-web", false, 1, &[]).unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    let mut record = |ep: Endpoint, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "pc", "league": "Standard" }),
                200,
                &body,
                at,
            )
            .unwrap();
    };
    record(
        Endpoint::Profile,
        json!({ "uuid": "u-web", "name": USER }),
        1,
    );
    record(
        Endpoint::Leagues,
        json!({ "leagues": [{ "id": "Standard" }, { "id": "Hardcore" }, { "id": "Settlers" }] }),
        2,
    );
    record(
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "stashes": [
            { "id": "t1", "name": "Rings", "type": "PremiumStash" },
            { "id": "t2", "name": "Launch", "type": "PremiumStash" } ] }),
        10,
    );
    let item = |id: &str, name: &str, base: &str, rarity: &str, more: Value| {
        let mut v = json!({ "id": id, "name": name, "typeLine": base, "baseType": base, "rarity": rarity,
                            "frameTypeId": rarity, "identified": true, "ilvl": 84, "x": 0, "y": 0 });
        for (k, value) in more.as_object().unwrap() {
            v[k] = value.clone();
        }
        v
    };
    let stash = |id: &str| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: None,
    };
    record(
        stash("t1"),
        json!({ "stash": { "id": "t1", "name": "Rings", "type": "PremiumStash", "items": [
            item("r1", "Doom Loop", "Two-Stone Ring", "Rare", json!({ "icon": ART, "explicitMods": ["+95 to maximum Life"] })),
            item("r2", "Rune Coil", "Two-Stone Ring", "Rare", json!({ "icon": ART, "implicitMods": ["+20 to maximum Life"], "explicitMods": ["+75 to maximum Life"] })),
            item("kaom", "Kaom's Heart", "Glorious Plate", "Unique", json!({ "icon": ART, "explicitMods": ["+500 to maximum Life"] })),
        ] } }),
        20,
    );
    record(
        stash("t2"),
        json!({ "stash": { "id": "t2", "name": "Launch", "type": "PremiumStash", "items": [
            item("new", "Brood Lure", "Wombgift Chalice", "Rare", json!({ "explicitMods": ["+40 to maximum Life"] })),
            item("mech", "Echo Seed", "Two-Stone Ring", "Rare", json!({ "wombgiftMods": [
                { "description": "Grants 3 Echoes of the Brood", "flags": { "echoing": "partly" } } ] })),
        ] } }),
        30,
    );
    // an Xbox league beside them, one ring in it
    let mut console = |ep: Endpoint, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "xbox", "league": "Hardcore" }),
                200,
                &body,
                at,
            )
            .unwrap();
    };
    console(
        Endpoint::Stashes {
            realm: "xbox".into(),
            league: "Hardcore".into(),
        },
        json!({ "stashes": [{ "id": "x1", "name": "Box", "type": "PremiumStash" }] }),
        40,
    );
    console(
        Endpoint::Stash {
            realm: "xbox".into(),
            league: "Hardcore".into(),
            id: "x1".into(),
            sub: None,
        },
        json!({ "stash": { "id": "x1", "name": "Box", "type": "PremiumStash", "items": [
            item("x", "Iron Grip", "Iron Ring", "Rare", json!({ "explicitMods": ["+30 to maximum Life"] })),
        ] } }),
        50,
    );
}

/// The offline world, seeded, and `acq web` over it.
pub(crate) fn offline_web(tag: &str) -> (harness::Scratch, WebUi) {
    let scratch = harness::scratch(tag);
    seed(&scratch.0);
    let web = start_web(offline_command(&scratch.0, &["web", "--no-browser"]));
    (scratch, web)
}

/// A store that holds tabs in two leagues and never fetched the league
/// list, as a copy of another store does where no session can fetch one:
/// the place pages offer the leagues it holds, and under a query say where
/// the matches this league's places do not hold stand, each a link to its
/// page.
#[test]
fn a_store_with_no_league_list_moves_between_the_leagues_it_holds() {
    let scratch = harness::scratch("webheldleagues");
    let mock = scratch.0.join("mock");
    std::fs::create_dir_all(&mock).unwrap();
    let mut index = Index::load(&mock).unwrap();
    index.record_login(USER, "u-held", false, 1, &[]).unwrap();
    let mut store = Store::open(&account_path(&mock, USER)).unwrap();
    store
        .record(
            &Endpoint::Profile,
            &json!({ "realm": "pc", "league": "Standard" }),
            200,
            &json!({ "uuid": "u-held", "name": USER }),
            1,
        )
        .unwrap();
    let ring = |id: &str| {
        json!({ "id": id, "name": "Doom Loop", "typeLine": "Iron Ring", "baseType": "Iron Ring",
                "rarity": "Rare", "frameTypeId": 2, "identified": true, "ilvl": 80, "x": 0, "y": 0 })
    };
    for (league, tab, rings, at) in [
        ("Standard", "s1", vec![ring("a"), ring("b")], 10),
        ("Settlers", "p1", vec![ring("c")], 20),
    ] {
        let params = json!({ "realm": "pc", "league": league });
        store
            .record(
                &Endpoint::Stashes {
                    realm: "pc".into(),
                    league: league.into(),
                },
                &params,
                200,
                &json!({ "stashes": [{ "id": tab, "name": "Rings", "type": "PremiumStash" }] }),
                at,
            )
            .unwrap();
        store
            .record(
                &Endpoint::Stash {
                    realm: "pc".into(),
                    league: league.into(),
                    id: tab.into(),
                    sub: None,
                },
                &params,
                200,
                &json!({ "stash": { "id": tab, "name": "Rings", "type": "PremiumStash", "items": rings } }),
                at + 1,
            )
            .unwrap();
    }
    drop(store);
    let web = start_web(offline_command(&scratch.0, &["web", "--no-browser"]));
    let tabs = web.get("tabs?realm=pc&league=Standard");
    assert_eq!(tabs.status, 200, "{}", tabs.body);
    assert!(
        tabs.body.contains("<label class=\"control\">League <select name=\"league\"><option value=\"Settlers\">Settlers (tabs on disk)</option><option value=\"Standard\" selected>Standard (tabs on disk)</option></select></label>"),
        "{}",
        tabs.body
    );
    let queried = web.get("tabs?realm=pc&league=Standard&text=class%3Arings");
    let settlers = format!(
        "/{}/tabs?realm=pc&amp;league=Settlers&amp;text=class%3Arings",
        web.token
    );
    assert!(
        queried
            .body
            .contains("2 matching items in these stash tabs, of 3 matches · ")
            && queried.body.contains(&format!(
                " · elsewhere: <a href=\"{settlers}\">1 in the stash tabs of Settlers</a></p>"
            )),
        "{}",
        queried.body
    );
    let there = follow(&web, &settlers.replace("&amp;", "&"));
    assert!(
        there
            .body
            .contains("1 matching item in these stash tabs, of 3 matches · ")
            && there
                .body
                .contains("2 in the stash tabs of Standard</a></p>"),
        "{}",
        there.body
    );
    // a league the store holds nothing of, and a kind it holds none of:
    // the control still names the page's own league, and the line still
    // says where every match stands
    let empty = web.get("tabs?realm=pc&league=Hardcore&text=class%3Arings");
    assert!(
        empty
            .body
            .contains("<option value=\"Hardcore\" selected>Hardcore</option>")
            && empty
                .body
                .contains("0 matching items in these stash tabs, of 3 matches · ")
            && empty
                .body
                .contains(">1 in the stash tabs of Settlers</a>, <a href=")
            && empty
                .body
                .contains(">2 in the stash tabs of Standard</a></p>"),
        "{}",
        empty.body
    );
    // a realm holding no league keeps the page's own in its control, so
    // its realm form sends the league back
    let console = web.get("tabs?realm=xbox&league=Settlers");
    assert!(
        console.body.contains("</select></label> <label class=\"control\">League <select name=\"league\"><option value=\"Settlers\" selected>Settlers</option></select></label>"),
        "{}",
        console.body
    );
    let characters = web.get("characters?realm=pc&league=Standard&text=class%3Arings");
    assert!(
        characters
            .body
            .contains("0 matching items on these characters, of 3 matches · ")
            && characters
                .body
                .contains(">2 in the stash tabs of Standard</a>"),
        "{}",
        characters.body
    );
}

/// No response of this server carries a CORS header (C138).
fn no_cors(reply: &harness::Reply, what: &str) {
    let cors: Vec<&(String, String)> = reply
        .headers
        .iter()
        .filter(|(k, _)| k.starts_with("access-control-"))
        .collect();
    assert!(cors.is_empty(), "{what}: {cors:?}");
}

/// A request the listener must refuse: its method, its path, its Host
/// where not the server's own, its other headers, and the status.
type Case<'a> = (
    &'a str,
    &'a str,
    Option<&'a str>,
    Vec<(&'a str, &'a str)>,
    u16,
);

/// C138, the listener: with the token it answers; without it, at another
/// Host, or sent from another site's page it refuses; a GET on a
/// mutating route is refused by its method; and no response, refusals
/// among them, carries a CORS header. The art's host never learns the
/// token, and the server alone gets a real Origin: every page's referrer
/// policy is `same-origin`. A POST carrying the server's own Origin, as
/// a browser sends it from the page, reaches its route; one carrying
/// `null`, what a browser sends under a `no-referrer` policy, is refused.
#[test]
fn c138_the_listener_answers_its_own_pages_alone() {
    let (_scratch, web) = offline_web("webguard");
    let token = web.token.clone();
    let form = web.get("");
    assert_eq!(form.status, 200, "{}", form.body);
    assert!(form.body.contains("Item Category"), "{}", form.body);
    assert_eq!(form.header("referrer-policy"), Some("same-origin"));
    no_cors(&form, "the form");
    let cases: [Case; 12] = [
        ("GET", "/", None, vec![], 404),
        (
            "GET",
            "/0123456789abcdef0123456789abcdef/",
            None,
            vec![],
            404,
        ),
        ("GET", &format!("/{}/", &token[..31]), None, vec![], 404),
        (
            "GET",
            &format!("/{token}/"),
            Some("evil.example"),
            vec![],
            403,
        ),
        (
            "GET",
            &format!("/{token}/"),
            Some(&format!(
                "localhost:{}",
                web.addr.rsplit(':').next().unwrap()
            )),
            vec![],
            403,
        ),
        (
            "GET",
            &format!("/{token}/"),
            None,
            vec![("Origin", "http://evil.example")],
            403,
        ),
        ("GET", &format!("/{token}/apply"), None, vec![], 405),
        // a page route takes no post
        ("POST", &format!("/{token}/plan"), None, vec![], 405),
        // the saved pages: a clear's GET asks and names an id, an opening
        // is a GET alone, and a write without the token or from another
        // site's page is refused before it runs
        ("GET", &format!("/{token}/saved/clear"), None, vec![], 400),
        ("POST", &format!("/{token}/saved/open"), None, vec![], 405),
        ("POST", "/saved", None, vec![], 404),
        (
            "POST",
            &format!("/{token}/saved"),
            None,
            vec![
                ("Origin", "http://evil.example"),
                ("Content-Type", "application/x-www-form-urlencoded"),
            ],
            403,
        ),
    ];
    for (method, path, host, headers, status) in &cases {
        let reply = web.raw(method, path, *host, headers, "");
        let what = format!("{method} {path} host {host:?} {headers:?}");
        assert_eq!(reply.status, *status, "{what}: {}", reply.body);
        assert!(!reply.body.contains("Item Category"), "{what}");
        no_cors(&reply, &what);
    }
    // a preflight is a method no route takes, and says nothing of CORS
    let preflight = web.raw(
        "OPTIONS",
        &format!("/{token}/apply"),
        None,
        &[
            ("Origin", "http://evil.example"),
            ("Access-Control-Request-Method", "POST"),
        ],
        "",
    );
    assert!(preflight.status >= 400, "{}", preflight.status);
    no_cors(&preflight, "a preflight");
    // a post from another site's page is refused before it runs
    let forged = web.raw(
        "POST",
        &format!("/{token}/apply"),
        None,
        &[
            ("Origin", "http://evil.example"),
            ("Content-Type", "application/x-www-form-urlencoded"),
        ],
        "plan=%7B%7D",
    );
    assert_eq!(forged.status, 403, "{}", forged.body);
    // the guard's two edges: the page's own Origin passes to the route,
    // which refuses the empty post in its own words; `null` never does
    let own = web.post("apply", &[]);
    assert_eq!(own.status, 400, "{}", own.body);
    assert!(own.body.contains("no plan was posted"), "{}", own.body);
    let null = web.raw(
        "POST",
        &format!("/{token}/apply"),
        None,
        &[
            ("Origin", "null"),
            ("Content-Type", "application/x-www-form-urlencoded"),
        ],
        "",
    );
    assert_eq!(null.status, 403, "{}", null.body);
    assert!(null.body.contains("another site"), "{}", null.body);
    // a field name the form never emits — an index past its stat rows
    // among them — is a usage error before anything is built
    let past = web.get("search?query.stats%5B1000000000%5D.type=and");
    assert_eq!(past.status, 400, "{}", past.body);
    assert!(past.body.contains("no field of this form"), "{}", past.body);
    let past = web.raw(
        "GET",
        &format!("/{token}/search?query.stats%5B1000000000%5D.type=and"),
        None,
        &[("Accept", "application/json")],
        "",
    );
    let refusal: Value = serde_json::from_str(&past.body).unwrap();
    assert_eq!(refusal["kind"], "usage", "{refusal}");
    // `--json` prints the URL as one document, as `auth` prints its own
    let scratch = harness::scratch("webjson");
    seed(&scratch.0);
    let json = start_web(offline_command(
        &scratch.0,
        &["web", "--json", "--no-browser"],
    ));
    assert_eq!(json.get("").status, 200);
    assert!(json.url.starts_with("http://127.0.0.1:"), "{}", json.url);
}

/// The form's fields for the ring search of the equality test, as the
/// browser submits them: the empty ones among them, which the fold drops.
fn ring_search() -> Vec<(&'static str, &'static str)> {
    vec![
        ("realm", "pc"),
        ("league", "Standard"),
        (
            "query.filters.type_filters.filters.category.option",
            "accessory.ring",
        ),
        ("query.filters.type_filters.filters.rarity.option", ""),
        ("query.filters.misc_filters.filters.ilvl.min", ""),
        ("group.type", "and"),
        ("stat.id", "explicit.stat_3299347043"),
        ("stat.min", "70"),
        ("stat.id", ""),
        ("view.limit", "20"),
        ("view.sort", ""),
    ]
}

/// The search form's opening tag on every page that carries it.
const SEARCH_FORM: &str =
    "<form id=\"search-form\" method=\"get\" action=\"search#answer\" novalidate data-count>";

/// The search route asked with a form's fields as a browser sends them.
fn sent_path(fields: &[(String, String)]) -> String {
    let pairs: Vec<(&str, &str)> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    format!("search?{}", query_string(&pairs))
}

fn query_string(fields: &[(&str, &str)]) -> String {
    fields
        .iter()
        .map(|(k, v)| format!("{}={}", harness::form_encoded(k), harness::form_encoded(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// The fold, pinned by equality: a form submission's answer is `acq
/// search --trade-file` over the envelope the fields name, in the same
/// realm with the same limit and the row fields the page asks, the art
/// among them (C140), over the same store.
#[test]
fn the_fold_answers_as_acq_search_trade_does() {
    let (scratch, web) = offline_web("webfold");
    let page = web.get_json(&format!("search?{}", query_string(&ring_search())));
    let envelope = json!({
        "league": "Standard",
        "query": {
            "filters": { "type_filters": { "filters": { "category": { "option": "accessory.ring" } } } },
            "stats": [{ "type": "and", "filters": [
                { "id": "explicit.stat_3299347043", "value": { "min": 70 } } ] }],
        },
    });
    let file = scratch.0.join("envelope.json");
    std::fs::write(&file, envelope.to_string()).unwrap();
    let out = offline_acq(
        &scratch.0,
        &[
            "search",
            "--trade-file",
            file.to_str().unwrap(),
            "--realm",
            "pc",
            "--limit",
            "20",
            "--fields",
            PAGE_FIELDS,
            "--json",
        ],
    );
    assert!(out.status.success(), "{out:?}");
    let cli = sole_json(&out);
    let rows = page["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2, "{rows:?}");
    for row in rows {
        assert_eq!(row["icon"], ART, "{row}");
    }
    assert_eq!(page, cli);
}

/// The count beside a search's button, pinned by equality: a form's
/// `count` is `acq search` of the same request at `--limit 0` and `--sort
/// none`, JSON whatever was asked, no row in it. The view the form names,
/// a sort or a count's keys, is not read, nor refused where its values
/// are no view's; the query box's count, the saved search it carries set
/// aside, is its search's total.
#[test]
fn the_count_answers_as_acq_search_at_limit_zero_does() {
    let (scratch, web) = offline_web("webcount");
    let count = |fields: &[(&str, &str)]| -> Value {
        let reply = web.get(&format!("count?{}", query_string(fields)));
        assert_eq!(reply.status, 200, "{}", reply.body);
        assert_eq!(reply.header("content-type"), Some("application/json"));
        serde_json::from_str(&reply.body).unwrap()
    };
    let counted = count(&ring_search());
    let envelope = json!({
        "league": "Standard",
        "query": {
            "filters": { "type_filters": { "filters": { "category": { "option": "accessory.ring" } } } },
            "stats": [{ "type": "and", "filters": [
                { "id": "explicit.stat_3299347043", "value": { "min": 70 } } ] }],
        },
    });
    let file = scratch.0.join("envelope.json");
    std::fs::write(&file, envelope.to_string()).unwrap();
    let out = offline_acq(
        &scratch.0,
        &[
            "search",
            "--trade-file",
            file.to_str().unwrap(),
            "--realm",
            "pc",
            "--limit",
            "0",
            "--sort",
            "none",
            "--fields",
            PAGE_FIELDS,
            "--json",
        ],
    );
    assert!(out.status.success(), "{out:?}");
    assert_eq!(counted, sole_json(&out));
    assert_eq!(counted["rows"], json!([]));
    assert_eq!(counted["total"]["matched"], 2);
    // a view the Search would refuse is the Search's to say: an
    // unfinished key, a limit or fields no view takes
    for view in [
        [("view.sort", "ilvl"), ("view.desc", "true")],
        [("view.count", "rarity"), ("view.limit", "50")],
        [("view.count", "line:\"life"), ("view.limit", "many")],
        [("view.fields", "bogus"), ("view.desc", "maybe")],
    ] {
        let mut fields: Vec<_> = ring_search()
            .into_iter()
            .filter(|(k, _)| !k.starts_with("view."))
            .collect();
        fields.extend(view);
        assert_eq!(count(&fields), counted, "{view:?}");
    }
    let boxed = [
        ("realm", "pc"),
        ("text", "class:rings"),
        ("view.limit", ""),
        ("view.sort", ""),
        ("view.fields", ""),
    ];
    let mut opened = boxed.to_vec();
    opened.extend([("saved", "0123456789ab"), ("saved_revision", "1")]);
    let total = web.get_json(&format!("search?{}", query_string(&boxed)))["total"].clone();
    assert!(total["matched"].as_u64().unwrap() > 0, "{total}");
    assert_eq!(count(&opened)["total"], total);
}

/// A refused form's count is its search's refusal as JSON: the same
/// status and kind, the words the server's, for a field no form has, a
/// query the language refuses and a site search that is none.
#[test]
fn a_refused_form_s_count_is_its_search_s_refusal() {
    let (_scratch, web) = offline_web("webcountrefused");
    for fields in [
        vec![("realm", "pc"), ("view.colour", "red")],
        vec![("realm", "pc"), ("text", "ilvl >")],
        vec![("realm", "pc"), ("trade", "not a search")],
    ] {
        let path = |route: &str| format!("/{}/{route}?{}", web.token, query_string(&fields));
        let search = web.raw(
            "GET",
            &path("search"),
            None,
            &[("Accept", "application/json")],
            "",
        );
        let count = web.get(&path("count")[web.token.len() + 2..]);
        assert_eq!(count.status, search.status, "{fields:?}: {}", count.body);
        assert_eq!(count.status, 400, "{fields:?}");
        let said = |body: &str| -> Value { serde_json::from_str(body).unwrap() };
        assert_eq!(said(&count.body), said(&search.body), "{fields:?}");
        assert!(said(&count.body)["kind"].is_string(), "{}", count.body);
    }
}

/// The page's answer to a site search beside `acq search` with `args`
/// and the page's own row fields in the same world: the two must be one
/// answer. The view a zero's offer of context carries is each asker's
/// own — the page leaves the limit its default — so it is taken out of
/// both.
fn trade_answers_as_the_cli(world: &Path, web: &WebUi, fields: &[(&str, &str)], args: &[&str]) {
    let mut page = web.get_json(&format!("search?{}", query_string(fields)));
    let mut args: Vec<&str> = ["search"].iter().chain(args).copied().collect();
    args.extend(["--fields", PAGE_FIELDS, "--json"]);
    let out = offline_acq(world, &args);
    assert!(out.status.success(), "{out:?}");
    let mut cli = sole_json(&out);
    for row in page["rows"].as_array().unwrap() {
        assert_eq!(row["icon"], ART, "{row}");
    }
    for answer in [&mut page, &mut cli] {
        if let Some(request) = answer.pointer_mut("/zero/offers/context/request") {
            request.as_object_mut().unwrap().shift_remove("view");
        }
    }
    assert_eq!(page, cli);
}

/// #188, its first half: a site search's URL in the trade box answers
/// as `acq search --trade` on the same URL — the translation's fidelity
/// and its clauses on the page — and the realm the page carried, the
/// URL's own, is no conflict.
#[test]
fn a_trade_url_in_the_box_answers_as_acq_search_trade_does() {
    use crate::search_json::{TRADE_TEXT, trade_url};
    let (scratch, web) = offline_web("webtradeurl");
    let url = trade_url();
    let fields = [("realm", "pc"), ("trade", url.as_str())];
    trade_answers_as_the_cli(&scratch.0, &web, &fields, &["--trade", &url]);
    let answer = web.get(&format!("search?{}", query_string(&fields)));
    assert_eq!(answer.status, 200, "{}", answer.body);
    let main = unescaped(main_of(&answer.body));
    for fact in [
        "not the site's search: 2 clauses not run",
        "from a URL: PARTIAL — 2 clauses not run",
        "<td>NOT RUN</td><td>stats.1</td><td>A crucible tree path is not read",
        "<td>reported</td><td>status</td>",
        TRADE_TEXT,
    ] {
        assert!(main.contains(fact), "{fact}: {main}");
    }
    assert!(!main.contains("it ran there, not in"), "{main}");
}

/// #188: a bare id names no realm, so the realm the page carried is the
/// search's scope, as `--realm` is beside `--trade <ID>`.
#[test]
fn a_trade_id_in_the_box_runs_in_the_pages_realm() {
    use crate::search_json::TRADE_ID;
    let (scratch, web) = offline_web("webtradeid");
    for realm in ["pc", "xbox"] {
        trade_answers_as_the_cli(
            &scratch.0,
            &web,
            &[("realm", realm), ("trade", TRADE_ID)],
            &["--trade", TRADE_ID, "--realm", realm],
        );
    }
}

/// #188, the realm rule as ruled for the page: the URL's realm is the
/// search's scope (the owner's words in the issue), so the realm the page
/// carried beside a URL naming another is not read, and the answer says
/// so; the terminal's `--realm` beside one stays its refusal.
#[test]
fn a_trade_url_runs_in_its_own_realm_whatever_the_page_carried() {
    use crate::search_json::TRADE_ID;
    let (scratch, web) = offline_web("webtraderealm");
    let url = format!("https://www.pathofexile.com/trade/search/xbox/Hardcore/{TRADE_ID}");
    let fields = [("realm", "pc"), ("trade", url.as_str())];
    trade_answers_as_the_cli(&scratch.0, &web, &fields, &["--trade", &url]);
    assert_eq!(
        web.get_json(&format!("search?{}", query_string(&fields)))["scope"]["realm"],
        "xbox"
    );
    let answer = web.get(&format!("search?{}", query_string(&fields)));
    assert_eq!(answer.status, 200, "{}", answer.body);
    assert!(
        unescaped(main_of(&answer.body)).contains(
            "The site search's URL names its realm, xbox: it ran there, not in pc, the realm the page showed."
        ),
        "{}",
        answer.body
    );
    let out = offline_acq(&scratch.0, &["search", "--trade", &url, "--realm", "pc"]);
    assert!(!out.status.success(), "{out:?}");
}

/// #188: the two boxes are two searches, so both sent is a usage
/// refusal in the page's words, and the trade box sent empty is one.
#[test]
fn the_trade_box_beside_the_query_box_or_empty_is_refused() {
    use crate::search_json::TRADE_ID;
    let (_scratch, web) = offline_web("webtradeboth");
    for (fields, says) in [
        (
            vec![
                ("realm", "pc"),
                ("text", "class:rings"),
                ("trade", TRADE_ID),
            ],
            "a search is a query in the language or a trade site search, not both",
        ),
        (
            vec![("realm", "pc"), ("text", ""), ("trade", TRADE_ID)],
            "a search is a query in the language or a trade site search, not both",
        ),
        (
            vec![("realm", "pc"), ("trade", " ")],
            "the trade site search box is empty",
        ),
    ] {
        let refused = web.get(&format!("search?{}", query_string(&fields)));
        assert_eq!(refused.status, 400, "{fields:?}: {}", refused.body);
        assert!(
            unescaped(&refused.body).contains(says),
            "{fields:?}: {}",
            refused.body
        );
    }
}

/// C194, the row keeps its source: a page's save form carries the site
/// search the page answered, a URL from the site box and the form's own
/// envelope as JSON, and the row stores it as sent, a JSON text as its
/// object; the list derives its text and fidelity; opened, the row
/// answers with its translation's report, and saved again from that
/// page it stores the same source. A field the translation does not run
/// survives each save.
#[test]
fn c194_a_page_saves_the_site_search_it_answered_and_opens_it_with_its_report() {
    use crate::search_json::{TRADE_TEXT, trade_url};
    let (scratch, web) = offline_web("webtradesave");
    let world = scratch.0.clone();
    let token = web.token.clone();
    let url = trade_url();
    let mut envelope = ring_search();
    // a stat no table holds: a clause the translation does not run
    // a row of the form's one group, as the page's order places it
    envelope.push(("stat.id", "explicit.stat_9999999999"));
    for (sent, name) in [
        (
            vec![("realm", "xbox"), ("trade", url.as_str())],
            "life rings",
        ),
        (envelope, "form rings"),
    ] {
        let answer = web.get(&format!("search?{}", query_string(&sent)));
        assert_eq!(answer.status, 200, "{}", answer.body);
        let save = form_on(&answer.body, "Save this search", None);
        assert_eq!(save.field("text"), None, "{save:?}");
        let trade = save.field("trade").unwrap().to_string();
        assert_eq!(save.field("realm"), Some("pc"), "{save:?}");
        landed_on_the_list(&web, &send(&web, &save.typed("name", name), false));
        let row = saved_json(&world, &["list"])["searches"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["name"] == name)
            .unwrap()
            .clone();
        // stored as sent: the URL as a string, the envelope as its object
        let stored = match serde_json::from_str::<Value>(&trade) {
            Ok(object @ Value::Object(_)) => object,
            _ => json!(trade),
        };
        assert_eq!(row["query"], json!({ "trade": stored }), "{row}");
        assert_eq!(row["scope"]["realm"], "pc", "{row}");
        assert!(row["text"].is_string(), "{row}");
        assert_eq!(row["fidelity"], "partial", "{row}");
        if name == "life rings" {
            assert_eq!(row["text"], TRADE_TEXT, "{row}");
        } else {
            assert!(trade.contains("explicit.stat_9999999999"), "{trade}");
        }
        // opened: the site box's search, its report in the answer
        let id = row["id"].as_str().unwrap().to_string();
        let open = follow(&web, &format!("/{token}/saved/open?id={id}"));
        assert_eq!(open.status, 303, "{}", open.body);
        let location = open.header("location").unwrap().to_string();
        assert!(location.contains("trade="), "{location}");
        let page = json_at(&web, &location);
        assert_eq!(page["query"]["trade"]["fidelity"], "partial", "{page}");
        assert_eq!(page["query"]["text"], row["text"], "{page}");
        let cli = sole_json(&offline_acq(
            &world,
            &["search", "--saved", &id, "--fields", PAGE_FIELDS, "--json"],
        ));
        assert_eq!(page, cli, "{id}");
        // saved again from the opened page: the same source, over the row
        let opened = follow(&web, &location);
        assert_eq!(opened.status, 200, "{}", opened.body);
        // the opened page stands under its editor filled (the one page):
        // its search form, as a browser submits it, carries the saved
        // search and asks the search the row holds, an unread stat a row
        let form = submitted(&opened.body, SEARCH_FORM);
        assert!(
            form.contains(&("saved".to_string(), id.clone())),
            "{form:?}"
        );
        if name == "form rings" {
            assert!(
                form.contains(&(
                    "stat.id".to_string(),
                    "explicit.stat_9999999999".to_string()
                )),
                "{form:?}"
            );
        }
        let asked = web.get_json(&sent_path(&form));
        assert_eq!(asked["query"]["text"], row["text"], "{asked}");
        let over = form_on(&opened.body, &format!("Save over \"{name}\""), Some(&id));
        landed_on_the_list(&web, &send(&web, &over, false));
        let again = saved_json(&world, &["show", &id]);
        assert_eq!(again["revision"], 2, "{again}");
        assert_eq!(again["query"], row["query"], "{again}");
    }
}

/// The one page (`4-full-1-form-design.md` §2): a search stands under its
/// editors filled from its source. A form search of two groups is asked
/// again, both groups in order, by its search form as a browser submits it
/// and by every link and hidden form its page writes; a site search fills
/// the form, and so does its continuation; a count's two editors keep its
/// keys and its limit; and a refused search is the same page, filled from
/// the nearest thing read, its status and its JSON the refusal's.
#[test]
fn every_search_stands_under_its_editor_filled_with_what_was_asked() {
    let (_scratch, web) = offline_web("webeditor");
    let token_search = format!("/{}/search?", web.token);
    // a search's stat clauses as its answer reports them, each where it
    // stands and what the site wrote for it
    let stats_of = |answer: &Value| -> Vec<(String, Value)> {
        answer["query"]["trade"]["clauses"]
            .as_array()
            .unwrap_or_else(|| panic!("no trade report: {answer}"))
            .iter()
            .filter(|c| c["at"].as_str().unwrap().starts_with("stats."))
            .map(|c| (c["at"].as_str().unwrap().to_string(), c["site"].clone()))
            .collect()
    };
    // the search a link or a form asks, as its answer's JSON
    let asked = |href: &str| -> Option<Value> {
        let href = unescaped(href);
        let rest = href
            .strip_prefix("search?")
            .or_else(|| href.strip_prefix(&token_search))?;
        Some(web.get_json(&format!("search?{rest}")))
    };
    // a form search of two groups, the second a count
    let two = [
        ("realm", "pc"),
        ("league", "Standard"),
        (
            "query.filters.type_filters.filters.category.option",
            "accessory.ring",
        ),
        ("group.type", "and"),
        ("stat.id", "explicit.stat_3299347043"),
        ("stat.min", "40"),
        ("group.type", "count"),
        ("group.min", "1"),
        ("stat.id", "explicit.stat_3372524247"),
        ("stat.id", "pseudo.pseudo_total_life"),
        ("view.limit", "20"),
    ];
    let mut tabled = two.to_vec();
    tabled.push(("view.fields", "icon"));
    for sent in [two.to_vec(), tabled] {
        let path = format!("search?{}", query_string(&sent));
        let page = web.get(&path);
        assert_eq!(page.status, 200, "{}", page.body);
        let answer = web.get_json(&path);
        let groups = stats_of(&answer);
        assert!(groups.iter().any(|(at, _)| at == "stats.1.1"), "{answer}");
        // the search form as a browser submits it asks the same search
        let form = submitted(&page.body, SEARCH_FORM);
        assert_eq!(web.get_json(&sent_path(&form)), answer, "{form:?}");
        // every link the page writes with the form's fields, and the
        // table's form, keeps both groups in their order (a route to
        // another search is written as its text, and carries none)
        let mut kept = 0;
        for href in page.body.split("href=\"").skip(1) {
            let href = &href[..href.find('"').unwrap()];
            if !href.contains("group.type=") {
                continue;
            }
            let linked = asked(href).unwrap_or_else(|| panic!("{href} is no search"));
            assert_eq!(stats_of(&linked), groups, "{href}");
            kept += 1;
        }
        let table = "<form class=\"table\" method=\"get\" action=\"search\">";
        if page.body.contains(table) {
            let held = submitted(&page.body, table);
            assert_eq!(
                stats_of(&web.get_json(&sent_path(&held))),
                groups,
                "{held:?}"
            );
            kept += 1;
        }
        assert!(kept > 1, "{}", page.body);
    }
    // a site search fills the form, and the form asks it again; so does
    // its continuation, read by its token's query
    let rings = r#"{"query": {"filters": {"type_filters": {"filters": {"category": {"option": "accessory.ring"}}}}}}"#;
    let site_path = format!(
        "search?{}",
        query_string(&[("realm", "pc"), ("trade", rings), ("view.limit", "1")])
    );
    let site = web.get(&site_path);
    assert_eq!(site.status, 200, "{}", site.body);
    let site_answer = web.get_json(&site_path);
    let form = submitted(&site.body, SEARCH_FORM);
    assert!(
        form.contains(&(
            "query.filters.type_filters.filters.category.option".to_string(),
            "accessory.ring".to_string()
        )),
        "{form:?}"
    );
    let resent = web.get_json(&sent_path(&form));
    assert_eq!(
        resent["query"]["text"], site_answer["query"]["text"],
        "{resent}"
    );
    assert_eq!(resent["total"], site_answer["total"], "{resent}");
    let more = site.body.split("href=\"").skip(1).find_map(|href| {
        let href = unescaped(&href[..href.find('"').unwrap()]);
        href.contains("search?next=").then_some(href)
    });
    let more = more.unwrap_or_else(|| panic!("no continuation:\n{}", site.body));
    let continued = go(&web, &more, false);
    let continued = continued.as_str().unwrap();
    assert_eq!(
        submitted(continued, SEARCH_FORM),
        form,
        "a continuation fills the form as its first page does"
    );
    // a count's two editors each keep its keys and its limit
    let mut counted = two.to_vec();
    counted.retain(|(k, _)| *k != "view.limit");
    counted.extend([("view.count", "rarity"), ("view.limit", "5")]);
    let count_path = format!("search?{}", query_string(&counted));
    let count_page = web.get(&count_path);
    assert_eq!(count_page.status, 200, "{}", count_page.body);
    let count_answer = web.get_json(&count_path);
    let form = submitted(&count_page.body, SEARCH_FORM);
    assert_eq!(web.get_json(&sent_path(&form)), count_answer, "{form:?}");
    let boxed = submitted(
        &count_page.body,
        "<form class=\"query-box\" method=\"get\" action=\"search#answer\" data-count>",
    );
    for kept in [("view.count", "rarity"), ("view.limit", "5")] {
        assert!(
            boxed.contains(&(kept.0.to_string(), kept.1.to_string())),
            "{boxed:?}"
        );
    }
    let boxed_answer = web.get_json(&sent_path(&boxed));
    assert_eq!(boxed_answer["view"], count_answer["view"], "{boxed_answer}");
    // an editor whose page carried an order and its columns, sent with a
    // count, answers the count over the query it held: a count shows no
    // rows, so a page's search lets go of what it carried hidden (#304)
    let box_form = "<form class=\"query-box\" method=\"get\" action=\"search#answer\" data-count>";
    let mut ordered_form = two.to_vec();
    ordered_form.extend([
        ("view.sort", "ilvl"),
        ("view.desc", "true"),
        ("view.value", "ilvl"),
    ]);
    for (sent, editor, own) in [
        (
            vec![
                ("realm", "pc"),
                ("text", "class:rings"),
                ("view.sort", "ilvl"),
                ("view.value", "ilvl"),
            ],
            box_form,
            vec![
                ("realm", "pc"),
                ("text", "class:rings"),
                ("view.count", "rarity"),
            ],
        ),
        (ordered_form, SEARCH_FORM, {
            let mut own = two.to_vec();
            own.retain(|(k, _)| *k != "view.limit");
            own.push(("view.count", "rarity"));
            own
        }),
    ] {
        let page = web.get(&format!("search?{}", query_string(&sent)));
        assert_eq!(page.status, 200, "{}", page.body);
        let mut form = submitted(&page.body, editor);
        for carried in [("view.sort", "ilvl"), ("view.value", "ilvl")] {
            assert!(
                form.contains(&(carried.0.to_string(), carried.1.to_string())),
                "{form:?}"
            );
        }
        assert!(
            !page.body.contains("type=\"checkbox\" name=\"view."),
            "{}",
            page.body
        );
        form.retain(|(k, _)| k != "view.count" && k != "view.limit");
        form.push(("view.count".to_string(), "rarity".to_string()));
        let path = sent_path(&form);
        let reply = web.get(&path);
        assert_eq!(reply.status, 200, "{path}: {}", reply.body);
        let counted = web.get_json(&path);
        let expected = web.get_json(&format!("search?{}", query_string(&own)));
        assert_eq!(counted["query"], expected["query"], "{path}");
        assert_eq!(counted["view"], expected["view"], "{path}");
        assert_eq!(counted["total"], expected["total"], "{path}");
    }
    // refused: the fold's refusal over the empty form; a site search the
    // decode refuses in its box as sent; the form's fields the request
    // refuses, filled; each its status and its JSON the refusal's
    let empty = submitted(&web.get("").body, SEARCH_FORM);
    let unfolded = format!(
        "search?{}",
        query_string(&[("realm", "pc"), ("nonsense", "1")])
    );
    let page = web.get(&unfolded);
    assert_eq!(page.status, 400, "{}", page.body);
    assert_eq!(submitted(&page.body, SEARCH_FORM), empty);
    assert_eq!(refused_json(&web, &unfolded, 400)["kind"], "usage");
    let undecoded = format!(
        "search?{}",
        query_string(&[("realm", "pc"), ("trade", "not an id")])
    );
    let page = web.get(&undecoded);
    assert_eq!(page.status, 400, "{}", page.body);
    assert!(
        page.body
            .contains("form=\"trade-form\" rows=\"1\" placeholder="),
        "{}",
        page.body
    );
    assert_eq!(submitted(&page.body, SEARCH_FORM), empty);
    assert_eq!(refused_json(&web, &undecoded, 400)["kind"], "trade");
    let mut unread = two.to_vec();
    unread.push(("view.sort", "no such value"));
    let unread = format!("search?{}", query_string(&unread));
    let page = web.get(&unread);
    assert_eq!(page.status, 400, "{}", page.body);
    let form = submitted(&page.body, SEARCH_FORM);
    for held in [
        (
            "query.filters.type_filters.filters.category.option",
            "accessory.ring",
        ),
        ("stat.id", "pseudo.pseudo_total_life"),
        ("view.sort", "no such value"),
    ] {
        assert!(
            form.contains(&(held.0.to_string(), held.1.to_string())),
            "{form:?}"
        );
    }
    assert!(refused_json(&web, &unread, 400)["error"].is_string());
}

/// The query box sorts and pages as the big form does: its "Show" is on
/// the form page, a typed query with a sort answers as `acq search --sort
/// --desc` does, and the answer's box comes back set to the rows view it
/// showed, the sort and its direction carried hidden (nothing is typed to
/// sort).
#[test]
fn the_query_box_sorts_and_pages_a_typed_query() {
    let (scratch, web) = offline_web("webquerysort");
    let form = web.get("");
    let boxed = &form.body[form.body.find("class=\"query-box\"").unwrap()..];
    assert!(
        boxed.contains("<select name=\"view.limit\"><option value=\"\" selected>10</option>"),
        "{boxed}"
    );
    let fields = [
        ("realm", "pc"),
        ("membership", "live"),
        ("text", "class:rings"),
        ("view.limit", "20"),
        ("view.sort", "ilvl"),
        ("view.desc", "true"),
    ];
    let page = web.get_json(&format!("search?{}", query_string(&fields)));
    let out = offline_acq(
        &scratch.0,
        &[
            "search",
            "--realm",
            "pc",
            "class:rings",
            "--limit",
            "20",
            "--sort",
            "ilvl",
            "--desc",
            "--fields",
            PAGE_FIELDS,
            "--json",
        ],
    );
    assert!(out.status.success(), "{out:?}");
    let cli = sole_json(&out);
    assert!(page["rows"].as_array().unwrap().len() > 1, "{page}");
    assert_eq!(page["rows"], cli["rows"]);
    assert_eq!(page["view"], cli["view"]);
    let answer = web.get(&format!("search?{}", query_string(&fields)));
    assert_eq!(answer.status, 200, "{}", answer.body);
    let boxed = &answer.body[answer.body.find("class=\"query-box\"").unwrap()..];
    for control in [
        "<option value=\"20\" selected>20</option>",
        "<input type=\"hidden\" name=\"view.sort\" value=\"ilvl\">",
        "<input type=\"hidden\" name=\"view.desc\" value=\"true\">",
    ] {
        assert!(boxed.contains(control), "{control}: {boxed}");
    }
}

/// The search pages, each a golden naming what a reader must find: the
/// form with each control's fate and the listing-only filters' sentence,
/// the answer with its clause statuses, its total and its rows with their
/// art, one item, and a refusal in the search's own words.
/// A link a page prints, absolute or relative to the token, followed as
/// a browser follows it; with `json`, its document.
fn go(web: &WebUi, href: &str, json: bool) -> Value {
    let href = unescaped(href);
    let path = match href.strip_prefix(&format!("/{}/", web.token)) {
        Some(rest) => rest.to_string(),
        None => href,
    };
    match json {
        true => web.get_json(&path),
        false => json!(web.get(&path).body),
    }
}

/// Each value head of a table page: its text, and its link.
fn heads(body: &str) -> Vec<(String, String)> {
    body.split("<a class=\"head\" href=\"")
        .skip(1)
        .map(|rest| {
            let href = rest[..rest.find('"').unwrap()].to_string();
            let inner = &rest[rest.find('>').unwrap() + 1..rest.find("</a>").unwrap()];
            // the head's own text, the mark and its words taken out
            let text = inner
                .split("<span")
                .map(|part| part.rsplit_once("</span>").map_or(part, |(_, t)| t))
                .collect::<String>();
            (unescaped(&text), href)
        })
        .collect()
}

/// The ids of an answer's rows, in its order.
fn ids(answer: &Value) -> Vec<&str> {
    answer["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect()
}

/// The fields the query box on a page sends: its hidden ones, in order.
fn box_fields(body: &str) -> Vec<(String, String)> {
    let at = body.find("class=\"query-box\"").unwrap();
    let form = &body[at..at + body[at..].find("</form>").unwrap()];
    form.split("<input ")
        .skip(1)
        .filter_map(|input| {
            let tag = format!(" {}", &input[..input.find('>')?]);
            Some((attribute(&tag, "name")?, attribute(&tag, "value")?))
        })
        .collect()
}

/// The values the URL names
/// fold into the rows view and are written back as fields that fold to
/// the same request; a value's head is a link that orders the rows by it,
/// largest first, and followed again the other way, the list unchanged,
/// the ordered head marked in a mark and in words; the query box carries
/// the sort, its direction and the values named, hidden, so an edited
/// query keeps them; and no page holds a control to type a sort.
#[test]
fn a_head_orders_by_its_value_and_flips_and_the_box_keeps_what_was_named() {
    let (_scratch, web) = offline_web("webheads");
    let life = "sum(line(\"# to maximum Life\" source=explicit).arg1)";
    let named = [
        ("realm", "pc"),
        ("text", "line(\"# to maximum Life\")"),
        ("view.fields", "icon"),
        ("view.value", "ilvl"),
        ("view.value", life),
    ];
    let first = web.get_json(&format!("search?{}", query_string(&named)));
    assert_eq!(first["view"]["rows"]["values"], json!(["ilvl", life]));
    let page = web.get(&format!("search?{}", query_string(&named))).body;
    let found = heads(&page);
    assert_eq!(
        found.iter().map(|(t, _)| t.as_str()).collect::<Vec<_>>(),
        ["ilvl", life],
        "{page}"
    );
    assert!(
        !page.contains("class=\"mark\""),
        "no order, no mark: {page}"
    );
    // the head's link: ordered by it, largest first, the list as named
    let largest = go(&web, &found[1].1, true);
    assert_eq!(largest["view"]["rows"]["sort"], life);
    assert_eq!(largest["view"]["rows"]["desc"], true);
    assert_eq!(largest["view"]["rows"]["values"], json!(["ilvl", life]));
    assert_eq!(ids(&largest), ["kaom", "r1", "r2", "new"]);
    let page = go(&web, &found[1].1, false);
    let page = page.as_str().unwrap();
    assert!(
        page.contains(&format!(
            "<span class=\"mark\" aria-hidden=\"true\">▼</span>{}<span class=\"sr\">, ordered largest first</span>",
            life.replace('"', "&quot;")
        )),
        "{page}"
    );
    // the same head again: the other way, the list unchanged
    let again = heads(page);
    let smallest = go(&web, &again[1].1, true);
    assert_eq!(smallest["view"]["rows"]["sort"], life);
    assert!(
        smallest["view"]["rows"]["desc"].is_null(),
        "{}",
        smallest["view"]
    );
    assert_eq!(smallest["view"]["rows"]["values"], json!(["ilvl", life]));
    assert_eq!(ids(&smallest), ["new", "r2", "r1", "kaom"]);
    // the box carries what the URL named, hidden; an edited query run
    // from it keeps the sort and the values
    let carried = box_fields(page);
    for field in [
        ("view.sort", life),
        ("view.desc", "true"),
        ("view.value", "ilvl"),
        ("view.value", life),
    ] {
        assert!(
            carried.contains(&(field.0.to_string(), field.1.to_string())),
            "{field:?}: {carried:?}"
        );
    }
    let mut edited: Vec<(&str, &str)> = carried
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    edited.push(("text", "line(\"# to maximum Life\") ilvl>=1"));
    let kept = web.get_json(&format!("search?{}", query_string(&edited)));
    assert_eq!(kept["query"]["text"], "line(\"# to maximum Life\") ilvl>=1");
    assert_eq!(kept["view"]["rows"]["sort"], life);
    assert_eq!(kept["view"]["rows"]["desc"], true);
    assert_eq!(kept["view"]["rows"]["values"], json!(["ilvl", life]));
    // what the query gave is never carried: the query's values, named
    // nowhere, leave the box empty of them
    let ring = web
        .get(&format!("search?{}", query_string(&ring_search())))
        .body;
    let carried = box_fields(&ring);
    assert!(
        carried.iter().all(|(k, _)| !k.starts_with("view.")),
        "{carried:?}"
    );
    // named values written back by the form's own fields: a continuation's
    // first rows link folds to the request the URL named
    let mut paged = named.to_vec();
    paged.retain(|(k, _)| *k != "view.fields");
    paged.push(("view.limit", "1"));
    let cards = web.get_json(&format!("search?{}", query_string(&paged)));
    let token = serde_json::from_value::<acquisition_search::answer::Request>(
        cards["view"]["rows"]["next"]["request"].clone(),
    )
    .unwrap()
    .token();
    let next = web.get(&format!("search?next={token}")).body;
    let first_rows = link_saying(&next, "first rows");
    assert_eq!(go(&web, &first_rows, true)["view"], cards["view"]);
    // no page holds a control that types a sort: the form, an answer
    for body in [web.get("").body, page.to_string()] {
        for name in ["view.sort", "view.desc"] {
            let typed = body.split("<input ").skip(1).any(|tag| {
                tag.contains(&format!("name=\"{name}\"")) && !tag.starts_with("type=\"hidden\"")
            });
            assert!(!typed, "a control types {name}");
        }
        assert!(!body.contains("Sort by") && !body.contains("highest first"));
    }
}

/// A card's number for a value the view
/// lacks is a link to the same answer with that value added to its list
/// and ordered by it, largest first, as cards; where the view's list is
/// full, a number outside it is words, and one inside it a link.
#[test]
fn a_cards_number_adds_its_value_and_a_full_list_adds_none() {
    let (_scratch, web) = offline_web("webcardnumber");
    let life = "sum(line(\"# to maximum Life\" source=explicit).arg1)";
    let asked = [("realm", "pc"), ("text", "class:rings")];
    let before = web.get_json(&format!("search?{}", query_string(&asked)));
    assert!(
        before["view"]["rows"]["values"].is_null(),
        "{}",
        before["view"]
    );
    let page = web.get(&format!("search?{}", query_string(&asked))).body;
    let at = page
        .find(">+95 to maximum Life</a>")
        .unwrap_or_else(|| panic!("{page}"));
    let href = &page[page[..at].rfind("href=\"").unwrap() + 6..at - 1];
    let added = go(&web, href, true);
    assert_eq!(added["view"]["rows"]["values"], json!([life]));
    assert_eq!(added["view"]["rows"]["sort"], life);
    assert_eq!(added["view"]["rows"]["desc"], true);
    assert_eq!(added["view"]["rows"]["fields"], json!(["icon", "item"]));
    // the ring with no life line last, as a sort puts an item lacking it
    assert_eq!(ids(&added), ["r1", "r2", "mech"]);
    // twelve values named: the life line is outside the list and words;
    // the item level, inside it, a link
    let full = [
        "ilvl",
        "reqlevel",
        "stack",
        "sockets",
        "links",
        "sockets.red",
        "sockets.green",
        "sockets.blue",
        "sockets.white",
        "price.amount",
        "price.lot",
        "quality",
    ];
    let mut named = asked.to_vec();
    named.extend(full.iter().map(|v| ("view.value", *v)));
    let answered = web.get_json(&format!("search?{}", query_string(&named)));
    assert_eq!(
        answered["view"]["rows"]["values"].as_array().unwrap().len(),
        acquisition_search::answer::VALUES
    );
    let page = web.get(&format!("search?{}", query_string(&named))).body;
    assert!(
        page.contains("<p class=\"mod\">+95 to maximum Life <span class=\"mod-kind\">"),
        "{page}"
    );
    assert!(
        page.contains("Item Level: <a class=\"value valued\" href=\""),
        "{page}"
    );
}

/// A value picked by name (`view.by`, the value finder;
/// `4-full-1-form-values.md`, Decision 3) is the page's: the search page
/// and a place page answer `303` to the URL their value's head writes,
/// held or added; beside a count, to the page less it; past a full list,
/// to the search's own refusal of the longer list; and a place page that
/// draws no item heads refuses it in words. The fold never sees it: a
/// route that folds the form's fields alone refuses it as no field of the
/// form.
#[test]
fn a_value_picked_by_name_lands_where_its_head_would() {
    let (scratch, web) = offline_web("webpicked");
    let life = "sum(line(\"# to maximum Life\" source=explicit).arg1)";
    let located = |path: &str| -> String {
        let reply = web.get(path);
        assert_eq!(reply.status, 303, "{path}: {}", reply.body);
        reply.header("location").expect("a location").to_string()
    };
    let head_of = |body: &str, text: &str| -> String {
        let (_, href) = heads(body)
            .into_iter()
            .find(|(t, _)| t == text)
            .unwrap_or_else(|| panic!("{text}: {body}"));
        let href = unescaped(&href);
        match href.starts_with('/') {
            true => href,
            false => format!("/{}/{href}", web.token),
        }
    };
    // the search page: a value held lands where its head links; one the
    // list lacks where a head of the list with it would, ordered largest
    // first, its column added
    let held = [
        ("realm", "pc"),
        ("text", "class:rings"),
        ("view.fields", "icon"),
        ("view.value", "ilvl"),
        ("view.value", life),
    ];
    let page = web.get(&format!("search?{}", query_string(&held))).body;
    let mut picked = held.to_vec();
    picked.push(("view.by", life));
    let location = located(&format!("search?{}", query_string(&picked)));
    assert_eq!(location, head_of(&page, life));
    let added = [
        ("realm", "pc"),
        ("text", "class:rings"),
        ("view.fields", "icon"),
    ];
    let mut picked = added.to_vec();
    picked.push(("view.by", life));
    let location = located(&format!("search?{}", query_string(&picked)));
    let landed = follow(&web, &location);
    assert_eq!(landed.status, 200, "{}", landed.body);
    assert!(!location.contains("view.by"), "{location}");
    let ordered = web.get_json(&location[location.find("search?").unwrap()..]);
    assert_eq!(ordered["view"]["rows"]["sort"], life);
    assert_eq!(ordered["view"]["rows"]["desc"], true);
    assert_eq!(ordered["view"]["rows"]["values"], json!([life]));
    assert!(
        !head_of(&landed.body, life).contains("view.desc"),
        "the ordered head flips"
    );
    // beside a count: the page less it
    let counted = [
        ("realm", "pc"),
        ("text", "class:rings"),
        ("view.count", "rarity"),
    ];
    let mut picked = counted.to_vec();
    picked.push(("view.by", "ilvl"));
    assert_eq!(
        located(&format!("search?{}", query_string(&picked))),
        format!("/{}/search?{}", web.token, query_string(&counted))
    );
    // past a full list: the search's own refusal of the longer one
    let full = [
        "ilvl",
        "reqlevel",
        "stack",
        "sockets",
        "links",
        "sockets.red",
        "sockets.green",
        "sockets.blue",
        "sockets.white",
        "price.amount",
        "price.lot",
        "quality",
    ];
    let mut picked = added.to_vec();
    picked.extend(full.iter().map(|v| ("view.value", *v)));
    picked.push(("view.by", life));
    let refused = follow(&web, &located(&format!("search?{}", query_string(&picked))));
    assert_eq!(refused.status, 400, "{}", refused.body);
    assert!(
        refused.body.contains("a row carries at most 12"),
        "{}",
        refused.body
    );
    // the fold never meets it: the count, which folds the form's fields
    // alone, refuses it as no field of the form
    let mut counted = added.to_vec();
    counted.push(("view.by", "ilvl"));
    let count = web.raw(
        "GET",
        &format!("/{}/count?{}", web.token, query_string(&counted)),
        None,
        &[("accept", "application/json")],
        "",
    );
    assert_eq!(count.status, 400, "{}", count.body);
    assert!(
        count.body.contains("`view.by` is no field of this form"),
        "{}",
        count.body
    );

    // a place page: the same, over the rows its heads are drawn from
    let tab = "tabs?realm=pc&league=Standard&view.value=ilvl&open=t1";
    let page = web.get(tab).body;
    assert_eq!(
        located(&format!("{tab}&view.by=ilvl")),
        head_of(&page, "ilvl")
    );
    let location = located(&format!(
        "tabs?realm=pc&league=Standard&open=t1&view.by={}",
        harness::form_encoded(life)
    ));
    let landed = follow(&web, &location);
    assert_eq!(landed.status, 200, "{}", landed.body);
    assert!(
        landed
            .body
            .contains("<span class=\"mark\" aria-hidden=\"true\">▼</span>"),
        "{}",
        landed.body
    );
    let at = |id: &str| landed.body.find(&format!("data-id=\"{id}\"")).unwrap();
    assert!(
        at("kaom") < at("r1") && at("r1") < at("r2"),
        "{}",
        landed.body
    );
    // past a full list there too: the search's refusal
    let mut full_tab = "tabs?realm=pc&league=Standard&open=t1".to_string();
    for value in full {
        full_tab.push_str(&format!("&view.value={value}"));
    }
    let refused = follow(
        &web,
        &located(&format!(
            "{full_tab}&view.by={}",
            harness::form_encoded(life)
        )),
    );
    assert!(
        refused.status >= 400 && refused.body.contains("a row carries at most 12"),
        "{}: {}",
        refused.status,
        refused.body
    );
    // a place page that draws no item heads refuses it in words: no place
    // open, an open tab of no match, a folder alone open
    let mut store = Store::open(&account_path(&scratch.0.join("mock"), USER)).unwrap();
    let mut record = |ep: Endpoint, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "pc", "league": "Standard" }),
                200,
                &body,
                at,
            )
            .unwrap();
    };
    record(
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "stashes": [
            { "id": "t1", "name": "Rings", "type": "PremiumStash" },
            { "id": "t2", "name": "Launch", "type": "PremiumStash" },
            { "id": "f1", "name": "Old", "type": "Folder", "children": [
                { "id": "o1", "name": "Old rings", "type": "PremiumStash" } ] } ] }),
        40,
    );
    drop(store);
    for path in [
        "tabs?realm=pc&league=Standard&view.by=ilvl",
        "tabs?realm=pc&league=Standard&text=class%3Aboots&open=t1&view.by=ilvl",
        "tabs?realm=pc&league=Standard&open=f1&view.by=ilvl",
    ] {
        let refused = web.get(path);
        assert_eq!(refused.status, 400, "{path}: {}", refused.body);
        assert!(
            refused
                .body
                .contains("no open place on this page lists items"),
            "{path}: {}",
            refused.body
        );
    }
}

/// The switch (`4-full-1-form-rides.md`, Decision 3) is the page's: `search`
/// and `count` take `all_fields=true` beside the form's fields and answer
/// as without it, the page renders it ticked, and no link or hidden field
/// of the page names it, Clear among them; the form page renders it ticked
/// where its URL names it, unticked where not.
#[test]
fn the_switch_is_the_pages_and_no_search_reads_or_carries_it() {
    let (_scratch, web) = offline_web("webswitch");
    let fields = ring_search();
    let mut switched = fields.clone();
    switched.push(("all_fields", "true"));
    let (plain, ticked) = (
        format!("search?{}", query_string(&fields)),
        format!("search?{}", query_string(&switched)),
    );
    assert_eq!(web.get_json(&ticked), web.get_json(&plain));
    let (plain_count, ticked_count) = (
        format!("count?{}", query_string(&fields)),
        format!("count?{}", query_string(&switched)),
    );
    assert_eq!(web.get_json(&ticked_count), web.get_json(&plain_count));
    let page = web.get(&ticked);
    assert_eq!(page.status, 200, "{}", page.body);
    let box_ = "<input type=\"checkbox\" id=\"all-fields\" name=\"all_fields\" value=\"true\" form=\"search-form\" data-page checked>";
    assert!(page.body.contains(box_), "{}", page.body);
    assert_eq!(
        page.body.matches("all_fields").count(),
        1,
        "a link or a field beside the switch names it:\n{}",
        page.body
    );
    // a refused search's page and the form page carry it as their URL names it
    let refused = web.get("search?realm=pc&view.sort=no+such+value&all_fields=true");
    assert_eq!(refused.status, 400, "{}", refused.body);
    assert!(refused.body.contains(box_), "{}", refused.body);
    assert!(web.get("?realm=pc&all_fields=true").body.contains(box_));
    let unticked = web.get("");
    assert!(
        unticked
            .body
            .contains("data-page> Show the fields this search does not run"),
        "{}",
        unticked.body
    );
}

/// Clear (`4-full-1-form-rides.md`, Decision 6): followed from a filled
/// page, a link to the form's page with the scope alone lands on the empty
/// form of that realm and membership, every other field let go.
#[test]
fn clear_lands_on_the_empty_form_of_the_realm_and_membership_shown() {
    let (_scratch, web) = offline_web("webclear");
    let filled = web.get(
        "search?realm=pc&membership=removed&league=Standard&query.term=Doom+Loop\
         &defences=displayed&view.limit=50",
    );
    assert_eq!(filled.status, 200, "{}", filled.body);
    let at = filled
        .body
        .find("<a class=\"clear\" href=\"")
        .unwrap_or_else(|| {
            panic!("no Clear:\n{}", filled.body);
        })
        + "<a class=\"clear\" href=\"".len();
    let href = unescaped(&filled.body[at..at + filled.body[at..].find('"').unwrap()]);
    assert_eq!(href, ".?realm=pc&membership=removed");
    // `.` resolved against the search page, as a browser resolves it
    let cleared = web.get(href.strip_prefix('.').unwrap());
    assert_eq!(cleared.status, 200, "{}", cleared.body);
    let mut empty = submitted(&web.get("").body, SEARCH_FORM);
    for (name, value) in &mut empty {
        if name == "membership" {
            *value = "removed".to_string();
        }
    }
    assert!(
        empty.iter().any(|(name, _)| name == "membership"),
        "{empty:?}"
    );
    assert_eq!(submitted(&cleared.body, SEARCH_FORM), empty);
    // a live scope's Clear names the realm alone
    let live = web.get("search?realm=pc&query.term=Doom+Loop");
    assert!(
        live.body
            .contains("<a class=\"clear\" href=\".?realm=pc\">Clear</a>"),
        "{}",
        live.body
    );
}

#[test]
fn every_search_page_is_a_wording_golden() {
    let (scratch, web) = offline_web("webpages");
    let mut pages = Pages::new(&scratch.0, &web);
    let form = web.get("");
    pages.page(
        "GET <the form>",
        &form,
        &[
            "Item Category",
            "name=\"query.filters.type_filters.filters.category.option\"",
            "Standard (tabs on disk)",
            "Settlers (no tabs on disk)",
            "Fetch the league list",
            "disabled",
            "The seller&#39;s account is the listing&#39;s",
            "<span class=\"fate fate-ambiguous\"",
            "<button type=\"button\" data-add=\"stat-group-crucible\" data-unrun>Crucible Passive Tree Path (unsupported)</button>",
            "data-page> Show the fields this search does not run</label> <span class=\"fate\">A field with no mark runs as the trade site's; a mark says it is read differently here, or not run.</span>",
            "placeholder=\"+ Add Stat Filter\"",
            "<summary>Add Stat Group</summary>",
            "<template id=\"stat-group-weight\">",
            "<summary>Type Filters</summary>",
            "<input type=\"checkbox\" name=\"defences\" value=\"q20\" checked> assume 20% quality",
            "<input type=\"hidden\" name=\"query.filters.armour_filters.disabled\" value=\"true\"><label><input type=\"checkbox\" name=\"query.filters.armour_filters.disabled\" value=\"false\" checked> group on</label>",
            "<input type=\"hidden\" name=\"group.disabled\" value=\"true\"><label><input type=\"checkbox\" name=\"group.disabled\" value=\"false\" checked> group on</label>",
            "<summary>Stat Filters</summary>",
            "placeholder=\"Search items\"",
            "class=\"search-btn\"",
            "A trade site search: its URL, its id or its query JSON",
            "name=\"trade\"",
            "Run the site's search",
        ],
    );
    // a fate is said where it is not exact (`4-full-1-form-rides.md`,
    // Decision 2): no mark, title or label word for an exact one
    for exact in [
        "fate-exact",
        "(exact)",
        "\u{b7} alone exact \u{b7} with a bound exact",
    ] {
        assert!(!form.body.contains(exact), "{exact}: {}", form.body);
    }
    let ring_path = format!("search?{}", query_string(&ring_search()));
    let answer = web.get(&ring_path);
    pages.page(
        "GET search?<the ring search>",
        &answer,
        &[
            "from the query JSON: exact",
            "<th>Total</th><td>2 matches",
            "Doom Loop",
            "Rune Coil",
            "alt=\"Doom Loop Two-Stone Ring\"",
            "class=\"name rarity-rare\"",
            "<p class=\"count\">2 matches</p>",
            "name=\"text\" rows=\"3\">league=Standard category=&quot;accessory.ring&quot;",
            "+95 to maximum Life",
            // the rows are cards by default: a line's kind, the price,
            // the place with its age, and the way to the table
            "<div class=\"cards\">",
            "<span class=\"mod-kind\">explicit</span>",
            "<p class=\"price\">price none (",
            "#i-r1\">pc · Standard / Rings",
            " ago</a></p>",
            "<a class=\"presentation\" href=\"search?",
            ">as a table</a>",
            // the order the rows stand in, where no sort was asked: the
            // store's
            "rows 1–2 of 2 shown, as they lie in their place, top row first and left to right (no sort asked)",
            // the query box's presentation, the big form's
            "Results as <select name=\"view.fields\"><option value=\"\" selected>cards</option>",
        ],
    );
    assert!(!answer.body.contains("class=\"window\""), "{}", answer.body);
    // a form search saves as the site's search, the form's envelope
    // kept as given (C194), and the note says so
    assert!(
        answer.body.contains(
            "<p class=\"save-note\">Saved, this keeps the site&#39;s search as given, \
             translated again each time it opens.</p>"
        ),
        "{}",
        answer.body
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &ring_path, &answer);
    // the same search as the table, asked on the form: every row in one
    // table, its heads the fixed columns and the query's one value, its
    // first block the server's, each row's price in the card's words, the
    // query box keeping it, and the way back to the cards, which answers
    let mut tabled = ring_search();
    tabled.push(("view.fields", "icon"));
    let table_path = format!("search?{}", query_string(&tabled));
    let table = web.get(&table_path);
    assert_eq!(table.status, 200, "{}", table.body);
    assert!(!table.body.contains("class=\"card\""), "{}", table.body);
    pages.page(
        "GET search?<the ring search as a table>",
        &table,
        &[
            "<p class=\"rows-shown\">2 matching items in the table, as they lie in their place, top row first and left to right (no sort asked)",
            "<div class=\"window\" data-row-height=\"24\" data-block-rows=\"50\">",
            "<span>Item</span><span>Base</span><span>Price</span><span>Location</span><span>Fetched</span><a class=\"head\" href=\"search?realm=pc&amp;league=Standard&amp;",
            "/rows?next=acq1.",
            "data-count=\"2\"",
            "<div class=\"block\" data-block=\"0\">",
            "data-i=\"0\" data-id=\"r1\"><button type=\"submit\" class=\"name rarity-rare\" name=\"toggle_item\" value=\"r1\">Doom Loop Two-Stone Ring</button>",
            "<span>none (",
            "<span>Standard / Rings · x 0, y 0</span>",
            "<span>95</span>",
            "<option value=\"icon\" selected>a table</option>",
            ">as cards</a>",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &table_path, &table);
    let link = "<a class=\"presentation\" href=\"";
    let back = &table.body[table.body.find(link).unwrap() + link.len()..];
    let back = back[..back.find('"').unwrap()].replace("&amp;", "&");
    assert!(!back.contains("view.fields"), "{back}");
    let cards = web.get(&back);
    assert_eq!(cards.status, 200, "{}", cards.body);
    assert!(
        cards.body.contains("<div class=\"cards\">"),
        "{}",
        cards.body
    );
    // the query box: the answer holds the text it ran, to edit; an edited
    // text runs as the user's own query, the trade input dropped — the
    // translation's section on the ring search's answer, none on the edit
    assert!(answer.body.contains("name=\"text\""), "{}", answer.body);
    assert!(answer.body.contains("class=\"trade\""), "{}", answer.body);
    let edited_path = format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("text", "class:rings \"# to maximum Life\">=90")
        ])
    );
    let edited = web.get(&edited_path);
    assert_eq!(edited.status, 200, "{}", edited.body);
    assert!(!edited.body.contains("class=\"trade\""), "{}", edited.body);
    pages.page(
        "GET search?<an edited text>",
        &edited,
        &[
            "<th>Query</th><td>class:rings line(&quot;# to maximum Life&quot; arg1&gt;=90)",
            "<th>Total</th><td>1 match",
            "Doom Loop",
            "The query, in the language",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &edited_path, &edited);
    // the trade box (#188): a URL, its fidelity and its clauses; a URL
    // naming another realm than the page's; the query JSON with the
    // site's sort, reported and not applied; the two boxes together
    let url = crate::search_json::trade_url();
    let traded_path = format!(
        "search?{}",
        query_string(&[("realm", "pc"), ("trade", url.as_str())])
    );
    let traded = web.get(&traded_path);
    assert_eq!(traded.status, 200, "{}", traded.body);
    pages.page(
        "GET search?<a trade site search's URL>",
        &traded,
        &[
            "not the site&#39;s search: 2 clauses not run",
            "from a URL: PARTIAL",
            // what a save of it keeps: the site's search (C194)
            "<p class=\"save-note\">Saved, this keeps the site&#39;s search as given, translated again each time it opens.</p>",
            "<td>NOT RUN</td><td>stats.1</td>",
            "<td>reported</td><td>status</td>",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &traded_path, &traded);
    let elsewhere = format!(
        "https://www.pathofexile.com/trade/search/xbox/Hardcore/{}",
        crate::search_json::TRADE_ID
    );
    let moved_path = format!(
        "search?{}",
        query_string(&[("realm", "pc"), ("trade", elsewhere.as_str())])
    );
    let moved = web.get(&moved_path);
    assert_eq!(moved.status, 200, "{}", moved.body);
    // a zero answer's offers, which no page asks: each in words, no link
    pages.page(
        "GET search?<an xbox URL from the pc page>",
        &moved,
        &[
            "names its realm, xbox: it ran there, not in pc",
            "<span class=\"way\" data-way=\"/zero/offers/explain/0\">part ",
            "<span class=\"way\" data-way=\"/zero/offers/explain/1\">part ",
            "<span class=\"way\" data-way=\"/zero/offers/context\">every item in scope, this query set aside</span>",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &moved_path, &moved);
    let sorted_path = format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            (
                "trade",
                r#"{"query":{"filters":{"type_filters":{"filters":{"category":{"option":"accessory.ring"}}}}},"sort":{"price":"asc"}}"#
            ),
        ])
    );
    let sorted = web.get(&sorted_path);
    assert_eq!(sorted.status, 200, "{}", sorted.body);
    pages.page(
        "GET search?<the site's query JSON with its sort>",
        &sorted,
        &[
            "from the query JSON: exact",
            "<td>reported</td><td>sort</td><td>The site&#39;s sort is reported and not applied",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &sorted_path, &sorted);
    let both = web.get(&format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("text", "class:rings"),
            ("trade", url.as_str())
        ])
    ));
    assert_eq!(both.status, 400, "{}", both.body);
    pages.page(
        "GET search?<both boxes>",
        &both,
        &["not both: run one box at a time"],
    );
    let counted_path = format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("league", "Standard"),
            ("view.count", "rarity"),
            ("view.limit", "20"),
        ])
    );
    let counted = web.get(&counted_path);
    assert_eq!(counted.status, 200, "{}", counted.body);
    // a count shows no rows: its tables, no rows table, no card and no
    // other presentation
    for none in [
        "class=\"window\"",
        "class=\"card\"",
        "class=\"presentation\"",
    ] {
        assert!(!counted.body.contains(none), "{none}: {}", counted.body);
    }
    assert!(
        counted.body.contains("class=\"counted\""),
        "{}",
        counted.body
    );
    pages.page(
        "GET search?<the league counted by rarity>",
        &counted,
        &[
            "Count rarity · 2 values",
            "<td>4</td><td>rare</td>",
            "<td>1</td><td>unique</td>",
            // a save leaves the count behind, as the editors carry it
            "Saved without its count: a saved search is its query and its scope.",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &counted_path, &counted);
    // rows left out: the continuation, a link that answers the next rows
    let first_path = "search?realm=pc&text=class%3Arings&view.limit=1";
    let first = web.get(first_path);
    assert_eq!(first.status, 200, "{}", first.body);
    pages.page(
        "GET search?<every ring, one row shown>",
        &first,
        &[
            // where no sort was asked, the way to one
            "rows 1–1 of 3 shown, as they lie in their place, top row first and left to right (no sort asked) · press a number on a card to order the rows by it · <a class=\"way\" data-way=\"/view/rows/next\" href=\"/",
            "search?next=acq1.",
            "\">more</a>",
            "data-way=\"/terms/0/undecided\" href=",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, first_path, &first);
    // the page "more" reaches keeps the other way and has the way back to
    // the first rows, each answering
    let href = |body: &str, before: &str| {
        let at = body[..body.find(before).unwrap()].rfind("href=\"").unwrap() + 6;
        body[at..at + body[at..].find('"').unwrap()]
            .replace("&amp;", "&")
            .trim_start_matches(&format!("/{}/", web.token))
            .to_string()
    };
    let more_path = href(&first.body, "\">more</a>");
    let more = web.get(&more_path);
    assert_eq!(more.status, 200, "{}", more.body);
    pages.page(
        "GET search?<every ring, the second row>",
        &more,
        &[
            "rows 2–2 of 3 shown, as they lie in their place, top row first and left to right (no sort asked)",
            "\">first rows</a>",
            ">as a table</a>",
        ],
    );
    let back = web.get(&href(&more.body, "\">first rows</a>"));
    assert_eq!(back.status, 200, "{}", back.body);
    assert!(back.body.contains("rows 1–1 of 3 shown"), "{}", back.body);
    assert!(!back.body.contains("first rows"), "{}", back.body);
    let tabled = web.get(&href(&more.body, "\">as a table</a>"));
    assert_eq!(tabled.status, 200, "{}", tabled.body);
    // the table holds every row, the one shown by the cards' limit and
    // the rest alike
    assert!(
        tabled.body.contains("3 matching items in the table"),
        "{}",
        tabled.body
    );
    assert!(
        tabled.body.contains("<div class=\"window\""),
        "{}",
        tabled.body
    );
    // a count whose undecided bucket carries its route by default: the
    // chalice whose base the class table lacks
    let classed_path = "search?realm=pc&view.count=class";
    let classed = web.get(classed_path);
    assert_eq!(classed.status, 200, "{}", classed.body);
    pages.page(
        "GET search?<every item counted by class>",
        &classed,
        &[
            "Count class · ",
            "data-way=\"/view/counts/tables/0/buckets/2\" href=",
            ">1</a></td><td>(undecided) · ",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, classed_path, &classed);
    // a zero answer whose one candidate is undecided: its undecided in the
    // zero block, a way, beside the total's
    let zero_path = "search?realm=pc&text=class%3Arings+name%3D%22Brood+Lure%22";
    let zero = web.get(zero_path);
    assert_eq!(zero.status, 200, "{}", zero.body);
    assert_eq!(web.get_json(zero_path)["total"]["matched"], 0);
    pages.page(
        "GET search?<rings named as the undecided chalice is>",
        &zero,
        &[
            "<p class=\"count\">0 matches</p>",
            "<p>No match · <a class=\"way\" data-way=\"/zero/undecided\" href=",
            "data-way=\"/total/undecided\" href=",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, zero_path, &zero);
    let item = web.get("item/r1");
    pages.page(
        "GET item/r1",
        &item,
        &[
            "Doom Loop Two-Stone Ring",
            "<th>Class</th><td>Rings</td>",
            "+95 to maximum Life",
            "<p class=\"price\">price none (",
            ART,
        ],
    );
    // the card's price says the price or why there is none, and nothing
    // after: no command and no link
    let price = &item.body[item.body.find("<p class=\"price\">").unwrap()..];
    let price = &price[..price.find("</p>").unwrap()];
    assert!(!price.contains("why") && !price.contains("href"), "{price}");
    let refused = web.get("search?view.sort=no%20such%20value");
    assert_eq!(refused.status, 400);
    pages.page("GET search?view.sort=no such value", &refused, &["refusal"]);
    // an opened row's card links the item against the answer's query,
    // its canonical text: the item page under `against`, whose JSON is
    // `acq show <id> --against`'s
    let opened_path =
        "search?realm=pc&text=class%3Arings%20%20ilvl%3E%3D1&view.fields=icon&item=r1";
    let canonical = web.get_json(opened_path)["query"]["text"]
        .as_str()
        .unwrap()
        .to_string();
    let opened = web.get(opened_path);
    let card = &opened.body[opened.body.find("<div class=\"opened\">").unwrap()..];
    let tag = &card[card.find("<a class=\"against\"").unwrap()..];
    let href = attribute(tag, "href").unwrap();
    assert_eq!(
        href,
        format!(
            "item/r1?against={}",
            canonical
                .replace(' ', "%20")
                .replace(':', "%3A")
                .replace('>', "%3E")
                .replace('=', "%3D")
        )
    );
    let against = web.get(&href);
    pages.page(
        "GET item/r1?against=<the answer's query>",
        &against,
        &[
            "Doom Loop Two-Stone Ring",
            "<p class=\"against\">",
            "<table class=\"against\">",
        ],
    );
    let shown = offline_acq(
        &scratch.0,
        &["show", "r1", "--against", &canonical, "--json"],
    );
    assert!(
        shown.status.success(),
        "{}",
        String::from_utf8_lossy(&shown.stderr)
    );
    let shown: Value = serde_json::from_slice(&shown.stdout).unwrap();
    assert_eq!(web.get_json(&href), shown);
    // a refused table query keeps its box (#190): the refusal, then the
    // text it refused and the view as sent, so it can be fixed and run
    let refused_table = web.get(
        "search?realm=pc&text=class%3Arings%20sort%3Alife&view.fields=icon&view.sort=ilvl&view.desc=true&view.value=ilvl&view.value=pseudo.total_life",
    );
    assert_eq!(refused_table.status, 400, "{}", refused_table.body);
    pages.page(
        "GET search?<a refused table query, an order and values named>",
        &refused_table,
        &[
            "<div class=\"refusal\">",
            "name=\"text\" rows=\"3\">class:rings sort:life</textarea>",
            "<input type=\"hidden\" name=\"view.sort\" value=\"ilvl\"><input type=\"hidden\" name=\"view.desc\" value=\"true\">",
            "<input type=\"hidden\" name=\"view.value\" value=\"ilvl\"><input type=\"hidden\" name=\"view.value\" value=\"pseudo.total_life\">",
            "<option value=\"icon\" selected>a table</option>",
        ],
    );
    let refused_doc = refused_json(
        &web,
        "search?realm=pc&text=class%3Arings%20sort%3Alife&view.fields=icon&view.sort=ilvl",
        400,
    );
    assert!(refused_doc["error"].is_string(), "{refused_doc}");
    // what a save leaves behind, said by the save form: a sorted table's
    // order and table, a table's table alone
    for (path, said, line) in [
        (
            "search?realm=pc&text=class%3Arings&view.fields=icon&view.sort=ilvl&view.desc=true",
            "Saved without its order, its columns and the table: a saved search is its query and its scope.",
            "3 matching items in the table, by ilvl, largest first, as asked</p>",
        ),
        (
            "search?realm=pc&text=class%3Arings&view.fields=icon",
            "Saved without the table: a saved search is its query and its scope.",
            // where no sort was asked, the way to one
            "3 matching items in the table, as they lie in their place, top row first and left to right (no sort asked) · open an item by its name and press a number on its card to order the rows by it</p>",
        ),
    ] {
        let saving = web.get(path);
        pages.page(
            &format!("GET {path}"),
            &saving,
            &[&format!("<p class=\"save-note\">{said}</p>"), line],
        );
    }
    let cards = web.get("search?realm=pc&text=class%3Arings");
    assert!(!cards.body.contains("Saved without"), "{}", cards.body);
    // a count asked of no realm over a store of two: each realm offered
    // keeps the count, and each link answers it — a key in another case
    // and a narrowing of two texts among them, which fold back through the
    // form's one count field
    for (asked, spelled, said) in [
        ("rarity", "rarity", "Count rarity"),
        ("Rarity", "rarity", "Count rarity"),
        (
            "line%3Aresist%2Clife",
            "line%3Aresist%2Cline%3Alife",
            "Count line:resist",
        ),
    ] {
        let counted = web.get(&format!("search?text=class%3Arings&view.count={asked}"));
        assert_eq!(counted.status, 400, "{asked}: {}", counted.body);
        let links = offered_links(&counted.body);
        assert_eq!(links.len(), 3, "{asked}: {}", counted.body);
        for (link, realm) in links.iter().zip(["pc", "xbox", "all"]) {
            assert!(
                link.ends_with(&format!(
                    "search?realm={realm}&text=class%3Arings&view.count={spelled}"
                )),
                "{asked}: {link}"
            );
            let answered = web.raw("GET", link, None, &[], "");
            assert_eq!(answered.status, 200, "{link}: {}", answered.body);
            assert!(answered.body.contains(said), "{}", answered.body);
            every_request_an_answer_carries_is_a_way_on_its_page(&web, link, &answered);
        }
    }
    // a tab's id: the refusal's way on is the page's own, a link to the
    // search the form asks, never a command line; followed, it answers
    let tab = web.get("item/t1");
    assert_eq!(tab.status, 400, "{}", tab.body);
    assert!(!tab.body.contains("acq "), "{}", tab.body);
    pages.page(
        "GET item/t1",
        &tab,
        &[
            "not an item",
            "class=\"offers\"",
            "search?realm=pc&amp;text=id%3At1\">the search of what is in the tab `Rings`</a>",
        ],
    );
    let asked = web.raw(
        "GET",
        &format!("/{}/item/t1", web.token),
        None,
        &[("Accept", "application/json")],
        "",
    );
    let offered: Value = serde_json::from_str(&asked.body).unwrap();
    let link = offered["offers"][0]["request"].as_str().unwrap();
    let listed = web.raw("GET", link, None, &[], "");
    assert_eq!(listed.status, 200, "{link}: {}", listed.body);
    assert!(listed.body.contains("Doom Loop"), "{}", listed.body);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, link, &listed);
    // a site search whose ranges run backwards matches nothing, and the
    // page says which clause and why, the term under `if` included
    let inverted = format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            (
                "trade",
                r#"{"query": {"stats": [{"type": "if", "filters": [{"id": "implicit.stat_3299347043", "value": {"min": 5, "max": 0}}]}], "filters": {"misc_filters": {"filters": {"ilvl": {"min": 85, "max": 80}}}}}}"#
            )
        ])
    );
    let nothing = web.get(&inverted);
    assert_eq!(nothing.status, 200, "{}", nothing.body);
    pages.page(
        &format!("GET {inverted}"),
        &nothing,
        &[
            "<td>nothing</td><td>misc_filters.ilvl</td><td>its minimum 85 is above its maximum 80",
            "<td>nothing</td><td>stats.0.0</td><td>its minimum 5 is above its maximum 0",
        ],
    );
    // two stat groups from the form, the second a weighted sum with its
    // bound: the page stands filled with both, their rows, the weights and
    // the bound (`4-full-1-form-rows.md`)
    let grouped = "search?realm=pc&league=Standard\
        &group.type=and&stat.id=explicit.stat_3299347043&stat.min=40\
        &group.type=weight&group.min=100&stat.id=explicit.stat_3299347043&stat.weight=1\
        &stat.id=implicit.stat_3299347043&stat.weight=1";
    let two = web.get(grouped);
    assert_eq!(two.status, 200, "{}", two.body);
    pages.page(
        "GET search?<two stat groups, the second weighted>",
        &two,
        &[
            "<option value=\"weight\" data-takes=\"group.min group.max stat.min stat.max stat.option stat.weight\" selected>",
            "name=\"group.min\" value=\"100\"",
            "name=\"stat.weight\" value=\"1\"",
            "name=\"stat.min\" value=\"40\"",
            "<p class=\"count\">1 match</p>",
        ],
    );
    // a form whose defences are read as the item stands: the quality box
    // unticked, its panel open (`4-full-1-form-rides.md`)
    // and its rows' fates said where not exact (Decision 2): a stat exact
    // both ways by its group word alone, one not exact with both statuses
    // and its Clear carrying the removed membership (Decision 6)
    // and the switch ticked by its URL (Decision 3)
    let displayed = "search?realm=pc&membership=removed&league=Standard&defences=displayed&all_fields=true\
        &group.type=and&stat.id=explicit.stat_3299347043&stat.id=explicit.stat_983749596";
    let unticked = web.get(displayed);
    assert_eq!(unticked.status, 200, "{}", unticked.body);
    pages.page(
        "GET search?<the defences as the item stands, two rows>",
        &unticked,
        &[
            "<input type=\"hidden\" name=\"defences\" value=\"displayed\"><label><input type=\"checkbox\" name=\"defences\" value=\"q20\"> assume 20% quality",
            "<details class=\"panel\" open><summary>Armour Filters</summary>",
            "<span class=\"stat-text\">+# to maximum Life</span>",
            "<span class=\"fate\">explicit</span>",
            "<span class=\"fate\">explicit \u{b7} alone exact \u{b7} with a bound ambiguous</span>",
            "<a class=\"clear\" href=\".?realm=pc&amp;membership=removed\">Clear</a>",
            "form=\"search-form\" data-page checked> Show the fields this search does not run",
        ],
    );
    // a site search its decode refused stands as sent in the bar's site
    // box (`4-full-1-form-rides.md`, Decision 5)
    let undecoded = format!(
        "search?{}",
        query_string(&[("realm", "pc"), ("trade", "not an id")])
    );
    pages.page(
        "GET search?<a site search the decode refuses>",
        &web.get(&undecoded),
        &["aria-label=\"A trade site search: its URL, its id or its query JSON\">not an id</textarea>"],
    );
    // a finder's listing: each option holding its row's template, the
    // vocabulary document where JSON is asked
    let listing = web.get("stats?q=pseudo+maximum+life");
    assert_eq!(listing.status, 200, "{}", listing.body);
    pages.page(
        "GET stats?q=pseudo maximum life",
        &listing,
        &[
            "<li role=\"option\" data-add>+# total maximum Life",
            "<template><div class=\"filter stat-row\" data-item><input type=\"hidden\" name=\"stat.id\" value=\"pseudo.pseudo_total_life\">",
        ],
    );
    let listing = web.get("stats?q=increased+maximum+life");
    assert_eq!(listing.status, 200, "{}", listing.body);
    pages.page(
        "GET stats?q=increased maximum life",
        &listing,
        &["<li role=\"option\" data-add>#% increased maximum Life <span class=\"fate\">explicit \u{b7} alone exact \u{b7} with a bound ambiguous</span>"],
    );
    let vocabulary = web.get_json("stats?q=pseudo+maximum+life");
    assert!(vocabulary["stats"].is_string(), "{vocabulary}");
    assert_eq!(vocabulary["left_out"], 0, "{vocabulary}");
    let entry = &vocabulary["entries"][0];
    assert_eq!(entry["id"], "pseudo.pseudo_total_life", "{vocabulary}");
    for key in ["texts", "group", "alone", "bounded"] {
        assert!(entry.get(key).is_some(), "{key}: {vocabulary}");
    }
    // the same route asked for values: the value finder's options, each a
    // submit button sending the value's text, and the listing as JSON
    let listing = web.get("stats?values=true&q=item+level");
    assert_eq!(listing.status, 200, "{}", listing.body);
    pages.page(
        "GET stats?values=true&q=item level",
        &listing,
        &["<li><button type=\"submit\" role=\"option\" name=\"view.by\" value=\"ilvl\">Item Level <span class=\"fate\">filter</span></button></li>"],
    );
    let values = web.get_json("stats?values=true&q=total+elemental+resistance");
    assert_eq!(
        values["entries"][0],
        json!({
            "id": "pseudo.pseudo_total_elemental_resistance",
            "texts": ["+#% total Elemental Resistance"],
            "group": "pseudo",
            "value": "pseudo.total_ele_res",
        }),
        "{values}"
    );
    for key in ["stats", "totals", "catalogue", "left_out"] {
        assert!(values.get(key).is_some(), "{key}: {values}");
    }
    let refused = web.get("stats?values=yes&q=life");
    assert_eq!(refused.status, 400, "{}", refused.body);
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "search_pages", &pages);
}

/// The form is one realm's render: asked for the console realm, it lists
/// that realm's league alone and its search carries that realm. A server
/// started with no account on disk serves: every page is the
/// `not_logged_in` refusal with its remedy, a page asked as JSON its
/// document, the stylesheet still served; once an account is on disk, a
/// reload serves its form. A server asked for an account by name over the
/// same empty world does not start: it refuses as `not_logged_in` before
/// the bind.
#[test]
fn the_form_renders_the_realm_it_is_asked_for() {
    let scratch = harness::scratch("webrealm");
    let named = offline_acq(
        &scratch.0,
        &["--account", "Typo", "web", "--no-browser", "--json"],
    );
    assert_eq!(named.status.code(), Some(1), "{named:?}");
    let refused = sole_json(&named);
    assert_eq!(refused["kind"], "not_logged_in", "{refused}");
    let web = start_web(offline_command(&scratch.0, &["web", "--no-browser"]));
    let mut pages = Pages::new(&scratch.0, &web);
    let none = web.get("");
    pages.page(
        "GET <no account on disk>",
        &none,
        &["no account&#39;s store in", "log in with `acq auth`"],
    );
    // the nav bar says no account is served yet
    assert!(
        none.body
            .contains("<span class=\"account\">no account yet</span>"),
        "{}",
        none.body
    );
    let jobs = web.raw(
        "GET",
        &format!("/{}/jobs", web.token),
        None,
        &[("Accept", "application/json")],
        "",
    );
    let refused: Value = serde_json::from_str(&jobs.body).unwrap();
    assert_eq!(refused["kind"], "not_logged_in", "{refused}");
    assert_eq!(refused["accounts"], json!([]), "{refused}");
    assert_eq!(web.get("assets/page.css").status, 200);
    seed(&scratch.0);
    let form = web.get("?realm=xbox");
    assert!(
        form.body
            .contains("<span class=\"account\">account Alice#1234</span>"),
        "{}",
        form.body
    );
    // the league list is the account's, across realms; whether a league
    // has tabs on disk is the realm's
    assert!(
        form.body.contains("Standard (no tabs on disk)"),
        "{}",
        form.body
    );
    pages.page(
        "GET ?realm=xbox",
        &form,
        &[
            "<option value=\"xbox\" selected>",
            "Show this realm's leagues",
            "name=\"realm\" value=\"xbox\"",
            "Hardcore (tabs on disk)",
        ],
    );
    // a league with no tabs: the stash tabs page's facts line, an empty
    // listing apart from one never seen, and the listing where the table
    // would be
    let mock = scratch.0.join("mock");
    Store::open(&account_path(&mock, USER))
        .unwrap()
        .record(
            &Endpoint::Stashes {
                realm: "pc".into(),
                league: "Empty".into(),
            },
            &json!({ "league": "Empty" }),
            200,
            &json!({ "stashes": [] }),
            50,
        )
        .unwrap();
    for (league, said) in [
        ("Empty", "facts: no tabs; stash listing "),
        ("Nowhere", "facts: no tabs; stash never listed"),
    ] {
        let page = web.get(&format!("tabs?realm=pc&league={league}"));
        pages.page(
            &format!("GET tabs?realm=pc&league={league}"),
            &page,
            &[
                &format!("No stash tabs of {league} are on record."),
                said,
                "List this league&#39;s stash tabs",
            ],
        );
    }
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "console_realm", &pages);
}

/// What a new league brings first renders
/// with a badge, never a refusal or an empty page — a base the class
/// table lacks, undecided with its reason; a mechanic's line whose flag
/// is no yes or no; and a site search naming a stat the table lacks,
/// answered as not the site's search. Each badge renders a state the
/// document carries, pinned in the search crate (`moved_world.rs`).
#[test]
fn the_moved_world_renders_with_a_badge() {
    let (scratch, web) = offline_web("webmoved");
    let mut pages = Pages::new(&scratch.0, &web);
    let new = web.get("item/new");
    assert_eq!(new.status, 200);
    pages.page(
        "GET item/new",
        &new,
        &["class undecided", "Wombgift Chalice", "+40 to maximum Life"],
    );
    let mech = web.get("item/mech");
    assert_eq!(mech.status, 200);
    pages.page(
        "GET item/mech",
        &mech,
        &["flags unknown", "echoing", "Grants 3 Echoes of the Brood"],
    );
    let untranslated_path = format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("group.type", "and"),
            ("stat.id", "explicit.stat_9999999999"),
            ("view.limit", "20"),
        ])
    );
    let untranslated = web.get(&untranslated_path);
    assert_eq!(untranslated.status, 200);
    pages.page(
        "GET search?<a stat the table lacks>",
        &untranslated,
        &["not the site&#39;s search", "PARTIAL", "stats.0.0"],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &untranslated_path, &untranslated);
    let undecided_path = format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            (
                "query.filters.type_filters.filters.category.option",
                "accessory.ring"
            ),
        ])
    );
    let undecided = web.get(&undecided_path);
    pages.page(
        "GET search?<every ring>",
        &undecided,
        &["undecided", "Wombgift Chalice"],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &undecided_path, &undecided);
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "moved_world", &pages);
}

/// A way on as a page marks it: the pointer its `data-way` names, and its
/// link where it is one.
#[derive(Debug)]
struct PageWay {
    at: String,
    href: Option<String>,
}

/// Every element a page marks `class="way"`, in order.
fn ways_on(body: &str) -> Vec<PageWay> {
    body.match_indices("class=\"way\"")
        .map(|(at, _)| {
            let start = body[..at].rfind('<').unwrap();
            let end = at + body[at..].find('>').unwrap();
            let tag = format!(" {}", &body[start + 1..end]);
            PageWay {
                at: attribute(&tag, "data-way").unwrap_or_else(|| panic!("no data-way: {tag}")),
                href: attribute(&tag, "href"),
            }
        })
        .collect()
}

/// The JSON pointer of every object in `doc` holding a `request`, a site
/// search's clause apart (its `site` is the site's own JSON), never
/// looking inside a request.
fn holding_requests(doc: &Value, at: &str, found: &mut Vec<String>) {
    match doc {
        Value::Object(map) => {
            if map.contains_key("request") {
                found.push(at.to_string());
            }
            for (key, child) in map {
                if key == "request" || (key == "site" && at.contains("/trade/clauses/")) {
                    continue;
                }
                holding_requests(child, &format!("{at}/{key}"), found);
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                holding_requests(child, &format!("{at}/{i}"), found);
            }
        }
        _ => {}
    }
}

/// Whether an answer's scope points at never-fetched tabs, read from its
/// JSON as the search crate's `tabs_pointer` reads it: under one realm, a
/// never-fetched league the query does not exclude and, where the query
/// names leagues (`league=` and-ed or or-ed), one of those named.
fn tabs_pointed(answer: &Value) -> bool {
    // each `league=` term with whether a `not` holds it
    fn terms(t: &Value, excluded: bool, found: &mut Vec<(String, bool)>) {
        if t["field"]
            .as_str()
            .is_some_and(|f| f.eq_ignore_ascii_case("league"))
            && t["op"] == "="
            && let Some(league) = t["value"].as_str()
        {
            found.push((league.to_lowercase(), excluded));
        }
        for key in ["all", "any"] {
            for child in t[key].as_array().into_iter().flatten() {
                terms(child, excluded, found);
            }
        }
        if !t["not"].is_null() {
            terms(&t["not"], true, found);
        }
    }
    if answer["scope"]["realm"] == "all" {
        return false;
    }
    let mut found = Vec::new();
    terms(&answer["query"]["tree"], false, &mut found);
    let named = found.iter().any(|(_, excluded)| !excluded);
    answer["scope"]["never_fetched_leagues"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_lowercase)
        .any(|league| {
            let has = |excluded: bool| found.iter().any(|(l, e)| *l == league && *e == excluded);
            !has(true) && (!named || has(false))
        })
}

/// The value lists' rests no page's count field reads, by their key: a
/// rest of these is said in words. None yet: every rest a page reaches is
/// a count the form asks.
const RESTS_NO_FORM_READS: [&str; 0] = [];

/// Every request an answer carries is a way on its page: for each object
/// of the page's document holding a
/// `request`, exactly one element marked `class="way"` names its pointer,
/// and none names another; the scope's never-fetched tabs a way exactly
/// where the answer points at them. Each link answers `200`: the tabs'
/// the refresh page, and every other with JSON asked the answer to its
/// request — the members counted where its holder has a count, the next
/// rows where it is the continuation, and the request's query, realm and
/// membership. Each way in words is a zero answer's offer, which no page
/// asks, or a rest no page's count field reads.
fn every_request_an_answer_carries_is_a_way_on_its_page(
    web: &WebUi,
    path: &str,
    page: &harness::Reply,
) {
    assert_eq!(page.status, 200, "{path}: {}", page.body);
    // a path as a test asks it, or absolute as a page links it
    let doc = match path.starts_with('/') {
        true => json_at(web, path),
        false => web.get_json(path),
    };
    let mut holders = Vec::new();
    holding_requests(&doc, "", &mut holders);
    let ways = ways_on(main_of(&page.body));
    for holder in &holders {
        assert_eq!(
            ways.iter().filter(|w| w.at == *holder).count(),
            1,
            "{path}: the request at {holder} is not one way on the page: {ways:?}"
        );
    }
    for way in &ways {
        assert!(
            way.at == "tabs" || holders.contains(&way.at),
            "{path}: the way {way:?} names no request of the document {holders:?}"
        );
    }
    assert_eq!(
        ways.iter().filter(|w| w.at == "tabs").count(),
        usize::from(tabs_pointed(&doc)),
        "{path}: the never-fetched tabs' way: {ways:?}"
    );
    for way in &ways {
        let Some(href) = &way.href else {
            let rest = way.at.ends_with("/resolved/rest")
                && RESTS_NO_FORM_READS.iter().any(|key| {
                    doc.pointer(&format!("{}/request/view/counts/keys/0", way.at))
                        .and_then(Value::as_str)
                        == Some(key)
                });
            let offer = way.at == "/zero/offers/context"
                || way
                    .at
                    .strip_prefix("/zero/offers/explain/")
                    .is_some_and(|i| i.parse::<usize>().is_ok());
            assert!(
                offer || rest,
                "{path}: {way:?} is words where a page takes it"
            );
            continue;
        };
        assert!(
            href.starts_with(&format!("/{}/", web.token)),
            "{path}: {href}"
        );
        if way.at == "tabs" {
            let tabs = follow(web, href);
            assert_eq!(tabs.status, 200, "{href}: {}", tabs.body);
            continue;
        }
        let reply = web.raw("GET", href, None, &[("Accept", "application/json")], "");
        assert_eq!(reply.status, 200, "{path}: {href}: {}", reply.body);
        let answer: Value = serde_json::from_str(&reply.body).unwrap();
        let holder = doc.pointer(&way.at).unwrap();
        if let Some(count) = holder.get("count") {
            assert_eq!(
                &answer["total"]["matched"], count,
                "{path}: {} returns other than the members counted",
                way.at
            );
        }
        if way.at == "/view/rows/next" {
            let rows = &doc["view"]["rows"];
            let after = rows["after"].as_u64().unwrap_or(0) + rows["returned"].as_u64().unwrap();
            assert_eq!(answer["view"]["rows"]["after"], after, "{path}: {href}");
        }
        let request = &holder["request"];
        if let Some(text) = request.pointer("/query/text") {
            assert_eq!(&answer["query"]["text"], text, "{path}: {}", way.at);
        }
        if let Some(realm) = request.pointer("/scope/realm") {
            assert_eq!(&answer["scope"]["realm"], realm, "{path}: {}", way.at);
        }
        assert_eq!(
            answer["scope"]["membership"],
            request
                .pointer("/scope/membership")
                .cloned()
                .unwrap_or(json!("live")),
            "{path}: {}",
            way.at
        );
    }
}

/// C143 on the answer and item pages: every page headed `GET search` or
/// `GET item/` that answered in a golden under `tests/goldens/web/` holds
/// no `acq ` and no ` --` before a letter — the ways on are the page's
/// own. Read from the files, as `no_ids.rs` reads them: the pages served
/// are the goldens'. A place page (`GET tabs`, `GET characters`) is read
/// too, and holds no `pre` either (#182); so is every answered plan,
/// apply and jobs page (`GET plan`, `POST apply`, `GET jobs`), which
/// outside attribute values also names none of the daemon's words:
/// `rails`, `tripwire`, `prio` and `eta` as words, `web:`, `cli:` and
/// `-limit` anywhere.
#[test]
fn an_answer_item_or_place_page_prints_no_command_line_and_no_flag() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/web");
    let mut read = 0;
    let mut places = 0;
    let mut documents = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let file = entry.unwrap().path();
        let text = std::fs::read_to_string(&file).unwrap();
        // a recording's label follows a blank line, or opens the file
        let mut pages: Vec<Vec<&str>> = Vec::new();
        let mut blank = true;
        for line in text.lines() {
            if blank && (line.starts_with("GET ") || line.starts_with("POST ")) {
                pages.push(Vec::new());
            }
            if let Some(page) = pages.last_mut() {
                page.push(line);
            }
            blank = line.is_empty();
        }
        for page in pages {
            let label = page[0];
            let answered = !page.iter().any(|l| l.starts_with("--- status"));
            let place = label.starts_with("GET tabs") || label.starts_with("GET characters");
            let document = label.starts_with("GET plan")
                || label.starts_with("POST apply")
                || label.starts_with("GET jobs");
            if !(label.starts_with("GET search")
                || label.starts_with("GET item/")
                || place
                || document)
                || !answered
            {
                continue;
            }
            read += 1;
            places += usize::from(place);
            documents += usize::from(document);
            // the page itself: a `# holds` line quotes a fact, not the page
            for line in page[1..].iter().filter(|l| !l.starts_with("# holds")) {
                let flag = line.match_indices(" --").any(|(at, _)| {
                    line[at + 3..]
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_alphabetic())
                });
                assert!(
                    !line.contains("acq ")
                        && !flag
                        && !((place || document) && line.contains("<pre")),
                    "{}: {label}: {line}",
                    file.display()
                );
                if document {
                    let said = outside_attribute_values(line);
                    let words: Vec<&str> =
                        said.split(|c: char| !c.is_ascii_alphanumeric()).collect();
                    for word in ["rails", "tripwire", "prio", "eta"] {
                        assert!(
                            !words.iter().any(|w| w.eq_ignore_ascii_case(word)),
                            "{}: {label}: {word}: {line}",
                            file.display()
                        );
                    }
                    for part in ["web:", "cli:", "-limit"] {
                        assert!(
                            !said.contains(part),
                            "{}: {label}: {part}: {line}",
                            file.display()
                        );
                    }
                }
            }
        }
    }
    assert!(read > 20, "only {read} answer and item pages read");
    assert!(places >= 6, "only {places} place pages read");
    assert!(
        documents >= 13,
        "only {documents} plan, apply and jobs pages read"
    );
}

/// `line` with every attribute value inside a tag emptied: what a reader
/// reads of it, and the tags' names.
fn outside_attribute_values(line: &str) -> String {
    let mut out = String::new();
    let (mut tag, mut quoted) = (false, false);
    for c in line.chars() {
        match c {
            '<' if !quoted => tag = true,
            '>' if !quoted => tag = false,
            '"' if tag => {
                quoted = !quoted;
                continue;
            }
            _ => {}
        }
        if !quoted {
            out.push(c);
        }
    }
    out
}

/// The ways no seeded page reaches are a wording golden: a world staged
/// apart from `seed`'s pages, each page
/// recorded after the staging it needs and checked as every answer page
/// is ([`every_request_an_answer_carries_is_a_way_on_its_page`]) — tabs
/// never fetched in two pc leagues, the scope pointing at the league
/// holding the most and counting the other, and at the other where the
/// query excludes the first; an id that left, with its way under
/// membership `all`; a value list cut with its rest; and an ambiguous
/// clause of a site search with what its other reading drops.
#[test]
fn the_ways_no_seeded_page_reaches_are_a_wording_golden() {
    let (scratch, web) = offline_web("webways");
    let mut pages = Pages::new(&scratch.0, &web);
    let mut store = Store::open(&account_path(&scratch.0.join("mock"), USER)).unwrap();
    let mut record = |ep: Endpoint, league: &str, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "pc", "league": league }),
                200,
                &body,
                at,
            )
            .unwrap();
    };
    let listing = |league: &str| Endpoint::Stashes {
        realm: "pc".into(),
        league: league.into(),
    };
    let stash = |id: &str| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: None,
    };
    let tab = |id: &str, name: &str| json!({ "id": id, "name": name, "type": "PremiumStash" });
    let mut tabs = vec![tab("t1", "Rings"), tab("t2", "Launch")];
    let check = |pages: &mut Pages, label: &str, path: &str, facts: &[&str]| {
        let page = web.get(path);
        pages.page(label, &page, facts);
        every_request_an_answer_carries_is_a_way_on_its_page(&web, path, &page);
    };

    // tabs never fetched: two in Standard, one in Hardcore
    tabs.extend([tab("t3", "Maps"), tab("t4", "Gems")]);
    record(
        listing("Standard"),
        "Standard",
        json!({ "stashes": tabs }),
        60,
    );
    record(
        listing("Hardcore"),
        "Hardcore",
        json!({ "stashes": [tab("h1", "Dump")] }),
        61,
    );
    check(
        &mut pages,
        "GET search?<every ring, tabs never fetched in two leagues>",
        "search?realm=pc&text=class%3Arings",
        &[
            "data-way=\"tabs\" href=\"/",
            "/tabs?realm=pc&amp;league=Standard&amp;fetched=never\">each tab of Standard never fetched</a>",
            "1 other league holds never-fetched tabs",
        ],
    );
    check(
        &mut pages,
        "GET search?<every ring outside Standard>",
        "search?realm=pc&text=class%3Arings+-league%3DStandard",
        &[
            "/tabs?realm=pc&amp;league=Hardcore&amp;fetched=never\">each tab of Hardcore never fetched</a>",
        ],
    );

    // an id that left: Rune Coil gone from its tab at a later fetch
    record(
        stash("t1"),
        "Standard",
        json!({ "stash": { "id": "t1", "name": "Rings", "type": "PremiumStash", "items": [
            { "id": "r1", "name": "Doom Loop", "typeLine": "Two-Stone Ring", "baseType": "Two-Stone Ring",
              "rarity": "Rare", "frameTypeId": 2, "identified": true, "ilvl": 84, "x": 0, "y": 0,
              "explicitMods": ["+95 to maximum Life"] } ] } }),
        70,
    );
    check(
        &mut pages,
        "GET search?<the id that left>",
        "search?realm=pc&text=id%3Ar2",
        &[
            "this id is an item that left the scope",
            "data-way=\"/terms/0/removed\" href=",
            ">find it under membership all</a>",
        ],
    );
    // its card's place lands at the place's head, not at its row, which
    // the place's live rows lack
    let gone = web.get("item/r2");
    let card = &gone.body[gone.body.find("<p class=\"place\">").unwrap()..];
    let href = attribute(card, "href").unwrap();
    assert!(
        href.ends_with("open=t1#p-t1") && !href.contains("at="),
        "{href}"
    );

    // a value list cut: eleven names a term's `:` resolves to
    let kestrels: Vec<Value> = (1..=11)
        .map(|n| {
            json!({ "id": format!("k{n:02}"), "name": format!("Kestrel {n:02}"), "typeLine": "Iron Ring",
                    "baseType": "Iron Ring", "rarity": "Rare", "frameTypeId": 2, "identified": true,
                    "ilvl": 70, "x": n, "y": 0 })
        })
        .collect();
    tabs.push(tab("t5", "Kestrels"));
    record(
        listing("Standard"),
        "Standard",
        json!({ "stashes": tabs }),
        80,
    );
    record(
        stash("t5"),
        "Standard",
        json!({ "stash": { "id": "t5", "name": "Kestrels", "type": "PremiumStash", "items": kestrels } }),
        81,
    );
    check(
        &mut pages,
        "GET search?<a name eleven items carry>",
        "search?realm=pc&text=name%3Akestrel",
        &[
            "Kestrel 01 (1)",
            "Kestrel 10 (1)",
            "data-way=\"/terms/0/resolved/rest\"",
            ">1 more<",
        ],
    );

    // an ambiguous clause with what its other reading drops (the recipe
    // of `seat_faults.rs`'s ambiguous clause, run through the trade box)
    let ring = |id: &str, rarity: &[&str]| {
        let mut lines: Vec<&str> = rarity.to_vec();
        lines.push("Recover 2% of Mana on Kill");
        json!({ "id": id, "name": "", "typeLine": "Iron Ring", "baseType": "Iron Ring", "frameTypeId": 2, "x": 0, "y": 0, "explicitMods": lines })
    };
    tabs.push(tab("m1", "Rarity"));
    record(
        listing("Standard"),
        "Standard",
        json!({ "stashes": tabs }),
        90,
    );
    record(
        stash("m1"),
        "Standard",
        json!({ "stash": { "id": "m1", "name": "Rarity", "type": "PremiumStash", "items": [
            ring("plain", &["25% increased Movement Speed"]),
            ring("both", &["30% increased Movement Speed", "10% reduced Movement Speed"]) ] } }),
        91,
    );
    check(
        &mut pages,
        "GET search?<a site search with an ambiguous clause>",
        &format!(
            "search?{}",
            query_string(&[
                ("realm", "pc"),
                (
                    "trade",
                    r#"{"query": {"stats": [{"type": "and", "filters": [{"id": "explicit.stat_2250533757", "value": {"min": 20}}, {"id": "explicit.stat_1030153674"}]}]}}"#
                ),
            ])
        ),
        &[
            "<td>ambiguous</td><td>stats.0.0</td>",
            "read the other way: adds 0, drops <a class=\"way\" data-way=\"/query/trade/clauses/1/other/drops\" href=",
        ],
    );
    // a row whose value sums eight Spirit lines: as a table, the sum
    // stands in its value's column as one number, the lines being the
    // item's, a link away by the row's name; the row carries no request
    // of its own, so no way on is lost with the lines (the check below)
    let spirit: Vec<String> = (1..=8).map(|n| format!("+{n} to Spirit")).collect();
    tabs.push(tab("s9", "Spirit"));
    record(
        listing("Standard"),
        "Standard",
        json!({ "stashes": tabs }),
        100,
    );
    record(
        stash("s9"),
        "Standard",
        json!({ "stash": { "id": "s9", "name": "Spirit", "type": "PremiumStash", "items": [
            { "id": "many", "name": "Many Loop", "typeLine": "Iron Ring", "baseType": "Iron Ring",
              "rarity": "Rare", "frameTypeId": 2, "identified": true, "ilvl": 70, "x": 0, "y": 0,
              "explicitMods": spirit } ] } }),
        101,
    );
    check(
        &mut pages,
        "GET search?<a row that leaves lines out>",
        &format!(
            "search?{}",
            query_string(&[
                ("realm", "pc"),
                ("text", "line(template:spirit)"),
                ("view.sort", "sum(line(template:spirit).arg1)"),
                ("view.fields", "icon"),
            ])
        ),
        &[
            "<span class=\"mark\" aria-hidden=\"true\">▲</span>sum(line(template:spirit).arg1)<span class=\"sr\">, ordered smallest first</span></a></div>",
            "value=\"many\">Many Loop Iron Ring</button>",
            "<span>36</span></div>",
        ],
    );
    // as a card the item stands whole beside what the row shows, so the
    // row says no "more"
    let carded = web.get(&format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("text", "line(template:spirit)"),
            ("view.sort", "sum(line(template:spirit).arg1)"),
        ])
    ));
    assert!(
        carded.body.contains("<div class=\"card\">"),
        "{}",
        carded.body
    );
    assert!(carded.body.contains("+8 to Spirit"), "{}", carded.body);
    assert!(!carded.body.contains(" more "), "{}", carded.body);

    // the table's cells, each in its own words under its own head: two
    // values the query compares, one of them incomplete where the item's
    // implicit lines cannot be read (C94, an incomplete subtotal is
    // marked); a price from a note, a price undecided where the note is
    // no string, and none where nothing applies
    let ledger = |id: &str, name: &str, ilvl: u32, more: Value| {
        let mut v = json!({ "id": id, "name": name, "typeLine": "Iron Ring", "baseType": "Iron Ring",
                            "rarity": "Rare", "frameTypeId": 2, "identified": true, "ilvl": ilvl,
                            "x": ilvl % 12, "y": 0 });
        for (k, value) in more.as_object().unwrap() {
            v[k] = value.clone();
        }
        v
    };
    tabs.push(json!({ "id": "p9", "name": "Ledger", "type": "PremiumStash", "metadata": { "public": true } }));
    record(
        listing("Standard"),
        "Standard",
        json!({ "stashes": tabs }),
        110,
    );
    record(
        stash("p9"),
        "Standard",
        json!({ "stash": { "id": "p9", "name": "Ledger", "type": "PremiumStash", "items": [
            ledger("lp", "Ledger Priced", 81, json!({ "note": "~price 5 chaos", "explicitMods": ["+90 to maximum Life"] })),
            ledger("lo", "Ledger Open", 82, json!({ "note": 5, "explicitMods": ["+60 to maximum Life"] })),
            ledger("li", "Ledger Unread", 83, json!({ "implicitMods": "unread", "explicitMods": ["+70 to maximum Life"] })),
        ] } }),
        111,
    );
    check(
        &mut pages,
        "GET search?<a table of two values, priced three ways>",
        &format!(
            "search?{}",
            query_string(&[
                ("realm", "pc"),
                ("text", "name:ledger ilvl>=80 \"# to maximum Life\">=50"),
                ("view.fields", "icon"),
            ])
        ),
        &[
            "<span>Fetched</span><a class=\"head\" href=\"search?",
            "view.sort=ilvl&amp;view.desc=true\">ilvl</a><a class=\"head\" href=\"search?",
            "value=\"lp\">Ledger Priced Iron Ring</button><span>Iron Ring</span><span>5 chaos",
            "value=\"lo\">Ledger Open Iron Ring</button><span>Iron Ring</span><span>undecided: the note cannot be read",
            "<span>incomplete at 70</span>",
            "<span>none (",
        ],
    );

    // a card's numbers, each a link that adds its value and orders by it:
    // a line whole by its value, `avg` on a ranged pair; a
    // property's number and each requirement's by the field it displays;
    // the value the rows are ordered by marked as its head is
    tabs.push(tab("k9", "Kit"));
    record(
        listing("Standard"),
        "Standard",
        json!({ "stashes": tabs }),
        120,
    );
    let property = |name: &str, value: &str, mode: i64| json!({ "name": name, "values": [[value, 0]], "displayMode": mode });
    record(
        stash("k9"),
        "Standard",
        json!({ "stash": { "id": "k9", "name": "Kit", "type": "PremiumStash", "items": [
            ledger("kit", "Kit Coat", 84, json!({
                "properties": [property("Armour", "553", 0)],
                "requirements": [property("Level", "64", 0), property("Str", "16", 1)],
                "explicitMods": ["+30 to maximum Life", "Adds 10 to 20 Physical Damage"],
            })),
        ] } }),
        121,
    );
    check(
        &mut pages,
        "GET search?<a card whose numbers are links>",
        &format!(
            "search?{}",
            query_string(&[
                ("realm", "pc"),
                (
                    "text",
                    "name:kit sum(line(\"# to maximum Life\" source=explicit).arg1)>=10",
                ),
                (
                    "view.sort",
                    "sum(line(\"# to maximum Life\" source=explicit).arg1)",
                ),
                ("view.desc", "true"),
            ])
        ),
        &[
            "Armour: <a class=\"value\" href=\"search?realm=pc&amp;text=",
            "view.sort=armour&amp;view.desc=true&amp;view.value=",
            ">553</a>",
            "Requires Level <a class=\"value\" href=\"",
            ">64</a>, <a class=\"value\" href=\"",
            ">16</a> Str",
            "<a class=\"value valued\" href=\"",
            "▼</span>+30 to maximum Life<span class=\"sr\">, ordered largest first</span></a>",
            "view.sort=sum%28line%28%22Adds%20%23%20to%20%23%20Physical%20Damage%22%20source%3Dexplicit%29.avg%29",
            ">Adds 10 to 20 Physical Damage</a>",
        ],
    );

    // C188: a character fetched from a release after the tables', which
    // the total says beside the never-fetched, as the terminal does
    let tables = acquisition_search::class::table().unwrap().game();
    let (major, minor) = acquisition_store::release(tables).unwrap();
    let ahead = format!("{major}.{}", minor + 1);
    let character = json!({ "id": "ch1", "name": "Ahead", "league": "Standard",
                            "metadata": { "version": format!("{ahead}.0") } });
    record(
        Endpoint::Characters { realm: "pc".into() },
        "Standard",
        json!({ "characters": [character.clone()] }),
        130,
    );
    record(
        Endpoint::Character {
            realm: "pc".into(),
            name: "Ahead".into(),
        },
        "Standard",
        json!({ "character": character }),
        131,
    );
    check(
        &mut pages,
        "GET search?<a character fetched from a later release>",
        "search?realm=pc&text=class%3Arings",
        &[&format!(
            "<br>these tables read release {major}.{minor} and a character here was fetched from {ahead} ("
        )],
    );
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "answer_ways", &pages);
}

/// The table's route (`rows`, the module doc's "The table"): a window of
/// the page's rows answers as the rows alone and, where JSON is asked, as
/// the answer `search?next=` gives for the same window's request made
/// here by hand — the page's request at its basis, `after` the offset,
/// `limit` the count, `item` among its fields. Each wrong window is
/// refused in the one refusal shape with 400, as the pages' refusals are,
/// nothing clipped; a basis moved since the page is the search's own
/// refusal with 409, as HTML and as JSON.
#[test]
fn the_rows_route_answers_a_window_and_refuses_each_wrong_one() {
    use acquisition_search::answer::{Request, RowField, Shape};
    let (scratch, web) = offline_web("webrows");
    let mut pages = Pages::new(&scratch.0, &web);
    let page = web.get("search?realm=pc&text=&view.fields=icon");
    assert_eq!(page.status, 200, "{}", page.body);
    assert!(page.body.contains("data-count=\"5\""), "{}", page.body);
    let route = attribute(
        &page.body[page.body.find("<div class=\"seg\"").unwrap()..],
        "data-route",
    )
    .unwrap()
    .trim_start_matches(&format!("/{}/", web.token))
    .to_string();
    let window = format!("{route}&offset=1&count=3");
    let rows = web.get(&window);
    assert_eq!(rows.status, 200, "{}", rows.body);
    assert!(!rows.body.contains("<html"), "{}", rows.body);
    pages.page(
        "GET rows?next=<the first window>&offset=1&count=3",
        &rows,
        &["data-i=\"1\"", "data-i=\"3\"", "<span>none ("],
    );
    assert!(!rows.body.contains("data-i=\"0\""), "{}", rows.body);
    assert!(!rows.body.contains("data-i=\"4\""), "{}", rows.body);

    // the JSON is `search?next=`'s for the window's request, made by hand
    let token = route.split_once("next=").unwrap().1;
    let mut by_hand = Request::from_token(token).unwrap();
    let Shape::Rows(view) = &mut by_hand.view.shape else {
        panic!("a table's token is a rows request");
    };
    view.after = Some(1);
    view.limit = Some(3);
    if !view.fields.contains(&RowField::Item) {
        view.fields.push(RowField::Item);
    }
    let asked = web.get_json(&window);
    let searched = web.get_json(&format!("search?next={}", by_hand.token()));
    assert_eq!(asked, searched);
    assert_eq!(asked["view"]["rows"]["returned"], 3, "{asked}");
    assert!(asked["rows"][0].get("item").is_some(), "{asked}");

    // each wrong window, refused, nothing clipped
    for (label, path) in [
        ("no token", "rows?offset=0&count=1".to_string()),
        (
            "a token no answer printed",
            "rows?next=acq1.nothing&offset=0&count=1".to_string(),
        ),
        (
            "an offset that is no number",
            format!("{route}&offset=one&count=1"),
        ),
        ("no count", format!("{route}&offset=0")),
        ("a count of none", format!("{route}&offset=0&count=0")),
        (
            "a count past the bound",
            format!("{route}&offset=0&count=201"),
        ),
        (
            "a window past the rows",
            format!("{route}&offset=4&count=2"),
        ),
        (
            "an offset past the rows",
            format!("{route}&offset=5&count=1"),
        ),
        (
            "an offset at the largest number",
            format!("{route}&offset={}&count=1", usize::MAX),
        ),
    ] {
        let refused = web.get(&path);
        assert_eq!(refused.status, 400, "{label}: {}", refused.body);
        pages.page(
            &format!("GET rows?<{label}>"),
            &refused,
            &["<div class=\"refusal\">"],
        );
        let doc = refused_json(&web, &path, 400);
        assert!(doc["error"].is_string(), "{label}: {doc}");
    }

    // a basis moved since the page: the search's refusal, with 409
    Store::open(&account_path(&scratch.0.join("mock"), USER))
        .unwrap()
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "t2".into(),
                sub: None,
            },
            &json!({ "realm": "pc", "league": "Standard" }),
            200,
            &json!({ "stash": { "id": "t2", "name": "Launch", "type": "PremiumStash", "items": [] } }),
            60,
        )
        .unwrap();
    let moved = web.get(&window);
    assert_eq!(moved.status, 409, "{}", moved.body);
    pages.page(
        "GET rows?<the first window, its basis moved>",
        &moved,
        &["another basis"],
    );
    let doc = refused_json(&web, &window, 409);
    assert_eq!(doc["kind"], "basis_changed", "{doc}");

    // two opened rows, over the three rows the
    // moved world holds: the order cut after each, three segments of one
    // route, each card in the flow with its Close
    let order = in_table_order(&web, "search?realm=pc&text=&view.fields=icon");
    assert_eq!(order.len(), 3, "{order:?}");
    let two = web.get(&format!(
        "search?realm=pc&text=&view.fields=icon&item={}&item={}",
        order[1], order[0]
    ));
    pages.page(
        "GET search?<the table, its first two rows opened>",
        &two,
        &[
            "data-count=\"1\" data-base=\"0\"",
            "data-count=\"1\" data-base=\"1\"",
            "data-count=\"1\" data-base=\"2\"",
            "<div class=\"opened\">",
            ">Close</button>",
        ],
    );
    assert_eq!(two.body.matches("<div class=\"seg\"").count(), 3);
    for (at, id) in [(0, &order[0]), (1, &order[1])] {
        let card = &two.body[two.body.find(&format!("data-i=\"{at}\"")).unwrap()..];
        let card = &card[card.find("<div class=\"opened\">").unwrap()..];
        assert!(
            card[..card.find("</div>\n</div>").unwrap_or(card.len())].contains(&format!(
                "name=\"toggle_item\" value=\"{id}\">Close</button>"
            )),
            "{at}: {}",
            two.body
        );
    }
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "table_rows", &pages);
}

/// The ids of a table's rows in its order, as its JSON lists them.
fn in_table_order(web: &WebUi, path: &str) -> Vec<String> {
    web.get_json(path)["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap().to_string())
        .collect()
}

/// Every link and form field a page writes that names `name`, its value
/// as written: what a test reads to say no URL carries a field.
fn written(body: &str, name: &str) -> Vec<String> {
    let mut out = Vec::new();
    // a card's place is the one link that lands elsewhere at its own row,
    // never a page's `at` carried on, so this helper reads past it
    let mut kept = String::new();
    let mut rest = body;
    while let Some(start) = rest.find("<a class=\"placed\" href=\"") {
        kept.push_str(&rest[..start]);
        rest = &rest[start..];
        rest = &rest[rest.find('>').map_or(rest.len(), |end| end + 1)..];
    }
    kept.push_str(rest);
    let body = kept.as_str();
    for key in [format!("?{name}="), format!("&amp;{name}=")] {
        out.extend(body.match_indices(&key).map(|(at, _)| {
            let rest = &body[at + key.len()..];
            rest[..rest.find(['&', '"', '#']).unwrap_or(rest.len())].to_string()
        }));
    }
    out.extend(
        body.match_indices(&format!("name=\"{name}\" value=\""))
            .map(|(at, k)| {
                let rest = &body[at + k.len()..];
                rest[..rest.find('"').unwrap()].to_string()
            }),
    );
    out
}

/// An item opened under its row: a row's name is a toggle in the table's
/// form, answered 303 to the page with the id added and `at`, at the
/// row's anchor; the page renders the row's block and its card after it,
/// and no URL it writes carries `at`. A head's link keeps the row open at
/// its new index; toggled again, it closes, its block still rendered for
/// the anchor. An id no order holds is dropped; "as cards" and the query
/// box carry no opened row.
#[test]
fn a_row_opens_under_itself_and_stays_open_as_the_view_changes() {
    let (_scratch, web) = offline_web("webopened");
    let table = "search?realm=pc&text=&view.fields=icon&view.value=pseudo.total_life";
    let order = in_table_order(&web, table);
    let id = order[2].clone();
    let page = web.get(table);
    let toggled = follow(
        &web,
        &place_form(
            &web,
            &page.body,
            &[],
            &name_of(&page.body, &id),
            Some(("toggle_item", &id)),
        ),
    );
    assert_eq!(toggled.status, 303, "{}", toggled.body);
    let location = toggled.header("location").unwrap().to_string();
    assert!(
        location.ends_with(&format!("&item={id}&at={id}#i-{id}")),
        "{location}"
    );
    let opened = follow(&web, &location);
    let at = |body: &str, words: &str| {
        body.find(words)
            .unwrap_or_else(|| panic!("{words}: {body}"))
    };
    // the row's segment ends at it, and its card stands after it
    assert!(
        opened.body.contains("data-count=\"3\" data-base=\"0\""),
        "{}",
        opened.body
    );
    assert!(
        at(&opened.body, &format!("id=\"i-{id}\"")) < at(&opened.body, "<div class=\"opened\">"),
        "{}",
        opened.body
    );
    assert!(
        written(&opened.body, "at").is_empty(),
        "{:?}",
        written(&opened.body, "at")
    );
    // "as cards" and the query box carry no opened row
    let cards = link_saying(&opened.body, "as cards");
    assert!(!cards.contains("item="), "{cards}");
    let query_box = &opened.body[at(&opened.body, "<form class=\"query-box\"")..];
    assert!(
        !query_box[..query_box.find("</form>").unwrap()].contains("name=\"item\""),
        "{query_box}"
    );

    // a head's link keeps it open, at its index in the new order
    let head = &opened.body[at(&opened.body, "<div class=\"r head\">")..];
    let href = attribute(&head[head.find("<a class=\"head\"").unwrap()..], "href").unwrap();
    assert!(href.contains(&format!("item={id}")), "{href}");
    let reordered = follow(&web, &format!("/{}/{href}", web.token));
    let sorted = in_table_order(&web, &href);
    let n = sorted.iter().position(|i| *i == id).unwrap();
    assert_ne!(sorted, order, "the head reorders the rows");
    assert!(
        reordered
            .body
            .contains(&format!("data-count=\"{}\" data-base=\"0\"", n + 1)),
        "{n}: {}",
        reordered.body
    );
    assert!(
        reordered.body.contains("<div class=\"opened\">"),
        "{}",
        reordered.body
    );

    // toggled again, it closes: the landing still renders its block
    let closed = follow(
        &web,
        &place_form(
            &web,
            &reordered.body,
            &[],
            "Close",
            Some(("toggle_item", &id)),
        ),
    );
    assert_eq!(closed.status, 303, "{}", closed.body);
    let location = closed.header("location").unwrap().to_string();
    assert!(!location.contains("item="), "{location}");
    assert!(location.ends_with(&format!("at={id}#i-{id}")), "{location}");
    let landed = follow(&web, &location);
    assert!(
        !landed.body.contains("<div class=\"opened\">"),
        "{}",
        landed.body
    );
    assert!(
        landed.body.contains(&format!("id=\"i-{id}\"")),
        "{}",
        landed.body
    );
    assert!(written(&landed.body, "at").is_empty(), "{}", landed.body);

    // a table reached by a continuation keeps it through a head's link
    // too
    let first = web.get_json(&format!("{table}&view.limit=1"));
    let next: acquisition_search::answer::Request =
        serde_json::from_value(first["view"]["rows"]["next"]["request"].clone()).unwrap();
    let continued = web.get(&format!("search?next={}&item={id}", next.token()));
    assert!(
        continued.body.contains("<div class=\"opened\">"),
        "{}",
        continued.body
    );
    let head = &continued.body[at(&continued.body, "<div class=\"r head\">")..];
    let href = attribute(&head[head.find("<a class=\"head\"").unwrap()..], "href").unwrap();
    assert!(href.contains(&format!("item={id}")), "{href}");
    let followed = follow(&web, &href);
    assert!(
        followed.body.contains("<div class=\"opened\">"),
        "{}",
        followed.body
    );

    // an id no order holds is dropped, in the page and every URL it writes
    let stale = web.get(&format!("{table}&item=no-such-item&item={id}"));
    assert_eq!(stale.status, 200, "{}", stale.body);
    assert_eq!(
        written(&stale.body, "item")
            .iter()
            .filter(|v| *v == "no-such-item")
            .count(),
        0
    );
    assert!(written(&stale.body, "item").contains(&id), "{}", stale.body);
    // a page of cards cuts nothing, and keeps no opened row
    let cards = web.get(&format!("search?realm=pc&text=&item={id}"));
    assert!(written(&cards.body, "item").is_empty(), "{}", cards.body);
}

/// A row's name as its toggle says it.
fn name_of(body: &str, id: &str) -> String {
    let key = format!("name=\"toggle_item\" value=\"{id}\">");
    let at = body.find(&key).unwrap_or_else(|| panic!("{id}: {body}")) + key.len();
    unescaped(&body[at..at + body[at..].find("</button>").unwrap()])
}

/// A refusal's document as the server answers a request under the token
/// that accepts JSON, with its status.
fn refused_json(web: &WebUi, path: &str, status: u16) -> Value {
    let reply = web.raw(
        "GET",
        &format!("/{}/{path}", web.token),
        None,
        &[("Accept", "application/json")],
        "",
    );
    assert_eq!(reply.status, status, "{path}: {}", reply.body);
    serde_json::from_str(&reply.body).unwrap_or_else(|e| panic!("{path}: {e}: {}", reply.body))
}

/// The links a refusal page offers, in order, as a browser follows them.
fn offered_links(body: &str) -> Vec<String> {
    let Some((_, offers)) = body.split_once("<ul class=\"offers\">") else {
        return Vec::new();
    };
    offers
        .split("href=\"")
        .skip(1)
        .filter_map(|rest| rest.split_once('"'))
        .map(|(href, _)| href.replace("&amp;", "&"))
        .collect()
}

/// The refresh loop through the place pages alone, over the mock: from a
/// world with nothing listed and no policy, the
/// stash tabs and characters pages list the league, each through its own
/// listing, with no command line and no "Plan under the policy"; a ticked
/// selection is planned and applied; `refresh --all` (a selection over
/// every tab with its listing, C76) lands the pages of the map and unique
/// tabs, which the stash tabs page holds under their closed parents with
/// their counts; an opener answers `303` to the page with that parent
/// open, at its head, every tick kept, and again closes it; a tick under
/// a closed folder travels to the plan; a selection compiles the plan
/// `refresh --tabs` applies; and once a policy names the league, the
/// policy's plan with its quote, apply posting the envelope whole, the
/// same envelope again sending nothing (C111), and a plan whose policy
/// moved refused (C44).
#[test]
fn the_refresh_loop_runs_through_the_pages() {
    let scratch = harness::scratch("webloop");
    let base = scratch.0.clone();
    let _daemon = harness::login_as_mock(&base, &[]);
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let mut pages = Pages::new(&base, &web).fetched_live();

    // nothing listed and no policy: each page offers its listing where
    // the table would be, and no plan under a policy
    let none = web.get("tabs?realm=pc&league=Standard");
    pages.page(
        "GET tabs?realm=pc&league=Standard <nothing listed>",
        &none,
        &[
            "<h1>Stash tabs</h1>",
            "No stash tabs of Standard are on record.",
            "facts: no tabs; stash never listed",
            "List this league&#39;s stash tabs",
            "Fetch the league list",
        ],
    );
    let no_one = web.get("characters?realm=pc&league=Standard");
    pages.page(
        "GET characters?realm=pc&league=Standard <nothing listed>",
        &no_one,
        &[
            "<h1>Characters</h1>",
            "no characters known in Standard",
            "List characters",
        ],
    );
    for page in [&none, &no_one] {
        assert!(
            !page.body.contains("Plan under the policy"),
            "{}",
            page.body
        );
    }
    for (page, button) in [
        (&none, "List this league's stash tabs"),
        (&no_one, "List characters"),
    ] {
        // the form's action is the page's own route, relative as a
        // browser resolves it
        let form = form_on(&page.body, button, None);
        let listed = send(&web, &form, false);
        // a listing lands at its job, its row marked, with the way back to
        // the place page it was asked from
        let from = form.field("from").expect("the page's `from`").to_string();
        assert!(from.ends_with("?realm=pc&league=Standard"), "{from}");
        let (id, landed) = landed_at_a_job(&web, &listed, Some(&from));
        assert!(
            landed
                .body
                .contains(&format!("<tr class=\"job opened\" id=\"j-{id}\">")),
            "{}",
            landed.body
        );
        until_done(&web);
    }
    // the league list's fetch posted with no page lands at its job with no
    // way back
    let (id, landed) = landed_at_a_job(&web, &web.post("leagues", &[]), None);
    assert!(
        landed.body.contains(&format!(
            "<tr class=\"job opened\" id=\"j-{id}\"><td>The league list</td>"
        )),
        "{}",
        landed.body
    );
    until_done(&web);
    let jobs = sole_json(&harness::acq(&base, &["jobs", "--all", "--json"]));
    let kinds: Vec<(&Value, &Value)> = jobs["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|j| (&j["kind"], &j["submitted_by"]))
        .collect();
    for kind in ["stashes", "characters"] {
        assert!(
            kinds
                .iter()
                .any(|(k, by)| *k == kind && by.as_str().unwrap().starts_with("web:")),
            "{kind}: {kinds:?}"
        );
    }

    // listed and never fetched: a parent closed, its counts on its head
    let tabs = web.get("tabs?realm=pc&league=Standard");
    pages.page(
        "GET tabs?realm=pc&league=Standard <listed, never fetched>",
        &tabs,
        &[
            "tabs in Standard",
            // a tick box is named by its place, not its id
            "aria-label=\"Dump\" name=\"tab\" value=\"dump\"",
            // never fetched reads apart from fetched and empty
            ">Dump</a></span><span>PremiumStash</span><span>-</span><span>never</span>",
            "<span>Old leagues <span class=\"holds-said\">2 tabs, 2 never fetched</span>",
            "<span>Folder</span><span></span><span></span></div>",
            "2 tabs, 2 never fetched",
            "name=\"toggle\" value=\"fold\">Open</button>",
            "Refresh ticked",
            // the filter's controls, sent with the form
            "<input type=\"text\" name=\"name\" value=\"\"",
            "<select name=\"type\"",
            "name=\"fetched\" value=\"never\"",
        ],
    );
    assert!(!tabs.body.contains(" checked"), "no default selection");
    assert!(
        !tabs.body.contains("value=\"old1\""),
        "a closed folder's tab"
    );
    assert!(
        !tabs.body.contains("Plan under the policy"),
        "{}",
        tabs.body
    );
    // the buttons and the substashes box above the list, "Show" the
    // form's first, so Enter sends the page again
    let at = |body: &str, words: &str| {
        body.find(words)
            .unwrap_or_else(|| panic!("{words}: {body}"))
    };
    // the list's summary names it, and the league's counts stand once, in
    // the counts line
    assert!(
        tabs.body.contains("<summary>Stash tabs</summary>"),
        "{}",
        tabs.body
    );
    assert!(at(&tabs.body, "Refresh ticked") < at(&tabs.body, "<summary>Stash tabs</summary>"));
    assert!(at(&tabs.body, "name=\"deep\"") < at(&tabs.body, "<span>Tab</span>"));
    let table_form = at(&tabs.body, "<form method=\"get\" action=\"tabs\">\n");
    assert_eq!(
        at(&tabs.body[table_form..], "<button "),
        at(
            &tabs.body[table_form..],
            "<button type=\"submit\">Show</button>"
        ),
        "Show is the table form's first button"
    );

    // two tabs ticked, planned and applied: a selection needs no policy
    let selected = follow(
        &web,
        &place_form(&web, &tabs.body, &["dump", "cur1"], "Refresh ticked", None),
    );
    assert_eq!(selected.status, 200, "{}", selected.body);
    pages.page(
        "GET plan?<two tabs ticked>",
        &selected,
        &[
            "the places you ticked",
            "<td>Tab Dump</td><td>ticked</td>",
            "3 requests:",
            "could go now",
            "not yet known",
            ">Apply</button>",
        ],
    );
    // C16: the basis line is the envelope's own, read unmasked (the
    // golden masks its listings' response ids)
    let held: Value = serde_json::from_str(&hidden_plan(&selected.body)).unwrap();
    assert_eq!(
        basis_line(&selected.body).expect("the plan page's basis line")["plan"],
        held["basis"],
        "{}",
        selected.body
    );
    // Apply, from the plan the ticked page asked: it lands at its job with
    // the way back to that page, every tick kept
    let apply = form_on(&selected.body, "Apply", None);
    let from = apply.field("from").expect("the plan's `from`").to_string();
    assert!(
        from.starts_with("tabs?realm=pc&league=Standard")
            && from.contains("&tab=dump")
            && from.contains("&tab=cur1"),
        "{from}"
    );
    let applied = send(&web, &apply, false);
    let (_, landed) = landed_at_a_job(&web, &applied, Some(&from));
    assert!(
        landed.body.contains("A refresh of 3 requests"),
        "{}",
        landed.body
    );
    until_done(&web);
    // `refresh --tabs` compiles and applies the same jobs (C44)
    let out = harness::acq(&base, &["refresh", "--tabs", "dump,cur1"]);
    assert!(out.status.success(), "{out:?}");
    let listed = sole_json(&harness::acq(&base, &["jobs", "--all", "--json"]));
    let applies: Vec<&Value> = listed["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["kind"] == "apply")
        .collect();
    let (page, cli) = (&applies[applies.len() - 2], &applies[applies.len() - 1]);
    assert!(
        page["submitted_by"].as_str().unwrap().starts_with("web:"),
        "{page}"
    );
    assert!(
        cli["submitted_by"].as_str().unwrap().starts_with("cli:"),
        "{cli}"
    );
    assert_eq!(page["params"]["jobs"], cli["params"]["jobs"]);
    assert!(
        listed["rows"]
            .as_array()
            .unwrap()
            .iter()
            .all(|j| j["kind"] != "refresh"),
        "no ad-hoc refresh job"
    );

    // the characters page plans its own ticks
    let characters = web.get("characters?realm=pc&league=Standard");
    pages.page(
        "GET characters?realm=pc&league=Standard",
        &characters,
        &[
            "characters: 0 fetched",
            "aria-label=\"StashHoarder\" name=\"character\" value=\"fake0001\"",
            "<input type=\"text\" name=\"name\"",
        ],
    );
    let ticked = first_value(&characters.body, "name=\"character\" value=\"");
    let planned = follow(
        &web,
        &place_form(&web, &characters.body, &[&ticked], "Refresh ticked", None),
    );
    assert_eq!(planned.status, 200, "{}", planned.body);
    assert!(
        planned.body.contains("the places you ticked"),
        "{}",
        planned.body
    );
    assert!(planned.body.contains("<td>Character "), "{}", planned.body);
    assert!(!planned.body.contains("<td>Tab "), "{}", planned.body);

    // a league with no character beside a realm with two: the page says
    // the terminal's own cause, which names where they are, with the
    // listing and no command
    let elsewhere = web.get("characters?realm=pc&league=Hardcore");
    assert_eq!(elsewhere.status, 200, "{}", elsewhere.body);
    let said = harness::acq(&base, &["store", "characters", "--league", "Hardcore"]);
    let said = String::from_utf8(said.stdout).unwrap();
    let cause = "no characters in Hardcore: the 2 known are in Standard";
    assert!(said.starts_with(&format!("{cause} (")), "{said}");
    assert!(
        unescaped(main_of(&elsewhere.body)).contains(&format!("{cause}</p>")),
        "{cause}: {}",
        elsewhere.body
    );
    assert!(
        elsewhere.body.contains(">List characters</button>"),
        "{}",
        elsewhere.body
    );

    // `refresh --all` is a selection too: every tab, the listing fetched
    // beside it (C76); it lands the map and unique tabs' pages
    let out = harness::acq(&base, &["refresh", "--all"]);
    assert!(out.status.success(), "{out:?}");
    let said = String::from_utf8_lossy(&out.stdout);
    for fact in [
        "a selection",
        "the listing is fetched beside the selection",
        "fetch 7 tabs",
        "1 folder",
    ] {
        assert!(said.contains(fact), "{fact}: {said}");
    }

    // a fetched empty tab, a never-fetched page under a closed parent with
    // its counts; opened, at its head with every tick, its pages listed
    let tabs = web.get("tabs?realm=pc&league=Standard&tab=dump");
    pages.page(
        "GET tabs?realm=pc&league=Standard&tab=dump <every tab fetched>",
        &tabs,
        &[
            "Maps (empty)</a></span><span>MapStash</span><span>0</span>",
            "4 pages, 4 never fetched",
            "name=\"toggle\" value=\"maps\">Open</button>",
            "name=\"tab\" value=\"dump\" checked",
        ],
    );
    assert!(!tabs.body.contains("value=\"m001\""), "{}", tabs.body);
    let toggled = follow(
        &web,
        &place_form(&web, &tabs.body, &[], "Open", Some(("toggle", "maps"))),
    );
    assert_eq!(toggled.status, 303, "{}", toggled.body);
    let to = toggled.header("location").unwrap().to_string();
    assert_eq!(
        to,
        format!(
            "/{}/tabs?realm=pc&league=Standard&open=maps&tab=dump#p-maps",
            web.token
        )
    );
    let open = follow(&web, &to);
    pages.page(
        "GET tabs?<Maps opened>",
        &open,
        &[
            "name=\"toggle\" value=\"maps\">Close</button>",
            "<div class=\"holds\">",
            "<div class=\"p row depth-1\" id=\"p-m001\">",
            "<div class=\"p row depth-0\" id=\"p-maps\">",
            "name=\"tab\" value=\"dump\" checked",
        ],
    );
    assert!(at(&open.body, "id=\"p-maps\"") < at(&open.body, "id=\"p-m004\""));
    let closed = follow(
        &web,
        &place_form(&web, &open.body, &[], "Close", Some(("toggle", "maps"))),
    );
    assert_eq!(
        closed.header("location"),
        Some(
            format!(
                "/{}/tabs?realm=pc&league=Standard&tab=dump#p-maps",
                web.token
            )
            .as_str()
        )
    );

    // a tick under a folder, opened, ticked and closed again, travels
    // hidden, is counted on the folder's head, and reaches the plan
    let folder = follow(
        &web,
        &place_form(&web, &tabs.body, &[], "Open", Some(("toggle", "fold"))),
    );
    let folder = follow(&web, folder.header("location").unwrap());
    let shut = follow(
        &web,
        &place_form(
            &web,
            &folder.body,
            &["old1"],
            "Close",
            Some(("toggle", "fold")),
        ),
    );
    let shut_at = shut.header("location").unwrap().to_string();
    assert!(shut_at.contains("&tab=dump&tab=old1#p-fold"), "{shut_at}");
    let shut = follow(&web, &shut_at);
    assert!(
        shut.body
            .contains("<input type=\"hidden\" name=\"tab\" value=\"old1\">"),
        "{}",
        shut.body
    );
    assert!(shut.body.contains("; 1 ticked inside"), "{}", shut.body);
    assert!(
        !shut.body.contains("value=\"old1\" checked"),
        "{}",
        shut.body
    );
    let plan = place_form(&web, &shut.body, &[], "Refresh ticked", None);
    let planned = web.raw(
        "GET",
        &unescaped(&plan),
        None,
        &[("Accept", "application/json")],
        "",
    );
    assert_eq!(planned.status, 200, "{}", planned.body);
    let planned: Value = serde_json::from_str(&planned.body).unwrap();
    let fetched: std::collections::BTreeSet<&str> = planned["plan"]["actions"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|a| a["id"].as_str())
        .collect();
    assert_eq!(
        fetched,
        std::collections::BTreeSet::from(["dump", "old1"]),
        "{planned}"
    );

    // the filter: the places whose name
    // holds "maps", a tick on a tab it does not list travelling hidden and
    // counted on its level's fold line, the map tab's pages folded with
    // their never-fetched count a link to the page under `fetched=never`
    // alone, every open and tick kept
    let filtered = web.get("tabs?realm=pc&league=Standard&name=maps&open=maps&tab=dump");
    assert_eq!(filtered.status, 200, "{}", filtered.body);
    assert!(
        !filtered.body.contains("id=\"p-dump\""),
        "{}",
        filtered.body
    );
    assert!(filtered.body.contains("id=\"p-mape\""), "{}", filtered.body);
    assert!(
        filtered
            .body
            .contains("<input type=\"hidden\" name=\"tab\" value=\"dump\">"),
        "{}",
        filtered.body
    );
    assert!(
        filtered.body.contains("4 tabs not shown; 1 ticked</p>"),
        "{}",
        filtered.body
    );
    let never = format!(
        "/{}/tabs?realm=pc&amp;league=Standard&amp;fetched=never&amp;open=maps&amp;tab=dump",
        web.token
    );
    assert!(
        filtered.body.contains(&format!(
            "4 pages not shown, <a href=\"{never}\">4 never fetched</a>"
        )),
        "{}",
        filtered.body
    );
    // the selects' form carries the filter, and no tick or open
    let selects = &filtered.body[..at(&filtered.body, "<div class=\"actions\">")];
    assert!(
        selects.contains("<input type=\"hidden\" name=\"name\" value=\"maps\">"),
        "{selects}"
    );
    assert!(!selects.contains("name=\"tab\""), "{selects}");
    // the never fetched, each with the parent that holds it
    let unfetched = follow(&web, &unescaped(&never));
    for listed in ["p-maps", "p-m001", "p-m004", "p-uniq"] {
        assert!(
            unfetched.body.contains(&format!("id=\"{listed}\"")),
            "{listed}: {}",
            unfetched.body
        );
    }
    assert!(
        !unfetched.body.contains("id=\"p-mape\""),
        "{}",
        unfetched.body
    );
    // a toggle keeps the filter; "Show" sends it with every tick and open
    let closed = follow(
        &web,
        &place_form(&web, &filtered.body, &[], "Close", Some(("toggle", "maps"))),
    );
    assert_eq!(
        closed.header("location"),
        Some(
            format!(
                "/{}/tabs?realm=pc&league=Standard&name=maps&tab=dump#p-maps",
                web.token
            )
            .as_str()
        )
    );
    let shown = follow(&web, &place_form(&web, &filtered.body, &[], "Show", None));
    assert_eq!(shown.status, 200, "{}", shown.body);
    assert_eq!(main_of(&shown.body), main_of(&filtered.body));
    // the tick on the tab the page does not list reaches the plan
    let plan = place_form(&web, &filtered.body, &[], "Refresh ticked", None);
    let planned = web.raw(
        "GET",
        &unescaped(&plan),
        None,
        &[("Accept", "application/json")],
        "",
    );
    let planned: Value = serde_json::from_str(&planned.body).unwrap();
    let fetched: Vec<&str> = planned["plan"]["actions"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|a| a["id"].as_str())
        .collect();
    assert_eq!(fetched, ["dump"], "{planned}");
    // a filter the document refuses is the page's refusal, in its words
    let refused = web.get("tabs?realm=pc&league=Standard&fetched=yesterday");
    assert!(
        refused.body.contains("is no way a place was fetched"),
        "{}",
        refused.body
    );

    // with every parent open, each place's name is the way to its items as
    // a table, and a folder's is no link; one page fetched, so its table
    // holds its items
    let out = harness::acq(&base, &["stash", "maps", "--sub", "m002"]);
    assert!(out.status.success(), "{out:?}");
    let every_open = "tabs?realm=pc&league=Standard&open=fold&open=uniq&open=maps";
    let whole = web.get(every_open);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, every_open, &whole);
    assert!(
        whole.body.contains("<span>Old leagues <span"),
        "{}",
        whole.body
    );
    for (place, holds) in [("dump", "Dump"), ("m002", "(unnamed)")] {
        let row = &whole.body[at(&whole.body, &format!("id=\"p-{place}\""))..];
        let href = attribute(&row[row.find("<a class=\"way\"").unwrap()..], "href").unwrap();
        assert!(href.contains("view.fields=icon"), "{href}");
        let table = follow(&web, &href);
        assert_eq!(table.status, 200, "{href}: {}", table.body);
        assert!(
            table.body.contains("class=\"window\""),
            "{place}: {}",
            table.body
        );
        assert!(row.contains(holds), "{place}");
    }
    let path = "characters?realm=pc&league=Standard";
    every_request_an_answer_carries_is_a_way_on_its_page(&web, path, &web.get(path));

    // a policy naming the league: its plan, applied once, then spent, and
    // refused once the policy moves
    // (its characters, listed and never fetched, are its plan's work)
    let policy = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["dump"],"characters":"all","max_age_seconds":3600}}}}}"#;
    assert!(
        harness::acq(&base, &["policy", "set", policy])
            .status
            .success()
    );
    let tabs = web.get("tabs?realm=pc&league=Standard");
    assert!(tabs.body.contains("Plan under the policy"), "{}", tabs.body);
    let plan = follow(
        &web,
        &place_form(&web, &tabs.body, &[], "Plan under the policy", None),
    );
    pages.page(
        "GET plan?realm=pc&league=Standard <the policy's>",
        &plan,
        &[
            "your sync policy covers (revision 1)",
            "Character StashHoarder",
            "Outside your sync policy: 13 of the league&#39;s 14 tabs",
            "Not fetched: 1 tab",
            ">Apply</button>",
        ],
    );
    // the page's way on is its Apply button, never the command a terminal
    // prints beside the plan
    assert!(!plan.body.contains("next: acq"), "{}", plan.body);
    // a league the policy does not cover: its way on is the plan of the
    // one it does, a link to that page
    let uncovered = web.get("plan?realm=pc&league=Hardcore");
    assert_eq!(uncovered.status, 400, "{}", uncovered.body);
    pages.page(
        "GET plan?realm=pc&league=Hardcore",
        &uncovered,
        &[
            "not Hardcore",
            "plan?realm=pc&amp;league=Standard\">the plan of pc Standard, which the policy covers</a>",
        ],
    );
    let envelope = hidden_plan(&plan.body);
    let held: Value = serde_json::from_str(&envelope).unwrap();
    assert_eq!(
        basis_line(&plan.body).expect("the plan page's basis line")["plan"],
        held["basis"],
        "{}",
        plan.body
    );
    let apply = form_on(&plan.body, "Apply", None);
    assert_eq!(apply.field("plan"), Some(envelope.as_str()));
    let from = apply.field("from").expect("the plan's `from`").to_string();
    landed_at_a_job(&web, &send(&web, &apply, false), Some(&from));
    let jobs = probes_in_order(until_done(&web));
    pages.page(
        "GET jobs <every job done>",
        &jobs,
        &[
            "<td>A refresh of 2 requests</td><td>done</td>",
            "<td>\u{21b3} Character StashHoarder</td><td>done</td>",
            "jobs: 0 running, 0 waiting,",
            "earlier finished jobs are not listed",
        ],
    );
    // the characters fetched: a character's name opens a table of the
    // items it holds
    let fetched = web.get("characters?realm=pc&league=Standard");
    let row = &fetched.body[at(&fetched.body, "name=\"character\" value=\"")..];
    let href = attribute(&row[row.find("<a class=\"way\"").unwrap()..], "href").unwrap();
    assert!(href.contains("view.fields=icon"), "{href}");
    let table = follow(&web, &href);
    assert_eq!(table.status, 200, "{href}: {}", table.body);
    assert!(table.body.contains("class=\"window\""), "{}", table.body);
    let listed = web.get_json("characters?realm=pc&league=Standard");
    let held = listed["places"][0]["item_count"].as_u64().unwrap();
    assert!(held > 0, "{listed}");
    let answered = web.raw(
        "GET",
        &unescaped(&href),
        None,
        &[("Accept", "application/json")],
        "",
    );
    let answered: Value = serde_json::from_str(&answered.body).unwrap();
    assert_eq!(answered["total"]["matched"], held, "{answered}");
    // each apply below posts the plan page's `from`, as its Apply form
    // does: every replan link it answers hands that page on (the landing
    // rule), so the way back survives a spent plan and a moved policy
    let onward = format!("&amp;from={}", url_encoded(&from));
    let replay = web.post("apply", &[("plan", &envelope), ("from", &from)]);
    assert_eq!(replay.status, 200, "{}", replay.body);
    pages.page(
        "POST apply <the same envelope again>",
        &replay,
        &[
            "Nothing was sent: every request of this plan is spent",
            "2 fetched after it was compiled",
            "A plan made now would fetch",
            "plan?realm=pc&amp;league=Standard&amp;from=tabs%3Frealm%3Dpc%26league%3DStandard",
            "\">The plan made again</a>",
        ],
    );
    let again = link_saying(&replay.body, "The plan made again");
    assert!(again.ends_with(&unescaped(&onward)), "{again}");
    // followed, the plan made again draws the way back to that page
    let replanned = follow(&web, &again);
    let back = format!(
        "<p class=\"back\"><a href=\"/{}/{}\">",
        web.token,
        from.replace('&', "&amp;")
    );
    assert!(replanned.body.contains(&back), "{back}: {}", replanned.body);
    // the policy's plan once all it covers is fresh: nothing to fetch, no
    // Apply; its envelope posted anyway sends nothing and says so
    let fresh = web.get("plan?realm=pc&league=Standard");
    pages.page(
        "GET plan?realm=pc&league=Standard <every request fresh>",
        &fresh,
        &["Nothing to fetch in Standard", "Not fetched: 2 characters"],
    );
    assert!(!fresh.body.contains(">Apply</button>"), "{}", fresh.body);
    let nothing = web.get_json("plan?realm=pc&league=Standard");
    let nothing = web.post(
        "apply",
        &[("plan", &nothing["plan"].to_string()), ("from", &from)],
    );
    assert_eq!(nothing.status, 200, "{}", nothing.body);
    pages.page(
        "POST apply <a plan with nothing to fetch>",
        &nothing,
        &[
            "Nothing was sent: this plan has nothing to fetch",
            "It left out 1 tab and 2 characters",
            "\">The plan made again</a>",
        ],
    );
    assert!(
        link_saying(&nothing.body, "The plan made again").ends_with(&unescaped(&onward)),
        "{}",
        nothing.body
    );
    let moved = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["cur1"],"max_age_seconds":3600}}}}}"#;
    assert!(
        harness::acq(&base, &["policy", "set", "--replace", moved])
            .status
            .success()
    );
    let refused = web.post("apply", &[("plan", &envelope), ("from", &from)]);
    assert_eq!(refused.status, 400, "{}", refused.body);
    // its way on is the plan page, a link, never the command a terminal
    // types; followed, the plan compiled again from the policy as it
    // stands
    assert!(!refused.body.contains("acq "), "{}", refused.body);
    pages.page(
        "POST apply <a plan whose policy moved>",
        &refused,
        &[
            "the sync policy moved",
            "plan?realm=pc&amp;league=Standard&amp;from=tabs%3Frealm%3Dpc%26league%3DStandard",
            "\">the plan of pc Standard compiled again, from the policy as it stands</a>",
        ],
    );
    let replanned = web.raw("GET", &offered_links(&refused.body)[0], None, &[], "");
    assert_eq!(replanned.status, 200, "{}", replanned.body);
    assert!(
        replanned.body.contains("<h1>Plan</h1>"),
        "{}",
        replanned.body
    );
    // the refusal and the plan it offers both lead back to the place page
    // the plan was asked from
    for page in [&refused.body, &replanned.body] {
        assert!(page.contains(&back), "{back}: {page}");
    }
    page_golden(file!(), "refresh_loop", &pages);
    drop(web);
    assert!(harness::acq(&base, &["daemon", "stop"]).status.success());
}

/// The place pages: each navigation entry is a page, the
/// stash tabs and characters among them where "Refresh" stood, and
/// `refresh` answers none. A tick is one league's: with one id listed in
/// two leagues and ticked in one, the realm and league selects are a form
/// of their own carrying no tick and no open parent, so the other
/// league's page, reached by them, ticks nothing; and a tick or an open
/// that names no place of the league is dropped from the page.
#[test]
fn a_place_page_ticks_one_leagues_places_and_each_navigation_entry_is_a_page() {
    let (scratch, web) = offline_web("webplaces");
    let mut store = Store::open(&account_path(&scratch.0.join("mock"), USER)).unwrap();
    for league in ["Standard", "Hardcore"] {
        store
            .record(
                &Endpoint::Stashes {
                    realm: "pc".into(),
                    league: league.into(),
                },
                &json!({ "league": league }),
                200,
                &json!({ "stashes": [{ "id": "same", "name": "Same", "type": "PremiumStash" }] }),
                70,
            )
            .unwrap();
    }
    drop(store);

    let nav = web.get("");
    let nav = &nav.body[nav.body.find("<nav>").unwrap()..nav.body.find("</nav>").unwrap()];
    let entries: Vec<(String, String)> = nav
        .split("<a href=\"")
        .skip(1)
        .map(|a| {
            let (href, rest) = a.split_once("\">").unwrap();
            (
                href.to_string(),
                rest[..rest.find("</a>").unwrap()].to_string(),
            )
        })
        .collect();
    let words: Vec<&str> = entries.iter().map(|(_, w)| w.as_str()).collect();
    assert_eq!(
        words,
        ["Search", "Saved", "Stash tabs", "Characters", "Jobs"]
    );
    for (href, words) in &entries {
        let page = follow(&web, href);
        assert_eq!(page.status, 200, "{words}: {}", page.body);
        let heading = match words.as_str() {
            "Search" => "<h1>Search</h1>",
            "Saved" => "<h1>Saved searches</h1>",
            "Jobs" => "<h1>Jobs</h1>",
            other => &format!("<h1>{other}</h1>"),
        };
        assert!(page.body.contains(heading), "{words}: {}", page.body);
        assert!(!page.body.contains("/refresh"), "{words}: {}", page.body);
    }
    assert_eq!(web.get("refresh?realm=pc&league=Standard").status, 404);

    let ticked = web.get("tabs?realm=pc&league=Standard&tab=same&tab=gone&open=nowhere");
    assert!(
        ticked.body.contains("name=\"tab\" value=\"same\" checked"),
        "{}",
        ticked.body
    );
    for dropped in ["gone", "nowhere"] {
        assert!(!ticked.body.contains(dropped), "{dropped}: {}", ticked.body);
    }
    // the selects' form: the realm and league, nothing else sent
    let start = ticked
        .body
        .find("<form method=\"get\" action=\"tabs\"><label")
        .unwrap();
    let selects = &ticked.body[start..start + ticked.body[start..].find("</form>").unwrap()];
    assert!(selects.contains("<select name=\"realm\">"), "{selects}");
    assert!(selects.contains("<select name=\"league\">"), "{selects}");
    assert!(!selects.contains("<input"), "{selects}");
    let other = web.get("tabs?realm=pc&league=Hardcore");
    assert!(
        other.body.contains("name=\"tab\" value=\"same\""),
        "{}",
        other.body
    );
    assert!(!other.body.contains(" checked"), "{}", other.body);
}

/// The first value a page gives after `key`, unescaped.
fn first_value(body: &str, key: &str) -> String {
    let at = body.find(key).unwrap_or_else(|| panic!("{key}: {body}")) + key.len();
    unescaped(&body[at..at + body[at..].find('"').unwrap()])
}

/// A place page's table form, or an answer's table's, as a browser sends
/// it, with the boxes `ticked` ticked besides those it holds ticked, and
/// the button saying `button` pressed (its `name` and `value` where it
/// carries them): the path it asks, under the button's `formaction` or
/// the form's action, its fields in the form's order.
fn place_form(
    web: &WebUi,
    body: &str,
    ticked: &[&str],
    button: &str,
    named: Option<(&str, &str)>,
) -> String {
    let start = body
        .find("<form method=\"get\" action=\"tabs\">\n")
        .or_else(|| body.find("<form method=\"get\" action=\"characters\">\n"))
        .or_else(|| body.find("<form class=\"table\" method=\"get\" action=\"search\">"))
        .unwrap_or_else(|| panic!("no table form: {body}"));
    let form = &body[start..start + body[start..].find("</form>").unwrap()];
    let action = attribute(&form[..form.find('>').unwrap()], "action").unwrap();
    let mut fields: Vec<(String, String)> = Vec::new();
    for input in form.split("<input ").skip(1) {
        let tag = format!(" {}", &input[..input.find('>').unwrap()]);
        let (Some(name), value) = (attribute(&tag, "name"), attribute(&tag, "value")) else {
            continue;
        };
        let value = value.unwrap_or_default();
        let sent = match attribute(&tag, "type").as_deref() {
            Some("checkbox") => tag.contains(" checked") || ticked.contains(&value.as_str()),
            _ => true,
        };
        if sent {
            fields.push((name, value));
        }
    }
    let pressed = form
        .split("<button ")
        .skip(1)
        .map(|b| {
            (
                format!(" {}", &b[..b.find('>').unwrap()]),
                &b[b.find('>').unwrap() + 1..],
            )
        })
        .find(|(tag, rest)| {
            rest.starts_with(&format!("{button}</button>"))
                && named.is_none_or(|(n, v)| {
                    attribute(tag, "name").as_deref() == Some(n)
                        && attribute(tag, "value").as_deref() == Some(v)
                })
        })
        .unwrap_or_else(|| panic!("no button {button} {named:?}: {form}"));
    let (tag, _) = pressed;
    if let (Some(name), Some(value)) = (attribute(&tag, "name"), attribute(&tag, "value")) {
        fields.push((name, value));
    }
    assert!(attribute(&tag, "formaction").is_none(), "{tag}");
    let query: Vec<String> = fields
        .iter()
        .map(|(k, v)| format!("{}={}", harness::form_encoded(k), harness::form_encoded(v)))
        .collect();
    let path = format!("/{}/{action}?{}", web.token, query.join("&"));
    // a plan asked is the place page's own field, answered `303` to the
    // plan with the page as sent as its `from` (the landing rule): the
    // path a browser lands on is that plan's
    if attribute(&tag, "name").as_deref() == Some("refresh") {
        let asked = follow(web, &path);
        assert_eq!(asked.status, 303, "{path}: {}", asked.body);
        let location = asked.header("location").unwrap().to_string();
        assert!(
            location.starts_with(&format!("/{}/plan?", web.token))
                && location.contains(&format!("&from={action}%3F")),
            "{location}"
        );
        return location;
    }
    path
}

/// The jobs page under a halt: a trip and a closed route at the cap staged
/// in the mock world's rails, as the rails tests stage them, and the
/// daemon started with the tripwire on. The page opens with the halt and
/// the closed route in its own words, then a job that waits under it, its
/// start said; `daemon status` says the same halt in the terminal's
/// words, which it alone now prints. No deadline but the hang ceiling
/// (C137).
#[test]
fn the_jobs_page_opens_with_the_halt() {
    let scratch = harness::scratch("webhalt");
    let base = scratch.0.clone();
    let login = harness::login_as_mock(&base, &[]);
    assert!(harness::acq(&base, &["daemon", "stop"]).status.success());
    while sole_json(&harness::acq(&base, &["daemon", "status", "--json"]))["running"] != false {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    // The test stopped the login's daemon itself, and the one it starts
    // next is its own: that one's kill leaves its socket on Unix, which
    // this guard, dropped after it, would count as a daemon left.
    drop(login);
    let rails = base.join("store").join("mock").join("rails.json");
    std::fs::create_dir_all(rails.parent().unwrap()).unwrap();
    // C190 beside the trip: a run of invalid answers on one league's
    // fetches closed the account's `stash` route, which every league's
    // tabs and substashes share, and its trials have reached the cap: the
    // next one an hour from the harness's now, which the page reads too.
    let now = harness::now();
    std::fs::write(
        &rails,
        format!(
            r#"{{"tripped":"429 on GET /stash/Standard (staged trip)","closed_routes":{{"stash@A#1":{{"invalid":13,"status":400,"closed_at":{now},"cooldown_secs":3600,"until":{}}}}}}}"#,
            now + 3600
        ),
    )
    .unwrap();
    let (_daemon, _pid) = harness::start_daemon_with(&base, &[("ACQ_TRIPWIRE", "1")]);
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let mut pages = Pages::new(&base, &web);
    // a listing submitted under the halt waits
    let out = harness::acq(&base, &["submit", "fetch", "--detach"]);
    assert!(out.status.success(), "{out:?}");
    let jobs = web.get("jobs");
    assert_eq!(jobs.status, 200, "{}", jobs.body);
    pages.page(
        "GET jobs <the daemon halted>",
        &jobs,
        &[
            "The daemon is sending nothing: 429 on GET /stash/Standard (staged trip)",
            "A person clears this stop",
            "Fetching tabs for A#1 is paused",
            "13 requests in a row",
            "tries again in ~60m",
            "an hour between tries",
            "whatever its league",
            "list the league again",
            "<td>A test fetch (the test world&#39;s)</td><td>waiting</td><td>submitted ",
        ],
    );
    // the terminal's words for the same halt stand where they stood when
    // this page printed them: `daemon status`, which no golden holds
    let status = harness::acq(&base, &["daemon", "status"]);
    let status = String::from_utf8(status.stdout).unwrap();
    for fact in [
        "rails:  tripwire ON",
        "HALTED: 429 on GET /stash/Standard (staged trip)",
        "clear with `acq daemon reset-tripwire` after the post-violation wait",
        "ROUTE CLOSED: stash@A#1 for 3600s",
        "one waiting job goes as the trial in ~60m",
        "at the cap: one trial an hour while jobs wait",
    ] {
        assert!(status.contains(fact), "{fact}: {status}");
    }
    page_golden(file!(), "jobs_halted", &pages);
    drop(web);
    assert!(harness::acq(&base, &["daemon", "stop"]).status.success());
}

/// The envelope a plan page holds in its hidden field, unescaped.
fn hidden_plan(body: &str) -> String {
    let at =
        body.find("name=\"plan\" value=\"").expect("a plan field") + "name=\"plan\" value=\"".len();
    let value = &body[at..at + body[at..].find('"').expect("the field's end")];
    value
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// The jobs page read until it no longer reloads itself: every job done.
/// No deadline but the hang ceiling (C137).
/// A submit's `303` to the jobs page (the landing rule): the location
/// names the job, at its row's anchor, carrying `from`, the page the
/// submit left as that page wrote it, where it was sent one; the page it
/// lands on draws that one way back, and none where none was sent. The
/// job's id and that page.
fn landed_at_a_job(
    web: &WebUi,
    reply: &harness::Reply,
    from: Option<&str>,
) -> (u64, harness::Reply) {
    assert_eq!(reply.status, 303, "{}", reply.body);
    let location = reply.header("location").expect("a location").to_string();
    let prefix = format!("/{}/jobs?job=", web.token);
    let rest = location
        .strip_prefix(&prefix)
        .unwrap_or_else(|| panic!("{location}"));
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    let id: u64 = rest[..digits].parse().unwrap();
    let query = match from {
        Some(from) => format!("&from={}", url_encoded(from)),
        None => String::new(),
    };
    assert_eq!(rest[digits..], format!("{query}#j-{id}"), "{location}");
    let (path, _) = location.split_once('#').unwrap();
    let page = web.raw("GET", path, None, &[], "");
    assert_eq!(page.status, 200, "{}", page.body);
    match from {
        Some(from) => {
            let back = format!("{}Back to ", back_link(web, from));
            assert!(page.body.contains(&back), "{back}: {}", page.body);
        }
        None => assert!(!page.body.contains("<p class=\"back\">"), "{}", page.body),
    }
    // the league's place links retired with `realm` and `league`: the way
    // back is the one link to where the submit came from
    assert!(!page.body.contains("<p class=\"places\">"), "{}", page.body);
    (id, page)
}

/// `text` as the server writes a URL component: everything but the
/// unreserved percent-encoded.
fn url_encoded(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            b => format!("%{b:02X}"),
        })
        .collect()
}

fn until_done(web: &WebUi) -> harness::Reply {
    loop {
        let jobs = web.get("jobs");
        assert_eq!(jobs.status, 200, "{}", jobs.body);
        if !jobs.body.contains("http-equiv=\"refresh\"") {
            return jobs;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// A jobs page with its probes' rows alike: the daemon starts the two
/// routes' first-contact probes together, so which takes the lower id is
/// the scheduler's; their rows say the same words, and their anchors'
/// ids are masked.
fn probes_in_order(mut jobs: harness::Reply) -> harness::Reply {
    let masked: Vec<String> = jobs
        .body
        .lines()
        .map(|line| match line.find("\" id=\"j-") {
            Some(at) if line.contains("A first check of a route") => {
                let digits = at + "\" id=\"j-".len();
                let end = digits
                    + line[digits..]
                        .bytes()
                        .take_while(u8::is_ascii_digit)
                        .count();
                format!("{}<n>{}", &line[..digits], &line[end..])
            }
            _ => line.to_string(),
        })
        .collect();
    jobs.body = masked.join("\n");
    jobs
}

/// The jobs page with no daemon, and a job opened by its URL: the
/// ledger's open rows with how old they are, a
/// child under its parent, and that the next job resumes them; a failed
/// parent the list leaves out, opened by `job`, above the list, its
/// failure by kind and its failed child a link to that child's own
/// result; that child opened, its refused token said in the page's words
/// and never the daemon's sentence, which names a command; the league
/// the URL names linked. The JSON is the `jobs` document whatever the
/// URL opens.
#[test]
fn the_jobs_page_opens_a_job_the_list_leaves_out_by_its_url() {
    use acquisition_store::jobs::{JobDb, JobRow, daemon_db_path};
    let scratch = harness::scratch("webjob");
    let base = scratch.0.clone();
    drop(harness::login(&base, "Alice"));
    let now = harness::now();
    let row = |id: u64, kind: &str, state: &str, parent: Option<u64>, params: Value| JobRow {
        id,
        kind: kind.into(),
        state: state.into(),
        priority: 0,
        submitted_by: format!("web:{id}"),
        parent,
        retries: 0,
        account: Some("Alice".into()),
        params,
        outcome: None,
        deferred: None,
        cancel_requested: false,
        submitted_at: now - 600,
        updated_at: now - 60,
        started_at: None,
        finished_at: None,
    };
    let tab = |id: &str| json!({ "realm": "pc", "league": "Standard", "id": id, "deep": false });
    let finished = |row: JobRow, outcome: Value| JobRow {
        outcome: Some(outcome),
        started_at: Some(now - 300),
        finished_at: Some(now - 120),
        ..row
    };
    let db = JobDb::open(&daemon_db_path(&base.join("store").join("mock"))).unwrap();
    let apply = json!({ "jobs": [
        { "kind": "stash", "params": tab("cur1") },
        { "kind": "stash", "params": tab("dump") },
    ] });
    for job in [
        finished(
            row(10, "apply", "failed", None, apply),
            json!({ "outcome": "failure", "kind": "children",
                    "error": "1 of 2 child jobs failed; see `acq result 12`",
                    "children": { "done": 1, "failed": 1, "cancelled": 0, "failed_ids": [12] } }),
        ),
        finished(
            row(11, "stash", "done", Some(10), tab("cur1")),
            json!({ "outcome": "success", "payload": {} }),
        ),
        finished(
            row(12, "stash", "failed", Some(10), tab("dump")),
            json!({ "outcome": "failure", "kind": "not_logged_in",
                    "error": "the token was refused: log in again with `acq auth`" }),
        ),
        row(20, "stash", "waiting", None, tab("maps")),
        JobRow {
            started_at: Some(now - 90),
            ..row(
                21,
                "stashes",
                "running",
                None,
                json!({ "realm": "pc", "league": "Standard" }),
            )
        },
        row(22, "stash", "waiting", Some(21), tab("uniq")),
        // another account's job, which the list holds beside Alice's
        JobRow {
            account: Some("Bob".into()),
            ..row(23, "stash", "waiting", None, tab("gems"))
        },
        // what was fetched and could not be recorded, kept with the result
        finished(
            row(30, "stash", "failed", None, tab("cur1")),
            json!({ "outcome": "failure", "kind": "not_recorded",
                    "error": "the store could not record it",
                    "payload": { "stash": {} } }),
        ),
        // a page GGG no longer has, listed by its parent tab's fetch
        finished(
            row(
                31,
                "stash",
                "failed",
                None,
                json!({ "realm": "pc", "league": "Standard", "id": "maps", "sub": "m002", "deep": false }),
            ),
            json!({ "outcome": "failure", "kind": "not_found", "error": "404 on the page" }),
        ),
    ] {
        db.upsert(&job).unwrap();
    }
    db.checkpoint().unwrap();
    drop(db);

    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let mut pages = Pages::new(&base, &web);
    let queue = web.get("jobs");
    pages.page(
        "GET jobs <no daemon>",
        &queue,
        &[
            "No daemon is running. The queue on record",
            "4 jobs, 3 waiting, 1 running when the daemon stopped",
            "The next job submitted starts a daemon, which resumes them",
            "<td>\u{21b3} Tab uniq in Standard</td><td>waiting</td>",
            // a job of an account the page does not serve says whose it
            // is
            "<td>Tab gems in Standard, for Bob</td><td>waiting</td>",
        ],
    );
    assert!(!queue.body.contains("for Alice"), "{}", queue.body);
    assert!(!queue.body.contains("j-10"), "{}", queue.body);
    let parent = web.get("jobs?job=10");
    pages.page(
        "GET jobs?job=10 <a failed parent the list leaves out>",
        &parent,
        &[
            "This job is not among those listed below",
            "<tr class=\"job opened\" id=\"j-10\"><td>A refresh of 2 requests</td><td>failed</td>",
            "Some of its requests failed",
            "Of its requests: 1 failed, 1 done.",
            "jobs?job=12#j-12\">Tab Standard/dump</a> failed",
        ],
    );
    let child = web.get("jobs?job=12&from=tabs%3Frealm%3Dpc%26league%3DStandard");
    pages.page(
        "GET jobs?job=12&from=tabs?realm=pc&league=Standard <a refused token opened>",
        &child,
        &[
            "Its account had no session to send with",
            "Signing in again is a person",
            "\">Back to the stash tabs</a></p>",
        ],
    );
    // a result's offer of itself is the page shown, and none
    let kept = web.get("jobs?job=30");
    pages.page(
        "GET jobs?job=30 <what was fetched, not recorded>",
        &kept,
        &["could not be saved"],
    );
    assert!(!kept.body.contains("jobs?job=30#j-30\""), "{}", kept.body);
    // a page's 404 is answered by its parent tab's fetch, so
    // the words name what lists it, not the league's list
    let gone = web.get("jobs?job=31");
    pages.page(
        "GET jobs?job=31 <a page GGG no longer has>",
        &gone,
        &["GGG has nothing there now", "what lists it"],
    );
    assert!(
        !gone.body.contains("the league or the characters"),
        "{}",
        gone.body
    );
    for page in [&parent, &child, &kept, &gone] {
        assert!(!page.body.contains("acq "), "{}", page.body);
        assert!(!page.body.contains("token was refused"), "{}", page.body);
    }
    assert_eq!(web.get_json("jobs?job=12"), web.get_json("jobs"));
    // a job no ledger holds, and a `job` that is no number, are refused
    assert_ne!(web.get("jobs?job=99").status, 200);
    assert_eq!(web.get("jobs?job=ten").status, 400);
    page_golden(file!(), "jobs_opened", &pages);
}

/// The served account, named on every submit: a server started with no
/// account on disk refuses every page as `not_logged_in` until Alice logs
/// in, then serves Alice, the account it found,
/// and keeps applying as Alice after Alice logs out and Bob logs in — the
/// daemon's sole live session, and a second account in the index — so
/// the plan page still renders for Alice, the apply of a selection's
/// plan is refused for Alice and no job runs as Bob; with both logged
/// in, where a submit naming no account would be ambiguous, the job is
/// Alice's.
#[test]
fn a_selection_submits_as_the_served_account_whatever_the_daemon_holds() {
    let scratch = harness::scratch("webacct");
    let base = scratch.0.clone();
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let none = web.get("plan?realm=pc&league=Standard&selected=yes&tab=dump");
    assert_eq!(none.status, 400, "{}", none.body);
    assert!(
        none.body.contains("log in with `acq auth`"),
        "{}",
        none.body
    );
    let _daemon = harness::login(&base, "Alice");
    let found = web.get("plan?realm=pc&league=Standard&selected=yes&tab=dump");
    assert_eq!(found.status, 200, "{}", found.body);
    assert!(found.body.contains("Alice"), "{}", found.body);
    // The mock world's keyring is off: the logout succeeds and says it
    // cleared nothing (C144).
    let out = harness::acq(&base, &["auth", "logout", "--account", "Alice"]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("logged out (keyring off, nothing cleared)"),
        "{out:?}"
    );
    let _login = harness::login(&base, "Bob");
    let plan = web.get("plan?realm=pc&league=Standard&selected=yes&tab=dump");
    assert_eq!(plan.status, 200, "{}", plan.body);
    assert!(plan.body.contains("Alice"), "{}", plan.body);
    // every page names the account it serves, the one found, not the
    // daemon's
    assert!(
        plan.body
            .contains("<span class=\"account\">account Alice</span>"),
        "{}",
        plan.body
    );
    assert!(!plan.body.contains("Bob"), "{}", plan.body);
    let envelope = hidden_plan(&plan.body);
    let refused = web.post("apply", &[("plan", &envelope)]);
    assert_eq!(refused.status, 400, "{}", refused.body);
    assert!(refused.body.contains("Alice"), "{}", refused.body);
    let jobs = |base: &Path| -> Vec<Value> {
        sole_json(&harness::acq(base, &["jobs", "--all", "--json"]))["rows"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|j| j["kind"] == "apply")
            .cloned()
            .collect()
    };
    assert!(jobs(&base).is_empty(), "{:?}", jobs(&base));
    let _again = harness::login(&base, "Alice");
    let submitted = web.post("apply", &[("plan", &envelope)]);
    assert_eq!(submitted.status, 303, "{}", submitted.body);
    let applies = jobs(&base);
    assert_eq!(applies.len(), 1, "{applies:?}");
    assert_eq!(applies[0]["account"], "Alice", "{}", applies[0]);
    drop(web);
    assert!(harness::acq(&base, &["daemon", "stop"]).status.success());
}

/// The plan page says "not signed in" only where the
/// daemon refused the quote for having no session for the account — not
/// where no daemon runs, which observed no session — and the plan's JSON
/// and the CLI's text say what they said before the cause was typed: the
/// page's JSON note is the sentence around the CLI's words, in both. The
/// pages are a golden: a policy's plan whose quote knows no timing, the
/// refused session, no daemon, and a route a run of invalid answers
/// closed (C190), whose requests the quote gives no time and the page
/// says paused.
#[test]
fn a_plan_says_not_signed_in_only_where_the_daemon_refused_the_session() {
    let scratch = harness::scratch("websession");
    let base = scratch.0.clone();
    let daemon = harness::login(&base, "Alice");
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let mut pages = Pages::new(&base, &web);
    let policy = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["dump"],"max_age_seconds":3600}}}}}"#;
    assert!(
        harness::acq(&base, &["policy", "set", policy])
            .status
            .success()
    );
    let path = "plan?realm=pc&league=Standard";
    let cli = |base: &Path| {
        let args = ["refresh", "--plan"];
        let text = harness::acq(base, &args);
        assert!(text.status.success(), "{text:?}");
        let json = sole_json(&harness::acq(base, &[&args[..], &["--json"]].concat()));
        (String::from_utf8(text.stdout).unwrap(), json)
    };
    let unknown = web.get(path);
    pages.page(
        "GET plan?<every timing unknown>",
        &unknown,
        &[
            "The timing of 1 request is not yet known",
            "Your sync policy names 1 tab not on record",
            ">Apply</button>",
        ],
    );
    assert!(!unknown.body.contains("signed in"), "{}", unknown.body);

    // Alice logged out: the daemon holds no session, and refuses the quote
    let out = harness::acq(&base, &["auth", "logout", "--account", "Alice"]);
    assert!(out.status.success(), "{out:?}");
    let refused = web.get(path);
    pages.page(
        "GET plan?<the daemon holds no session>",
        &refused,
        &[
            "Not signed in: the daemon holds no session",
            ">Apply</button>",
        ],
    );
    // the CLI's decision view names the daemon's words, as it did when
    // the note was `Other`; the page's JSON note is `Other`'s sentence
    // around the same words, and the CLI's envelope carries no note
    let (text, json) = cli(&base);
    assert!(json.get("quote_note").is_none(), "{json}");
    let said = text
        .lines()
        .find_map(|l| l.strip_prefix("quote: unavailable — "))
        .unwrap_or_else(|| panic!("{text}"));
    assert!(said.contains("session"), "{text}");
    assert_eq!(
        web.get_json(path)["quote_note"],
        format!("no quote: {said} — plan compiled offline")
    );

    // no daemon: nothing was observed of a session, so nothing is said
    drop(daemon);
    let none = web.get(path);
    pages.page(
        "GET plan?<no daemon>",
        &none,
        &["No daemon is running", ">Apply</button>"],
    );
    assert!(!none.body.contains("signed in"), "{}", none.body);
    let (text, json) = cli(&base);
    assert!(json.get("quote_note").is_none(), "{json}");
    assert_eq!(
        web.get_json(path)["quote_note"],
        "no quote: no daemon running (the plan needs none; a running daemon adds its ETA)"
    );
    assert!(
        text.contains("quote: unavailable — no daemon running\n"),
        "{text}"
    );

    // a closed route, staged in the world's rails as the rails tests stage
    // it before the next daemon starts: its request has no time, said
    // paused and not unlearned
    let now = harness::now();
    std::fs::write(
        base.join("store").join("mock").join("rails.json"),
        format!(
            r#"{{"closed_routes":{{"stash-list@Alice":{{"invalid":5,"status":400,"closed_at":{now},"cooldown_secs":60,"until":{}}}}}}}"#,
            now + 60
        ),
    )
    .unwrap();
    let _again = harness::login(&base, "Alice");
    let paused = web.get(path);
    pages.page(
        "GET plan?<a closed route>",
        &paused,
        &[
            "The timing of 1 request is not known: GGG",
            "paused its kind of request",
            ">Apply</button>",
        ],
    );
    assert!(!paused.body.contains("not yet known"), "{}", paused.body);
    page_golden(file!(), "plan_session", &pages);
}

// ---- the saved pages (C16, C168) -------------------------------------

/// A form a page prints, as a browser sends it: its action, its button's
/// words, and every field it carries, hidden or shown, unescaped.
#[derive(Clone, Debug)]
struct PageForm {
    action: String,
    button: String,
    fields: Vec<(String, String)>,
}

impl PageForm {
    /// The value of the field `name`, where the form carries it.
    fn field(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// The form with the field `name` set to `value`, as a reader types
    /// it into the shown input.
    fn typed(mut self, name: &str, value: &str) -> PageForm {
        match self.fields.iter_mut().find(|(k, _)| k == name) {
            Some(field) => field.1 = value.to_string(),
            None => panic!("no field {name} in {self:?}"),
        }
        self
    }
}

fn unescaped(text: &str) -> String {
    text.replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// An attribute's value in one tag, unescaped.
fn attribute(tag: &str, name: &str) -> Option<String> {
    let key = format!(" {name}=\"");
    let at = tag.find(&key)? + key.len();
    Some(unescaped(&tag[at..at + tag[at..].find('"')?]))
}

/// Every form a page prints that posts, in order.
fn posted_forms(body: &str) -> Vec<PageForm> {
    body.split("<form ")
        .skip(1)
        .filter_map(|rest| {
            let form = &rest[..rest.find("</form>")?];
            let head = &form[..form.find('>')?];
            if attribute(&format!(" {head}"), "method").as_deref() != Some("post") {
                return None;
            }
            let fields = form
                .split("<input ")
                .skip(1)
                .filter_map(|input| {
                    let tag = format!(" {}", &input[..input.find('>')?]);
                    Some((
                        attribute(&tag, "name")?,
                        attribute(&tag, "value").unwrap_or_default(),
                    ))
                })
                .collect();
            let button = form.rsplit_once("<button type=\"submit\">")?.1;
            Some(PageForm {
                action: attribute(&format!(" {head}"), "action")?,
                button: unescaped(&button[..button.find("</button>")?]),
                fields,
            })
        })
        .collect()
}

/// The one form on a page whose button says `button` and, where `id` is
/// given, that names that saved search.
fn form_on(body: &str, button: &str, id: Option<&str>) -> PageForm {
    let found: Vec<PageForm> = posted_forms(body)
        .into_iter()
        .filter(|f| f.button == button && id.is_none_or(|id| f.field("id") == Some(id)))
        .collect();
    let [form] = found.as_slice() else {
        panic!("{} forms say {button:?} for {id:?}:\n{body}", found.len())
    };
    form.clone()
}

/// A form posted as a browser posts it, from the server's own page; with
/// `json`, asking for the document.
fn send(web: &WebUi, form: &PageForm, json: bool) -> harness::Reply {
    let body = form
        .fields
        .iter()
        .map(|(k, v)| format!("{}={}", harness::form_encoded(k), harness::form_encoded(v)))
        .collect::<Vec<_>>()
        .join("&");
    let origin = format!("http://{}", web.addr);
    let mut headers = vec![
        ("Origin", origin.as_str()),
        ("Content-Type", "application/x-www-form-urlencoded"),
    ];
    if json {
        headers.push(("Accept", "application/json"));
    }
    // a relative action, as a browser resolves it from a page of the
    // server's (every page stands one segment under the token)
    let action = match form.action.starts_with('/') {
        true => form.action.clone(),
        false => format!("/{}/{}", web.token, form.action),
    };
    assert!(action.starts_with(&format!("/{}/", web.token)), "{form:?}");
    web.raw("POST", &action, None, &headers, &body)
}

/// A JSON document's offer or undo that is a page's write, as a form.
fn offered_form(offer: &Value) -> PageForm {
    PageForm {
        action: offer["request"]["post"]
            .as_str()
            .unwrap_or_else(|| panic!("no write: {offer}"))
            .to_string(),
        button: offer["says"].as_str().unwrap_or_default().to_string(),
        fields: offer["request"]["fields"]
            .as_object()
            .unwrap_or_else(|| panic!("no fields: {offer}"))
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect(),
    }
}

/// A document posted as JSON: its status and its body read.
fn sent_json(web: &WebUi, form: &PageForm) -> (u16, Value) {
    let reply = send(web, form, true);
    let doc = serde_json::from_str(&reply.body)
        .unwrap_or_else(|e| panic!("{form:?}: {e}: {}", reply.body));
    (reply.status, doc)
}

/// A page under its absolute path, as a browser follows a link or a
/// redirect.
fn follow(web: &WebUi, path: &str) -> harness::Reply {
    web.raw("GET", &unescaped(path), None, &[], "")
}

/// A write answered as a browser asks it: `303` to the list, carrying the
/// page the write left as `from` (the landing rule), or to the list as
/// sent where the write left the list itself; the list it lands on draws
/// that way back.
fn landed_on_the_list(web: &WebUi, reply: &harness::Reply) {
    assert_eq!(reply.status, 303, "{}", reply.body);
    assert!(reply.body.is_empty(), "{}", reply.body);
    let location = reply.header("location").expect("a location").to_string();
    let list = format!("/{}/saved", web.token);
    let from = match location.strip_prefix(&list) {
        Some("") | Some("?expand=true") => None,
        Some(rest) => Some(
            rest.strip_prefix("?from=")
                .unwrap_or_else(|| panic!("{location}")),
        ),
        None => panic!("{location}"),
    };
    let landed = follow(web, &location);
    assert_eq!(landed.status, 200, "{}", landed.body);
    assert_eq!(
        landed.body.contains("<p class=\"back\">"),
        from.is_some(),
        "{location}: {}",
        landed.body
    );
}

/// A clear as a page makes it, asking first:
/// the list's Clear link for `id`, never a form, followed to the asking
/// page, which writes nothing (the file's revision unchanged across it),
/// and the clear's one form on that page.
fn asked_clear(web: &WebUi, world: &Path, list: &str, id: &str) -> (harness::Reply, PageForm) {
    let prefix = format!("href=\"/{}/saved/clear?id={id}", web.token);
    let at = list
        .find(&prefix)
        .unwrap_or_else(|| panic!("no Clear link for {id}:\n{list}"))
        + "href=\"".len();
    let link = &list[at..at + list[at..].find('"').unwrap()];
    assert!(
        list[at..].starts_with(&format!("{link}\">Clear</a>")),
        "no Clear link for {id}:\n{list}"
    );
    // the clear's ask is a step: it hands on the list it was asked from
    assert!(link.contains("&amp;from=saved"), "{link}");
    let before = file_revision(world);
    let asking = follow(web, link);
    assert_eq!(asking.status, 200, "{}", asking.body);
    assert_eq!(file_revision(world), before, "asking wrote");
    let form = form_on(&asking.body, "Clear", Some(id));
    (asking, form)
}

/// `acq saved <args> --json` in the page's world: its one document.
fn saved_json(world: &Path, args: &[&str]) -> Value {
    let mut args: Vec<&str> = ["saved"].iter().chain(args).copied().collect();
    args.push("--json");
    sole_json(&offline_acq(world, &args))
}

/// The user file's revision, as `acq saved list` reads it.
fn file_revision(world: &Path) -> i64 {
    saved_json(world, &["list"])["revision"].as_i64().unwrap()
}

/// `acq saved <args>` in the page's world, which must succeed.
fn saved_cli(world: &Path, args: &[&str]) {
    let args: Vec<&str> = ["saved"].iter().chain(args).copied().collect();
    let out = offline_acq(world, &args);
    assert!(out.status.success(), "{args:?}: {out:?}");
}

/// The head's basis line as its JSON object, where the page carries one.
fn basis_line(body: &str) -> Option<Value> {
    let line = body
        .lines()
        .find(|l| l.starts_with("<meta name=\"basis\""))?;
    let content = attribute(&format!(" {line}"), "content")?;
    Some(serde_json::from_str(&content).unwrap_or_else(|e| panic!("{e}: {content}")))
}

/// A page's document as the same URL answers it as JSON.
fn json_at(web: &WebUi, path: &str) -> Value {
    let reply = web.raw(
        "GET",
        &unescaped(path),
        None,
        &[("Accept", "application/json")],
        "",
    );
    serde_json::from_str(&reply.body).unwrap_or_else(|e| panic!("{path}: {e}: {}", reply.body))
}

/// The redirect a link answers, followed: the page it lands on, and the
/// location.
fn redirected(web: &WebUi, path: &str) -> (String, harness::Reply) {
    let reply = follow(web, path);
    assert_eq!(reply.status, 303, "{path}: {}", reply.body);
    let location = reply.header("location").unwrap().to_string();
    (location.clone(), follow(web, &location))
}

/// The link on a page whose words are `says`.
fn link_saying(body: &str, says: &str) -> String {
    let end = body
        .find(&format!("\">{says}</a>"))
        .unwrap_or_else(|| panic!("no link says {says:?}:\n{body}"));
    let start = body[..end].rfind("href=\"").unwrap() + "href=\"".len();
    unescaped(&body[start..end])
}

/// An answer's saved parts: the notices above it and the save forms
/// under its query box, the answer itself apart.
fn saved_parts(body: &str) -> String {
    let notices = body
        .lines()
        .filter(|l| l.starts_with("<p class=\"saved\">"))
        .collect::<Vec<_>>()
        .join("\n");
    let forms = body
        .split_once("<div class=\"save\">")
        .map(|(_, rest)| &rest[..rest.find("</div>").unwrap_or(rest.len())])
        .unwrap_or_default();
    assert!(!forms.is_empty(), "no save form:\n{body}");
    format!("{notices}\n{forms}")
}

/// The page's `<main>` alone: what its words are, the nav apart.
fn main_of(body: &str) -> &str {
    let open = body.find("<main>").unwrap_or(0);
    let close = body.find("</main>").unwrap_or(body.len());
    &body[open..close]
}

/// The words of the scan (C16): a script that names any of them
/// reaches a storage, a cookie or the history.
const NO_STATE: [&str; 8] = [
    "localStorage",
    "sessionStorage",
    "indexedDB",
    "caches",
    "cookie",
    "history",
    "serviceWorker",
    ".storage",
];

/// The words of the scan a script's text holds.
fn state_words(script: &str) -> Vec<&'static str> {
    NO_STATE
        .iter()
        .copied()
        .filter(|w| script.contains(w))
        .collect()
}

/// The prefixes of the search form's field names (`form.rs`): a script
/// that names one fills or composes what a form sends, which is the
/// server's (C46; `4-full-1-form-rows.md`, Decision 3).
const FIELD_PREFIXES: [&str; 4] = ["stat.", "group.", "query.", "view."];

/// The form's field names a script's text holds: a prefix standing as a
/// word's start and a letter after its dot, so a sentence ending in
/// "group." is none.
fn field_names(script: &str) -> Vec<String> {
    let mut out = Vec::new();
    for prefix in FIELD_PREFIXES {
        for (at, _) in script.match_indices(prefix) {
            let before = script[..at].chars().next_back();
            let after = script[at + prefix.len()..].chars().next();
            if !before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.')
                && after.is_some_and(|c| c.is_ascii_lowercase())
            {
                let end = script[at..]
                    .find(|c: char| !(c.is_alphanumeric() || c == '.' || c == '_'))
                    .map_or(script.len(), |n| at + n);
                out.push(script[at..end].to_string());
            }
        }
    }
    out
}

/// C16's state sentence, as far as a lexical scan holds it: every page a
/// golden records passes the harness's seam (`harness::no_state`: no
/// cookie set, no script named off `harness::SCRIPTS`), and each script
/// on that list, fetched as a page fetches it, names no storage, cookie
/// or history API, and no field of the search form ([`field_names`]). The
/// breakers: each word in a script's text is refused, a field of each
/// prefix is, and the seam refuses a reply that sets a cookie and one
/// naming a script off the list. What another spelling reaches is the
/// review's (`REVIEW-CHECKLIST.md`).
#[test]
fn c16_no_served_script_names_a_storage_cookie_or_history_api() {
    for word in NO_STATE {
        let broken = format!("(function () {{ const x = window.{word}; }})();");
        assert_eq!(state_words(&broken), vec![word], "the scan misses {word}");
    }
    for prefix in FIELD_PREFIXES {
        let broken = format!("(function () {{ el.name = '{prefix}min'; }})(); // a group. Then");
        assert_eq!(
            field_names(&broken),
            vec![format!("{prefix}min")],
            "the scan misses {prefix}"
        );
    }
    let (_scratch, web) = offline_web("webscan");
    let token = web.token.clone();
    let reply = |headers: Vec<(&str, &str)>, body: String| harness::Reply {
        status: 200,
        headers: headers
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        body,
    };
    let listed = format!("<script src=\"/{token}/assets/lists.js\"></script>");
    assert_eq!(
        harness::no_state(&reply(vec![], listed.clone()), &token),
        Ok(())
    );
    let cookie = harness::no_state(&reply(vec![("set-cookie", "seen=1")], listed), &token);
    assert!(cookie.is_err(), "the seam passed a cookie");
    let unlisted = harness::no_state(
        &reply(
            vec![],
            format!("<script src=\"/{token}/assets/keep.js\"></script>"),
        ),
        &token,
    );
    assert!(
        unlisted.is_err(),
        "the seam passed a script the scan never reads"
    );
    // the form page names the listed script, so the list is the pages' own
    let form = web.get("");
    assert_eq!(harness::no_state(&form, &token), Ok(()));
    assert!(form.body.contains("/assets/lists.js\""), "{}", form.body);
    // the count's script is on every page with a form that submits a
    // search: the form and an answer
    assert!(form.body.contains("/assets/count.js\""), "{}", form.body);
    let answer = web.get("search?realm=pc&text=class%3Arings");
    assert_eq!(answer.status, 200, "{}", answer.body);
    assert!(
        answer.body.contains("/assets/count.js\""),
        "{}",
        answer.body
    );
    assert!(answer.body.contains("data-count>"), "{}", answer.body);
    for path in harness::SCRIPTS {
        let script = web.get(path);
        assert_eq!(script.status, 200, "{path}");
        assert!(script.header("set-cookie").is_none(), "{path}");
        assert_eq!(
            state_words(&script.body),
            Vec::<&str>::new(),
            "{path} names a storage, cookie or history API"
        );
        assert_eq!(
            field_names(&script.body),
            Vec::<String>::new(),
            "{path} names a field of the search form"
        );
    }
}

/// C16, every document carries its basis: each page's basis line holds,
/// for each document it renders that carries a version, that document's
/// own field as the same URL answers it as JSON — the answer's and the
/// item's `basis`, the list's `revision`, the row's `revision` as `acq
/// saved show` reads it on an opened answer and on a clear asked first, a
/// conflict's `current.revision` — and a page
/// whose documents carry none carries no line. The plan's is pinned in
/// the refresh loop.
#[test]
fn c16_every_page_s_basis_line_is_its_documents_own() {
    let (scratch, web) = offline_web("webbasis");
    let world = scratch.0.clone();
    let mock = world.join("mock");
    let token = web.token.clone();
    let at = |path: &str| format!("/{token}/{path}");
    let entry = |body: &str, key: &str| -> Value {
        basis_line(body).unwrap_or_else(|| panic!("no basis line:\n{body}"))[key].clone()
    };
    // an answer from the form, an edited text, a counted answer, an item
    for path in [
        format!("search?{}", query_string(&ring_search())),
        "search?realm=pc&text=class%3Arings+%22%23+to+maximum+Life%22%3E%3D90".to_string(),
        "search?realm=pc&league=Standard&view.count=rarity&view.limit=20".to_string(),
    ] {
        let page = follow(&web, &at(&path));
        assert_eq!(page.status, 200, "{path}: {}", page.body);
        let line = basis_line(&page.body).unwrap();
        assert_eq!(line.as_object().unwrap().len(), 1, "{line}");
        assert_eq!(line["answer"], json_at(&web, &at(&path))["basis"], "{path}");
    }
    // a continuation, followed as the page links it
    let first = web.get("search?realm=pc&text=class%3Arings&view.limit=1");
    let more = link_saying(&first.body, "more");
    let next = follow(&web, &more);
    assert_eq!(next.status, 200, "{}", next.body);
    assert_eq!(entry(&next.body, "answer"), json_at(&web, &more)["basis"]);
    let item = web.get("item/r1");
    assert_eq!(
        entry(&item.body, "item"),
        json_at(&web, &at("item/r1"))["basis"]
    );
    // the list, empty and full: two rows, one written twice, so the file's
    // revision is no count the page could make of its rows
    for full in [false, true] {
        if full {
            crate::saved_json::put(&mock, "0000000000a1", "rings");
            crate::saved_json::put(&mock, "0000000000a2", "amulets");
            saved_cli(
                &world,
                &["set", "--id", "0000000000a2", "--name", "necklaces"],
            );
        }
        let list = web.get("saved");
        assert_eq!(
            entry(&list.body, "saved_searches"),
            json_at(&web, &at("saved"))["revision"]
        );
    }
    // an answer opened from a row: the answer's and the row's
    let (location, opened) = redirected(&web, &at("saved/open?id=0000000000a1"));
    assert_eq!(
        entry(&opened.body, "answer"),
        json_at(&web, &location)["basis"]
    );
    assert_eq!(
        entry(&opened.body, "saved_search"),
        saved_json(&world, &["show", "0000000000a1"])["revision"]
    );
    // a conflict's page: the row as it stands
    saved_cli(&world, &["set", "--id", "0000000000a1", "--name", "moved"]);
    let save = form_on(&opened.body, "Save over \"rings\"", None);
    let conflict = send(&web, &save, false);
    assert_eq!(conflict.status, 400, "{}", conflict.body);
    let (_, refused) = sent_json(&web, &save);
    assert_eq!(refused["kind"], "conflict", "{refused}");
    assert_eq!(
        entry(&conflict.body, "saved_search"),
        refused["current"]["revision"]
    );
    // a clear asked first: the row it would clear, as its JSON and the
    // terminal read it
    let asking = at("saved/clear?id=0000000000a1");
    let asked = entry(&follow(&web, &asking).body, "saved_search");
    assert_eq!(asked, json_at(&web, &asking)["revision"]);
    assert_eq!(
        asked,
        saved_json(&world, &["show", "0000000000a1"])["revision"]
    );
    // a place page under a query with a place open: the places document's
    // basis and its segment's window's, one basis; a place open with no
    // query, the window's alone; nothing open and no query, no line
    let path = "tabs?realm=pc&league=Standard&text=class%3Arings&open=t1";
    let page = web.get(path);
    let line = basis_line(&page.body).unwrap_or_else(|| panic!("no basis line:\n{}", page.body));
    assert_eq!(line["places"], json_at(&web, &at(path))["basis"], "{line}");
    let route = |body: &str| {
        let at = body.find("data-route=\"").expect("a segment") + "data-route=\"".len();
        unescaped(&body[at..at + body[at..].find('"').unwrap()])
    };
    assert_eq!(
        line["rows"],
        json_at(&web, &format!("{}&offset=0&count=1", route(&page.body)))["basis"],
        "{line}"
    );
    assert_eq!(line["places"], line["rows"], "one basis");
    let path = "tabs?realm=pc&league=Standard&open=t1";
    let page = web.get(path);
    let line = basis_line(&page.body).unwrap_or_else(|| panic!("no basis line:\n{}", page.body));
    assert_eq!(line.as_object().unwrap().len(), 1, "{line}");
    assert_eq!(
        line["rows"],
        json_at(&web, &format!("{}&offset=0&count=1", route(&page.body)))["basis"]
    );
    assert_eq!(
        basis_line(&web.get("tabs?realm=pc&league=Standard").body),
        None
    );
    // the form page and a refusal with no `current` carry no line
    assert_eq!(basis_line(&web.get("").body), None);
    assert_eq!(basis_line(&web.get("item/t1").body), None);
    assert_eq!(
        basis_line(&web.get("saved/open?id=0000000000ff").body),
        None
    );
}

/// C168, a page writes as the verbs write: a create from an answer, a
/// rename and a clear from the list, each as its form sends it — as JSON
/// the receipt, its `written` the row `acq saved show` reads, through the
/// channel `web`, and its undo posted as spelled putting the prior back;
/// as a browser sends it, `303` to the list with the same write landed.
#[test]
fn c168_a_page_writes_a_saved_search_as_the_verbs_write_it() {
    let (scratch, web) = offline_web("webwrite");
    let world = scratch.0.clone();
    let answer = web.get(&format!("search?{}", query_string(&ring_search())));
    let save = form_on(&answer.body, "Save this search", None);
    assert!(
        !saved_parts(&answer.body).contains("acq "),
        "{}",
        answer.body
    );
    // the browser's create: 303 to the list, the row on it
    landed_on_the_list(
        &web,
        &send(&web, &save.clone().typed("name", "rings"), false),
    );
    let listed = saved_json(&world, &["list"]);
    assert_eq!(listed["total"], 1, "{listed}");
    let row = &listed["searches"][0];
    assert_eq!(
        (&row["name"], &row["written_via"], &row["scope"]["realm"]),
        (&json!("rings"), &json!("web"), &json!("pc")),
        "{row}"
    );
    let browsed = row["id"].as_str().unwrap().to_string();
    // the create asked as JSON: the receipt, its row the verbs' own
    let (status, created) = sent_json(&web, &save.clone().typed("name", "rings again"));
    assert_eq!(status, 200, "{created}");
    let id = created["target"]["id"].as_str().unwrap().to_string();
    assert_eq!(
        created["written"],
        saved_json(&world, &["show", &id]),
        "{created}"
    );
    assert_eq!(created["written"]["written_via"], "web");
    // its undo, the clear, posted as spelled
    let (status, undone) = sent_json(&web, &offered_form(&created["undo"]));
    assert_eq!(status, 200, "{undone}");
    assert_eq!(
        saved_json(&world, &["show", &id])["kind"],
        "saved_search_not_found"
    );
    // a rename from the list, as the browser sends it
    let list = web.get("saved");
    assert!(!main_of(&list.body).contains("acq "), "{}", list.body);
    let rename = form_on(&list.body, "Rename", Some(&browsed));
    assert_eq!(
        rename.field("text"),
        None,
        "a rename sends no text: {rename:?}"
    );
    landed_on_the_list(
        &web,
        &send(&web, &rename.clone().typed("name", "kept rings"), false),
    );
    let shown = saved_json(&world, &["show", &browsed]);
    assert_eq!(
        (&shown["name"], &shown["revision"]),
        (&json!("kept rings"), &json!(2))
    );
    // and as JSON, from the list as it stands; its undo puts the prior back
    let rename = form_on(&web.get("saved").body, "Rename", Some(&browsed));
    let (status, renamed) = sent_json(&web, &rename.typed("name", "renamed"));
    assert_eq!(status, 200, "{renamed}");
    assert_eq!(renamed["written"], saved_json(&world, &["show", &browsed]));
    assert_eq!(renamed["prior"]["name"], "kept rings");
    let (status, _) = sent_json(&web, &offered_form(&renamed["undo"]));
    assert_eq!(status, 200);
    assert_eq!(
        saved_json(&world, &["show", &browsed])["name"],
        "kept rings"
    );
    // a clear from the list as JSON, its undo the create at the id
    let (_, clear) = asked_clear(&web, &world, &web.get("saved").body, &browsed);
    let (status, cleared) = sent_json(&web, &clear);
    assert_eq!(status, 200, "{cleared}");
    assert_eq!(cleared["written"], Value::Null);
    assert_eq!(
        saved_json(&world, &["show", &browsed])["kind"],
        "saved_search_not_found"
    );
    let undo = offered_form(&cleared["undo"]);
    assert_eq!(undo.field("if_revision"), Some("0"), "{undo:?}");
    let (status, back) = sent_json(&web, &undo);
    assert_eq!(status, 200, "{back}");
    assert_eq!(back["written"], saved_json(&world, &["show", &browsed]));
    assert_eq!(back["written"]["name"], "kept rings");
    // and as the browser sends it: 303, the row gone
    let (_, clear) = asked_clear(&web, &world, &web.get("saved").body, &browsed);
    landed_on_the_list(&web, &send(&web, &clear, false));
    assert_eq!(saved_json(&world, &["list"])["total"], 0);
}

/// C168, a stale page: a write or a clear at a revision that moved is
/// refused `conflict`, nothing written, and the write offered again over
/// the row as it stands lands, posted as the conflict's page renders it;
/// a clear's undo posted twice meets the row it put back, and its offer,
/// the list whole, shows that row past the first ten; a create naming no
/// realm over a store of two is refused `realm_needed`, each realm's
/// create offered landing.
#[test]
fn c168_a_stale_page_is_refused_and_offered_the_write_over_the_row_as_it_stands() {
    let (scratch, web) = offline_web("webstale");
    let world = scratch.0.clone();
    let mock = world.join("mock");
    for n in 0..11 {
        crate::saved_json::put(&mock, &format!("00000000000{n:x}"), &format!("a{n:02}"));
    }
    crate::saved_json::put(&mock, "0000000000ff", "zz");
    let list = web.get("saved?expand=true");
    // the clear asked at the revision listed, before anything moves
    let (_, clear) = asked_clear(&web, &world, &list.body, "0000000000ff");
    // the row renamed at the terminal; the page's rename at the old one
    saved_cli(
        &world,
        &["set", "--id", "0000000000ff", "--name", "elsewhere"],
    );
    let before = file_revision(&world);
    let rename = form_on(&list.body, "Rename", Some("0000000000ff")).typed("name", "mine");
    let refused = send(&web, &rename, false);
    assert_eq!(refused.status, 400, "{}", refused.body);
    let (_, conflict) = sent_json(&web, &rename);
    assert_eq!(
        (
            &conflict["kind"],
            &conflict["revision"],
            &conflict["current"]["name"]
        ),
        (&json!("conflict"), &json!(2), &json!("elsewhere")),
        "{conflict}"
    );
    assert_eq!(file_revision(&world), before, "a refused write wrote");
    assert!(
        refused.body.contains("It stands now as:"),
        "{}",
        refused.body
    );
    assert!(!main_of(&refused.body).contains("acq "), "{}", refused.body);
    let again = form_on(
        &refused.body,
        "the same write over revision 2, the row as it stands",
        None,
    );
    landed_on_the_list(&web, &send(&web, &again, false));
    let shown = saved_json(&world, &["show", "0000000000ff"]);
    assert_eq!(
        (&shown["name"], &shown["revision"]),
        (&json!("mine"), &json!(3))
    );
    // a clear at a revision that moved, its offer posted as rendered
    let before = file_revision(&world);
    let refused = send(&web, &clear, false);
    assert_eq!(refused.status, 400, "{}", refused.body);
    assert_eq!(file_revision(&world), before);
    let again = form_on(
        &refused.body,
        "the same clear at revision 3, the row as it stands",
        None,
    );
    landed_on_the_list(&web, &send(&web, &again, false));
    // a clear's undo posted twice: the second a create meeting the row it
    // put back, offered the list whole, where that row stands past ten
    let (_, clear) = asked_clear(
        &web,
        &world,
        &web.get("saved?expand=true").body,
        "00000000000a",
    );
    let (_, cleared) = sent_json(&web, &clear);
    let undo = offered_form(&cleared["undo"]);
    assert_eq!(sent_json(&web, &undo).0, 200);
    let before = file_revision(&world);
    let twice = send(&web, &undo, false);
    assert_eq!(twice.status, 400, "{}", twice.body);
    assert_eq!(file_revision(&world), before);
    let every = link_saying(&twice.body, "the saved search 00000000000a as it stands");
    assert!(every.ends_with("/saved?expand=true"), "{every}");
    let followed = follow(&web, &every);
    assert_eq!(followed.status, 200);
    assert!(
        followed.body.contains("<td>00000000000a</td>"),
        "{}",
        followed.body
    );
    assert!(
        !web.get("saved").body.contains("<td>00000000000a</td>"),
        "the row stands in the first ten"
    );
    // a create naming no realm, posted by hand over pc and xbox
    let by_hand = PageForm {
        action: format!("/{}/saved", web.token),
        button: String::new(),
        fields: vec![
            ("name".to_string(), "life".to_string()),
            ("text".to_string(), "rarity=rare".to_string()),
        ],
    };
    let before = file_revision(&world);
    let refused = send(&web, &by_hand, false);
    assert_eq!(refused.status, 400, "{}", refused.body);
    assert_eq!(sent_json(&web, &by_hand).1["kind"], "realm_needed");
    assert_eq!(file_revision(&world), before);
    let offers: Vec<PageForm> = posted_forms(&refused.body);
    assert!(offers.len() >= 2, "{}", refused.body);
    for offer in &offers {
        landed_on_the_list(&web, &send(&web, offer, false));
    }
    let realms: Vec<Value> = saved_json(&world, &["list", "--expand"])["searches"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["name"] == "life")
        .map(|row| row["scope"]["realm"].clone())
        .collect();
    for realm in ["pc", "xbox"] {
        assert!(realms.contains(&json!(realm)), "{realms:?}");
    }
}

/// C168 over the text `""`, a query of every item in its scope: saved
/// from its answer, opened from the list, saved over and cleared through
/// the forms as rendered, the clear undone from its receipt's undo.
#[test]
fn c168_a_search_over_every_item_saves_opens_and_clears_through_the_forms() {
    let (scratch, web) = offline_web("webevery");
    let world = scratch.0.clone();
    let answer = web.get("search?realm=pc&text=");
    assert_eq!(answer.status, 200, "{}", answer.body);
    let save = form_on(&answer.body, "Save this search", None);
    assert_eq!(save.field("text"), Some(""), "{save:?}");
    landed_on_the_list(&web, &send(&web, &save.typed("name", "everything"), false));
    let list = web.get("saved");
    let row = &saved_json(&world, &["list"])["searches"][0];
    let id = row["id"].as_str().unwrap().to_string();
    assert_eq!(row["text"], "", "{row}");
    let (location, opened) = redirected(&web, &link_saying(&list.body, "everything"));
    assert!(location.contains("text=&"), "{location}");
    assert_eq!(opened.status, 200, "{}", opened.body);
    assert!(
        opened
            .body
            .contains("Saved search &quot;everything&quot;, opened at revision 1"),
        "{}",
        opened.body
    );
    let over = form_on(&opened.body, "Save over \"everything\"", None).typed("name", "all of it");
    landed_on_the_list(&web, &send(&web, &over, false));
    let shown = saved_json(&world, &["show", &id]);
    assert_eq!(
        (&shown["name"], &shown["text"]),
        (&json!("all of it"), &json!(""))
    );
    let (_, clear) = asked_clear(&web, &world, &web.get("saved").body, &id);
    let (status, cleared) = sent_json(&web, &clear);
    assert_eq!(status, 200, "{cleared}");
    let (status, back) = sent_json(&web, &offered_form(&cleared["undo"]));
    assert_eq!(status, 200, "{back}");
    assert_eq!(back["written"]["text"], "");
    assert_eq!(back["written"]["name"], "all of it");
}

/// A write's and an opening's malformed inputs, and a clear asked first
/// with no row to ask about: each refused, nothing written.
#[test]
fn c168_each_malformed_write_or_opening_is_refused_and_writes_nothing() {
    let (scratch, web) = offline_web("webmalformed");
    let world = scratch.0.clone();
    crate::saved_json::put(&world.join("mock"), "0000000000a1", "rings");
    let before = file_revision(&world);
    let token = web.token.clone();
    let write = |route: &str, fields: &[(&str, &str)]| PageForm {
        action: format!("/{token}/{route}"),
        button: String::new(),
        fields: fields
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    };
    for (form, kind, said) in [
        (
            write(
                "saved",
                &[
                    ("name", "x"),
                    ("text", "rarity=rare"),
                    ("league", "Standard"),
                ],
            ),
            "usage",
            "`league` is no field of a saved search's write",
        ),
        (
            write(
                "saved",
                &[
                    ("id", "0000000000a1"),
                    ("if_revision", "one"),
                    ("name", "x"),
                ],
            ),
            "usage",
            "is no revision",
        ),
        (
            write(
                "saved",
                &[("id", "0000000000a1"), ("if_revision", ""), ("name", "x")],
            ),
            "usage",
            "is no revision",
        ),
        (
            write(
                "saved",
                &[("if_revision", "1"), ("name", "x"), ("text", "rarity=rare")],
            ),
            "arguments",
            "a revision names the saved search it replaces by its id",
        ),
        (
            write(
                "saved",
                &[("name", ""), ("text", "rarity=rare"), ("realm", "pc")],
            ),
            "arguments",
            "",
        ),
        (
            write(
                "saved",
                &[
                    ("id", "0000000000a1"),
                    ("if_revision", "0"),
                    ("name", "x"),
                    ("text", "rarity=rare"),
                    ("realm", "pc"),
                    ("membership", "live"),
                ],
            ),
            "conflict",
            "stands at revision 1",
        ),
        (
            write(
                "saved",
                &[("id", "0000000000a1"), ("if_revision", "7"), ("name", "x")],
            ),
            "conflict",
            "stands at revision 1",
        ),
        (
            write("saved/clear", &[("id", "0000000000a1")]),
            "usage",
            "a clear names the revision it clears",
        ),
        (
            write(
                "saved/clear",
                &[("id", "0000000000a1"), ("if_revision", "2")],
            ),
            "conflict",
            "stands at revision 1",
        ),
    ] {
        let page = send(&web, &form, false);
        assert_eq!(page.status, 400, "{form:?}: {}", page.body);
        assert!(!main_of(&page.body).contains("acq "), "{}", page.body);
        let (status, refused) = sent_json(&web, &form);
        assert_eq!(status, 400, "{form:?}");
        assert_eq!(refused["kind"], kind, "{form:?}: {refused}");
        assert!(
            refused["error"].as_str().unwrap().contains(said),
            "{form:?}: {refused}"
        );
        assert_eq!(file_revision(&world), before, "{form:?} wrote");
    }
    // the answer's two fields: together, an id and a positive revision
    let answer = "search?realm=pc&text=rarity%3Drare";
    for (query, said) in [
        ("&saved=rings&saved_revision=1", "is no saved search's id"),
        ("&saved=0000000000a1", "the two together"),
        ("&saved_revision=1", "the two together"),
        ("&saved=0000000000a1&saved_revision=0", "is no revision"),
        ("&saved=0000000000a1&saved_revision=-1", "is no revision"),
        ("&saved=0000000000a1&saved_revision=one", "is no revision"),
    ] {
        let page = web.get(&format!("{answer}{query}"));
        assert_eq!(page.status, 400, "{query}: {}", page.body);
        let refused = json_at(&web, &format!("/{token}/{answer}{query}"));
        assert_eq!(refused["kind"], "usage", "{query}: {refused}");
        assert!(
            refused["error"].as_str().unwrap().contains(said),
            "{refused}"
        );
    }
    // a clear asked first: an id's shape, the id alone, a row at it; the
    // id no row has offers the list
    for (query, kind, said) in [
        ("", "usage", "is no saved search's id"),
        ("?id=rings", "usage", "is no saved search's id"),
        (
            "?id=0000000000a1&if_revision=1",
            "usage",
            "`if_revision` is no field",
        ),
        (
            "?id=0000000000ff",
            "saved_search_not_found",
            "no saved search has the id 0000000000ff",
        ),
    ] {
        let path = format!("/{token}/saved/clear{query}");
        let page = follow(&web, &path);
        assert_eq!(page.status, 400, "{query}: {}", page.body);
        let refused = json_at(&web, &path);
        assert_eq!(refused["kind"], kind, "{query}: {refused}");
        assert!(
            refused["error"].as_str().unwrap().contains(said),
            "{refused}"
        );
        if kind == "saved_search_not_found" {
            assert_eq!(
                link_saying(&page.body, "the saved searches, each with its id"),
                format!("/{token}/saved")
            );
        }
    }
    assert_eq!(file_revision(&world), before);
}

/// The routes: `saved/open` writes nothing and answers `303` to
/// the search the row's request folds to, which answers as `acq search
/// --saved` does — for a row over the live items and one over all of
/// them, whose membership the query box and the continuation carry; an
/// id no row holds offers the list; a name is a usage refusal; and a row
/// whose name has another id's shape is never opened by that id.
#[test]
fn saved_open_runs_the_rows_search_and_writes_nothing() {
    let (scratch, web) = offline_web("webopen");
    let world = scratch.0.clone();
    let mock = world.join("mock");
    let token = web.token.clone();
    crate::saved_json::put(&mock, "0000000000a1", "rings");
    saved_cli(
        &world,
        &[
            "set",
            "--name",
            "every ring",
            "--realm",
            "pc",
            "--membership",
            "all",
            "class:rings",
        ],
    );
    let all = saved_json(&world, &["list"])["searches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == "every ring")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();
    for (id, membership) in [("0000000000a1", "live"), (all.as_str(), "all")] {
        let before = file_revision(&world);
        let open = follow(&web, &format!("/{token}/saved/open?id={id}"));
        assert_eq!(open.status, 303, "{}", open.body);
        let location = open.header("location").unwrap().to_string();
        let revision = saved_json(&world, &["show", id])["revision"].clone();
        assert!(
            location.starts_with(&format!("/{token}/search?")),
            "{location}"
        );
        assert!(
            location.ends_with(&format!("&saved={id}&saved_revision={revision}")),
            "{location}"
        );
        assert_eq!(
            location.contains("membership=all"),
            membership == "all",
            "{location}"
        );
        assert_eq!(file_revision(&world), before, "an opening wrote");
        let page = json_at(&web, &location);
        let cli = sole_json(&offline_acq(
            &world,
            &["search", "--saved", id, "--fields", PAGE_FIELDS, "--json"],
        ));
        assert_eq!(page, cli, "{id}");
        let answer = follow(&web, &location);
        assert!(
            answer.body.contains(&format!(
                "<input type=\"hidden\" name=\"membership\" value=\"{membership}\">"
            )),
            "{}",
            answer.body
        );
    }
    // the membership `all` carried through an edit and a page turn
    let location = follow(&web, &format!("/{token}/saved/open?id={all}"))
        .header("location")
        .unwrap()
        .to_string();
    let edited = web.get(&format!(
        "search?realm=pc&membership=all&text=class%3Arings&view.limit=1&saved={all}&saved_revision=1"
    ));
    assert_eq!(edited.status, 200, "{}", edited.body);
    assert_eq!(json_at(&web, &location)["scope"]["membership"], "all");
    let more = link_saying(&edited.body, "more");
    assert!(
        more.contains(&format!("&saved={all}&saved_revision=1")),
        "{more}"
    );
    let next = follow(&web, &more);
    assert_eq!(next.status, 200, "{}", next.body);
    assert_eq!(json_at(&web, &more)["scope"]["membership"], "all");
    assert!(next.body.contains("opened at revision 1"), "{}", next.body);
    // an id no row holds: the list offered, followed as rendered
    let missing = web.get("saved/open?id=0000000000ff");
    assert_eq!(missing.status, 400, "{}", missing.body);
    assert!(!main_of(&missing.body).contains("acq "), "{}", missing.body);
    let list = link_saying(&missing.body, "the saved searches, each with its id");
    assert_eq!(follow(&web, &list).status, 200);
    // a name is no id
    let named = json_at(&web, &format!("/{token}/saved/open?id=rings"));
    assert_eq!(named["kind"], "usage", "{named}");
    // a row at A named as B's id, with no row at B: never opened by B
    crate::saved_json::put(&mock, "0000000000a2", "0000000000b2");
    let by_name = json_at(&web, &format!("/{token}/saved/open?id=0000000000b2"));
    assert_eq!(by_name["kind"], "saved_search_not_found", "{by_name}");
    let answer = web.get("search?realm=pc&text=rarity%3Drare&saved=0000000000b2&saved_revision=1");
    assert_eq!(answer.status, 200, "{}", answer.body);
    assert!(
        answer.body.contains("Saved search 0000000000b2 is gone"),
        "{}",
        answer.body
    );
    assert!(
        answer.body.contains(">Save this search</button>"),
        "{}",
        answer.body
    );
}

/// The handoff between the terminal and the page: a row the terminal
/// saves is on the list page, and a row the page saves is in `acq saved
/// list`, its channel `web`.
#[test]
fn the_handoff_a_row_saved_elsewhere_is_on_the_page_and_the_pages_is_listed() {
    let (scratch, web) = offline_web("webhandoff");
    let world = scratch.0.clone();
    saved_cli(
        &world,
        &[
            "set",
            "--name",
            "from the terminal",
            "--realm",
            "pc",
            "rarity=rare",
        ],
    );
    let list = web.get("saved");
    assert!(list.body.contains("from the terminal"), "{}", list.body);
    assert!(list.body.contains("via cli"), "{}", list.body);
    let answer = web.get(&format!("search?{}", query_string(&ring_search())));
    let save = form_on(&answer.body, "Save this search", None).typed("name", "from the page");
    landed_on_the_list(&web, &send(&web, &save, false));
    let listed = saved_json(&world, &["list"]);
    let row = listed["searches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == "from the page")
        .unwrap_or_else(|| panic!("{listed}"));
    assert_eq!(row["written_via"], "web", "{row}");
}

/// C53's bound on the list page: eleven rows are ten, with the link that
/// lists every one, which lists eleven; the page's JSON is `acq saved
/// list`'s, and under `expand=true` `acq saved list --expand`'s.
#[test]
fn c53_the_saved_list_shows_ten_and_links_every_one() {
    let (scratch, web) = offline_web("webbound");
    let world = scratch.0.clone();
    for n in 0..11 {
        crate::saved_json::put(
            &world.join("mock"),
            &format!("00000000000{n:x}"),
            &format!("a{n:02}"),
        );
    }
    let list = web.get("saved");
    assert_eq!(
        list.body.matches(">Rename</button>").count(),
        10,
        "{}",
        list.body
    );
    assert!(list.body.contains("10 of 11 listed"), "{}", list.body);
    let every = follow(&web, &link_saying(&list.body, "list every one"));
    assert_eq!(
        every.body.matches(">Rename</button>").count(),
        11,
        "{}",
        every.body
    );
    let token = web.token.clone();
    assert_eq!(
        json_at(&web, &format!("/{token}/saved")),
        saved_json(&world, &["list"])
    );
    assert_eq!(
        json_at(&web, &format!("/{token}/saved?expand=true")),
        saved_json(&world, &["list", "--expand"])
    );
    let other = json_at(&web, &format!("/{token}/saved?expand=yes"));
    assert_eq!(other["kind"], "usage", "{other}");
}

/// The saved pages, a golden naming what a reader must find: the
/// list empty and with rows, one this build cannot read among them; a
/// create from an answer, a rename and a clear, each a redirect to the
/// list it lands on; an answer opened from a row, then after the row
/// moved, its text changed, it was renamed and it was cleared; a stale
/// save's conflict showing the row as it stands; the unreadable row
/// cleared; a save over a row replaced by one this build cannot read and
/// its offer posted; a row over every item opened; and a row replaced so:
/// its link followed, the list offered, its answer reloaded.
#[test]
fn the_saved_pages_are_a_wording_golden() {
    use acquisition_store::{Provenance, SAVED_SEARCH_SCOPE, User};
    let (scratch, web) = offline_web("websaved");
    let world = scratch.0.clone();
    let mock = world.join("mock");
    let newer = |id: &str, over: Option<i64>| {
        User::open_for(&mock)
            .unwrap()
            .put::<crate::saved_json::NewerSaved>(
                SAVED_SEARCH_SCOPE,
                id,
                &json!({ "version": 3, "name": "x", "query": { "text": "x" }, "scope": {} }),
                over,
                &Provenance::via("future"),
            )
            .unwrap();
    };
    let mut pages = Pages::new(&world, &web);
    let empty = web.get("saved");
    pages.page(
        "GET saved <none yet>",
        &empty,
        &["No saved searches: save one from any search&#39;s answer, under its query."],
    );
    for (id, name) in [
        ("0000000000a1", "amulets"),
        ("0000000000a2", "rings"),
        ("0000000000a3", "boots"),
        ("0000000000a4", "belts"),
    ] {
        crate::saved_json::put(&mock, id, name);
    }
    newer("0000000000f1", None);
    let listed = web.get("saved");
    pages.page(
        "GET saved <four rows and one this build cannot read>",
        &listed,
        &[
            "5 saved searches · the file at revision 5",
            "<th>Name</th><th>Scope</th><th>Query</th><th>Revision</th><th>Last written</th><th>Id</th>",
            // an opening and a clear's ask are steps: each hands on the
            // list as `from` (the landing rule)
            "saved/open?id=0000000000a2&amp;from=saved\">rings</a>",
            "<td>pc · live</td><td>rarity=rare</td><td>1</td>",
            "via test",
            "this build cannot read it:",
            ">Rename</button>",
            "saved/clear?id=0000000000a2&amp;from=saved\">Clear</a>",
        ],
    );
    assert!(!main_of(&listed.body).contains("acq "), "{}", listed.body);
    // the list holds no clear's form: a clear asks first
    assert!(
        !listed.body.contains(">Clear</button>"),
        "the list clears without asking: {}",
        listed.body
    );
    // a create from an answer, and the list it lands on
    let ring_path = format!("search?{}", query_string(&ring_search()));
    let answer = web.get(&ring_path);
    pages.page(
        "GET search?<the ring search>",
        &answer,
        &[
            // the save form posts the search page itself as `from` first
            "/saved\"><input type=\"hidden\" name=\"from\" value=\"search?realm=pc&amp;",
            "\"><input type=\"hidden\" name=\"trade\" value=\"{&quot;league&quot;:&quot;Standard&quot;",
            "name=\"membership\" value=\"live\"",
            ">Save this search</button>",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &ring_path, &answer);
    let created = send(
        &web,
        &form_on(&answer.body, "Save this search", None).typed("name", "my rings"),
        false,
    );
    pages.redirect("POST saved <the answer saved as my rings>", &created);
    let list = follow(&web, created.header("location").unwrap());
    let mine = saved_json(&world, &["list"])["searches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == "my rings")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();
    pages.placeholder(&mine, "created");
    pages.page(
        "GET saved <after the create>",
        &list,
        &["my rings", "via web", "6 saved searches"],
    );
    let renamed = send(
        &web,
        &form_on(&list.body, "Rename", Some(&mine)).typed("name", "kept rings"),
        false,
    );
    pages.redirect("POST saved <my rings renamed kept rings>", &renamed);
    let list = follow(&web, renamed.header("location").unwrap());
    pages.page("GET saved <after the rename>", &list, &["kept rings"]);
    let (asking, clear) = asked_clear(&web, &world, &list.body, "0000000000a3");
    pages.page(
        "GET saved/clear?id=0000000000a3 <boots, asked first>",
        &asking,
        &[
            "Clear the saved search &quot;boots&quot;? It is this store&#39;s saved search, \
             whoever saved it",
            "<td>boots</td><td>pc · live</td><td>rarity=rare</td><td>1</td>",
            "name=\"if_revision\" value=\"1\"",
            ">Clear</button>",
            "/saved\">Keep it</a>",
        ],
    );
    assert!(!main_of(&asking.body).contains("acq "), "{}", asking.body);
    assert_eq!(
        basis_line(&asking.body),
        Some(json!({ "saved_search": 1 })),
        "{}",
        asking.body
    );
    // a reload of the asking page writes nothing either
    let before = file_revision(&world);
    assert_eq!(
        follow(&web, &format!("/{}/saved/clear?id=0000000000a3", web.token)).status,
        200
    );
    assert_eq!(
        file_revision(&world),
        before,
        "a reload of the asking page wrote"
    );
    let cleared = send(&web, &clear, false);
    pages.redirect("POST saved/clear <boots, from the asking page>", &cleared);
    let list = follow(&web, cleared.header("location").unwrap());
    assert!(!list.body.contains("boots"), "{}", list.body);
    pages.page("GET saved <after the clear>", &list, &["5 saved searches"]);
    // an answer opened from a row, then as the row moves under it
    let open = follow(&web, &link_saying(&list.body, "rings"));
    pages.redirect("GET saved/open?id=0000000000a2", &open);
    let location = open.header("location").unwrap().to_string();
    let opened = follow(&web, &location);
    pages.page(
        "GET search?<rings, opened>",
        &opened,
        &[
            "Saved search &quot;rings&quot;, opened at revision 1",
            ">the saved searches</a>",
            "name=\"saved\" value=\"0000000000a2\"",
            "name=\"saved_revision\" value=\"1\"",
            "name=\"if_revision\" value=\"1\"",
            ">Save over &quot;rings&quot;</button>",
            ">Save as a new search</button>",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &location, &opened);
    assert!(
        !saved_parts(&opened.body).contains("acq "),
        "{}",
        opened.body
    );
    saved_cli(&world, &["set", "--id", "0000000000a2", "--realm", "pc"]);
    let moved = follow(&web, &location);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &location, &moved);
    pages.page(
        "GET search?<rings, opened, after the row moved>",
        &moved,
        &[
            "Saved search 0000000000a2, opened at revision 1, has been written since: it is now \
             &quot;rings&quot; at revision 2, last written",
            "via cli. Saving over it here is refused as stale and offers the write over revision 2.",
        ],
    );
    saved_cli(&world, &["set", "--id", "0000000000a2", "rarity=unique"]);
    let moved = follow(&web, &location);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &location, &moved);
    pages.page(
        "GET search?<rings, opened, after its text changed>",
        &moved,
        &["This answer&#39;s query is not the saved one&#39;s: the saved query is rarity=unique over pc · live."],
    );
    saved_cli(
        &world,
        &["set", "--id", "0000000000a2", "--name", "unique rings"],
    );
    let reloaded = follow(&web, &location);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &location, &reloaded);
    pages.page(
        "GET search?<rings, opened, after it was renamed>",
        &reloaded,
        &["it is now &quot;unique rings&quot; at revision 4"],
    );
    // the page's save over the row, stale: the row as it stands, then the
    // write over it as a button
    let stale = send(
        &web,
        &form_on(&opened.body, "Save over \"rings\"", None),
        false,
    );
    assert_eq!(stale.status, 400, "{}", stale.body);
    pages.page(
        "POST saved <a save over rings at revision 1, the row at 4>",
        &stale,
        &[
            "stands at revision 4, not the revision the write named: nothing was written",
            "It stands now as:",
            "<td>unique rings</td><td>pc · live</td><td>rarity=unique</td><td>4</td>",
            ">the same write over revision 4, the row as it stands</button>",
        ],
    );
    saved_cli(&world, &["clear", "0000000000a2"]);
    let moved = follow(&web, &location);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &location, &moved);
    pages.page(
        "GET search?<rings, opened, after it was cleared>",
        &moved,
        &[
            "Saved search 0000000000a2 is gone: no saved search has that id now. Saving here makes a new one.",
            ">Save this search</button>",
        ],
    );
    // the row this build cannot read, cleared from the list
    let list = web.get("saved");
    let (asking, clear) = asked_clear(&web, &world, &list.body, "0000000000f1");
    pages.page(
        "GET saved/clear?id=0000000000f1 <the row this build cannot read, asked first>",
        &asking,
        &[
            "Clear the saved search 0000000000f1?",
            "this build cannot read it:",
            ">Clear</button>",
        ],
    );
    let cleared = send(&web, &clear, false);
    pages.redirect(
        "POST saved/clear <the row this build cannot read>",
        &cleared,
    );
    let list = follow(&web, cleared.header("location").unwrap());
    assert!(!list.body.contains("cannot read"), "{}", list.body);
    pages.page("GET saved <after it>", &list, &["3 saved searches"]);
    // a save over a row since replaced by one this build cannot read
    let (amulets_at, amulets) = redirected(&web, &link_saying(&list.body, "amulets"));
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &amulets_at, &amulets);
    newer("0000000000a1", Some(1));
    let over = send(
        &web,
        &form_on(&amulets.body, "Save over \"amulets\"", None),
        false,
    );
    pages.page(
        "POST saved <a save over amulets, replaced by a row this build cannot read>",
        &over,
        &[
            "saved search 0000000000a1 stands at revision 2",
            ">the same write over revision 2, the row as it stands</button>",
        ],
    );
    let offered = send(
        &web,
        &form_on(
            &over.body,
            "the same write over revision 2, the row as it stands",
            None,
        ),
        false,
    );
    pages.redirect("POST saved <its offer>", &offered);
    pages.page(
        "GET saved <after it>",
        &follow(&web, offered.header("location").unwrap()),
        &["\">amulets</a></td><td>pc · live</td><td>rarity=rare</td><td>3</td>"],
    );
    // a row over every item, saved from its answer and opened
    let every = web.get("search?realm=pc&text=");
    let saved = send(
        &web,
        &form_on(&every.body, "Save this search", None).typed("name", "everything"),
        false,
    );
    let list = follow(&web, saved.header("location").unwrap());
    let everything = saved_json(&world, &["list"])["searches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == "everything")
        .unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string();
    pages.placeholder(&everything, "everything");
    let open = follow(&web, &link_saying(&list.body, "everything"));
    pages.redirect("GET saved/open?id=<everything>", &open);
    let everything_at = open.header("location").unwrap().to_string();
    let everything_page = follow(&web, &everything_at);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &everything_at, &everything_page);
    pages.page(
        "GET search?<everything, opened>",
        &everything_page,
        &[
            "Saved search &quot;everything&quot;, opened at revision 1",
            "<textarea id=\"query-text\" aria-label=\"the query, in the language\" name=\"text\" rows=\"3\"></textarea>",
        ],
    );
    // a row replaced by one this build cannot read: its link as the list
    // rendered it, followed; the list it offers; its answer reloaded
    let belts = link_saying(&list.body, "belts");
    let (belted, _) = redirected(&web, &belts);
    newer("0000000000a4", Some(1));
    let refused = follow(&web, &belts);
    pages.page(
        "GET saved/open?id=0000000000a4 <replaced by a row this build cannot read>",
        &refused,
        &[
            "saved search 0000000000a4 at revision 2 holds a value this build cannot read",
            ">the saved searches, each with its id</a>",
        ],
    );
    assert!(!main_of(&refused.body).contains("acq "), "{}", refused.body);
    // refused from the row the opening read: its id, its revision, and
    // the list its one way on
    let doc = json_at(&web, &belts);
    assert_eq!(doc["kind"], "refused", "{doc}");
    let words = doc["error"].as_str().unwrap();
    assert!(
        words.contains("0000000000a4") && words.contains("at revision 2"),
        "{doc}"
    );
    let [offer] = doc["offers"].as_array().unwrap().as_slice() else {
        panic!("{doc}")
    };
    assert_eq!(offer["request"], format!("/{}/saved", web.token), "{doc}");
    pages.page(
        "GET saved <its offer followed>",
        &follow(
            &web,
            &link_saying(&refused.body, "the saved searches, each with its id"),
        ),
        &["this build cannot read it:"],
    );
    let belts_page = follow(&web, &belted);
    every_request_an_answer_carries_is_a_way_on_its_page(&web, &belted, &belts_page);
    pages.page(
        "GET search?<belts, opened, reloaded>",
        &belts_page,
        &[
            "Saved search 0000000000a4 holds a value this build cannot read; clear it from",
            ">Save this search</button>",
        ],
    );
    // C194: a stored source this build cannot read lists its fault where
    // its query stands, and its opening is refused; a version 1 row, the
    // text alone, lists, opens and answers, and stays as it was written
    User::open_for(&mock)
        .unwrap()
        .put::<acquisition_store::SavedSearch>(
            SAVED_SEARCH_SCOPE,
            "0000000000c1",
            &json!({ "version": 2, "name": "lost", "query": { "trade": "not a site search" },
                     "scope": { "realm": "pc", "membership": "live" } }),
            None,
            &Provenance::via("test"),
        )
        .unwrap();
    crate::saved_json::put_v1(
        &mock,
        "0000000000c2",
        "old rings",
        "class:rings   rarity=rare",
    );
    let stored = || {
        User::open_for(&mock)
            .unwrap()
            .get(SAVED_SEARCH_SCOPE, "0000000000c2", "saved-search")
            .unwrap()
            .unwrap()
            .value
    };
    let v1 = stored();
    let listed = web.get("saved?expand=true");
    pages.page(
        "GET saved?expand=true <a source this build cannot read, and a version 1 row>",
        &listed,
        &[
            "saved/open?id=0000000000c1&amp;from=saved%3Fexpand%3Dtrue\">lost</a></td><td>pc · live</td><td>this build cannot read its query:",
            "saved/open?id=0000000000c2&amp;from=saved%3Fexpand%3Dtrue\">old rings</a></td><td>pc · live</td><td>class:rings rarity=rare</td>",
        ],
    );
    let lost = follow(&web, &format!("/{}/saved/open?id=0000000000c1", web.token));
    pages.page(
        "GET saved/open?id=0000000000c1 <a source this build cannot read>",
        &lost,
        &[
            "saved search 0000000000c1 at revision 1 asks what this build cannot read",
            "a page cannot open it",
        ],
    );
    let (old, opened) = redirected(&web, &format!("/{}/saved/open?id=0000000000c2", web.token));
    assert_eq!(opened.status, 200, "{}", opened.body);
    assert!(old.contains("text=class%3Arings"), "{old}");
    assert_eq!(stored(), v1, "a read rewrote the version 1 row");
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "saved_pages", &pages);
}

/// The grouped table: the seeded world with a folder holding a tab of
/// rings, a map tab with a ring of its own beside a page's, and a tab
/// never fetched, and a character carrying a ring. Under a query, the
/// stash tabs page lists the places that hold a match, each head its
/// matches beside its items, a parent's its places' sum, a map tab's own
/// said apart, the rest folded into one counted line; each open place's
/// items one segment of the page's window under the item columns' heads,
/// a value's head ordering them. Golden: that page, the page under
/// `fetched=never`, and the characters page under the query. Then the
/// URL each way names: an answer's "by stash tab" and "by character", the
/// league its query fixes or the default's, none over all realms or
/// removed items; the fold's; a value's head; a toggle; and a place
/// opened with no query and a sort named orders its rows by it.
#[test]
fn the_grouped_table_is_a_wording_golden() {
    let (scratch, web) = offline_web("webgrouped");
    let mut pages = Pages::new(&scratch.0, &web);
    let at = |body: &str, words: &str| {
        body.find(words)
            .unwrap_or_else(|| panic!("{words}: {body}"))
    };
    let mut store = Store::open(&account_path(&scratch.0.join("mock"), USER)).unwrap();
    let mut record = |ep: Endpoint, body: Value, at: i64| {
        store
            .record(
                &ep,
                &json!({ "realm": "pc", "league": "Standard" }),
                200,
                &body,
                at,
            )
            .unwrap();
    };
    let ring = |id: &str, name: &str, ilvl: i64| {
        json!({ "id": id, "name": name, "typeLine": "Iron Ring", "baseType": "Iron Ring",
                "rarity": "Rare", "frameTypeId": 2, "identified": true, "ilvl": ilvl, "x": 0, "y": 0 })
    };
    let stash = |id: &str, sub: Option<&str>| Endpoint::Stash {
        realm: "pc".into(),
        league: "Standard".into(),
        id: id.into(),
        sub: sub.map(str::to_string),
    };
    record(
        Endpoint::Stashes {
            realm: "pc".into(),
            league: "Standard".into(),
        },
        json!({ "stashes": [
            { "id": "t1", "name": "Rings", "type": "PremiumStash" },
            { "id": "t2", "name": "Launch", "type": "PremiumStash" },
            { "id": "f1", "name": "Old", "type": "Folder", "children": [
                { "id": "o1", "name": "Old rings", "type": "PremiumStash" } ] },
            { "id": "m1", "name": "Maps", "type": "MapStash" },
            { "id": "t3", "name": "Never", "type": "PremiumStash" } ] }),
        60,
    );
    record(
        stash("o1", None),
        json!({ "stash": { "id": "o1", "name": "Old rings", "type": "PremiumStash", "items": [
            ring("o-low", "Cold Band", 62), ring("o-high", "Grim Knot", 81) ] } }),
        61,
    );
    record(
        stash("m1", None),
        json!({ "stash": { "id": "m1", "name": "Maps", "type": "MapStash", "items": [
            ring("m-own", "Atlas Loop", 75) ], "children": [
            { "id": "p1", "name": "Tier 16", "type": "MapStash", "parent": "m1", "metadata": { "items": 1 } },
            { "id": "p2", "name": "Tier 15", "type": "MapStash", "parent": "m1", "metadata": { "items": 1 } } ] } }),
        62,
    );
    record(
        stash("m1", Some("p1")),
        json!({ "stash": { "id": "p1", "name": "Tier 16", "type": "MapStash", "parent": "m1", "items": [
            ring("m-page", "Tier Loop", 70) ] } }),
        63,
    );
    record(
        Endpoint::Characters { realm: "pc".into() },
        json!({ "characters": [
            { "id": "ch1", "name": "Exile", "league": "Standard", "level": 90 },
            { "id": "ch2", "name": "Idle", "league": "Standard", "level": 12 } ] }),
        64,
    );
    record(
        Endpoint::Character {
            realm: "pc".into(),
            name: "Exile".into(),
        },
        json!({ "character": { "id": "ch1", "name": "Exile", "league": "Standard", "level": 90,
                               "equipment": [ ring("c-ring", "Worn Loop", 66) ], "inventory": [] } }),
        65,
    );
    drop(store);

    let query = "text=class%3Arings%20ilvl%3E%3D60";
    let grouped = format!(
        "tabs?realm=pc&league=Standard&{query}&view.sort=ilvl&view.desc=true&open=t1&open=f1&open=o1&open=m1&tab=t3"
    );
    let page = web.get(&grouped);
    pages.page(
        "GET tabs?<rings, four places open>",
        &page,
        &[
            // the matches here beside the query's own, and the way to them
            "7 matching items in these stash tabs, of 8 matches",
            ">every match as a table</a>",
            // a head whose matches are fewer than its items
            "<span>PremiumStash</span><span>2 of 3</span>",
            // a parent's, its places' sum; a map tab's own said apart
            "<span>Folder</span><span>2 matching</span>",
            "<span>MapStash</span><span>2 matching</span>",
            "1 matching item in the tab itself",
            // the fold: the tab never fetched, its tick travelling hidden
            "1 tab not shown, <a href=",
            ">1 never fetched</a>; 1 ticked</p>",
            "<input type=\"hidden\" name=\"tab\" value=\"t3\">",
            "1 page not shown, <a href=",
            // an open place's segment under the columns' heads, once
            "<div class=\"r head\"><span>Item</span>",
            "<div class=\"seg\" data-route=\"",
            "data-count=\"2\" data-base=\"0\"",
            "ilvl<span class=\"sr\">, ordered largest first</span></a>",
        ],
    );
    assert_eq!(
        page.body.matches("<div class=\"r head\">").count(),
        1,
        "{}",
        page.body
    );
    let never = web.get("tabs?realm=pc&league=Standard&fetched=never");
    pages.page(
        "GET tabs?realm=pc&league=Standard&fetched=never",
        &never,
        &["id=\"p-t3\"", "id=\"p-m1\"", "3 tabs not shown</p>"],
    );
    let characters = web.get(&format!(
        "characters?realm=pc&league=Standard&{query}&open=ch1"
    ));
    pages.page(
        "GET characters?<rings, Exile open>",
        &characters,
        &[
            "1 matching item on these characters, of 8 matches",
            // where the rest stand
            "elsewhere: <a href=\"/",
            ">7 in the stash tabs of Standard</a></p>",
            "<span>90</span><span>1 of 1</span>",
            "1 character not shown, <a href=",
            "data-id=\"c-ring\"",
        ],
    );
    // an open place's opened row under the query: the place's
    // order cut after it, its card in the flow, its place a link
    let opened = web.get(&format!(
        "tabs?realm=pc&league=Standard&{query}&open=f1&open=o1&item=o-high"
    ));
    pages.page(
        "GET tabs?<rings, Old rings open, its first row opened>",
        &opened,
        &[
            "data-count=\"1\" data-base=\"0\"",
            "data-count=\"1\" data-base=\"1\"",
            "<div class=\"opened\">",
            "name=\"toggle_item\" value=\"o-high\">Close</button>",
            "<input type=\"hidden\" name=\"item\" value=\"o-high\">",
            "open=f1&amp;open=o1&amp;at=o-high#i-o-high\">",
        ],
    );
    // a place open with no query: its items as they lie, and the
    // way to order them said
    let plain = web.get("tabs?realm=pc&league=Standard&open=t1");
    pages.page(
        "GET tabs?<a place open, no query>",
        &plain,
        &[
            "<p class=\"rows-shown\">Each place&#39;s items stand as they lie in their place, top row first and left to right: open one by its name and press a number on its card to order them by it.</p>",
        ],
    );
    page_golden(file!(), "grouped", &pages);

    // a card's place, followed: its page with the place and each holding
    // it open, no query, its items listed, landing at the card's own row,
    // so a long tab is not searched again for it — a tab in a folder, a
    // map tab's page, a character
    for (item, open, route) in [
        ("o-low", "open=f1&open=o1", "tabs"),
        ("m-page", "open=m1&open=p1", "tabs"),
        ("c-ring", "open=ch1", "characters"),
    ] {
        let shown = web.get(&format!("item/{item}"));
        let card = &shown.body[at(&shown.body, "<p class=\"place\">")..];
        let href = attribute(card, "href").unwrap();
        assert_eq!(
            href,
            format!(
                "/{}/{route}?realm=pc&league=Standard&{open}&at={item}#i-{item}",
                web.token
            ),
            "{item}"
        );
        let place = follow(&web, &href);
        assert_eq!(place.status, 200, "{item}: {}", place.body);
        assert!(
            place.body.contains(&format!("id=\"i-{item}\" data-i=")),
            "{item}: {}",
            place.body
        );
    }

    // the document's counts are the page's
    let doc = web.get_json(&grouped);
    assert_eq!(doc["query"]["matched"], 7, "{doc}");
    assert_eq!(doc["query"]["total"]["matched"], 8, "{doc}");
    assert_eq!(doc["folded"], json!({ "places": 1, "never_fetched": 1 }));

    // a value's head: the same page ordered by it the other way from the
    // order named (largest first), every open and tick kept, the
    // places in stash order and the rows inside each ordered
    let head = &page.body[at(&page.body, "<div class=\"r head\">")..];
    let href = attribute(&head[head.find("<a class=\"head\"").unwrap()..], "href").unwrap();
    assert_eq!(
        href,
        format!(
            "/{}/tabs?realm=pc&league=Standard&text=class%3Arings%20ilvl%3E%3D60&view.sort=ilvl&open=t1&open=f1&open=o1&open=m1&tab=t3",
            web.token
        )
    );
    let ordered = follow(&web, &href);
    let ids = |body: &str| -> Vec<String> {
        body.split("data-id=\"")
            .skip(1)
            .map(|s| s[..s.find('"').unwrap()].to_string())
            .collect()
    };
    let shown = ids(&ordered.body);
    assert!(
        shown.iter().position(|i| i == "o-low") < shown.iter().position(|i| i == "o-high"),
        "{shown:?}"
    );
    assert!(at(&ordered.body, "id=\"p-t1\"") < at(&ordered.body, "id=\"p-f1\""));
    // a toggle keeps the query, the view and every tick
    let toggled = follow(
        &web,
        &place_form(&web, &ordered.body, &[], "Close", Some(("toggle", "t1"))),
    );
    assert_eq!(
        toggled.header("location"),
        Some(
            format!(
                "/{}/tabs?realm=pc&league=Standard&text=class%3Arings%20ilvl%3E%3D60&view.sort=ilvl&open=f1&open=o1&open=m1&tab=t3#p-t1",
                web.token
            )
            .as_str()
        )
    );
    // a toggle naming no place of the league (one a refresh took away,
    // sent from a page loaded before it) is dropped, and the open places
    // still list their items
    for (path, row) in [
        (
            "tabs?realm=pc&league=Standard&open=t1&toggle=gone",
            "data-id=\"r1\"",
        ),
        (
            "characters?realm=pc&league=Standard&open=ch1&toggle=gone",
            "data-id=\"c-ring\"",
        ),
    ] {
        let stale = web.get(path);
        assert_eq!(stale.status, 200, "{path}: {}", stale.body);
        assert!(stale.body.contains(row), "{path}: {}", stale.body);
    }
    // the fold's never-fetched count: the page under `fetched=never`
    // alone, every open and tick kept
    let fold = &page.body[at(&page.body, "1 tab not shown")..];
    let href = attribute(&fold[fold.find("<a ").unwrap()..], "href").unwrap();
    assert_eq!(
        href,
        format!(
            "/{}/tabs?realm=pc&league=Standard&fetched=never&open=t1&open=f1&open=o1&open=m1&tab=t3",
            web.token
        )
    );
    // a place opened with no query and a sort named orders its rows by
    // it, either way
    for (desc, first) in [("&view.desc=true", "o-high"), ("", "o-low")] {
        let sorted = web.get(&format!(
            "tabs?realm=pc&league=Standard&open=f1&open=o1&view.sort=ilvl{desc}"
        ));
        assert_eq!(
            ids(&sorted.body).first().map(String::as_str),
            Some(first),
            "{}",
            sorted.body
        );
    }

    // an answer's ways to the place pages: the league its query fixes, or
    // the default's, the canonical text and the view
    let by = |path: &str| -> Vec<String> {
        let body = web.get(path).body;
        ["by stash tab", "by character"]
            .iter()
            .filter_map(|words| {
                let end = body.find(&format!("\">{words}</a>"))?;
                let start = body[..end].rfind("href=\"")? + "href=\"".len();
                Some(unescaped(&body[start..end]))
            })
            .collect()
    };
    assert_eq!(
        by("search?realm=pc&text=class%3Arings&view.fields=icon&view.sort=ilvl"),
        [
            format!(
                "/{}/tabs?realm=pc&league=Standard&text=class%3Arings&view.sort=ilvl",
                web.token
            ),
            format!(
                "/{}/characters?realm=pc&league=Standard&text=class%3Arings&view.sort=ilvl",
                web.token
            ),
        ]
    );
    let fixed = by("search?realm=pc&text=class%3Arings%20league%3DHardcore");
    assert!(
        fixed[0].contains("tabs?realm=pc&league=Hardcore&text=class%3Arings%20league%3DHardcore"),
        "{fixed:?}"
    );
    // a league the query names in another case is the store's spelling
    let cased = by("search?realm=pc&text=class%3Arings%20league%3Dstandard");
    assert!(
        cased[0].contains("tabs?realm=pc&league=Standard&text="),
        "{cased:?}"
    );
    assert!(
        cased[1].contains("characters?realm=pc&league=Standard&text="),
        "{cased:?}"
    );
    let landed = follow(&web, &cased[0]);
    assert!(landed.body.contains("id=\"p-t1\""), "{}", landed.body);
    let landed = follow(&web, &by("search?realm=pc&text=class%3Arings")[0]);
    assert!(
        landed.body.contains("matching items in these stash tabs"),
        "{}",
        landed.body
    );
    // none over all realms, none over removed items
    for path in [
        "search?realm=all&text=class%3Arings",
        "search?realm=pc&membership=all&text=class%3Arings",
    ] {
        assert_eq!(by(path), Vec::<String>::new(), "{path}");
    }
}

// ---- the landing rule (`roadmap/briefs/4-full-1-form-landing.md`) ------------------------------

/// Where a leaving request lands: on its page as sent (a write from the
/// saved list itself), or on another page carrying `from`.
enum Lands {
    AsSent,
    Jobs,
    Saved,
}

/// The landing rule: each request that leaves a page carries that page as
/// `from`, written by the page, and lands on the page as sent or on one
/// that draws the way back to it — the league list's fetch from every page
/// that holds it among them, which once landed with no way back (#194).
/// One table: the page, the button pressed, where it lands. Apply, after a
/// plan asked from a ticked place page, is the refresh loop's
/// (`the_refresh_loop_runs_through_the_pages`), whose world has tabs to
/// tick.
#[test]
fn every_leaving_request_lands_with_the_way_back_to_its_page() {
    let scratch = harness::scratch("weblanding");
    let base = scratch.0.clone();
    let daemon = harness::login(&base, "Alice");
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let search = "search?realm=pc&text=rarity%3Drare";
    // a saved search to rename, made by the page's own save form
    let saved = send(
        &web,
        &form_on(&web.get(search).body, "Save this search", None).typed("name", "rares"),
        false,
    );
    assert_eq!(saved.status, 303, "{}", saved.body);
    let id = sole_json(&harness::acq(&base, &["saved", "list", "--json"]))["searches"][0]["id"]
        .as_str()
        .unwrap()
        .to_string();
    let table: [(&str, &str, Option<&str>, Lands); 9] = [
        (search, "Save this search", None, Lands::Saved),
        (search, "Fetch the league list", None, Lands::Jobs),
        ("", "Fetch the league list", None, Lands::Jobs),
        (
            "tabs?realm=pc&league=Standard",
            "Fetch the league list",
            None,
            Lands::Jobs,
        ),
        (
            "tabs?realm=pc&league=Standard",
            "List this league's stash tabs",
            None,
            Lands::Jobs,
        ),
        (
            "characters?realm=pc&league=Standard",
            "Fetch the league list",
            None,
            Lands::Jobs,
        ),
        (
            "characters?realm=pc&league=Standard",
            "List characters",
            None,
            Lands::Jobs,
        ),
        ("saved", "Rename", Some(&id), Lands::AsSent),
        ("saved?expand=true", "Rename", Some(&id), Lands::AsSent),
    ];
    for (n, (page, button, id, lands)) in table.into_iter().enumerate() {
        let shown = web.get(page);
        assert_eq!(shown.status, 200, "{page}: {}", shown.body);
        let mut form = form_on(&shown.body, button, id);
        if form.field("name").is_some() {
            form = form.typed("name", &format!("rares {n}"));
        }
        // the page writes itself, as its own URL names it; the home page
        // is `.`
        let from = match page {
            "" => ".",
            page => page,
        };
        assert_eq!(form.field("from"), Some(from), "{page} {button}");
        let reply = send(&web, &form, false);
        let location = reply.header("location").unwrap_or_default().to_string();
        match lands {
            Lands::Jobs => {
                landed_at_a_job(&web, &reply, Some(from));
            }
            Lands::Saved => {
                assert_eq!(
                    location,
                    format!("/{}/saved?from={}", web.token, url_encoded(from)),
                    "{page} {button}"
                );
                let landed = follow(&web, &location);
                let back = back_link(&web, from);
                assert!(landed.body.contains(&back), "{back}: {}", landed.body);
            }
            Lands::AsSent => {
                assert_eq!(location, format!("/{}/{page}", web.token), "{button}");
                let landed = follow(&web, &location);
                assert!(
                    !landed.body.contains("<p class=\"back\">"),
                    "{}",
                    landed.body
                );
            }
        }
        until_done(&web);
    }
    // the clear's ask is a step: asked from the list whole, it hands the
    // list on, and its Clear lands there as sent
    let list = web.get("saved?expand=true");
    let ask = link_saying(&list.body, "Clear");
    assert!(ask.ends_with("&from=saved%3Fexpand%3Dtrue"), "{ask}");
    let asking = follow(&web, &ask);
    assert!(
        asking.body.contains(&format!(
            "<p class=\"back\"><a href=\"/{}/saved?expand=true\">Back to the saved searches</a></p>",
            web.token
        )),
        "{}",
        asking.body
    );
    let cleared = send(&web, &form_on(&asking.body, "Clear", None), false);
    assert_eq!(
        cleared.header("location"),
        Some(format!("/{}/saved?expand=true", web.token).as_str())
    );
    // an opening is a step too: the search it lands on draws the list
    saved_list_opens_with_the_way_back(&web, &base);
    drop(web);
    drop(daemon);
}

/// A saved search opened from the list lands on its search page, which
/// draws the way back to the list it was opened from and posts itself, not
/// that way back, from its own forms.
fn saved_list_opens_with_the_way_back(web: &WebUi, base: &Path) {
    let made = harness::acq(base, &["saved", "set", "--name", "opened", "rarity=rare"]);
    assert!(made.status.success(), "{made:?}");
    let list = web.get("saved");
    let open = link_saying(&list.body, "opened");
    assert!(open.ends_with("&from=saved"), "{open}");
    let (location, page) = redirected(web, &open);
    assert!(location.ends_with("&from=saved"), "{location}");
    assert!(
        page.body.contains(&format!(
            "<p class=\"back\"><a href=\"/{}/saved\">Back to the saved searches</a></p>",
            web.token
        )),
        "{}",
        page.body
    );
    let save = form_on(&page.body, "Save as a new search", None);
    let from = save.field("from").expect("the page's `from`");
    assert!(
        from.starts_with("search?") && !from.contains("from"),
        "{from}"
    );
}

/// The way back a page draws to `from` as a page wrote it, up to its
/// words: the home page's `.` is the token's own path.
fn back_link(web: &WebUi, from: &str) -> String {
    let path = from.strip_prefix('.').unwrap_or(from);
    format!(
        "<p class=\"back\"><a href=\"/{}/{}\">",
        web.token,
        path.replace('&', "&amp;")
    )
}

/// What a refusal's page offers as its way on, besides the way back.
#[derive(Debug, Clone, Copy)]
enum WayOn {
    /// The form as sent, its name box as typed.
    FormAsSent,
    /// The page as sent, which the way back is: a refused plan's ticks.
    PageAsSent,
    /// An offer a page takes: a link or a form under the refusal.
    Offer,
    /// Words naming a command, a person's act at a terminal.
    Command,
}

/// The census of the refusals the leaving requests can meet
/// (`4-full-1-form-design.md` §5): each, staged as a page meets it, is
/// refused on a page holding the way back to the page it left and its way
/// on — its form as sent, an offer a page takes, or words naming the
/// command a person runs at a terminal. Its JSON is the refusal alone: no
/// `from` meets it. The census by route, and each refusal no page meets
/// with why, is the commit message of `4-full-1-form-landing`'s build.
#[test]
fn every_refusal_a_leaving_request_meets_has_its_way_on_and_its_way_back() {
    let (scratch, web) = offline_web("webcensus");
    let world = scratch.0.clone();
    let mock = world.join("mock");
    crate::saved_json::put(&mock, "0000000000a1", "rings");
    let search = "search?realm=pc&text=rarity%3Drare";
    let tabs = "tabs?realm=pc&league=Standard&open=t1";
    let policy = |value: &str| {
        let out = harness::offline_acq(&world, &["policy", "set", "--replace", value]);
        assert!(out.status.success(), "{out:?}");
    };
    let standard = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["t1"],"max_age_seconds":3600}}}}}"#;
    let mut census: Vec<(&str, WayOn, harness::Reply, String)> = Vec::new();

    // POST saved: a name no search takes, its form kept as sent
    let empty = form_on(&web.get(search).body, "Save this search", None);
    census.push((
        "POST saved <an empty name>",
        WayOn::FormAsSent,
        send(&web, &empty, false),
        search.into(),
    ));
    // POST saved: a write over a row written since, offered over it
    let opened = web.get(&format!("{search}&saved=0000000000a1&saved_revision=1"));
    saved_cli(
        &world,
        &["set", "--id", "0000000000a1", "--name", "elsewhere"],
    );
    let over = form_on(&opened.body, "Save over \"rings\"", None).typed("name", "mine");
    let from = over.field("from").unwrap().to_string();
    census.push((
        "POST saved <a row written since>",
        WayOn::Offer,
        send(&web, &over, false),
        from,
    ));
    // POST saved/clear: a clear at a revision that moved, offered at it
    let asking = follow(
        &web,
        &link_saying(&web.get("saved").body, "Clear").replace("&amp;", "&"),
    );
    let clear = form_on(&asking.body, "Clear", None);
    saved_cli(&world, &["set", "--id", "0000000000a1", "--name", "moved"]);
    census.push((
        "POST saved/clear <a row written since>",
        WayOn::Offer,
        send(&web, &clear, false),
        "saved".into(),
    ));
    // GET saved/clear and saved/open: a row no longer there, the list
    // offered
    for (label, route) in [
        ("GET saved/clear <a row no longer there>", "saved/clear"),
        ("GET saved/open <a row no longer there>", "saved/open"),
    ] {
        let reply = web.get(&format!("{route}?id=0000000000ff&from=saved"));
        census.push((label, WayOn::Offer, reply, "saved".into()));
    }
    // GET plan, asked from the place page: nothing ticked, the page as
    // sent its way back; no policy stored, its command; a league the
    // policy does not cover, the plan of one it does
    let (_, unticked) = redirected(&web, &format!("/{}/{tabs}&refresh=ticked", web.token));
    census.push((
        "GET plan <nothing ticked>",
        WayOn::PageAsSent,
        unticked,
        tabs.into(),
    ));
    let (_, none) = redirected(&web, &format!("/{}/{tabs}&refresh=policy", web.token));
    census.push(("GET plan <no policy>", WayOn::Command, none, tabs.into()));
    policy(standard);
    let hardcore = "tabs?realm=pc&league=Hardcore";
    let (_, uncovered) = redirected(&web, &format!("/{}/{hardcore}&refresh=policy", web.token));
    census.push((
        "GET plan <a league not covered>",
        WayOn::Offer,
        uncovered,
        hardcore.into(),
    ));
    // POST apply: a plan whose policy moved, the plan compiled again
    let (_, plan) = redirected(&web, &format!("/{}/{tabs}&refresh=policy", web.token));
    let apply = form_on(&plan.body, "Apply", None);
    policy(
        r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["t2"],"max_age_seconds":3600}}}}}"#,
    );
    census.push((
        "POST apply <the policy moved>",
        WayOn::Offer,
        send(&web, &apply, false),
        tabs.into(),
    ));

    // a listing the realm's endpoints refuse is no page's offer, so no
    // page meets its refusal, which has no way on
    let poe2 = web.get("tabs?realm=poe2&league=Standard");
    assert_eq!(poe2.status, 200, "{}", poe2.body);
    assert!(
        !poe2.body.contains("List this league&#39;s stash tabs"),
        "{}",
        poe2.body
    );
    let mut pages = Pages::new(&world, &web);
    for (label, way, reply, from) in &census {
        assert_eq!(reply.status, 400, "{label}: {}", reply.body);
        let back = back_link(&web, from);
        assert!(
            reply.body.contains(&back),
            "{label}: {back}: {}",
            reply.body
        );
        pages.page(
            label,
            reply,
            &["<p class=\"back\"><a href=", "<div class=\"refusal\">"],
        );
        let refusal = &reply.body[reply.body.find("<div class=\"refusal\">").unwrap()..];
        match way {
            WayOn::FormAsSent => {
                let form = form_on(refusal, "Save", None);
                assert_eq!(form.field("name"), Some(""), "{label}");
                assert_eq!(form.field("text"), Some("rarity=rare"), "{label}");
                assert_eq!(form.field("from"), Some(from.as_str()), "{label}");
            }
            WayOn::PageAsSent => {
                assert!(back.contains("open=t1"), "{label}: {back}");
            }
            WayOn::Offer => {
                assert!(
                    refusal.contains("<ul class=\"offers\">"),
                    "{label}: {refusal}"
                );
            }
            WayOn::Command => {
                assert!(refusal.contains("`acq "), "{label}: {refusal}");
            }
        }
    }
    page_golden(file!(), "landing", &pages);
    // a write offered over a row written since shows the name the reader
    // typed in its box, to be read and changed before it is sent again
    let (_, _, conflict, _) = census
        .iter()
        .find(|(label, ..)| *label == "POST saved <a row written since>")
        .unwrap();
    assert!(
        conflict
            .body
            .contains("<label class=\"filter-title\">Name <input name=\"name\" value=\"mine\""),
        "{}",
        conflict.body
    );
    // a submit the daemon refuses for want of a session: its command
    let signed = harness::scratch("webcensussession");
    let daemon = harness::login(&signed.0, "Alice");
    let live = start_web(harness::command(&signed.0, &["web", "--no-browser"]));
    let shown = live.get("tabs?realm=pc&league=Standard");
    let out = harness::acq(&signed.0, &["auth", "logout", "--account", "Alice"]);
    assert!(out.status.success(), "{out:?}");
    for button in ["Fetch the league list", "List this league's stash tabs"] {
        let reply = send(&live, &form_on(&shown.body, button, None), false);
        assert_eq!(reply.status, 400, "{button}: {}", reply.body);
        let back = back_link(&live, "tabs?realm=pc&league=Standard");
        assert!(reply.body.contains(&back), "{button}: {}", reply.body);
        assert!(
            reply.body.contains("run `acq auth`"),
            "{button}: {}",
            reply.body
        );
    }
    drop(live);
    drop(daemon);
    // a refusal's JSON is the refusal alone: the page's `from` meets no
    // document (Decision 7)
    let json = web.raw(
        "POST",
        &format!("/{}/saved", web.token),
        None,
        &[
            ("Origin", &format!("http://{}", web.addr)),
            ("Content-Type", "application/x-www-form-urlencoded"),
            ("Accept", "application/json"),
        ],
        &format!(
            "from={}&name=&text=rarity%3Drare&realm=pc&membership=live",
            url_encoded(search)
        ),
    );
    assert_eq!(json.status, 400, "{}", json.body);
    assert!(!json.body.contains("from"), "{}", json.body);
}

/// `from` is read by one reader, and every place and step meets it first:
/// a step, a route no place names, a token, an absolute URL and a nested
/// `from` are each refused as usage, never dropped; a place's URL with a
/// `from` draws it, and its own forms carry the place itself, never that
/// `from`.
#[test]
fn a_from_is_a_place_of_this_server_read_once_and_never_carried_on() {
    let (scratch, web) = offline_web("webfrom");
    let token = web.token.clone();
    for (word, why) in [
        ("plan%3Frealm%3Dpc", "never to a step"),
        ("saved%2Fopen%3Fid%3D0000000000a1", "never to a step"),
        ("nowhere", "no page of this server"),
        (&*format!("{token}%2Fsearch"), "no page of this server"),
        (
            "https%3A%2F%2Fexample.com%2Fsearch",
            "no scheme, host or token",
        ),
        ("jobs%3Ffrom%3Dtabs", "no way back of its own"),
    ] {
        for route in [
            "jobs",
            "saved",
            "tabs?realm=pc&league=Standard",
            "search?text=x",
            "plan?realm=pc",
        ] {
            let joint = if route.contains('?') { '&' } else { '?' };
            let refused = refused_json(&web, &format!("{route}{joint}from={word}"), 400);
            assert_eq!(refused["kind"], "usage", "{route} {word}: {refused}");
            assert!(
                refused["error"].as_str().unwrap().contains(why),
                "{route}: {refused}"
            );
        }
    }
    let tabs = web.get("tabs?realm=pc&league=Standard&from=jobs%3Fjob%3D3");
    assert!(
        tabs.body.contains(&format!(
            "<p class=\"back\"><a href=\"/{token}/jobs?job=3\">Back to the jobs</a></p>"
        )),
        "{}",
        tabs.body
    );
    let listing = form_on(&tabs.body, "List this league's stash tabs", None);
    assert_eq!(listing.field("from"), Some("tabs?realm=pc&league=Standard"));
    assert!(
        !tabs.body.contains("from=jobs") && !tabs.body.contains("jobs%3Fjob"),
        "{}",
        tabs.body
    );
    // a search page is itself with the rows it keeps open, as its every
    // link writes it, so leaving it and coming back opens them again; a
    // row it no longer holds is dropped, as its links drop it
    let table = "search?realm=pc&text=class%3Arings&view.fields=icon";
    let opened = web.get(&format!("{table}&item=r1&item=no-such-item&from=saved"));
    assert_eq!(opened.status, 200, "{}", opened.body);
    for button in ["Save this search", "Fetch the league list"] {
        assert_eq!(
            form_on(&opened.body, button, None).field("from"),
            Some(format!("{table}&item=r1").as_str()),
            "{button}"
        );
    }
    drop(scratch);
}
