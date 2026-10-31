use super::*;
use serde_json::{Value, json};

fn tab(id: &str, parent: Option<&str>, name: &str, r#type: &str, metadata: Value) -> TabSnapshot {
    TabSnapshot {
        id: id.into(),
        parent: parent.map(str::to_string),
        name: name.into(),
        r#type: r#type.into(),
        idx: None,
        listed_at: Some(100),
        listed_response: Some(2),
        fetched_at: Some(110),
        metadata,
        item_count: 0,
    }
}

fn item(id: &str, kind: &str, location: &str, note: Option<&str>) -> ItemSnapshot {
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
        type_line: "Imperial Bow".into(),
        stack_size: None,
        x: Some(0),
        y: Some(0),
        note: note.map(str::to_string),
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
        last_seen: 120,
    }
}

fn row(scope: &str, key: &str, value: Value, revision: i64) -> AnnotationRow {
    AnnotationRow {
        scope: scope.into(),
        key: key.into(),
        kind: "buyout".into(),
        value,
        revision,
        created_at: 50,
        updated_at: 60,
        written_via: "test".into(),
        actor: None,
    }
}

fn exact(amount: &str, currency: &str) -> Value {
    json!({ "version": 1, "type": "exact", "amount": amount, "currency": currency })
}

/// The stash in miniature: a folder named like a price holding a
/// public priced tab, a plain tab, a public priced map tab with a
/// substash, an unpriced unique tab with a substash, one character.
/// C151 — a row the store could not parse is one kind of unreadable
/// with a row the buyout kind refuses (the second review of #63, its
/// first and third findings): the item it names resolves unresolved,
/// so its public tab's price never wins past it (C81, the checklist's
/// "fall-through past an unreadable row"); the accounting lists it
/// with the parse fault as why, bounded as the status bounds every
/// list, under one total with the kind-refused rows; a row of another
/// realm counts there and names no subject.
#[test]
fn c151_a_row_the_store_could_not_parse_leaves_its_item_unresolved_and_is_accounted() {
    let unparsed = |scope: &str, key: &str, revision: i64| acquisition_store::UnreadableRow {
        scope: scope.into(),
        key: key.into(),
        kind: "buyout".into(),
        revision,
        fault: "key must be a string at line 1 column 2".into(),
    };
    let mut snap = snapshot();
    snap.buyouts.retain(|r| r.key != "i-plain");
    snap.unreadable_buyouts = vec![
        unparsed("item", "i-plain", 9), // above the public tab price
        unparsed("tab", "xbox/c9", 2),  // another realm
    ];
    let report = resolve(&snap).unwrap();
    let l = get(&report, &item_target("i-plain"));
    assert!(l.game.reading.price().is_some(), "the tab names a price");
    assert_eq!(l.effective.kind, "unresolved", "{}", l.effective.why);
    assert_eq!(l.effective.side, None);
    assert_eq!(
        l.manual_problem.as_deref(),
        Some("the row on item/i-plain cannot be read: key must be a string at line 1 column 2")
    );
    // Beside the rows the kind already refuses in this snapshot: one
    // accounting, one total.
    let base = resolve(&snapshot()).unwrap().rows;
    let rows = &report.rows;
    assert_eq!(rows.total, snap.buyouts.len() + 2, "every row counted once");
    assert_eq!(rows.unreadable_total, base.unreadable_total + 1);
    let mine = rows
        .unreadable
        .iter()
        .find(|p| p.key == "i-plain")
        .expect("the unparsed row is accounted");
    assert_eq!(
        (mine.scope.as_str(), mine.revision, mine.why.as_str()),
        ("item", 9, "key must be a string at line 1 column 2")
    );
    assert_eq!(rows.other_realm, base.other_realm + 1);
    // Eleven such rows: the status lists ten under the whole count.
    let mut snap = snapshot();
    snap.unreadable_buyouts = (0..11)
        .map(|n| unparsed("item", &format!("i-x{n}"), 1))
        .collect();
    let status = resolve(&snap).unwrap().status(false, 300);
    assert_eq!(
        (status.rows.unreadable.len(), status.rows.unreadable_total),
        (10, base.unreadable_total + 11)
    );
    assert_eq!(
        resolve(&snap)
            .unwrap()
            .status(true, 300)
            .rows
            .unreadable
            .len(),
        base.unreadable_total + 11
    );
}

fn snapshot() -> PricingSnapshot {
    let public = json!({ "public": true });
    PricingSnapshot {
        account_uuid: "u-1".into(),
        account_name: Some("tom".into()),
        realm: "pc".into(),
        league: "Standard".into(),
        taken_at: 200,
        stash_listing: Some(ListingBasis {
            response_id: 2,
            fetched_at: 100,
        }),
        tabs: vec![
            tab(
                "f1",
                None,
                "~price 9 chaos",
                "Folder",
                json!({ "folder": true }),
            ),
            tab(
                "c1",
                Some("f1"),
                "~price 3 chaos (A)",
                "PremiumStash",
                public.clone(),
            ),
            tab(
                "t1",
                None,
                "Dump",
                "PremiumStash",
                json!({ "colour": "ff0000" }),
            ),
            tab(
                "m1",
                None,
                "~price 1 divine (Remove-only)",
                "MapStash",
                public.clone(),
            ),
            tab(
                "s1",
                Some("m1"),
                "1 (Remove-only)",
                "MapStash",
                json!({ "items": 5, "map": {} }),
            ),
            tab("u1", None, "Uniques", "UniqueStash", Value::Null),
            tab(
                "s2",
                Some("u1"),
                "Belts",
                "UniqueStash",
                json!({ "items": 2 }),
            ),
        ],
        character_listing: Some(ListingBasis {
            response_id: 3,
            fetched_at: 101,
        }),
        characters: vec![CharacterSnapshot {
            id: "ch1".into(),
            name: "Exile".into(),
            league: Some("Standard".into()),
            listed_at: Some(101),
            listed_response: Some(3),
            fetched_at: Some(111),
            listed: json!({ "id": "ch1", "name": "Exile", "league": "Standard" }),
            fetched: json!({ "id": "ch1", "name": "Exile" }),
        }],
        items: vec![
            item("i-worn", "character", "ch1", Some("~b/o 2 divine")),
            item("i-exact", "stash", "c1", Some("~price 5 chaos")),
            item("i-plain", "stash", "c1", None),
            item("i-invalid", "stash", "c1", Some("~price  chaos")),
            item("i-fifty", "stash", "c1", Some("50")),
            item("i-skip", "stash", "c1", Some("~skip ")),
            item("i-dump", "stash", "t1", None),
            item("i-sub", "stash", "s1", None),
            item("i-sub2", "stash", "s2", None),
        ],
        unreadable_buyouts: Vec::new(),
        buyouts: vec![
            row("item", "i-plain", exact("3", "chaos"), 1),
            row(
                "item",
                "i-exact",
                json!({ "version": 1, "type": "skip" }),
                2,
            ),
            row("tab", "pc/f1", json!({ "version": 1, "type": "skip" }), 3),
            row("tab", "pc/m1", exact("1", "divine"), 4),
            row(
                "substash",
                "pc/u1/s2",
                json!({ "version": 1, "type": "negotiable", "amount": "7", "currency": "chaos" }),
                5,
            ),
            row("character", "ch1", exact("2222", "jewellers"), 6),
            row("tab", "xbox/c1", exact("1", "chaos"), 7),
            row("item", "i-elsewhere", exact("1", "chaos"), 8),
            row("tab", "pc", exact("1", "chaos"), 9),
            row(
                "item",
                "i-dump",
                json!({ "version": 2, "type": "exact" }),
                10,
            ),
        ],
    }
}

fn item_target(id: &str) -> PriceTarget {
    PriceTarget::Item { id: id.into() }
}

fn tab_target(id: &str) -> PriceTarget {
    PriceTarget::Tab {
        realm: Realm::Pc,
        id: id.into(),
    }
}

fn get<'a>(report: &'a ListingReport, target: &PriceTarget) -> &'a Listing {
    report
        .find(target)
        .unwrap_or_else(|| panic!("no listing for {target}"))
}

/// C69 — the game side reads the note, then the tab name, as a price
/// or `skip`; an invalid note has no effect and the tab applies (T18);
/// the raw texts ride beside every reading; a note where the index
/// cannot see it is residue; the relation names both sides; the
/// versions are stamped.
#[test]
fn c69_two_sides_resolve_independently_and_the_relation_names_both() {
    let report = resolve(&snapshot()).unwrap();
    assert_eq!(report.header.note_parser_version, NOTE_PARSER_VERSION);
    assert_eq!(report.header.currency_table_version, CURRENCY_TABLE_VERSION);
    assert_eq!(report.header.stash_listing.map(|b| b.response_id), Some(2));

    // A note in front of a priced, public tab: the note applies, and
    // beats the item's own row on a tie (C81).
    let l = get(&report, &item_target("i-exact"));
    assert_eq!(l.game.source, Some(Source::Note));
    assert_eq!(l.game.reading.to_string(), "5 chaos");
    assert_eq!(l.game.note.as_ref().unwrap().text, "~price 5 chaos");
    assert_eq!(l.game.tab_name.as_ref().unwrap().text, "~price 3 chaos (A)");
    assert_eq!(
        l.game.tab_name.as_ref().unwrap().reading.to_string(),
        "3 chaos"
    );
    assert_eq!(l.game.public, Some(true));
    assert_eq!(l.manual.as_ref().unwrap().value, Buyout::Skip);
    assert_eq!(l.relation, Relation::Conflict);
    assert_eq!(l.why, "by hand: skip; in game: 5 chaos");
    assert_eq!(
        l.basis,
        Basis {
            response: Some(7),
            at: Some(120),
            at_age: None,
        }
    );

    // No note: the tab name applies, and it agrees with the own row.
    let l = get(&report, &item_target("i-plain"));
    assert_eq!(l.game.source, Some(Source::TabName));
    assert_eq!(l.game.reading.to_string(), "3 chaos");
    assert_eq!(l.game.note, None);
    assert_eq!(l.relation, Relation::Agree);
    assert_eq!(l.why, "by hand and in game: 3 chaos");
    assert_eq!(l.manual.as_ref().unwrap().revision, 1);

    // The dialog's residue reads `invalid`, has no effect, and the tab
    // applies — as the site does (T18); the note is still shown.
    let l = get(&report, &item_target("i-invalid"));
    assert!(
        matches!(
            l.game.note.as_ref().unwrap().reading,
            GamePrice::Invalid { .. }
        ),
        "{:?}",
        l.game.note
    );
    assert_eq!(l.game.note.as_ref().unwrap().text, "~price  chaos");
    assert_eq!(l.game.source, Some(Source::TabName));
    assert_eq!(l.game.reading.to_string(), "3 chaos");
    assert_eq!(l.relation, Relation::Conflict); // beside the folder's skip
    assert_eq!(l.why, "by hand: skip; in game: 3 chaos");

    // A note that is not a price note: the tab name applies.
    let l = get(&report, &item_target("i-fifty"));
    assert_eq!(l.game.note.as_ref().unwrap().reading, GamePrice::None);
    assert_eq!(l.game.source, Some(Source::TabName));
    assert_eq!(l.game.reading.to_string(), "3 chaos");

    // `~skip` beside an inherited `skip`: both leave it out.
    let l = get(&report, &item_target("i-skip"));
    assert_eq!(l.game.reading, GamePrice::Skip);
    assert_eq!(l.relation, Relation::Agree);

    // A character item: the note is read and shown, but a character
    // is not a public tab, so it is residue, not a statement (C81).
    let l = get(&report, &item_target("i-worn"));
    assert_eq!(
        l.game.note.as_ref().unwrap().reading.to_string(),
        "2 divine b/o"
    );
    assert_eq!(l.game.reading, GamePrice::None);
    assert!(l.game.residue);
    assert_eq!(l.game.tab, None);
    assert_eq!(l.game.public, None);
    assert_eq!(l.relation, Relation::ManualOnly);
    assert_eq!(l.why, "by hand: 2222 jewellers; the game states nothing");

    // An unreadable own row and nothing in game: the relation is
    // `none`, and the sentence names the row, not "no row applies".
    let l = get(&report, &item_target("i-dump"));
    assert_eq!(l.relation, Relation::None);
    assert_eq!(
        l.why,
        "the row on item/i-dump cannot be read and the game states nothing"
    );
    assert_eq!(l.game.public, Some(false));
    assert!(!l.game.residue);

    // The counts are the items' — containers are not items.
    assert_eq!(report.counts.items, 9);
    assert_eq!(report.counts.containers, 8);
    assert_eq!(report.counts.by_relation["conflict"], 3);
    assert_eq!(report.counts.by_relation["agree"], 3);
    assert_eq!(report.counts.by_relation["manual_only"], 2);
    assert_eq!(report.counts.by_relation["game_only"], 0);
    assert_eq!(report.counts.by_relation["none"], 1);
    assert_eq!(report.counts.invalid_notes, 1);
    assert_eq!(report.counts.by_game_statement["exact"], 5);
    assert_eq!(report.counts.by_game_statement["skip"], 1);
    assert_eq!(report.counts.by_game_statement["none"], 3);
    assert_eq!(report.counts.game_priced, 5);
    assert_eq!(report.counts.residue, 1);

    // Each count's members are a list's selection under the count's own
    // word (#11), whatever their relation: the residue a character's
    // `manual_only` item, the invalid note one the tab's price decides.
    let selected = |text: GameText| {
        let view = report
            .list_view(ListFilter {
                game_text: Some(text),
                ..ListFilter::default()
            })
            .unwrap();
        let ids: Vec<String> = view
            .items
            .iter()
            .map(|l| l.subject.target.to_string())
            .collect();
        (view.total, ids)
    };
    assert_eq!(
        selected(GameText::Residue),
        (report.counts.residue, vec!["item/i-worn".to_string()])
    );
    assert_eq!(
        selected(GameText::InvalidNote),
        (
            report.counts.invalid_notes,
            vec!["item/i-invalid".to_string()]
        )
    );
}

/// C81 — a note the store could not read is a statement of unknown
/// content at item level, so nothing decides: unresolved beside a
/// game tab price and beside the item's own row alike, the row and
/// the tab still shown. Found by the search's generators (step 9).
#[test]
fn c81_a_note_that_cannot_be_read_leaves_the_price_unresolved() {
    let mut snap = snapshot();
    for item in snap.items.iter_mut() {
        if item.id == "i-plain" || item.id == "i-invalid" {
            item.note = None;
            item.note_unread = true;
        }
    }
    let report = resolve(&snap).unwrap();
    for id in ["i-plain", "i-invalid"] {
        let l = get(&report, &item_target(id));
        assert!(l.game.note_unread && l.game.note.is_none());
        assert_eq!(
            (l.effective.side, l.effective.kind.as_str()),
            (None, "unresolved"),
            "{id}"
        );
        assert_eq!(l.effective.from, Some(item_target(id)));
        assert!(
            l.effective
                .why
                .starts_with(&format!("unresolved: the note on item/{id} cannot be read")),
            "{}",
            l.effective.why
        );
        assert!(l.why.ends_with("; the note cannot be read"), "{}", l.why);
    }
    // where the game side cannot speak — the character's item, an item
    // in the non-public tab — an unread note is residue at most, and
    // the effective price is what it was without it (the step-9 review)
    let baseline = resolve(&snapshot()).unwrap();
    let mut muted = snapshot();
    let mut gated = Vec::new();
    for item in muted.items.iter_mut() {
        let private = item.location_kind == "character" || item.location_id == "t1";
        if private {
            item.note = None;
            item.note_unread = true;
            gated.push(item_target(&item.id));
        }
    }
    assert!(gated.len() >= 2, "{gated:?}");
    let gated_report = resolve(&muted).unwrap();
    for target in &gated {
        let (now, was) = (get(&gated_report, target), get(&baseline, target));
        assert!(now.game.note_unread);
        // the decision, not the sentence, which now says the note
        // cannot be read
        let decision = |e: &Effective| (e.kind.clone(), e.price.clone(), e.side, e.from.clone());
        assert_eq!(
            decision(&now.effective),
            decision(&was.effective),
            "{target}"
        );
    }
    // the row on `i-plain` is still reported, and the tab price on
    // `i-invalid`'s tab: nothing is hidden by the unread note
    assert_eq!(
        get(&report, &item_target("i-plain"))
            .manual
            .as_ref()
            .map(|m| m.value.kind()),
        Some("exact")
    );
    assert!(
        get(&report, &item_target("i-invalid"))
            .game
            .tab_name
            .is_some()
    );
    assert_eq!(report.counts.by_effective["unresolved"], 3);
}

/// C81 — the effective price is the more specific statement, the
/// game's on a tie; only a public tab's game side is a statement.
#[test]
fn c81_the_effective_price_is_the_most_specific_statement_and_the_games_on_a_tie() {
    let report = resolve(&snapshot()).unwrap();
    let eff = |t: PriceTarget| get(&report, &t).effective.clone();
    // A game note beats the item's own row (a tie at item level).
    let e = eff(item_target("i-exact"));
    assert_eq!(
        (e.side, e.to_string()),
        (Some(Side::Game), "5 chaos".into())
    );
    assert_eq!(e.from, Some(item_target("i-exact")));
    assert!(
        e.why
            .ends_with("the game wins a tie with the row on the item"),
        "{}",
        e.why
    );
    // An item row beats a game tab price.
    let e = eff(item_target("i-plain"));
    assert_eq!((e.side, e.kind.as_str()), (Some(Side::Manual), "exact"));
    assert!(
        e.why
            .contains("the row on the item is more specific than the game's tab name"),
        "{}",
        e.why
    );
    // A game tab price beats a folder row (an invalid note has no effect).
    let e = eff(item_target("i-invalid"));
    assert_eq!(
        (e.side, e.to_string()),
        (Some(Side::Game), "3 chaos".into())
    );
    assert_eq!(e.from, Some(tab_target("c1")));
    assert!(
        e.why
            .contains("the tab name is more specific than the row on the tab"),
        "{}",
        e.why
    );
    // Game `skip` beside the folder's `skip`: the game's, by level.
    let e = eff(item_target("i-skip"));
    assert_eq!((e.side, e.kind.as_str()), (Some(Side::Game), "skip"));
    // A substash item under a public priced parent: the parent's name
    // and the parent's row tie at tab level; the game wins.
    let e = eff(item_target("i-sub"));
    assert_eq!(
        (e.side, e.to_string()),
        (Some(Side::Game), "1 divine".into())
    );
    assert_eq!(e.from, Some(tab_target("m1")));
    // A substash row is more specific than the parent's name: the
    // substash's own listing takes its row, its item inherits it.
    let s2 = PriceTarget::Substash {
        realm: Realm::Pc,
        parent: "u1".into(),
        id: "s2".into(),
    };
    let e = eff(item_target("i-sub2"));
    assert_eq!(
        (e.side, e.to_string()),
        (Some(Side::Manual), "7 chaos b/o".into())
    );
    assert_eq!(e.from, Some(s2));
    // A character's items: the note is residue, the row applies, and
    // the sentence says why the game's text does not.
    let e = eff(item_target("i-worn"));
    assert_eq!(
        (e.side, e.to_string()),
        (Some(Side::Manual), "2222 jewellers".into())
    );
    assert!(
        e.why.contains("not where the index can see it"),
        "{}",
        e.why
    );
    // An unreadable own row and no game statement: unresolved, naming
    // the row — never "nothing applies".
    let e = eff(item_target("i-dump"));
    assert_eq!((e.side, e.kind.as_str()), (None, "unresolved"));
    assert_eq!(e.from, Some(item_target("i-dump")));
    assert!(
        e.why
            .starts_with("unresolved: the row on item/i-dump cannot be read"),
        "{}",
        e.why
    );
    // Containers resolve the same way: the tab's own name beats the
    // folder's row; the folder itself has only its row.
    let e = eff(tab_target("c1"));
    assert_eq!(
        (e.side, e.to_string()),
        (Some(Side::Game), "3 chaos".into())
    );
    let e = eff(tab_target("f1"));
    assert_eq!((e.side, e.kind.as_str()), (Some(Side::Manual), "skip"));
    assert_eq!(report.counts.by_effective["game"], 5);
    assert_eq!(report.counts.by_effective["manual"], 3);
    assert_eq!(report.counts.by_effective["none"], 0);
    assert_eq!(report.counts.by_effective["unresolved"], 1);
    // A public tab's price text is a statement; the same text on a
    // non-public tab is residue: the manual row applies alone.
    let mut snap = snapshot();
    snap.tabs[1].metadata = Value::Null; // c1 no longer public
    let report = resolve(&snap).unwrap();
    let l = get(&report, &item_target("i-exact"));
    assert_eq!(l.game.reading, GamePrice::None);
    assert!(l.game.residue);
    assert_eq!(l.game.note.as_ref().unwrap().reading.to_string(), "5 chaos");
    assert_eq!(l.relation, Relation::ManualOnly);
    assert_eq!(l.effective.side, Some(Side::Manual));
    assert_eq!(l.effective.kind, "skip");
    // An unreadable row that could beat the game's statement leaves
    // the price unresolved; one the game beats by level does not.
    let unreadable =
        |scope: &str, key: &str| row(scope, key, json!({ "version": 9, "type": "exact" }), 20);
    let mut snap = snapshot();
    snap.buyouts.retain(|r| r.key != "i-plain");
    snap.buyouts.push(unreadable("item", "i-plain")); // above the public tab price
    snap.buyouts.push(unreadable("substash", "pc/m1/s1")); // above the parent's price
    let report = resolve(&snap).unwrap();
    for id in ["i-plain", "i-sub"] {
        let l = get(&report, &item_target(id));
        assert!(l.game.reading.price().is_some(), "{id}");
        assert_eq!(l.effective.kind, "unresolved", "{id}: {}", l.effective.why);
        assert_eq!(l.effective.side, None);
        assert!(l.manual_problem.is_some(), "{id}");
    }
    assert_eq!(
        get(&report, &item_target("i-plain")).effective.from,
        Some(item_target("i-plain"))
    );
    let mut snap = snapshot();
    snap.buyouts.retain(|r| r.key != "pc/f1");
    snap.buyouts.push(unreadable("tab", "pc/f1")); // below the tab's price
    let report = resolve(&snap).unwrap();
    let l = get(&report, &item_target("i-fifty"));
    assert_eq!(l.effective.side, Some(Side::Game));
    assert!(
        l.effective.why.contains("readable or not"),
        "{}",
        l.effective.why
    );
    assert!(l.manual_problem.is_some());
    // An item's show view carries no item sets.
    let view = report.show_view(&item_target("i-fifty")).unwrap();
    assert!(view.items_here.is_empty() && view.items_covered_below.is_empty());
    // The relation sentence names the unreadable row beside a game
    // statement too, and the default list never hides an unresolved
    // item behind its relation word; `effective` selects by side.
    assert_eq!(
        l.why,
        "in game: 3 chaos; the row on tab/pc/f1 cannot be read"
    );
    let report = resolve(&snapshot()).unwrap();
    let default = report.list_view(ListFilter::default()).unwrap();
    assert!(
        default
            .items
            .iter()
            .any(|l| l.subject.target == item_target("i-dump"))
    );
    let unresolved = report
        .list_view(ListFilter {
            effective: Some("unresolved".into()),
            ..ListFilter::default()
        })
        .unwrap();
    assert_eq!(
        unresolved
            .items
            .iter()
            .map(|l| l.subject.target.to_string())
            .collect::<Vec<_>>(),
        ["item/i-dump"]
    );
    // Every effective side selects on its own, `none` included: the
    // default suppression of relation `none` does not run under a
    // selector, or `--effective none` could never find anything.
    let mut snap = snapshot();
    snap.buyouts.retain(|r| r.key != "i-dump"); // i-dump: nothing applies
    let report = resolve(&snap).unwrap();
    let by_side = |side: &str| -> Vec<String> {
        report
            .list_view(ListFilter {
                effective: Some(side.into()),
                ..ListFilter::default()
            })
            .unwrap()
            .items
            .iter()
            .map(|l| l.subject.target.to_string())
            .collect()
    };
    assert_eq!(by_side("game").len(), 5);
    assert_eq!(by_side("manual").len(), 3);
    assert_eq!(by_side("none"), ["item/i-dump"]);
    assert_eq!(by_side("unresolved"), Vec::<String>::new());
    assert_eq!(
        by_side("game").len() + by_side("manual").len() + by_side("none").len(),
        report.counts.items
    );
}

/// C53 — the JSON shape is the contract: the resolved fixture must
/// serialize to exactly the committed document. A difference is
/// either additive (regenerate with `ACQ_UPDATE_FIXTURES=1`) or a
/// schema bump (`LISTING_SCHEMA`), decided by reading the diff.
#[test]
fn the_report_json_matches_the_committed_fixture() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/reference/listing-report-schema-3.json"
    );
    let report = resolve(&snapshot()).unwrap();
    let actual = serde_json::to_value(&report).unwrap();
    if std::env::var_os("ACQ_UPDATE_FIXTURES").is_some() {
        std::fs::write(path, serde_json::to_string_pretty(&actual).unwrap() + "\n").unwrap();
    }
    let expected: Value = serde_json::from_str(
        &std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}")),
    )
    .unwrap();
    assert_eq!(expected["schema"], json!(LISTING_SCHEMA));
    assert_eq!(
        actual, expected,
        "the report's JSON changed; additive → ACQ_UPDATE_FIXTURES=1, else bump LISTING_SCHEMA"
    );
    let back: ListingReport = serde_json::from_value(expected).unwrap();
    assert_eq!(back, report);
}

/// C70 — the manual side by specificity: the own row, else the
/// substash's, else the tab's, else the folder's; a character row
/// covers its items; coverage is a row's existence, never its type.
#[test]
fn c70_the_manual_side_resolves_by_specificity() {
    let report = resolve(&snapshot()).unwrap();
    let from = |id: &str| {
        let l = get(&report, &item_target(id));
        let m = l
            .manual
            .as_ref()
            .unwrap_or_else(|| panic!("{id} has no manual side"));
        (m.from.clone(), m.inherited, m.value.to_string())
    };
    assert_eq!(
        from("i-plain"),
        (item_target("i-plain"), false, "3 chaos".into())
    );
    assert_eq!(from("i-fifty"), (tab_target("f1"), true, "skip".into()));
    assert_eq!(from("i-sub"), (tab_target("m1"), true, "1 divine".into()));
    assert_eq!(
        from("i-sub2"),
        (
            PriceTarget::Substash {
                realm: Realm::Pc,
                parent: "u1".into(),
                id: "s2".into()
            },
            true,
            "7 chaos b/o".into()
        )
    );
    assert_eq!(
        from("i-worn"),
        (
            PriceTarget::Character { id: "ch1".into() },
            true,
            "2222 jewellers".into()
        )
    );
    assert_eq!(
        get(&report, &item_target("i-sub")).relation,
        Relation::Agree
    );
    assert_eq!(
        get(&report, &item_target("i-sub2")).relation,
        Relation::ManualOnly
    );
    // A container's own listing walks the same chain: the substash
    // under the priced map tab inherits it; the folder child inherits
    // the folder's row.
    let s1 = PriceTarget::Substash {
        realm: Realm::Pc,
        parent: "m1".into(),
        id: "s1".into(),
    };
    assert_eq!(
        get(&report, &s1).manual.as_ref().unwrap().from,
        tab_target("m1")
    );
    assert_eq!(
        get(&report, &tab_target("c1"))
            .manual
            .as_ref()
            .unwrap()
            .from,
        tab_target("f1")
    );
    assert_eq!(report.counts.inherited, 6);
    // An unreadable row that would apply stops the walk: the item
    // under it has no manual side and says why, and does not fall
    // through to the tab.
    let l = get(&report, &item_target("i-dump"));
    assert_eq!(l.manual, None);
    assert!(
        l.manual_problem.as_deref().unwrap().contains("item/i-dump"),
        "{:?}",
        l.manual_problem
    );
}

/// C80 — a substash reads its parent's name and `public`, its own
/// name beside; a folder child reads itself; a folder's name is never
/// a price, for the folder or through it.
#[test]
fn c80_a_substash_reads_its_parent_a_folder_child_itself_and_a_folder_never() {
    let report = resolve(&snapshot()).unwrap();
    let l = get(&report, &item_target("i-sub"));
    assert_eq!(l.game.tab.as_deref(), Some("m1"));
    assert_eq!(
        l.game.tab_name.as_ref().unwrap().text,
        "~price 1 divine (Remove-only)"
    );
    assert_eq!(l.game.reading.to_string(), "1 divine");
    assert_eq!(l.game.substash_name.as_deref(), Some("1 (Remove-only)"));
    assert_eq!(l.game.public, Some(true));
    let l = get(&report, &item_target("i-sub2"));
    assert_eq!(l.game.tab.as_deref(), Some("u1"));
    assert_eq!(l.game.reading, GamePrice::None);
    assert_eq!(l.game.substash_name.as_deref(), Some("Belts"));
    assert_eq!(l.game.public, Some(false));
    // The folder child reads its own name, not the folder's.
    let l = get(&report, &item_target("i-plain"));
    assert_eq!(l.game.tab.as_deref(), Some("c1"));
    assert_eq!(l.game.reading.to_string(), "3 chaos");
    // The folder itself: named like a price, read as nothing.
    let l = get(&report, &tab_target("f1"));
    assert_eq!(l.subject.tab_type.as_deref(), Some("Folder"));
    assert_eq!(l.game.reading, GamePrice::None);
    assert_eq!(l.game.tab_name, None);
    assert_eq!(l.game.public, None);
    assert_eq!(l.relation, Relation::ManualOnly);
    // Priced tabs count the tabs a user can name: not folders, not substashes.
    assert_eq!(report.counts.priced_tabs, 2);
    assert_eq!(report.counts.priced_tabs_public, 2);
    // A substash whose parent is not on record reads nothing and says so.
    let mut snap = snapshot();
    snap.tabs
        .push(tab("s9", Some("gone"), "9", "MapStash", Value::Null));
    snap.items.push(item("i-orphan", "stash", "s9", None));
    let report = resolve(&snap).unwrap();
    let l = get(&report, &item_target("i-orphan"));
    assert_eq!(l.game.tab, None);
    assert_eq!(l.game.substash_name.as_deref(), Some("9"));
    assert_eq!(l.game.public, None);
}

/// C53 — every nothing says which nothing: rows of another realm are
/// counted, readable rows naming nothing in these facts are listed,
/// unreadable rows are listed with why; and the report is its own
/// contract — it re-reads exactly.
#[test]
fn rows_are_accounted_for_and_the_report_round_trips() {
    let report = resolve(&snapshot()).unwrap();
    assert_eq!(report.rows.total, 10);
    assert_eq!(report.rows.applied, 6);
    assert_eq!(report.rows.other_realm, 1);
    let unmatched: Vec<&PriceTarget> = report.rows.unmatched.iter().map(|m| &m.from).collect();
    assert_eq!(unmatched, [&item_target("i-elsewhere")]);
    assert!(report.rows.unmatched.iter().all(|m| !m.inherited));
    let unreadable: Vec<(&str, &str)> = report
        .rows
        .unreadable
        .iter()
        .map(|p| (p.key.as_str(), p.why.as_str()))
        .collect();
    assert_eq!(unreadable.len(), 2, "{unreadable:?}");
    assert_eq!(unreadable[0].0, "pc");
    assert!(
        unreadable[0].1.contains("is not a tab key"),
        "{}",
        unreadable[0].1
    );
    assert_eq!(unreadable[1].0, "i-dump");
    assert!(
        unreadable[1].1.contains("a newer build wrote it"),
        "{}",
        unreadable[1].1
    );

    let text = serde_json::to_string(&report).unwrap();
    let back: ListingReport = serde_json::from_str(&text).unwrap();
    assert_eq!(back, report);
    assert_eq!(report.summary().listings.len(), 0);
    assert_eq!(report.summary().counts, report.counts);
    assert_eq!(report.items_in(&tab_target("c1")).count(), 5);
    // A realm word the snapshot carries that Realm does not know is
    // an error, never a report with every tab row set aside.
    let mut snap = snapshot();
    snap.realm = "ps5".into();
    assert!(matches!(
        resolve(&snap),
        Err(ListingError::UnknownRealm { .. })
    ));
}

/// C53, the read economy (step 2g, round 2): a container's two sets
/// list ten each by default, counted whole beside the rows — the
/// total, how many listed and left out, and the relation breakdown
/// made before the cut — `limit` lists more, and `expand`, the audit
/// request, holds every item the default left out (the cold review
/// of the brief, findings 6 and 7). A parent map tab of thirteen
/// items of its own over a substash of fifteen.
#[test]
fn c53_a_containers_two_sets_list_ten_beside_their_whole_counts_and_expand_holds_every_omitted_item()
 {
    use acquisition_store::bound::LISTED;
    let mut snap = snapshot();
    snap.items
        .extend((0..13).map(|i| item(&format!("i-m{i:02}"), "stash", "m1", None)));
    snap.items.extend(
        (0..14).map(|i| item(&format!("i-s{i:02}"), "stash", "s1", Some("~price 2 chaos"))),
    );
    let report = resolve(&snap).unwrap();
    let m1 = tab_target("m1");
    let whole = report.show_view(&m1).unwrap();
    let ids = |set: &[Listing]| -> Vec<String> {
        set.iter().map(|l| l.subject.target.to_string()).collect()
    };
    let view = whole.clone().bounded(None, false);
    for (set, counts, all) in [
        (&view.items_here, &view.items_here_counts, &whole.items_here),
        (
            &view.items_covered_below,
            &view.items_covered_below_counts,
            &whole.items_covered_below,
        ),
    ] {
        assert!(all.len() > LISTED, "{}", all.len());
        assert_eq!(set.len(), LISTED);
        assert_eq!(
            (counts.total, counts.returned, counts.left_out),
            (all.len(), LISTED, all.len() - LISTED)
        );
        // the breakdown is the whole set's, never the sample's
        let summed: usize = counts.by_relation.values().sum();
        assert_eq!(summed + counts.unresolved, all.len(), "{counts:?}");
        assert_eq!(ids(set), ids(&all[..LISTED]), "the head, in order");
    }
    assert_eq!(view.items_here_counts.total, 13);
    assert_eq!(view.items_covered_below_counts.total, 15);
    assert_eq!(
        view.items_here_counts.by_relation,
        whole.items_here_counts.by_relation
    );

    let more = whole.clone().bounded(Some(12), false);
    assert_eq!(more.items_here.len(), 12);
    assert_eq!(more.items_covered_below_counts.left_out, 3);

    // the audit request: every item the default left out, in it
    let audit = whole.clone().bounded(Some(2), true);
    for (shown, all, listed) in [
        (&view.items_here, &whole.items_here, &audit.items_here),
        (
            &view.items_covered_below,
            &whole.items_covered_below,
            &audit.items_covered_below,
        ),
    ] {
        let listed = ids(listed);
        let omitted: Vec<String> = ids(all)
            .into_iter()
            .filter(|id| !ids(shown).contains(id))
            .collect();
        assert!(!omitted.is_empty());
        assert!(omitted.iter().all(|id| listed.contains(id)), "{omitted:?}");
        assert_eq!(listed.len(), all.len());
    }
    assert_eq!(audit.items_here_counts.left_out, 0);
}

/// C53, the read economy (step 2g, round 2; the second cold review of
/// the brief, finding 2: no exemption): the status's three row lists
/// — rows naming nothing here, rows that cannot be read, rows of
/// another league — each hold ten beside a count of the whole, and
/// `expand` lists each whole; the whole report keeps every row.
#[test]
fn c53_the_status_lists_ten_of_each_row_list_beside_its_whole_count() {
    use acquisition_store::bound::LISTED;
    let mut snap = snapshot();
    let skip = json!({ "version": 1, "type": "skip" });
    for i in 0..12 {
        snap.buyouts.push(row(
            "item",
            &format!("i-gone-{i:02}"),
            skip.clone(),
            100 + i,
        ));
        snap.buyouts.push(row(
            "item",
            &format!("i-else-{i:02}"),
            skip.clone(),
            200 + i,
        ));
        // a tab key with no realm cannot be read
        snap.buyouts
            .push(row("tab", &format!("k{i:02}"), skip.clone(), 300 + i));
    }
    let mut report = resolve(&snap).unwrap();
    report
        .place_other_leagues(|t| {
            Ok::<_, std::convert::Infallible>(t.to_string().starts_with("item/i-else-"))
        })
        .unwrap();
    let whole = &report.rows;
    // the fixture's own: one row naming nothing, two unreadable
    assert_eq!(
        (
            whole.unmatched.len(),
            whole.unreadable.len(),
            whole.other_league.len()
        ),
        (13, 14, 12)
    );
    assert_eq!(
        (
            whole.unmatched_total,
            whole.unreadable_total,
            whole.other_league_total
        ),
        (13, 14, 12)
    );
    let status = report.status(false, 0);
    let rows = &status.rows;
    assert_eq!(
        (
            rows.unmatched.len(),
            rows.unreadable.len(),
            rows.other_league.len()
        ),
        (LISTED, LISTED, LISTED)
    );
    assert_eq!(
        (
            rows.unmatched_total,
            rows.unreadable_total,
            rows.other_league_total
        ),
        (13, 14, 12)
    );
    assert_eq!(&rows.unmatched[..], &whole.unmatched[..LISTED]);
    // the whole-set counts stand as they are
    assert_eq!(
        (rows.total, rows.applied, rows.other_realm),
        (whole.total, whole.applied, whole.other_realm)
    );
    assert!(status.listings.is_empty());
    assert_eq!(report.status(true, 0).rows, report.rows);
}

/// C53 (an age the text prints stands in words beside its epoch,
/// made at the answer's own time): `status` says the snapshot and the
/// two listings it was resolved over, `show` the snapshot, its
/// target's hand row and the fact its texts came from — each in the
/// store's words at the time given, beside its epoch — and a view no
/// text ages (`list`, the report) carries none.
#[test]
fn c53_status_and_show_say_the_ages_their_texts_print() {
    let report = resolve(&snapshot()).unwrap();
    let now = report.header.taken_at + 3_600;
    let said = |at: Option<i64>| Some(acquisition_store::age::ago(now, at));
    let status = report.status(false, now);
    let h = &status.header;
    assert_eq!(h.taken_at_age.as_deref(), Some("60m ago"));
    assert_eq!(
        h.stash_listing_fetched_at_age,
        said(h.stash_listing.map(|l| l.fetched_at))
    );
    assert_eq!(
        h.character_listing_fetched_at_age,
        said(h.character_listing.map(|l| l.fetched_at))
    );
    let unaged = serde_json::to_value(&report).unwrap();
    assert!(unaged.get("taken_at_age").is_none(), "{unaged}");

    let item = report
        .listings
        .iter()
        .find(|l| l.subject.is_item() && l.manual.is_some())
        .expect("an item a hand row prices")
        .subject
        .target
        .clone();
    let shown = report.show_view(&item).unwrap().aged(now);
    let doc = serde_json::to_value(&shown).unwrap();
    assert_eq!(doc["taken_at_age"], "60m ago", "{doc}");
    assert!(doc.get("stash_listing_fetched_at_age").is_none(), "{doc}");
    let manual = shown.listing.manual.as_ref().unwrap();
    assert_eq!(manual.updated_at_age, said(Some(manual.updated_at)));
    assert_eq!(shown.listing.basis.at_age, said(shown.listing.basis.at));
}

/// C70 — every listing carries its chain, so a target's coverage is
/// a query: a folder covers its tabs' items without holding any, a
/// parent tab covers its substashes' items beside its own, an
/// ordinary tab's two sets are one.
#[test]
fn c70_a_targets_coverage_is_every_listing_whose_chain_holds_it() {
    let report = resolve(&snapshot()).unwrap();
    let ids = |it: Box<dyn Iterator<Item = &Listing> + '_>| -> Vec<String> {
        it.map(|l| l.subject.target.to_string()).collect()
    };
    let f1 = tab_target("f1");
    assert_eq!(ids(Box::new(report.items_in(&f1))), Vec::<String>::new());
    assert_eq!(
        ids(Box::new(report.items_covered_by(&f1))),
        [
            "item/i-exact",
            "item/i-plain",
            "item/i-invalid",
            "item/i-fifty",
            "item/i-skip"
        ]
    );
    let m1 = tab_target("m1");
    assert_eq!(ids(Box::new(report.items_in(&m1))), Vec::<String>::new());
    assert_eq!(ids(Box::new(report.items_covered_by(&m1))), ["item/i-sub"]);
    let c1 = tab_target("c1");
    assert_eq!(
        ids(Box::new(report.items_in(&c1))),
        ids(Box::new(report.items_covered_by(&c1)))
    );
    let l = get(&report, &item_target("i-sub"));
    assert_eq!(
        l.chain.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["item/i-sub", "substash/pc/m1/s1", "tab/pc/m1"]
    );
    let l = get(&report, &item_target("i-worn"));
    assert_eq!(
        l.chain.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["item/i-worn", "character/ch1"]
    );
    // The views carry both sets and refuse a container not on record.
    let view = report.show_view(&f1).unwrap();
    assert!(view.items_here.is_empty());
    assert_eq!(view.items_covered_below.len(), 5);
    assert_eq!(view.container, None);
    let view = report.show_view(&item_target("i-sub")).unwrap();
    assert_eq!(
        view.container.as_ref().map(|c| c.subject.name.as_str()),
        Some("1 (Remove-only)")
    );
    let view = report
        .list_view(ListFilter {
            covered_by: Some(m1.clone()),
            ..ListFilter::default()
        })
        .unwrap();
    assert_eq!(ids(Box::new(view.items.iter())), ["item/i-sub"]);
    assert_eq!(
        ids(Box::new(view.containers.iter())),
        ["tab/pc/m1", "substash/pc/m1/s1"]
    );
    assert_eq!(view.items_on_record, 9);
    assert_eq!(
        report.list_view(ListFilter {
            r#in: Some(tab_target("nope")),
            ..ListFilter::default()
        }),
        None
    );
    // A view is its own contract: it re-reads exactly.
    let text = serde_json::to_string(&view).unwrap();
    assert_eq!(serde_json::from_str::<ListView>(&text).unwrap(), view);
    let show = report.show_view(&m1).unwrap();
    let text = serde_json::to_string(&show).unwrap();
    assert_eq!(serde_json::from_str::<ShowView>(&text).unwrap(), show);
    assert_eq!(
        text.matches(&format!("\"schema\":{LISTING_SCHEMA}"))
            .count(),
        1,
        "{text}"
    );
}

/// A character the listing gave no league is carried by every
/// league's snapshot (the planner's rule); its items are flagged and
/// counted here, never silently attributed.
#[test]
fn a_league_less_characters_items_are_flagged_in_every_leagues_report() {
    let mut snap = snapshot();
    snap.characters.push(CharacterSnapshot {
        id: "ch-none".into(),
        name: "Drifter".into(),
        league: None,
        listed_at: Some(101),
        listed_response: Some(3),
        fetched_at: Some(111),
        listed: json!({ "id": "ch-none", "name": "Drifter" }),
        fetched: Value::Null,
    });
    snap.items.push(item(
        "i-drift",
        "character",
        "ch-none",
        Some("~price 1 chaos"),
    ));
    for league in ["Standard", "Hardcore"] {
        snap.league = league.into();
        let report = resolve(&snap).unwrap();
        let l = get(&report, &item_target("i-drift"));
        assert!(l.subject.league_unknown, "{league}");
        assert!(
            get(
                &report,
                &PriceTarget::Character {
                    id: "ch-none".into()
                }
            )
            .subject
            .league_unknown
        );
        assert!(!get(&report, &item_target("i-worn")).subject.league_unknown);
        assert_eq!(report.counts.league_unknown, 1);
        assert_eq!(report.counts.items, 10);
    }
    // The flag is absent from JSON when false, present when true.
    let report = resolve(&snap).unwrap();
    let text = serde_json::to_string(&report).unwrap();
    assert_eq!(text.matches("\"league_unknown\":true").count(), 2, "{text}");
    assert!(!text.contains("\"league_unknown\":false"));
}

/// The real-scale fixture (plan step 7, item 2, `PRICING-SLICE.md` at `b6197a82`):
/// the owner's pc/Standard league as `redact-pricing` wrote it — 402
/// tabs, 41 characters, 1,977 items, 16 rows — resolved whole. The
/// counts are read against the file, not against themselves.
mod real_scale {
    use std::collections::BTreeSet;
    use std::time::Instant;

    use super::*;
    use crate::real_scale_fixture::{self, OWNERS_TEST_TAB};

    fn counts(pairs: &[(&str, usize)]) -> BTreeMap<String, usize> {
        pairs.iter().map(|(k, n)| (k.to_string(), *n)).collect()
    }

    /// The fixture's one row of a scope, as a target.
    fn the_row(s: &PricingSnapshot, scope: &str) -> PriceTarget {
        let mut all = rows(s, scope);
        assert_eq!(all.len(), 1, "one {scope} row");
        all.pop().unwrap()
    }

    /// The fixture's rows of one scope, as targets.
    fn rows(s: &PricingSnapshot, scope: &str) -> Vec<PriceTarget> {
        s.buyouts
            .iter()
            .filter(|r| r.scope == scope)
            .map(|r| PriceTarget::from_address(&r.scope, &r.key).unwrap())
            .collect()
    }

    /// C69, C81 — every fact of the owner's league has one listing,
    /// every reference resolves within the file, every count sums to
    /// the items, and the game side reads what the file holds: 50
    /// notes, all in the one public tab, are 49 statements and the
    /// dialog's one residue; 16 `~` tab names are 14 prices and the
    /// two lot ratios a tab name cannot carry (T19); 16 rows are 5
    /// applied and 11 naming another league's facts. The resolve's time
    /// is printed, never asserted (C137): 7 ms in a debug build on
    /// 2026-09-07.
    #[test]
    fn c69_at_real_scale_every_fact_has_one_listing_and_the_counts_are_the_files() {
        let s = real_scale_fixture::snapshot();
        let t = Instant::now();
        let r = resolve(&s).unwrap();
        let took = t.elapsed();
        eprintln!("resolve took {took:?}");

        assert_eq!(
            r.listings.len(),
            s.tabs.len() + s.characters.len() + s.items.len()
        );
        assert_eq!(r.counts.containers, s.tabs.len() + s.characters.len());
        assert_eq!(r.counts.items, s.items.len());
        assert_eq!((r.counts.containers, r.counts.items), (443, 1977));
        let containers: BTreeSet<&PriceTarget> = r
            .listings
            .iter()
            .filter(|l| !l.subject.is_item())
            .map(|l| &l.subject.target)
            .collect();
        assert_eq!(containers.len(), r.counts.containers);
        let mut seen = BTreeSet::new();
        for l in &r.listings {
            assert!(
                seen.insert(&l.subject.target),
                "{} listed twice",
                l.subject.target
            );
            assert_eq!(l.chain[0], l.subject.target);
            for c in &l.chain[1..] {
                assert!(
                    containers.contains(c),
                    "{}: chain names {c}",
                    l.subject.target
                );
            }
            if let Some(loc) = &l.subject.location {
                assert!(
                    containers.contains(loc),
                    "{}: sits in {loc}",
                    l.subject.target
                );
                assert!(l.subject.is_item());
                assert_eq!(l.chain.get(1), Some(loc));
            }
        }

        let c = &r.counts;
        let sum = |m: &BTreeMap<String, usize>| m.values().sum::<usize>();
        assert_eq!(sum(&c.by_relation), c.items);
        assert_eq!(sum(&c.by_effective), c.items);
        assert_eq!(sum(&c.by_game_statement), c.items);
        let items = || r.listings.iter().filter(|l| l.subject.is_item());
        for rel in Relation::ALL {
            assert_eq!(
                c.by_relation[rel.as_str()],
                items().filter(|l| l.relation == rel).count(),
                "{rel}"
            );
        }
        for side in ["game", "manual", "none", "unresolved"] {
            assert_eq!(
                c.by_effective[side],
                items().filter(|l| l.effective.side_word() == side).count(),
                "{side}"
            );
        }
        assert_eq!(
            c.by_relation,
            counts(&[
                ("agree", 0),
                ("conflict", 49),
                ("game_only", 0),
                ("manual_only", 39),
                ("none", 1889)
            ])
        );
        assert_eq!(
            c.by_effective,
            counts(&[
                ("game", 49),
                ("manual", 39),
                ("none", 1889),
                ("unresolved", 0)
            ])
        );
        assert_eq!(
            c.by_game_statement,
            counts(&[
                ("exact", 46),
                ("negotiable", 2),
                ("none", 1928),
                ("skip", 1)
            ])
        );
        assert_eq!(c.game_priced, 48);
        assert_eq!(c.invalid_notes, 1);
        assert_eq!(c.residue, 0);
        assert_eq!(c.inherited, 86);
        assert_eq!(c.league_unknown, 0);
        assert!(s.characters.iter().all(|ch| ch.league.is_some()));

        // Against the file: every note sits in the one public tab
        // with a row on it, so each is a statement or the residue.
        let noted = s.items.iter().filter(|i| i.note.is_some()).count();
        assert_eq!(noted, 50);
        assert_eq!(noted, 46 + 2 + 1 + c.invalid_notes);
        let test_tab = real_scale_fixture::owners_test_tab(&s);
        assert!(
            items()
                .filter(|l| l.game.note.is_some())
                .all(|l| l.subject.location.as_ref() == Some(&test_tab))
        );
        assert_eq!(
            s.tabs
                .iter()
                .filter(|t| t.metadata["public"] == Value::Bool(true))
                .count(),
            13
        );
        // The `~` names: 14 prices, none public but one, and the two
        // lot ratios that read as nothing in a tab name (T19).
        let tilde: Vec<&Listing> = r
            .listings
            .iter()
            .filter(|l| l.subject.name.starts_with('~'))
            .collect();
        assert_eq!(tilde.len(), 16);
        let priced: Vec<&Listing> = tilde
            .iter()
            .copied()
            .filter(|l| {
                l.game
                    .tab_name
                    .as_ref()
                    .is_some_and(|t| t.reading.price().is_some())
            })
            .collect();
        assert_eq!(priced.len(), 14);
        assert_eq!((c.priced_tabs, c.priced_tabs_public), (14, 1));
        assert_eq!(
            priced
                .iter()
                .filter(|l| l.game.public == Some(true))
                .count(),
            1
        );
        for l in tilde.iter().filter(|l| !priced.contains(l)) {
            assert!(l.subject.name.contains('/'), "{}", l.subject.name);
            assert!(
                matches!(
                    l.game.tab_name.as_ref().map(|t| &t.reading),
                    Some(GamePrice::Invalid { .. })
                ),
                "{}: {:?}",
                l.subject.name,
                l.game.tab_name
            );
        }

        // The rows: every one accounted for, none unreadable under
        // this build, the unmatched ones naming another league's
        // items (10) and a tab no longer listed (1) — real data.
        let rows = &r.rows;
        assert_eq!(rows.total, s.buyouts.len());
        assert_eq!((rows.total, rows.applied, rows.other_realm), (16, 5, 0));
        assert_eq!(
            rows.total,
            rows.applied + rows.other_realm + rows.unmatched.len() + rows.unreadable.len()
        );
        assert!(rows.unreadable.is_empty(), "{:?}", rows.unreadable);
        assert_eq!(rows.unmatched.len(), 11);
        assert_eq!(
            rows.unmatched
                .iter()
                .filter(|m| matches!(m.from, PriceTarget::Item { .. }))
                .count(),
            10
        );
        assert!(rows.unmatched.iter().all(|m| r.find(&m.from).is_none()));
    }

    /// C70, C80, C81 — the owner's five applied rows land where the
    /// rules say: the tab row covers the test tab's 80 items, 78 by
    /// inheritance and 2 under their own rows, and the 49 game
    /// statements among them decide over it (a note is level 0, the
    /// tab row level 2); the substash row covers its 5 items under a
    /// parent that holds nothing itself and is not public; the
    /// character's `skip` row covers its 3 items. `list`'s default
    /// selection is exactly these 88, in the four containers on their
    /// chains — the substash's parent among them, since its name is
    /// the one C80 reads.
    #[test]
    fn c70_at_real_scale_the_owners_rows_cover_what_the_rules_say() {
        let s = real_scale_fixture::snapshot();
        let r = resolve(&s).unwrap();
        let tab = real_scale_fixture::owners_test_tab(&s);
        let tab_listing = r.find(&tab).unwrap();
        assert_eq!(tab_listing.subject.name, OWNERS_TEST_TAB);
        assert_eq!(tab_listing.game.public, Some(true));
        let own_row = tab_listing.manual.as_ref().unwrap();
        assert!(!own_row.inherited);
        assert_eq!(own_row.from, tab);

        // An ordinary tab's two sets are one (C70).
        let here: Vec<&Listing> = r.items_in(&tab).collect();
        let covered: Vec<&Listing> = r.items_covered_by(&tab).collect();
        assert_eq!(here.len(), 80);
        assert_eq!(
            here.iter().map(|l| &l.subject.target).collect::<Vec<_>>(),
            covered
                .iter()
                .map(|l| &l.subject.target)
                .collect::<Vec<_>>()
        );
        let item_rows: Vec<PriceTarget> = rows(&s, "item")
            .into_iter()
            .filter(|t| r.find(t).is_some())
            .collect();
        assert_eq!(item_rows.len(), 2);
        let mut inherited = 0;
        let mut game_decided = 0;
        for l in &here {
            let m = l.manual.as_ref().expect("a row covers every item here");
            if m.inherited {
                inherited += 1;
                assert_eq!(m.from, tab);
                assert_eq!(l.chain, [l.subject.target.clone(), tab.clone()]);
            } else {
                assert_eq!(m.from, l.subject.target);
                assert!(item_rows.contains(&l.subject.target));
            }
            assert_eq!(l.game.public, Some(true));
            assert!(!l.game.residue);
            match l.game.reading {
                GamePrice::Exact(_) | GamePrice::Negotiable(_) | GamePrice::Skip => {
                    game_decided += 1;
                    assert_eq!(l.relation, Relation::Conflict, "{}", l.why);
                    assert_eq!(l.effective.side, Some(Side::Game), "{}", l.effective.why);
                    assert_eq!(l.effective.from.as_ref(), Some(&l.subject.target));
                }
                GamePrice::Invalid { .. } => panic!("a tab name read as a price"),
                GamePrice::None => {
                    assert_eq!(l.relation, Relation::ManualOnly, "{}", l.why);
                    assert_eq!(l.effective.side, Some(Side::Manual), "{}", l.effective.why);
                    assert_eq!(l.effective.from.as_ref(), Some(&m.from));
                }
            }
        }
        assert_eq!((inherited, game_decided), (78, 49));
        // The dialog's residue (T18): the note is shown, reads invalid,
        // has no effect, and the tab row applies.
        let residue: Vec<&Listing> = here
            .iter()
            .copied()
            .filter(|l| {
                l.game
                    .note
                    .as_ref()
                    .is_some_and(|n| matches!(n.reading, GamePrice::Invalid { .. }))
            })
            .collect();
        assert_eq!(residue.len(), 1);
        assert_eq!(residue[0].game.note.as_ref().unwrap().text, "~price  chaos");
        assert_eq!(residue[0].game.reading, GamePrice::None);
        assert_eq!(residue[0].relation, Relation::ManualOnly);
        assert_eq!(residue[0].effective.from.as_ref(), Some(&tab));

        // The substash row (C70, C80).
        let substash = the_row(&s, "substash");
        let PriceTarget::Substash { realm, parent, .. } = &substash else {
            unreachable!()
        };
        let parent = PriceTarget::Tab {
            realm: *realm,
            id: parent.clone(),
        };
        let sub_listing = r.find(&substash).unwrap();
        let parent_listing = r.find(&parent).unwrap();
        assert_eq!(parent_listing.subject.name, "Maps (Remove-only)");
        assert_eq!(parent_listing.subject.tab_type.as_deref(), Some("MapStash"));
        assert_ne!(parent_listing.game.public, Some(true));
        assert!(parent_listing.manual.is_none());
        let in_sub: Vec<&Listing> = r.items_in(&substash).collect();
        assert_eq!(in_sub.len(), 5);
        for l in &in_sub {
            let m = l.manual.as_ref().unwrap();
            assert!(m.inherited);
            assert_eq!(m.from, substash);
            assert_eq!(
                l.chain,
                [l.subject.target.clone(), substash.clone(), parent.clone()]
            );
            assert_eq!(l.relation, Relation::ManualOnly);
            assert_eq!(l.effective.side, Some(Side::Manual));
            assert_eq!(l.game.reading, GamePrice::None);
            assert_eq!(
                l.game.substash_name.as_deref(),
                Some(sub_listing.subject.name.as_str())
            );
        }
        assert_eq!(r.items_in(&parent).count(), 0);
        let below: Vec<&Listing> = r.items_covered_by(&parent).collect();
        assert!(in_sub.iter().all(|l| below.contains(l)));
        assert!(below.len() > in_sub.len(), "the parent's other substashes");

        // The character's `skip` row.
        let character = the_row(&s, "character");
        let in_char: Vec<&Listing> = r.items_in(&character).collect();
        assert_eq!(in_char.len(), 3);
        for l in &in_char {
            let m = l.manual.as_ref().unwrap();
            assert!(m.inherited);
            assert_eq!(m.from, character);
            assert_eq!(l.chain, [l.subject.target.clone(), character.clone()]);
            assert_eq!(l.effective.kind, "skip");
            assert_eq!(l.effective.side, Some(Side::Manual));
            assert_eq!(l.game.public, None);
            assert!(!l.subject.league_unknown);
        }
        assert_eq!(r.counts.inherited, 78 + 5 + 3);

        // `list`'s default selection, every one of it (C53: the
        // default lists ten), and the containers it carries.
        let view = r
            .list_view(ListFilter {
                expand: true,
                ..ListFilter::default()
            })
            .unwrap();
        assert_eq!(view.items.len(), 80 + 5 + 3);
        let mut containers: Vec<&PriceTarget> =
            view.containers.iter().map(|c| &c.subject.target).collect();
        containers.sort();
        let mut expected = vec![&tab, &substash, &parent, &character];
        expected.sort();
        assert_eq!(containers, expected);
        assert_eq!(view.items_on_record, 1977);
        let show = r.show_view(&parent).unwrap();
        assert!(show.items_here.is_empty());
        assert_eq!(show.items_covered_below.len(), below.len());
        let show = r.show_view(&tab).unwrap();
        assert_eq!(show.items_here.len(), 80);
        assert!(show.items_covered_below.is_empty());
    }

    /// C53, C70 — the report is its own contract at scale (1.8 MB of
    /// JSON re-reads exactly, `status` is the counts alone), and
    /// `show` of every one of the 443 containers holds its two sets
    /// (C70): what sits here plus what its children hold is what a
    /// row on it covers; a folder holds nothing itself; a substash
    /// and a character have nothing below.
    #[test]
    fn c53_at_real_scale_the_report_round_trips_and_every_container_shows_its_two_sets() {
        let s = real_scale_fixture::snapshot();
        let r = resolve(&s).unwrap();
        let text = serde_json::to_string(&r).unwrap();
        let back: ListingReport = serde_json::from_str(&text).unwrap();
        assert_eq!(back, r);
        assert_eq!(r.summary().counts, r.counts);
        assert!(r.summary().listings.is_empty());

        let t = Instant::now();
        let mut below_total = 0;
        for c in r.listings.iter().filter(|l| !l.subject.is_item()) {
            let target = &c.subject.target;
            let v = r.show_view(target).unwrap();
            assert_eq!(v.listing, *c);
            assert!(v.container.is_none());
            assert_eq!(
                v.items_here.len() + v.items_covered_below.len(),
                r.items_covered_by(target).count(),
                "{target}"
            );
            assert!(
                v.items_here
                    .iter()
                    .all(|l| l.subject.location.as_ref() == Some(target))
            );
            assert!(
                v.items_covered_below
                    .iter()
                    .all(|l| l.subject.location.as_ref() != Some(target))
            );
            if c.subject.tab_type.as_deref() == Some(FOLDER) {
                assert!(v.items_here.is_empty(), "{target}: a folder holds nothing");
            }
            if !matches!(target, PriceTarget::Tab { .. }) {
                assert!(v.items_covered_below.is_empty(), "{target}");
            }
            below_total += v.items_covered_below.len();
        }
        // What sits below anything is counted once per container
        // above it: an item in a substash under a folder's tab is
        // below both.
        assert!(below_total > 0);
        let took = t.elapsed();
        eprintln!("443 show views took {took:?}");
    }
}
