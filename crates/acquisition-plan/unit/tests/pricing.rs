use super::*;

/// The refusal's quoting, as a frontend without a shell would give it.
fn plain(word: &str) -> String {
    word.to_string()
}

/// One account's facts in memory: pc Standard's tab `t1` fetched with
/// two unpriced items, `i1` and `i2`; and its intent file, empty.
fn priced() -> (Store, Annotations) {
    use acquisition_store::Endpoint;
    let mut store = Store::open_memory().unwrap();
    let item = |id: &str, x: i64| {
        json!({ "id": id, "name": "", "typeLine": "Iron Ring", "baseType": "Iron Ring",
                    "frameType": 2, "x": x, "y": 0, "w": 1, "h": 1, "inventoryId": "Stash1" })
    };
    let pc = json!({ "realm": "pc", "league": "Standard" });
    store
        .record(
            &Endpoint::Profile,
            &json!({}),
            200,
            &json!({ "uuid": "u-1", "name": "A" }),
            1,
        )
        .unwrap();
    store
            .record(
                &Endpoint::Stashes {
                    realm: "pc".into(),
                    league: "Standard".into(),
                },
                &pc,
                200,
                &json!({ "stashes": [{ "id": "t1", "name": "One", "type": "PremiumStash", "index": 0 }] }),
                10,
            )
            .unwrap();
    store
        .record(
            &Endpoint::Stash {
                realm: "pc".into(),
                league: "Standard".into(),
                id: "t1".into(),
                sub: None,
            },
            &pc,
            200,
            &json!({ "stash": { "id": "t1", "name": "One", "type": "PremiumStash", "index": 0,
                                    "items": [item("i1", 0), item("i2", 1)] } }),
            20,
        )
        .unwrap();
    (store, Annotations::open_memory_for("u-1").unwrap())
}

/// `price list --limit N` holds the first N in listing order and says
/// how many the selection holds and how many it lists; the containers
/// are the listed items' alone (the release seat: a league's conflicts
/// were 49 to 67 KB, cut by a harness after the first item).
#[test]
fn a_list_is_bounded_by_its_limit_and_counts_the_selection() {
    let (store, a) = priced();
    let args = |limit| ListArgs {
        effective: Some("none".into()),
        limit,
        ..ListArgs::default()
    };
    let whole = list(&store, &a, None, None, &args(None), "a", plain).unwrap();
    assert_eq!((whole.total, whole.listed, whole.items.len()), (2, 2, 2));
    let limited = list(&store, &a, None, None, &args(Some(1)), "a", plain).unwrap();
    assert_eq!((limited.total, limited.listed), (2, 1));
    assert_eq!(limited.items, whole.items[..1].to_vec());
    assert_eq!(limited.filter.limit, Some(1));
    let chain = serde_json::to_value(&limited.items[0]).unwrap()["chain"].clone();
    for c in &limited.containers {
        let target = serde_json::to_value(c).unwrap()["subject"]["target"].clone();
        assert!(chain.as_array().unwrap().contains(&target), "{target}");
    }
}

/// A write refused for its revision is `conflict` with the row's
/// revision now, in the verb's words, never the store's: a stale
/// revision over a live row; a revision named over a cleared one; a
/// clear of a cleared row. A target the facts do not hold is
/// `not_found`.
/// C151 — a stale write over a row this build cannot read is a
/// `conflict` with the row's revision, in the verb's words naming the
/// `--if-revision` that goes through, never the store's generic
/// refusal (the second review of #63, its fourth finding); the write
/// naming the revision then lands. The damage is staged through a
/// second connection on a file-backed intent store.
#[test]
fn c151_a_stale_write_over_an_unreadable_row_is_a_conflict_naming_its_revision() {
    let (store, _) = priced();
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let mut a = Annotations::open_for(&dir, "u-1").unwrap();
    let via = Provenance::via("test");
    let refused = |e: anyhow::Error| {
        Refused::find(&e)
            .cloned()
            .unwrap_or_else(|| panic!("{e:#}"))
    };
    set(
        &store,
        &mut a,
        "item/i1",
        "exact",
        Some("2"),
        Some("divine"),
        Expect::Current,
        &via,
        "a",
        plain,
    )
    .unwrap();
    rusqlite::Connection::open(acquisition_store::annotations_path(&dir, "u-1"))
        .unwrap()
        .execute(
            "UPDATE annotations SET value = '{nope' WHERE key = 'i1'",
            [],
        )
        .unwrap();

    let r = refused(clear(&mut a, "item/i1", Expect::Exactly(6), &via, "a", plain).unwrap_err());
    assert!(
        matches!(
            r.kind,
            RefusedKind::Conflict {
                revision: Some(1),
                current: None
            }
        ),
        "{:?}",
        r.kind
    );
    assert!(
        r.message
            .starts_with("item/i1 holds a value this build cannot read (revision 1: ")
            && r.message
                .contains("`--if-revision 1` clears it deliberately"),
        "{}",
        r.message
    );
    assert_eq!(r.fields()["kind"], json!("conflict"));
    assert_eq!(r.fields()["revision"], json!(1));
    let r = refused(
        set(
            &store,
            &mut a,
            "item/i1",
            "exact",
            Some("7"),
            Some("divine"),
            Expect::Exactly(6),
            &via,
            "a",
            plain,
        )
        .unwrap_err(),
    );
    assert!(
        r.message
            .contains("`--if-revision 1` replaces it deliberately"),
        "{}",
        r.message
    );
    // Naming the revision goes through.
    clear(&mut a, "item/i1", Expect::Exactly(1), &via, "a", plain).unwrap();
    assert_eq!(a.get("item", "i1", BUYOUT_KIND).unwrap(), None);
}

/// A revision of 0 is the create-only form under either frontend's
/// default (#101): no row carries it, so the CLI's `--if-revision 0` and
/// the MCP's `if_revision: 0` both create where no price stands and are
/// refused naming the stored revision where one does.
#[test]
fn a_revision_of_0_creates_only_where_no_price_stands_under_either_default() {
    let (store, mut a) = priced();
    let via = Provenance::via("test");
    for default in [Expect::Current, Expect::Create] {
        assert_eq!(Expect::or(Some(0), default), Expect::Create);
        assert_eq!(Expect::or(Some(4), default), Expect::Exactly(4));
        assert_eq!(Expect::or(None, default), default);
    }
    let mut zero = |a: &mut Annotations| {
        set(
            &store,
            a,
            "item/i1",
            "exact",
            Some("2"),
            Some("divine"),
            Expect::or(Some(0), Expect::Current),
            &via,
            "a",
            plain,
        )
    };
    assert_eq!(zero(&mut a).unwrap().write.revision, 1);
    let r = Refused::find(&zero(&mut a).unwrap_err()).cloned().unwrap();
    assert!(
        matches!(
            r.kind,
            RefusedKind::Conflict {
                revision: Some(1),
                ..
            }
        ),
        "{r:?}"
    );
}

#[test]
fn a_write_refused_for_its_revision_is_a_conflict_naming_it_in_the_verbs_words() {
    let (store, mut a) = priced();
    let via = Provenance::via("test");
    let refused = |e: anyhow::Error| {
        Refused::find(&e)
            .cloned()
            .unwrap_or_else(|| panic!("{e:#}"))
    };
    set(
        &store,
        &mut a,
        "item/i1",
        "exact",
        Some("2"),
        Some("divine"),
        Expect::Current,
        &via,
        "a",
        plain,
    )
    .unwrap();
    set(
        &store,
        &mut a,
        "item/i1",
        "exact",
        Some("3"),
        Some("divine"),
        Expect::Exactly(1),
        &via,
        "a",
        plain,
    )
    .unwrap();
    let r = refused(
        set(
            &store,
            &mut a,
            "item/i1",
            "exact",
            Some("7"),
            Some("divine"),
            Expect::Exactly(1),
            &via,
            "a",
            plain,
        )
        .unwrap_err(),
    );
    assert!(matches!(
        r.kind,
        RefusedKind::Conflict {
            revision: Some(2),
            ..
        }
    ));
    assert_eq!(
        r.message,
        "item/i1 has a price at revision 2 (re-read and retry)"
    );
    let r = refused(clear(&mut a, "item/i1", Expect::Exactly(1), &via, "a", plain).unwrap_err());
    assert!(matches!(
        r.kind,
        RefusedKind::Conflict {
            revision: Some(2),
            ..
        }
    ));

    clear(&mut a, "item/i1", Expect::Exactly(2), &via, "a", plain).unwrap();
    let r = refused(
        set(
            &store,
            &mut a,
            "item/i1",
            "exact",
            Some("1"),
            Some("divine"),
            Expect::Exactly(99),
            &via,
            "a",
            plain,
        )
        .unwrap_err(),
    );
    assert!(matches!(
        r.kind,
        RefusedKind::Conflict {
            revision: Some(3),
            ..
        }
    ));
    assert_eq!(
        r.message,
        "item/i1's price was cleared (revision 3); a new price names no revision"
    );
    let r = refused(clear(&mut a, "item/i1", Expect::Current, &via, "a", plain).unwrap_err());
    assert!(matches!(
        r.kind,
        RefusedKind::Conflict {
            revision: Some(3),
            ..
        }
    ));
    assert!(
        r.message
            .starts_with("item/i1's price was cleared (revision 3)"),
        "{}",
        r.message
    );
    for word in ["annotation", "buyout"] {
        assert!(!r.message.contains(word), "{}", r.message);
    }

    let e = show(
        &store,
        &a,
        "item/i-nowhere",
        None,
        None,
        Bound::default(),
        "a",
        plain,
    )
    .unwrap_err();
    assert!(matches!(refused(e).kind, RefusedKind::NotFound(_)));
}

/// A row this build cannot read (a newer build's value) is never
/// replaced or cleared blind: both refuse naming its revision and the
/// reviewed path, and the row is untouched; a named revision replaces
/// it deliberately.
#[test]
fn a_row_this_build_cannot_read_is_never_written_blind() {
    #[derive(Serialize)]
    #[serde(transparent)]
    struct NewerBuyout(Value);
    impl IntentValue for NewerBuyout {
        const KIND: &'static str = "buyout";
        const VERSION: i64 = 99;
        fn parse(value: &Value) -> Result<Self, String> {
            Ok(NewerBuyout(value.clone()))
        }
    }
    let (store, mut a) = priced();
    let via = Provenance::via("test");
    let newer = json!({ "version": 2, "type": "auction", "reserve": "5 chaos" });
    a.put::<NewerBuyout>("item", "i1", &newer, None, &Provenance::via("future"))
        .unwrap();
    let said = [
        "item/i1 holds a value this build cannot read (revision 1: buyout declares version 2, newer than this build's v1)",
        "`--if-revision 1` replaces it deliberately",
    ];
    let blind = [
        set(
            &store,
            &mut a,
            "item/i1",
            "skip",
            None,
            None,
            Expect::Current,
            &via,
            "a",
            plain,
        )
        .map(|_| ())
        .unwrap_err(),
        clear(&mut a, "item/i1", Expect::Current, &via, "a", plain)
            .map(|_| ())
            .unwrap_err(),
    ];
    for e in blind {
        let e = format!("{e:#}");
        for words in said {
            assert!(e.contains(words), "{words}: {e}");
        }
    }
    let row = a.get("item", "i1", BUYOUT_KIND).unwrap().unwrap();
    assert_eq!((row.revision, &row.value), (1, &newer), "untouched");
    let written = set(
        &store,
        &mut a,
        "item/i1",
        "skip",
        None,
        None,
        Expect::Exactly(1),
        &via,
        "a",
        plain,
    )
    .unwrap();
    assert_eq!(written.write.prior.map(|r| r.revision), Some(1));
}

/// A tab or substash address carries its realm; a realm given beside
/// it must agree; an item or character takes the given realm, default
/// pc.
#[test]
fn the_realm_comes_from_the_address_and_a_disagreement_refuses() {
    let xbox = PriceTarget::Tab {
        realm: Realm::Xbox,
        id: "t".into(),
    };
    let item = PriceTarget::Item { id: "i".into() };
    assert_eq!(realm_of(&[&xbox], None).unwrap(), Realm::Xbox);
    assert_eq!(realm_of(&[&xbox], Some(Realm::Xbox)).unwrap(), Realm::Xbox);
    assert_eq!(realm_of(&[&item], None).unwrap(), Realm::Pc);
    assert_eq!(realm_of(&[&item], Some(Realm::Sony)).unwrap(), Realm::Sony);
    assert_eq!(realm_of(&[&item, &xbox], None).unwrap(), Realm::Xbox);
    let err = realm_of(&[&xbox], Some(Realm::Pc)).unwrap_err().to_string();
    assert!(
        err.contains("tab/xbox/t is a xbox address; pc was given"),
        "{err}"
    );
    let pc = PriceTarget::Tab {
        realm: Realm::Pc,
        id: "u".into(),
    };
    assert!(realm_of(&[&xbox, &pc], None).is_err());
}

/// C35 — `set` reads everything it reports before the write and
/// nothing after it: a facts read that fails refuses naming the facts
/// before anything changes, so no write lands whose receipt was lost.
#[test]
fn a_set_whose_facts_cannot_be_read_refuses_before_the_write() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = dir.join("facts.db");
    let store = Store::open(&path).unwrap();
    // The facts made unreadable under the open store: their item table
    // gone, through a second connection.
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch("DROP TABLE items")
        .unwrap();
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let via = Provenance::via("test");
    let first = set(
        &store,
        &mut a,
        "item/i1",
        "exact",
        Some("30"),
        Some("chaos"),
        Expect::Create,
        &via,
        "a",
        plain,
    );
    let err = first.unwrap_err();
    assert!(format!("{err:#}").contains("reading the facts"), "{err:#}");
    assert!(
        a.get("item", "i1", BUYOUT_KIND).unwrap().is_none(),
        "a create landed"
    );
    a.put::<Buyout>(
        "item",
        "i1",
        &read_buyout("exact", Some("30"), Some("chaos"))
            .unwrap()
            .to_value(),
        None,
        &via,
    )
    .unwrap();
    for expect in [Expect::Exactly(1), Expect::Current] {
        let err = set(
            &store, &mut a, "item/i1", "skip", None, None, expect, &via, "a", plain,
        )
        .unwrap_err();
        assert!(format!("{err:#}").contains("reading the facts"), "{err:#}");
        let row = a.get("item", "i1", BUYOUT_KIND).unwrap().unwrap();
        assert_eq!(row.revision, 1, "{expect:?}: a replace landed");
    }
}

/// `set`'s value words are one grammar with the value's own: the
/// C67 type words and the game's, an amount tolerated as a human
/// types it and stored canonical, and every refusal in the value's
/// words — a priced type without its two words, an amount on `skip`,
/// an alias that is not a tag.
#[test]
fn set_reads_the_value_words_through_the_value_s_own_parse() {
    let exact = |amount: &str, currency: &str| {
        Buyout::Exact(crate::price::Price {
            amount: amount.parse().unwrap(),
            currency: currency.into(),
        })
    };
    assert_eq!(
        read_buyout("exact", Some("12.50"), Some("chaos")).unwrap(),
        exact("12.5", "chaos")
    );
    assert_eq!(
        read_buyout("~price", Some("1"), Some("divine")).unwrap(),
        exact("1", "divine")
    );
    assert_eq!(
        read_buyout("b/o", Some("1/5"), Some("divine"))
            .unwrap()
            .to_string(),
        "1/5 divine b/o"
    );
    assert_eq!(read_buyout("skip", None, None).unwrap(), Buyout::Skip);
    assert_eq!(read_buyout("~skip", None, None).unwrap(), Buyout::Skip);
    assert_eq!(
        read_buyout("no-price", None, None).unwrap(),
        Buyout::NoPrice
    );
    for (words, said) in [
        (("bogus", None, None), "is not a price type"),
        (
            ("exact", Some("1"), None),
            "takes an amount and a currency tag",
        ),
        (
            ("negotiable", None, None),
            "takes an amount and a currency tag",
        ),
        (("skip", Some("1"), Some("chaos")), "carries no amount"),
        (
            ("exact", Some("1"), Some("exa")),
            "resolves to tag \"exalted\"",
        ),
        (
            ("exact", Some("1"), Some("c")),
            "is not in currency table v1",
        ),
        (
            ("exact", Some("1.23456"), Some("chaos")),
            "four fractional digits",
        ),
        (("exact", Some("0"), Some("chaos")), "positive"),
    ] {
        let err = read_buyout(words.0, words.1, words.2).unwrap_err();
        assert!(err.to_string().contains(said), "{words:?}: {err}");
    }
}

/// Round 4 of `roadmap/briefs/2g-build.md`: a conflict carries the
/// live row it was refused over as it stands — `show`'s `manual` less
/// the age words an answer adds (round 6's fix round) — so an
/// agent reads the current value without a second call; over a
/// cleared row `current` is absent and `revision` the tombstone's,
/// and where no row was ever written `revision` stays null (C11).
#[test]
fn a_conflict_carries_the_live_row_as_show_prints_it() {
    let (store, mut a) = priced();
    let via = Provenance::via("test");
    let write = |a: &mut Annotations, expect| {
        set(
            &store,
            a,
            "item/i1",
            "exact",
            Some("3"),
            Some("divine"),
            expect,
            &via,
            "a",
            plain,
        )
    };
    write(&mut a, Expect::Current).unwrap();
    let shown = show(
        &store,
        &a,
        "item/i1",
        None,
        None,
        Bound::default(),
        "a",
        plain,
    )
    .unwrap()
    .listing
    .manual
    .unwrap();
    // less the words C53 says beside its `updated_at` on `show` (step
    // 2g round 6), which a refusal does not carry
    let shown = ManualSide {
        updated_at_age: None,
        ..shown
    };
    for e in [
        write(&mut a, Expect::Exactly(9)).unwrap_err(),
        write(&mut a, Expect::Create).unwrap_err(),
        clear(&mut a, "item/i1", Expect::Create, &via, "a", plain).unwrap_err(),
    ] {
        let r = Refused::find(&e).unwrap();
        let fields = r.fields();
        assert_eq!(fields["revision"], json!(1), "{e:#}");
        assert_eq!(fields["current"], json!(shown), "{e:#}");
    }
    clear(&mut a, "item/i1", Expect::Exactly(1), &via, "a", plain).unwrap();
    let e = write(&mut a, Expect::Exactly(9)).unwrap_err();
    let fields = Refused::find(&e).unwrap().fields();
    assert_eq!(fields["revision"], json!(2));
    assert!(!fields.contains_key("current"), "{fields:?}");
    let e = clear(&mut a, "item/i2", Expect::Current, &via, "a", plain).unwrap_err();
    let fields = Refused::find(&e).unwrap().fields();
    assert_eq!(fields["revision"], Value::Null);
    assert!(!fields.contains_key("current"), "{fields:?}");
}

/// Round 4: a target the facts do not hold is refused with the target
/// and the scope the verb resolved — the account it ran as, the realm
/// the address carries or pc, the league named or Standard — and the
/// leagues that do hold it, from which the offers are made; every
/// command its words print carries that scope.
#[test]
fn a_not_found_carries_its_target_and_the_scope_it_was_looked_for_in() {
    let (store, a) = priced();
    let missing = |target: &str, league: Option<&str>| {
        let e = show(
            &store,
            &a,
            target,
            None,
            league,
            Bound::default(),
            "a",
            plain,
        )
        .unwrap_err();
        let r = Refused::find(&e).unwrap().clone();
        match r.kind {
            RefusedKind::NotFound(m) => (*m, r.message),
            other => panic!("{other:?}"),
        }
    };
    let (m, said) = missing("tab/xbox/nope", None);
    assert_eq!(
        m,
        Missing {
            target: read_target("tab/xbox/nope").unwrap(),
            account: "a".into(),
            realm: Realm::Xbox,
            league: "Standard".into(),
            elsewhere: vec![],
            own_row: None,
        }
    );
    assert!(
        said.ends_with(
            "`acq search --account a --realm xbox` finds an item's id, \
                 `acq tabs --account a --realm xbox --league Standard` a tab's"
        ),
        "{said}"
    );
    let (m, said) = missing("item/i1", Some("Other"));
    assert_eq!(
        (m.realm, m.league.as_str(), m.elsewhere.as_slice()),
        (
            Realm::Pc,
            "Other",
            &[(Realm::Pc, "Standard".to_string())][..]
        )
    );
    assert!(
        said.contains("the facts hold it in Standard: `--league` reads it there"),
        "{said}"
    );
    // an item's address names no realm: one held in another is found
    // there, its realm said
    let e = show(
        &store,
        &a,
        "item/i1",
        Some(Realm::Xbox),
        None,
        Bound::default(),
        "a",
        plain,
    )
    .unwrap_err();
    let r = Refused::find(&e).unwrap();
    let RefusedKind::NotFound(m) = &r.kind else {
        panic!("{r:?}")
    };
    assert_eq!(m.elsewhere, vec![(Realm::Pc, "Standard".to_string())]);
    assert!(
        r.message
            .contains("the facts hold it in pc/Standard: `--realm` and `--league` read it there"),
        "{}",
        r.message
    );
}

/// Q6 of the 2g design, the teaching refusal, as round 5's fix round
/// narrowed it (its review's finding 2): a write naming a revision the
/// target's own row does not stand at is refused as a `conflict`
/// naming the target's own revision. `price_show`'s `manual.revision`
/// is a container's when the row is inherited, and a write over the
/// item naming it is refused naming the item's own revision — none
/// while it has no row of its own, its own once it has one — never
/// landed over the container's row. Revisions count per target, so
/// where the item's own row stands at the container's number the
/// write lands: a revision cannot say where it was read. The
/// refusal's read-back carries the account the write ran as (round 5
/// of the 2g build: a write resolves no realm or league, so the
/// account alone).
#[test]
fn a_write_naming_a_containers_revision_is_a_conflict_naming_the_targets_own() {
    let (store, mut a) = priced();
    let via = Provenance::via("test");
    let refused = |e: anyhow::Error| {
        Refused::find(&e)
            .cloned()
            .unwrap_or_else(|| panic!("{e:#}"))
    };
    fn quoted(w: &str) -> String {
        format!("'{w}'")
    }
    for _ in 0..3 {
        let rev = show(
            &store,
            &a,
            "tab/pc/t1",
            None,
            None,
            Bound::default(),
            "a",
            plain,
        )
        .unwrap()
        .listing
        .manual
        .map(|m| m.revision);
        set(
            &store,
            &mut a,
            "tab/pc/t1",
            "exact",
            Some("1"),
            Some("chaos"),
            Expect::or(rev, Expect::Create),
            &via,
            "a",
            plain,
        )
        .unwrap();
    }
    let manual = show(
        &store,
        &a,
        "item/i1",
        None,
        None,
        Bound::default(),
        "a",
        plain,
    )
    .unwrap()
    .listing
    .manual
    .unwrap();
    assert!(manual.inherited);
    assert_eq!(manual.revision, 3, "the tab's");
    let over = |a: &mut Annotations| {
        refused(
            set(
                &store,
                a,
                "item/i1",
                "skip",
                None,
                None,
                Expect::Exactly(manual.revision),
                &via,
                "A#1",
                quoted,
            )
            .unwrap_err(),
        )
    };
    let r = over(&mut a);
    assert!(
        matches!(
            r.kind,
            RefusedKind::Conflict {
                revision: None,
                current: None
            }
        ),
        "{r:?}"
    );
    assert_eq!(
        r.message,
        "item/i1 has no price of its own; a new price names no revision"
    );
    set(
        &store,
        &mut a,
        "item/i1",
        "skip",
        None,
        None,
        Expect::Create,
        &via,
        "a",
        plain,
    )
    .unwrap();
    let r = over(&mut a);
    assert!(
        matches!(
            r.kind,
            RefusedKind::Conflict {
                revision: Some(1),
                ..
            }
        ),
        "{r:?}"
    );
    // The equal-number case: the item's own row moved to 3, the tab's
    // number, and a write naming 3 lands over the item's own row —
    // the number is the item's own revision, whatever it was read from.
    for revision in [1, 2] {
        set(
            &store,
            &mut a,
            "item/i1",
            "skip",
            None,
            None,
            Expect::Exactly(revision),
            &via,
            "a",
            plain,
        )
        .unwrap();
    }
    let landed = set(
        &store,
        &mut a,
        "item/i1",
        "no_price",
        None,
        None,
        Expect::Exactly(manual.revision),
        &via,
        "a",
        plain,
    )
    .unwrap();
    assert_eq!(
        landed.write.prior.map(|row| (row.key, row.revision)),
        Some(("i1".to_string(), 3)),
        "the item's own row replaced, never the tab's"
    );
    // the write refusals' read-back: `price show` as the account
    let r = refused(clear(&mut a, "item/i2", Expect::Current, &via, "A#1", quoted).unwrap_err());
    assert!(
        r.message
            .contains("`acq price show --account 'A#1' 'item/i2'` reads what covers it"),
        "{}",
        r.message
    );
}

/// Round 5's fix round, its review's finding 1: a price row at the
/// last revision an `i64` holds is refused in the price's words,
/// never the store's, and nothing is written.
#[test]
fn a_price_at_the_last_revision_is_refused_in_the_prices_words() {
    let (store, _) = priced();
    let dir = tempfile::tempdir().unwrap();
    let mut a = Annotations::open_for(dir.path(), "u-1").unwrap();
    let via = Provenance::via("test");
    set(
        &store,
        &mut a,
        "item/i1",
        "skip",
        None,
        None,
        Expect::Create,
        &via,
        "a",
        plain,
    )
    .unwrap();
    rusqlite::Connection::open(acquisition_store::annotations_path(dir.path(), "u-1"))
        .unwrap()
        .execute("UPDATE annotations SET revision = ?1", [i64::MAX])
        .unwrap();
    let said = format!(
        "item/i1's price is at revision {}, the last a row can take: nothing was written",
        i64::MAX
    );
    let e = set(
        &store,
        &mut a,
        "item/i1",
        "no_price",
        None,
        None,
        Expect::Current,
        &via,
        "a",
        plain,
    )
    .unwrap_err();
    assert_eq!(format!("{e:#}"), said);
    let e = clear(&mut a, "item/i1", Expect::Current, &via, "a", plain).unwrap_err();
    assert_eq!(format!("{e:#}"), said);
    let row = a.get("item", "i1", BUYOUT_KIND).unwrap().unwrap();
    assert_eq!(
        (row.revision, row.value),
        (i64::MAX, json!({ "version": 1, "type": "skip" }))
    );
}
