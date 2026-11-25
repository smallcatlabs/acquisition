# `Store::rebuild` re-extracts item columns only

Labels: area:store trigger:step

`rebuild` (`crates/acquisition-store/src/lib.rs:2572-2594`) updates nine item columns from each row's JSON and no extracted tab or character column, though its doc says every derived column.
