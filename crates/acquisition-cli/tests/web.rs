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

const USER: &str = "Alice#1234";
const ART: &str = "https://web.poecdn.com/gen/image/ring.png";

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
fn offline_web(tag: &str) -> (harness::Scratch, WebUi) {
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
        // the saved pages: a clear is a POST alone, an opening a GET
        // alone, and a write without the token or from another site's
        // page is refused before it runs
        ("GET", &format!("/{token}/saved/clear"), None, vec![], 405),
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
/// realm with the same limit, over the same store — but for the art each
/// row carries, which the page asks of the view (C140).
#[test]
fn the_fold_answers_as_acq_search_trade_does() {
    let (scratch, web) = offline_web("webfold");
    let mut page = web.get_json(&format!("search?{}", query_string(&ring_search())));
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
            "--json",
        ],
    );
    assert!(out.status.success(), "{out:?}");
    let cli = sole_json(&out);
    let rows = page["rows"].as_array_mut().unwrap();
    assert_eq!(rows.len(), 2, "{rows:?}");
    for row in rows.iter_mut() {
        assert_eq!(row["icon"], ART, "{row}");
        row.as_object_mut().unwrap().remove("icon");
    }
    assert_eq!(page, cli);
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
        ],
    );
    let answer = web.get(&format!("search?{}", query_string(&ring_search())));
    pages.page(
        "GET search?<the ring search>",
        &answer,
        &[
            "from the query JSON: exact",
            "total   2 matches",
            "Doom Loop",
            "Rune Coil",
            "alt=\"Doom Loop Two-Stone Ring\"",
            "class=\"name rarity-rare\"",
            "<p class=\"count\">2 matches</p>",
            "name=\"text\" rows=\"3\">league=Standard category=&quot;accessory.ring&quot;",
            "+95 to maximum Life",
        ],
    );
    // the query box: the answer holds the text it ran, to edit; an edited
    // text runs as the user's own query, the trade input dropped
    assert!(answer.body.contains("name=\"text\""), "{}", answer.body);
    let edited = web.get(&format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("text", "class:rings \"# to maximum Life\">=90")
        ])
    ));
    assert_eq!(edited.status, 200, "{}", edited.body);
    assert!(!edited.body.contains("trade   from"), "{}", edited.body);
    pages.page(
        "GET search?<an edited text>",
        &edited,
        &[
            "query   class:rings line(&quot;# to maximum Life&quot; arg1&gt;=90)",
            "total   1 match",
            "Doom Loop",
            "The query, in the language",
        ],
    );
    let counted = web.get(&format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("league", "Standard"),
            ("view.count", "rarity"),
            ("view.limit", "20"),
        ])
    ));
    assert_eq!(counted.status, 200, "{}", counted.body);
    assert!(!counted.body.contains("<table>"), "a count shows no rows");
    pages.page(
        "GET search?<the league counted by rarity>",
        &counted,
        &["count   rarity · 2 values", "4  rare", "1  unique"],
    );
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
            "class   Rings",
            "+95 to maximum Life",
            ART,
        ],
    );
    let refused = web.get("search?view.sort=no%20such%20value");
    assert_eq!(refused.status, 400);
    pages.page("GET search?view.sort=no such value", &refused, &["refusal"]);
    // a count asked of no realm over a store of two: each realm offered
    // keeps the count, and each link answers it (the review of #97's
    // build, its second finding) — a key in another case and a narrowing
    // of two texts among them, which fold back through the form's one
    // count field (the review at 3ce22d81)
    for (asked, spelled, said) in [
        ("rarity", "rarity", "count   rarity"),
        ("Rarity", "rarity", "count   rarity"),
        (
            "line%3Aresist%2Clife",
            "line%3Aresist%2Cline%3Alife",
            "count   line:resist",
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
    let untranslated = web.get(&format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            ("query.stats[0].type", "and"),
            ("query.stats[0].filters[0].id", "explicit.stat_9999999999"),
            ("view.limit", "20"),
        ])
    ));
    assert_eq!(untranslated.status, 200);
    pages.page(
        "GET search?<a stat the table lacks>",
        &untranslated,
        &["not the site&#39;s search", "PARTIAL", "stats.0.0"],
    );
    let undecided = web.get(&format!(
        "search?{}",
        query_string(&[
            ("realm", "pc"),
            (
                "query.filters.type_filters.filters.category.option",
                "accessory.ring"
            ),
        ])
    ));
    pages.page(
        "GET search?<every ring>",
        &undecided,
        &["undecided", "Wombgift Chalice"],
    );
    page_golden(file!(), "moved_world", &pages);
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
            "a folder holds tabs, not items: tick the tabs in it",
            "<summary>Stash tabs (",
            "pages of Uniques</summary>",
            "<summary>Characters (2)</summary>",
            "a page of Uniques: tick Uniques with the substashes box",
            "Refresh selected",
            "Plan under the policy",
        ],
    );
    assert!(!tabs.body.contains(" checked"), "no default selection");
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
/// under its query box (its own tail keeps the CLI's lines until step 4).
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
/// saved show` reads it, a conflict's `current.revision` — and a page
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
    let next = follow(&web, &at(&more));
    assert_eq!(next.status, 200, "{}", next.body);
    assert_eq!(
        entry(&next.body, "answer"),
        json_at(&web, &at(&more))["basis"]
    );
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
    let clear = form_on(&web.get("saved").body, "Clear", Some(&browsed));
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
    let clear = form_on(&web.get("saved").body, "Clear", Some(&browsed));
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
    let clear = form_on(&list.body, "Clear", Some("0000000000ff"));
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
    let clear = form_on(
        &web.get("saved?expand=true").body,
        "Clear",
        Some("00000000000a"),
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
    let clear = form_on(&web.get("saved").body, "Clear", Some(&id));
    let (status, cleared) = sent_json(&web, &clear);
    assert_eq!(status, 200, "{cleared}");
    let (status, back) = sent_json(&web, &offered_form(&cleared["undo"]));
    assert_eq!(status, 200, "{back}");
    assert_eq!(back["written"]["text"], "");
    assert_eq!(back["written"]["name"], "all of it");
}

/// Do 6's and Do 7's failing inputs: each refused, nothing written.
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
        let mut page = json_at(&web, &location);
        let cli = sole_json(&offline_acq(&world, &["search", "--saved", id, "--json"]));
        for row in page["rows"].as_array_mut().unwrap() {
            row.as_object_mut().unwrap().remove("icon");
        }
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
    let next = follow(&web, &format!("/{token}/{more}"));
    assert_eq!(next.status, 200, "{}", next.body);
    assert_eq!(
        json_at(&web, &format!("/{token}/{more}"))["scope"]["membership"],
        "all"
    );
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
        &["No saved searches yet: save one from any search&#39;s answer, under its query."],
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
            ">Clear</button>",
        ],
    );
    assert!(!main_of(&listed.body).contains("acq "), "{}", listed.body);
    // a create from an answer, and the list it lands on
    let answer = web.get(&format!("search?{}", query_string(&ring_search())));
    pages.page(
        "GET search?<the ring search>",
        &answer,
        &[
            "/saved\"><input type=\"hidden\" name=\"text\" value=\"league=Standard",
            "name=\"membership\" value=\"live\"",
            ">Save this search</button>",
        ],
    );
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
    let cleared = send(
        &web,
        &form_on(&list.body, "Clear", Some("0000000000a3")),
        false,
    );
    pages.redirect("POST saved/clear <boots>", &cleared);
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
    assert!(
        !saved_parts(&opened.body).contains("acq "),
        "{}",
        opened.body
    );
    saved_cli(&world, &["set", "--id", "0000000000a2", "--realm", "pc"]);
    pages.page(
        "GET search?<rings, opened, after the row moved>",
        &follow(&web, &location),
        &[
            "Saved search 0000000000a2, opened at revision 1, has been written since: it is now \
             &quot;rings&quot; at revision 2, last written",
            "via cli. Saving over it here is refused as stale and offers the write over revision 2.",
        ],
    );
    saved_cli(&world, &["set", "--id", "0000000000a2", "rarity=unique"]);
    pages.page(
        "GET search?<rings, opened, after its text changed>",
        &follow(&web, &location),
        &["This answer&#39;s query is not the saved one&#39;s: the saved query is rarity=unique over pc · live."],
    );
    saved_cli(
        &world,
        &["set", "--id", "0000000000a2", "--name", "unique rings"],
    );
    let reloaded = follow(&web, &location);
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
    pages.page(
        "GET search?<rings, opened, after it was cleared>",
        &follow(&web, &location),
        &[
            "Saved search 0000000000a2 is gone: no saved search has that id now. Saving here makes a new one.",
            ">Save this search</button>",
        ],
    );
    // the row this build cannot read, cleared from the list
    let list = web.get("saved");
    let cleared = send(
        &web,
        &form_on(&list.body, "Clear", Some("0000000000f1")),
        false,
    );
    pages.redirect(
        "POST saved/clear <the row this build cannot read>",
        &cleared,
    );
    let list = follow(&web, cleared.header("location").unwrap());
    assert!(!list.body.contains("cannot read"), "{}", list.body);
    pages.page("GET saved <after it>", &list, &["3 saved searches"]);
    // a save over a row since replaced by one this build cannot read
    let (_, amulets) = redirected(&web, &link_saying(&list.body, "amulets"));
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
    pages.page(
        "GET search?<everything, opened>",
        &follow(&web, open.header("location").unwrap()),
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
    pages.page(
        "GET search?<belts, opened, reloaded>",
        &follow(&web, &belted),
        &[
            "Saved search 0000000000a4 holds a value this build cannot read; clear it from",
            ">Save this search</button>",
        ],
    );
    page_golden(file!(), "saved_pages", &pages);
}
