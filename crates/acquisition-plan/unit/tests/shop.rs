use super::*;
use crate::listing::resolve;
use acquisition_store::{
    AnnotationRow, CharacterSnapshot, ItemSnapshot, ListingBasis, PricingSnapshot, TabSnapshot,
};
use serde_json::{Value, json};

fn tab(
    id: &str,
    parent: Option<&str>,
    name: &str,
    r#type: &str,
    public: bool,
    idx: Option<i64>,
) -> TabSnapshot {
    TabSnapshot {
        id: id.into(),
        parent: parent.map(str::to_string),
        name: name.into(),
        r#type: r#type.into(),
        idx,
        listed_at: Some(1_000),
        listed_response: Some(2),
        fetched_at: Some(1_100),
        metadata: if public {
            json!({ "public": true })
        } else {
            Value::Null
        },
        item_count: 0,
    }
}

fn item(id: &str, kind: &str, location: &str, x: i64, y: i64) -> ItemSnapshot {
    ItemSnapshot {
        id: id.into(),
        location_kind: kind.into(),
        location_id: location.into(),
        container: Some(
            if kind == "stash" {
                "items"
            } else {
                "equipment"
            }
            .into(),
        ),
        socketed_in: None,
        name: String::new(),
        type_line: "Chaos Orb".into(),
        stack_size: None,
        x: Some(x),
        y: Some(y),
        note: None,
        note_unread: false,
        inventory_id_unread: false,
        inventory_id: Some(
            if kind == "stash" {
                "Stash1"
            } else {
                "BodyArmour"
            }
            .into(),
        ),
        seen_response: Some(7),
        last_seen: 1_200,
    }
}

fn row(scope: &str, key: &str, value: Value) -> AnnotationRow {
    AnnotationRow {
        scope: scope.into(),
        key: key.into(),
        kind: "buyout".into(),
        value,
        revision: 3,
        created_at: 900,
        updated_at: 950,
        written_via: "test".into(),
        actor: None,
    }
}

fn exact(amount: &str, currency: &str) -> Value {
    json!({ "version": 1, "type": "exact", "amount": amount, "currency": currency })
}

fn character(id: &str, name: &str, league: Option<&str>) -> CharacterSnapshot {
    CharacterSnapshot {
        id: id.into(),
        name: name.into(),
        league: league.map(str::to_string),
        listed_at: Some(1_001),
        listed_response: Some(3),
        fetched_at: Some(1_101),
        listed: Value::Null,
        fetched: Value::Null,
    }
}

/// Every cell of the table has an item: a folder holding a public
/// priced tab (the game lists most of it; one item priced by hand
/// beats the tab name, one skipped by hand, one noted so the game
/// wins the tie, one socketed, one a ratio, one a retired tag, one
/// no_price); a non-public priced tab whose row prices it by hand; a
/// public unpriced tab with an item nothing applies to and one with
/// an unreadable row; a map tab's substash; a tab never listed; a
/// character with a worn item and a socketed gem; a league-less
/// character with a priced item, an unpriced one and a skipped one.
fn snapshot() -> PricingSnapshot {
    let mut items = vec![
        item("i-game", "stash", "c1", 0, 0),
        item("i-hand", "stash", "c1", 1, 0),
        item("i-hand2", "stash", "c1", 2, 0),
        item("i-bo", "stash", "c1", 3, 0),
        item("i-skip", "stash", "c1", 4, 0),
        item("i-noted", "stash", "c1", 5, 0),
        item("i-gem", "stash", "c1", 0, 0),
        item("i-ratio", "stash", "c1", 6, 0),
        item("i-retired", "stash", "c1", 7, 0),
        item("i-noprice", "stash", "c1", 8, 0),
        item("i-gameskip", "stash", "c1", 9, 0),
        item("i-tabrow", "stash", "t2", 0, 1),
        item("i-plain", "stash", "t3", 0, 0),
        item("i-unres", "stash", "t3", 1, 0),
        item("i-sub", "stash", "s1", 0, 0),
        item("i-unlisted", "stash", "u1", 0, 0),
        item("i-worn", "character", "ch1", 0, 0),
        item("i-worngem", "character", "ch1", 0, 0),
        item("i-nowhere", "character", "ch2", 0, 0),
        item("i-nowhere-plain", "character", "ch2", 1, 0),
        item("i-nowhere-skip", "character", "ch2", 2, 0),
    ];
    items[5].note = Some("~price 4 chaos".into());
    items[6].socketed_in = Some("i-game".into());
    items[6].x = None;
    items[6].y = None;
    items[6].inventory_id = None;
    items[10].note = Some("~skip".into());
    items[11].seen_response = Some(1);
    items[11].last_seen = 100;
    items[17].socketed_in = Some("i-worn".into());
    items[17].x = None;
    items[17].y = None;
    items[17].inventory_id = None;
    PricingSnapshot {
        account_uuid: "u-1".into(),
        account_name: Some("tom".into()),
        realm: "pc".into(),
        league: "Standard".into(),
        taken_at: 2_000,
        stash_listing: Some(ListingBasis {
            response_id: 2,
            fetched_at: 1_000,
        }),
        tabs: vec![
            tab("f1", None, "Sale", "Folder", false, Some(0)),
            tab(
                "c1",
                Some("f1"),
                "~price 3 chaos",
                "PremiumStash",
                true,
                Some(1),
            ),
            tab(
                "t2",
                None,
                "~price 9 chaos (Remove-only)",
                "PremiumStash",
                false,
                Some(2),
            ),
            tab("t3", None, "Plain", "PremiumStash", true, Some(3)),
            tab("m1", None, "Maps", "MapStash", false, Some(4)),
            tab("s1", Some("m1"), "1", "MapStash", false, Some(5)),
            tab("u1", None, "Fetched only", "PremiumStash", false, None),
        ],
        character_listing: Some(ListingBasis {
            response_id: 3,
            fetched_at: 1_001,
        }),
        characters: vec![
            character("ch1", "Exile", Some("Standard")),
            character("ch2", "Nowhere", None),
        ],
        items,
        unreadable_buyouts: Vec::new(),
        buyouts: vec![
            row("item", "i-hand", exact("5", "chaos")),
            row("item", "i-hand2", exact("5", "chaos")),
            row(
                "item",
                "i-bo",
                json!({ "version": 1, "type": "negotiable", "amount": "1.5", "currency": "divine" }),
            ),
            row("item", "i-skip", json!({ "version": 1, "type": "skip" })),
            row("item", "i-noted", exact("1", "divine")),
            row("item", "i-gem", exact("1", "chaos")),
            row("item", "i-ratio", exact("1/5", "divine")),
            row("item", "i-retired", exact("1", "silver")),
            row(
                "item",
                "i-noprice",
                json!({ "version": 1, "type": "no_price" }),
            ),
            row("item", "i-gameskip", exact("2", "chaos")),
            row(
                "tab",
                "pc/t2",
                json!({ "version": 1, "type": "negotiable", "amount": "2", "currency": "divine" }),
            ),
            row(
                "item",
                "i-unres",
                json!({ "version": 9, "type": "auction" }),
            ),
            row("item", "i-sub", exact("1", "chaos")),
            row("item", "i-unlisted", exact("1", "chaos")),
            row("item", "i-worn", exact("10", "divine")),
            row("item", "i-worngem", exact("1", "chaos")),
            row("item", "i-nowhere", exact("1", "chaos")),
            row(
                "item",
                "i-nowhere-skip",
                json!({ "version": 1, "type": "skip" }),
            ),
        ],
    }
}

fn report() -> ListingReport {
    resolve(&snapshot()).unwrap()
}

fn opts<'a>(policy: PolicySource<'a>) -> RenderOptions<'a> {
    RenderOptions {
        size: DEFAULT_PAGE_SIZE,
        template: ITEMS_TOKEN,
        now: 5_000,
        policy,
        expand: false,
    }
}

fn by_cell(r: &ShopRender, cell: Cell) -> Vec<String> {
    let mut out: Vec<String> = r
        .posted
        .iter()
        .filter(|p| p.cell == cell)
        .map(|p| p.target.to_string())
        .chain(
            r.left_out
                .iter()
                .filter(|l| l.cell == cell)
                .map(|l| l.target.to_string()),
        )
        .collect();
    out.sort();
    out
}

/// C74 — every item lands in exactly one cell of the table, each
/// cell's count is its items, and the four verdicts sum to the item
/// count: nothing is skipped silently.
#[test]
fn c74_every_item_is_posted_omitted_blocked_or_off_the_page_and_counted() {
    let r = render(
        &report(),
        &RenderOptions {
            expand: true,
            ..opts(PolicySource::NotSet)
        },
    )
    .unwrap();
    let c = &r.counts;
    assert_eq!(c.items, 21);
    assert_eq!(c.posted + c.omitted + c.blocked + c.off_page, c.items);
    assert_eq!(c.posted, 6, "{:?}", r.posted);
    assert_eq!(r.posted.len(), c.posted);
    assert_eq!(r.left_out.len(), c.items - c.posted);
    assert_eq!(c.by_cell.values().sum::<usize>(), c.items);
    assert_eq!(c.by_cell.len(), Cell::ALL.len());
    for row in &r.policy {
        assert_eq!(row.count, c.by_cell[&row.cell], "{}", row.cell);
        assert_eq!(row.verdict, row.cell.verdict());
        assert_eq!(row.why, row.cell.why());
    }
    let expect = |cell: Cell, targets: &[&str]| {
        let mut want: Vec<String> = targets.iter().map(|t| format!("item/{t}")).collect();
        want.sort();
        assert_eq!(by_cell(&r, cell), want, "{cell}");
    };
    expect(Cell::StashItem, &["i-hand", "i-hand2", "i-bo", "i-tabrow"]);
    expect(Cell::CharacterItem, &["i-worn"]);
    expect(Cell::GameLists, &["i-game", "i-noted"]);
    expect(Cell::GameSkips, &["i-gameskip"]);
    // A league-less character's items read their effective outcome
    // first: only a hand price reaches the league cell.
    expect(Cell::HandSkip, &["i-skip", "i-nowhere-skip"]);
    expect(Cell::NothingApplies, &["i-plain", "i-nowhere-plain"]);
    expect(Cell::Unresolved, &["i-unres"]);
    expect(Cell::LeagueUnknown, &["i-nowhere"]);
    // Q5 answered (T21): a no-price item posts as its link alone.
    expect(Cell::HandNoPrice, &["i-noprice"]);
    expect(Cell::Ratio, &["i-ratio"]);
    expect(Cell::RetiredCurrency, &["i-retired"]);
    expect(Cell::Socketed, &["i-gem", "i-worngem"]);
    expect(Cell::Substash, &["i-sub"]);
    expect(Cell::TabUnlisted, &["i-unlisted"]);
    expect(Cell::RealmUnlisted, &[]);
    expect(Cell::NoPosition, &[]);
    expect(Cell::NoSlot, &[]);
    expect(Cell::PageSize, &[]);
    expect(Cell::InvalidIndex, &[]);
    expect(Cell::UnruledKind, &[]);

    // A poe2 report: nothing the site lists (T4).
    let mut poe2 = snapshot();
    poe2.realm = "poe2".into();
    poe2.tabs.clear();
    poe2.items.retain(|i| i.location_kind == "character");
    let r = render(&resolve(&poe2).unwrap(), &opts(PolicySource::NotSet)).unwrap();
    assert_eq!(r.counts.posted, 0);
    // The worn item and its gem: the realm is read before the address.
    assert_eq!(r.counts.by_cell[&Cell::RealmUnlisted], 2);
    assert_eq!(r.counts.by_cell[&Cell::LeagueUnknown], 1);
    assert_eq!(r.counts.by_cell[&Cell::HandSkip], 1);
    assert_eq!(r.counts.by_cell[&Cell::NothingApplies], 1);

    // C53, the read economy (step 2g): the default document's lists
    // hold the bound, and every item is still counted, each list's
    // whole beside it
    let whole = render(
        &report(),
        &RenderOptions {
            expand: true,
            ..opts(PolicySource::NotSet)
        },
    )
    .unwrap();
    let r = render(&report(), &opts(PolicySource::NotSet)).unwrap();
    let c = &r.counts;
    assert_eq!(c, &whole.counts);
    assert_eq!(c.posted + c.omitted + c.blocked + c.off_page, c.items);
    assert_eq!(r.posted_total, c.posted);
    assert_eq!(
        r.posted.len(),
        c.posted.min(acquisition_store::bound::LISTED)
    );
    assert_eq!(r.left_out_total, c.omitted + c.blocked);
    assert_eq!(
        r.left_out.len(),
        r.left_out_total.min(acquisition_store::bound::LISTED)
    );
    assert_eq!(whole.posted_total, whole.posted.len());
}

/// C47, C74 — a store row never panics the render and a vocabulary
/// the table has no row for fails closed: a corrupt timestamp
/// saturates the age, a negative index is `invalid_index`,
/// a hand-price kind the table does not write is `unruled_kind`, and
/// a realm the site does not list is `realm_unlisted`.
#[test]
fn c47_corrupt_facts_and_unruled_words_are_cells_never_panics() {
    let mut snap = snapshot();
    for i in &mut snap.items {
        if i.id == "i-hand" {
            i.last_seen = i64::MIN;
        }
    }
    for t in &mut snap.tabs {
        if t.id == "c1" {
            t.idx = Some(-1);
        }
        if t.id == "t2" {
            // Not a corrupt fact any more: a rank never adds to it.
            t.idx = Some(i64::MAX);
        }
    }
    let policy = SyncPolicy::from_value(&json!({
        "version": 3,
        "realms": { "pc": { "leagues": { "Standard": {
            "tabs": "all", "characters": "all", "max_age_seconds": 3600 } } } }
    }))
    .unwrap();
    let r = render(
        &resolve(&snap).unwrap(),
        &opts(PolicySource::Set {
            policy: &policy,
            revision: 1,
            refresh: Err("no such league".into()),
        }),
    )
    .unwrap();
    // c1's four hand-priced items are blocked; t2's item, at the
    // largest index, is simply the last-ranked tab: after t3 and m1
    // (the folder and the corrupt tab take no number) it is Stash3.
    assert_eq!(r.counts.by_cell[&Cell::InvalidIndex], 4);
    assert_eq!(r.counts.posted, 2);
    assert!(
        r.posted
            .iter()
            .any(|p| p.target.to_string() == "item/i-tabrow" && p.link.contains("Stash3")),
        "{:?}",
        r.posted
    );
    assert_eq!(
        r.freshness.refresh_problem.as_deref(),
        Some("no such league")
    );
    assert_eq!(r.freshness.refresh_requests, None);

    // The corrupt timestamp on a posted item: the age saturates.
    let mut snap = snapshot();
    for i in &mut snap.items {
        if i.id == "i-hand" {
            i.last_seen = i64::MIN;
        }
    }
    let r = render(
        &resolve(&snap).unwrap(),
        &opts(PolicySource::Set {
            policy: &policy,
            revision: 1,
            refresh: Ok(3),
        }),
    )
    .unwrap();
    assert_eq!(r.freshness.oldest_stale_seconds, Some(i64::MAX));
    assert!(
        r.freshness
            .stale
            .iter()
            .any(|t| t.to_string() == "item/i-hand")
    );

    // A kind the table does not write: the report is edited in place
    // (no value schema carries one yet), and the cell blocks it.
    let mut report = report();
    for l in &mut report.listings {
        if l.subject.target.to_string() == "item/i-hand" {
            l.effective.kind = "current_offer".into();
        }
    }
    let r = render(&report, &opts(PolicySource::NotSet)).unwrap();
    assert_eq!(by_cell(&r, Cell::UnruledKind), ["item/i-hand"]);
    assert_eq!(r.counts.posted, 5);
}

/// C74, T7, T13, T24 — the two link shapes in the website's attribute
/// order: a stash item by its tab's rank among the tabs the website
/// lists (the folder at index 0 takes no number, so c1 at index 1 is
/// `Stash1` and t2 at index 2 is `Stash2`), a character item by its
/// slot and the character's name; `~price` and `~b/o` as spoiler
/// titles with the table's word; one spoiler per price.
#[test]
fn c74_the_link_codes_and_spoiler_titles_are_the_observed_shapes() {
    let r = render(&report(), &opts(PolicySource::NotSet)).unwrap();
    let find = |id: &str| {
        r.posted
            .iter()
            .find(|p| p.target.to_string() == format!("item/{id}"))
            .unwrap()
    };
    let hand = find("i-hand");
    assert_eq!(
        hand.link,
        "[linkItem realm=\"pc\" location=\"Stash1\" league=\"Standard\" x=\"1\" y=\"0\"]"
    );
    assert_eq!(hand.title, " ~price 5 chaos");
    let worn = find("i-worn");
    assert_eq!(
        worn.link,
        "[linkItem realm=\"pc\" location=\"BodyArmour\" character=\"Exile\" x=\"0\" y=\"0\"]"
    );
    assert_eq!(worn.title, " ~price 10 divine");
    assert_eq!(find("i-bo").title, " ~b/o 1.5 divine");
    assert_eq!(find("i-tabrow").title, " ~b/o 2 divine");
    assert_eq!(find("i-noprice").title, "");
    assert_eq!(r.pages.len(), 1);
    let page = &r.pages[0];
    // The owner's shape (2026-09-07): one spoiler per price with the
    // links run together, one page spoiler labelled n of N, newlines
    // only between spoiler tags; no price last, under an empty title.
    assert_eq!(
        page.text,
        "[spoiler=\"Shop Post 1 of 1 (6 items)\"]\n\
             [spoiler=\" ~price 5 chaos\"][linkItem realm=\"pc\" location=\"Stash1\" league=\"Standard\" x=\"1\" y=\"0\"][linkItem realm=\"pc\" location=\"Stash1\" league=\"Standard\" x=\"2\" y=\"0\"][/spoiler]\n\
             [spoiler=\" ~price 10 divine\"][linkItem realm=\"pc\" location=\"BodyArmour\" character=\"Exile\" x=\"0\" y=\"0\"][/spoiler]\n\
             [spoiler=\" ~b/o 1.5 divine\"][linkItem realm=\"pc\" location=\"Stash1\" league=\"Standard\" x=\"3\" y=\"0\"][/spoiler]\n\
             [spoiler=\" ~b/o 2 divine\"][linkItem realm=\"pc\" location=\"Stash2\" league=\"Standard\" x=\"0\" y=\"1\"][/spoiler]\n\
             [spoiler=\"\"][linkItem realm=\"pc\" location=\"Stash1\" league=\"Standard\" x=\"8\" y=\"0\"][/spoiler]\n\
             [/spoiler]\n"
    );
    assert_eq!(page.chars, page.text.chars().count());
    assert_eq!(page.items, 6);
    assert!(r.posted.iter().all(|p| p.page == 1));
    // C146 — the page names each `Stash<n>` its links write by the
    // tab's own name, with its links counted; a character's link
    // names its character already.
    let report = report();
    let tabs: Vec<(&str, usize)> = page
        .tabs
        .iter()
        .map(|t| (t.stash.as_str(), t.items))
        .collect();
    assert_eq!(tabs, [("Stash1", 4), ("Stash2", 1)]);
    for t in &page.tabs {
        let named = report.find(&t.tab).map(|l| l.subject.name.as_str());
        assert_eq!(Some(t.name.as_str()), named, "{}", t.stash);
    }
}

/// C74 — pages are cut under the size with the template counted, a
/// group runs on across the cut, every page is labelled n of N, the
/// template wraps each page, an entry too large for an empty page is
/// blocked, and a template without the token refuses.
#[test]
fn c74_pages_are_cut_under_the_size_and_carry_the_template() {
    let report = report();
    let template = "Shop\n[items]Thanks\n";
    let overhead = template.chars().count() - ITEMS_TOKEN.len();
    let full = render(&report, &opts(PolicySource::NotSet)).unwrap();
    let link = |id: &str| {
        full.posted
            .iter()
            .find(|p| p.target.to_string() == format!("item/{id}"))
            .unwrap()
            .link
            .chars()
            .count()
    };
    // Exactly the first group's two links fit the first page with
    // the page spoiler reserved at its widest; the next group does
    // not, so the cut falls between spoilers.
    let size = page_fixed_chars(6, overhead)
        + group_open(" ~price 5 chaos").chars().count()
        + GROUP_CLOSE.len()
        + link("i-hand")
        + link("i-hand2");
    let r = render(
        &report,
        &RenderOptions {
            size,
            template,
            now: 5_000,
            policy: PolicySource::NotSet,
            expand: false,
        },
    )
    .unwrap();
    assert_eq!(r.counts.posted, 6);
    assert_eq!(r.counts.blocked, 8);
    assert_eq!(r.counts.omitted, 5);
    assert_eq!(r.counts.off_page, 2);
    assert!(r.pages.len() >= 3, "{:#?}", r.pages);
    assert_eq!(r.counts.pages, r.pages.len());
    for page in &r.pages {
        assert!(
            page.chars <= size,
            "page {} is {} > {size}",
            page.number,
            page.chars
        );
        assert_eq!(page.of, r.pages.len());
        assert!(
            page.text.starts_with(&format!(
                "Shop\n[spoiler=\"Shop Post {} of {} ({} {})\"]\n[spoiler=\"",
                page.number,
                page.of,
                page.items,
                if page.items == 1 { "item" } else { "items" }
            )),
            "{}",
            page.text
        );
        assert!(
            page.text.ends_with("[/spoiler]\n[/spoiler]\nThanks\n"),
            "{}",
            page.text
        );
        assert_eq!(
            r.posted.iter().filter(|p| p.page == page.number).count(),
            page.items
        );
    }
    assert_eq!(r.pages[0].items, 2);
    assert_eq!(r.pages.iter().map(|p| p.items).sum::<usize>(), 6);
    // Page numbers follow the group order.
    assert!(r.posted.windows(2).all(|w| w[0].page <= w[1].page));

    // A group cut in two is closed on one page and reopened on the
    // next: one link of the first group per page.
    let r = render(
        &report,
        &RenderOptions {
            size: size - link("i-hand2"),
            template,
            now: 5_000,
            policy: PolicySource::NotSet,
            expand: false,
        },
    )
    .unwrap();
    assert_eq!(r.pages[0].items, 1);
    assert_eq!(r.pages[1].items, 1);
    assert!(r.pages[0].text.contains("[spoiler=\" ~price 5 chaos\"]"));
    assert!(r.pages[1].text.contains("[spoiler=\" ~price 5 chaos\"]"));

    // Below one entry: every candidate is blocked as page_size and
    // there are no pages.
    let r = render(
        &report,
        &RenderOptions {
            size: overhead + 10,
            template,
            now: 5_000,
            policy: PolicySource::NotSet,
            expand: false,
        },
    )
    .unwrap();
    assert_eq!(r.counts.posted, 0);
    assert_eq!(r.counts.by_cell[&Cell::PageSize], 6);
    assert!(r.pages.is_empty());
    assert_eq!(r.counts.blocked, 14);

    for (template, want) in [
        ("no token", RenderError::Template { occurrences: 0 }),
        ("[items] [items]", RenderError::Template { occurrences: 2 }),
    ] {
        let err = render(
            &report,
            &RenderOptions {
                size: 100,
                template,
                now: 5_000,
                policy: PolicySource::NotSet,
                expand: false,
            },
        )
        .unwrap_err();
        assert_eq!(err, want);
    }
    let err = render(
        &report,
        &RenderOptions {
            size: 3,
            template,
            now: 5_000,
            policy: PolicySource::NotSet,
            expand: false,
        },
    )
    .unwrap_err();
    assert!(matches!(err, RenderError::TemplateTooLarge { .. }), "{err}");
}

/// C72 — coverage, staleness and positions are reported over the
/// posted items, never enforced: containers outside the policy's
/// selection are named (a tab through its folder counts as covered,
/// C37), facts older than the window are named with the refresh's
/// request count, a stash item whose fetch predates the listing is
/// named; an absent, unreadable or non-covering policy says which.
/// C53 (an age the text prints stands in words beside its epoch,
/// made at the answer's own time): the oldest stale item's age, the
/// span the render measured at its own time, stands in the words the
/// text's "oldest" says, beside the seconds.
#[test]
fn c53_the_oldest_stale_age_stands_in_words_beside_its_seconds() {
    let policy = SyncPolicy::from_value(&json!({
        "version": 3,
        "realms": { "pc": { "leagues": { "Standard": {
            "tabs": ["f1"], "characters": ["ch1"], "max_age_seconds": 3600 } } } }
    }))
    .unwrap();
    let r = render(
        &report(),
        &opts(PolicySource::Set {
            policy: &policy,
            revision: 4,
            refresh: Ok(7),
        }),
    )
    .unwrap();
    let f = &r.freshness;
    assert_eq!(f.oldest_stale_seconds, Some(3_800));
    assert_eq!(f.oldest_stale_seconds_age.as_deref(), Some("63m ago"));
    let none = render(&report(), &opts(PolicySource::NotSet)).unwrap();
    assert_eq!(none.freshness.oldest_stale_seconds, None);
    assert!(none.freshness.oldest_stale_seconds_age.is_none());
}

#[test]
fn c72_coverage_staleness_and_positions_are_reported_never_enforced() {
    let report = report();
    let policy = SyncPolicy::from_value(&json!({
        "version": 3,
        "realms": { "pc": { "leagues": { "Standard": {
            "tabs": ["f1"], "characters": ["ch1"], "max_age_seconds": 3600 } } } }
    }))
    .unwrap();
    let r = render(
        &report,
        &opts(PolicySource::Set {
            policy: &policy,
            revision: 4,
            refresh: Ok(7),
        }),
    )
    .unwrap();
    let f = &r.freshness;
    assert_eq!(f.policy, "covers");
    assert_eq!(f.policy_revision, Some(4));
    assert_eq!(f.window_seconds, Some(3600));
    assert_eq!(f.refresh_requests, Some(7));
    // c1 is covered through its folder f1; t2 is not named.
    assert_eq!(
        f.uncovered
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["tab/pc/t2"]
    );
    // now 5000: the c1 items and the worn item were seen at 1200
    // (3800 s ago), over the window, in covered containers — the
    // cited refresh fetches them; the t2 item (seen at 100, 4900 s
    // ago) is past the window too, but its container is uncovered,
    // so it is counted beside the coverage line, not as stale — and
    // the oldest age is the stale set's, never the uncovered item's.
    assert_eq!(f.stale.len(), 5);
    assert!(!f.stale.iter().any(|t| t.to_string() == "item/i-tabrow"));
    assert_eq!(f.stale_uncovered, 1);
    assert_eq!(f.oldest_stale_seconds, Some(3_800));
    // The t2 item's fetch (response 1) predates the listing (2).
    assert_eq!(
        f.position_before_listing
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["item/i-tabrow"]
    );

    // A wide window and full coverage: nothing to report.
    let policy = SyncPolicy::from_value(&json!({
        "version": 3,
        "realms": { "pc": { "leagues": { "Standard": {
            "tabs": "all", "characters": "all", "max_age_seconds": 10000 } } } }
    }))
    .unwrap();
    let r = render(
        &report,
        &opts(PolicySource::Set {
            policy: &policy,
            revision: 5,
            refresh: Err("nothing to plan".into()),
        }),
    )
    .unwrap();
    assert!(r.freshness.uncovered.is_empty());
    assert!(r.freshness.stale.is_empty());
    assert_eq!(r.freshness.stale_uncovered, 0);
    assert_eq!(r.freshness.refresh_requests, None);
    assert_eq!(
        r.freshness.refresh_problem.as_deref(),
        Some("nothing to plan")
    );

    // The policy covers another league only.
    let policy = SyncPolicy::from_value(&json!({
        "version": 3,
        "realms": { "pc": { "leagues": { "Hardcore": {
            "tabs": "all", "max_age_seconds": 10 } } } }
    }))
    .unwrap();
    let r = render(
        &report,
        &opts(PolicySource::Set {
            policy: &policy,
            revision: 6,
            refresh: Ok(0),
        }),
    )
    .unwrap();
    assert_eq!(r.freshness.policy, "league_not_covered");
    assert_eq!(r.freshness.window_seconds, None);
    assert!(r.freshness.uncovered.is_empty());

    let r = render(
        &report,
        &opts(PolicySource::Unreadable {
            revision: 9,
            why: "version 12".into(),
        }),
    )
    .unwrap();
    assert_eq!(r.freshness.policy, "unreadable");
    assert_eq!(r.freshness.policy_problem.as_deref(), Some("version 12"));
    let r = render(&report, &opts(PolicySource::NotSet)).unwrap();
    assert_eq!(r.freshness.policy, "not_set");
    // The position line needs no policy.
    assert_eq!(r.freshness.position_before_listing.len(), 1);
}

/// T24 — `Stash<n>` against the website's own numbering, at scale: the
/// owner's two leagues as the site listed them on 2026-09-07 beside
/// the API's listings (`reference/website-tabs-2026-09-07.json`).
/// Standard has 16 folders interleaved among 402 tabs, 64 substashes
/// and 274 remove-only tabs; every tab the site lists ranks exactly at
/// its site number plus one, and no folder or substash has a number.
#[test]
fn t24_stash_numbers_match_the_websites_own_list() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/reference/website-tabs-2026-09-07.json"
    );
    let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    for (league, lists) in doc["leagues"].as_object().unwrap() {
        let tabs: Vec<TabSnapshot> = lists["api"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| TabSnapshot {
                id: t["id"].as_str().unwrap().into(),
                parent: t["parent"].as_str().map(str::to_string),
                name: String::new(),
                r#type: t["type"].as_str().unwrap().into(),
                idx: t["index"].as_i64(),
                listed_at: Some(1_000),
                listed_response: Some(2),
                fetched_at: None,
                metadata: Value::Null,
                item_count: 0,
            })
            .collect();
        let folders: usize = tabs.iter().filter(|t| t.r#type == FOLDER).count();
        let substashes: usize = tabs
            .iter()
            .filter(|t| {
                t.parent
                    .as_deref()
                    .is_some_and(|p| tabs.iter().any(|q| q.id == p && q.r#type != FOLDER))
            })
            .count();
        let mut snap = snapshot();
        snap.league = league.clone();
        snap.tabs = tabs;
        snap.items.clear();
        snap.buyouts.clear();
        let numbers = stash_numbers(&resolve(&snap).unwrap());
        let web = lists["web"].as_array().unwrap();
        assert_eq!(
            numbers.len(),
            web.len(),
            "{league}: {} numbered, {} on the site ({folders} folders, {substashes} substashes)",
            numbers.len(),
            web.len()
        );
        for t in web {
            let site_id = t["id"].as_str().unwrap();
            let api_id: String = site_id.chars().take(10).collect();
            let want = t["i"].as_u64().unwrap() as usize + 1;
            assert_eq!(
                numbers.get(&api_id),
                Some(&StashNumber::Rank(want)),
                "{league}: {} ({}) at site number {}",
                t["n"],
                api_id,
                t["i"]
            );
        }
        for t in &snap.tabs {
            if t.r#type == FOLDER
                || t.parent
                    .as_deref()
                    .is_some_and(|p| snap.tabs.iter().any(|q| q.id == p && q.r#type != FOLDER))
            {
                assert!(
                    !numbers.contains_key(&t.id),
                    "{league}: {} has a number",
                    t.id
                );
            }
        }
    }
}

/// C53 — the JSON shape is the contract: the rendered fixture must
/// serialize to exactly the committed document. A difference is
/// either additive (regenerate with `ACQ_UPDATE_FIXTURES=1`) or a
/// schema bump (`SHOP_SCHEMA`), decided by reading the diff.
#[test]
fn the_render_json_matches_the_committed_fixture() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/reference/shop-render-schema-5.json"
    );
    let policy = SyncPolicy::from_value(&json!({
        "version": 3,
        "realms": { "pc": { "leagues": { "Standard": {
            "tabs": ["f1"], "characters": ["ch1"], "max_age_seconds": 3600 } } } }
    }))
    .unwrap();
    let r = render(
        &report(),
        &opts(PolicySource::Set {
            policy: &policy,
            revision: 4,
            refresh: Ok(7),
        }),
    )
    .unwrap();
    let actual = serde_json::to_value(&r).unwrap();
    if std::env::var_os("ACQ_UPDATE_FIXTURES").is_some() {
        std::fs::write(path, serde_json::to_string_pretty(&actual).unwrap() + "\n").unwrap();
    }
    let expected: Value = serde_json::from_str(
        &std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}")),
    )
    .unwrap();
    assert_eq!(expected["schema"], json!(SHOP_SCHEMA));
    assert_eq!(
        actual, expected,
        "the render's JSON changed; additive → ACQ_UPDATE_FIXTURES=1, else bump SHOP_SCHEMA"
    );
    let back: ShopRender = serde_json::from_value(expected).unwrap();
    assert_eq!(back, r);
}

/// The plan step 7 property tests (`PRICING-SLICE.md` at `b6197a82`): the page
/// cutter over any entries, template and size.
mod properties {
    use super::*;
    use proptest::prelude::*;

    /// An entry's parts: its group order (`~price`, `~b/o`, no price),
    /// tag, whole amount, and the text that makes its link.
    fn entry_parts() -> impl Strategy<Value = (u8, &'static str, u64, String)> {
        (
            0u8..3,
            prop_oneof![Just("chaos"), Just("divine"), Just("exalted")],
            1u64..20,
            "[a-z ]{0,30}",
        )
    }

    /// The entry as [`cell`] would build it: the title a function of
    /// the group, the link unique to the entry.
    fn entry(i: usize, (order, tag, amount, text): (u8, &'static str, u64, String)) -> Entry {
        let amount = Amount::Decimal {
            ten_thousandths: amount * 10_000,
        };
        let (group, title) = if order == 2 {
            ((2, String::new(), None), String::new())
        } else {
            let prefix = if order == 0 { " ~price" } else { " ~b/o" };
            (
                (order, tag.to_string(), Some(amount)),
                format!("{prefix} {amount} {tag}"),
            )
        };
        Entry {
            target: PriceTarget::Item {
                id: format!("i{i}"),
            },
            label: format!("item {i}"),
            location: PriceTarget::Tab {
                realm: Realm::Pc,
                id: "t".into(),
            },
            cell: if order == 2 {
                Cell::HandNoPrice
            } else {
                Cell::StashItem
            },
            group,
            link: format!("[link #{i}# {text}]"),
            stash: None,
            title,
            seen_response: None,
            seen_at: None,
            parent: None,
        }
    }

    proptest! {
        /// C74 — for any entries, any template and any size: every
        /// candidate is on a page or set aside, never both; every page
        /// is within the size, the template around it, numbered n of N
        /// and holding at least one item; every posted item's link is on
        /// its page exactly once and on no other; groups are sorted
        /// across the run and contiguous on a page — no group reopens
        /// on the page that closed it, and a page's spoilers are its
        /// groups plus its own.
        #[test]
        fn c74_any_entries_cut_into_pages_within_the_size_each_posted_once_in_contiguous_groups(
            parts in prop::collection::vec(entry_parts(), 0..40),
            before in "[^\\[]{0,30}",
            after in "[^\\[]{0,30}",
            size in 0usize..1200,
        ) {
            let template = format!("{before}{ITEMS_TOKEN}{after}");
            let candidates: Vec<Entry> = parts.into_iter().enumerate().map(|(i, p)| entry(i, p)).collect();
            let n = candidates.len();
            let cut = cut_pages(candidates, &template, size);

            prop_assert_eq!(cut.entries.len() + cut.too_large.len(), n);
            prop_assert_eq!(cut.posted.len(), cut.entries.len());
            let mut targets: Vec<&PriceTarget> = cut
                .entries
                .iter()
                .chain(&cut.too_large)
                .map(|e| &e.target)
                .collect();
            targets.sort();
            targets.dedup();
            prop_assert_eq!(targets.len(), n, "an entry landed twice or not at all");

            let of = cut.pages.len();
            prop_assert_eq!(cut.pages.iter().map(|p| p.items).sum::<usize>(), cut.posted.len());
            for (i, page) in cut.pages.iter().enumerate() {
                prop_assert_eq!(page.number, i + 1);
                prop_assert_eq!(page.of, of);
                prop_assert!(page.items > 0);
                prop_assert_eq!(page.chars, page.text.chars().count());
                prop_assert!(page.chars <= size, "page {} holds {} chars over {}:\n{}", page.number, page.chars, size, page.text);
                prop_assert!(page.text.starts_with(&before) && page.text.ends_with(&after));
                prop_assert_eq!(cut.posted.iter().filter(|p| p.page == page.number).count(), page.items);
            }

            for (e, p) in cut.entries.iter().zip(&cut.posted) {
                prop_assert_eq!(&e.target, &p.target);
                prop_assert_eq!(&e.link, &p.link);
                prop_assert!((1..=of).contains(&p.page));
                for page in &cut.pages {
                    prop_assert_eq!(
                        page.text.matches(&e.link).count(),
                        usize::from(page.number == p.page),
                        "{} on page {}", e.link, page.number
                    );
                }
            }

            prop_assert!(cut.entries.windows(2).all(|w| w[0].group <= w[1].group), "groups out of order");
            prop_assert!(cut.posted.windows(2).all(|w| w[0].page <= w[1].page), "pages out of order");
            for page in &cut.pages {
                let mut keys: Vec<&(u8, String, Option<Amount>)> = cut
                    .entries
                    .iter()
                    .zip(&cut.posted)
                    .filter(|(_, p)| p.page == page.number)
                    .map(|(e, _)| &e.group)
                    .collect();
                keys.dedup();
                let mut distinct = keys.clone();
                distinct.sort();
                distinct.dedup();
                prop_assert_eq!(keys.len(), distinct.len(), "a group reopened on page {}", page.number);
                prop_assert_eq!(page.text.matches("[spoiler=").count(), keys.len() + 1, "{}", page.text);
                prop_assert_eq!(page.text.matches("[/spoiler]").count(), keys.len() + 1, "{}", page.text);
            }
        }
    }
}

/// The real-scale fixture (plan step 7, item 2, `PRICING-SLICE.md` at `b6197a82`):
/// the owner's pc/Standard league, rendered whole — the page the
/// owner pasted for validation reading 2, from the same facts.
mod real_scale {
    use std::time::Instant;

    use super::*;
    use crate::real_scale_fixture;
    use acquisition_store::{PricingSnapshot, TabSnapshot};

    fn fixture() -> (PricingSnapshot, ListingReport) {
        let s = real_scale_fixture::snapshot();
        let r = resolve(&s).unwrap();
        (s, r)
    }

    fn opts_at(now: i64, policy: PolicySource<'_>) -> RenderOptions<'_> {
        RenderOptions {
            size: DEFAULT_PAGE_SIZE,
            template: ITEMS_TOKEN,
            now,
            policy,
            expand: false,
        }
    }

    fn tab_id(target: &PriceTarget) -> &str {
        let PriceTarget::Tab { id, .. } = target else {
            panic!("{target} is not a tab");
        };
        id
    }

    /// C74, C81 — every one of the 1,977 items lands in one cell and
    /// the cells are read off the listing state: the 48 game prices
    /// and the one `~skip` are omitted, the character's 3 hand skips
    /// too, the substash row's 5 items are blocked on Q3, 1,889 have
    /// nothing applying, and the 31 the hand decides are the page:
    /// one page, under the size, every link once, under three titles.
    /// The render's time is printed, never asserted (C137): under 1 ms
    /// in a debug build on 2026-09-07.
    #[test]
    fn c74_at_real_scale_every_item_lands_in_one_cell_and_the_page_is_the_owners_test_tab() {
        let (s, report) = fixture();
        let t = Instant::now();
        let r = render(&report, &opts_at(s.taken_at, PolicySource::NotSet)).unwrap();
        let took = t.elapsed();
        eprintln!("render took {took:?}");
        assert_eq!(r.schema, SHOP_SCHEMA);
        assert_eq!(r.listing, report.header);

        let c = &r.counts;
        assert_eq!(c.items, 1977);
        assert_eq!(
            (c.posted, c.omitted, c.blocked, c.off_page, c.pages),
            (31, 52, 5, 1889, 1)
        );
        assert_eq!(c.posted + c.omitted + c.blocked + c.off_page, c.items);
        assert_eq!(r.posted_total + r.left_out_total + c.off_page, c.items);
        assert_eq!(c.by_cell.values().sum::<usize>(), c.items);
        let expected: BTreeMap<Cell, usize> = Cell::ALL
            .iter()
            .map(|cell| {
                let n = match cell {
                    Cell::GameLists => 48,
                    Cell::GameSkips => 1,
                    Cell::HandSkip => 3,
                    Cell::Substash => 5,
                    Cell::StashItem => 31,
                    Cell::NothingApplies => 1889,
                    _ => 0,
                };
                (*cell, n)
            })
            .collect();
        assert_eq!(c.by_cell, expected);
        for row in &r.policy {
            assert_eq!(row.count, c.by_cell[&row.cell], "{}", row.cell);
        }
        assert_eq!(c.by_cell[&Cell::GameLists], report.counts.game_priced);
        assert_eq!(
            c.by_cell[&Cell::GameSkips],
            report.counts.by_game_statement["skip"]
        );
        assert_eq!(
            c.by_cell[&Cell::NothingApplies],
            report.counts.by_relation["none"]
        );
        assert!(
            r.left_out
                .iter()
                .filter(|l| l.cell == Cell::HandSkip)
                .all(|l| matches!(l.location, Some(PriceTarget::Character { .. })))
        );
        assert!(
            r.left_out
                .iter()
                .filter(|l| l.cell == Cell::Substash)
                .all(|l| matches!(l.location, Some(PriceTarget::Substash { .. })))
        );

        // every posted item, past the default's ten (C53): the whole
        // list, under `expand`, beside the same pages
        let whole = render(
            &report,
            &RenderOptions {
                expand: true,
                ..opts_at(s.taken_at, PolicySource::NotSet)
            },
        )
        .unwrap();
        assert_eq!(whole.pages, r.pages);
        assert_eq!(whole.posted.len(), r.posted_total);
        let r = whole;
        let tab = real_scale_fixture::owners_test_tab(&s);
        assert!(
            r.posted
                .iter()
                .all(|p| p.location == tab && p.page == 1 && p.cell == Cell::StashItem)
        );
        let mut titles: BTreeMap<&str, usize> = BTreeMap::new();
        for p in &r.posted {
            *titles.entry(p.title.as_str()).or_default() += 1;
        }
        assert_eq!(
            titles,
            [
                (" ~b/o 1.5 divine", 1),
                (" ~price 2 chaos", 1),
                (" ~price 5 chaos", 29)
            ]
            .into()
        );
        let [page] = &r.pages[..] else {
            panic!("one page");
        };
        assert_eq!((page.number, page.of, page.items), (1, 1, 31));
        assert!(page.chars <= DEFAULT_PAGE_SIZE);
        assert_eq!(page.chars, page.text.chars().count());
        assert_eq!(page.text.matches("[spoiler=").count(), 1 + titles.len());
        assert!(
            page.text
                .starts_with("[spoiler=\"Shop Post 1 of 1 (31 items)\"]\n")
        );
        for p in &r.posted {
            assert_eq!(page.text.matches(&p.link).count(), 1, "{}", p.link);
            assert!(
                page.text.contains(&format!("[spoiler=\"{}\"]", p.title)),
                "{}",
                p.title
            );
        }

        let text = serde_json::to_string(&r).unwrap();
        let back: ShopRender = serde_json::from_str(&text).unwrap();
        assert_eq!(back, r);
    }

    /// C74, C53 — the release seat's shop document: 5 MB on the MCP
    /// and 7.5 MB as `--json`, because `left_out` listed every item
    /// nothing applies to (20,749 of the owner's 21,311). Over the
    /// owner's league here: the items left off for a reason — the game
    /// lists it, skipped by hand, blocked — are listed, every one; the
    /// 1,889 nothing applies to are counted and listed only when
    /// asked, and the document says which it did (listed to a bound since
    /// the rerun: the test below).
    #[test]
    fn the_shop_document_lists_what_is_left_off_for_a_reason_and_counts_what_nothing_applies_to() {
        let (s, report) = fixture();
        let r = render(&report, &opts_at(s.taken_at, PolicySource::NotSet)).unwrap();
        let c = &r.counts;
        assert_eq!(c.off_page, 1889);
        assert!(
            r.left_out.iter().all(|l| l.verdict != Verdict::OffPage),
            "the default document lists what nothing applies to"
        );
        assert_eq!(r.left_out_total, c.omitted + c.blocked);
        assert!(!r.off_page_listed);
        let bytes = serde_json::to_string_pretty(&r).unwrap().len();
        eprintln!("the default shop document over the owner's league: {bytes} bytes");
        assert!(bytes < 64_000, "{bytes} bytes");

        // Asked, the same document lists them too, and nothing else moves.
        let all = render(
            &report,
            &RenderOptions {
                expand: true,
                ..opts_at(s.taken_at, PolicySource::NotSet)
            },
        )
        .unwrap();
        assert!(all.off_page_listed);
        assert_eq!(all.left_out.len(), c.items - c.posted);
        assert_eq!(
            all.left_out
                .iter()
                .filter(|l| l.verdict != Verdict::OffPage)
                .take(acquisition_store::bound::LISTED)
                .collect::<Vec<_>>(),
            r.left_out.iter().collect::<Vec<_>>()
        );
        assert_eq!(
            (&all.counts, &all.pages, &all.policy),
            (&r.counts, &r.pages, &r.policy)
        );
        // the posted list: the default is the whole one's head, counted
        assert_eq!(&all.posted[..r.posted.len()], &r.posted[..]);
        assert_eq!(all.posted.len(), r.posted_total);
        eprintln!(
            "expanded: {} bytes",
            serde_json::to_string_pretty(&all).unwrap().len()
        );
    }

    /// C53, C74 — the release seat's rerun (mcp-opus-2, calls 37, 38):
    /// the shop document was still 160 KB on the owner's league, 135 KB
    /// of it `left_out`'s 521 items left off for a reason, and `page`
    /// drops the counts. The default document lists the store's bound
    /// of them (`LISTED`, C53) — the blocked first, which a person
    /// can act on — with `left_out_total` counting them all and every
    /// cell's count intact; `expand` lists them all. Read through the
    /// JSON, the contract both surfaces print. Here 57 are left off for
    /// a reason (52 omitted, 5 blocked).
    #[test]
    fn the_default_shop_document_lists_a_bound_of_what_is_left_out_and_counts_it_all() {
        let (s, report) = fixture();
        let r = render(&report, &opts_at(s.taken_at, PolicySource::NotSet)).unwrap();
        let doc = serde_json::to_value(&r).unwrap();
        let listed = doc["left_out"].as_array().unwrap();
        assert_eq!(
            listed.len(),
            acquisition_store::bound::LISTED,
            "the default lists the bound"
        );
        assert_eq!(doc["left_out_total"], json!(57), "and counts them all");
        assert!(
            listed[..5].iter().all(|l| l["verdict"] == json!("block")),
            "the blocked are listed first"
        );
        assert_eq!(doc["counts"]["omitted"], json!(52));
        assert_eq!(doc["counts"]["blocked"], json!(5));
        assert_eq!(doc["counts"]["off_page"], json!(1889));
        let bytes = serde_json::to_string_pretty(&r).unwrap().len();
        eprintln!("the default shop document over the owner's league: {bytes} bytes");

        let all = serde_json::to_value(
            render(
                &report,
                &RenderOptions {
                    expand: true,
                    ..opts_at(s.taken_at, PolicySource::NotSet)
                },
            )
            .unwrap(),
        )
        .unwrap();
        let every = all["left_out"].as_array().unwrap();
        assert_eq!(every.len(), 1977 - 31, "expand lists every one");
        assert_eq!(all["left_out_total"], json!(1977 - 31));
        assert_eq!(
            &every[..acquisition_store::bound::LISTED],
            &listed[..],
            "the default is the expanded list's head"
        );
        assert_eq!(all["counts"], doc["counts"]);
    }

    /// T24 — the site as the oracle for the numbering, over the real
    /// posted items: the tabs the site lists (top-level and folder
    /// children; folders and substashes out) in the snapshot's order
    /// are the site's own list of the same day by name and type
    /// (`reference/website-tabs-2026-09-07.json`), and every posted
    /// link's `Stash<n>` is its tab's site number plus one — the
    /// owner's test tab is the site's 56, so `Stash57`, the page the
    /// owner read correct on the trade site.
    #[test]
    fn t24_at_real_scale_every_posted_link_numbers_its_tab_as_the_website_does() {
        let (s, report) = fixture();
        // every posted link: the whole list, past the default's ten
        let r = render(
            &report,
            &RenderOptions {
                expand: true,
                ..opts_at(s.taken_at, PolicySource::NotSet)
            },
        )
        .unwrap();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/reference/website-tabs-2026-09-07.json"
        );
        let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let web = doc["leagues"][&s.league]["web"].as_array().unwrap();

        let by_id: HashMap<&str, &TabSnapshot> =
            s.tabs.iter().map(|t| (t.id.as_str(), t)).collect();
        let listed: Vec<&TabSnapshot> = s
            .tabs
            .iter()
            .filter(|t| t.r#type != FOLDER)
            .filter(|t| {
                t.parent
                    .as_deref()
                    .is_none_or(|p| by_id.get(p).is_some_and(|q| q.r#type == FOLDER))
            })
            .collect();
        assert_eq!(listed.len(), web.len());
        assert_eq!(listed.len(), 322);
        for (i, (t, w)) in listed.iter().zip(web).enumerate() {
            assert_eq!(w["i"], json!(i));
            assert_eq!(w["n"].as_str(), Some(t.name.as_str()), "site {i}");
            assert_eq!(w["type"].as_str(), Some(t.r#type.as_str()), "site {i}");
        }

        assert_eq!(r.posted.len(), 31);
        for p in &r.posted {
            let id = tab_id(&p.location);
            let rank = listed.iter().position(|t| t.id == id).unwrap();
            let site = web[rank]["i"].as_u64().unwrap() as usize;
            assert_eq!(site, rank);
            let location = format!(" location=\"Stash{}\" ", site + 1);
            assert!(p.link.contains(&location), "{}: {}", p.target, p.link);
            assert!(
                p.link.contains(&format!(" league=\"{}\" ", s.league)),
                "{}",
                p.link
            );
        }
        let tab = real_scale_fixture::owners_test_tab(&s);
        let rank = listed.iter().position(|t| t.id == tab_id(&tab)).unwrap();
        assert_eq!(rank, 56);
        assert!(r.posted[0].link.contains(" location=\"Stash57\" "));
    }

    /// C72 — the freshness lines over the real page, at the snapshot's
    /// own clock: the test tab was fetched four days before the
    /// snapshot was taken, so under a policy that covers it with an
    /// hour's window every posted item is stale and the oldest age is
    /// that gap; under a policy that does not name it, it is the one
    /// container outside coverage and its items are counted beside
    /// that line; under a window wider than the gap, nothing. The
    /// tab's fetch came after the stash listing the links number by,
    /// so no position predates the listing.
    #[test]
    fn c72_at_real_scale_the_freshness_lines_name_the_owners_test_tab() {
        let (s, report) = fixture();
        let tab = real_scale_fixture::owners_test_tab(&s);
        let seen: Vec<i64> = s
            .items
            .iter()
            .filter(|i| i.location_id == tab_id(&tab))
            .map(|i| i.last_seen)
            .collect();
        assert_eq!(seen.len(), 80);
        let gap = s.taken_at - seen.iter().min().unwrap();
        assert!(gap > 4 * 86_400 && gap < 5 * 86_400, "{gap}");
        let listing = s.stash_listing.unwrap().response_id;
        assert!(
            s.items
                .iter()
                .filter(|i| i.location_id == tab_id(&tab))
                .all(|i| i.seen_response.is_some_and(|seen| seen > listing))
        );

        let policy = |value: Value| SyncPolicy::from_value(&value).unwrap();
        let covering = policy(json!({
            "version": 3,
            "realms": { "pc": { "leagues": { "Standard": {
                "tabs": [tab_id(&tab)], "max_age_seconds": 3600 } } } }
        }));
        let r = render(
            &report,
            &opts_at(
                s.taken_at,
                PolicySource::Set {
                    policy: &covering,
                    revision: 3,
                    refresh: Ok(2),
                },
            ),
        )
        .unwrap();
        let f = &r.freshness;
        assert_eq!(f.policy, "covers");
        assert_eq!(f.window_seconds, Some(3600));
        assert_eq!(f.refresh_requests, Some(2));
        assert!(f.uncovered.is_empty());
        // counted whole, the default's ten listed (C53)
        assert_eq!(f.stale_total, 31);
        assert_eq!(
            f.stale,
            r.posted
                .iter()
                .take(acquisition_store::bound::LISTED)
                .map(|p| p.target.clone())
                .collect::<Vec<_>>()
        );
        assert_eq!(f.stale_uncovered, 0);
        assert_eq!(f.oldest_stale_seconds, Some(gap));
        assert!(f.position_before_listing.is_empty());

        let elsewhere = policy(json!({
            "version": 3,
            "realms": { "pc": { "leagues": { "Standard": {
                "characters": "all", "max_age_seconds": 3600 } } } }
        }));
        let r = render(
            &report,
            &opts_at(
                s.taken_at,
                PolicySource::Set {
                    policy: &elsewhere,
                    revision: 4,
                    refresh: Ok(0),
                },
            ),
        )
        .unwrap();
        let f = &r.freshness;
        assert_eq!(f.uncovered, std::slice::from_ref(&tab));
        assert!(f.stale.is_empty());
        assert_eq!(f.stale_uncovered, 31);
        assert_eq!(f.oldest_stale_seconds, None);

        let wide = policy(json!({
            "version": 3,
            "realms": { "pc": { "leagues": { "Standard": {
                "tabs": "all", "max_age_seconds": 5 * 86_400 } } } }
        }));
        let r = render(
            &report,
            &opts_at(
                s.taken_at,
                PolicySource::Set {
                    policy: &wide,
                    revision: 5,
                    refresh: Ok(400),
                },
            ),
        )
        .unwrap();
        let f = &r.freshness;
        assert!(f.uncovered.is_empty());
        assert!(f.stale.is_empty());
        assert_eq!(f.stale_uncovered, 0);
        assert_eq!(f.oldest_stale_seconds, None);
    }
}
