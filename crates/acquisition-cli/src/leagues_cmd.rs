//! What the store holds beyond the league an answer took, as text (#157,
//! #22): the `leagues:` line `price status`, `shop render` and `tabs`
//! print beside the league they read, the `held:` line of `policy show`,
//! and `acq leagues --stored`, the stored league list with no daemon.
//! Each line is a function of the verbs' document it renders
//! ([`LeaguesHeld`], [`LeagueListShown`]), whose `--json` is that
//! document (C53); the ten the document lists are named and the rest
//! counted, with the flag that lists them.

use acquisition_verbs::leagues::{HeldLeague, PolicyState};
use acquisition_verbs::{LeagueListShown, LeaguesHeld};
use anyhow::Result;

use crate::Ctx;
use crate::plural;

/// The rest a bounded list left out, and the flag that lists them.
fn more(left_out: usize) -> String {
    if left_out == 0 {
        String::new()
    } else {
        format!(" … and {left_out} more: `--expand` lists them")
    }
}

/// What the store holds of one league: its tabs and characters, zeros
/// left out.
fn holding(tabs: i64, characters: i64) -> Vec<String> {
    let mut out = Vec::new();
    if tabs > 0 {
        out.push(plural(tabs as usize, "tab", "tabs"));
    }
    if characters > 0 {
        out.push(plural(characters as usize, "character", "characters"));
    }
    out
}

/// The policy's word on a league named or not, as its state allows.
fn coverage(policy: PolicyState, named: bool) -> &'static str {
    match (policy, named) {
        (PolicyState::None, _) => "no policy",
        (PolicyState::Unreadable, _) => "policy unreadable",
        (PolicyState::Set, true) => "named",
        (PolicyState::Set, false) => "not named",
    }
}

/// The `leagues:` line: the league the answer read, then every other
/// league the realm holds on disk with its counts and the policy's word.
pub(crate) fn held_line(h: &LeaguesHeld) -> String {
    let read = h.league.as_deref().unwrap_or("");
    let others: Vec<String> = h
        .leagues
        .iter()
        .filter(|l| l.league.as_deref() != Some(read))
        .map(|l| {
            let mut words = holding(l.tabs, l.characters);
            match &l.league {
                Some(league) => {
                    words.push(coverage(h.policy, l.named).to_string());
                    format!("{league} ({})", words.join(", "))
                }
                None => format!("no league ({})", words.join(", ")),
            }
        })
        .collect();
    if others.is_empty() && h.left_out == 0 {
        return format!("leagues: {read} (this); none other\n");
    }
    format!(
        "leagues: {read} (this); also held: {}{}\n",
        others.join(", "),
        more(h.left_out)
    )
}

/// One league of `policy show`'s `held:` line: its name and the policy's
/// word in its realm.
fn policy_held_league(h: &LeaguesHeld, l: &HeldLeague) -> String {
    let Some(league) = &l.league else {
        return "characters with no league".into();
    };
    let word = match h.policy {
        PolicyState::Set if !h.policy_names_realm => format!("no policy for {}", h.realm.as_str()),
        state => coverage(state, l.named).to_string(),
    };
    format!("{league} ({word})")
}

/// `policy show`'s `held:` line: each realm the store holds, its leagues
/// with the policy's word on each.
pub(crate) fn policy_held_line(held: &[LeaguesHeld]) -> String {
    if held.is_empty() {
        return "held: no tabs or characters on disk in any realm\n".into();
    }
    let realms: Vec<String> = held
        .iter()
        .map(|h| {
            let leagues: Vec<String> = h.leagues.iter().map(|l| policy_held_league(h, l)).collect();
            format!(
                "{} {}{}",
                h.realm.as_str(),
                leagues.join(", "),
                more(h.left_out)
            )
        })
        .collect();
    format!("held: {}\n", realms.join("; "))
}

/// `acq leagues --stored`'s text: the list's age, then each league, the
/// newest start first, with its realm, when it started, the policy's word
/// and what the store holds of it, "on disk" said once.
pub(crate) fn list_text(l: &LeagueListShown) -> String {
    if l.fetched_at.is_none() {
        return "no league list on record; `acq leagues` fetches it\n".into();
    }
    if l.leagues.is_empty() {
        return format!(
            "league list fetched {}: no leagues; `acq leagues` fetches it again\n",
            l.fetched_at_age
        );
    }
    let mut on_disk_said = false;
    let leagues: Vec<String> = l
        .leagues
        .iter()
        .map(|league| {
            let mut words = Vec::new();
            if let Some(realm) = &league.realm {
                words.push(realm.clone());
            }
            // which league started last (the answer-facts seat, MCP fault
            // 3 and CLI fault 6: nothing marked the new league)
            words.push(match league.started_at {
                Some(_) => format!("started {}", league.started_at_age),
                None => "no start given".into(),
            });
            words.push(coverage(l.policy, league.named).to_string());
            let held = holding(league.held.tabs, league.held.characters);
            if held.is_empty() {
                words.push("nothing on disk".into());
            } else {
                let mut held = held.join(", ");
                if !on_disk_said {
                    held.push_str(" on disk");
                    on_disk_said = true;
                }
                words.push(held);
            }
            format!("{} ({})", league.id, words.join(", "))
        })
        .collect();
    format!(
        "league list fetched {}: {}{}\n",
        l.fetched_at_age,
        leagues.join(", "),
        more(l.left_out)
    )
}

/// `acq leagues --stored`: the stored list read with no daemon.
pub fn stored(cx: &Ctx, expand: bool, json: bool) -> Result<()> {
    let shown = cx.account()?.league_list(expand)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&shown)?);
    } else {
        print!("{}", list_text(&shown));
    }
    Ok(())
}
