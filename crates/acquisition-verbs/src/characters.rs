//! The characters on record as one document (C149, every answer one
//! object, its lists counted beside their rows; C53, the read economy):
//! the counts the CLI's footer prints, made over every character the
//! selection holds, then the rows the request asked for — ten, more or
//! fewer on `limit`, every one on `expand` — and how many it left out.
//! One maker for `acq store characters` (its text and `--json`), the
//! MCP's `characters` and the web UI's refresh page (C46).
//!
//! The rows are the store's own (`CharacterRow`), in the store's order
//! (by realm, then league, then level, highest first, then name), so the
//! ten are the highest-level characters of the first league listed. A
//! count's words are C146's: each says what it counts, on every surface
//! (the CLI's footer, these fields' docs, and the MCP's `describe` entry
//! `character counts`).

use acquisition_store::CharacterRow;
use acquisition_store::bound::bounded;
use serde::Serialize;

/// The characters a selection holds, counted, then listed.
#[derive(Clone, Debug, Serialize)]
pub struct Characters {
    /// The realm asked about, when one was; every realm otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    /// The league asked about, when one was; every league otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub league: Option<String>,
    /// Every character the selection holds.
    pub characters: usize,
    /// Of them, those a full fetch (equipment and inventory) has filled.
    pub fetched: usize,
    /// Of the fetched, those whose last full fetch carried no items
    /// (`fetched_items` 0): fetched, and empty.
    pub empty_bodies: usize,
    /// Of them, those listed and never fetched: no answer says what they
    /// hold.
    pub never_fetched: usize,
    /// The live items the store holds at every one of them.
    pub items: i64,
    /// The characters listed: ten, the request's `limit`, or every one
    /// under its `expand`, in the store's order.
    pub rows: Vec<CharacterRow>,
    /// The characters not in `rows`.
    pub left_out: usize,
    /// What lists them all: the request's `expand` (its `limit` lists
    /// more).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

impl Characters {
    /// The selection's characters, as the store lists them, counted whole
    /// and then cut by the read economy's one rule (C53), each listed
    /// row's listing and fetch said as ages at `now`, the answer's time.
    pub fn of(
        realm: Option<&str>,
        league: Option<&str>,
        rows: Vec<CharacterRow>,
        limit: Option<usize>,
        expand: bool,
        now: i64,
    ) -> Characters {
        let characters = rows.len();
        let fetched = rows.iter().filter(|c| c.fetched_at.is_some()).count();
        let empty_bodies = rows
            .iter()
            .filter(|c| c.fetched_at.is_some() && c.fetched_items == Some(0))
            .count();
        let items = rows.iter().map(|c| c.item_count).sum();
        let (rows, left_out) = bounded(rows, limit, expand);
        let rows = rows.into_iter().map(|row| row.aged(now)).collect();
        Characters {
            realm: realm.map(str::to_string),
            league: league.map(str::to_string),
            characters,
            fetched,
            empty_bodies,
            never_fetched: characters - fetched,
            items,
            rows,
            left_out,
            left_out_needs: (left_out > 0).then_some("expand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acquisition_store::bound::LISTED;

    /// The answer's time the documents here are made at (C53: an age
    /// under test is made from a time given, never the clock).
    const NOW: i64 = 2 * 3600 + 1;

    fn character(n: usize, fetched: bool, items: i64) -> CharacterRow {
        CharacterRow {
            id: format!("c{n:02}"),
            name: format!("Exile{n}"),
            realm: "pc".into(),
            league: Some("Standard".into()),
            class: None,
            level: Some(90),
            listed_at: Some(1),
            listed_at_age: None,
            fetched_at: fetched.then_some(2),
            fetched_at_age: None,
            item_count: items,
            fetched_items: fetched.then_some(items),
        }
    }

    /// The release seat made two counts by eye from a bare array of 65
    /// rows and retracted both (C149): the counts stand whole beside the
    /// ten rows, the ones the footer prints.
    #[test]
    fn c149_characters_are_counted_whole_beside_ten_rows() {
        // 65 characters: 50 fetched, 45 of them with 3 items each and 5
        // fetched empty, and 15 never fetched
        let rows: Vec<CharacterRow> = (0..65)
            .map(|n| match n {
                0..45 => character(n, true, 3),
                45..50 => character(n, true, 0),
                _ => character(n, false, 0),
            })
            .collect();
        let listed = Characters::of(None, Some("Standard"), rows.clone(), None, false, NOW);
        assert_eq!(listed.characters, 65);
        assert_eq!(listed.fetched, 50);
        assert_eq!(listed.empty_bodies, 5);
        assert_eq!(listed.never_fetched, 15);
        assert_eq!(listed.items, 135);
        assert_eq!(listed.rows.len(), LISTED);
        assert_eq!(listed.rows[0].id, "c00", "the store's order, its head kept");
        assert_eq!(listed.left_out, 65 - LISTED);
        assert_eq!(listed.left_out_needs, Some("expand"));
        let doc = serde_json::to_value(&listed).unwrap();
        assert!(doc.is_object(), "{doc}");
        assert_eq!(doc["league"], "Standard");
        assert!(
            doc.get("realm").is_none(),
            "an unasked realm is absent: {doc}"
        );

        let more = Characters::of(None, None, rows.clone(), Some(20), false, NOW);
        assert_eq!((more.rows.len(), more.left_out), (20, 45));
        let none = Characters::of(None, None, rows.clone(), Some(0), false, NOW);
        assert_eq!(
            (none.rows.len(), none.left_out, none.characters),
            (0, 65, 65)
        );
        let all = Characters::of(None, None, rows, None, true, NOW);
        assert_eq!((all.rows.len(), all.left_out), (65, 0));
        assert_eq!(all.left_out_needs, None);
    }

    /// C53 (an age the text prints stands in words beside its epoch,
    /// made at the answer's own time): a listed character says its
    /// listing and its fetch as the CLI's columns do, `never` beside a
    /// null, under each epoch's name and `_age`.
    #[test]
    fn c53_a_listed_character_says_its_listing_and_fetch_as_ages() {
        let rows = vec![character(0, true, 3), character(1, false, 0)];
        let listed = Characters::of(None, None, rows, None, false, NOW);
        let doc = serde_json::to_value(&listed).unwrap();
        let fetched = &doc["rows"][0];
        assert_eq!(fetched["listed_at"], 1);
        assert_eq!(fetched["listed_at_age"], "2h ago", "{doc}");
        assert_eq!(fetched["fetched_at"], 2);
        assert_eq!(fetched["fetched_at_age"], "1h ago", "{doc}");
        let never = &doc["rows"][1];
        assert_eq!(never["fetched_at"], serde_json::Value::Null);
        assert_eq!(never["fetched_at_age"], "never", "{doc}");
    }
}
