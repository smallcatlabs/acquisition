use super::*;

#[test]
fn round_trip_and_resolution() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    let mut idx = Index::load(dir).unwrap();
    assert_eq!(idx.resolve(None), Err(Resolve::Empty));
    idx.upsert("Alice#1234", true, 10, &[]).unwrap();
    assert_eq!(idx.resolve(None).unwrap().username, "Alice#1234");
    idx.upsert("Bob#0001", false, 20, &[]).unwrap();
    assert!(matches!(idx.resolve(None), Err(Resolve::Ambiguous(ref v)) if v.len() == 2));
    assert_eq!(idx.resolve(Some("alice")).unwrap().username, "Alice#1234");
    assert_eq!(
        idx.resolve(Some("Alice#1234")).unwrap().username,
        "Alice#1234"
    );
    assert_eq!(
        idx.resolve(Some("Ali")),
        Err(Resolve::NotFound("Ali".into()))
    );
    idx.set_uuid("Bob#0001", "u-bob").unwrap();
    assert_eq!(idx.resolve(Some("u-bob")).unwrap().username, "Bob#0001");
    // Reload from disk: same content; persisted order is by recency.
    let idx = Index::load(dir).unwrap();
    assert_eq!(idx.entries().len(), 2);
    assert_eq!(
        idx.persisted()
            .iter()
            .map(|e| e.username.as_str())
            .collect::<Vec<_>>(),
        vec!["Alice#1234"]
    );
    let mut idx = idx;
    idx.upsert("Bob#0001", true, 30, &[]).unwrap();
    assert_eq!(idx.persisted()[0].username, "Bob#0001");
    idx.set_persisted("Bob#0001", false).unwrap();
    assert_eq!(idx.persisted().len(), 1);
    assert_eq!(
        account_path(dir, "Alice#1234").file_name().unwrap(),
        "Alice_1234.db"
    );
}

#[test]
fn record_login_requires_the_uuid_and_follows_renames() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    let mut idx = Index::load(dir).unwrap();
    idx.record_login("Alice#1234", "u-alice", true, 10, &[])
        .unwrap();
    let e = idx.get("Alice#1234").unwrap();
    assert_eq!((e.uuid.as_deref(), e.persisted), (Some("u-alice"), true));
    assert_eq!(idx.resolve(Some("u-alice")).unwrap().username, "Alice#1234");
    // A later login with the same uuid under a new name is a rename:
    // the mapping updates, no second entry appears.
    idx.record_login("Alicia#9999", "u-alice", true, 20, &[])
        .unwrap();
    assert_eq!(idx.entries().len(), 1);
    assert_eq!(
        idx.resolve(Some("u-alice")).unwrap().username,
        "Alicia#9999"
    );
    assert!(idx.get("Alice#1234").is_none());
    // A pre-uuid leftover entry gains its uuid in place.
    idx.upsert("Bob#0001", true, 30, &[]).unwrap();
    assert!(idx.get("Bob#0001").unwrap().uuid.is_none());
    idx.record_login("Bob#0001", "u-bob", true, 40, &[])
        .unwrap();
    assert_eq!(idx.entries().len(), 2);
    assert_eq!(idx.get("Bob#0001").unwrap().uuid.as_deref(), Some("u-bob"));
    // A rename onto a name a uuid-less leftover holds drops the twin.
    idx.upsert("Cleo#0002", false, 50, &[]).unwrap();
    idx.record_login("Cleo#0002", "u-alice", true, 60, &[])
        .unwrap();
    let cleos: Vec<_> = idx
        .entries()
        .iter()
        .filter(|e| e.username == "Cleo#0002")
        .collect();
    assert_eq!(cleos.len(), 1);
    assert_eq!(cleos[0].uuid.as_deref(), Some("u-alice"));
}

/// Issue #74: when no entry matches a selector by username or uuid, a
/// name an entry holds for its keyring matches it whole
/// (case-insensitive), never without its discriminator or by prefix;
/// a current username wins over another entry's held name, and a held
/// name two entries match is ambiguous.
#[test]
fn a_selector_matches_a_held_keyring_name() {
    let temp = tempfile::tempdir().unwrap();
    let mut idx = Index::load(temp.path()).unwrap();
    idx.record_login(
        "test-user",
        "u-1",
        true,
        1,
        &["B#2".into(), "Carol#3".into()],
    )
    .unwrap();
    idx.record_login("Carol#3", "u-3", true, 2, &[]).unwrap();
    for sel in ["B#2", "b#2"] {
        assert_eq!(
            idx.resolve(Some(sel)).unwrap().username,
            "test-user",
            "{sel}"
        );
    }
    for sel in ["B", "B#"] {
        assert_eq!(idx.resolve(Some(sel)), Err(Resolve::NotFound(sel.into())));
    }
    // A current username, whole or without its discriminator, wins
    // over the name another entry holds.
    for sel in ["Carol#3", "carol"] {
        assert_eq!(idx.resolve(Some(sel)).unwrap().username, "Carol#3", "{sel}");
    }

    idx.record_login("Dave#4", "u-4", true, 3, &["B#2".into()])
        .unwrap();
    assert!(matches!(
        idx.resolve(Some("B#2")),
        Err(Resolve::Ambiguous(ref v)) if *v == ["test-user", "Dave#4"]
    ));
}

/// C144 (no keyring entry out of the index's reach), the review of the
/// h1 fix round, its second finding: a rename records the new username
/// and the old name its keyring entry may still be under in one
/// write, so a process that stops after that write — staged as a
/// crash that lets one write land — leaves an index that reaches
/// both. Before, the username and the held name were two writes, and
/// a stop between them left the old name's entry out of every verb's
/// reach.
#[test]
fn c144_a_stop_after_the_renames_write_leaves_the_old_name_in_reach() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    let mut idx = Index::load(dir).unwrap();
    idx.record_login("Alice#1", "u-a", true, 10, &["Alice#1".into()])
        .unwrap();

    // The rename's persist: the new name saved, the old not cleared.
    idx.saves_left.set(Some(1));
    let _ = idx.record_login(
        "Alicia#2",
        "u-a",
        true,
        20,
        &["Alice#1".into(), "Alicia#2".into()],
    );
    let read = Index::load(dir).unwrap();
    let entry = read.find(Some("u-a"), "Alicia#2").unwrap();
    assert_eq!(entry.username, "Alicia#2");
    for name in ["Alice#1", "Alicia#2"] {
        assert!(
            entry.keyring_reach().iter().any(|n| n == name),
            "{name} in reach of {entry:?}"
        );
    }
}

/// C144: a held name leaves only on a clear that succeeded, a failed
/// clear holds its name, and an entry written with the fix round's
/// single `keyring_name` reads into the list and is never written
/// with it again.
#[test]
fn c144_a_name_leaves_only_on_its_successful_clear() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    std::fs::write(
            index_path(dir),
            r#"{"accounts":[{"username":"C#3","last_login":1,"persisted":true,"uuid":"u-a","keyring_name":"A#1"}]}"#,
        )
        .unwrap();
    let mut idx = Index::load(dir).unwrap();
    assert_eq!(idx.find(Some("u-a"), "x").unwrap().keyring_names, ["A#1"]);
    idx.settle_keyring_names(
        Some("u-a"),
        "x",
        &["B#2".into()],
        &["A#1".into(), "C#3".into()],
        Some(false),
    )
    .unwrap();
    let text = std::fs::read_to_string(index_path(dir)).unwrap();
    assert!(!text.contains("\"keyring_name\""), "{text}");
    let read = Index::load(dir).unwrap();
    let entry = read.get("C#3").unwrap();
    assert_eq!(entry.keyring_names, ["B#2"]);
    assert!(!entry.persisted);
}

#[test]
fn corrupt_index_is_an_error_not_a_reset() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    std::fs::write(index_path(dir), "{nope").unwrap();
    assert!(Index::load(dir).is_err());
}

/// C144 (the index is written atomically), issue #63: a writer that
/// stops between writing its temporary file and the rename — a crash
/// staged by writing the temporary and never renaming it — leaves the
/// index as it was, readable whole, and the next save replaces it.
#[test]
fn c144_a_crash_between_write_and_rename_leaves_the_index_whole() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    let mut idx = Index::load(dir).unwrap();
    idx.upsert("Alice#1234", true, 10, &[]).unwrap();
    let before = std::fs::read_to_string(index_path(dir)).unwrap();

    idx.entries.push(AccountEntry {
        username: "Bob#0001".into(),
        last_login: 20,
        persisted: true,
        uuid: None,
        keyring_names: Vec::new(),
    });
    let orphan = idx.write_temp().unwrap();
    assert!(orphan.exists());
    assert_eq!(std::fs::read_to_string(index_path(dir)).unwrap(), before);
    let read = Index::load(dir).unwrap();
    assert_eq!(read.entries().len(), 1, "the old index, whole");

    idx.save().unwrap();
    assert_eq!(Index::load(dir).unwrap().entries().len(), 2);
}

/// C144, issue #63: two writers for two accounts saving at once — two
/// processes, or a daemon writer beside a stray one — never tear the
/// file: each save has a temporary of its own, so a reader always
/// parses an index one of them wrote whole. (That neither loses the
/// other's change is the daemon's lock: its every index write takes
/// the credentials hold as a value, C148.)
#[test]
fn c144_two_accounts_saving_at_once_never_tear_the_index() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    Index::load(&dir)
        .unwrap()
        .upsert("seed", false, 0, &[])
        .unwrap();
    // Scoped: every thread has finished before the folder goes, a
    // panicking one included.
    std::thread::scope(|s| {
        for name in ["Alice#1234", "Bob#0001"] {
            let dir = &dir;
            s.spawn(move || {
                for at in 0..50 {
                    let mut idx = Index::load(dir).unwrap();
                    idx.upsert(name, at % 2 == 0, at, &[]).unwrap();
                }
            });
        }
        s.spawn(|| {
            for _ in 0..100 {
                Index::load(&dir).expect("never a torn index");
            }
        });
    });
    assert!(Index::load(&dir).is_ok());
}
