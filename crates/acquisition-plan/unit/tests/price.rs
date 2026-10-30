use super::*;
use acquisition_store::{AnnotationError, Annotations, Provenance, ValueError, check_value};
use serde_json::json;

fn exact(amount: &str, currency: &str) -> Buyout {
    Buyout::Exact(Price {
        amount: amount.parse().unwrap(),
        currency: currency.into(),
    })
}

/// A `buyout` as an older build stored it: the door's parse and the
/// round-trip, no writer's rule — how a row citing a since-retired
/// tag got there.
#[derive(Serialize)]
#[serde(transparent)]
struct StoredBuyout(Value);

impl IntentValue for StoredBuyout {
    const KIND: &'static str = BUYOUT_KIND;
    const VERSION: i64 = BUYOUT_VERSION;
    fn parse(value: &Value) -> Result<Self, String> {
        Buyout::parse(value)?;
        Ok(StoredBuyout(value.clone()))
    }
}

/// C67 — the target's key carries the realm for tabs and substashes,
/// and every address this renders parses back to the same target.
#[test]
fn c67_the_target_key_carries_the_realm_and_round_trips() {
    let targets = [
        (
            PriceTarget::Item {
                id: "abc123".into(),
            },
            "item",
            "abc123",
        ),
        (
            PriceTarget::Character {
                id: "deadbeef".into(),
            },
            "character",
            "deadbeef",
        ),
        (
            PriceTarget::Tab {
                realm: Realm::Pc,
                id: "t1".into(),
            },
            "tab",
            "pc/t1",
        ),
        (
            PriceTarget::Tab {
                realm: Realm::Xbox,
                id: "t1".into(),
            },
            "tab",
            "xbox/t1",
        ),
        (
            PriceTarget::Substash {
                realm: Realm::Pc,
                parent: "m1".into(),
                id: "s9".into(),
            },
            "substash",
            "pc/m1/s9",
        ),
    ];
    for (target, scope, key) in targets {
        assert_eq!(
            target.address().unwrap(),
            (scope, key.to_string()),
            "{target}"
        );
        assert_eq!(PriceTarget::from_address(scope, key).unwrap(), target);
        assert_eq!(target.to_string(), format!("{scope}/{key}"));
    }
    // The same tab id under two realms is two rows (C58).
    let pc = PriceTarget::Tab {
        realm: Realm::Pc,
        id: "t1".into(),
    };
    let xbox = PriceTarget::Tab {
        realm: Realm::Xbox,
        id: "t1".into(),
    };
    assert_ne!(pc.address().unwrap(), xbox.address().unwrap());
    // pc is spelled out in the key: the wire's omission is not a key rule.
    assert!(pc.address().unwrap().1.starts_with("pc/"));
}

/// C67 — an address that could not be read back is refused at render,
/// and a key that does not fit its scope is refused at parse.
#[test]
fn c67_a_target_that_cannot_round_trip_is_refused() {
    for bad in [
        PriceTarget::Item { id: "".into() },
        PriceTarget::Item { id: "a/b".into() },
        PriceTarget::Tab {
            realm: Realm::Pc,
            id: "".into(),
        },
        PriceTarget::Substash {
            realm: Realm::Pc,
            parent: "p/q".into(),
            id: "s".into(),
        },
    ] {
        assert!(
            matches!(bad.address(), Err(TargetError::MalformedComponent { .. })),
            "{bad:?}"
        );
    }
    assert_eq!(
        PriceTarget::from_address("tab", "t1"),
        Err(TargetError::MalformedKey {
            scope: "tab",
            key: "t1".into()
        })
    );
    assert_eq!(
        PriceTarget::from_address("tab", "ps5/t1"),
        Err(TargetError::UnknownRealm {
            realm: "ps5".into()
        })
    );
    assert!(matches!(
        PriceTarget::from_address("item", "a/b"),
        Err(TargetError::MalformedKey { .. })
    ));
    assert!(matches!(
        PriceTarget::from_address("substash", "pc//s"),
        Err(TargetError::MalformedComponent { .. })
    ));
    assert_eq!(
        PriceTarget::from_address("account", ""),
        Err(TargetError::UnknownScope {
            scope: "account".into()
        })
    );
}

/// C67 — the amount's grammar: a decimal of at most four places (what
/// the game writes, T10) or an unreduced lot pair, canonical text,
/// structural equality, more digits refused.
#[test]
fn c67_the_amount_is_a_four_place_decimal_or_an_unreduced_lot_pair() {
    let dec = |t: u64| Amount::Decimal { ten_thousandths: t };
    let ratio = |w: u64, l: u64| Amount::Ratio { wanted: w, lot: l };
    for (text, amount, canonical) in [
        ("1", dec(10_000), "1"),
        ("150", dec(1_500_000), "150"),
        ("12345", dec(123_450_000), "12345"),
        ("2.5", dec(25_000), "2.5"),
        ("2.50", dec(25_000), "2.5"),
        ("2.05", dec(20_500), "2.05"),
        ("0.5", dec(5_000), "0.5"),
        ("0.01", dec(100), "0.01"),
        ("0.0001", dec(1), "0.0001"),
        // The fixture's own decimals (price-notes-2026-09-04.txt).
        ("999.1", dec(9_991_000), "999.1"),
        ("999.12", dec(9_991_200), "999.12"),
        ("999.123", dec(9_991_230), "999.123"),
        ("999.1234", dec(9_991_234), "999.1234"),
        ("999.1230", dec(9_991_230), "999.123"),
        ("22/10", ratio(22, 10), "22/10"),
        ("55/600", ratio(55, 600), "55/600"),
        ("3/1", ratio(3, 1), "3/1"),
    ] {
        let parsed: Amount = text.parse().unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(parsed, amount, "{}", text);
        assert_eq!(parsed.to_string(), canonical, "{}", text);
    }
    // Structural equality: a trailing zero is the same decimal; a
    // reduced ratio is a different ratio; a ratio over 1 is not a
    // decimal.
    assert_eq!("2.50".parse::<Amount>().unwrap(), "2.5".parse().unwrap());
    assert_ne!("22/10".parse::<Amount>().unwrap(), "11/5".parse().unwrap());
    assert_ne!("3/1".parse::<Amount>().unwrap(), "3".parse().unwrap());
    for (bad, why) in [
        ("2.12345", "four fractional"),
        ("999.00001", "four fractional"),
        ("2.", "end in"),
        (".5", "digits"),
        ("02", "digits"),
        ("0", "positive"),
        ("0.00", "positive"),
        ("-1", "digits"),
        ("1 ", "digits"),
        ("1,5", "digits"),
        ("", "digits"),
        ("0/5", "positive integers"),
        ("5/0", "positive integers"),
        ("2.5/10", "positive integers"),
        ("1/2/3", "positive integers"),
        ("99999999999999999999", "too large"),
        ("1/99999999999999999999", "too large"),
    ] {
        let err = bad.parse::<Amount>().unwrap_err();
        assert!(err.why.contains(why), "{bad:?}: {err}");
    }
}

/// C67 — the four shapes, their JSON, and the exact round-trip
/// through the store's generic: a v1 value re-serializes to what was
/// read, so a non-canonical spelling is refused naming the path.
#[test]
fn c67_the_buyout_value_has_four_shapes_and_round_trips_exactly() {
    let cases = [
        (
            exact("12.5", "chaos"),
            json!({ "version": 1, "type": "exact", "amount": "12.5", "currency": "chaos" }),
        ),
        (
            Buyout::Negotiable(Price {
                amount: "22/10".parse().unwrap(),
                currency: "divine".into(),
            }),
            json!({ "version": 1, "type": "negotiable", "amount": "22/10", "currency": "divine" }),
        ),
        (Buyout::NoPrice, json!({ "version": 1, "type": "no_price" })),
        (Buyout::Skip, json!({ "version": 1, "type": "skip" })),
    ];
    for (buyout, wire) in cases {
        assert_eq!(buyout.to_value(), wire, "{buyout}");
        assert_eq!(check_value::<Buyout>(&wire).unwrap(), buyout);
    }
    // A trailing zero is the same amount in memory but not the
    // canonical text: refused at the door, naming the path, so the
    // stored text is always the one spelling.
    let err = check_value::<Buyout>(
        &json!({ "version": 1, "type": "exact", "amount": "12.50", "currency": "chaos" }),
    )
    .unwrap_err();
    assert_eq!(
        err,
        ValueError::NotCanonical {
            kind: "buyout",
            path: "/amount".into(),
            read: Some(json!("12.50")),
            canonical: Some(json!("12.5")),
        }
    );
    // A JSON number is not an amount: text only, so 2.50 cannot
    // become 2.5 by accident.
    let err = check_value::<Buyout>(
        &json!({ "version": 1, "type": "exact", "amount": 12.5, "currency": "chaos" }),
    )
    .unwrap_err();
    assert!(matches!(err, ValueError::Malformed { .. }), "{err}");
    // Display, for a human reading a row.
    assert_eq!(exact("12.5", "chaos").to_string(), "12.5 chaos");
    assert_eq!(
        Buyout::Negotiable(Price {
            amount: "3/1".parse().unwrap(),
            currency: "exalted".into()
        })
        .to_string(),
        "3/1 exalted b/o"
    );
}

/// C67 — what the value refuses: an unknown type, a price on a shape
/// that carries none (the 0.18 `[ignore]` rows' 4321 blessed), a
/// missing amount or currency, a third digit, an unknown field, a
/// version that is not 1.
#[test]
fn c67_the_buyout_parse_is_strict() {
    let refused = |v: Value, why: &str| {
        let err = check_value::<Buyout>(&v).unwrap_err();
        assert!(err.to_string().contains(why), "{v}: {err}");
    };
    refused(
        json!({ "version": 1, "type": "b/o", "amount": "1", "currency": "chaos" }),
        "not one of",
    );
    refused(
        json!({ "version": 1, "type": "skip", "amount": "4321", "currency": "blessed" }),
        "carries no amount",
    );
    refused(
        json!({ "version": 1, "type": "no_price", "currency": "chaos" }),
        "carries no amount",
    );
    refused(
        json!({ "version": 1, "type": "exact", "currency": "chaos" }),
        "needs an amount",
    );
    refused(
        json!({ "version": 1, "type": "exact", "amount": "1" }),
        "needs a currency",
    );
    refused(
        json!({ "version": 1, "type": "exact", "amount": "1.00005", "currency": "chaos" }),
        "four fractional",
    );
    refused(
        json!({ "version": 1, "type": "exact", "amount": "1", "currency": "chaos", "note": "x" }),
        "note",
    );
    refused(json!({ "version": 0, "type": "skip" }), "version 0");
    refused(json!({ "type": "skip" }), "missing integer `version`");
    assert_eq!(
        check_value::<Buyout>(&json!({ "version": 2, "type": "skip" })).unwrap_err(),
        ValueError::VersionUnsupported {
            kind: "buyout",
            found: 2,
            supported: 1
        }
    );
}

/// C67 — the currency is a tag of the shipped table (C68): an alias
/// is refused naming the tag, an unknown word naming the version, and
/// a retired tag still parses (a stored row may cite it forever).
#[test]
fn c67_the_currency_must_be_a_tag_of_the_reference_table() {
    let value =
        |word: &str| json!({ "version": 1, "type": "exact", "amount": "1", "currency": word });
    assert_eq!(
        check_value::<Buyout>(&value("chaos")).unwrap(),
        exact("1", "chaos")
    );
    // The legacy C++ tag is an alias of the game's word; the message
    // names the tag to cite instead.
    let err = check_value::<Buyout>(&value("exa"))
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("not a tag") && err.contains("\"exalted\""),
        "{err}"
    );
    let err = check_value::<Buyout>(&value("Chaos"))
        .unwrap_err()
        .to_string();
    assert!(err.contains("not in currency table v1"), "{err}");
    // Retired: parses; refusing a new one is the writer's rule.
    assert_eq!(
        check_value::<Buyout>(&value("chisel")).unwrap(),
        exact("1", "chisel")
    );
}

/// C66/C67 — through the store: a price lands typed, reads back typed
/// and raw, and a `clear` then `set` on one target works through the
/// tombstone with the revision carrying on (2b constraint (3), reduced).
#[test]
fn c67_clear_then_set_on_one_target_works_through_the_tombstone() {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let via = Provenance::via("test");
    let target = PriceTarget::Tab {
        realm: Realm::Pc,
        id: "t1".into(),
    };
    let (scope, key) = target.address().unwrap();
    // set
    let row = a
        .put::<Buyout>(scope, &key, &exact("30", "chaos").to_value(), None, &via)
        .unwrap();
    assert_eq!(row.revision, 1);
    let (read, typed) = a.get_as::<Buyout>(scope, &key).unwrap().unwrap();
    assert_eq!((read.revision, typed), (1, exact("30", "chaos")));
    assert_eq!(
        PriceTarget::from_address(&read.scope, &read.key).unwrap(),
        target
    );
    // clear
    a.delete(scope, &key, BUYOUT_KIND, 1, &via).unwrap();
    assert!(a.get_as::<Buyout>(scope, &key).unwrap().is_none());
    // set again, as a caller who read "nothing there" would: a create.
    let row = a
        .put::<Buyout>(scope, &key, &Buyout::Skip.to_value(), None, &via)
        .unwrap();
    assert_eq!(row.revision, 3, "the tombstone's revision carries on");
    let (_, typed) = a.get_as::<Buyout>(scope, &key).unwrap().unwrap();
    assert_eq!(typed, Buyout::Skip);
    // A price that does not parse never lands, even over a tombstone.
    a.delete(scope, &key, BUYOUT_KIND, 3, &via).unwrap();
    let err = a
        .put::<Buyout>(
            scope,
            &key,
            &json!({ "version": 1, "type": "exact", "amount": "1", "currency": "exa" }),
            None,
            &via,
        )
        .unwrap_err();
    assert!(matches!(err, AnnotationError::Invalid(_)), "{err}");
    assert!(a.get(scope, &key, BUYOUT_KIND).unwrap().is_none());
    // One kind across every price scope: the list a pricing read wants.
    for t in [
        PriceTarget::Item { id: "i1".into() },
        PriceTarget::Character { id: "c1".into() },
        PriceTarget::Substash {
            realm: Realm::Pc,
            parent: "m1".into(),
            id: "s1".into(),
        },
    ] {
        let (scope, key) = t.address().unwrap();
        a.put::<Buyout>(scope, &key, &Buyout::NoPrice.to_value(), None, &via)
            .unwrap();
    }
    let rows = a.list(None, Some(BUYOUT_KIND)).unwrap().rows;
    let targets: Vec<PriceTarget> = rows
        .iter()
        .map(|r| PriceTarget::from_address(&r.scope, &r.key).unwrap())
        .collect();
    assert_eq!(targets.len(), 3);
    assert!(
        targets
            .iter()
            .all(|t| !matches!(t, PriceTarget::Tab { .. }))
    );
}

/// C78 — until receipts exist, a single write returns the row's prior
/// value, so it can be undone by hand: a create's prior is nothing, a
/// replacement's is the row replaced, a clear's is the row cleared,
/// and setting the prior back restores the value with the revision
/// sequence carrying on (C35). Each write says the revision now
/// current — a clear its tombstone's, from the delete itself — and
/// the write that undoes it, which lands: a create's the clear naming
/// the revision written, a replace's the prior's value naming it, a
/// clear's a create naming none (round 5 of the 2g build).
#[test]
fn c78_a_write_returns_the_prior_value_so_a_hand_can_undo_it() {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let via = Provenance::via("test");
    let target = PriceTarget::Item { id: "i1".into() };
    let undone = |a: &mut Annotations, w: &PriceWrite| -> PriceWrite {
        match w.undo().unwrap() {
            Undo::Set { value, if_revision } => {
                set_buyout(a, &target, &value, if_revision, &via).unwrap()
            }
            Undo::Clear { if_revision } => clear_buyout(a, &target, if_revision, &via).unwrap(),
        }
    };
    let first = set_buyout(&mut a, &target, &exact("30", "chaos"), None, &via).unwrap();
    assert_eq!(first.target, target);
    assert_eq!(first.prior, None);
    assert_eq!(first.revision, 1);
    assert_eq!(first.undo(), Ok(Undo::Clear { if_revision: 1 }));
    let w1 = first.written.clone().unwrap();
    assert_eq!((w1.revision, w1.written_via.as_str()), (1, "test"));
    assert_eq!(Buyout::parse(&w1.value).unwrap(), exact("30", "chaos"));
    // the create's undo clears it, at the tombstone's revision
    let gone = undone(&mut a, &first);
    assert_eq!((gone.written, gone.revision), (None, 2));
    assert!(a.get(ITEM_SCOPE, "i1", BUYOUT_KIND).unwrap().is_none());
    let first = set_buyout(&mut a, &target, &exact("30", "chaos"), None, &via).unwrap();
    let w1 = first.written.unwrap();
    assert_eq!(w1.revision, 3);
    let second = set_buyout(&mut a, &target, &Buyout::Skip, Some(3), &via).unwrap();
    assert_eq!(second.prior.as_ref(), Some(&w1));
    assert_eq!(second.written.as_ref().map(|r| r.revision), Some(4));
    assert_eq!(second.revision, 4);
    assert_eq!(
        second.undo(),
        Ok(Undo::Set {
            value: exact("30", "chaos"),
            if_revision: Some(4)
        })
    );
    let cleared = clear_buyout(&mut a, &target, 4, &via).unwrap();
    assert_eq!(cleared.written, None);
    assert_eq!(cleared.prior, second.written);
    assert_eq!(cleared.revision, 5, "the tombstone's, from the delete");
    assert!(a.get(ITEM_SCOPE, "i1", BUYOUT_KIND).unwrap().is_none());
    // Undo by hand: the prior value, set again, as a caller who read
    // "nothing there" would — a create over the tombstone.
    assert_eq!(
        cleared.undo(),
        Ok(Undo::Set {
            value: Buyout::Skip,
            if_revision: None
        })
    );
    let restored = undone(&mut a, &cleared);
    assert_eq!(restored.prior, None);
    assert_eq!(restored.revision, 6);
    let (row, typed) = a.get_as::<Buyout>(ITEM_SCOPE, "i1").unwrap().unwrap();
    assert_eq!((row.revision, typed), (6, Buyout::Skip));
    // and the replace's undo puts its prior back over the revision it wrote
    let third = set_buyout(&mut a, &target, &exact("1", "divine"), Some(6), &via).unwrap();
    let back = undone(&mut a, &third);
    assert_eq!(back.prior, third.written);
    let (_, typed) = a.get_as::<Buyout>(ITEM_SCOPE, "i1").unwrap().unwrap();
    assert_eq!(typed, Buyout::Skip);
}

/// Round 5 of the 2g build, its review's finding 1: an undo reads its
/// prior as every reader of a stored row does, through the store's
/// door check, so a prior stored as `"12.50"` — not its canonical
/// text, which no reader reads — gets no undo (`"12.5"` would be
/// another value than the one stored), after a replace and after a
/// clear alike; the canonical one does.
#[test]
fn an_undo_reads_its_prior_through_the_stores_door_and_a_noncanonical_one_has_none() {
    let target = PriceTarget::Item { id: "i1".into() };
    let row = |amount: &str, revision: i64| AnnotationRow {
        scope: ITEM_SCOPE.into(),
        key: "i1".into(),
        kind: BUYOUT_KIND.into(),
        value: json!({ "version": 1, "type": "exact", "amount": amount, "currency": "chaos" }),
        revision,
        created_at: 0,
        updated_at: 0,
        written_via: "old".into(),
        actor: None,
    };
    assert!(check_value::<Buyout>(&row("12.50", 1).value).is_err());
    for written in [Some(row("1", 2)), None] {
        let write = |prior: AnnotationRow| PriceWrite {
            target: target.clone(),
            written: written.clone(),
            prior: Some(prior),
            prior_unreadable: None,
            revision: 2,
        };
        assert_eq!(
            write(row("12.50", 1)).undo(),
            Err("this build cannot read it".to_string())
        );
        assert!(write(row("12.5", 1)).undo().is_ok());
    }
}

/// C35, C78 — what a write replaced is the row at the revision the
/// write named, and a create names none and replaces none: a create
/// that read the row, then waited on the lock while a clear landed,
/// reports no prior — never the row the clear removed. Staged as the
/// store's lock tests stage a race: a second connection holds
/// `BEGIN IMMEDIATE`, tombstones the row inside it, and commits once
/// the create has read and is waiting on the lock.
/// C151 — a price write naming the revision of a row this build
/// cannot read goes through: `set --if-revision` replaces it and
/// `clear --if-revision` clears it, each receipt naming what stood by
/// its revision and fault (`prior_unreadable`) since no value can be
/// shown, and the clear's undo honestly says nothing puts it back;
/// a write that does not name the revision is refused in the store's
/// words, which name the revision. The damage is staged through a
/// second connection, as a damaged file would hold it.
#[test]
fn c151_a_write_naming_the_revision_replaces_or_clears_a_row_this_build_cannot_read() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let mut a = Annotations::open_for(&dir, "u-damaged").unwrap();
    let via = Provenance::via("test");
    let target = PriceTarget::Item { id: "i1".into() };
    set_buyout(&mut a, &target, &exact("30", "chaos"), None, &via).unwrap();
    let damage = || {
        rusqlite::Connection::open(acquisition_store::annotations_path(&dir, "u-damaged"))
            .unwrap()
            .execute(
                "UPDATE annotations SET value = '{nope' WHERE key = 'i1'",
                [],
            )
            .unwrap();
    };
    damage();

    let blind = set_buyout(&mut a, &target, &exact("5", "divine"), None, &via).unwrap_err();
    let words = blind.to_string();
    assert!(
        words.contains("item/i1/buyout") && words.contains("revision 1"),
        "{words}"
    );

    let replaced = set_buyout(&mut a, &target, &exact("5", "divine"), Some(1), &via).unwrap();
    assert_eq!(replaced.prior, None);
    let stood = replaced
        .prior_unreadable
        .clone()
        .expect("what stood is named");
    assert_eq!((stood.key.as_str(), stood.revision), ("i1", 1));
    assert!(
        !stood.fault.is_empty() && !stood.fault.contains("nope"),
        "{}",
        stood.fault
    );
    assert_eq!(replaced.revision, 2);
    assert!(
        matches!(replaced.undo(), Ok(Undo::Clear { if_revision: 2 })),
        "a replace of an unreadable row is undone by clearing what it wrote"
    );
    assert_eq!(
        a.get("item", "i1", BUYOUT_KIND).unwrap().unwrap().revision,
        2,
        "readable again"
    );

    damage();
    let cleared = clear_buyout(&mut a, &target, 2, &via).unwrap();
    assert_eq!((&cleared.written, &cleared.prior), (&None, &None));
    assert_eq!(
        cleared.prior_unreadable.as_ref().map(|r| r.revision),
        Some(2)
    );
    assert_eq!(cleared.revision, 3);
    let no_undo = cleared.undo().unwrap_err();
    assert!(no_undo.contains("could not be read"), "{no_undo}");
    assert_eq!(a.get("item", "i1", BUYOUT_KIND).unwrap(), None);
}

#[test]
fn a_create_after_a_racing_clear_reports_no_prior() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let mut a = Annotations::open_for(&dir, "u-race").unwrap();
    let via = Provenance::via("test");
    let target = PriceTarget::Item { id: "i1".into() };
    set_buyout(&mut a, &target, &exact("30", "chaos"), None, &via).unwrap();
    let racer =
        rusqlite::Connection::open(acquisition_store::annotations_path(&dir, "u-race")).unwrap();
    racer
        .execute_batch(
            "BEGIN IMMEDIATE;
                 UPDATE annotations SET deleted_at = 1, updated_at = 1, revision = revision + 1,
                        written_via = 'racer'
                  WHERE scope = 'item' AND key = 'i1' AND kind = 'buyout';",
        )
        .unwrap();
    let receipt = std::thread::scope(|s| {
        let create = s.spawn(|| set_buyout(&mut a, &target, &Buyout::Skip, None, &via));
        // The create reads the committed row at revision 1, then waits
        // on the lock the racer holds (the busy timeout is 5 s).
        std::thread::sleep(std::time::Duration::from_millis(500));
        racer.execute_batch("COMMIT").unwrap();
        create.join().unwrap()
    })
    .unwrap();
    assert_eq!(
        receipt.written.as_ref().map(|r| r.revision),
        Some(3),
        "the create landed over the clear's tombstone"
    );
    assert_eq!(receipt.prior, None, "a create replaced nothing");
}

/// C67 — whether a new price may name a retired tag is the writer's
/// rule, not the value's: `set_buyout` refuses one naming when it was
/// retired and lands nothing; a stored row citing it still reads, and
/// is replaced like any other, its value returned as the prior.
#[test]
fn c67_a_new_price_never_names_a_retired_tag_though_a_stored_row_may() {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let via = Provenance::via("test");
    let target = PriceTarget::Tab {
        realm: Realm::Pc,
        id: "t1".into(),
    };
    let refused = |err: PriceWriteError| match err {
        PriceWriteError::Store(AnnotationError::Invalid(ValueError::RefusedForWrite {
            kind,
            detail,
        })) => {
            assert_eq!(kind, BUYOUT_KIND);
            assert!(
                detail.contains("\"chisel\" is retired (2026-09-04"),
                "{detail}"
            );
        }
        other => panic!("expected RefusedForWrite, got {other}"),
    };
    refused(set_buyout(&mut a, &target, &exact("1", "chisel"), None, &via).unwrap_err());
    assert!(a.get(TAB_SCOPE, "pc/t1", BUYOUT_KIND).unwrap().is_none());
    // The rule is the door's: the store's own put refuses it too.
    let err = a
        .put::<Buyout>(
            TAB_SCOPE,
            "pc/t1",
            &exact("1", "chisel").to_value(),
            None,
            &via,
        )
        .unwrap_err();
    refused(PriceWriteError::Store(err));
    // A row written before the currency was retired: the value reads.
    a.put::<StoredBuyout>(
        TAB_SCOPE,
        "pc/t1",
        &exact("1", "chisel").to_value(),
        None,
        &via,
    )
    .unwrap();
    let (_, typed) = a.get_as::<Buyout>(TAB_SCOPE, "pc/t1").unwrap().unwrap();
    assert_eq!(typed, exact("1", "chisel"));
    let w = set_buyout(&mut a, &target, &exact("2", "chaos"), Some(1), &via).unwrap();
    assert_eq!(
        Buyout::parse(&w.prior.unwrap().value).unwrap(),
        exact("1", "chisel")
    );
}

/// C35 — the write is a compare-and-swap at this door too: a stale
/// revision conflicts carrying the current row and changes nothing;
/// so does a create over a row, and a clear at the wrong revision.
#[test]
fn c35_a_stale_revision_conflicts_and_changes_nothing() {
    let mut a = Annotations::open_memory_for("u-1").unwrap();
    let via = Provenance::via("test");
    let target = PriceTarget::Character { id: "c1".into() };
    let first = set_buyout(&mut a, &target, &exact("30", "chaos"), None, &via)
        .unwrap()
        .written
        .unwrap();
    for stale in [Some(5), None] {
        let err = set_buyout(&mut a, &target, &Buyout::Skip, stale, &via).unwrap_err();
        match err {
            PriceWriteError::Store(AnnotationError::Conflict { current: Some(row) }) => {
                assert_eq!(*row, first)
            }
            other => panic!("expected a Conflict carrying the row, got {other}"),
        }
    }
    let err = clear_buyout(&mut a, &target, 7, &via).unwrap_err();
    assert!(
        matches!(
            err,
            PriceWriteError::Store(AnnotationError::Conflict { .. })
        ),
        "{err}"
    );
    let (row, typed) = a.get_as::<Buyout>(CHARACTER_SCOPE, "c1").unwrap().unwrap();
    assert_eq!((row.revision, typed), (1, exact("30", "chaos")));
    clear_buyout(&mut a, &target, 1, &via).unwrap();
    assert!(a.get(CHARACTER_SCOPE, "c1", BUYOUT_KIND).unwrap().is_none());
}

/// The plan step 7 property tests (`PRICING-SLICE.md` at `b6197a82`): the amount's
/// grammar over every spelling, not the hand-picked ones above.
mod properties {
    use super::*;
    use proptest::prelude::*;

    /// Any amount the type can hold: every positive ten-thousandth,
    /// every pair of positive integers.
    fn any_amount() -> impl Strategy<Value = Amount> {
        prop_oneof![
            (1..=u64::MAX).prop_map(|ten_thousandths| Amount::Decimal { ten_thousandths }),
            (1..=u64::MAX, 1..=u64::MAX).prop_map(|(wanted, lot)| Amount::Ratio { wanted, lot }),
        ]
    }

    proptest! {
        /// C67 — any amount's canonical text parses back to the same
        /// amount, is a fixed point of the parse (the shortest
        /// spelling: no trailing zero, no trailing `.`), and is the
        /// JSON, both ways.
        #[test]
        fn c67_any_amount_round_trips_through_its_canonical_text(a in any_amount()) {
            let text = a.to_string();
            prop_assert_eq!(text.parse::<Amount>(), Ok(a));
            prop_assert_eq!(text.parse::<Amount>().unwrap().to_string(), text.clone());
            if let Amount::Decimal { .. } = a {
                prop_assert!(!text.ends_with('.'), "{}", text);
                prop_assert!(!(text.contains('.') && text.ends_with('0')), "{}", text);
            }
            prop_assert_eq!(serde_json::to_value(a).unwrap(), Value::String(text.clone()));
            prop_assert_eq!(serde_json::from_value::<Amount>(json!(text)).unwrap(), a);
        }

        /// C67, T10 — any spelling of the decimal grammar (a whole
        /// part, up to four fractional digits) parses to the amount
        /// it names, zero refused; its canonical text is no longer
        /// and names the same amount.
        #[test]
        fn c67_any_spelling_of_the_decimal_grammar_parses_to_the_amount_it_names(
            whole in prop_oneof![Just("0".to_string()), "[1-9][0-9]{0,14}"],
            fraction in prop::option::of("[0-9]{1,4}"),
        ) {
            let text = match &fraction {
                Some(f) => format!("{whole}.{f}"),
                None => whole.clone(),
            };
            let units: u64 = whole.parse().unwrap();
            let frac: u64 = fraction.as_deref().map_or(0, |f| {
                f.parse::<u64>().unwrap() * 10u64.pow(4 - f.len() as u32)
            });
            let want = units * 10_000 + frac;
            let got = text.parse::<Amount>();
            if want == 0 {
                prop_assert!(got.is_err(), "{} parsed", text);
            } else {
                prop_assert_eq!(got.clone(), Ok(Amount::Decimal { ten_thousandths: want }));
                let canonical = got.unwrap().to_string();
                prop_assert!(canonical.len() <= text.len(), "{} from {}", canonical, text);
                prop_assert_eq!(canonical.parse::<Amount>(), text.parse::<Amount>());
            }
        }

        /// C67, T2 — any lot pair parses and is kept unreduced: its
        /// canonical text is the pair verbatim.
        #[test]
        fn c67_any_lot_pair_parses_and_is_kept_unreduced(
            wanted in "[1-9][0-9]{0,18}",
            lot in "[1-9][0-9]{0,18}",
        ) {
            let text = format!("{wanted}/{lot}");
            let a = text.parse::<Amount>().unwrap();
            prop_assert_eq!(a, Amount::Ratio { wanted: wanted.parse().unwrap(), lot: lot.parse().unwrap() });
            prop_assert_eq!(a.to_string(), text);
        }

        /// C47, C67 — any text parses or is refused naming the text and
        /// a reason, never panics; what parses is digits, `.` and `/`
        /// only and round-trips.
        #[test]
        fn c47_any_text_parses_or_is_refused_naming_it(text in "\\PC{0,12}") {
            match text.parse::<Amount>() {
                Ok(a) => {
                    prop_assert!(text.bytes().all(|b| b.is_ascii_digit() || b == b'.' || b == b'/'), "{:?}", text);
                    prop_assert_eq!(a.to_string().parse::<Amount>(), Ok(a));
                }
                Err(e) => {
                    prop_assert_eq!(&e.text, &text);
                    prop_assert!(!e.why.is_empty());
                }
            }
        }
    }
}
