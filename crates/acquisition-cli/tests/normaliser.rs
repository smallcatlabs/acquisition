//! The golden normaliser's own tests (C122): one case per rule the
//! daemon-backed transcripts need, from text the CLI prints, the
//! negatives each rule must leave alone, and that normalising every
//! committed golden under `tests/goldens/` changes nothing, which guards
//! each later rule against rewriting what an earlier one wrote
//! (`<scratch>`, `<world>`, `<age>`, `<jN>`, `<pid>`). A module of `tests/main`,
//! never of the adapter: `watch_recovery.rs`'s binary takes the adapter
//! too and would run these twice.

use std::path::Path;

use crate::harness::normalise;

/// A base no text names, so the `scratch` rule never fires here.
fn base() -> &'static Path {
    Path::new("/no-such-normaliser-base")
}

fn n(text: &str) -> String {
    normalise(text, base())
}

#[test]
fn the_job_rule_numbers_ids_by_first_appearance_across_the_transcript() {
    assert_eq!(
        n("applying 9 requests as job 7\n\
           job 7 done: 9 requests, 9 done (jobs 8–16)\n\
           $ acq result 7\n\
           the failed children are in jobs 8, 12\n"),
        "applying 9 requests as job <j1>\n\
         job <j1> done: 9 requests, 9 done (jobs <j2>–<j3>)\n\
         $ acq result <j1>\n\
         the failed children are in jobs <j2>, <j4>\n"
    );
    assert_eq!(n("in jobs 7, 12\n"), "in jobs <j1>, <j2>\n");
    for command in ["status", "cancel", "priority"] {
        assert_eq!(
            n(&format!("$ acq {command} 31\n")),
            format!("$ acq {command} <j1>\n")
        );
    }
}

#[test]
fn the_pid_rule_names_a_process_id() {
    assert_eq!(
        n("daemon stopped (pid 58345)\n"),
        "daemon stopped (pid <pid>)\n"
    );
}

#[test]
fn the_progress_rule_keeps_each_jobs_last_stderr_line() {
    let text = "$ acq refresh --apply\n\
                --- stdout\n\
                applying 2 requests as job 2\n\
                job 2 done: 2 requests, 2 done (jobs 3–4)\n\
                --- stderr\n\
                job 2 (alice): 0/2 done, 2 waiting\n\
                job 2 (alice): 1/2 done, 1 running\n\
                job 2 (alice): 2/2 done\n";
    assert_eq!(
        n(text),
        "$ acq refresh --apply\n\
         --- stdout\n\
         applying 2 requests as job <j1>\n\
         job <j1> done: 2 requests, 2 done (jobs <j2>–<j3>)\n\
         --- stderr\n\
         job <j1> (alice): 2/2 done\n"
    );
    // stdout's `job` lines are the command's answer, never progress
    let answer = "--- stdout\njob 2: running\njob 2: done\n";
    assert_eq!(n(answer), "--- stdout\njob <j1>: running\njob <j1>: done\n");
}

#[test]
fn the_log_rule_names_the_world_in_a_log_path_under_the_scratch() {
    assert_eq!(
        n("daemon log: <scratch>/logs/f323bd59fa67/mock/daemon.log\n"),
        "daemon log: <scratch>/logs/<world>/mock/daemon.log\n"
    );
    // the scratch rule runs first: a test's own base becomes `<scratch>`
    let base = Path::new("/tmp/acq-test-base");
    assert_eq!(
        normalise(
            "daemon log: /tmp/acq-test-base/logs/9fc87288b973/mock/daemon.log\n",
            base
        ),
        "daemon log: <scratch>/logs/<world>/mock/daemon.log\n"
    );
    for text in [
        // not under the scratch: the owner's log directory is never a test's
        "daemon log: /Users/x/Library/Logs/acq/9fc87288b973/mock/daemon.log\n",
        // not twelve hex digits, or not a directory
        "<scratch>/logs/9fc87288b97/mock/daemon.log\n",
        "<scratch>/logs/9fc87288b973x/mock/daemon.log\n",
        "<scratch>/logs/9FC87288B973/mock/daemon.log\n",
        "<scratch>/logs/9fc87288b973\n",
        "<scratch>/store/9fc87288b973/mock\n",
    ] {
        assert_eq!(n(text), text);
    }
}

#[test]
fn the_new_rules_leave_counts_names_and_ids_that_are_no_job_alone() {
    for text in [
        "26 jobs: 0 running, 0 waiting\n",
        "1 job: 0 running\n",
        "1 of 9 child jobs failed, 8 done\n",
        "logged in as Alice#1234\n",
        "  (unnamed)  UniqueStash  2  <age> ago  uniq  u001\n",
        "id dump-item3\n",
        "a subjob 7 and a jobs7 and pid7\n",
        "# holds \"applying 9 requests as job 7\"\n",
    ] {
        assert_eq!(n(text), text);
    }
}

/// Every committed golden is already normal: a rule added later that
/// rewrites what another wrote, or matches words no transcript means by
/// it, fails here by the golden's name.
#[test]
fn normalising_a_committed_golden_changes_nothing() {
    let goldens = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens");
    let mut seen = 0;
    for stem in std::fs::read_dir(&goldens).expect("tests/goldens") {
        let stem = stem.expect("an entry").path();
        for file in std::fs::read_dir(&stem).expect("a golden directory") {
            let file = file.expect("an entry").path();
            if file.extension().is_some_and(|e| e == "txt") {
                let text = std::fs::read_to_string(&file).expect("a golden");
                crate::harness::same_text(
                    &text,
                    &n(&text),
                    &file.display().to_string(),
                    "a rule rewrites a normalised golden",
                );
                seen += 1;
            }
        }
    }
    assert!(
        seen >= 25,
        "only {seen} goldens under {}",
        goldens.display()
    );
}
