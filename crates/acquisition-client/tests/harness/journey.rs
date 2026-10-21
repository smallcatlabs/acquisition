//! The journey's one table of facts (C122): what a first session against
//! the mock must see, read by both doors, the CLI's golden transcript
//! (`acquisition-cli/tests/journey.rs`) and the MCP's twin
//! (`acquisition-mcp/tests/journey.rs`), each adapter including this file
//! by `#[path]`. The CLI builds its `# holds` facts from these values and
//! keeps the golden for the wording; the MCP compares its tools' fields
//! to the same numbers. A change to the mock's items (C123, the mock's
//! real-shaped items) is one edit here and one regeneration, read as a
//! diff. Standard library only: the two adapters link different crates.
#![allow(dead_code)] // each door reads the values it asserts

/// The sync policy the journey writes: every pc Standard tab and
/// character, an hour's freshness.
pub const POLICY: &str = r#"{"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}"#;

/// The policy's revision once written: the account's first.
pub const POLICY_REVISION: u64 = 1;

/// The mock account's name, as the login takes it.
pub const ACCOUNT: &str = "ExileTester";

/// The requests each plan names until the plan is empty: the two
/// listings, then the seven listed tabs and two characters, then the
/// seven substashes the map and unique tabs revealed. The fourth plan
/// names none.
pub const PLAN_REQUESTS: [u64; 3] = [2, 9, 7];

/// The tabs `acq tabs` lists in Standard, those fetched (all but the
/// folder) and the items stored in them: the mock's 31 and the six gems
/// socketed in the armour, each a row of its own.
pub const TABS_LISTED: u64 = 15;
pub const TABS_FETCHED: u64 = 14;
pub const STASH_ITEMS: u64 = 37;

/// The rare ring the journey prices: its GGG id and name.
pub const RING: &str = "51e6c1055b1d94f8c53921012d459ec5d2f4e353f674f4b03192c79375c4c682";
pub const RING_NAME: &str = "Empyrean Knot";

/// The first query, by class and rarity: the ring alone.
pub const RING_QUERY: &str = "class=Rings rarity=rare";
pub const RING_MATCHES: u64 = 1;

/// The second query, by a life line's value, sorted by the pseudo total:
/// the ring and Doryani's Prototype, whose total lives are these.
pub const LIFE_QUERY: &str = r#""+# to maximum Life">=70"#;
pub const LIFE_SORT: &str = "pseudo.total_life";
pub const LIFE_MATCHES: u64 = 2;
pub const ARMOUR_NAME: &str = "Doryani's Prototype";
pub const ARMOUR_TOTAL_LIFE: u64 = 89;
/// The ring's total life: 22 + 78 + 16 × 0.5 (`totals-v6.toml`).
pub const RING_TOTAL_LIFE: u64 = 108;

/// The third query, the stackable currency, counted by base with its
/// stacks summed: the Mirror Shard stack of seven.
pub const CURRENCY_QUERY: &str = r#"class="Stackable Currency""#;
pub const CURRENCY_MATCHES: u64 = 1;
pub const SHARD: &str = "Mirror Shard";
pub const SHARD_STACK: u64 = 7;

/// The price the journey sets on the ring and clears again: the words
/// `acq price set` takes, and the price as it prints.
pub const PRICE: [&str; 3] = ["exact", "5", "chaos"];
pub const PRICE_SHOWN: &str = "5 chaos";
/// The price's revision once written: the item's first.
pub const PRICE_REVISION: u64 = 1;

/// What the shop posts with the ring priced: one item on one page.
pub const POSTED_ITEMS: u64 = 1;
pub const POSTED_PAGES: u64 = 1;

/// The GETs the journal holds per route, without its account: the
/// login's profile, then what the plans named, one stash listing, one
/// character listing, the two characters and the fourteen tabs.
pub const GETS: [(&str, usize); 5] = [
    ("profile", 1),
    ("stash-list", 1),
    ("character-list", 1),
    ("character", 2),
    ("stash", 14),
];

/// The token endpoint's POSTs: the login's code exchange alone.
pub const TOKEN_POSTS: usize = 1;
