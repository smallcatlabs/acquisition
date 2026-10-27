//! Who is asking, in what world, and for which account: the values every
//! verb takes in place of process statics (C46; the design's "values, not
//! globals"). A frontend builds one [`Frontend`] per process or server,
//! resolves an [`Account`] from it per verb (C121), and opens the facts
//! and intent through the account, so a long-lived server holding several
//! accounts fits the same shape as a one-shot CLI.
//!
//! Every field is public: a test builds a `Frontend` or an `Account` by
//! hand over a fixture store with no environment; the environment-reading
//! constructors are for the binaries.
//!
//! C147, a door and a ledger read take the world they act on: the
//! environment is read once, when [`Frontend::human`] or
//! [`Frontend::agent`] builds the frontend, and every door, ledger read
//! and log path after that is given the frontend's [`Frontend::root`].
//! A frontend over a chosen world — a long-lived server holding more
//! than one — reads its facts, intent, ledger and daemon there, whatever
//! the environment names.

use std::path::PathBuf;

use acquisition_client::client::{Client, ConnectError, ConnectOptions, Observed};
use acquisition_plan::pricing::Expect;
use acquisition_plan::{LeagueNote, PolicyWrite, SyncPolicy, league_notes, put_sync_policy};
use acquisition_protocol::provider::GGG;
use acquisition_protocol::realm::Realm;
use acquisition_store::world::intended_root;
use acquisition_store::{
    AccountEntry, AnnotationRow, Annotations, Index, Provenance, Resolve, SYNC_POLICY_KEY,
    SYNC_POLICY_KIND, SYNC_POLICY_SCOPE, Store, account_path,
};
use serde::Serialize;
use serde_json::Value;

use crate::fault::Fault;

/// The connect policy a frontend's verbs use (C10, the doors; C13, the
/// MCP never spawns in real mode).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Door {
    /// A human at a terminal: a use verb spawns, and replaces a daemon of
    /// another contract, artifact or provider (C84).
    Human,
    /// An agent's server: never replaces a daemon, and spawns into an
    /// empty socket in mock mode only (C13).
    Agent,
}

/// Who is asking, and in what world (C46). Built once per process or
/// server, after the environment is final.
#[derive(Clone, Debug)]
pub struct Frontend {
    /// The provenance channel every intent write is stamped with (C65):
    /// `cli`, `mcp`, later `web`.
    pub via: &'static str,
    /// The connect policy (C10, C13).
    pub door: Door,
    /// The provider this process wants (C88), checked at construction.
    pub provider: &'static str,
    /// The world's root as named at construction (C83, C147): the store
    /// root above the provider directories, not yet canonicalised (a use
    /// door creates it). Every door, ledger read and log path of this
    /// frontend acts on it.
    pub root: PathBuf,
    /// `ACQ_JOURNAL` as this frontend's environment named it, read once
    /// with the rest (C147): which journal a daemon started from here
    /// writes, for a reader with no daemon to ask (the diagnosis).
    pub journal: JournalKnob,
}

/// What `ACQ_JOURNAL` says, in the daemon's reading of it (`rails.rs`):
/// unset (or not UTF-8), the world's default journal; empty or `0`, no
/// journal; anything else, that path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JournalKnob {
    Default,
    Off,
    At(PathBuf),
}

impl JournalKnob {
    /// This process's `ACQ_JOURNAL`.
    pub fn from_environment() -> JournalKnob {
        match std::env::var("ACQ_JOURNAL") {
            Ok(p) if p.trim().is_empty() || p.trim() == "0" => JournalKnob::Off,
            Ok(p) => JournalKnob::At(PathBuf::from(p)),
            Err(_) => JournalKnob::Default,
        }
    }
}

/// The index listing, every account with its store file (C12: the
/// store's non-secret index, no daemon).
#[derive(Clone, Debug, Serialize)]
pub struct Accounts {
    pub provider: String,
    pub accounts: Vec<AccountRow>,
}

/// One account the index knows.
#[derive(Clone, Debug, Serialize)]
pub struct AccountRow {
    pub username: String,
    pub last_login: i64,
    pub persisted: bool,
    pub uuid: Option<String>,
    /// The account's facts file, whether or not it exists yet.
    pub store: PathBuf,
}

impl Frontend {
    /// C88 (the provider knob checked once) and C10: a human's frontend,
    /// from this process's environment.
    pub fn human(via: &'static str) -> Result<Frontend, Fault> {
        Frontend::from_environment(via, Door::Human)
    }

    /// C88 and C13: an agent's frontend, from this process's environment.
    pub fn agent(via: &'static str) -> Result<Frontend, Fault> {
        Frontend::from_environment(via, Door::Agent)
    }

    fn from_environment(via: &'static str, door: Door) -> Result<Frontend, Fault> {
        acquisition_protocol::provider::check_environment().map_err(Fault::Environment)?;
        let provider = acquisition_protocol::provider::wanted();
        Ok(Frontend {
            via,
            door,
            provider,
            root: intended_root(),
            journal: JournalKnob::from_environment(),
        })
    }

    /// The provider's store directory in this frontend's world:
    /// `<root>/<provider>`, where the index, the facts and the intent
    /// live.
    pub fn dir(&self) -> PathBuf {
        self.root.join(self.provider)
    }

    fn index(&self) -> Result<Index, Fault> {
        Index::load(&self.dir()).map_err(Fault::Unopened)
    }

    /// C12: every account the index knows, each with its store path; an
    /// empty listing is an answer, never a fault.
    pub fn accounts(&self) -> Result<Accounts, Fault> {
        let index = self.index()?;
        Ok(Accounts {
            provider: self.provider.to_string(),
            accounts: index
                .entries()
                .iter()
                .map(|e| AccountRow {
                    username: e.username.clone(),
                    last_login: e.last_login,
                    persisted: e.persisted,
                    uuid: e.uuid.clone(),
                    store: account_path(&self.dir(), &e.username),
                })
                .collect(),
        })
    }

    /// C121 (an account that cannot be picked): the one account a
    /// selector names — an exact username, a name without its
    /// discriminator, or a uuid — or the sole one known. Refused as
    /// `not_logged_in` or `ambiguous_account`, each with the accounts.
    pub fn account(&self, selector: Option<&str>) -> Result<Account, Fault> {
        let index = self.index()?;
        let known =
            || -> Vec<String> { index.entries().iter().map(|e| e.username.clone()).collect() };
        match index.resolve(selector) {
            Ok(entry) => Ok(Account {
                frontend: self.clone(),
                entry: entry.clone(),
            }),
            Err(Resolve::Empty) => Err(Fault::NoAccounts {
                dir: self.dir(),
                provider: self.provider.to_string(),
            }),
            Err(Resolve::NotFound(selector)) => Err(Fault::UnknownAccount {
                selector,
                known: known(),
            }),
            Err(Resolve::Ambiguous(candidates)) => Err(Fault::AmbiguousAccount { candidates }),
        }
    }

    /// C10 and C13: the use door under this frontend's [`Door`]. A human
    /// door spawns and replaces; an agent door never replaces and spawns
    /// in mock mode only. Absence this door may not fill is
    /// [`Fault::NoDaemon`]; the caller reads [`Client::spawned`].
    pub async fn connect(&self) -> Result<Client, Fault> {
        let options = match self.door {
            Door::Human => ConnectOptions::interactive(true),
            Door::Agent => ConnectOptions::autonomous(self.provider != GGG),
        };
        match Client::connect_at(&self.root, options).await {
            Ok(client) => Ok(client),
            Err(ConnectError::Absent { because }) => Err(Fault::NoDaemon {
                because: Some(because),
                next: self.next(),
            }),
            Err(door) => Err(Fault::Door(door)),
        }
    }

    /// C10 (an observer never spawns or replaces): this world's daemon if
    /// it is this client's, `None` when nothing listens, and a fault for
    /// another daemon or an unresponsive peer.
    pub async fn observe(&self) -> Result<Option<Client>, Fault> {
        match Client::observe_at(&self.root)
            .await
            .map_err(Fault::Transport)?
        {
            Observed::Compatible(client) => Ok(Some(client)),
            Observed::Absent => Ok(None),
            Observed::Incompatible(found) => Err(Fault::AnotherDaemon(Box::new(found))),
            Observed::Unresponsive(found) => Err(Fault::Unresponsive(found)),
        }
    }

    /// C10: the observation door for a verb that acts on the daemon
    /// running (a cancel, a priority, a logout): absence is
    /// [`Fault::NoDaemon`].
    pub async fn attach(&self) -> Result<Client, Fault> {
        self.observe().await?.ok_or_else(|| Fault::NoDaemon {
            because: None,
            next: self.next(),
        })
    }

    /// C13 (a real-mode daemon is a human's act): the shell line that
    /// starts a daemon in this world, as printed — the provider when it
    /// is not the default, and always this frontend's store root (C147,
    /// the world it acts on), quoted for a shell, so it runs as printed
    /// beside this frontend whatever the shell's own `ACQ_STORE_DIR`
    /// names (the README, "Your agent"). The default root is named too:
    /// left out, the command would start a daemon in the world the
    /// shell's environment names.
    pub fn start_command(&self) -> String {
        let mut world = String::new();
        if self.provider != GGG {
            world.push_str(&format!(
                "{}={} ",
                acquisition_protocol::provider::KNOB,
                self.provider
            ));
        }
        world.push_str(&format!(
            "ACQ_STORE_DIR={} ",
            acquisition_search::answer::shell_quoted(&self.root.display().to_string())
        ));
        format!("{world}acq auth check")
    }

    /// What a no-daemon refusal's `next` says: a person's act, the
    /// command that does it.
    pub(crate) fn next(&self) -> String {
        format!(
            "a person starts one from a terminal: `{}` (the README, \"Your agent\")",
            self.start_command()
        )
    }
}

/// The one refusal sentence for a word that names no realm (C58, the
/// realm above league), for every frontend's argument reader.
pub fn realm(word: &str) -> Result<Realm, String> {
    Realm::parse(word).ok_or_else(|| {
        format!(
            "unknown realm {word:?} (one of {})",
            Realm::ALL.map(Realm::as_str).join(", ")
        )
    })
}

/// A resolved account (C121). Only [`Frontend::account`] makes one in
/// the binaries; a test builds it by hand.
#[derive(Clone, Debug)]
pub struct Account {
    pub frontend: Frontend,
    pub entry: AccountEntry,
}

impl Account {
    /// C12: the account's facts, read through the store's own API.
    pub fn facts(&self) -> Result<Store, Fault> {
        Store::open(&account_path(&self.frontend.dir(), &self.entry.username))
            .map_err(Fault::Unopened)
    }

    /// C35 (intent is irreplaceable, one file per account uuid): the
    /// account's intent file, or [`Fault::NoUuid`] for an entry that
    /// predates uuid-at-login.
    pub fn intent(&self) -> Result<Annotations, Fault> {
        let Some(uuid) = self.entry.uuid.as_deref() else {
            return Err(Fault::NoUuid {
                username: self.entry.username.clone(),
            });
        };
        Annotations::open_for(&self.frontend.dir(), uuid).map_err(|e| Fault::Unopened(e.into()))
    }

    /// C12: both, intent first (an entry without a uuid is refused before
    /// a facts file is opened).
    pub fn open(&self) -> Result<Opened, Fault> {
        let intent = self.intent()?;
        let facts = self.facts()?;
        Ok(Opened {
            account: self.clone(),
            facts,
            intent,
        })
    }

    /// C34 (intent through the annotation API alone): the stored sync
    /// policy row, or none. Intent alone: a facts file this build cannot
    /// read stops neither read nor write.
    pub fn policy(&self) -> Result<Option<AnnotationRow>, Fault> {
        self.intent()?
            .get(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)
            .map_err(|e| Fault::Other(e.into()))
    }

    /// C52 (no blind clobber) and C35: write the sync policy through the
    /// planner's validate-then-compare-and-swap, stamped with this
    /// frontend's channel (C65). `expect` is what naming no revision
    /// means, as pricing's verbs take it: [`Expect::Current`] reads the
    /// stored revision just before the write and refuses a stored row
    /// this build cannot read; [`Expect::Create`] writes only where none
    /// stands. `write` is what the value replaces (C36). The receipt
    /// carries a note for each league the value names that the account's
    /// fetched league list does not hold (C126); nothing refuses on it, and
    /// a facts file this build cannot read leaves the receipt without notes.
    pub fn set_policy(
        &self,
        value: &Value,
        expect: Expect,
        write: PolicyWrite,
    ) -> Result<PolicyWritten, Fault> {
        let mut intent = self.intent()?;
        let expected = match expect {
            Expect::Exactly(revision) => Some(revision),
            Expect::Create => None,
            Expect::Current => {
                match intent
                    .get(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)
                    .map_err(|e| Fault::Other(e.into()))?
                {
                    None => None,
                    Some(row) => match SyncPolicy::from_value(&row.value) {
                        Ok(_) => Some(row.revision),
                        Err(e) => {
                            return Err(Fault::UnreadablePolicy {
                                revision: row.revision,
                                detail: e.to_string(),
                            });
                        }
                    },
                }
            }
        };
        let row = put_sync_policy(
            &mut intent,
            value,
            expected,
            write,
            &Provenance::via(self.frontend.via),
        )?;
        let notes = match (
            SyncPolicy::from_value(value),
            self.facts().ok().map(|f| f.league_list()),
        ) {
            (Ok(written), Some(Ok(list))) => league_notes(written.leagues(), list.as_ref()),
            _ => Vec::new(),
        };
        Ok(PolicyWritten {
            row,
            league_notes: notes,
        })
    }
}

/// A sync-policy write as both doors report it (C36, C126): the stored
/// row, whole, and a note for each league the written value names that
/// the account's fetched league list does not hold.
#[derive(Clone, Debug, Serialize)]
pub struct PolicyWritten {
    #[serde(flatten)]
    pub row: AnnotationRow,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub league_notes: Vec<LeagueNote>,
}

/// An account's facts and intent, open together (C12): what the refresh
/// flow reads.
pub struct Opened {
    pub account: Account,
    pub facts: Store,
    pub intent: Annotations,
}
