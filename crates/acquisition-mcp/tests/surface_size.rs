//! The first turn's cost, held (#83): an agent pays for the
//! `initialize` instructions and every tool of `tools/list` before its
//! first call, so each text is held to a length and the surface cannot
//! grow back. A tool's description says what it answers, what it costs
//! and where the rest is — `describe` by name, read once — and a
//! parameter's doc says what the argument is.

use crate::harness;

use harness::Mcp;
use serde_json::Value;

/// A tool's description, in bytes.
const DESCRIPTION: usize = 600;
/// A parameter's description, in bytes.
const PARAMETER: usize = 300;
/// The instructions, in bytes.
const INSTRUCTIONS: usize = 1_000;

/// C146 (a count says what it counts, in the tool's description among
/// the surfaces it stands on): the tools whose text carries a count's
/// gloss, each held to the bound its glosses need while the owner's
/// ruling on where the MCP's gloss stands is pending (the brief's Do 1).
const GLOSSED: [(&str, usize); 6] = [
    ("search", 2_000),
    ("price_status", 1_500),
    ("store_status", 1_100),
    ("shop_render", 800),
    ("tabs", 720),
    ("price_list", 680),
];

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
        let bound = GLOSSED
            .iter()
            .find(|(glossed, _)| *glossed == name)
            .map_or(DESCRIPTION, |(_, bound)| *bound);
        let said = tool["description"].as_str().unwrap_or_default().len();
        if said > bound {
            over.push(format!("`{name}`: description {said} bytes, bound {bound}"));
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
