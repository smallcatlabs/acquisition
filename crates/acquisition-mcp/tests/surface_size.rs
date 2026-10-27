//! The first turn's cost, held (#83): an agent pays for the
//! `initialize` instructions and every tool of `tools/list` before its
//! first call, so each text is held to a length and the surface cannot
//! grow back. A tool's description says what it answers, what it costs
//! and where the rest is — `describe` by name, read once — and a
//! parameter's doc says what the argument is. A count's gloss stands in
//! the `describe` entry the description names in one line (C146,
//! amended 2026-10-27).

use crate::harness;

use harness::Mcp;
use serde_json::Value;

/// A tool's description, in bytes.
const DESCRIPTION: usize = 600;
/// A parameter's description, in bytes.
const PARAMETER: usize = 300;
/// The instructions, in bytes.
const INSTRUCTIONS: usize = 1_000;

fn surface() -> (String, Vec<Value>) {
    let scratch = harness::scratch("mcp-surface-size");
    let mut mcp = Mcp::start(&scratch.0, &[]);
    let tools = mcp.list_tools();
    let instructions = mcp.instructions.clone();
    let _ = mcp.child.kill();
    (instructions, tools)
}

#[test]
fn every_text_of_the_first_turn_is_held_to_its_bound() {
    let (instructions, tools) = surface();
    let mut over = Vec::new();
    if instructions.len() > INSTRUCTIONS {
        over.push(format!(
            "the instructions: {} bytes, bound {INSTRUCTIONS}",
            instructions.len()
        ));
    }
    for tool in &tools {
        let name = tool["name"].as_str().unwrap_or("?");
        let said = tool["description"].as_str().unwrap_or_default().len();
        if said > DESCRIPTION {
            over.push(format!(
                "`{name}`: description {said} bytes, bound {DESCRIPTION}"
            ));
        }
        let properties = tool["inputSchema"]["properties"].as_object();
        for (argument, schema) in properties.into_iter().flatten() {
            let said = schema["description"].as_str().unwrap_or_default().len();
            if said > PARAMETER {
                over.push(format!(
                    "`{name}`, `{argument}`: description {said} bytes, bound {PARAMETER}"
                ));
            }
        }
    }
    assert!(over.is_empty(), "past the bound: {over:#?}");
}

/// The release seat counted `account` undescribed on seven tools: every
/// tool that takes it says what it is, in one sentence shared by all.
#[test]
fn account_is_one_sentence_on_every_tool_that_takes_it() {
    let (_, tools) = surface();
    let said: Vec<(&str, &str)> = tools
        .iter()
        .filter_map(|tool| {
            let account = tool["inputSchema"]["properties"].get("account")?;
            Some((
                tool["name"].as_str().unwrap_or("?"),
                account["description"].as_str().unwrap_or_default(),
            ))
        })
        .collect();
    assert!(said.len() >= 15, "{said:#?}");
    let first = said[0].1;
    assert!(!first.is_empty(), "{said:#?}");
    assert!(said.iter().all(|(_, s)| *s == first), "{said:#?}");
}

/// C146, amended 2026-10-27: on the MCP a count's gloss stands in the
/// `describe` entry the tool's description names. The six tools whose
/// descriptions carried their counts' glosses until then each name the
/// section, and the section says what each count counts, in the words
/// the description said (a word of each count, sampled).
#[test]
fn every_count_a_description_left_is_glossed_where_it_points() {
    let scratch = harness::scratch("mcp-surface-glosses");
    let mut mcp = Mcp::start(&scratch.0, &[]);
    let tools = mcp.list_tools();
    for (tool, section, glosses) in [
        (
            "search",
            "answer",
            &[
                "`items` every live item in scope before any term",
                "`never_fetched` counts the tabs, their pages and the characters",
                "never over the matches, which are `total`",
                "counted in `view.rows.left_out`",
            ][..],
        ),
        (
            "price_status",
            "pricing",
            &[
                "`counts.residue` the items carrying price text the trade site cannot see",
                "`counts.invalid_notes` the notes that start like a price",
                "`rows` accounts for every hand-written row",
            ][..],
        ),
        (
            "price_list",
            "pricing",
            &[
                "`items_on_record`, how many items the league holds",
                "`listed`, how many `items` lists",
            ][..],
        ),
        (
            "shop_render",
            "pricing",
            &[
                "`left_out_total` counts them all",
                "with how many `items` link to it",
            ][..],
        ),
        (
            "store_status",
            "store",
            &[
                "`withheld_responses` counts fetches",
                "`unlifted_items` counts item-shaped objects",
                "`granted_skills` counts the skill entries",
            ][..],
        ),
        (
            "tabs",
            "store",
            &[
                "`left_out` counts the tabs not listed",
                "`selected` counting them",
            ][..],
        ),
    ] {
        let said = tools
            .iter()
            .find(|t| t["name"] == tool)
            .and_then(|t| t["description"].as_str())
            .unwrap_or_default();
        let pointer = format!("`describe` with names [\"{section}\"]");
        assert!(said.contains(&pointer), "{tool}: {said}");
        let result = mcp.result("describe", serde_json::json!({ "names": [section] }));
        let text = result["content"][0]["text"].as_str().unwrap_or_default();
        for gloss in glosses {
            assert!(
                text.contains(gloss),
                "{tool}'s `{gloss}` is not in {section}: {text}"
            );
        }
    }
    let _ = mcp.child.kill();
}

/// The build review's P2: a sentence over a group stated one tool's
/// behaviour for all — "the job tools observe only" where `submit_job`
/// starts a mock daemon, "the pricing tools … write" where five of them
/// read. A group sentence names only the tools it is true of: the
/// daemon section's observing sentence names the four observers and no
/// submitter, and the instructions' writing line names the writers.
#[test]
fn a_group_sentence_names_only_the_tools_it_is_true_of() {
    let scratch = harness::scratch("mcp-surface-groups");
    let mut mcp = Mcp::start(&scratch.0, &[]);
    let result = mcp.result("describe", serde_json::json!({ "names": ["daemon"] }));
    let daemon: Value =
        serde_json::from_str(result["content"][0]["text"].as_str().unwrap_or("null")).unwrap();
    let observing = daemon["daemon"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|e| e["what"].as_str().filter(|w| w.contains("observe only")))
        .expect("an entry says which tools observe only");
    let (observers, _) = observing.split_once("observe only").unwrap();
    for tool in ["list_jobs", "job_status", "job_result", "daemon_status"] {
        assert!(observers.contains(tool), "{observing}");
    }
    for tool in ["submit_job", "apply_plan", "cancel_job"] {
        assert!(!observers.contains(tool), "{observing}");
    }
    let writes = mcp
        .instructions
        .lines()
        .find(|l| l.contains("write locally"))
        .expect("the instructions say which tools write");
    assert!(
        writes.contains("price_set") && writes.contains("price_clear"),
        "{writes}"
    );
    assert!(!writes.contains("pricing tools"), "{writes}");
    let _ = mcp.child.kill();
}
