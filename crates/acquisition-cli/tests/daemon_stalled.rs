//! #103, the adapter's part (C122): `acq daemon status` against a daemon
//! that answers the handshake and then never its status. Its own binary:
//! the test sets the process environment (`session`), which the one
//! binary of modules must not (`tools/arch-check.sh`, the layout rule).
//! The bound itself is the client crate's (`contract.rs`, forced at
//! zero); here the expiry is forced by a peer that never answers.

mod harness;

use harness::command;
use serde_json::Value;

/// `acq daemon status` against the stalled peer exits non-zero
/// with the client's refusal on stderr, naming the daemon's pid and the
/// kill line. The bound is the client crate's (`contract.rs`, forced at
/// zero); here the expiry is forced by a peer that never answers, under
/// `ACQ_HELLO_WITHIN=1` on the command, and no wall-clock bound is
/// asserted. The peer replays the hello a real daemon of this world
/// gave, as the verbs crate's diagnosis pin does.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_daemon_that_answers_hello_and_not_its_status_is_refused_naming_its_pid() {
    use acquisition_store::world::World;
    use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};

    let s = harness::session("stalled");
    let daemon = harness::start_daemon_in_process(&s, &harness::acqd_built()).await;
    let world = World::observe().unwrap();
    let socket = world.socket_path().unwrap();
    let hello = {
        let mut line = serde_json::to_string(&acquisition_protocol::protocol::Bootstrap::Hello {
            version: acquisition_protocol::VERSION.to_string(),
            contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
            world: world.name(),
        })
        .unwrap();
        line.push('\n');
        line
    };
    let reply = {
        let stream = acquisition_client::endpoint::connect(&socket)
            .await
            .unwrap();
        let (read, mut write) = tokio::io::split(stream);
        write.write_all(hello.as_bytes()).await.unwrap();
        let mut reply = String::new();
        BufReader::new(read).read_line(&mut reply).await.unwrap();
        reply
    };
    let pid = serde_json::from_str::<Value>(&reply).unwrap()["pid"]
        .as_u64()
        .expect("the daemon's pid");
    drop(daemon);

    let mut listener = acquisition_client::endpoint::Listener::bind(&socket).unwrap();
    let peer = tokio::spawn(async move {
        loop {
            let stream = listener.accept().await.unwrap();
            let reply = reply.clone();
            tokio::spawn(async move {
                let (read, mut write) = tokio::io::split(stream);
                let mut lines = BufReader::new(read).lines();
                let _hello = lines.next_line().await.unwrap();
                write.write_all(reply.as_bytes()).await.unwrap();
                let _status = lines.next_line().await.unwrap();
                // Holds the connection, and never answers.
                std::future::pending::<()>().await;
                drop(write);
            });
        }
    });

    let base = s.base.clone();
    let out = tokio::task::spawn_blocking(move || {
        command(&base, &["daemon", "status"])
            .env("ACQ_HELLO_WITHIN", "1")
            .output()
            .expect("spawning acq")
    })
    .await
    .unwrap();
    peer.abort();
    let _ = peer.await;

    assert_eq!(out.status.code(), Some(1), "{out:?}");
    let shown = String::from_utf8_lossy(&out.stderr);
    assert!(
        shown.contains(&format!(
            "(pid {pid}) answered the handshake and then not its status within 1s"
        )) && shown.contains(&format!(
            "stop it by hand (`{} {pid}`) and run the command again",
            if cfg!(windows) {
                "taskkill /F /PID"
            } else {
                "kill"
            }
        )),
        "{shown}"
    );
    assert!(out.stdout.is_empty(), "{out:?}");
}
