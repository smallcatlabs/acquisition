//! `acq web` at process level (C138, C139; `roadmap/briefs/4b-build.md`):
//! the listener answers only its own pages, at its own address, with the
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
        ("query.stats[0].type", "and"),
        ("query.stats[0].filters[0].id", "explicit.stat_3299347043"),
        ("query.stats[0].filters[0].value.min", "70"),
        ("query.stats[0].filters[1].id", ""),
        ("view.limit", "20"),
        ("view.sort", ""),
    ]
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

/// The count beside a search's button (4-full part 1, step 6.2, #245),
/// pinned by equality: a form's `count` is `acq search` of the same
/// request at `--limit 0` and `--sort none`, JSON whatever was asked, no
/// row in it. The view the form names, a sort or a count's keys, is not
/// read, nor refused where its values are no view's; the query box's count, the saved search it carries set aside, is
/// its search's total.
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

/// #188: a save from a site search's answer stores what any save
/// stores, the translation's text in the answer's realm and membership;
/// the site search itself is in no row.
#[test]
fn a_save_from_a_trade_answer_stores_its_translated_text() {
    use crate::search_json::{TRADE_ID, TRADE_TEXT, trade_url};
    let (scratch, web) = offline_web("webtradesave");
    let url = trade_url();
    let answer = web.get(&format!(
        "search?{}",
        query_string(&[("realm", "xbox"), ("trade", url.as_str())])
    ));
    assert_eq!(answer.status, 200, "{}", answer.body);
    let save = form_on(&answer.body, "Save this search", None);
    assert_eq!(save.field("text"), Some(TRADE_TEXT), "{save:?}");
    assert_eq!(save.field("realm"), Some("pc"), "{save:?}");
    landed_on_the_list(&web, &send(&web, &save.typed("name", "life rings"), false));
    let row = &saved_json(&scratch.0, &["list"])["searches"][0];
    assert_eq!(
        (&row["name"], &row["text"], &row["scope"]["realm"]),
        (&json!("life rings"), &json!(TRADE_TEXT), &json!("pc")),
        "{row}"
    );
    assert!(!row.to_string().contains(TRADE_ID), "{row}");
}

/// The query box sorts and pages as the big form does (the page seat's
/// fault 6): its three controls are on the form page, a typed query with
/// a sort answers as `acq search --sort --desc` does, and the answer's
/// box comes back set to the rows view it showed.
#[test]
fn the_query_box_sorts_and_pages_a_typed_query() {
    let (scratch, web) = offline_web("webquerysort");
    let form = web.get("");
    let boxed = &form.body[form.body.find("class=\"query-box\"").unwrap()..];
    for control in [
        "<select name=\"view.limit\"><option value=\"\" selected>10</option>",
        "name=\"view.sort\" value=\"\"",
        "name=\"view.desc\" value=\"true\">",
    ] {
        assert!(boxed.contains(control), "{control}: {boxed}");
    }
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
        "name=\"view.sort\" value=\"ilvl\"",
        "name=\"view.desc\" value=\"true\" checked>",
    ] {
        assert!(boxed.contains(control), "{control}: {boxed}");
    }
}

/// The search pages, each a golden naming what a reader must find: the
/// form with each control's fate and the listing-only filters' sentence,
/// the answer with its clause statuses, its total and its rows with their
/// art, one item, and a refusal in the search's own words.
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
            "fate fate-exact",
            "Find a stat",
            "<summary>Type Filters</summary>",
            "name=\"defences\" value=\"displayed\"",
            "name=\"query.filters.armour_filters.disabled\" value=\"true\"",
            "name=\"query.stats[0].disabled\" value=\"true\"",
            "<summary>Stat Filters</summary>",
            "placeholder=\"Search items\"",
            "class=\"search-btn\"",
            "A trade site search: its URL, its id or its query JSON",
            "name=\"trade\"",
            "Run the site's search",
        ],
    );
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
            "<p class=\"place\">pc · Standard / Rings",
            " ago</p>",
            "<a class=\"presentation\" href=\"search?",
            ">as a table</a>",
            // the order the rows stand in, where no sort was asked: the
            // query's one value, the stat filter's explicit sum (#38)
            "rows 1–2 of 2 shown, by sum(line(&quot;# to maximum Life&quot; source=explicit).arg1), largest first: the one value the query compares",
            // the query box's presentation, the big form's
            "Results as <select name=\"view.fields\"><option value=\"\" selected>cards</option>",
        ],
    );
    assert!(!answer.body.contains("class=\"window\""), "{}", answer.body);
    // an exact translation saves as the site's search: no note
    assert!(
        !answer.body.contains("class=\"save-note\""),
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
            "<p class=\"rows-shown\">2 matching items in the table, by sum(",
            "<div class=\"window\" data-row-height=\"24\" data-block-rows=\"50\">",
            "<span>Item</span><span>Base</span><span>Price</span><span>Location</span><span>Fetched</span><span>sum(line(",
            "/rows?next=acq1.",
            "data-count=\"2\"",
            "<div class=\"block\" data-block=\"0\">",
            "data-i=\"0\" data-id=\"r1\"><a class=\"name rarity-rare\" href=\"item/r1\">Doom Loop Two-Stone Ring</a>",
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
            // what a save of it keeps (the page seat of 2026-11-08, its
            // fault 1)
            "<p class=\"save-note\">Saved, this is the query as translated above, not the site&#39;s search: the clauses not run are not saved.</p>",
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
            "rows 1–1 of 3 shown, in the store&#39;s order (no sort asked) · <a class=\"way\" data-way=\"/view/rows/next\" href=\"/",
            "search?next=acq1.",
            "\">more</a>",
            "data-way=\"/terms/0/undecided\" href=",
        ],
    );
    every_request_an_answer_carries_is_a_way_on_its_page(&web, first_path, &first);
    // the page "more" reaches keeps the other way and has the way back to
    // the first rows, each answering (the page seat of 2026-11-08, its
    // fault 6)
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
            "rows 2–2 of 3 shown, in the store&#39;s order (no sort asked)",
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
    // a sort beside a count is the shared view selection's refusal, as
    // the CLI's `--count` beside `--sort` is
    let both = web.get("search?realm=pc&view.count=rarity&view.sort=ilvl");
    assert_eq!(both.status, 400, "{}", both.body);
    pages.page(
        "GET search?realm=pc&view.count=rarity&view.sort=ilvl",
        &both,
        &["orders rows, and a count shows none"],
    );
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
    // a count asked of no realm over a store of two: each realm offered
    // keeps the count, and each link answers it (the review of #97's
    // build, its second finding) — a key in another case and a narrowing
    // of two texts among them, which fold back through the form's one
    // count field (the review at 3ce22d81)
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
    // search the form asks, never a command line (#97); followed, it
    // answers
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
    // page says which clause and why, the term under `if` included (#150,
    // the cold review of 23f225c1)
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
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "search_pages", &pages);
}

/// The form is one realm's render (the review of 4b, its fourth
/// finding): asked for the console realm, it lists that realm's league
/// alone and its search carries that realm. A server started with no
/// account on disk serves (`roadmap/briefs/99-web-leftovers.md`, Do 1):
/// every page is the `not_logged_in` refusal with its remedy, a page
/// asked as JSON its document, the stylesheet still served; once an
/// account is on disk, a reload serves its form. A server asked for an
/// account by name over the same empty world does not start: it refuses
/// as `not_logged_in` before the bind (the review at 49a74010, its first
/// finding).
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
    // the nav bar says no account is served yet (the page seat of
    // 2026-11-08, its fault 11)
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
    // a league with no tabs: the refresh page's facts line, an empty
    // listing apart from one never seen (the review of round C, finding 3)
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
        let page = web.get(&format!("refresh?realm=pc&league={league}"));
        pages.page(
            &format!("GET refresh?realm=pc&league={league}"),
            &page,
            &["no tabs known for", said],
        );
    }
    panes_golden(file!(), "search_form", &mut pages);
    page_golden(file!(), "console_realm", &pages);
}

/// The moved-world review (Do 9): what a new league brings first renders
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
            ("query.stats[0].type", "and"),
            ("query.stats[0].filters[0].id", "explicit.stat_9999999999"),
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
    page_golden(file!(), "moved_world", &pages);
}

/// A way on as a page marks it (4-full part 1, step 4): the pointer its
/// `data-way` names, and its link where it is one.
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

/// Every request an answer carries is a way on its page (4-full part 1,
/// step 4, Do 8): for each object of the page's document holding a
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

/// C143 on the answer and item pages (4-full part 1, step 4): every page
/// headed `GET search` or `GET item/` that answered in a golden under
/// `tests/goldens/web/` holds no `acq ` and no ` --` before a letter —
/// the ways on are the page's own. Read from the files, as `no_ids.rs`
/// reads them: the pages served are the goldens'. The refresh and jobs
/// pages are not read; they print the CLI's command lines still.
#[test]
fn an_answer_or_item_page_prints_no_command_line_and_no_flag() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/web");
    let mut read = 0;
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
            if !(label.starts_with("GET search") || label.starts_with("GET item/")) || !answered {
                continue;
            }
            read += 1;
            for line in &page[1..] {
                let flag = line.match_indices(" --").any(|(at, _)| {
                    line[at + 3..]
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_alphabetic())
                });
                assert!(
                    !line.contains("acq ") && !flag,
                    "{}: {label}: {line}",
                    file.display()
                );
            }
        }
    }
    assert!(read > 20, "only {read} answer and item pages read");
}

/// The ways no seeded page reaches are a wording golden (4-full part 1,
/// step 4, Do 7): a world staged apart from `seed`'s pages, each page
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
            "/refresh?realm=pc&amp;league=Standard\">each tab of Standard</a>",
            "1 other league holds never-fetched tabs",
        ],
    );
    check(
        &mut pages,
        "GET search?<every ring outside Standard>",
        "search?realm=pc&text=class%3Arings+-league%3DStandard",
        &["/refresh?realm=pc&amp;league=Hardcore\">each tab of Hardcore</a>"],
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
            ring("plain", &["25% increased Rarity of Items found"]),
            ring("both", &["30% increased Rarity of Items found", "10% reduced Rarity of Items found"]) ] } }),
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
                    r#"{"query": {"stats": [{"type": "and", "filters": [{"id": "explicit.stat_3917489142", "value": {"min": 20}}, {"id": "explicit.stat_1030153674"}]}]}}"#
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
            "<span>sum(line(template:spirit).arg1)</span></div>",
            "<a class=\"name rarity-rare\" href=\"item/many\">Many Loop Iron Ring</a>",
            "<span>36</span></div>",
        ],
    );
    // as a card the item stands whole beside what the row shows, so the
    // row says no "more" (the cold review of step 5's build, finding 6)
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
            "<span>Fetched</span><span>ilvl</span><span>",
            "href=\"item/lp\">Ledger Priced Iron Ring</a><span>Iron Ring</span><span>5 chaos",
            "href=\"item/lo\">Ledger Open Iron Ring</a><span>Iron Ring</span><span>undecided: the note cannot be read",
            "<span>incomplete at 70</span>",
            "<span>none (",
        ],
    );
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
    page_golden(file!(), "table_rows", &pages);
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

/// The refresh loop through the pages, over the mock: the plan page with
/// its quote, apply posting the envelope whole, the jobs page until every
/// job is done, the same envelope posted again sending nothing (C111),
/// `refresh --all` a selection over every tab with its listing (C76), a
/// ticked selection compiled into the plan `refresh --tabs` applies, and
/// a plan whose policy moved refused (C44).
#[test]
fn the_refresh_loop_runs_through_the_pages() {
    let scratch = harness::scratch("webloop");
    let base = scratch.0.clone();
    let _daemon = harness::login_as_mock(&base, &[]);
    let policy = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}"#;
    assert!(
        harness::acq(&base, &["policy", "set", policy])
            .status
            .success()
    );
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let mut pages = Pages::new(&base, &web).fetched_live();

    let plan = web.get("plan?realm=pc&league=Standard");
    pages.page(
        "GET plan?realm=pc&league=Standard",
        &plan,
        &[
            "2 requests:",
            "list stashes",
            "list characters",
            "quote",
            ">Apply</button>",
        ],
    );
    // the page's way on is its Apply button, never the command a terminal
    // prints beside the plan (#97)
    assert!(!plan.body.contains("next: acq"), "{}", plan.body);
    // a league the policy does not cover: its way on is the plan of the
    // one it does, a link to that page (#97)
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
    let asked = web.raw(
        "GET",
        &format!("/{}/plan?realm=pc&league=Hardcore", web.token),
        None,
        &[("Accept", "application/json")],
        "",
    );
    let refused: Value = serde_json::from_str(&asked.body).unwrap();
    let link = refused["offers"][0]["request"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(web.raw("GET", &link, None, &[], "").status, 200, "{link}");
    let envelope = hidden_plan(&plan.body);
    // C16: the plan page's basis line is the basis of the envelope it holds
    let held: Value = serde_json::from_str(&envelope).unwrap();
    assert_eq!(
        basis_line(&plan.body).expect("the plan page's basis line")["plan"],
        held["basis"],
        "{}",
        plan.body
    );
    let applied = web.post("apply", &[("plan", &envelope)]);
    assert_eq!(applied.status, 303, "{}", applied.body);
    assert_eq!(
        applied.header("location"),
        Some(format!("/{}/jobs", web.token).as_str())
    );
    let jobs = probes_in_order(until_done(&web));
    pages.page(
        "GET jobs <every job done>",
        &jobs,
        &["rails:  tripwire", "apply", "done", "web:"],
    );
    let replay = web.post("apply", &[("plan", &envelope)]);
    assert_eq!(replay.status, 200, "{}", replay.body);
    pages.page(
        "POST apply <the same envelope again>",
        &replay,
        &["nothing to do: every request of this plan is spent"],
    );

    // `refresh --all` is a selection too: every tab, the listing
    // fetched beside it (C76), applied through the same apply kind; it
    // lands the map and unique tabs' pages, which the tabs page lists.
    // It runs before the two selections below so the run's tab fetches
    // stay inside the mock's stash policy, 15 in 10 s: past it an apply
    // waits out the window in real time, which nothing here tests
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
    let listed = sole_json(&harness::acq(&base, &["jobs", "--all", "--json"]));
    let all = listed["rows"]
        .as_array()
        .unwrap()
        .iter()
        .rfind(|j| j["kind"] == "apply")
        .unwrap();
    assert!(
        all["submitted_by"].as_str().unwrap().starts_with("cli:"),
        "{all}"
    );
    let sent: Vec<(&Value, &Value)> = all["params"]["jobs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|j| (&j["kind"], &j["params"]["id"]))
        .collect();
    assert_eq!(sent[0], (&json!("stashes"), &Value::Null), "{all}");
    let tabs: Vec<&Value> = sent[1..].iter().map(|(_, id)| *id).collect();
    for id in ["cur1", "dump", "old1", "old2", "uniq", "maps", "mape"] {
        assert!(tabs.contains(&&json!(id)), "{id} not fetched: {all}");
    }
    assert!(!tabs.contains(&&json!("fold")), "a folder fetched: {all}");
    // the characters the selection left, fetched by name: the tabs page
    // lists them
    let out = harness::acq(&base, &["refresh", "--characters", "fake0001,fake0002"]);
    assert!(out.status.success(), "{out:?}");
    // no league list on disk yet: no select, the fetch button alone; the
    // button fetches it, and the select lists it
    let unlisted = web.get("refresh?realm=pc&league=Standard");
    assert!(
        !unlisted.body.contains("<select name=\"league\">"),
        "{}",
        unlisted.body
    );
    assert!(
        unlisted.body.contains("Fetch the league list"),
        "{}",
        unlisted.body
    );
    let fetched = web.post("leagues", &[]);
    assert_eq!(fetched.status, 303, "{}", fetched.body);
    until_done(&web);
    let tabs = web.get("refresh?realm=pc&league=Standard");
    pages.page(
        "GET refresh?realm=pc&league=Standard",
        &tabs,
        &[
            "tabs in Standard",
            "<option value=\"Standard\" selected>Standard (tabs on disk)</option>",
            "name=\"tab\" value=\"dump\"",
            "name=\"character\"",
            "name=\"tab\" value=\"old1\"",
            "<summary>Old leagues · Folder · 2 tabs</summary>",
            "<summary>Stash tabs (",
            "aria-label=\"filter the Tab column\"",
            "aria-label=\"filter the Character column\"",
            "/assets/filter.js\"",
            "pages of Uniques</summary>",
            "<summary>Characters (2)</summary>",
            "a page of Uniques: tick Uniques with the substashes box",
            "Refresh selected",
            "Plan under the policy",
        ],
    );
    assert!(!tabs.body.contains(" checked"), "no default selection");
    // the folder's tabs in the folder's group, the buttons and the
    // substashes box above the lists, and no filter box sent (the page
    // seat's probe)
    let at = |words: &str| {
        tabs.body
            .find(words)
            .unwrap_or_else(|| panic!("{words}: {}", tabs.body))
    };
    let folder = at("<details class=\"folder holds\" open><summary>Old leagues");
    let folder_end = folder + tabs.body[folder..].find("</details>").unwrap();
    for inside in ["value=\"old1\"", "value=\"old2\""] {
        assert!(
            (folder..folder_end).contains(&at(inside)),
            "{inside} outside the folder"
        );
    }
    // the folder's tabs are its members, indented under it; the tabs
    // after it are the list's own, at its level (the page seat of
    // 2026-11-08, its fault 15)
    for (member, tab) in [
        (true, "old1"),
        (true, "old2"),
        (false, "uniq"),
        (false, "maps"),
    ] {
        let row = format!(
            "<tr class=\"{}\"><td><input type=\"checkbox\" name=\"tab\" value=\"{tab}\">",
            if member { "row member" } else { "row" }
        );
        assert!(tabs.body.contains(&row), "{row}: {}", tabs.body);
    }
    assert!(at("value=\"old2\"") < folder_end && folder_end < at("value=\"uniq\""));
    assert!(at("value=\"uniq\"") < at("value=\"maps\""));
    assert!(at("Refresh selected") < at("<summary>Stash tabs ("));
    assert!(at("Plan under the policy") < at("<summary>Characters ("));
    // a league with no character beside a realm with two: the page says
    // the terminal's own sentence, which names where they are (the page
    // seat's fault 11)
    let elsewhere = web.get("refresh?realm=pc&league=Hardcore");
    assert_eq!(elsewhere.status, 200, "{}", elsewhere.body);
    let said = harness::acq(&base, &["store", "characters", "--league", "Hardcore"]);
    let said = String::from_utf8(said.stdout).unwrap();
    assert!(
        said.starts_with("no characters in Hardcore: the 2 known are in Standard"),
        "{said}"
    );
    assert!(
        unescaped(main_of(&elsewhere.body)).contains(said.trim_end()),
        "{said}: {}",
        elsewhere.body
    );
    assert!(at("name=\"deep\"") < at("<th>Tab</th>"));
    assert!(
        !tabs.body.contains("a folder holds"),
        "the folder's note stands"
    );
    for filter in tabs.body.split("<input class=\"col-filter\"").skip(1) {
        let tag = &filter[..filter.find('>').unwrap()];
        assert!(!tag.contains("name="), "a filter box is sent: {tag}");
    }
    for unticked in ["fold", "u001", "m001"] {
        assert!(
            !tabs.body.contains(&format!("value=\"{unticked}\"")),
            "{unticked} carries a checkbox"
        );
    }
    // a ticked selection is a plan (C44 as amended): its page, then the
    // one apply route; `refresh --tabs` compiles and applies the same
    let selected = web.get("plan?realm=pc&league=Standard&selected=yes&tab=dump&tab=cur1");
    assert_eq!(selected.status, 200, "{}", selected.body);
    pages.page(
        "GET plan?<two tabs selected>",
        &selected,
        &["a selection", "fetch 2 tabs", ">Apply</button>"],
    );
    // C16: the basis line is the envelope's own, read unmasked (the
    // golden masks its listings' response ids, #192)
    let held: Value = serde_json::from_str(&hidden_plan(&selected.body)).unwrap();
    assert_eq!(
        basis_line(&selected.body).expect("the plan page's basis line")["plan"],
        held["basis"],
        "{}",
        selected.body
    );
    let applied = web.post("apply", &[("plan", &hidden_plan(&selected.body))]);
    assert_eq!(applied.status, 303, "{}", applied.body);
    until_done(&web);
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

    let moved = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["dump"],"max_age_seconds":3600}}}}}"#;
    assert!(
        harness::acq(&base, &["policy", "set", "--replace", moved])
            .status
            .success()
    );
    let refused = web.post("apply", &[("plan", &envelope)]);
    assert_eq!(refused.status, 400, "{}", refused.body);
    // its way on is the plan page, a link, never the command a terminal
    // types (the review of #97's build, its first finding); followed, the
    // plan compiled again from the policy as it stands
    assert!(!refused.body.contains("acq "), "{}", refused.body);
    pages.page(
        "POST apply <a plan whose policy moved>",
        &refused,
        &[
            "the sync policy moved",
            "plan?realm=pc&amp;league=Standard\">the plan of pc Standard compiled again, from the policy as it stands</a>",
        ],
    );
    let replanned = web.raw("GET", &offered_links(&refused.body)[0], None, &[], "");
    assert_eq!(replanned.status, 200, "{}", replanned.body);
    assert!(
        replanned.body.contains("<h1>Plan</h1>"),
        "{}",
        replanned.body
    );
    page_golden(file!(), "refresh_loop", &pages);
    drop(web);
    assert!(harness::acq(&base, &["daemon", "stop"]).status.success());
}

/// The jobs page under a halt: a trip staged in the mock world's rails, as
/// the rails tests stage it, and the daemon started with the tripwire on.
/// The page opens with the halt and its advice in the words `daemon
/// status` gives them, the one maker both render. No deadline but the
/// hang ceiling (C137).
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
    std::fs::write(
        &rails,
        r#"{"tripped":"429 on GET /stash/Standard (staged trip)"}"#,
    )
    .unwrap();
    let (_daemon, _pid) = harness::start_daemon_with(&base, &[("ACQ_TRIPWIRE", "1")]);
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
    let mut pages = Pages::new(&base, &web);
    let jobs = web.get("jobs");
    assert_eq!(jobs.status, 200, "{}", jobs.body);
    pages.page(
        "GET jobs <the daemon halted>",
        &jobs,
        &[
            "rails:  tripwire ON",
            "HALTED: 429 on GET /stash/Standard (staged trip)",
            "acq daemon reset-tripwire",
        ],
    );
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

/// A jobs page with its probes' rows in one order: the daemon starts the
/// two routes' first-contact probes together, so which takes the lower
/// id is the scheduler's; the rows keep their places, their ids masked.
fn probes_in_order(mut jobs: harness::Reply) -> harness::Reply {
    let lines: Vec<&str> = jobs.body.lines().collect();
    let at: Vec<usize> = (0..lines.len())
        .filter(|&i| lines[i].contains("  probe  "))
        .collect();
    let mut probes: Vec<String> = at
        .iter()
        .map(|&i| {
            let line = lines[i].trim_start();
            let digits = line.bytes().take_while(u8::is_ascii_digit).count();
            format!(" <n>{}", &line[digits..])
        })
        .collect();
    probes.sort();
    let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    for (i, probe) in at.into_iter().zip(probes) {
        out[i] = probe;
    }
    jobs.body = out.join("\n");
    jobs
}

/// The served account, named on every submit (the review of 4b, its
/// first finding): a server started with no account on disk refuses
/// every page as `not_logged_in` until Alice logs in, then serves Alice,
/// the account it found (the review at 49a74010, its second finding),
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

// ---- the saved pages (C16, C168; `roadmap/briefs/4-full-1-pages.md`) -----

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
    assert!(
        form.action.starts_with(&format!("/{}/", web.token)),
        "{form:?}"
    );
    web.raw("POST", &form.action, None, &headers, &body)
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

/// A write answered as a browser asks it: `303` to the list.
fn landed_on_the_list(web: &WebUi, reply: &harness::Reply) {
    assert_eq!(reply.status, 303, "{}", reply.body);
    assert_eq!(
        reply.header("location"),
        Some(format!("/{}/saved", web.token).as_str())
    );
    assert!(reply.body.is_empty(), "{}", reply.body);
}

/// A clear as a page makes it (the owner, "Clear should ask first"):
/// the list's Clear link for `id`, never a form, followed to the asking
/// page, which writes nothing (the file's revision unchanged across it),
/// and the clear's one form on that page.
fn asked_clear(web: &WebUi, world: &Path, list: &str, id: &str) -> (harness::Reply, PageForm) {
    let link = format!("/{}/saved/clear?id={id}", web.token);
    assert!(
        list.contains(&format!("href=\"{link}\">Clear</a>")),
        "no Clear link for {id}:\n{list}"
    );
    let before = file_revision(world);
    let asking = follow(web, &link);
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

/// The words of the scan (C16, Do 9): a script that names any of them
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

/// C16's state sentence, as far as a lexical scan holds it: every page a
/// golden records passes the harness's seam (`harness::no_state`: no
/// cookie set, no script named off `harness::SCRIPTS`), and each script
/// on that list, fetched as a page fetches it, names no storage, cookie
/// or history API. The breakers: each word in a script's text is refused,
/// and the seam refuses a reply that sets a cookie and one naming a
/// script off the list. What another spelling reaches is the review's
/// (`REVIEW-CHECKLIST.md`).
#[test]
fn c16_no_served_script_names_a_storage_cookie_or_history_api() {
    for word in NO_STATE {
        let broken = format!("(function () {{ const x = window.{word}; }})();");
        assert_eq!(state_words(&broken), vec![word], "the scan misses {word}");
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
    let listed = format!("<script src=\"/{token}/assets/picker.js\"></script>");
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
    assert!(form.body.contains("/assets/picker.js\""), "{}", form.body);
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

/// Do 6's and Do 7's failing inputs, and a clear asked first with no
/// row to ask about: each refused, nothing written.
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

/// The routes (Do 5): `saved/open` writes nothing and answers `303` to
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

/// Morning 1's handoff on the owner's side: a row the terminal saves is on
/// the list page, and a row the page saves is in `acq saved list`, its
/// channel `web`.
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

/// The saved pages, a golden naming what a reader must find (Do 10): the
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
                &json!({ "version": 2, "name": "x", "text": "x", "scope": {} }),
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
            "saved/open?id=0000000000a2\">rings</a>",
            "<td>pc · live</td><td>rarity=rare</td><td>1</td>",
            "via test",
            "this build cannot read it:",
            ">Rename</button>",
            "saved/clear?id=0000000000a2\">Clear</a>",
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
            "/saved\"><input type=\"hidden\" name=\"text\" value=\"league=Standard",
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
            "<textarea id=\"query-text\" name=\"text\" rows=\"3\"></textarea>",
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
    page_golden(file!(), "saved_pages", &pages);
}
