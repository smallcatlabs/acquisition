# The MCP's `leagues` tool requires a field whose only legal value is `true`

Labels: area:frontends trigger:seat

`leagues` refuses `{}` because `stored` must be `true` (`crates/acquisition-mcp/src/params.rs`). Met by two readers independently.
