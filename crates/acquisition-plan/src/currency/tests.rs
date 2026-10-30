use super::*;
use crate::price_notes_fixture::price_notes;

/// The fixture the table cites: the notes' currency words (the word
/// after the amount) and the dialog's word→display list.
fn fixture() -> (Vec<String>, Vec<(String, String)>) {
    let f = price_notes();
    let note_words = f
        .notes
        .iter()
        .filter_map(|n| n.note.splitn(3, ' ').nth(2))
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect();
    (note_words, f.dialog)
}

/// C68: the shipped file parses under the loader's review checks and
/// carries the version this build cites.
#[test]
fn c68_the_shipped_table_parses_and_is_version_1() {
    let t = table().unwrap();
    assert_eq!(t.version(), CURRENCY_TABLE_VERSION);
    assert_eq!(t.rows().len(), 42);
    assert_eq!(t.active().count(), 39);
    assert!(!t.sources().is_empty());
    assert!(t.rows().iter().all(|r| !r.evidence.is_empty()));
}

/// C68: `emit` is the word the game writes — every word the in-game
/// dialog wrote into a note resolves to a row whose `emit` is that
/// word, and the dialog's list (word → display name, as the owner
/// read it) is exactly the active rows, in order.
#[test]
fn c68_the_active_rows_are_the_dialog_words_and_display_names() {
    let t = table().unwrap();
    let (note_words, dialog) = fixture();
    assert!(!note_words.is_empty());
    for word in &note_words {
        let row = t
            .resolve(word)
            .unwrap_or_else(|| panic!("{word} unresolved"));
        assert_eq!(&row.emit, word, "{word} is a note word, so it is an emit");
        assert!(
            !row.is_retired(),
            "{word}: the game wrote it, so it is not retired"
        );
    }
    let active: Vec<(String, String)> = t
        .active()
        .map(|r| (r.emit.clone(), r.display.clone()))
        .collect();
    assert_eq!(active, dialog);
    let noted: std::collections::BTreeSet<&str> = note_words.iter().map(String::as_str).collect();
    for (word, _) in &dialog {
        assert!(
            noted.contains(word.as_str()),
            "{word}: in the dialog, never in a note"
        );
    }
}

/// C68: the legacy C++ tag is an alias where it differed from the
/// game's word. The owner's shorthand joins row by row (the count is
/// not pinned — "I have probably missed some", 2026-09-05); each such
/// row cites where its aliases came from.
#[test]
fn c68_the_legacy_tags_resolve_as_aliases() {
    let t = table().unwrap();
    for (legacy, tag) in [
        ("exa", "exalted"),
        ("chrom", "chrome"),
        ("jew", "jewellers"),
        ("fuse", "fusing"),
    ] {
        let row = t.resolve(legacy).unwrap();
        assert_eq!(row.tag, tag);
        assert_eq!(row.aliases, vec![legacy.to_string()]);
        assert!(row.evidence.iter().any(|e| e.starts_with("cpp:")));
    }
    for owner_word in ["alts", "mirr", "mirrors"] {
        let row = t.resolve(owner_word).unwrap();
        assert!(
            row.evidence.iter().any(|e| e.starts_with("browser:")),
            "{owner_word}"
        );
    }
}

/// C68: a dropped currency keeps its row, marked, and still resolves
/// by tag — intent that cites it never dangles.
#[test]
fn c68_a_retired_tag_keeps_its_row_and_is_marked() {
    let t = table().unwrap();
    for tag in ["chisel", "coin", "silver"] {
        let row = t.resolve(tag).unwrap();
        assert_eq!(row.tag, tag);
        assert!(row.is_retired(), "{tag} is retired");
        assert!(row.retired.as_deref().unwrap().starts_with("2026-09-04"));
    }
    let retired: Vec<&str> = t
        .rows()
        .iter()
        .filter(|r| r.is_retired())
        .map(|r| r.tag.as_str())
        .collect();
    assert_eq!(retired, ["chisel", "coin", "silver"]);
}

/// C68: resolution is exact — the indexer's loose matching is not
/// modelled — and a word nobody wrote resolves to nothing.
#[test]
fn c68_resolution_is_exact_and_case_sensitive() {
    let t = table().unwrap();
    assert!(t.resolve("chaos").is_some());
    assert!(t.resolve("Chaos").is_none());
    assert!(t.resolve("chaos ").is_none());
    assert!(t.resolve("exalt").is_none());
    assert!(t.resolve("").is_none());
    assert!(t.by_tag("exa").is_none(), "by_tag ignores aliases");
    assert_eq!(t.by_tag("exalted").unwrap().tag, "exalted");
}

/// C68: the loader is the reviewer's checklist — a duplicate word, an
/// uncited row, a foreign version stamp, an unknown field: refused
/// whole, with the offending row named.
#[test]
fn c68_the_loader_refuses_what_a_reviewer_would() {
    let row = |tag: &str, emit: &str, aliases: &str, evidence: &str| {
        format!(
            "[[currency]]\ntag = \"{tag}\"\ndisplay = \"X\"\nemit = \"{emit}\"\naliases = [{aliases}]\nrealms = [\"pc\"]\nevidence = [{evidence}]\n"
        )
    };
    let head = "version = 1\nstatus = \"draft\"\nsources = []\n";
    let ok = format!("{head}{}", row("chaos", "chaos", "", "\"game:x\""));
    assert!(CurrencyTable::parse(&ok).is_ok());

    let dup = format!(
        "{head}{}{}",
        row("exalted", "exalted", "\"exa\"", "\"game:x\", \"cpp:x\""),
        row("exa2", "exa", "", "\"game:x\"")
    );
    assert_eq!(
        CurrencyTable::parse(&dup),
        Err(CurrencyTableError::AmbiguousWord {
            word: "exa".into(),
            tags: ["exalted".into(), "exa2".into()]
        })
    );

    let bare_alias = format!(
        "{head}{}",
        row("chaos", "chaos", "\"chaoses\"", "\"game:x\"")
    );
    assert_eq!(
        CurrencyTable::parse(&bare_alias),
        Err(CurrencyTableError::UncitedAliases {
            tag: "chaos".into()
        })
    );

    let uncited = format!("{head}{}", row("chaos", "chaos", "", "\"cpp:x\""));
    assert_eq!(
        CurrencyTable::parse(&uncited),
        Err(CurrencyTableError::Uncited {
            tag: "chaos".into()
        })
    );

    let spaced = format!("{head}{}", row("chaos", "chaos orb", "", "\"game:x\""));
    assert!(matches!(
        CurrencyTable::parse(&spaced),
        Err(CurrencyTableError::MalformedWord { .. })
    ));

    let v2 = ok.replacen("version = 1", "version = 2", 1);
    assert_eq!(
        CurrencyTable::parse(&v2),
        Err(CurrencyTableError::Version {
            found: 2,
            expected: 1
        })
    );

    let unknown = format!("{ok}colour = \"gold\"\n");
    assert!(matches!(
        CurrencyTable::parse(&unknown),
        Err(CurrencyTableError::Parse(_))
    ));

    assert_eq!(CurrencyTable::parse(head), Err(CurrencyTableError::Empty));
}

/// C68: read-only and enumerable — the JSON a surface emits is the
/// table itself, every row with its evidence, and round-trips.
#[test]
fn c68_the_table_serializes_whole_and_round_trips() {
    let t = table().unwrap();
    let json = serde_json::to_value(t).unwrap();
    assert_eq!(json["version"], 1);
    assert_eq!(json["currency"].as_array().unwrap().len(), 42);
    assert!(json["currency"][0]["evidence"].as_array().unwrap().len() >= 2);
    let back: CurrencyTable = serde_json::from_value(json).unwrap();
    assert_eq!(&back, t);
}
