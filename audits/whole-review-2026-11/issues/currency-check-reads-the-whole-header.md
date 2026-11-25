# The resident corpus rebuilds its whole header to compare a revision

Labels: area:search trigger:gui

`Corpus::is_current` (`crates/acquisition-search/src/corpus.rs:856-864`) builds every realm, listing and location to compare two scalars, under the holder's lock; `Store::revision()` already answers in one row. Unfiled since 2026-10-19.
