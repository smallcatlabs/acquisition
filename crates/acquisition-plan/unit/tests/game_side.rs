use super::*;
use crate::currency::table;
use crate::price_notes_fixture::{Note, price_notes};

fn note(text: &str) -> GamePrice {
    read(Source::Note, text, table().unwrap())
}

fn tab(text: &str) -> GamePrice {
    read(Source::TabName, text, table().unwrap())
}

fn price(amount: &str, tag: &str) -> Price {
    Price {
        amount: amount.parse().unwrap(),
        currency: tag.into(),
    }
}

/// C69, T10 — every note the game wrote reads as the game meant it:
/// `~skip ` is skip, the empty amount is invalid (never "no price"),
/// `~b/o` is negotiable, everything else is exact; the amount's
/// canonical text is the note's own and the currency is the tag of
/// the row whose `emit` is the note's word.
#[test]
fn c69_every_fixture_note_reads_as_the_game_meant_it() {
    let fixture = price_notes();
    assert!(fixture.notes.len() >= 50);
    let t = table().unwrap();
    for Note { note, .. } in &fixture.notes {
        let got = read(Source::Note, note, t);
        let mut parts = note.trim_end().splitn(3, ' ');
        let (prefix, amount, word) = (
            parts.next().unwrap(),
            parts.next().unwrap_or(""),
            parts.next().unwrap_or(""),
        );
        match (prefix, amount) {
            ("~skip", "") => assert_eq!(got, GamePrice::Skip, "{note:?}"),
            (_, "") => assert!(
                matches!(&got, GamePrice::Invalid { why } if why.contains("empty")),
                "{note:?} → {got}"
            ),
            (_, _) => {
                let p = got
                    .price()
                    .unwrap_or_else(|| panic!("{note:?} → {got}, not a price"));
                assert_eq!(p.amount.to_string(), amount, "{note:?}");
                assert_eq!(p.currency, t.resolve(word).unwrap().tag, "{note:?}");
                assert_eq!(&t.resolve(word).unwrap().emit, word);
                assert_eq!(
                    got.kind(),
                    if prefix == "~b/o" {
                        "negotiable"
                    } else {
                        "exact"
                    }
                );
            }
        }
    }
    let kinds = |k: &str| {
        fixture
            .notes
            .iter()
            .filter(|n| note(&n.note).kind() == k)
            .count()
    };
    assert_eq!(kinds("skip"), 1);
    assert_eq!(kinds("invalid"), 1);
    assert_eq!(kinds("negotiable"), 2);
    assert_eq!(kinds("none"), 0);
}

/// C69, T11 — every priced tab name in the owner's listing reads as
/// an exact price despite the game's `(Remove-only)` and the owner's
/// `(A)`…`(G)` after the word; the same text as a *note* reads the
/// same (parser v2: the game stores a note's suffix and the site
/// reads through it — the owner's `~price 777 chaos testing`,
/// 2026-09-08).
#[test]
fn c69_text_after_the_word_is_tolerated_by_a_tab_name_and_a_note_alike() {
    let fixture = price_notes();
    assert!(fixture.tab_names.len() >= 13);
    for name in &fixture.tab_names {
        let got = tab(name);
        let p = got.price().unwrap_or_else(|| panic!("{name:?} → {got}"));
        assert_eq!(got.kind(), "exact", "{name:?}");
        assert!(matches!(p.amount, Amount::Decimal { .. }), "{name:?}");
        assert!(
            ["chaos", "divine"].contains(&p.currency.as_str()),
            "{name:?}"
        );
        if name.contains('(') {
            assert_eq!(note(name), got, "{name:?} as a note");
        }
    }
    assert_eq!(
        note("~price 777 chaos testing"),
        GamePrice::Exact(price("777", "chaos"))
    );
    assert_eq!(
        tab("~price 30 chaos (C)"),
        GamePrice::Exact(price("30", "chaos"))
    );
    assert_eq!(
        tab("~price 20 chaos (A) (Remove-only)"),
        GamePrice::Exact(price("20", "chaos"))
    );
    // The suffix is tolerated, not read: the word must still stand
    // alone, under both sources.
    assert!(matches!(
        tab("~price 30 chaos(C)"),
        GamePrice::Invalid { .. }
    ));
    assert!(matches!(
        note("~price 30 chaos(C)"),
        GamePrice::Invalid { .. }
    ));
}

/// C69, T2, T11 — a lot ratio is a price in a note (the owner's own
/// `~price 22/10 chaos`, kept unreduced) and invalid in a tab name,
/// where it unlists the whole tab.
#[test]
fn c69_a_ratio_is_a_price_in_a_note_and_invalid_in_a_tab_name() {
    for (text, wanted, lot) in [
        ("~price 22/10 chaos", 22, 10),
        ("~price 55/600 chaos", 55, 600),
        ("~price 10/80 chaos", 10, 80),
        ("~price 3/1 alch", 3, 1),
    ] {
        let got = note(text);
        assert_eq!(
            got.price().unwrap().amount,
            Amount::Ratio { wanted, lot },
            "{:?}",
            text
        );
        assert!(
            matches!(&tab(text), GamePrice::Invalid { why } if why.contains("in a tab name is invalid")),
            "{text:?} as a tab name"
        );
    }
    // The grammar does not know that the dialog refuses a ratio for
    // `~b/o` (T12); what it means is the consumer's rule.
    assert_eq!(
        note("~b/o 2/35 divine"),
        GamePrice::Negotiable(price("2/35", "divine"))
    );
}

/// C69, T10 — the game's residue after an invalid entry is invalid,
/// never "no price"; `~skip` with or without the game's trailing
/// space is skip; `~skip` followed by anything in a note is invalid.
#[test]
fn c69_the_games_residue_is_invalid_and_skip_is_skip() {
    assert!(matches!(&note("~price  chaos"), GamePrice::Invalid { why } if why.contains("empty")));
    assert!(matches!(&note("~b/o  divine"), GamePrice::Invalid { why } if why.contains("empty")));
    assert_eq!(note("~skip "), GamePrice::Skip);
    assert_eq!(note("~skip"), GamePrice::Skip);
    assert!(matches!(note("~skip this one"), GamePrice::Invalid { .. }));
    assert_eq!(tab("~skip (Remove-only)"), GamePrice::Skip);
    assert!(matches!(note("~price"), GamePrice::Invalid { .. }));
    assert!(matches!(note("~price chaos"), GamePrice::Invalid { .. }));
    assert!(matches!(note("~price 5 "), GamePrice::Invalid { .. }));
}

/// C69 — a leading `~` opens a price note: text without it is none,
/// whatever it says; a `~` prefix the game does not write is invalid
/// (the C++ app's `~c/o` and `~gb/o` included).
#[test]
fn c69_no_marker_is_none_and_an_unknown_marker_is_invalid() {
    for text in [
        "",
        "50",
        "Sale ~price 5 chaos",
        "price 5 chaos",
        " ~price 5 chaos",
    ] {
        assert_eq!(note(text), GamePrice::None, "{:?}", text);
        assert_eq!(tab(text), GamePrice::None, "{:?}", text);
    }
    for text in ["~c/o 5 chaos", "~gb/o 5 chaos", "~PRICE 5 chaos", "~"] {
        assert!(
            matches!(&note(text), GamePrice::Invalid { why } if why.contains("prefix")),
            "{:?}",
            text
        );
    }
}

/// C69, C68 — the word is resolved through the shipped table, exact
/// and case-sensitive: a cited alias reads as the row's tag, a
/// retired word still reads as a price, and an unknown or miscased
/// word is invalid naming the table version.
#[test]
fn c69_the_currency_word_resolves_to_a_tag_or_is_invalid() {
    assert_eq!(
        note("~price 5 exa"),
        GamePrice::Exact(price("5", "exalted"))
    );
    assert_eq!(
        note("~price 5 exalted"),
        GamePrice::Exact(price("5", "exalted"))
    );
    assert_eq!(
        note("~b/o 1.5 divine"),
        GamePrice::Negotiable(price("1.5", "divine"))
    );
    let retired = note("~price 5 chisel");
    assert_eq!(retired, GamePrice::Exact(price("5", "chisel")));
    assert!(table().unwrap().by_tag("chisel").unwrap().is_retired());
    for text in ["~price 5 Chaos", "~price 5 chaoss"] {
        assert!(
            matches!(&note(text), GamePrice::Invalid { why } if why.contains("v1")),
            "{text:?} → {}",
            note(text)
        );
    }
    // `chaos orb`: the word is `chaos`, `orb` a tolerated suffix (v2).
    assert_eq!(
        note("~price 5 chaos orb"),
        GamePrice::Exact(price("5", "chaos"))
    );
}

/// C69, C67 — the amount is `Amount`'s grammar: a fifth decimal, a
/// zero, a leading zero or a sign is invalid, naming the amount rule;
/// the game's own four places and five digits read back verbatim.
#[test]
fn c69_the_amount_is_the_typed_grammar() {
    assert_eq!(
        note("~price 999.1234 chaos"),
        GamePrice::Exact(price("999.1234", "chaos"))
    );
    assert_eq!(
        note("~price 12345 chaos"),
        GamePrice::Exact(price("12345", "chaos"))
    );
    for text in [
        "~price 999.12345 chaos",
        "~price 0 chaos",
        "~price 05 chaos",
        "~price -5 chaos",
        "~price 5. chaos",
        "~price 10/0 chaos",
    ] {
        assert!(
            matches!(&note(text), GamePrice::Invalid { why } if why.starts_with("amount")),
            "{text:?} → {}",
            note(text)
        );
    }
}

/// The plan step 7 property tests (`PRICING-SLICE.md` at `b6197a82`): the parser
/// over any text, not the corpus and the hand-picked lines above.
mod properties {
    use super::*;
    use proptest::prelude::*;

    /// Every word the shipped table resolves: tag, emit and alias.
    fn table_word() -> impl Strategy<Value = String> {
        let words: Vec<String> = table()
            .unwrap()
            .rows()
            .iter()
            .flat_map(|r| r.words().map(str::to_string))
            .collect();
        prop::sample::select(words)
    }

    /// Spellings of the amount grammar, zero included (refused).
    fn amount_text() -> impl Strategy<Value = String> {
        prop_oneof![
            "[1-9][0-9]{0,4}",
            "[1-9][0-9]{0,4}\\.[0-9]{1,4}",
            "0\\.[0-9]{1,4}",
            "[1-9][0-9]{0,2}/[1-9][0-9]{0,3}",
        ]
    }

    fn valid_amount() -> impl Strategy<Value = Amount> {
        amount_text().prop_filter_map("a positive amount", |t| t.parse().ok())
    }

    /// A piece a hand or the game might write: the grammar's words,
    /// the neighbours that trip it, the game's suffixes, and noise.
    fn piece() -> impl Strategy<Value = String> {
        prop_oneof![
            Just(EXACT.to_string()),
            Just(NEGOTIABLE.to_string()),
            Just(SKIP.to_string()),
            Just("~c/o".to_string()),
            Just("~".to_string()),
            Just(String::new()),
            Just("0".to_string()),
            Just("-5".to_string()),
            Just("5.".to_string()),
            Just("(Remove-only)".to_string()),
            Just("(A)".to_string()),
            amount_text(),
            table_word(),
            "\\PC{0,6}",
            "\\s{1,2}",
        ]
    }

    /// Pieces joined by one space (an empty piece leaves the game's
    /// double space), or any string at all.
    fn any_text() -> impl Strategy<Value = String> {
        prop_oneof![
            3 => prop::collection::vec(piece(), 0..6).prop_map(|ps| ps.join(" ")),
            1 => any::<String>(),
        ]
    }

    fn any_source() -> impl Strategy<Value = Source> {
        prop_oneof![Just(Source::Note), Just(Source::TabName)]
    }

    proptest! {
        /// C47, C69 — any text under either source yields a reading and
        /// never panics; trailing whitespace changes nothing; no `~` is
        /// `none`; a price reading is the grammar re-read from the text
        /// itself — the amount parses to the price's, the word resolves
        /// to its tag, and the price written back reads the same; a
        /// skip is `~skip` (a tab name with anything after it); an
        /// invalid reading names why.
        #[test]
        fn c47_c69_any_text_reads_and_a_price_is_its_own_grammar(
            text in any_text(),
            source in any_source(),
        ) {
            let t = table().unwrap();
            let got = read(source, &text, t);
            let trimmed = text.trim_end();
            prop_assert_eq!(matches!(got, GamePrice::None), !trimmed.starts_with('~'), "{:?} → {}", text, got);
            prop_assert_eq!(read(source, &format!("{text} \t"), t), got.clone(), "{:?}", text);
            match &got {
                GamePrice::Exact(p) | GamePrice::Negotiable(p) => {
                    let prefix = if matches!(got, GamePrice::Exact(_)) { EXACT } else { NEGOTIABLE };
                    let rest = trimmed.strip_prefix(prefix).and_then(|r| r.strip_prefix(' '));
                    prop_assert!(rest.is_some(), "{:?} → {}", text, got);
                    let (amount, after) = rest.unwrap().split_once(' ').unwrap();
                    prop_assert_eq!(amount.parse::<Amount>(), Ok(p.amount), "{:?}", text);
                    // The word stands before the first whitespace under both
                    // sources; what follows it is tolerated, not read (module
                    // doc). proptest found `~price 1 chaos 0` on 2026-09-09.
                    let word = after.split(char::is_whitespace).next().unwrap();
                    prop_assert_eq!(&t.resolve(word).unwrap().tag, &p.currency, "{:?}", text);
                    if source == Source::TabName {
                        prop_assert!(matches!(p.amount, Amount::Decimal { .. }), "{:?}", text);
                    }
                    let written = format!("{prefix} {} {}", p.amount, t.by_tag(&p.currency).unwrap().emit);
                    prop_assert_eq!(read(source, &written, t), got.clone(), "{:?} → {}", text, written);
                }
                GamePrice::Skip => prop_assert!(
                    trimmed == SKIP || (source == Source::TabName && trimmed.starts_with("~skip ")),
                    "{:?}", text
                ),
                GamePrice::Invalid { why } => {
                    prop_assert!(!why.is_empty());
                    prop_assert!(trimmed.starts_with('~'));
                }
                GamePrice::None => {}
            }
        }

        /// C69, T10, T11 — a well-formed price reads as written under
        /// both sources, the word resolved to its row's tag; a ratio is
        /// invalid in a tab name (T11); text after the word is tolerated
        /// by both sources (parser v2), whitespace too.
        #[test]
        fn c69_a_well_formed_price_reads_as_written(
            negotiable in any::<bool>(),
            amount in valid_amount(),
            word in table_word(),
            suffix in prop_oneof![Just(String::new()), "\\s\\PC{0,12}", "\\s{0,3}"],
        ) {
            let t = table().unwrap();
            let prefix = if negotiable { NEGOTIABLE } else { EXACT };
            let price = Price { amount, currency: t.resolve(&word).unwrap().tag.clone() };
            let want = if negotiable { GamePrice::Negotiable(price) } else { GamePrice::Exact(price) };
            let bare = format!("{prefix} {amount} {word}");
            prop_assert_eq!(read(Source::Note, &bare, t), want.clone(), "{:?}", bare);
            let as_tab = read(Source::TabName, &bare, t);
            if matches!(amount, Amount::Ratio { .. }) {
                prop_assert!(matches!(&as_tab, GamePrice::Invalid { why } if why.contains("in a tab name is invalid")), "{:?} → {}", bare, as_tab);
            } else {
                prop_assert_eq!(as_tab.clone(), want.clone(), "{:?}", bare);
            }
            let with_suffix = format!("{bare}{suffix}");
            prop_assert_eq!(read(Source::TabName, &with_suffix, t), as_tab, "{:?}", with_suffix);
            prop_assert_eq!(read(Source::Note, &with_suffix, t), want, "{:?}", with_suffix);
        }
    }
}
