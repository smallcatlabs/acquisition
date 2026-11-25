# `Basis::against` leaves out the properties table

Labels: area:search trigger:step

The basis carries the properties version in neither its difference list nor its text line (`crates/acquisition-search/src/corpus.rs`, `search_cmd.rs:764`), so a continuation made before a properties-only change is refused with an empty list of what moved.
