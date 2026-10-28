//! `acq jobs --watch` under lag, through the binary (C85's subscriber
//! sequence, as the reference consumer). The watcher is stopped with
//! SIGSTOP while the daemon runs more jobs than its event channel holds,
//! so its subscription lags; on SIGCONT it must drop that subscription,
//! subscribe afresh and print the snapshot again — and after that
//! snapshot no event about an older job may appear, because a fresh
//! subscription carries only what happens after it was opened. The
//! original implementation kept the lagged subscription and would print
//! its queued events after the snapshot; this test fails on it.
//!
//! What is not observable from outside: that each printed event line is
//! a re-read of the job rather than the hint. The watch has no other
//! path (`main.rs`, `watch_jobs`), and a read that fails restarts the
//! sequence; that stays a reading of the code, not a pin.
//!
//! Unix only: Windows has no stop signal. The watch's code is one on
//! every platform, and the lag rule itself is pinned through the client
//! on both (`acquisition-client/tests/contract.rs`, C85).
#![cfg(unix)]

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

mod harness;

use harness::{Proc, command, start_daemon, wait_all_terminal};

use acquisition_client::client::{Client, Observed};
use acquisition_protocol::protocol::{Request, Response};
use serde_json::{Value, json};

fn signal(child: &Child, sig: &str) {
    let status = Command::new("kill")
        .args([sig, &child.id().to_string()])
        .status()
        .expect("kill");
    assert!(status.success(), "kill {sig}");
}

/// One thing the watcher printed under `--json`: a snapshot (the `jobs`
/// document, pretty, over lines; C149) or an event (one object per line).
#[derive(Debug)]
enum Item {
    Snapshot(Value),
    Event(Value),
}

/// A snapshot's rows.
fn rows(snapshot: &Value) -> &Vec<Value> {
    snapshot["rows"]
        .as_array()
        .unwrap_or_else(|| panic!("a snapshot lists its rows: {snapshot}"))
}

struct Output {
    lines: mpsc::Receiver<String>,
}

impl Output {
    fn line(&self, deadline: Instant) -> String {
        let left = deadline.saturating_duration_since(Instant::now());
        self.lines
            .recv_timeout(left)
            .expect("the watcher printed nothing more before the deadline")
    }

    fn item(&self, deadline: Instant) -> Item {
        let first = self.line(deadline);
        // a pretty document opens on a line of its own; an event is one line
        if first == "{" {
            let mut text = first;
            loop {
                let line = self.line(deadline);
                text.push('\n');
                text.push_str(&line);
                if line == "}" {
                    break;
                }
            }
            return Item::Snapshot(serde_json::from_str(&text).expect("a snapshot document"));
        }
        Item::Event(serde_json::from_str(&first).unwrap_or_else(|e| panic!("{e}: {first}")))
    }
}

fn watcher(base: &Path, args: &[&str]) -> (Proc, Output) {
    let mut child = command(base, &[&["jobs", "--watch", "--json"], args].concat())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawning the watcher");
    let stdout = child.stdout.take().expect("piped");
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    (
        Proc(child, PathBuf::from(env!("CARGO_BIN_EXE_acq"))),
        Output { lines: rx },
    )
}

async fn client() -> Client {
    match Client::observe().await.expect("observe") {
        Observed::Compatible(client) => client,
        other => panic!(
            "{}",
            match other {
                Observed::Absent => "no daemon".to_string(),
                Observed::Incompatible(found) => found.to_string(),
                Observed::Unresponsive(found) => found.to_string(),
                Observed::Compatible(_) => unreachable!(),
            }
        ),
    }
}

async fn submit_sleep(client: &mut Client) -> u64 {
    match client
        .request(&Request::Submit {
            kind: "sleep".into(),
            params: json!({ "seconds": 0 }),
            priority: 0,
            submitted_by: "watch-test".into(),
            account: None,
        })
        .await
        .expect("submit")
    {
        Response::Submitted { id } => id,
        other => panic!("{other:?}"),
    }
}

#[test]
fn c85_the_watch_subscribes_and_reads_again_after_it_lagged_with_no_leftover_events() {
    // The in-process client below reads the world from the environment:
    // the harness's session applies the one isolation list to this
    // process, and this file is its own test binary (C122).
    let session = harness::session("watch");
    let base = session.base.clone();
    let (_daemon, _) = start_daemon(&base);
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // every job in every snapshot (`--all`): the proof below reads all of them
    let (watch, out) = watcher(&base, &["--all"]);
    let deadline = Instant::now() + Duration::from_secs(120);
    match out.item(deadline) {
        Item::Snapshot(jobs) => assert!(rows(&jobs).is_empty(), "{jobs}"),
        other => panic!("the watch opens with a snapshot: {other:?}"),
    }

    // Stopped, the watcher reads nothing; the daemon's writes to its
    // subscription block once the socket is full, and the event channel
    // overruns the receiver behind them.
    signal(&watch.0, "-STOP");
    const JOBS: usize = 800;
    runtime.block_on(async {
        let mut client = client().await;
        for _ in 0..JOBS {
            submit_sleep(&mut client).await;
        }
        wait_all_terminal(&mut client, JOBS).await;
    });
    signal(&watch.0, "-CONT");

    // Some events the socket held, then the snapshot after `resync_required`.
    let deadline = Instant::now() + Duration::from_secs(120);
    let mut events_before = 0usize;
    let snapshot = loop {
        match out.item(deadline) {
            Item::Event(_) => events_before += 1,
            Item::Snapshot(jobs) => break jobs,
        }
    };
    assert_eq!(
        snapshot["total"], JOBS,
        "{events_before} events, then {snapshot}"
    );
    let snapshot = rows(&snapshot).clone();
    assert_eq!(
        snapshot.len(),
        JOBS,
        "{events_before} events, then a partial snapshot"
    );
    assert!(
        snapshot.iter().all(|j| j["state"] == "done"),
        "{snapshot:?}"
    );
    let newest_old = snapshot
        .iter()
        .map(|j| j["id"].as_u64().unwrap())
        .max()
        .unwrap();

    // A watch with no `--all` prints the same document bounded (C53, the
    // brief's second review, finding 11): the 800 counted done beside the
    // ten most recent, the rest counted as left out.
    let (default_watch, default_out) = watcher(&base, &[]);
    match default_out.item(Instant::now() + Duration::from_secs(120)) {
        Item::Snapshot(listed) => {
            assert_eq!(listed["total"], JOBS, "{listed}");
            assert_eq!(listed["by_state"]["done"], JOBS, "{listed}");
            assert_eq!(rows(&listed).len(), 10, "{listed}");
            assert_eq!(listed["finished_left_out"], JOBS - 10, "{listed}");
            assert_eq!(
                rows(&listed).last().map(|j| j["id"].as_u64().unwrap()),
                Some(newest_old),
                "{listed}"
            );
        }
        other => panic!("the watch opens with a snapshot: {other:?}"),
    }
    drop(default_watch);

    // After that snapshot, the only events are about jobs that came later:
    // the lagged subscription and everything it held are gone.
    let next = runtime.block_on(async {
        let mut client = client().await;
        submit_sleep(&mut client).await
    });
    assert!(next > newest_old);
    let deadline = Instant::now() + Duration::from_secs(120);
    match out.item(deadline) {
        Item::Event(job) => {
            let id = job["id"].as_u64().unwrap();
            assert_eq!(
                id, next,
                "an event about job {id} (before {next}) after the fresh snapshot: a leftover of the lagged subscription: {job}"
            );
            assert!(job["state"].is_string(), "{job}");
        }
        Item::Snapshot(jobs) => panic!("a second resync: {jobs}"),
    }
}
