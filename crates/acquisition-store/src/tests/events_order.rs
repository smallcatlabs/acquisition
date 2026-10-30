/// The release seat (mcp-opus call 10): `item_events` asked for the
/// recent events and got the oldest first. The newest come first,
/// so a bounded read is the recent ones.
#[test]
fn events_since_lists_the_newest_first() {
    let s = crate::Store::open_memory().unwrap();
    for (at, id) in [(100, "a"), (300, "c"), (200, "b")] {
        s.conn
                .execute(
                    "INSERT INTO item_events (at, item_id, kind, from_location, to_location, response_id) VALUES (?1, ?2, 'added', NULL, 'stash/x', 1)",
                    rusqlite::params![at, id],
                )
                .unwrap();
    }
    let ev = s.events_since(0, 2).unwrap();
    assert_eq!(
        ev.iter().map(|e| e.at).collect::<Vec<_>>(),
        [300, 200],
        "the two newest, newest first"
    );
}

/// #91: a limit past `i64::MAX` binds as `i64::MAX` at each of the
/// three bounded reads, where an `as` cast wrapped it negative.
#[test]
fn a_limit_past_i64_max_binds_as_the_largest() {
    assert_eq!(crate::sql_limit(usize::MAX), i64::MAX);
    assert_eq!(crate::sql_limit(7), 7);
    let s = crate::Store::open_memory().unwrap();
    for (at, id) in [(100, "a"), (200, "b")] {
        s.conn
                .execute(
                    "INSERT INTO item_events (at, item_id, kind, from_location, to_location, response_id) VALUES (?1, ?2, 'added', NULL, 'stash/x', 1)",
                    rusqlite::params![at, id],
                )
                .unwrap();
    }
    assert_eq!(s.events_since(0, usize::MAX).unwrap().len(), 2);
    assert_eq!(s.events_after(0, usize::MAX).unwrap().len(), 2);
    assert!(s.refused_list(usize::MAX).unwrap().is_empty());
}
