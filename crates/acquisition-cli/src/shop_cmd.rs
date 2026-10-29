//! The forum shop page's CLI surface: `acq shop render` reads the shared
//! store and the intent file (no daemon, no network), asks the plan
//! crate's pricing verbs for the render ([`pricing::shop_render`], over the
//! listing state, C69/C81, and the page set, C74) — the function the MCP's
//! `shop_render` calls (C46) — and writes it to stdout for the owner to
//! paste by hand — nothing here sends anything. The sync policy
//! is read for the C72 report only: coverage, staleness and positions
//! are named with their remedies, never enforced, and the `RefreshPlan`
//! the staleness line cites is compiled offline the way `acq refresh
//! --plan` compiles it, so the count it prints is that command's.
//!
//! Under C53: the default text opens with the one line that answers
//! "what would this post" (items, on how many pages, omitted, blocked,
//! nothing applies — each omission by its cell), then the C72 lines that
//! change what to do next (a container outside the policy, facts older
//! than the window, a position seen before the listing), lists the
//! blocked items when ten or fewer and counts them otherwise, prints
//! each page under its *n* of *N* label, and ends with the next action;
//! `--expand` adds the whole policy table with every row's rule and
//! count, and every item left off with its cell; `--page N` prints one
//! page alone, for the clipboard; `--json` is the [`ShopRender`], of
//! which every line here is a function (`tests/shop_json.rs` pins it
//! from the spawned binary).

use std::path::Path;

use acquisition_plan::pricing;
use acquisition_plan::shop::{Cell, LeftOut, ShopRender, Verdict};
use acquisition_protocol::realm::Realm;
use anyhow::{Context, Result};

use crate::Ctx;
use crate::store_cmd::{ago, clip, realm_prefix};

use crate::plural;

/// Groups of this many items or fewer are listed one per line; larger
/// groups are counted (C53; `--expand` lists every one): the store's one
/// bound, which the document's lists hold too.
const LIST_UP_TO: usize = acquisition_store::bound::LISTED;

/// `render`'s flags as typed.
#[derive(Debug)]
pub struct RenderArgs<'a> {
    pub size: usize,
    pub template: Option<&'a Path>,
    pub page: Option<usize>,
    pub expand: bool,
    pub json: bool,
}

/// `acq shop render`.
pub fn render_cmd(cx: &Ctx, realm: Realm, league: &str, args: &RenderArgs<'_>) -> Result<()> {
    let template = match args.template {
        Some(path) => Some(
            std::fs::read_to_string(path)
                .with_context(|| format!("reading the template {}", path.display()))?,
        ),
        None => None,
    };
    let now = acquisition_store::now();
    let opened = cx.account()?.open()?;
    let rendered = pricing::shop_render(
        &opened.facts,
        &opened.intent,
        cx.frontend.provider,
        realm,
        league,
        &pricing::ShopArgs {
            size: args.size,
            template: template.as_deref(),
            now,
            expand: args.expand,
        },
    )?;

    if let Some(n) = args.page {
        let page = pricing::shop_page(&rendered, n)?;
        if args.json {
            println!("{}", serde_json::to_string_pretty(page)?);
        } else {
            print!("{}", page.text);
        }
        return Ok(());
    }
    if args.json {
        println!("{}", serde_json::to_string_pretty(&rendered)?);
        return Ok(());
    }
    print!(
        "{}",
        render_text(&rendered, now, args.expand, &opened.account.entry.username)
    );
    Ok(())
}

/// The cell's words in a breakdown.
fn cell_word(cell: Cell) -> &'static str {
    match cell {
        Cell::StashItem => "stash items",
        Cell::CharacterItem => "character items",
        Cell::GameLists => "the game lists",
        Cell::GameSkips => "the game skips",
        Cell::HandSkip => "skipped by hand",
        Cell::NothingApplies => "nothing applies",
        Cell::Unresolved => "unresolved",
        Cell::LeagueUnknown => "league unknown",
        Cell::HandNoPrice => "listed with no price",
        Cell::Ratio => "a ratio",
        Cell::RetiredCurrency => "a retired currency",
        Cell::RealmUnlisted => "a realm the site does not list",
        Cell::Socketed => "socketed",
        Cell::NoPosition => "no position",
        Cell::Substash => "in a substash",
        Cell::TabUnlisted => "tab not listed",
        Cell::InvalidIndex => "a corrupt tab index",
        Cell::NoSlot => "no slot",
        Cell::UnruledKind => "a price kind without a rule",
        Cell::PageSize => "over the page size",
    }
}

/// The cells of one verdict with their counts, zeros left out.
fn breakdown(r: &ShopRender, verdict: Verdict) -> String {
    let parts: Vec<String> = r
        .policy
        .iter()
        .filter(|row| row.verdict == verdict && row.count > 0)
        .map(|row| format!("{} {}", row.count, cell_word(row.cell)))
        .collect();
    parts.join(", ")
}

/// The C72 lines: only those that change the next decision.
fn freshness_lines(r: &ShopRender, now: i64) -> String {
    let mut out = String::new();
    let f = &r.freshness;
    if r.counts.posted == 0 {
        return out;
    }
    match f.policy.as_str() {
        "not_set" => out.push_str(
            "policy: none set, so no refresh covers the page's facts; `acq policy set` declares coverage\n",
        ),
        "unreadable" => out.push_str(&format!(
            "policy: revision {} cannot be read by this build ({}); its coverage is unknown\n",
            f.policy_revision.unwrap_or(0),
            f.policy_problem.as_deref().unwrap_or("?")
        )),
        "league_not_covered" => out.push_str(&format!(
            "policy: revision {} does not cover {}{}; the page's facts refresh only if it does\n",
            f.policy_revision.unwrap_or(0),
            realm_prefix(r.listing.realm),
            r.listing.league
        )),
        _ => {
            // each count is the document's whole one, never the length of
            // the list it bounds (C53)
            if f.uncovered_total > 0 {
                let named: Vec<String> = f
                    .uncovered
                    .iter()
                    .take(LIST_UP_TO)
                    .map(ToString::to_string)
                    .collect();
                let list = if f.uncovered_total <= named.len() {
                    named.join(", ")
                } else {
                    format!("{} …", named.join(", "))
                };
                let past = if f.stale_uncovered > 0 {
                    format!(
                        " ({} of their items past the window too)",
                        f.stale_uncovered
                    )
                } else {
                    String::new()
                };
                out.push_str(&format!(
                    "coverage: {} on the page outside the sync policy (revision {}): {list}; add them with `acq policy set`{past}\n",
                    plural(f.uncovered_total, "container", "containers"),
                    f.policy_revision.unwrap_or(0)
                ));
            }
            if f.stale_total > 0 {
                let remedy = match (f.refresh_requests, &f.refresh_problem) {
                    (Some(n), _) => format!("`acq refresh --plan` would send {n} requests"),
                    (None, Some(why)) => {
                        format!("`acq refresh --plan` could not be compiled: {why}")
                    }
                    (None, None) => {
                        "`acq refresh --plan` shows what a refresh would fetch".to_string()
                    }
                };
                out.push_str(&format!(
                    "stale: {} on the page last seen past the policy's window of {}s (oldest {}); {remedy}\n",
                    plural(f.stale_total, "item", "items"),
                    f.window_seconds.unwrap_or(0),
                    ago(now, f.oldest_stale_seconds.map(|s| now.saturating_sub(s)))
                ));
            }
        }
    }
    if f.position_before_listing_total > 0 {
        out.push_str(&format!(
            "positions: {} on the page seen before the current stash listing, whose indices the links use; a tab reindexed between would move the link — a refresh re-reads them\n",
            plural(f.position_before_listing_total, "item", "items")
        ));
    }
    out
}

fn left_out_line(l: &LeftOut) -> String {
    format!(
        "  {:<18} {:<32} {:<40} {}\n",
        l.cell.as_str(),
        clip(&l.label, 32),
        l.location
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
        l.target
    )
}

fn render_text(r: &ShopRender, now: i64, expand: bool, account: &str) -> String {
    // every printed `acq price` command carries the render's scope
    let flags = crate::price_cmd::scope_flags(&r.listing, account);
    let write = crate::price_cmd::write_flags(account);
    let mut out = String::new();
    let c = &r.counts;
    let place = format!("{}{}", realm_prefix(r.listing.realm), r.listing.league);
    if c.items == 0 {
        out.push_str(&format!(
            "no items on record for {place}: nothing to render\n"
        ));
        out.push_str(&format!(
            "next: `acq price status {flags}` says what is on record\n"
        ));
        return out;
    }
    let omitted = breakdown(r, Verdict::Omit);
    let blocked = breakdown(r, Verdict::Block);
    let tail = format!(
        "{} omitted{}; {} blocked{}; {} nothing applies",
        c.omitted,
        if omitted.is_empty() {
            String::new()
        } else {
            format!(" ({omitted})")
        },
        c.blocked,
        if blocked.is_empty() {
            String::new()
        } else {
            format!(" ({blocked})")
        },
        c.off_page
    );
    if c.posted == 0 {
        out.push_str(&format!(
            "nothing to paste for {place}: {} — {tail}\n",
            plural(c.items, "item", "items")
        ));
    } else {
        out.push_str(&format!(
            "{} in {place}: {} on {}; {tail}\n",
            plural(c.items, "item", "items"),
            plural(c.posted, "item to post", "items to post"),
            plural(c.pages, "page", "pages")
        ));
    }
    out.push_str(&freshness_lines(r, now));
    let blocked: Vec<_> = r
        .left_out
        .iter()
        .filter(|l| l.verdict == Verdict::Block)
        .collect();
    if expand {
        out.push_str(&format!(
            "policy table (page size {}, template {} characters):\n",
            r.size,
            r.template.chars().count()
        ));
        for row in &r.policy {
            out.push_str(&format!(
                "  {:<9} {:<18} {:>5}  {}\n",
                row.verdict.as_str(),
                row.cell.as_str(),
                row.count,
                row.why
            ));
        }
        if !r.left_out.is_empty() {
            out.push_str("left off the page:\n");
            for l in &r.left_out {
                out.push_str(&left_out_line(l));
            }
        }
    } else if c.blocked > 0 && c.blocked <= LIST_UP_TO {
        // decided by the document's count, never by the sample's length:
        // the blocked come first in `left_out`, so ten or fewer are all
        // there
        out.push_str("blocked:\n");
        for l in &blocked {
            out.push_str(&left_out_line(l));
        }
    }
    for page in &r.pages {
        out.push_str(&format!(
            "page {} of {}: {} characters, {}\n",
            page.number,
            page.of,
            page.chars,
            plural(page.items, "item", "items")
        ));
        // each `Stash<n>` the links write, by the tab's own name (C146)
        for t in &page.tabs {
            out.push_str(&format!(
                "  {} is the tab {:?} ({}): {}\n",
                t.stash,
                t.name,
                t.tab,
                plural(t.items, "item", "items")
            ));
        }
        out.push_str(&page.text);
        if !page.text.ends_with('\n') {
            out.push('\n');
        }
        out.push('\n');
    }
    let next = if c.posted > 0 {
        format!(
            "paste page 1 of {} into the shop thread and preview it before posting; `acq shop render --page N` prints one page alone{}",
            c.pages,
            if !expand && c.blocked > LIST_UP_TO {
                "; `--expand` names each blocked item"
            } else {
                ""
            }
        )
    } else if c.blocked > 0 {
        format!(
            "{} is blocked; `--expand` shows the policy table and each item; `acq price set {write} item/<id> exact <amount> <tag>` prices one by hand",
            plural(c.blocked, "item", "items")
        )
    } else {
        format!(
            "`acq price set {write} item/<id> exact <amount> <tag>` prices one by hand; `acq price list {flags}` shows what the game already lists"
        )
    };
    out.push_str(&format!("next: {next}\n"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::real_scale_fixture;
    use acquisition_plan::listing::{ReportHeader, resolve};
    use acquisition_plan::price::PriceTarget;
    use acquisition_plan::shop::{
        DEFAULT_PAGE_SIZE, Freshness, ITEMS_TOKEN, LeftOut, Page, PageTab, PolicyRow, PolicySource,
        Posted, RenderOptions, ShopCounts, render,
    };
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::time::{Duration, Instant};

    /// A render with three posted items on two pages, one item of each
    /// omission and two blocked, and a policy that covers one of the two
    /// containers.
    fn rendered() -> ShopRender {
        let header: ReportHeader = serde_json::from_value(json!({
            "schema": 2, "account_uuid": "u", "realm": "pc", "league": "Standard",
            "taken_at": 2000, "note_parser_version": 1, "currency_table_version": 1,
            "stash_listing": { "response_id": 2, "fetched_at": 1000 }
        }))
        .unwrap();
        let mut by_cell: BTreeMap<Cell, usize> = Cell::ALL.iter().map(|c| (*c, 0)).collect();
        for (cell, n) in [
            (Cell::StashItem, 2),
            (Cell::CharacterItem, 1),
            (Cell::GameLists, 4),
            (Cell::HandSkip, 1),
            (Cell::Substash, 1),
            (Cell::Unresolved, 1),
            (Cell::NothingApplies, 3),
        ] {
            by_cell.insert(cell, n);
        }
        let policy = Cell::ALL
            .iter()
            .map(|c| PolicyRow {
                cell: *c,
                verdict: c.verdict(),
                count: by_cell[c],
                why: c.why().into(),
            })
            .collect();
        let item = |id: &str| PriceTarget::Item { id: id.into() };
        let tab = |id: &str| PriceTarget::Tab {
            realm: Realm::Pc,
            id: id.into(),
        };
        let posted = |id: &str, page: usize, loc: PriceTarget| Posted {
            target: item(id),
            label: "Chaos Orb".into(),
            location: loc,
            cell: Cell::StashItem,
            page,
            title: " ~price 5 chaos".into(),
            link: format!(
                "[linkItem realm=\"pc\" location=\"Stash1\" league=\"Standard\" x=\"0\" y=\"0\"] {id}"
            ),
        };
        ShopRender {
            schema: 1,
            listing: header,
            rendered_at: 5000,
            size: 200,
            template: "[items]".into(),
            policy,
            counts: ShopCounts {
                items: 13,
                posted: 3,
                omitted: 5,
                blocked: 2,
                off_page: 3,
                pages: 2,
                by_cell,
            },
            off_page_listed: false,
            left_out_total: 3,
            posted_total: 3,
            posted: vec![
                posted("a", 1, tab("c1")),
                posted("b", 1, tab("c1")),
                posted("c", 2, tab("t2")),
            ],
            left_out: vec![
                LeftOut {
                    target: item("s"),
                    label: "Map".into(),
                    location: Some(PriceTarget::Substash {
                        realm: Realm::Pc,
                        parent: "m1".into(),
                        id: "s1".into(),
                    }),
                    cell: Cell::Substash,
                    verdict: Verdict::Block,
                },
                LeftOut {
                    target: item("u"),
                    label: "Ring".into(),
                    location: Some(tab("t2")),
                    cell: Cell::Unresolved,
                    verdict: Verdict::Block,
                },
                LeftOut {
                    target: item("k"),
                    label: "Boots".into(),
                    location: Some(tab("c1")),
                    cell: Cell::HandSkip,
                    verdict: Verdict::Omit,
                },
            ],
            pages: vec![
                Page {
                    number: 1,
                    of: 2,
                    chars: 40,
                    items: 2,
                    tabs: vec![PageTab {
                        stash: "Stash1".into(),
                        tab: tab("c1"),
                        name: "Dump".into(),
                        items: 2,
                    }],
                    text: "page one body\n".into(),
                },
                Page {
                    number: 2,
                    of: 2,
                    chars: 20,
                    items: 1,
                    tabs: vec![PageTab {
                        stash: "Stash2".into(),
                        tab: tab("t2"),
                        name: "~price 1 chaos".into(),
                        items: 1,
                    }],
                    text: "page two body\n".into(),
                },
            ],
            freshness: Freshness {
                policy: "covers".into(),
                policy_revision: Some(4),
                policy_problem: None,
                window_seconds: Some(3600),
                refresh_requests: Some(7),
                refresh_problem: None,
                uncovered: vec![tab("t2")],
                uncovered_total: 1,
                stale: vec![item("c")],
                stale_total: 1,
                stale_uncovered: 1,
                oldest_stale_seconds: Some(3800),
                oldest_stale_seconds_age: None,
                position_before_listing: vec![item("c")],
                position_before_listing_total: 1,
            },
        }
    }

    /// C53 — the default text opens with the one line, names the C72
    /// lines with their remedies, lists the blocked items, labels each
    /// page n of N, and ends with the next action; `--expand` adds the
    /// policy table and every item left off.
    #[test]
    fn c53_shop_render_opens_with_one_line_and_labels_every_page() {
        let r = rendered();
        let text = render_text(&r, 5000, false, "a");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(
            lines[0],
            "13 items in Standard: 3 items to post on 2 pages; 5 omitted (4 the game lists, 1 skipped by hand); 2 blocked (1 unresolved, 1 in a substash); 3 nothing applies"
        );
        assert_eq!(
            lines[1],
            "coverage: 1 container on the page outside the sync policy (revision 4): tab/pc/t2; add them with `acq policy set` (1 of their items past the window too)"
        );
        assert_eq!(
            lines[2],
            "stale: 1 item on the page last seen past the policy's window of 3600s (oldest 63m ago); `acq refresh --plan` would send 7 requests"
        );
        assert!(
            lines[3]
                .starts_with("positions: 1 item on the page seen before the current stash listing"),
            "{}",
            lines[3]
        );
        assert_eq!(lines[4], "blocked:");
        assert!(
            lines[5].starts_with(
                "  substash           Map                              substash/pc/m1/s1"
            ),
            "{}",
            lines[5]
        );
        assert!(lines[5].ends_with("item/s"), "{}", lines[5]);
        assert!(
            lines[6].starts_with("  unresolved         Ring"),
            "{}",
            lines[6]
        );
        // C146 — each page names the tabs its `Stash<n>` links write
        assert_eq!(lines[7], "page 1 of 2: 40 characters, 2 items");
        assert_eq!(
            lines[8],
            "  Stash1 is the tab \"Dump\" (tab/pc/c1): 2 items"
        );
        assert_eq!(lines[9], "page one body");
        assert_eq!(lines[10], "");
        assert_eq!(lines[11], "page 2 of 2: 20 characters, 1 item");
        assert_eq!(
            lines[12],
            "  Stash2 is the tab \"~price 1 chaos\" (tab/pc/t2): 1 item"
        );
        assert_eq!(lines[13], "page two body");
        assert_eq!(lines[14], "");
        assert_eq!(
            lines[15],
            "next: paste page 1 of 2 into the shop thread and preview it before posting; `acq shop render --page N` prints one page alone"
        );
        assert_eq!(lines.len(), 16, "{text}");
        assert!(!text.contains("policy table"), "{text}");

        let text = render_text(&r, 5000, true, "a");
        assert!(
            text.contains("policy table (page size 200, template 7 characters):\n"),
            "{text}"
        );
        assert!(
            text.contains("  post      stash_item             2  a hand-priced stash item"),
            "{text}"
        );
        assert!(
            text.contains("  block     page_size              0  the entry alone"),
            "{text}"
        );
        assert!(text.contains("left off the page:\n"), "{text}");
        assert!(text.contains("  hand_skip          Boots"), "{text}");

        // Nothing posted: the line says so and the C72 lines are moot.
        let mut none = rendered();
        none.posted.clear();
        none.pages.clear();
        none.counts.posted = 0;
        none.counts.pages = 0;
        none.counts.blocked = 5;
        let text = render_text(&none, 5000, false, "a");
        assert!(
            text.starts_with("nothing to paste for Standard: 13 items — 5 omitted"),
            "{text}"
        );
        assert!(!text.contains("coverage:"), "{text}");
        assert!(
            text.ends_with(
                "`acq price set --account a item/<id> exact <amount> <tag>` prices one by hand\n"
            ),
            "{text}"
        );

        // A plan that could not compile is said, never swallowed.
        let mut broken = rendered();
        broken.freshness.refresh_requests = None;
        broken.freshness.refresh_problem = Some("no such league".into());
        let text = render_text(&broken, 5000, false, "a");
        assert!(
            text.contains("; `acq refresh --plan` could not be compiled: no such league\n"),
            "{text}"
        );

        // No policy: the one line names the remedy.
        let mut unset = rendered();
        unset.freshness.policy = "not_set".into();
        let text = render_text(&unset, 5000, false, "a");
        assert!(text.contains("policy: none set, so no refresh covers the page's facts; `acq policy set` declares coverage\n"), "{text}");
        assert!(!text.contains("coverage:"), "{text}");
        assert!(text.contains("positions: 1 item"), "{text}");

        let empty = ShopRender {
            counts: ShopCounts::default(),
            posted: vec![],
            left_out: vec![],
            pages: vec![],
            ..rendered()
        };
        let text = render_text(&empty, 5000, false, "a");
        assert_eq!(
            text,
            "no items on record for Standard: nothing to render\nnext: `acq price status --account a --realm pc --league Standard` says what is on record\n"
        );
    }

    /// The step-6 review's fourth finding: `--expand` searched the whole
    /// left-out list per line, quadratic over the 26k items it was read
    /// at. The audit view over the real-scale fixture tiled to 63k items
    /// (plan step 7, item 2, `PRICING-SLICE.md` at `b6197a82`), measured 2026-09-07 in
    /// a debug build: 45 ms as built; with the search put back, 3.9 s at
    /// half this scale and four-fold per doubling. The bound is the cliff
    /// between the two, not a budget — and the view still says
    /// everything: the table, a line per item left off, every page.
    #[test]
    fn c53_the_expanded_render_is_linear_over_the_owners_league_tiled() {
        let s = real_scale_fixture::tiled(32);
        let report = resolve(&s).unwrap();
        let r = render(
            &report,
            &RenderOptions {
                size: DEFAULT_PAGE_SIZE,
                template: ITEMS_TOKEN,
                now: s.taken_at,
                policy: PolicySource::NotSet,
                expand: true,
            },
        )
        .unwrap();
        assert_eq!(r.counts.items, 1977 * 32);
        assert_eq!(r.counts.posted, 31 * 32);
        assert_eq!(r.left_out.len(), (1977 - 31) * 32);

        let t = Instant::now();
        let text = render_text(&r, s.taken_at, true, "a");
        let took = t.elapsed();
        assert!(
            took < Duration::from_secs(2),
            "the expanded render took {took:?} over {} items",
            r.counts.items
        );
        assert!(
            text.contains("policy table (page size 50000, template 7 characters):\n"),
            "{}",
            &text[..200]
        );
        assert_eq!(
            text.matches("\n  ").count(),
            r.policy.len() + r.left_out.len() + r.pages.iter().map(|p| p.tabs.len()).sum::<usize>()
        );
        let left_off = text.split("left off the page:\n").nth(1).unwrap();
        assert_eq!(
            left_off.lines().take_while(|l| l.starts_with("  ")).count(),
            r.left_out.len()
        );
        for page in &r.pages {
            assert!(text.contains(&format!(
                "page {} of {}: {} characters, {} items\n",
                page.number, page.of, page.chars, page.items
            )));
        }
    }
}
