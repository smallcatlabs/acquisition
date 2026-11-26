//! The seeded world (C122, the process tests share one harness): one
//! table describing a store shaped like an owner's, one writer that puts
//! it on disk through the store's own ingest (C34, one mutation path per
//! layer), and the counts the world decides, read by every door that
//! asserts them. The CLI's tests, the MCP's tests and the verbs crate's
//! `seeded-world` example include this file by `#[path]`; it is never
//! included from the client's harness, since the intent is written
//! through the plan crate's typed puts, so the world cannot hold a value
//! the planner refuses, and the client does not link the planner (C1).
//!
//! The world is for reads. The mock gives every user the same stash and
//! a login overwrites the uuid an account was seeded under, which orphans
//! the seeded intent (the annotation file is named by the uuid, C35): a
//! test that logs in or applies runs over the mock's own world.
//!
//! A variation is a method on the table that drops, adds or changes
//! rows and returns a table, so a test that means one account, or one
//! league, says so by name.
#![allow(dead_code)] // each consumer reads the slice it asserts

use std::path::{Path, PathBuf};

use acquisition_plan::price::Buyout;
use acquisition_plan::{PolicyWrite, put_sync_policy};
use acquisition_store::{
    Annotations, Endpoint, Index, Provenance, SAVED_SEARCH_SCOPE, SavedSearch, Store, User,
    account_path,
};
use serde_json::{Value, json};

/// The owner: two realms, three leagues, prices and a sync policy.
pub const ALICE: &str = "Alice#1234";
pub const ALICE_UUID: &str = "u-alice";
/// The second account, with a handful of items.
pub const BOB: &str = "Bob#9";
pub const BOB_UUID: &str = "u-bob";
/// The third account, which only logged in.
pub const CAROL: &str = "Carol#77";
pub const CAROL_UUID: &str = "u-carol";

pub const PC: &str = "pc";
pub const POE2: &str = "poe2";
pub const STANDARD: &str = "Standard";
pub const HARDCORE: &str = "Hardcore";
/// Listed by the account's league list and named by the policy, with
/// nothing on disk.
pub const SETTLERS: &str = "Settlers";

/// Alice's pc Standard tab listed and never fetched.
pub const NEVER_FETCHED_TAB: &str = "t3";
pub const NEVER_FETCHED_NAME: &str = "Never";

/// The hostile row: a pc Hardcore tab and an item note holding markup.
pub const HOSTILE_TAB: &str = "h1";
pub const HOSTILE_TAB_NAME: &str = r#"<script>alert("tab")</script> & <b>"Loot"</b>"#;
pub const HOSTILE_ITEM: &str = "i-hostile";
pub const HOSTILE_NOTE: &str = r#"<script>alert("note")</script> & "quoted" <i>"#;
/// A pc Hardcore tab fetched and then left out of the league's listing.
pub const GONE_TAB: &str = "t-gone";
pub const GONE_ITEM: &str = "i-gone";

/// The two prices on things the world holds: an item's exact price and a
/// folder's skip; a third row names an item the world does not hold.
pub const PRICED_ITEM: &str = "i-plain";
pub const PRICED_TAB: &str = "pc/f1";
pub const PRICED_NOWHERE: &str = "i-elsewhere";

/// The plain orbs' price by hand, as the world writes it on `i-plain`.
pub const PRICED_AMOUNT: &str = "3";
pub const PRICED_CURRENCY: &str = "chaos";

/// Alice's pc Standard tab `t1` and the two rings it holds, each with its
/// life summed over its lines: `r1`'s one line, `r2`'s implicit 20 and
/// explicit 75.
pub const RINGS_TAB_NAME: &str = "Rings";
pub const DOOM_LOOP: &str = "Doom Loop";
pub const DOOM_LOOP_LIFE: u64 = 95;
pub const RUNE_COIL: &str = "Rune Coil";
pub const RUNE_COIL_LIFE: u64 = 95;
/// Tab `t2`'s unique, its one life line and its sockets as (group,
/// colour): three linked, one apart.
pub const KAOM: &str = "Kaom's Heart";
pub const KAOM_LIFE: u64 = 500;
/// [`KAOM_LIFE`] as its line reads.
pub const KAOM_LIFE_LINE: &str = "+500 to maximum Life";
pub const KAOM_SOCKETS: [(u64, &str); 4] = [(0, "R"), (0, "R"), (0, "G"), (1, "B")];
/// Tab `t2`'s ring whose implicit lines cannot be read, so its life total
/// is undecided.
pub const HEX_BAND: &str = "Hex Band";
pub const HEX_BAND_ID: &str = "r3";

/// The world's one saved search (C167: the world's, in `user.db`).
pub const SAVED_ID: &str = "5eed5eed5eed";
pub const SAVED_NAME: &str = "Life rings";
pub const SAVED_TEXT: &str = "line(template:life arg1>=90)";

/// An instant to write the world at when a test then records facts of
/// its own at fixed instants after it (40, 50 …), so that its records are
/// the newer fetches.
pub const EARLY: i64 = 30;

/// Counts the world decides, read by both doors. Alice's pc Standard
/// items: the priced tab's two, the substash's one, the six search items
/// and the character's one.
pub const ALICE_PC_STANDARD_ITEMS: usize = 10;
/// The places holding them: the folder, the priced tab, the map tab and
/// its substash, the three search tabs (one never fetched) and the
/// character.
pub const ALICE_PC_STANDARD_CONTAINERS: usize = 8;
/// What the site search of explicit life summing to 70 or more
/// (`sum(line("# to maximum Life" source=explicit).arg1)>=70`) matches in
/// Alice's pc Standard: `r1` (95), `r2` (75 explicit) and `kaom` (500).
pub const EXPLICIT_LIFE_70: usize = 3;
/// Bob's items, all in one pc Standard tab.
pub const BOB_ITEMS: usize = 3;

/// One response as GGG would have answered it: recorded `age` seconds
/// before the writer's time.
#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub endpoint: Endpoint,
    pub body: Value,
    pub age: i64,
}

/// One account's rows: its facts in the order they are recorded, and its
/// intent.
#[derive(Clone, Debug)]
pub struct Account {
    pub name: String,
    pub uuid: String,
    pub responses: Vec<Response>,
    /// Buyout rows: scope, key and the value as the plan crate types it.
    pub prices: Vec<(String, String, Value)>,
    pub policy: Option<Value>,
}

/// The table: the accounts, and the world's saved searches by id.
#[derive(Clone, Debug)]
pub struct Seeded {
    pub accounts: Vec<Account>,
    pub saved: Vec<(String, Value)>,
}

fn stashes(realm: &str, league: &str, tabs: Value, age: i64) -> Response {
    Response {
        endpoint: Endpoint::Stashes {
            realm: realm.into(),
            league: league.into(),
        },
        body: json!({ "stashes": tabs }),
        age,
    }
}

fn stash(
    realm: &str,
    league: &str,
    id: &str,
    sub: Option<&str>,
    body: Value,
    age: i64,
) -> Response {
    Response {
        endpoint: Endpoint::Stash {
            realm: realm.into(),
            league: league.into(),
            id: id.into(),
            sub: sub.map(str::to_string),
        },
        body: json!({ "stash": body }),
        age,
    }
}

/// A currency stack, its note given.
pub fn orb(id: &str, note: Option<&str>) -> Value {
    let mut v = json!({ "id": id, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb",
                        "x": 0, "y": 0, "stackSize": 5 });
    if let Some(n) = note {
        v["note"] = json!(n);
    }
    v
}

/// An identified item of level 84, `more` merged over it.
pub fn gear(id: &str, name: &str, base: &str, rarity: &str, more: Value) -> Value {
    let mut v = json!({ "id": id, "name": name, "typeLine": base, "baseType": base, "rarity": rarity,
                        "frameTypeId": rarity, "identified": true, "ilvl": 84, "x": 0, "y": 0 });
    for (k, value) in more.as_object().into_iter().flatten() {
        v[k] = value.clone();
    }
    v
}

/// Kaom's sockets as GGG writes them: [`KAOM_SOCKETS`], each a socket.
fn kaom_sockets() -> Value {
    let attr = |colour: &str| match colour {
        "R" => "S",
        "G" => "D",
        _ => "I",
    };
    KAOM_SOCKETS
        .iter()
        .map(|(group, colour)| json!({ "group": group, "attr": attr(colour), "sColour": colour }))
        .collect()
}

/// A rare Iron Ring of level 84 carrying `mods`.
fn iron_ring(id: &str, mods: &[&str]) -> Value {
    gear(id, "", "Iron Ring", "Rare", json!({ "explicitMods": mods }))
}

impl Seeded {
    /// The owner's world.
    ///
    /// Alice holds pc Standard, pc Hardcore and poe2 Standard, lists
    /// Settlers too, and prices and syncs:
    ///
    /// | tab | name | holds |
    /// | --- | --- | --- |
    /// | `f1` | Sale (folder) | `c1` |
    /// | `c1` | ~price 3 chaos (public) | `i-noted` (noted `~price 5 chaos`), `i-plain` |
    /// | `m1` | Maps | substash `s1`: `i-map` |
    /// | `t1` | Rings | `r1` (life 95, fire res 65), `r2` (life 20 implicit and 75, fire res 40) |
    /// | `t2` | Dump | `kaom` (life 500, sockets R-R-G B), `r3` (implicit unread, life 40), `plain` (mana 20), `str` (strength 25) |
    /// | `t3` | Never | never fetched |
    /// | poe2 `p1` | Rings | `two` (spirit 30), `twin` (life 10, 20, 10) |
    /// | Hardcore `h1` | the hostile name | `i-hostile`, noted with markup |
    /// | Hardcore `t-gone` | Old | `i-gone`; fetched, then left out of the listing |
    ///
    /// The character Exile (pc Standard) wears `i-worn`. The prices: `i-plain`
    /// at 3 chaos, the folder `pc/f1` skipped, and `i-elsewhere`, held
    /// nowhere, skipped. The policy syncs pc Standard and Settlers.
    /// Bob holds one pc Standard tab of three items; Carol only logged in.
    pub fn owner() -> Seeded {
        Seeded {
            accounts: vec![alice(), bob(), carol()],
            saved: vec![(
                SAVED_ID.to_string(),
                json!({ "version": 2, "name": SAVED_NAME, "query": { "text": SAVED_TEXT },
                        "scope": { "realm": PC, "membership": "live" } }),
            )],
        }
    }

    /// The owner alone in her pc Standard, the league she plays: one
    /// account and one league, what the pricing and search tests read.
    pub fn home() -> Seeded {
        Seeded::owner().alone(ALICE).scopes(&[(PC, STANDARD)])
    }

    /// What this table holds beyond `written`, a table already on disk:
    /// the responses `written` lacks, of the accounts that have any, with
    /// no intent and no saved search. Written after `written`, it brings
    /// the store to this table's facts.
    pub fn beyond(&self, written: &Seeded) -> Seeded {
        let accounts = self
            .accounts
            .iter()
            .filter_map(|a| {
                let before = written.accounts.iter().find(|w| w.name == a.name);
                let responses: Vec<Response> = a
                    .responses
                    .iter()
                    .filter(|r| before.is_none_or(|w| !w.responses.contains(r)))
                    .cloned()
                    .collect();
                (!responses.is_empty()).then(|| Account {
                    responses,
                    prices: Vec::new(),
                    policy: None,
                    ..a.clone()
                })
            })
            .collect();
        Seeded {
            accounts,
            saved: Vec::new(),
        }
    }

    /// The accounts named, in the table's order; every other one dropped.
    pub fn accounts(mut self, names: &[&str]) -> Seeded {
        self.accounts.retain(|a| names.contains(&a.name.as_str()));
        self
    }

    /// The one account named, alone: what a test that means one account
    /// reads.
    pub fn alone(self, name: &str) -> Seeded {
        self.accounts(&[name])
    }

    /// Only the facts of the scopes named, each a realm and a league: a
    /// league listing and its tabs outside them dropped, and a realm's
    /// characters dropped where no scope names the realm. The league
    /// list, the profiles and the intent stay.
    pub fn scopes(mut self, kept: &[(&str, &str)]) -> Seeded {
        let keeps = |realm: &str, league: Option<&str>| {
            kept.iter()
                .any(|(r, l)| *r == realm && league.is_none_or(|league| *l == league))
        };
        for a in &mut self.accounts {
            a.responses.retain(|r| match &r.endpoint {
                Endpoint::Stashes { realm, league } | Endpoint::Stash { realm, league, .. } => {
                    keeps(realm, Some(league))
                }
                Endpoint::Characters { realm } | Endpoint::Character { realm, .. } => {
                    keeps(realm, None)
                }
                Endpoint::Leagues | Endpoint::Profile => true,
            });
        }
        self
    }

    /// The tabs named listed and never fetched: their fetches dropped, on
    /// every account.
    pub fn unfetched(mut self, tabs: &[&str]) -> Seeded {
        for a in &mut self.accounts {
            a.responses.retain(
                |r| !matches!(&r.endpoint, Endpoint::Stash { id, .. } if tabs.contains(&id.as_str())),
            );
        }
        self
    }

    /// No saved search.
    pub fn without_saved(mut self) -> Seeded {
        self.saved.clear();
        self
    }

    /// No account's league list: never fetched.
    pub fn without_league_list(mut self) -> Seeded {
        for a in &mut self.accounts {
            a.responses.retain(|r| r.endpoint != Endpoint::Leagues);
        }
        self
    }

    /// No intent on any account: no price, no policy.
    pub fn without_intent(mut self) -> Seeded {
        for a in &mut self.accounts {
            a.prices.clear();
            a.policy = None;
        }
        self
    }

    /// Alice's pc Standard tab `t3` fetched, holding what the category
    /// tests ask for: a pair of boots, a gem with a damage line, a captured
    /// Dune Hellion (yellow by the colour table), a captured Bleached
    /// Crustacean (unknown there) and a breachstone with no frame.
    pub fn with_categories(self) -> Seeded {
        let plain = |id: &str, base: &str, more: Value| {
            let mut v = json!({ "id": id, "name": "", "typeLine": base, "baseType": base,
                                "identified": true, "ilvl": 0, "x": 0, "y": 0 });
            for (k, value) in more.as_object().into_iter().flatten() {
                v[k] = value.clone();
            }
            v
        };
        let bestiary = "Right-click to add this to your bestiary.";
        self.fetched(
            ALICE,
            stash(
                PC,
                STANDARD,
                NEVER_FETCHED_TAB,
                None,
                json!({ "id": NEVER_FETCHED_TAB, "name": NEVER_FETCHED_NAME, "type": "PremiumStash", "items": [
                    plain("boots", "Iron Greaves", json!({ "frameTypeId": "Rare", "rarity": "Rare",
                        "requirements": [{ "name": "Level", "values": [["20", 0]], "displayMode": 0 }],
                        "explicitMods": ["+30 to maximum Life"] })),
                    plain("fire", "Fireball", json!({ "frameTypeId": "Gem",
                        "explicitMods": ["Deals 9 to 14 Fire Damage"] })),
                    plain("hellion", "Dune Hellion", json!({ "frameTypeId": "Rare", "descrText": bestiary })),
                    plain("crab", "Bleached Crustacean", json!({ "frameTypeId": "Rare", "descrText": bestiary })),
                    plain("stone", "Tul's Breachstone", json!({})),
                ] }),
                0,
            ),
        )
    }

    /// Alice's pc Standard tab `t3` fetched, holding `many`, a ring with
    /// eight Spirit lines: more lines than a row shows.
    pub fn with_many_lines(self) -> Seeded {
        let lines: Vec<String> = (1..=8).map(|n| format!("+{n} to Spirit")).collect();
        let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
        self.fetched(
            ALICE,
            stash(
                PC,
                STANDARD,
                NEVER_FETCHED_TAB,
                None,
                json!({ "id": NEVER_FETCHED_TAB, "name": NEVER_FETCHED_NAME, "type": "PremiumStash",
                        "items": [iron_ring("many", &lines)] }),
                0,
            ),
        )
    }

    /// A response added to the account's facts, recorded last.
    fn fetched(mut self, account: &str, response: Response) -> Seeded {
        if let Some(a) = self.accounts.iter_mut().find(|a| a.name == account) {
            a.responses.push(response);
        }
        self
    }

    /// Write the world under `dir`, what `ACQ_STORE_DIR` names, at `at`:
    /// the mock provider's directory, which it returns, holds the index,
    /// each account's facts and intent, and the world's saved searches.
    /// Every write goes through the store's door, the intent through the
    /// plan crate's typed puts.
    pub fn write(&self, dir: &Path, at: i64) -> PathBuf {
        let mock = dir.join("mock");
        std::fs::create_dir_all(&mock).unwrap();
        let mut index = Index::load(&mock).unwrap();
        let via = Provenance::via("test");
        for a in &self.accounts {
            index
                .record_login(&a.name, &a.uuid, false, at, &[])
                .unwrap();
            let mut store = Store::open(&account_path(&mock, &a.name)).unwrap();
            store
                .record(
                    &Endpoint::Profile,
                    &json!({}),
                    200,
                    &json!({ "uuid": a.uuid, "name": a.name }),
                    at,
                )
                .unwrap();
            for r in &a.responses {
                store
                    .record(&r.endpoint, &params(&r.endpoint), 200, &r.body, at - r.age)
                    .unwrap();
            }
            drop(store);
            if a.prices.is_empty() && a.policy.is_none() {
                continue;
            }
            let mut intent = Annotations::open_for(&mock, &a.uuid).unwrap();
            for (scope, key, value) in &a.prices {
                intent.put::<Buyout>(scope, key, value, None, &via).unwrap();
            }
            if let Some(policy) = &a.policy {
                put_sync_policy(&mut intent, policy, None, PolicyWrite::Replace, &via).unwrap();
            }
        }
        if !self.saved.is_empty() {
            let mut user = User::open_for(&mock).unwrap();
            for (id, value) in &self.saved {
                user.put::<SavedSearch>(SAVED_SEARCH_SCOPE, id, value, None, &via)
                    .unwrap();
            }
        }
        mock
    }
}

/// The request's parameters as the daemon would have sent them.
fn params(endpoint: &Endpoint) -> Value {
    match endpoint {
        Endpoint::Leagues | Endpoint::Profile => json!({}),
        Endpoint::Characters { realm } => json!({ "realm": realm }),
        Endpoint::Character { realm, name } => json!({ "realm": realm, "name": name }),
        Endpoint::Stashes { realm, league } | Endpoint::Stash { realm, league, .. } => {
            json!({ "realm": realm, "league": league })
        }
    }
}

fn alice() -> Account {
    let tab = |id: &str, name: &str, index: u64| json!({ "id": id, "name": name, "type": "PremiumStash", "index": index });
    let life = |n: u64| format!("+{n} to maximum Life");
    let responses = vec![
        Response {
            endpoint: Endpoint::Leagues,
            body: json!({ "leagues": [
                { "id": STANDARD, "realm": PC }, { "id": HARDCORE, "realm": PC },
                { "id": SETTLERS, "realm": PC }, { "id": STANDARD, "realm": POE2 } ] }),
            age: 0,
        },
        stashes(
            PC,
            STANDARD,
            json!([
                { "id": "f1", "name": "Sale", "type": "Folder", "index": 0,
                  "children": [ { "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash", "index": 1, "metadata": { "public": true } } ] },
                { "id": "m1", "name": "Maps", "type": "MapStash", "index": 2 },
                tab("t1", RINGS_TAB_NAME, 3),
                tab("t2", "Dump", 4),
                tab(NEVER_FETCHED_TAB, NEVER_FETCHED_NAME, 5),
            ]),
            0,
        ),
        stash(
            PC,
            STANDARD,
            "c1",
            None,
            json!({ "id": "c1", "name": "~price 3 chaos", "type": "PremiumStash",
                    "items": [ orb("i-noted", Some("~price 5 chaos")), orb(PRICED_ITEM, None) ] }),
            0,
        ),
        stash(
            PC,
            STANDARD,
            "m1",
            None,
            json!({ "id": "m1", "name": "Maps", "type": "MapStash", "items": [],
                    "children": [ { "id": "s1", "name": "1", "type": "MapStash", "metadata": { "items": 1 } } ] }),
            0,
        ),
        stash(
            PC,
            STANDARD,
            "m1",
            Some("s1"),
            json!({ "id": "s1", "name": "1", "type": "MapStash", "items": [ orb("i-map", None) ] }),
            0,
        ),
        stash(
            PC,
            STANDARD,
            "t1",
            None,
            json!({ "id": "t1", "name": RINGS_TAB_NAME, "type": "PremiumStash", "items": [
                gear("r1", DOOM_LOOP, "Two-Stone Ring", "Rare", json!({ "explicitMods":
                    [life(DOOM_LOOP_LIFE), "+65% to Fire Resistance".to_string()] })),
                gear("r2", RUNE_COIL, "Two-Stone Ring", "Rare", json!({ "implicitMods": [life(20)],
                    "explicitMods": [life(RUNE_COIL_LIFE - 20), "+40% to Fire Resistance".to_string()] })),
            ] }),
            20,
        ),
        stash(
            PC,
            STANDARD,
            "t2",
            None,
            json!({ "id": "t2", "name": "Dump", "type": "PremiumStash", "items": [
                gear("kaom", KAOM, "Glorious Plate", "Unique", json!({ "explicitMods": [KAOM_LIFE_LINE],
                    "sockets": kaom_sockets() })),
                gear(HEX_BAND_ID, HEX_BAND, "Two-Stone Ring", "Rare", json!({ "implicitMods": "unreadable", "explicitMods": ["+40 to maximum Life"] })),
                gear("plain", "Dull Turn", "Iron Ring", "Rare", json!({ "explicitMods": ["+20 to maximum Mana"] })),
                gear("str", "Grim Knot", "Iron Ring", "Rare", json!({ "explicitMods": ["+25 to Strength"] })),
            ] }),
            10,
        ),
        Response {
            endpoint: Endpoint::Characters { realm: PC.into() },
            body: json!({ "characters": [ { "id": "ch1", "name": "Exile", "league": STANDARD } ] }),
            age: 0,
        },
        Response {
            endpoint: Endpoint::Character {
                realm: PC.into(),
                name: "Exile".into(),
            },
            body: json!({ "character": { "id": "ch1", "name": "Exile", "league": STANDARD,
                                         "equipment": [ orb("i-worn", Some("~b/o 2 divine")) ], "inventory": [] } }),
            age: 0,
        },
        stashes(POE2, STANDARD, json!([tab("p1", "Rings", 0)]), 0),
        stash(
            POE2,
            STANDARD,
            "p1",
            None,
            json!({ "id": "p1", "name": "Rings", "type": "PremiumStash", "items": [
                iron_ring("two", &["+30 to Spirit"]),
                iron_ring("twin", &["+10 to maximum Life", "+20 to maximum Life", "+10 to maximum Life"]),
            ] }),
            0,
        ),
        stash(
            PC,
            HARDCORE,
            GONE_TAB,
            None,
            json!({ "id": GONE_TAB, "name": "Old", "type": "PremiumStash", "items": [
                { "id": GONE_ITEM, "name": "", "typeLine": "Chaos Orb", "baseType": "Chaos Orb",
                  "x": 0, "y": 0, "stackSize": 1 } ] }),
            60,
        ),
        stashes(
            PC,
            HARDCORE,
            json!([tab(HOSTILE_TAB, HOSTILE_TAB_NAME, 0)]),
            0,
        ),
        stash(
            PC,
            HARDCORE,
            HOSTILE_TAB,
            None,
            json!({ "id": HOSTILE_TAB, "name": HOSTILE_TAB_NAME, "type": "PremiumStash",
                    "items": [ orb(HOSTILE_ITEM, Some(HOSTILE_NOTE)) ] }),
            0,
        ),
    ];
    let price = |scope: &str, key: &str, value: Value| (scope.to_string(), key.to_string(), value);
    Account {
        name: ALICE.into(),
        uuid: ALICE_UUID.into(),
        responses,
        prices: vec![
            price(
                "item",
                PRICED_ITEM,
                json!({ "version": 1, "type": "exact", "amount": PRICED_AMOUNT, "currency": PRICED_CURRENCY }),
            ),
            price("tab", PRICED_TAB, json!({ "version": 1, "type": "skip" })),
            price(
                "item",
                PRICED_NOWHERE,
                json!({ "version": 1, "type": "skip" }),
            ),
        ],
        policy: Some(json!({ "version": 3, "realms": { PC: { "leagues": {
            STANDARD: { "tabs": "all", "characters": "all", "max_age_seconds": 86400 },
            SETTLERS: { "tabs": "all", "max_age_seconds": 86400 } } } } })),
    }
}

fn bob() -> Account {
    Account {
        name: BOB.into(),
        uuid: BOB_UUID.into(),
        responses: vec![
            stashes(
                PC,
                STANDARD,
                json!([{ "id": "b1", "name": "Bobs", "type": "PremiumStash", "index": 0 }]),
                10,
            ),
            stash(
                PC,
                STANDARD,
                "b1",
                None,
                json!({ "id": "b1", "name": "Bobs", "type": "PremiumStash", "items": [
                    gear("bob1", "Lone Band", "Iron Ring", "Rare", json!({})),
                    gear("bob2", "Cold Ring", "Iron Ring", "Rare", json!({ "explicitMods": ["+30% to Cold Resistance"] })),
                    orb("bob3", None),
                ] }),
                0,
            ),
        ],
        prices: Vec::new(),
        policy: None,
    }
}

fn carol() -> Account {
    Account {
        name: CAROL.into(),
        uuid: CAROL_UUID.into(),
        responses: Vec::new(),
        prices: Vec::new(),
        policy: None,
    }
}
