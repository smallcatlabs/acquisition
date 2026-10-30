use super::*;

/// C83: a world is a canonical root — one id for two spellings of
/// one directory — with its provider directories, lock file and
/// rails state under it, and its diagnostics keyed by that id under
/// the log base.
#[test]
fn c83_a_world_is_a_canonical_root_and_its_paths_derive_from_it() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(base.join("real")).unwrap();
    let world = World::observe_at(&base.join("real")).unwrap();
    assert!(world.root().is_absolute());
    // A dotted spelling of the same directory is the same world.
    let dotted = World::observe_at(&base.join("real").join(".").join("..").join("real")).unwrap();
    assert_eq!(world, dotted);
    assert_eq!(world.id(), dotted.id());
    assert_eq!(world.id().len(), 12);
    assert!(world.id().bytes().all(|b| b.is_ascii_hexdigit()));
    assert_eq!(world.provider_dir("mock"), world.root().join("mock"));
    assert_eq!(world.lock_path(), world.root().join("daemon.lock"));
    assert_eq!(
        world.rails_state_path("ggg"),
        world.root().join("ggg").join("rails.json")
    );
    assert_eq!(
        world.log_path("mock"),
        log_base_dir()
            .join(world.id())
            .join("mock")
            .join("daemon.log")
    );
    assert_eq!(
        world.journal_path("mock"),
        log_base_dir()
            .join(world.id())
            .join("mock")
            .join("sends.jsonl")
    );
    // Another directory is another world.
    std::fs::create_dir_all(base.join("other")).unwrap();
    let other = World::observe_at(&base.join("other")).unwrap();
    assert_ne!(world, other);
    assert_ne!(world.id(), other.id());
    // A missing root is an error naming it, never a world.
    let err = World::observe_at(&base.join("missing")).unwrap_err();
    assert!(err.absent, "{err}");
    assert!(err.to_string().contains("missing"), "{err}");
}

/// C83: the world lock is exclusive and names its holder; it is
/// released when the `Lock` is dropped, however that happens.
#[test]
fn c83_the_lock_is_exclusive_and_names_its_holder() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();
    let path = base.join("daemon.lock");
    let held = Lock::acquire(&path).unwrap();
    assert_eq!(held.path(), path.as_path());
    assert_eq!(
        std::fs::read_to_string(Lock::pid_path(&path))
            .unwrap()
            .trim(),
        std::process::id().to_string(),
        "the holder wrote its pid"
    );
    // A second acquisition from this process is a second open file
    // description: flock refuses it, and the refusal names the holder.
    let err = Lock::acquire(&path).unwrap_err();
    assert_eq!(err.holder, Some(std::process::id()), "{err}");
    assert!(err.io.is_none(), "{err}");
    assert!(
        err.to_string().contains("held by another process")
            && err.to_string().contains(&std::process::id().to_string()),
        "{err}"
    );
    drop(held);
    let again = Lock::acquire(&path).expect("released on drop");
    drop(again);
}

/// `path` at `mode` until this drops, then back at the mode it had, so
/// the folder can go however the test ends. The daemon harness's
/// `with_mode`, which this crate's tests cannot reach.
#[cfg(unix)]
struct WithMode<'a> {
    path: &'a std::path::Path,
    was: std::fs::Permissions,
}

#[cfg(unix)]
fn with_mode(path: &std::path::Path, mode: u32) -> WithMode<'_> {
    use std::os::unix::fs::PermissionsExt;
    let was = std::fs::metadata(path).unwrap().permissions();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
    WithMode { path, was }
}

#[cfg(unix)]
impl Drop for WithMode<'_> {
    fn drop(&mut self) {
        let _ = std::fs::set_permissions(self.path, self.was.clone());
    }
}

/// C83: the private runtime directory is this user's: made 0700,
/// brought back to 0700 when it drifted, refused when it is a symlink
/// or a file.
#[cfg(unix)]
#[test]
fn c83_the_private_dir_is_this_users() {
    use std::os::unix::fs::MetadataExt;
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();
    let fresh = private_dir(&base.join("fresh")).unwrap();
    assert_eq!(std::fs::metadata(&fresh).unwrap().mode() & 0o777, 0o700);
    for (name, drifted) in [("loose", 0o755), ("tight", 0o500), ("shut", 0o000)] {
        let dir = base.join(name);
        std::fs::create_dir(&dir).unwrap();
        // Held through the assertion, so the restore cannot hide a
        // regression; a failing one still leaves the folder removable.
        let drift = with_mode(&dir, drifted);
        private_dir(&dir).unwrap();
        assert_eq!(
            std::fs::metadata(&dir).unwrap().mode() & 0o7777,
            0o700,
            "{name}: restored to exactly 0700"
        );
        drop(drift);
    }
    let err = private_dir(&base.join("absent-parent").join("acq")).unwrap_err();
    assert!(err.to_string().contains("could not create"), "{err}");
    std::os::unix::fs::symlink(&fresh, base.join("link")).unwrap();
    let err = private_dir(&base.join("link")).unwrap_err();
    assert!(err.to_string().contains("symlink"), "{err}");
    std::fs::write(base.join("file"), b"").unwrap();
    let err = private_dir(&base.join("file")).unwrap_err();
    assert!(err.to_string().contains("not a directory"), "{err}");
    let runtime = app_runtime_dir().unwrap();
    assert_eq!(std::fs::metadata(&runtime).unwrap().mode() & 0o077, 0);
    // C132: the real-mode lock is no longer in the runtime directory
    // (`c83_the_lock_path_reads_no_environment` holds where it is).
    assert_ne!(
        real_mode_lock_location().unwrap().parent(),
        Some(runtime.as_path())
    );
}

/// C132: the real-mode lock's path is a pure function of the home the
/// system records and the platform's layout, so no variable moves it:
/// the shape for a sample home on each platform, the home read from
/// the password database (the process token on Windows) and nothing
/// else, and the lock out of the runtime directory, where the socket
/// stays (C112). Nothing here
/// sets or reads a variable; the process test in
/// `acquisition-cli/tests/world.rs` (`c83_c31_`) moves the variables
/// under two real daemons. The real path is computed, not created, so
/// the store's tests make nothing in the user's home.
#[test]
fn c83_the_lock_path_reads_no_environment() {
    assert_eq!(
        real_mode_run_dir_in(Path::new("/Users/someone"), HomeLayout::MacOs),
        Path::new("/Users/someone/Library/Application Support/com.smallcatlabs.acq/run")
    );
    assert_eq!(
        real_mode_run_dir_in(Path::new("/home/someone"), HomeLayout::Xdg),
        Path::new("/home/someone/.local/share/acq/run")
    );
    let windows = real_mode_run_dir_in(Path::new("C:/Users/someone"), HomeLayout::Windows);
    assert_eq!(
        windows.components().collect::<Vec<_>>(),
        Path::new("C:/Users/someone/AppData/Local/smallcatlabs/acq/run")
            .components()
            .collect::<Vec<_>>()
    );
    // The real derivation is the pure one over the system's home.
    let home = user_home().unwrap();
    assert!(home.is_absolute(), "{}", home.display());
    let lock = real_mode_lock_location().unwrap();
    assert_eq!(lock, real_mode_lock_in(&home));
    assert_eq!(
        lock.parent(),
        Some(real_mode_run_dir_in(&home, HomeLayout::HERE).as_path())
    );
    // Out of the runtime directory; the socket is still in it.
    let runtime = runtime_dir_path();
    assert_ne!(lock.parent(), Some(runtime.as_path()));
    assert!(!lock.starts_with(&runtime), "{}", lock.display());
    #[cfg(unix)]
    {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().join("base");
        std::fs::create_dir_all(base.join("real")).unwrap();
        let world = World::observe_at(&base.join("real")).unwrap();
        if let Ok(socket) = world.socket_path_under(&runtime) {
            assert_eq!(socket.parent(), Some(runtime.as_path()));
        }
    }
}

/// C132: a home that cannot hold the real-mode lock is refused by
/// name, never worked around: a home that is a file, a data directory
/// that cannot be made (a file where its first component under the
/// home should be), and a `run` that fails [`private_dir`] (a symlink
/// to a directory, a file) — each error names the path it met. A loose
/// `run` is made 0700, not refused (private_dir's restore). Every home
/// here is a scratch directory; the user's own is never touched.
#[test]
fn c132_a_home_that_cannot_hold_the_lock_is_refused_by_name() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();

    // The home is a file.
    let file_home = base.join("home-file");
    std::fs::write(&file_home, b"").unwrap();
    let err = real_mode_lock_made_under(&file_home).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains(&file_home.display().to_string())
            && msg.contains("home directory")
            && msg.contains("not a directory"),
        "{msg}"
    );
    // A missing home is the same refusal, and nothing is created.
    let missing = base.join("home-missing");
    let msg = real_mode_lock_made_under(&missing).unwrap_err().to_string();
    assert!(msg.contains(&missing.display().to_string()), "{msg}");
    assert!(!missing.exists());

    // The data directory cannot be made: its first component under
    // the home is a file.
    let home = base.join("home-blocked");
    std::fs::create_dir_all(&home).unwrap();
    let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
    let data = run.parent().unwrap();
    let first = home.join(
        data.strip_prefix(&home)
            .unwrap()
            .components()
            .next()
            .unwrap(),
    );
    std::fs::write(&first, b"").unwrap();
    let msg = real_mode_lock_made_under(&home).unwrap_err().to_string();
    assert!(
        msg.contains("could not create the data directory")
            && msg.contains(&data.display().to_string())
            && msg.contains("real-mode lock"),
        "{msg}"
    );

    // `run` is a file.
    let home = base.join("home-run-file");
    let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
    std::fs::create_dir_all(run.parent().unwrap()).unwrap();
    std::fs::write(&run, b"").unwrap();
    let msg = real_mode_lock_made_under(&home).unwrap_err().to_string();
    assert!(
        msg.contains(&run.display().to_string()) && msg.contains("not a directory"),
        "{msg}"
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // `run` is a symlink to a private directory of this user's.
        let home = base.join("home-run-link");
        let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
        std::fs::create_dir_all(run.parent().unwrap()).unwrap();
        let target = base.join("elsewhere");
        private_dir(&target).unwrap();
        std::os::unix::fs::symlink(&target, &run).unwrap();
        let msg = real_mode_lock_made_under(&home).unwrap_err().to_string();
        assert!(
            msg.contains(&run.display().to_string()) && msg.contains("is a symlink"),
            "{msg}"
        );

        // A loose `run` is restored to 0700, and the lock is in it.
        let home = base.join("home-run-loose");
        let run = real_mode_run_dir_in(&home, HomeLayout::HERE);
        std::fs::create_dir_all(&run).unwrap();
        std::fs::set_permissions(&run, std::fs::Permissions::from_mode(0o755)).unwrap();
        let lock = real_mode_lock_made_under(&home).unwrap();
        assert_eq!(lock, real_mode_lock_in(&home));
        let mode = std::fs::symlink_metadata(&run)
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o7777, 0o700);
    }

    // A fresh home gets its data directory and a private `run`.
    let home = base.join("home-fresh");
    std::fs::create_dir_all(&home).unwrap();
    let lock = real_mode_lock_made_under(&home).unwrap();
    assert_eq!(lock, real_mode_lock_in(&home));
    assert!(lock.parent().unwrap().is_dir());
}

/// C83: two of our own starts making one fresh private directory at
/// once both succeed: the one that loses the create's race meets
/// AlreadyExists and checks the directory as made, and the directory
/// passes the checks. Raced on a barrier, two hundred fresh paths, so
/// the race is met (a cold CI runner's first tests did, on Windows).
#[test]
fn two_starts_making_one_private_dir_at_once_both_succeed() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();
    let mut refused = Vec::new();
    for i in 0..200 {
        let path = base.join(format!("acq-{i}"));
        let barrier = std::sync::Barrier::new(2);
        // Scoped: both racers have finished before the folder goes.
        let results: Vec<_> = std::thread::scope(|s| {
            let racers: Vec<_> = (0..2)
                .map(|_| {
                    s.spawn(|| {
                        barrier.wait();
                        private_dir(&path)
                    })
                })
                .collect();
            racers.into_iter().map(|r| r.join().unwrap()).collect()
        });
        for result in results {
            if let Err(e) = result {
                refused.push(format!("{i}: {e}"));
            }
        }
        existing_private_dir(&path).unwrap();
    }
    assert!(
        refused.is_empty(),
        "{} of 400 calls refused: {refused:?}",
        refused.len()
    );
}

/// C83: a non-UTF-8 root is refused as a world. Linux only: APFS
/// refuses an invalid byte sequence in a name (EILSEQ), so on macOS
/// no such directory can exist and the filesystem holds the property.
#[cfg(target_os = "linux")]
#[test]
fn c83_a_root_that_is_not_utf8_is_refused() {
    use std::os::unix::ffi::OsStrExt;
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();
    let odd = base.join(std::ffi::OsStr::from_bytes(b"r\xff"));
    std::fs::create_dir(&odd).unwrap();
    let err = World::observe_at(&odd).unwrap_err();
    assert!(err.to_string().contains("UTF-8"), "{err}");
}

/// C83: the socket derives from the world into the runtime directory
/// and from nothing else — two spellings of one root reach one
/// socket, another root another. That no knob names it is held by
/// `tools/docs-check.sh`: a knob read anywhere in the crates needs a
/// README row, and the socket has none. The derivation's refusals —
/// over the cap, not UTF-8 — are pinned on the pure helper with
/// runtime directories this test names, since the real one is the
/// environment's (review 2026-09-11).
#[test]
fn c83_the_socket_derives_from_the_world_into_the_runtime_directory() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(base.join("real")).unwrap();
    std::fs::create_dir_all(base.join("other")).unwrap();
    // The derivation under a runtime directory this test names: the
    // world's id names the socket, two spellings of one root reach
    // one socket, another root another, all in that directory.
    let named = Path::new("/run/user/1000/acq");
    let world = World::observe_at(&base.join("real")).unwrap();
    let socket = world.socket_path_under(named).unwrap();
    assert_eq!(socket, named.join(world.socket_name()));
    assert_eq!(world.socket_name(), format!("{}.sock", world.id()));
    let dotted = World::observe_at(&base.join("real").join(".").join("..").join("real")).unwrap();
    assert_eq!(dotted.socket_path_under(named).unwrap(), socket);
    let other = World::observe_at(&base.join("other")).unwrap();
    assert_ne!(other.socket_path_under(named).unwrap(), socket);
    assert_eq!(
        other.socket_path_under(named).unwrap().parent(),
        socket.parent()
    );
    // The environment's runtime directory, whatever it is: the real
    // door agrees with the pure derivation over it — the same path
    // when that derivation accepts it, and a refusal of the same
    // kind when it does not (a deep or non-UTF-8 runtime directory
    // is a valid environment; review 2026-09-11).
    #[cfg(unix)]
    {
        let runtime = runtime_dir_path();
        match world.socket_path_under(&runtime) {
            Ok(expected) => {
                app_runtime_dir().unwrap();
                assert_eq!(world.socket_path().unwrap(), expected);
                assert_eq!(dotted.socket_path().unwrap(), expected);
            }
            Err(e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput, "{e}");
                let real = world.socket_path().unwrap_err();
                assert_eq!(real.kind(), std::io::ErrorKind::InvalidInput, "{real}");
                assert_eq!(real.to_string(), e.to_string());
            }
        }
    }
    // Windows (C112): the real door is the pipe of this user and this
    // world, the same for two spellings of one root, another for
    // another root.
    #[cfg(windows)]
    {
        let sid = crate::world_windows::current_user_sid().unwrap();
        let pipe = world.socket_path().unwrap();
        assert_eq!(
            pipe,
            PathBuf::from(format!(r"\\.\pipe\acq-{sid}-{}", world.id()))
        );
        assert_eq!(dotted.socket_path().unwrap(), pipe);
        assert_ne!(other.socket_path().unwrap(), pipe);
    }
    let deep = PathBuf::from("/").join("x".repeat(SOCKET_PATH_MAX));
    let err = world.socket_path_under(&deep).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(
        err.to_string().contains("at most 103") && err.to_string().contains("bytes"),
        "{err}"
    );
    // Exactly at the cap passes; one over is refused: the bound is the
    // path's byte length, terminator excluded.
    let name_len = world.socket_name().len();
    // "/" + the y's + "/" + the name: two separators.
    let at_cap = PathBuf::from("/".to_string() + &"y".repeat(SOCKET_PATH_MAX - name_len - 2));
    assert_eq!(
        world.socket_path_under(&at_cap).unwrap().as_os_str().len(),
        SOCKET_PATH_MAX
    );
    let over = PathBuf::from("/".to_string() + &"y".repeat(SOCKET_PATH_MAX - name_len - 1));
    assert!(world.socket_path_under(&over).is_err());
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        let odd = PathBuf::from(std::ffi::OsStr::from_bytes(b"/run/user/1000/acq-\xff"));
        let err = world.socket_path_under(&odd).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("not valid UTF-8")
                && err.to_string().contains("XDG_RUNTIME_DIR"),
            "{err}"
        );
    }
}

/// The rails state's fields are three owners'; a field this build
/// does not know is a later build's, and survives a read, a reset and
/// a write unchanged. A file holding only such a field is kept. The
/// write leaves nothing beside the file (its partial is renamed).
#[test]
fn the_rails_state_keeps_a_later_builds_fields() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();
    let path = base.join("rails.json");
    let later = r#"{"since":"2027-01-01","cause":"a later halt"}"#;
    std::fs::write(
            &path,
            format!(
                r#"{{"tripped":"429 on GET /x","later_build_mark":{later},"refresh_failed_by_account":{{"A#1":"400"}}}}"#
            ),
        )
        .unwrap();
    let mut held: RailsState =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert!(held.reset(), "the trip was cleared");
    held.write(&path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let written: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(
        written["later_build_mark"],
        serde_json::from_str::<serde_json::Value>(later).unwrap(),
        "{text}"
    );
    assert_eq!(written["refresh_failed_by_account"]["A#1"], "400", "{text}");
    assert!(written["tripped"].is_null(), "{text}");

    let only_later = RailsState {
        unknown: [("later_build_mark".to_string(), serde_json::json!(1))].into(),
        ..RailsState::default()
    };
    assert!(!only_later.holds_nothing());
    only_later.write(&path).unwrap();
    let reread: RailsState =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(reread, only_later);
    let beside: Vec<_> = std::fs::read_dir(&base)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(beside, ["rails.json"], "no partial left beside the file");
}

/// A rewrite keeps the file's mode: a rails state its owner narrowed
/// to 0600 is not widened to the umask's default by the next mark.
#[cfg(unix)]
#[test]
fn the_rails_state_rewrite_keeps_the_files_mode() {
    use std::os::unix::fs::PermissionsExt;
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().join("base");
    std::fs::create_dir_all(&base).unwrap();
    let path = base.join("rails.json");
    std::fs::write(&path, r#"{"tripped":"429 on GET /x"}"#).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let state = RailsState {
        blocked: Some("a block".into()),
        ..RailsState::default()
    };
    state.write(&path).unwrap();
    let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "the rewrite changed the mode to {mode:o}");
}
