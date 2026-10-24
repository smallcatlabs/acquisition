//! The new-league rehearsal (C122, C126 — a league the list lacks is a
//! note): launch morning over two leagues, as the owner will type it,
//! through the real pair of binaries over the mock, held as one golden
//! transcript whose facts come from the journey's one table
//! (`acquisition-client/tests/harness/journey.rs`). Standard is synced
//! first, unrecorded, so the store is a user's; then, recorded: the
//! league list, the policy shown and written back with the realm whole
//! (Standard kept, the new league added), plans and applies over the new
//! league until the plan says nothing to do in it, its tabs and
//! Standard's (every item where it was, nothing moved), the reads that
//! take a league, the bare plan, and the negative case: a league the
//! list does not hold, noted on the receipt and the plan, and the
//! apply's failure naming it on the mock's 404 (the mock's convention,
//! never GGG's answer, which is unrecorded and never provoked). Then the
//! journal, its GETs counted per route and league, the league read from
//! each send's path since the route hides it, and every verb the
//! transcript ran is one the README's tour shows.

use crate::harness;
use crate::readme_tour;

use harness::journey::*;
use harness::{Transcript, acq, golden};

/// One recorded command: its words, how many of them are the verb path
/// (the tour check's), its facts and the exit it must end with.
struct Step {
    args: Vec<String>,
    verbs: usize,
    facts: Vec<String>,
    exit: i32,
}

fn step(args: &[&str], verbs: usize, facts: &[String]) -> Step {
    Step {
        args: args.iter().map(|a| a.to_string()).collect(),
        verbs,
        facts: facts.to_vec(),
        exit: 0,
    }
}

#[test]
fn a_new_league_starts_empty_beside_standard_and_a_typo_is_named() {
    let scratch = harness::scratch("newleague");
    let base = scratch.0.clone();
    let (knob, journal) = harness::journal_on(&base);
    let (mut daemon, pid) = harness::start_daemon_with(&base, &[(knob, journal.as_str())]);
    harness::login_as_mock(&base, &[]);

    // Standard synced first, as the owner's store will be: the journey's
    // policy, planned and applied until the plan names nothing.
    let out = acq(&base, &["policy", "set", POLICY]);
    assert!(out.status.success(), "{out:?}");
    for _ in PLAN_REQUESTS {
        let out = acq(&base, &["refresh", "--apply"]);
        assert!(out.status.success(), "{out:?}");
    }

    let league = ["--league", LEAGUE];
    let typo = ["--league", TYPO];
    let mut steps = vec![
        step(
            &["leagues"],
            1,
            &[format!("{LEAGUES_LISTED} leagues"), format!("{LEAGUE}  ")],
        ),
        step(&["policy"], 1, &[format!("revision {POLICY_REVISION}")]),
        step(
            &["policy", "set", NEW_LEAGUE_POLICY],
            2,
            &[format!("revision {NEW_LEAGUE_POLICY_REVISION}")],
        ),
    ];
    let [first, second] = NEW_LEAGUE_PLAN_REQUESTS;
    steps.extend([
        step(
            &["refresh", "--plan", league[0], league[1]],
            1,
            &[
                format!("refresh {LEAGUE} on mock"),
                format!("{first} request:"),
                format!("characters: none in {LEAGUE}"),
                format!("next: acq refresh --apply --league {LEAGUE}"),
            ],
        ),
        step(
            &["refresh", "--apply", league[0], league[1]],
            1,
            &[
                format!("{0} request, {0} done", first),
                format!("next: `acq refresh --plan --league {LEAGUE}` — {second} requests more"),
            ],
        ),
        step(
            &["refresh", "--plan", league[0], league[1]],
            1,
            &[
                format!("fetch {NEW_LEAGUE_TABS_FETCHED} tabs"),
                format!("{second} requests:"),
                // a wait, not `ready`: the golden masks its seconds,
                // which move with the run's speed
                format!("stash-request-limit  {second} requests — ~"),
            ],
        ),
        step(
            &["refresh", "--apply", league[0], league[1]],
            1,
            &[
                format!("{0} requests, {0} done", second),
                "nothing added, changed, moved, or removed".into(),
                "left: nothing".into(),
            ],
        ),
        step(
            &["refresh", "--plan", league[0], league[1]],
            1,
            &[format!("nothing to do in {LEAGUE}:")],
        ),
        step(
            &["tabs", league[0], league[1]],
            1,
            &[format!(
                "{NEW_LEAGUE_TABS_LISTED} tabs in {LEAGUE}: {NEW_LEAGUE_TABS_FETCHED} fetched, 1 folder (never fetched); 0 items"
            )],
        ),
        step(
            &["tabs"],
            1,
            &[format!(
                "{TABS_LISTED} tabs in Standard: {TABS_FETCHED} fetched, 1 folder (never fetched); {STASH_ITEMS} items"
            )],
        ),
        step(
            &["search", "--count", "league"],
            1,
            &["count   league · 1 value".into(), "  Standard".into()],
        ),
        step(
            &["price", "status", league[0], league[1]],
            2,
            &[
                format!("no items on record for {LEAGUE}"),
                format!("next: `acq refresh --plan --league {LEAGUE}`"),
            ],
        ),
        step(
            &["store", "characters", league[0], league[1]],
            2,
            &[format!("no characters known in {LEAGUE} (")],
        ),
        step(&["refresh", "--plan"], 1, &["nothing to do in Standard:".into()]),
        step(
            &["policy", "set", TYPO_POLICY],
            2,
            &[format!("note: {TYPO} is not among the leagues fetched")],
        ),
        step(
            &["refresh", "--plan", typo[0], typo[1]],
            1,
            &[
                format!("note: {TYPO} is not among the leagues fetched"),
                "1 request:".into(),
            ],
        ),
        // the apply over the typo fails naming it, on the mock's 404
        Step {
            exit: 1,
            ..step(
                &["refresh", "--apply", typo[0], typo[1]],
                1,
                &[format!(
                    "stashes {TYPO}  GET /stash/{TYPO} returned 404 Not Found"
                )],
            )
        },
    ]);

    let mut t = Transcript::new(&base);
    let mut said = String::new();
    for s in &steps {
        let args: Vec<&str> = s.args.iter().map(String::as_str).collect();
        let facts: Vec<&str> = s.facts.iter().map(String::as_str).collect();
        let out = t.run(&args, &facts);
        assert_eq!(out.status.code(), Some(s.exit), "acq {args:?}: {out:?}");
        said.push_str(&harness::text(&out));
    }
    // The league's items are Standard's alone: no fetch moved one.
    assert!(
        !said.contains(" moved at "),
        "an item moved between leagues:\n{said}"
    );
    // The new league is on the list when the policy names it: no note.
    let receipt = &said[said.find("revision 2").expect("the second receipt")..];
    assert!(
        !receipt[..receipt.find('\n').unwrap_or(receipt.len())].contains("note"),
        "{receipt}"
    );

    let stop = step(&["daemon", "stop"], 2, &["daemon stopped (pid".into()]);
    let args: Vec<&str> = stop.args.iter().map(String::as_str).collect();
    let facts: Vec<&str> = stop.facts.iter().map(String::as_str).collect();
    assert!(t.run(&args, &facts).status.success());
    daemon.wait_exit(std::time::Duration::from_secs(120));
    golden(file!(), "launch_morning", &t);

    harness::assert_journal_holds(&base, pid, &NEW_LEAGUE_GETS, TOKEN_POSTS);

    // Every verb path the rehearsal ran is one the README's tour shows.
    let toured: Vec<Vec<String>> = readme_tour::tour_lines()
        .iter()
        .filter(|line| line.starts_with("acq "))
        .flat_map(|line| readme_tour::parse(line))
        .map(|(path, _)| path)
        .collect();
    for path in steps.iter().chain([&stop]).map(|s| &s.args[..s.verbs]) {
        assert!(
            toured.iter().any(|p| p == path),
            "`acq {}` is in the rehearsal and in no tour line of README.md",
            path.join(" ")
        );
    }
}
