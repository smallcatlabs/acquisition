//! The bound on the daemon's diagnostic files.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

#[cfg(unix)]
use crate::daemon::*;

/// C83: a diagnostic over the cap is rotated to `<name>.1`; when it
/// cannot be renamed (a directory that refuses new entries) it is
/// truncated in place and the loss is named — the bound holds either
/// way (review 2026-09-11); under the cap nothing happens.
#[cfg(unix)]
#[test]
fn c83_a_diagnostic_over_the_cap_is_rotated_or_truncated_never_kept() {
    use std::os::unix::fs::PermissionsExt;
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().to_path_buf();
    let log = dir.join("daemon.log");
    std::fs::write(&log, b"small").unwrap();
    assert_eq!(rotate_if_over(&log, 16), Ok(None));
    std::fs::write(&log, vec![b'x'; 32]).unwrap();
    let what = rotate_if_over(&log, 16).unwrap().expect("rotated");
    assert!(what.starts_with("rotated"), "{what}");
    assert!(!log.exists() && dir.join("daemon.log.1").exists());
    // A directory that refuses new entries: the rename fails, the file
    // is truncated in place, and the message says the previous
    // generation is lost.
    std::fs::write(&log, vec![b'y'; 32]).unwrap();
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o500)).unwrap();
    let what = rotate_if_over(&log, 16).unwrap().expect("bounded");
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).unwrap();
    assert!(
        what.starts_with("truncated") && what.contains("previous generation is lost"),
        "{what}"
    );
    assert_eq!(std::fs::metadata(&log).unwrap().len(), 0);
    assert_eq!(
        std::fs::read(dir.join("daemon.log.1")).unwrap(),
        vec![b'x'; 32],
        "the earlier generation is untouched"
    );
    // Neither: the directory refuses the rename and the file refuses
    // the truncation — the refusal that stops the daemon's start.
    std::fs::write(&log, vec![b'z'; 32]).unwrap();
    std::fs::set_permissions(&log, std::fs::Permissions::from_mode(0o400)).unwrap();
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o500)).unwrap();
    let err = rotate_if_over(&log, 16).unwrap_err();
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).unwrap();
    std::fs::set_permissions(&log, std::fs::Permissions::from_mode(0o600)).unwrap();
    assert!(
        err.contains("neither rotated") && err.contains("nor truncated"),
        "{err}"
    );
    assert_eq!(std::fs::metadata(&log).unwrap().len(), 32, "untouched");
}
