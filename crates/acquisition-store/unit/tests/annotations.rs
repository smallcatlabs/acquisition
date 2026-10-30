use super::test_kinds::{Buyout, GuardedNote, Note, Policy, via_test};
use super::*;
use serde_json::json;

/// C66 — a kind's writer's rule runs at the door on every write and
/// never on a read: a value it refuses as new never lands, the same
/// value already stored (written before the rule, or by a kind
/// without it) still reads through `get_as`, and is replaced or
/// deleted like any other row.
#[test]
fn c66_a_writer_s_rule_refuses_a_new_write_and_never_a_read() {
    let mut a = Annotations::open_memory().unwrap();
    let retired = json!({ "version": 1, "retired": true });
    let err = a
        .put::<GuardedNote>("item", "i1", &retired, None, &via_test())
        .unwrap_err();
    match err {
        AnnotationError::Invalid(ValueError::RefusedForWrite { kind, detail }) => {
            assert_eq!((kind, detail.as_str()), ("note", "names a retired thing"))
        }
        other => panic!("expected RefusedForWrite, got {other}"),
    }
    assert!(a.get("item", "i1", "note").unwrap().is_none());
    // Stored by a kind without the rule: the guarded kind reads it.
    a.put::<Note>("item", "i1", &retired, None, &via_test())
        .unwrap();
    let (row, read) = a.get_as::<GuardedNote>("item", "i1").unwrap().unwrap();
    assert_eq!((row.revision, read.0), (1, retired));
    let row = a
        .put::<GuardedNote>(
            "item",
            "i1",
            &json!({ "version": 1, "retired": false }),
            Some(1),
            &via_test(),
        )
        .unwrap();
    assert_eq!(row.revision, 2);
}

/// The file's revision moves by one on every write — create, update,
/// delete, create over the tombstone — and by nothing else.
#[test]
fn the_files_revision_rises_by_one_per_write_tombstones_included() {
    let mut a = Annotations::open_memory().unwrap();
    assert_eq!(a.revision().unwrap(), 0);
    a.put::<Buyout>("item", "i1", &price("1"), None, &via_test())
        .unwrap();
    assert_eq!(a.revision().unwrap(), 1);
    a.put::<Buyout>("item", "i2", &price("1"), None, &via_test())
        .unwrap();
    assert_eq!(a.revision().unwrap(), 2);
    a.put::<Buyout>("item", "i1", &price("2"), Some(1), &via_test())
        .unwrap();
    assert_eq!(a.revision().unwrap(), 3);
    a.delete("item", "i1", "buyout", 2, &via_test()).unwrap();
    assert_eq!(a.revision().unwrap(), 4);
    // the tombstone still counts; a create over it carries on from it
    a.put::<Buyout>("item", "i1", &price("3"), None, &via_test())
        .unwrap();
    assert_eq!(a.revision().unwrap(), 5);
    // a read moves nothing
    a.list(None, None).unwrap();
    a.get("item", "i1", "buyout").unwrap();
    assert_eq!(a.revision().unwrap(), 5);
}

fn conflict_revision(e: AnnotationError) -> Option<i64> {
    match e {
        AnnotationError::Conflict { current } => current.map(|r| r.revision),
        other => panic!("expected a conflict, got {other}"),
    }
}

fn price(p: &str) -> Value {
    json!({ "version": 1, "price": p })
}

/// C35 — integer-revision compare-and-swap: create requires absence, update the exact revision, delete the same.
#[test]
fn compare_and_swap_creates_updates_and_conflicts() {
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    // Create requires "does not exist yet".
    let row = a
        .put::<Buyout>("item", "i1", &price("1 divine"), None, &via)
        .unwrap();
    assert_eq!(row.revision, 1);
    let err = a
        .put::<Buyout>("item", "i1", &price("2 divine"), None, &via)
        .unwrap_err();
    assert_eq!(conflict_revision(err), Some(1));
    // Update requires the exact current revision.
    let row = a
        .put::<Buyout>("item", "i1", &price("2 divine"), Some(1), &via)
        .unwrap();
    assert_eq!(row.revision, 2);
    assert!(row.created_at <= row.updated_at);
    let stale = a
        .put::<Buyout>("item", "i1", &price("3c"), Some(1), &via)
        .unwrap_err();
    assert_eq!(conflict_revision(stale), Some(2));
    // The stored value is the last accepted write.
    let got = a.get("item", "i1", "buyout").unwrap().unwrap();
    assert_eq!(got.value, price("2 divine"));
    // The same key under another kind is its own row.
    a.put::<Note>(
        "item",
        "i1",
        &json!({ "version": 1, "text": "keep" }),
        None,
        &via,
    )
    .unwrap();
    assert_eq!(a.list(Some("item"), None).unwrap().rows.len(), 2);
    // Delete is CAS too; a stale delete conflicts and changes nothing.
    let err = a.delete("item", "i1", "buyout", 1, &via).unwrap_err();
    assert_eq!(conflict_revision(err), Some(2));
    a.delete("item", "i1", "buyout", 2, &via).unwrap();
    assert!(a.get("item", "i1", "buyout").unwrap().is_none());
    // Updating what is gone reports "gone", not "wrong revision".
    let err = a
        .put::<Buyout>("item", "i1", &price("x"), Some(2), &via)
        .unwrap_err();
    assert_eq!(conflict_revision(err), None);
}

/// C35 — delete tombstones; the revision sequence survives delete/recreate, so a writer holding any revision of the live row conflicts once it moves (a create over the tombstone lands, whatever its generation — see `put`).
/// A live row's value overwritten with text that is not JSON, as a
/// damaged file holds it: the fixture every `c151_` test stages.
fn damaged(a: &Annotations, key: &str) {
    a.conn
        .execute(
            "UPDATE annotations SET value = '{nope' WHERE key = ?1",
            [key],
        )
        .unwrap();
}

/// C151 — an annotation row this build cannot read is named, never
/// dropped, never a wall: `list` answers the readable rows and names
/// the damaged one by its coordinate, revision and fault (the
/// parser's words, which never quote the value, C127), where it used
/// to fail whole; `get` answers the same facts as a structured
/// refusal, in words that say what replaces the row; the file's
/// revision still sums every row; a tombstone is never the row.
#[test]
fn c151_list_names_an_unreadable_row_beside_the_readable_ones_and_get_refuses_it() {
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    a.put::<Buyout>("item", "i-good", &price("1c"), None, &via)
        .unwrap();
    let bad = a
        .put::<Buyout>("item", "i-bad", &price("2c"), None, &via)
        .unwrap();
    a.put::<Buyout>("item", "i-bad", &price("3c"), Some(bad.revision), &via)
        .unwrap();
    damaged(&a, "i-bad");

    let list = a.list(None, None).unwrap();
    let readable: Vec<&str> = list.rows.iter().map(|r| r.key.as_str()).collect();
    assert_eq!(readable, ["i-good"], "the readable rows, whole");
    let [named] = list.unreadable.as_slice() else {
        panic!("one unreadable row named: {:?}", list.unreadable)
    };
    assert_eq!(
        (
            named.scope.as_str(),
            named.key.as_str(),
            named.kind.as_str(),
            named.revision
        ),
        ("item", "i-bad", "buyout", 2)
    );
    assert!(
        !named.fault.is_empty() && !named.fault.contains("nope"),
        "the fault is the parser's and never quotes the value: {}",
        named.fault
    );
    assert_eq!(
        a.revision().unwrap(),
        1 + 2,
        "the file's revision sums every row"
    );

    let err = a.get("item", "i-bad", "buyout").unwrap_err();
    assert!(
        matches!(&err, AnnotationError::Unreadable(row) if row.revision == 2 && row.key == "i-bad"),
        "{err:?}"
    );
    let words = err.to_string();
    assert!(
        words.contains("item/i-bad/buyout")
            && words.contains("revision 2")
            && words.contains("naming that revision replaces it"),
        "{words}"
    );
    assert_eq!(a.cleared("item", "i-bad", "buyout").unwrap(), None);
    assert_eq!(
        a.get("item", "i-good", "buyout").unwrap().unwrap().value,
        price("1c")
    );
}

/// C151 — a write naming the row's revision does not parse its value:
/// `put` at the revision replaces an unreadable row and `delete` at
/// the revision clears it, through the one compare-and-swap (C35);
/// at a wrong revision, or a create over the live row, each answers
/// `Unreadable` with the revision the row stands at, never a
/// `Conflict` that would need the row it cannot read; a create over
/// the tombstone of a damaged row continues the sequence without
/// reading it. Nothing repairs a value in place.
#[test]
fn c151_a_write_naming_the_revision_replaces_or_clears_an_unreadable_row() {
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    let first = a
        .put::<Buyout>("item", "i-bad", &price("2c"), None, &via)
        .unwrap();
    a.put::<Buyout>("item", "i-bad", &price("3c"), Some(first.revision), &via)
        .unwrap();
    damaged(&a, "i-bad");
    let unreadable_at = |err: AnnotationError| match err {
        AnnotationError::Unreadable(row) => row.revision,
        other => panic!("expected the row named as unreadable, got {other:?}"),
    };

    // The wrong revision, and a create over the live row: named.
    let stale = a
        .put::<Buyout>("item", "i-bad", &price("4c"), Some(1), &via)
        .unwrap_err();
    assert_eq!(unreadable_at(stale), 2);
    let blind = a
        .put::<Buyout>("item", "i-bad", &price("4c"), None, &via)
        .unwrap_err();
    assert_eq!(unreadable_at(blind), 2);
    let stale_clear = a.delete("item", "i-bad", "buyout", 1, &via).unwrap_err();
    assert_eq!(unreadable_at(stale_clear), 2);
    assert!(
        matches!(
            a.get("item", "i-bad", "buyout"),
            Err(AnnotationError::Unreadable(_))
        ),
        "nothing was written"
    );

    // The right revision: replaced, readable again, the sequence kept.
    let replaced = a
        .put::<Buyout>("item", "i-bad", &price("4c"), Some(2), &via)
        .unwrap();
    assert_eq!((replaced.revision, &replaced.value), (3, &price("4c")));
    assert_eq!(
        a.get("item", "i-bad", "buyout").unwrap().unwrap().value,
        price("4c")
    );
    assert!(a.list(None, None).unwrap().unreadable.is_empty());

    // Damaged again, cleared at its revision; the tombstone's value is
    // still unreadable and a create over it never reads it.
    damaged(&a, "i-bad");
    let tombstone = a.delete("item", "i-bad", "buyout", 3, &via).unwrap();
    assert_eq!(tombstone, 4);
    let list = a.list(None, None).unwrap();
    assert!(
        list.rows.is_empty() && list.unreadable.is_empty(),
        "{list:?}"
    );
    assert_eq!(a.cleared("item", "i-bad", "buyout").unwrap(), Some(4));
    let again = a
        .put::<Buyout>("item", "i-bad", &price("5c"), None, &via)
        .unwrap();
    assert_eq!(
        again.revision, 5,
        "the sequence continues past the damaged tombstone"
    );
}

#[test]
fn delete_and_recreate_never_reset_the_revision() {
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    // A stale reader remembers revision 1...
    let stale = a
        .put::<Buyout>("item", "i1", &price("1c"), None, &via)
        .unwrap()
        .revision;
    // ...while someone else deletes and recreates the annotation.
    a.delete("item", "i1", "buyout", 1, &via).unwrap();
    let recreated = a
        .put::<Buyout>("item", "i1", &price("5 divine"), None, &via)
        .unwrap();
    assert!(
        recreated.revision > stale,
        "recreation continues the sequence ({} > {stale})",
        recreated.revision
    );
    // The stale writer's update conflicts instead of silently landing
    // its pre-delete value over the recreated one (the ABA hole).
    let err = a
        .put::<Buyout>("item", "i1", &price("1c"), Some(stale), &via)
        .unwrap_err();
    assert_eq!(conflict_revision(err), Some(recreated.revision));
    assert_eq!(
        a.get("item", "i1", "buyout").unwrap().unwrap().value,
        price("5 divine")
    );
    // A stale delete cannot remove the recreated value either.
    assert!(a.delete("item", "i1", "buyout", stale, &via).is_err());
}

/// C35, C78 — a delete returns the revision its own transaction gave
/// the tombstone, the generation a later conflict names and the next
/// create continues past; and the row a write names is read at exactly
/// that revision or refused as the write would be, carrying the live
/// row (none over a tombstone), so a receipt's prior is the row
/// replaced and never one a racing writer moved (round 5 of the 2g
/// build).
#[test]
fn a_delete_returns_its_tombstones_revision_and_a_write_reads_only_the_row_it_names() {
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    a.put::<Buyout>("item", "i1", &price("1c"), None, &via)
        .unwrap();
    a.put::<Buyout>("item", "i1", &price("2c"), Some(1), &via)
        .unwrap();
    let at = a.at_revision("item", "i1", "buyout", 2).unwrap();
    assert_eq!((at.revision, &at.value), (2, &price("2c")));
    let err = a.at_revision("item", "i1", "buyout", 1).unwrap_err();
    assert_eq!(
        conflict_revision(err),
        Some(2),
        "a moved row is the conflict"
    );
    let tombstone = a.delete("item", "i1", "buyout", 2, &via).unwrap();
    assert_eq!(tombstone, 3);
    assert_eq!(a.cleared("item", "i1", "buyout").unwrap(), Some(tombstone));
    let err = a.at_revision("item", "i1", "buyout", 3).unwrap_err();
    assert_eq!(
        conflict_revision(err),
        None,
        "a tombstone is no row to replace"
    );
    let err = a.at_revision("item", "i9", "buyout", 1).unwrap_err();
    assert_eq!(conflict_revision(err), None, "nor is a row never written");
    let recreated = a
        .put::<Buyout>("item", "i1", &price("3c"), None, &via)
        .unwrap();
    assert_eq!(recreated.revision, tombstone + 1);
}

/// C47 (a stored value at the bound is a structured refusal) — a row
/// at the last revision an `i64` holds takes no next one: a replace, a
/// delete and a create over such a tombstone each refuse as
/// `RevisionExhausted`, and nothing is written; one below it moves to
/// the bound, the number written the number returned (round 5 of the
/// 2g build, its review's finding 3).
#[test]
fn a_row_at_the_last_revision_refuses_every_write_and_changes_nothing() {
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    a.put::<Buyout>("item", "i1", &price("1c"), None, &via)
        .unwrap();
    let at = |a: &Annotations, revision: i64| {
        a.conn
            .execute(
                "UPDATE annotations SET revision = ?1 WHERE key = 'i1'",
                [revision],
            )
            .unwrap();
    };
    let exhausted = |e: AnnotationError| {
        assert!(
            matches!(
                e,
                AnnotationError::RevisionExhausted {
                    revision: i64::MAX,
                    ..
                }
            ),
            "{e}"
        );
    };
    at(&a, i64::MAX - 1);
    let row = a
        .put::<Buyout>("item", "i1", &price("2c"), Some(i64::MAX - 1), &via)
        .unwrap();
    assert_eq!(row.revision, i64::MAX);
    assert_eq!(a.get("item", "i1", "buyout").unwrap(), Some(row.clone()));
    exhausted(
        a.put::<Buyout>("item", "i1", &price("3c"), Some(i64::MAX), &via)
            .unwrap_err(),
    );
    exhausted(
        a.delete("item", "i1", "buyout", i64::MAX, &via)
            .unwrap_err(),
    );
    assert_eq!(
        a.get("item", "i1", "buyout").unwrap(),
        Some(row),
        "untouched"
    );
    at(&a, i64::MAX - 1);
    assert_eq!(
        a.delete("item", "i1", "buyout", i64::MAX - 1, &via)
            .unwrap(),
        i64::MAX
    );
    assert_eq!(a.cleared("item", "i1", "buyout").unwrap(), Some(i64::MAX));
    exhausted(
        a.put::<Buyout>("item", "i1", &price("4c"), None, &via)
            .unwrap_err(),
    );
    assert_eq!(a.cleared("item", "i1", "buyout").unwrap(), Some(i64::MAX));
}

/// C35 — two writers on one file serialize under BEGIN IMMEDIATE: one wins, the other gets a Conflict, never a clobber.
#[test]
fn two_connections_conflict_deterministically_never_clobber() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-race");
    let mut a = Annotations::open(&path).unwrap();
    let mut b = Annotations::open(&path).unwrap();
    let via = via_test();
    // Both connections race the same create. BEGIN IMMEDIATE
    // serializes them, so exactly one wins and the loser gets the
    // documented Conflict carrying the winner's row — never a raw
    // SQLite busy/snapshot error, never two revision-1 writes.
    let results = std::thread::scope(|scope| {
        let ta = scope.spawn(|| {
            a.put::<Buyout>(
                "item",
                "i1",
                &json!({"version": 1, "from": "a"}),
                None,
                &via,
            )
        });
        let tb = scope.spawn(|| {
            b.put::<Buyout>(
                "item",
                "i1",
                &json!({"version": 1, "from": "b"}),
                None,
                &via,
            )
        });
        [ta.join().unwrap(), tb.join().unwrap()]
    });
    let (ok, err): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
    assert_eq!((ok.len(), err.len()), (1, 1), "one winner, one conflict");
    let winner = ok.into_iter().next().unwrap().unwrap();
    assert_eq!(winner.revision, 1);
    match err.into_iter().next().unwrap().unwrap_err() {
        AnnotationError::Conflict { current: Some(row) } => assert_eq!(*row, winner),
        other => panic!("the loser must see a Conflict, got {other}"),
    }
}

/// A writer that cannot get the lock inside the busy timeout is told
/// to retry later — its own kind, not a bare SQLite error — because
/// the remedy differs from a Conflict's "re-read and retry".
#[test]
fn a_writer_held_past_the_busy_timeout_is_busy_not_db() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-busy");
    let mut a = Annotations::open(&path).unwrap();
    let b = Annotations::open(&path).unwrap();
    a.conn
        .busy_timeout(std::time::Duration::from_millis(50))
        .unwrap();
    // `b` holds the write lock in an open transaction the whole time.
    b.conn.execute_batch("BEGIN IMMEDIATE").unwrap();
    let err = a
        .put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
        .unwrap_err();
    assert!(matches!(err, AnnotationError::Busy { .. }), "{err}");
    assert!(err.to_string().contains("retry later"), "{err}");
    b.conn.execute_batch("ROLLBACK").unwrap();
    // The row is untouched: the same create succeeds once the lock is free.
    assert_eq!(
        a.put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
            .unwrap()
            .revision,
        1
    );
}

/// C137: the annotation wait at zero (`ACQ_ANNOTATION_WAIT=0`) refuses
/// a writer the file's lock is held from at once, as busy, naming
/// zero seconds: the expiry forced, no real waiting, and no
/// wall-clock bound asserted. The seconds named are the wait SQLite
/// enforced, read back from the connection, so a knob unwired from
/// the busy timeout leaves the default's five and fails here. The
/// knob is the process's, so the test runs itself again as a child
/// with it set, and no other test of this binary reads it changed.
#[test]
fn c137_the_annotation_wait_at_zero_refuses_a_held_writer_at_once() {
    if std::env::var("ACQ_ANNOTATION_WAIT").as_deref() != Ok("0") {
        let module = module_path!()
            .split_once("::")
            .map_or(module_path!(), |(_, rest)| rest);
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                format!("{module}::c137_the_annotation_wait_at_zero_refuses_a_held_writer_at_once"),
                "--exact".into(),
                "--nocapture".into(),
                "--test-threads=1".into(),
            ])
            .env("ACQ_ANNOTATION_WAIT", "0")
            .output()
            .unwrap();
        let said = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success() && said.contains("1 passed"),
            "the child with the wait at zero: {}\n{said}\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
        return;
    }
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-wait");
    let mut a = Annotations::open(&path).unwrap();
    let b = Annotations::open(&path).unwrap();
    // `b` holds the write lock in an open transaction the whole time.
    b.conn.execute_batch("BEGIN IMMEDIATE").unwrap();
    let err = a
        .put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
        .unwrap_err();
    assert!(
        matches!(
            err,
            AnnotationError::Busy {
                waited: Some(std::time::Duration::ZERO)
            }
        ),
        "the wait the connection enforced, not the default: {err:?}"
    );
    assert_eq!(
        err.to_string(),
        "annotation file is busy: another writer held it past 0 s (retry later)"
    );
    b.conn.execute_batch("ROLLBACK").unwrap();
}

/// C137: the knob names only a wait SQLite can enforce. Over the
/// bound (whose milliseconds overflow SQLite's signed 32-bit timeout),
/// `u64::MAX` and garbage keep the default; zero stays zero, the
/// forced expiry; the bound itself is honoured.
#[test]
fn c137_the_annotation_wait_keeps_the_default_for_a_value_sqlite_cannot_hold() {
    use std::time::Duration;
    assert_eq!(parse_annotation_wait("2147484"), BUSY_TIMEOUT);
    assert_eq!(parse_annotation_wait(&u64::MAX.to_string()), BUSY_TIMEOUT);
    assert_eq!(parse_annotation_wait("0"), Duration::ZERO);
    assert_eq!(parse_annotation_wait("five"), BUSY_TIMEOUT);
    assert_eq!(parse_annotation_wait("-1"), BUSY_TIMEOUT);
    assert_eq!(
        parse_annotation_wait(" 2147483 "),
        Duration::from_secs(MAX_WAIT_SECS)
    );
    // The bound is one SQLite holds: set and read back, unchanged.
    let a = Annotations::open_memory().unwrap();
    a.conn
        .busy_timeout(Duration::from_secs(MAX_WAIT_SECS))
        .unwrap();
    assert_eq!(
        enforced_wait(&a.conn),
        Some(Duration::from_secs(MAX_WAIT_SECS))
    );
}

#[test]
fn account_singletons_use_the_empty_key() {
    let mut a = Annotations::open_memory().unwrap();
    let policy = json!({ "version": 1, "leagues": ["Standard"], "deep": false });
    a.put::<Policy>("account", "", &policy, None, &via_test())
        .unwrap();
    let row = a.get("account", "", "sync-policy").unwrap().unwrap();
    assert_eq!(row.value, policy);
}

/// `list` filters by scope, by kind, or both: a pricing read wants one
/// kind across every scope.
#[test]
fn list_filters_by_scope_and_by_kind() {
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    a.put::<Buyout>("item", "i1", &price("1c"), None, &via)
        .unwrap();
    a.put::<Buyout>("tab", "pc/t1", &price("2c"), None, &via)
        .unwrap();
    a.put::<Note>(
        "item",
        "i1",
        &json!({ "version": 1, "text": "n" }),
        None,
        &via,
    )
    .unwrap();
    let keys = |rows: Vec<AnnotationRow>| -> Vec<String> {
        rows.into_iter()
            .map(|r| format!("{}/{}/{}", r.scope, r.key, r.kind))
            .collect()
    };
    assert_eq!(
        keys(a.list(None, None).unwrap().rows),
        ["item/i1/buyout", "item/i1/note", "tab/pc/t1/buyout"]
    );
    assert_eq!(
        keys(a.list(None, Some("buyout")).unwrap().rows),
        ["item/i1/buyout", "tab/pc/t1/buyout"]
    );
    assert_eq!(
        keys(a.list(Some("item"), Some("buyout")).unwrap().rows),
        ["item/i1/buyout"]
    );
    assert!(
        a.list(Some("account"), Some("buyout"))
            .unwrap()
            .rows
            .is_empty()
    );
}

/// C65 — every write carries who wrote it; a delete stamps the
/// tombstone; an empty or whitespace channel is refused before
/// anything lands.
#[test]
fn c65_every_write_carries_its_provenance() {
    let mut a = Annotations::open_memory().unwrap();
    let cli = Provenance::via("cli").as_actor("tom");
    let row = a
        .put::<Buyout>("item", "i1", &price("1c"), None, &cli)
        .unwrap();
    assert_eq!(
        (row.written_via.as_str(), row.actor.as_deref()),
        ("cli", Some("tom"))
    );
    let read = a.get("item", "i1", "buyout").unwrap().unwrap();
    assert_eq!(read, row);
    // The next writer's stamp replaces it, actor included.
    let mcp = Provenance::via("mcp");
    let row = a
        .put::<Buyout>("item", "i1", &price("2c"), Some(1), &mcp)
        .unwrap();
    assert_eq!((row.written_via.as_str(), row.actor), ("mcp", None));
    // The tombstone records who cleared the row.
    a.delete("item", "i1", "buyout", 2, &cli).unwrap();
    let tomb = stored(&a.conn, "item", "i1", "buyout").unwrap().unwrap();
    assert!(tomb.deleted);
    assert_eq!(tomb.written_via, "cli");
    assert_eq!(tomb.actor.as_deref(), Some("tom"));
    // Refused channels: nothing lands, nothing is deleted.
    for bad in ["", "two words"] {
        let err = a
            .put::<Buyout>("item", "i2", &price("1c"), None, &Provenance::via(bad))
            .unwrap_err();
        assert!(
            matches!(err, AnnotationError::Provenance { .. }),
            "{bad:?}: {err}"
        );
        assert!(a.get("item", "i2", "buyout").unwrap().is_none());
    }
    a.put::<Buyout>("item", "i2", &price("1c"), None, &mcp)
        .unwrap();
    let err = a
        .delete("item", "i2", "buyout", 1, &Provenance::via(""))
        .unwrap_err();
    assert!(matches!(err, AnnotationError::Provenance { .. }), "{err}");
    assert!(a.get("item", "i2", "buyout").unwrap().is_some());
    // The row's JSON carries the stamp; a missing actor is omitted, not null.
    let json = serde_json::to_value(a.get("item", "i2", "buyout").unwrap().unwrap()).unwrap();
    assert_eq!(json["written_via"], "mcp");
    assert!(json.get("actor").is_none());
}

/// C66 — the generic at the write door: no version, a newer version,
/// the kind's own refusal, and a current-schema value that does not
/// re-serialize to what was read all fail before anything lands.
#[test]
fn c66_the_write_door_is_typed_and_a_value_that_fails_never_lands() {
    /// A kind with one field, `n`, that must be an integer; its
    /// canonical form has nothing else.
    #[derive(Debug, Serialize)]
    struct Strict {
        version: i64,
        n: i64,
    }
    impl IntentValue for Strict {
        const KIND: &'static str = "strict";
        const VERSION: i64 = 2;
        fn parse(value: &Value) -> Result<Self, String> {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Wire {
                version: i64,
                n: i64,
                // v2 only; v1 read `m` and upgrades.
                #[serde(default)]
                m: Option<i64>,
            }
            let w: Wire = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
            match (w.version, w.m) {
                (1, Some(m)) => Ok(Strict { version: 2, n: m }),
                (1, None) => Err("v1 needs `m`".into()),
                (2, None) => Ok(Strict { version: 2, n: w.n }),
                (2, Some(_)) => Err("v2 has no `m`".into()),
                (v, _) => Err(format!("version {v}")),
            }
        }
    }
    let mut a = Annotations::open_memory().unwrap();
    let via = via_test();
    let refused = |a: &mut Annotations, v: Value| {
        let err = a.put::<Strict>("item", "i1", &v, None, &via).unwrap_err();
        assert!(
            a.get("item", "i1", "strict").unwrap().is_none(),
            "{v} landed"
        );
        match err {
            AnnotationError::Invalid(e) => e,
            other => panic!("{v}: expected Invalid, got {other}"),
        }
    };
    assert_eq!(
        refused(&mut a, json!({ "n": 1 })),
        ValueError::MissingVersion { kind: "strict" }
    );
    // Newer is reported as newer, before the shape is read.
    assert_eq!(
        refused(&mut a, json!({ "version": 3, "n": 1, "future": true })),
        ValueError::VersionUnsupported {
            kind: "strict",
            found: 3,
            supported: 2
        }
    );
    // The kind's own strictness: an unknown field.
    assert!(matches!(
        refused(&mut a, json!({ "version": 2, "n": 1, "typo": 1 })),
        ValueError::Malformed { detail, .. } if detail.contains("typo")
    ));
    // Current schema, parses, but the canonical form differs
    // (`m: null` is read as absent and written as nothing): refused
    // naming the path and both sides — null is not absent.
    let err = refused(&mut a, json!({ "version": 2, "n": 1, "m": null }));
    assert_eq!(
        err,
        ValueError::NotCanonical {
            kind: "strict",
            path: "/m".into(),
            read: Some(Value::Null),
            canonical: None,
        }
    );
    assert_eq!(
        err.to_string(),
        "strict: not in canonical form at /m: read null, the canonical form has absent"
    );
    // An older stamp upgrades in memory and lands as written.
    let v1 = json!({ "version": 1, "n": 0, "m": 7 });
    a.put::<Strict>("item", "i1", &v1, None, &via).unwrap();
    let (row, typed) = a.get_as::<Strict>("item", "i1").unwrap().unwrap();
    assert_eq!(row.value, v1, "stored as written");
    assert_eq!((typed.version, typed.n), (2, 7), "upgraded in memory");
    // A current-schema value lands and reads back typed.
    let v2 = json!({ "version": 2, "n": 9 });
    a.put::<Strict>("item", "i1", &v2, Some(1), &via).unwrap();
    let (_, typed) = a.get_as::<Strict>("item", "i1").unwrap().unwrap();
    assert_eq!(typed.n, 9);
    // A stored row a newer build wrote is an error on read, never "none".
    a.conn
        .execute(
            "UPDATE annotations SET value = ?1 WHERE kind = 'strict'",
            [json!({ "version": 3, "n": 1 }).to_string()],
        )
        .unwrap();
    assert!(matches!(
        a.get_as::<Strict>("item", "i1").unwrap_err(),
        AnnotationError::Invalid(ValueError::VersionUnsupported { found: 3, .. })
    ));
    assert!(a.get_as::<Note>("item", "nope").unwrap().is_none());
}

/// The difference finder names the first path that differs, both
/// sides, and treats key order as no difference.
#[test]
fn the_first_difference_names_the_path_and_both_sides() {
    let same = json!({ "b": [1, { "c": 2 }], "a": 1 });
    let reordered = json!({ "a": 1, "b": [1, { "c": 2 }] });
    assert_eq!(first_difference(&same, &reordered, ""), None);
    assert_eq!(
        first_difference(
            &json!({ "a": { "b": [1, 2] } }),
            &json!({ "a": { "b": [1, 3] } }),
            ""
        ),
        Some(("/a/b/1".into(), Some(json!(2)), Some(json!(3))))
    );
    assert_eq!(
        first_difference(&json!({ "a": [1] }), &json!({ "a": [1, 2] }), ""),
        Some(("/a/1".into(), None, Some(json!(2))))
    );
    assert_eq!(
        first_difference(&json!({ "a": 1, "x": null }), &json!({ "a": 1 }), ""),
        Some(("/x".into(), Some(Value::Null), None))
    );
    assert_eq!(
        first_difference(&json!(1), &json!("1"), ""),
        Some(("/".into(), Some(json!(1)), Some(json!("1"))))
    );
}

/// C35 — a backup is a whole, standalone annotation file that leaves
/// no partial and never overwrites. That the copy is consistent under
/// a concurrent writer is `VACUUM INTO`'s and no test here drives a
/// writer through an export: a torn copy fails too rarely to pin.
#[test]
fn export_is_a_whole_standalone_file_and_never_overwrites() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let uuid = "00000000-0000-4000-8000-000000000001";
    let mut a = Annotations::open(&annotations_path(&dir, uuid)).unwrap();
    a.put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
        .unwrap();
    let backup = dir.join("backup.db");
    a.export(&backup).unwrap();
    let leftovers: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().contains(".partial"))
        .collect();
    assert!(
        leftovers.is_empty(),
        "no partial outlives the export: {leftovers:?}"
    );
    // The snapshot is a complete, standalone annotation file.
    let restored = Annotations::open(&backup).unwrap();
    assert_eq!(restored.list(None, None).unwrap().rows.len(), 1);
    // A second export to the same path is refused, not an overwrite —
    // and refused before anything is written.
    assert!(a.export(&backup).is_err());
}

/// C35 — two exports racing one destination: each writes its own
/// partial, exactly one is published, the other is refused as
/// AlreadyExists, and the published file is a whole backup.
#[test]
fn simultaneous_exports_to_one_destination_publish_exactly_one() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-x");
    let mut a = Annotations::open(&path).unwrap();
    a.put::<Buyout>("item", "i1", &price("1c"), None, &via_test())
        .unwrap();
    let b = Annotations::open(&path).unwrap();
    let backup = dir.join("backup.db");
    let dest = &backup;
    let results = std::thread::scope(|scope| {
        let ta = scope.spawn(move || a.export(dest));
        let tb = scope.spawn(move || b.export(dest));
        [ta.join().unwrap(), tb.join().unwrap()]
    });
    let (ok, err): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
    assert_eq!((ok.len(), err.len()), (1, 1), "one backup, one refusal");
    match err.into_iter().next().unwrap().unwrap_err() {
        AnnotationError::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::AlreadyExists),
        other => panic!("expected AlreadyExists, got {other}"),
    }
    let restored = Annotations::open(&backup).unwrap();
    assert_eq!(restored.list(None, None).unwrap().rows.len(), 1);
    let leftovers = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().contains(".partial"))
        .count();
    assert_eq!(leftovers, 0);
}

/// A file that exists with content but no schema stamp is refused and
/// left as found — never adopted as empty intent; a zero-byte file
/// holds nothing and is created over.
#[test]
fn an_unstamped_file_with_content_is_refused_and_an_empty_one_is_created_over() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-1");
    {
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(
            "CREATE TABLE annotations (scope TEXT, key TEXT, kind TEXT, value TEXT);
                 INSERT INTO annotations VALUES ('item', 'i1', 'buyout', '{}');",
        )
        .unwrap();
        // no user_version stamp
    }
    let before = std::fs::read(&path).unwrap();
    match Annotations::open(&path).err() {
        Some(AnnotationError::SchemaTooOld {
            found: 0, path: p, ..
        }) => {
            assert_eq!(p, path);
        }
        other => panic!("expected SchemaTooOld found 0, got {other:?}"),
    }
    assert_eq!(std::fs::read(&path).unwrap(), before, "left as found");
    let err = match Annotations::open(&path) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("opened"),
    };
    assert!(err.contains("no schema stamp"), "{err}");
    // Zero bytes: nothing to preserve.
    let empty = annotations_path(&dir, "u-2");
    std::fs::write(&empty, b"").unwrap();
    let a = Annotations::open_for(&dir, "u-2").unwrap();
    assert_eq!(a.uuid(), Some("u-2"));
}

/// C35 — the creation race's second window, held open by hand: the
/// winner has created the file and its tables inside its immediate
/// transaction and has not yet committed the stamp. A losing first
/// opener must wait on the lock and then proceed as if it had found
/// the stamp — never be refused as a foreign file (seen once in five
/// gate runs, 2026-09-09, before this pin).
#[test]
fn c35_a_losing_first_opener_waits_for_the_winners_stamp_and_proceeds() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-w");
    // Scoped, the winner inside: a failing assertion drops the winner,
    // so the loser gets the lock and finishes before the folder goes.
    let opened = std::thread::scope(|s| {
        let winner = Connection::open(&path).unwrap();
        // As a creator does: the journal-mode switch writes the file's
        // first page, so the file exists with content before the stamp.
        winner.pragma_update(None, "journal_mode", "WAL").unwrap();
        winner.execute_batch("BEGIN IMMEDIATE").unwrap();
        winner.execute_batch(SCHEMA).unwrap();
        // The file exists with the winner's uncommitted tables; no stamp.
        let (tx, rx) = std::sync::mpsc::channel();
        let dir = &dir;
        let loser = s.spawn(move || {
            let opened = Annotations::open_for(dir, "u-w").map(|a| a.uuid().map(str::to_string));
            tx.send(()).unwrap();
            opened
        });
        // The loser is on the lock, not refused: nothing arrives while
        // the winner holds it.
        assert!(
            rx.recv_timeout(std::time::Duration::from_millis(400))
                .is_err(),
            "the loser returned while the winner still held its create"
        );
        winner
            .pragma_update(None, "user_version", SCHEMA_VERSION)
            .unwrap();
        winner.execute_batch("COMMIT").unwrap();
        drop(winner);
        loser.join().unwrap()
    });
    match opened {
        Ok(uuid) => assert_eq!(uuid.as_deref(), Some("u-w")),
        Err(e) => panic!("the loser was refused: {e}"),
    }
    // And the file is a v3 file both can use.
    let again = Annotations::open_for(&dir, "u-w").unwrap();
    assert_eq!(again.uuid(), Some("u-w"));
}

/// C35 — the other boundary under the same hold: a writer that lands
/// tables under the lock and commits *without* a stamp is a foreign
/// file by the time the loser gets the lock. The loser is refused
/// there, and the file is byte for byte what the foreign writer left —
/// no journal-mode switch, no schema, no stamp of ours.
#[test]
fn c35_a_foreign_writer_landing_under_the_lock_is_refused_and_left_untouched() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-f");
    // Scoped, the foreign writer inside: a failing assertion drops it,
    // so the loser gets the lock and finishes before the folder goes.
    let (after_commit, loser) = std::thread::scope(|s| {
        let foreign = Connection::open(&path).unwrap();
        foreign.execute_batch("BEGIN IMMEDIATE").unwrap();
        foreign
            .execute_batch("CREATE TABLE theirs (x INTEGER); INSERT INTO theirs VALUES (1);")
            .unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        let path = &path;
        let loser = s.spawn(move || {
            let r = Annotations::open(path)
                .map(|_| ())
                .map_err(|e| e.to_string());
            tx.send(()).unwrap();
            r
        });
        assert!(
            rx.recv_timeout(std::time::Duration::from_millis(400))
                .is_err(),
            "the loser returned while the foreign writer still held its lock"
        );
        foreign.execute_batch("COMMIT").unwrap();
        drop(foreign);
        let after_commit = std::fs::read(path).unwrap();
        (after_commit, loser.join().unwrap())
    });
    let err = loser.expect_err("a stampless file with tables opened");
    assert!(err.contains("no schema stamp"), "{err}");
    assert_eq!(std::fs::read(&path).unwrap(), after_commit, "left as found");
    assert!(
        !path.with_extension("annotations.db-wal").exists()
            && !dir.join("u-f.annotations.db-wal").exists(),
        "a journal-mode switch touched the foreign file"
    );
}

/// C35 — the intent file is fully synchronous: a commit that returned
/// survives a power loss, not only a process crash (the fact store's
/// NORMAL is fine for refetchable facts, not here).
#[test]
fn c35_the_intent_file_is_fully_synchronous() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let a = Annotations::open_for(&dir, "u-1").unwrap();
    let sync: i64 = a
        .conn
        .query_row("PRAGMA synchronous", [], |r| r.get(0))
        .unwrap();
    assert_eq!(sync, 2, "PRAGMA synchronous must be FULL (2)");
    let mode: String = a
        .conn
        .query_row("PRAGMA journal_mode", [], |r| r.get(0))
        .unwrap();
    assert_eq!(mode, "wal");
}

/// C35 — the file is bound to the account uuid inside it; a copy under another account's name is refused.
#[test]
fn open_for_stamps_the_uuid_and_refuses_a_foreign_file() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    // First open stamps; reopening from the raw path still knows the
    // owner, because the uuid lives inside the file.
    {
        let a = Annotations::open_for(&dir, "u-1").unwrap();
        assert_eq!(a.uuid(), Some("u-1"));
    }
    let raw = Annotations::open(&annotations_path(&dir, "u-1")).unwrap();
    assert_eq!(raw.uuid(), Some("u-1"));
    // u-1's file copied over u-2's path: open_for(u-2) refuses to
    // rebind it rather than adopting the copy.
    std::fs::copy(annotations_path(&dir, "u-1"), annotations_path(&dir, "u-2")).unwrap();
    match Annotations::open_for(&dir, "u-2").err() {
        Some(AnnotationError::WrongAccount { stored, requested }) => {
            assert_eq!((stored.as_str(), requested.as_str()), ("u-1", "u-2"));
        }
        other => panic!("expected WrongAccount, got {other:?}"),
    }
}

/// C65 — v3 is the floor: a file a development build wrote below it
/// is refused naming the file and the fix, never migrated with a
/// writer invented for its rows, and never touched.
#[test]
fn c65_a_file_below_the_floor_is_refused_never_migrated() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-1");
    let v2_table = "CREATE TABLE annotations (
                scope TEXT NOT NULL, key TEXT NOT NULL, kind TEXT NOT NULL,
                value TEXT NOT NULL, revision INTEGER NOT NULL,
                created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
                deleted_at INTEGER, PRIMARY KEY (scope, key, kind));
            CREATE TABLE meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
            INSERT INTO meta VALUES ('account_uuid', 'u-1');
            INSERT INTO annotations VALUES ('account', '', 'sync-policy', '{\"version\":1}', 9, 1, 2, NULL);";
    {
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(v2_table).unwrap();
        conn.pragma_update(None, "user_version", 2).unwrap();
    }
    let before = std::fs::read(&path).unwrap();
    match Annotations::open_for(&dir, "u-1").err() {
        Some(AnnotationError::SchemaTooOld {
            found: 2,
            floor: 3,
            path: p,
        }) => assert_eq!(p, path),
        other => panic!("expected SchemaTooOld, got {other:?}"),
    }
    let err = match Annotations::open(&path) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("a v2 file opened"),
    };
    assert!(
        err.contains("v2") && err.contains("floor v3") && err.contains("delete it"),
        "{err}"
    );
    assert_eq!(
        std::fs::read(&path).unwrap(),
        before,
        "refused, not touched"
    );
    // A fresh file is created at the current version outright.
    let a = Annotations::open_for(&dir, "u-2").unwrap();
    let v: i64 = a
        .conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(v, SCHEMA_VERSION);
}

/// C35 — a file a newer build wrote is refused, never guessed at.
#[test]
fn a_newer_schema_is_refused_not_guessed_at() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let path = annotations_path(&dir, "u-1");
    {
        let a = Annotations::open(&path).unwrap();
        a.conn.pragma_update(None, "user_version", 99).unwrap();
    }
    match Annotations::open(&path).err() {
        Some(AnnotationError::SchemaTooNew { found: 99, .. }) => {}
        other => panic!("expected SchemaTooNew, got {other:?}"),
    }
}

fn file_hash(path: &Path) -> Vec<u8> {
    use sha2::Digest;
    sha2::Sha256::digest(std::fs::read(path).unwrap()).to_vec()
}

/// C124 (a read never migrates): the annotations file's read-only
/// door refuses every older file and one a version ahead with
/// `NotCurrent`, in words naming both versions and the remedy, and
/// leaves its bytes as found. The remedy is the writing door's: a file
/// below the floor (v1, v2) is told to be deleted and its policy set
/// again, never to be migrated, as `init` refuses it; one from the
/// floor up, which `init` would migrate, is told to open it with `acq`.
/// An unstamped (empty) file is refused with the next step and left
/// empty; a missing file is refused, not created.
#[test]
fn c124_a_read_only_open_refuses_another_version_and_leaves_the_file_unchanged() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    assert!(Annotations::open_read_only_for(&dir, "u-0").is_err());
    assert!(
        !annotations_path(&dir, "u-0").exists(),
        "a read-only open created the file"
    );
    for stamp in (1..SCHEMA_VERSION).chain([SCHEMA_VERSION + 1]) {
        let uuid = format!("u-{stamp}");
        {
            let a = Annotations::open_for(&dir, &uuid).unwrap();
            a.conn.pragma_update(None, "user_version", stamp).unwrap();
        }
        let path = annotations_path(&dir, &uuid);
        let before = file_hash(&path);
        let err = Annotations::open_read_only_for(&dir, &uuid).err();
        match &err {
            Some(AnnotationError::NotCurrent(e)) => assert_eq!(
                (e.file, e.found, e.supported),
                ("annotation", stamp, SCHEMA_VERSION)
            ),
            other => panic!("expected NotCurrent, got {other:?}"),
        }
        let text = err.map(|e| e.to_string()).unwrap_or_default();
        assert!(
            text.contains(&format!("schema version {stamp}"))
                && text.contains(&format!("this build's version {SCHEMA_VERSION}")),
            "{text}"
        );
        let remedy = if stamp < SCHEMA_FLOOR {
            "a development build wrote it; delete it and set the policy again"
        } else if stamp < SCHEMA_VERSION {
            "open it with `acq` once to migrate, or use the build that wrote it"
        } else {
            "use the build that wrote it"
        };
        assert!(text.contains(remedy), "{text}");
        assert_eq!(
            file_hash(&path),
            before,
            "the refused v{stamp} file changed"
        );
        if stamp < SCHEMA_FLOOR {
            assert!(!text.contains("migrat"), "{text}");
            // The writing door refuses it in the same words.
            let writing = Annotations::open_for(&dir, &uuid).err();
            assert!(
                matches!(writing, Some(AnnotationError::SchemaTooOld { .. })),
                "{writing:?}"
            );
            let writing = writing.map(|e| e.to_string()).unwrap_or_default();
            assert!(
                writing.contains("delete it and set the policy again"),
                "{writing}"
            );
        }
    }
    let empty = annotations_path(&dir, "u-empty");
    std::fs::write(&empty, b"").unwrap();
    match Annotations::open_read_only_for(&dir, "u-empty").err() {
        Some(AnnotationError::NotCurrent(e)) => {
            assert_eq!(e.found, 0);
            let text = e.to_string();
            assert!(
                text.contains("carries no schema version")
                    && text.contains(
                        "check that `--dir` and `--account` name an existing Acquisition store"
                    ),
                "{text}"
            );
        }
        other => panic!("expected NotCurrent, got {other:?}"),
    }
    assert!(
        std::fs::read(&empty).unwrap().is_empty(),
        "the empty file was written"
    );
}

/// C124: a current file opens through the read-only door bound to
/// the uuid it stores and reads; a write through it is refused; its
/// bytes are unchanged; another account's file is refused as
/// `WrongAccount`, as `open_for` refuses it.
#[test]
fn c124_a_read_only_open_reads_a_current_file_and_writes_nothing() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    {
        let mut a = Annotations::open_for(&dir, "u-1").unwrap();
        a.put::<Buyout>("item", "i1", &price("1 divine"), None, &via_test())
            .unwrap();
    }
    let path = annotations_path(&dir, "u-1");
    let before = file_hash(&path);
    let mut ro = Annotations::open_read_only_for(&dir, "u-1").unwrap();
    assert_eq!(ro.uuid(), Some("u-1"));
    assert_eq!(ro.list(None, None).unwrap().rows.len(), 1);
    assert!(
        ro.put::<Buyout>("item", "i2", &price("2 divine"), None, &via_test())
            .is_err(),
        "a write through the read-only door landed"
    );
    drop(ro);
    assert_eq!(file_hash(&path), before, "the current file changed");
    std::fs::copy(&path, annotations_path(&dir, "u-2")).unwrap();
    match Annotations::open_read_only_for(&dir, "u-2").err() {
        Some(AnnotationError::WrongAccount { stored, requested }) => {
            assert_eq!((stored.as_str(), requested.as_str()), ("u-1", "u-2"));
        }
        other => panic!("expected WrongAccount, got {other:?}"),
    }
}

/// C35 — named by the account uuid, never the username.
#[test]
fn the_file_is_named_by_uuid() {
    let p = annotations_path(Path::new("/store/mock"), "0000-4000#odd");
    assert_eq!(p.file_name().unwrap(), "0000-4000_odd.annotations.db");
}

/// C35, the first-open race's third window (round 25), staged
/// exactly: the loser's connection carries an authorizer that fires
/// when its second read — the table count — is prepared, after the
/// stamp was read; there the winner attempts, on its own connection,
/// to create the schema, stamp it and commit. Under the round-24
/// gate the loser held no lock between its two statements, so the
/// attempt commits and the loser then sees stamp 0 beside the
/// winner's tables and refuses a well-formed file as "content but no
/// schema stamp". Under the fixed gate the loser's stamp read already
/// holds the snapshot, so the attempt cannot commit between the
/// reads — it waits on the loser and gives up after its busy
/// timeout, which the pass path asserts — and the loser opens the
/// file. What this test asserts is the fixed outcome — the attempt
/// could not commit and the loser opened the file; the old outcome
/// was observed by breaking the gate (the round-24 shape staged
/// back, review rounds 25 and 26: the attempt committed and the
/// loser refused with the production message), not asserted here. A
/// stress test of 3000 concurrent first opens did not reproduce the
/// window; this does, every run.
#[test]
fn c35_a_create_attempted_between_the_gates_two_reads_cannot_commit_or_cause_a_refusal() {
    use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, Ordering},
    };
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().to_path_buf();
    let path = base.join("u-race.annotations.db");
    let loser = Connection::open(&path).unwrap();
    let fired = Arc::new(AtomicBool::new(false));
    let committed = Arc::new(AtomicU32::new(0));
    let (winner_path, fired_in, committed_in) = (path.clone(), fired.clone(), committed.clone());
    loser.authorizer(Some(move |ctx: AuthContext<'_>| {
        if let AuthAction::Read { table_name, .. } = ctx.action
            && table_name == "sqlite_master"
            && !fired_in.swap(true, Ordering::SeqCst)
        {
            // The winner, between the loser's two reads: a short
            // busy timeout, since under the fixed gate its commit
            // waits on the loser's snapshot and must give up.
            let winner = Connection::open(&winner_path).unwrap();
            winner
                .busy_timeout(std::time::Duration::from_millis(200))
                .unwrap();
            let create = (|| -> rusqlite::Result<()> {
                winner.execute_batch("BEGIN IMMEDIATE")?;
                winner.execute_batch(SCHEMA)?;
                winner.pragma_update(None, "user_version", SCHEMA_VERSION)?;
                winner.execute_batch("COMMIT")
            })();
            if create.is_ok() {
                committed_in.fetch_add(1, Ordering::SeqCst);
            } else {
                let _ = winner.execute_batch("ROLLBACK");
            }
        }
        Authorization::Allow
    }));
    let opened = Annotations::init(loser, path.clone());
    assert!(fired.load(Ordering::SeqCst), "the staging never fired");
    match opened {
        // The fixed gate: the attempt could not commit between the
        // loser's reads, and the loser opened the file.
        Ok(_) => assert_eq!(
            committed.load(Ordering::SeqCst),
            0,
            "the winner committed between the loser's two reads"
        ),
        Err(e) => panic!(
            "the loser refused the winner's file (winner committed between the reads: {}): {e}",
            committed.load(Ordering::SeqCst) == 1
        ),
    }
    // Whoever created it, the file is one well-formed store afterwards.
    if let Err(e) = Annotations::open(&path) {
        panic!("the file is not a well-formed store afterwards: {e}");
    }
}
