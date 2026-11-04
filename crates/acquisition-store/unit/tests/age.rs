use super::*;

/// The vocabulary the CLI printed before step 2g's round 6 held it
/// here, boundary by boundary: the unit changes at 90 seconds, 90
/// minutes and 48 hours, rounding down.
#[test]
fn c53_an_age_is_said_in_one_vocabulary() {
    for (seconds, said) in [
        (0, "0s"),
        (45, "45s"),
        (89, "89s"),
        (90, "1m"),
        (5399, "89m"),
        (5400, "1h"),
        (172_799, "47h"),
        (172_800, "2d"),
        (9 * 86_400 + 5, "9d"),
        (-30, "0s"),
    ] {
        assert_eq!(span(seconds), said, "{seconds} seconds");
    }
}

/// The two readings of the one table: an epoch's age at the answer's
/// time, `never` for none, and its age at another epoch, named.
#[test]
fn c53_an_epoch_reads_as_its_age_at_a_moment_or_never() {
    let now = 1_000_000;
    assert_eq!(ago(now, Some(now - 6 * 86_400)), "6d ago");
    assert_eq!(ago(now, Some(now - 45)), "45s ago");
    assert_eq!(ago(now, None), "never");
    assert_eq!(ago(now, Some(now + 60)), "0s ago", "a clock set back");
    assert_eq!(
        before("generated_at", now, Some(now - 3 * 3600)),
        "3h before generated_at"
    );
    assert_eq!(before("generated_at", now, None), "never");
}

/// The round's review, P2: an epoch at either end of `i64`, which a
/// corrupt index or row could hold, is read without overflow — the
/// difference saturates, so the oldest possible epoch is the most
/// days ago and one past `now` none.
#[test]
fn c53_an_epoch_at_either_end_of_its_range_reads_without_overflow() {
    let most = format!("{}d ago", i64::MAX / 86_400);
    for now in [0, 1_000_000, i64::MAX] {
        assert_eq!(ago(now, Some(i64::MIN)), most, "now {now}");
    }
    assert_eq!(ago(0, Some(i64::MAX)), "0s ago");
    assert_eq!(ago(i64::MIN, Some(i64::MAX)), "0s ago");
    assert_eq!(ago(i64::MIN, Some(i64::MIN)), "0s ago");
    assert_eq!(
        before("generated_at", 0, Some(i64::MIN)),
        format!("{}d before generated_at", i64::MAX / 86_400)
    );
    assert_eq!(
        before("generated_at", i64::MIN, Some(i64::MAX)),
        "0s before generated_at"
    );
}

/// The store's rows a maker ages (C53): each says its epoch in the
/// vocabulary at the time given, under the epoch's name and `_age`,
/// `never` beside a null; a row no maker aged carries no words.
#[test]
fn c53_a_row_a_maker_ages_says_its_epochs_beside_them() {
    let now = 10_000;
    let refused = crate::Refused {
        id: 1,
        endpoint: "stash".into(),
        params: serde_json::json!({}),
        fetched_at: now - 120,
        fetched_at_age: None,
        status: 200,
        reason: "r".into(),
        body: None,
    };
    let unaged = serde_json::to_value(&refused).unwrap();
    assert!(unaged.get("fetched_at_age").is_none(), "{unaged}");
    assert_eq!(refused.aged(now).fetched_at_age.as_deref(), Some("2m ago"));
    let event = crate::EventRow {
        at: now - 7200,
        at_age: None,
        item_id: "i".into(),
        kind: "added".into(),
        from_location: None,
        to_location: None,
        name: None,
        type_line: None,
    };
    assert_eq!(event.aged(now).at_age.as_deref(), Some("2h ago"));
    let tab = crate::TabRow {
        realm: "pc".into(),
        league: "Standard".into(),
        id: "t".into(),
        parent: None,
        name: "T".into(),
        r#type: "PremiumStash".into(),
        idx: None,
        listed_at: Some(now),
        listed_at_age: None,
        fetched_at: None,
        fetched_at_age: None,
        removed_at: None,
        removed_at_age: None,
        item_count: 0,
        fetched_items: None,
    };
    let tab = tab.aged(now);
    assert_eq!(tab.fetched_at_age.as_deref(), Some("never"));
    // C163: the listing's time in words, a removal's only where it left
    assert_eq!(tab.listed_at_age.as_deref(), Some("0s ago"));
    assert_eq!(tab.removed_at_age, None);
}

/// A league's `startAt` as GGG gives it, ISO 8601 UTC, read as seconds
/// since 1970 (the answer-facts seat, MCP fault 3); a fraction dropped, a
/// leap day kept, and any other shape none, never a panic (C47).
#[test]
fn a_time_ggg_gives_as_text_reads_as_its_epoch_or_none() {
    for (text, at) in [
        ("1970-01-01T00:00:00Z", Some(0)),
        ("2013-01-23T21:00:00Z", Some(1_358_974_800)),
        ("2026-11-02T20:00:00Z", Some(1_793_649_600)),
        ("2026-11-02T20:00:00.250Z", Some(1_793_649_600)),
        ("2000-02-29T00:00:00Z", Some(951_782_400)),
        ("2026-11-02T20:00:00", None),
        ("2026-11-02 20:00:00Z", None),
        ("2026-13-02T20:00:00Z", None),
        ("2026-11-02T20:00Z", None),
        ("26-11-02T20:00:00Z", None),
        // a day its month does not have, and a fraction that is not
        // decimal digits (the round's review)
        ("2026-02-31T00:00:00Z", None),
        ("2025-02-29T00:00:00Z", None),
        ("2026-04-31T00:00:00Z", None),
        ("2026-11-02T20:00:00.garbageZ", None),
        ("2026-11-02T20:00:00.Z", None),
        ("", None),
    ] {
        assert_eq!(parse_utc(text), at, "{text}");
    }
}
