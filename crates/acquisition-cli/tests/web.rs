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
    Pages, WebUi, offline_acq, offline_command, page_golden, sole_json, start_web, unaged,
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
    let cases: [Case; 8] = [
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
    assert_eq!(unaged(page), unaged(cli));
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
    page_golden(file!(), "search_pages", &pages);
}

/// The form is one realm's render (the review of 4b, its fourth
/// finding): asked for the console realm, it lists that realm's league
/// alone and its search carries that realm.
#[test]
fn the_form_renders_the_realm_it_is_asked_for() {
    let (scratch, web) = offline_web("webrealm");
    let mut pages = Pages::new(&scratch.0, &web);
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

/// The refresh loop through the pages, over the mock: the plan page with
/// its quote, apply posting the envelope whole, the jobs page until every
/// job is done, the same envelope posted again sending nothing (C111), a
/// ticked selection compiled into the plan `refresh --tabs` applies,
/// `refresh --all` the same plan over every tab with its listing (C76),
/// and a plan whose policy moved refused (C44).
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
    let mut pages = Pages::new(&base, &web);

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
    let envelope = hidden_plan(&plan.body);
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

    // a second plan and apply through the CLI fetch every tab, which
    // lands the map and unique tabs' pages: the tabs page lists them
    let out = harness::acq(&base, &["refresh", "--apply"]);
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

    // `refresh --all` is a selection too: every tab, the listing
    // fetched beside it (C76), applied through the same apply kind
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

    let moved = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":["dump"],"max_age_seconds":3600}}}}}"#;
    assert!(
        harness::acq(&base, &["policy", "set", "--replace", moved])
            .status
            .success()
    );
    let refused = web.post("apply", &[("plan", &envelope)]);
    assert_eq!(refused.status, 400, "{}", refused.body);
    pages.page(
        "POST apply <a plan whose policy moved>",
        &refused,
        &["the sync policy moved"],
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
/// first finding): a server started over Alice, the sole account on disk,
/// keeps applying as Alice after Alice logs out and Bob logs in — the
/// daemon's sole live session — so the apply of a selection's plan is
/// refused for Alice and no job runs as Bob; with both logged in, where a
/// submit naming no account would be ambiguous, the job is Alice's.
#[test]
fn a_selection_submits_as_the_served_account_whatever_the_daemon_holds() {
    let scratch = harness::scratch("webacct");
    let base = scratch.0.clone();
    let _daemon = harness::login(&base, "Alice");
    let web = start_web(harness::command(&base, &["web", "--no-browser"]));
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
