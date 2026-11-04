//! What the store holds beyond the league an answer took (round 5 of the
//! answer-facts group, #157, #22): two documents, made once here (C46)
//! and carried by every surface that takes one league.
//!
//! - [`LeaguesHeld`]: every league of a realm the store holds tabs or
//!   characters of, its counts, and whether the sync policy names it;
//!   carried under `leagues_held` by `price status`, `shop render` and
//!   `tabs` beside the league they read ([`Held`]), and by `policy show`
//!   once per realm the store holds.
//! - [`LeagueListShown`]: the stored league list (C126: the latest
//!   successful leagues response), each league beside whether the policy
//!   names it and what the store holds of it; `acq leagues --stored`, the
//!   MCP's `leagues` and the web's league control read it, no daemon.
//!
//! The store's reads are neutral (C39): the policy's word is added here.
//! Both lists are bounded by the one bound (C53), counted whole beside
//! the rows they keep, `expand` listing all (C149).

use acquisition_plan::{StoredPolicy, SyncPolicy, stored_policy};
use acquisition_protocol::realm::Realm;
use acquisition_store::bound::bounded;
use acquisition_store::{AnnotationRow, Annotations, LeagueHeld, Store};
use serde::Serialize;

/// The sync policy's state as these documents read it: none set, a row
/// this build cannot read, or a policy at its revision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyState {
    None,
    Unreadable,
    Set,
}

/// The stored policy, read once for a document: its state, its revision
/// where a row stands, and the policy where it parses.
pub struct PolicyRead {
    pub state: PolicyState,
    pub revision: Option<i64>,
    pub policy: Option<SyncPolicy>,
}

impl PolicyRead {
    /// The policy as the planner's one reader gives it
    /// ([`acquisition_plan::stored_policy`]).
    pub fn of_stored(stored: StoredPolicy) -> PolicyRead {
        match stored {
            StoredPolicy::NotSet => PolicyRead {
                state: PolicyState::None,
                revision: None,
                policy: None,
            },
            StoredPolicy::Unreadable { revision, .. } => PolicyRead {
                state: PolicyState::Unreadable,
                revision: Some(revision),
                policy: None,
            },
            StoredPolicy::Set { revision, policy } => PolicyRead {
                state: PolicyState::Set,
                revision: Some(revision),
                policy: Some(policy),
            },
        }
    }

    /// The policy a stored row holds, or none.
    pub fn of(row: Option<&AnnotationRow>) -> PolicyRead {
        PolicyRead::of_stored(StoredPolicy::of_row(row))
    }

    /// The account's stored policy, for a verb that opens no intent file
    /// of its own (`tabs`, `leagues --stored`): an intent file that cannot
    /// be opened is a policy that cannot be read.
    pub fn of_account(account: &crate::Account) -> PolicyRead {
        match account.intent().map(|intent| PolicyRead::read(&intent)) {
            Ok(Ok(read)) => read,
            _ => PolicyRead {
                state: PolicyState::Unreadable,
                revision: None,
                policy: None,
            },
        }
    }

    /// The account's stored policy, read from its intent file through the
    /// planner's one reader: a row this build cannot read, malformed or
    /// newer, is `unreadable` with its revision (C151), never a failure of
    /// the answer that carries it.
    pub fn read(intent: &Annotations) -> anyhow::Result<PolicyRead> {
        Ok(PolicyRead::of_stored(stored_policy(intent)?))
    }

    /// Whether the policy names `league` in `realm`, whatever its entry
    /// covers (tabs, characters or both); false with no policy or one
    /// this build cannot read.
    fn names(&self, realm: Realm, league: &str) -> bool {
        self.policy
            .as_ref()
            .is_some_and(|p| p.league(realm, league).is_some())
    }

    /// Whether the policy names `realm` at all.
    fn names_realm(&self, realm: Realm) -> bool {
        self.policy
            .as_ref()
            .is_some_and(|p| p.realms.contains_key(&realm))
    }
}

/// What the store holds of a realm's leagues, beside the league an answer
/// read (C53: the leagues listed ten, the read one first in one of the
/// ten, the count whole).
#[derive(Clone, Debug, Serialize)]
pub struct LeaguesHeld {
    pub realm: Realm,
    /// The league the answer read; absent where the answer read none
    /// (`policy show`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,
    /// The sync policy's state: `none`, `unreadable` or `set`.
    pub policy: PolicyState,
    /// The policy row's revision, where one stands.
    pub policy_revision: Option<i64>,
    /// Whether the policy names this realm at all: false where it is set
    /// and names other realms alone.
    pub policy_names_realm: bool,
    /// Every league of the realm with live tabs or characters on disk,
    /// the realm's characters with no league counted as one.
    pub leagues_total: usize,
    /// The leagues listed: ten, the read one first where the store holds
    /// it, the rest in name order and the league-less characters last;
    /// every one under `expand`.
    pub leagues: Vec<HeldLeague>,
    /// The leagues not in `leagues`.
    pub left_out: usize,
    /// What lists them all: the request's `expand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

/// One league of a realm as the store holds it.
#[derive(Clone, Debug, Serialize)]
pub struct HeldLeague {
    /// The league; null for the realm's characters the listing gave no
    /// league.
    pub league: Option<String>,
    /// Its live tabs on disk.
    pub tabs: i64,
    /// Its live characters on disk.
    pub characters: i64,
    /// Whether the sync policy names the league in this realm, whatever
    /// its entry covers; false with no policy or an unreadable one, and
    /// for the league-less characters, which no policy entry names.
    pub named: bool,
}

impl LeaguesHeld {
    /// The realm's leagues as `store` holds them, `league` (the one the
    /// answer read) first, each beside the policy's word, cut by the one
    /// bound (C53) unless `expand`.
    pub fn read(
        store: &Store,
        policy: &PolicyRead,
        realm: Realm,
        league: Option<&str>,
        expand: bool,
    ) -> anyhow::Result<LeaguesHeld> {
        Ok(LeaguesHeld::of(
            store.leagues_held(realm.as_str())?,
            policy,
            realm,
            league,
            expand,
        ))
    }

    /// [`LeaguesHeld::read`] over rows already read.
    pub fn of(
        rows: Vec<LeagueHeld>,
        policy: &PolicyRead,
        realm: Realm,
        league: Option<&str>,
        expand: bool,
    ) -> LeaguesHeld {
        let mut rows: Vec<HeldLeague> = rows
            .into_iter()
            .map(|row| HeldLeague {
                named: row
                    .league
                    .as_deref()
                    .is_some_and(|l| policy.names(realm, l)),
                league: row.league,
                tabs: row.tabs,
                characters: row.characters,
            })
            .collect();
        if let Some(at) = rows
            .iter()
            .position(|r| r.league.as_deref() == league && league.is_some())
        {
            let read = rows.remove(at);
            rows.insert(0, read);
        }
        let leagues_total = rows.len();
        let (leagues, left_out) = bounded(rows, None, expand);
        LeaguesHeld {
            realm,
            league: league.map(str::to_string),
            policy: policy.state,
            policy_revision: policy.revision,
            policy_names_realm: policy.names_realm(realm),
            leagues_total,
            leagues,
            left_out,
            left_out_needs: (left_out > 0).then_some("expand"),
        }
    }

    /// One per realm the store holds tabs or characters of, the realms the
    /// policy does not name included (`policy show`).
    pub fn every_realm(
        store: &Store,
        policy: &PolicyRead,
        expand: bool,
    ) -> anyhow::Result<Vec<LeaguesHeld>> {
        let mut out = Vec::new();
        for realm in Realm::ALL {
            let rows = store.leagues_held(realm.as_str())?;
            if !rows.is_empty() {
                out.push(LeaguesHeld::of(rows, policy, realm, None, expand));
            }
        }
        Ok(out)
    }
}

/// An answer that takes one league, with what the store holds beyond it:
/// the answer's own keys and `leagues_held` beside them (C53, additive).
#[derive(Clone, Debug, Serialize)]
pub struct Held<T> {
    #[serde(flatten)]
    pub answer: T,
    pub leagues_held: LeaguesHeld,
}

impl<T> Held<T> {
    /// `answer`, an answer about `league` of `realm`, beside what `store`
    /// holds of the realm's leagues, the policy read from `intent`.
    pub fn read(
        answer: T,
        store: &Store,
        intent: &Annotations,
        realm: Realm,
        league: &str,
        expand: bool,
    ) -> anyhow::Result<Held<T>> {
        Held::with(
            answer,
            store,
            &PolicyRead::read(intent)?,
            realm,
            league,
            expand,
        )
    }

    /// [`Held::with`] over the holdings already read: the ones a
    /// document read in its own snapshot (`tabs`, the review of round
    /// C's fix round, finding 3), never read again.
    pub fn of(
        answer: T,
        rows: Vec<acquisition_store::LeagueHeld>,
        policy: &PolicyRead,
        realm: Realm,
        league: &str,
        expand: bool,
    ) -> Held<T> {
        Held {
            answer,
            leagues_held: LeaguesHeld::of(rows, policy, realm, Some(league), expand),
        }
    }

    /// [`Held::read`] with the policy already read.
    pub fn with(
        answer: T,
        store: &Store,
        policy: &PolicyRead,
        realm: Realm,
        league: &str,
        expand: bool,
    ) -> anyhow::Result<Held<T>> {
        Ok(Held {
            answer,
            leagues_held: LeaguesHeld::read(store, policy, realm, Some(league), expand)?,
        })
    }
}

/// The stored league list (C126: the latest successful leagues response,
/// never an accumulated table), each league beside the policy's word and
/// what the store holds of it, read with no daemon.
#[derive(Clone, Debug, Serialize)]
pub struct LeagueListShown {
    /// When the list was fetched; null when it never was.
    pub fetched_at: Option<i64>,
    /// C53: `fetched_at` in words at the answer's time, `never` when no
    /// list was fetched.
    pub fetched_at_age: String,
    /// The sync policy's state: `none`, `unreadable` or `set`.
    pub policy: PolicyState,
    /// The policy row's revision, where one stands.
    pub policy_revision: Option<i64>,
    /// Every league the list holds.
    pub leagues_total: usize,
    /// The leagues listed: ten, the newest start first, those with no
    /// start after them in the list's order; every one under `expand`
    /// (the answer-facts seat, MCP fault 3: nothing marked the new
    /// league).
    pub leagues: Vec<ListedLeague>,
    /// The leagues not in `leagues`.
    pub left_out: usize,
    /// What lists them all: the request's `expand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

/// One league of the stored list.
#[derive(Clone, Debug, Serialize)]
pub struct ListedLeague {
    pub id: String,
    /// The realm the list's entry names; null where it names none.
    pub realm: Option<String>,
    /// When the league started: the entry's `startAt` as GGG gives it,
    /// null where it gives none (C163, a time the document carries).
    pub started_at: Option<i64>,
    /// C53: `started_at` in words at the answer's time, `never` where
    /// the entry gives no start.
    pub started_at_age: String,
    /// Whether the sync policy names the league in the entry's realm, or
    /// in any realm where the entry names none; false with no policy or
    /// an unreadable one.
    pub named: bool,
    /// What the store holds of it: in the entry's realm, or summed over
    /// every realm where the entry names none.
    pub held: Holding,
}

/// Live tabs and characters on disk.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Holding {
    pub tabs: i64,
    pub characters: i64,
}

impl LeagueListShown {
    /// The stored list read from `store`, each entry beside `policy`'s
    /// word and the store's holdings, said at `now`, cut by the one bound
    /// (C53) unless `expand`.
    pub fn read(
        store: &Store,
        policy: &PolicyRead,
        expand: bool,
        now: i64,
    ) -> anyhow::Result<LeagueListShown> {
        let Some(list) = store.league_list()? else {
            return Ok(LeagueListShown {
                fetched_at: None,
                fetched_at_age: acquisition_store::age::ago(now, None),
                policy: policy.state,
                policy_revision: policy.revision,
                leagues_total: 0,
                leagues: Vec::new(),
                left_out: 0,
                left_out_needs: None,
            });
        };
        let mut held: Vec<(Realm, Vec<LeagueHeld>)> = Vec::new();
        for realm in Realm::ALL {
            held.push((realm, store.leagues_held(realm.as_str())?));
        }
        let holding = |realm: Option<Realm>, id: &str| {
            held.iter()
                .filter(|(r, _)| realm.is_none_or(|realm| realm == *r))
                .flat_map(|(_, rows)| rows)
                .filter(|row| row.league.as_deref() == Some(id))
                .fold(Holding::default(), |sum, row| Holding {
                    tabs: sum.tabs + row.tabs,
                    characters: sum.characters + row.characters,
                })
        };
        let rows: Vec<ListedLeague> = list
            .entries
            .iter()
            .map(|entry| {
                let realm = entry.realm.as_deref().and_then(Realm::parse);
                let named = match (&entry.realm, realm) {
                    (None, _) => Realm::ALL.into_iter().any(|r| policy.names(r, &entry.id)),
                    (Some(_), Some(realm)) => policy.names(realm, &entry.id),
                    // a realm this build does not know: no policy names it
                    (Some(_), None) => false,
                };
                ListedLeague {
                    id: entry.id.clone(),
                    realm: entry.realm.clone(),
                    started_at: entry.started_at,
                    started_at_age: acquisition_store::age::ago(now, entry.started_at),
                    named,
                    held: match (&entry.realm, realm) {
                        (Some(_), None) => Holding::default(),
                        _ => holding(realm, &entry.id),
                    },
                }
            })
            .collect();
        // the newest start first, a stable sort: those with no start
        // after, in the list's order
        let mut rows = rows;
        rows.sort_by_key(|l| std::cmp::Reverse(l.started_at.unwrap_or(i64::MIN)));
        let leagues_total = rows.len();
        let (leagues, left_out) = bounded(rows, None, expand);
        Ok(LeagueListShown {
            fetched_at: Some(list.fetched_at),
            fetched_at_age: acquisition_store::age::ago(now, Some(list.fetched_at)),
            policy: policy.state,
            policy_revision: policy.revision,
            leagues_total,
            leagues,
            left_out,
            left_out_needs: (left_out > 0).then_some("expand"),
        })
    }
}
