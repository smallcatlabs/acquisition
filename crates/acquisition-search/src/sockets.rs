//! The sockets (C101): what `sockets`, `links`, `sockets.<word>` and a
//! link group's `red green blue white abyssal size` count on one item, and what
//! leaves each open. Arithmetic over the collection the deriver read
//! (`derive::Item::sockets`), and nothing of the query tree: what
//! `linked( … )` means is `group.rs`'s, which reads the counts here.
//!
//! The rules that decide a count are this doc's (C101).
//!
//! # Rulings held here
//!
//! - **C101** (colours, and abyssal sockets beside them, over the item
//!   and within one link group): Socket words are asked two ways: over
//!   the whole item — `sockets.red>=2`, a count over every socket the item
//!   has — and within a link group — `linked(red>=3 green>=1)`, where every
//!   word and count named must be met together by one group (S59).
//!   `sockets` is how many sockets the item has, `links` the size of its
//!   largest link group, a link group's `size` its own count. Every socket
//!   counts in `sockets`, whatever its letter: an abyssal socket (GGG's
//!   `A`) is asked for by its word, `abyssal`, as a colour is, and is a
//!   link group of its own on every item the site has shown (#185); a
//!   resonator's (`DV`) is a socket no word names, asked for by its base
//!   (K1).
//!
//! # As built
//!
//! - **A socket word is GGG's letter** ([`SOCKET_WORDS`]): `red` is `R`,
//!   `green` `G`, `blue` `B`, `white` `W`, `abyssal` `A`, matched in any
//!   case as every closed word is (B1); a socket's letter is compared as
//!   GGG spelled it. A word outside the table is an authoring error
//!   offering every one. The table is the one list of the words: the
//!   binder's fields, a link group's counts, the help, the trade site's
//!   keys beside them (`trade/translate.rs`) and the search form's boxes
//!   each read it.
//! - **A count is an interval** ([`Counted`]): what was read is its floor,
//!   and each socket that may or may not add to it — an element that was
//!   no socket, a socket whose colour or group could not be read — widens
//!   its ceiling by one (rule 8 of the plan, in the unit of one socket).
//!   A comparison on it is decided when the whole interval agrees and
//!   undecided otherwise (`Counted::truth`); the value is established
//!   when the interval is one number, which is when it sorts and sums as
//!   a value and `undecided( … )` of it is false (`eval::scalar`).
//!   `sockets` counts every element, so only an element that was no
//!   socket leaves it open; `sockets.<word>` is open by that and by a
//!   socket whose colour is unread; `links` and a group's counts by that
//!   and by a socket whose group is unread, which may sit in any group or
//!   make one of its own ([`Groups`]). A socket whose group is unread is
//!   a socket still: it adds to its own colour's ceiling and to no other,
//!   it makes a link group exist, and alone — no group known, no element
//!   unread — it is a group of one, exactly. A colour a socket's group
//!   could turn is a reason a `linked( … )` is open.
//! - **Present and absent** (C93): an item whose body carries no `sockets`
//!   lacks every count, so `sockets>=1` is false on a ring and `-has:sockets`
//!   finds it. An item whose collection is present holds `sockets` and
//!   every `sockets.<word>` as a number — zero on GGG's `[]`, a poe2
//!   weapon that grants a skill (N44) — and `links` only while it has a
//!   link group: the largest of no groups is nothing, so such an item
//!   lacks `links`, `-has:links` finds it, and a `linked( … )` asked of it
//!   is lacked as a `line( … )` is of an item with no such occurrence.
//!   A poe2 socket has no colour, known: `sockets.red` is 0 on it.
//! - **The layout** ([`layout`]) is the collection as the game shows it:
//!   groups in GGG's order, the sockets of a group joined by `-`, groups
//!   set apart by a space — `R-R-G B W` — a socket with no colour shown
//!   by its `type` or as `?`, one whose colour is unread as `?`, and one
//!   whose group is unread set apart last, marked `(group unread)`. It
//!   is what `show` and a row print, made here once (rule 10).

use std::sync::LazyLock;

use crate::bind::{FieldDef, Kind, NumTest, Thing};
use crate::derive::{Item, Part, Socket};
use crate::group::Truth;
use crate::tree::Op;

/// One socket word of the language: the word a query names it by, the
/// letter GGG spells it with in `sColour`, and the key the trade site's
/// socket filters give its count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketWord {
    pub word: &'static str,
    pub letter: &'static str,
    pub key: &'static str,
}

/// The socket words (the module doc): every list of the words, the
/// letters or the site's keys reads this table.
pub const SOCKET_WORDS: [SocketWord; 5] = [
    SocketWord {
        word: "red",
        letter: "R",
        key: "r",
    },
    SocketWord {
        word: "green",
        letter: "G",
        key: "g",
    },
    SocketWord {
        word: "blue",
        letter: "B",
        key: "b",
    },
    SocketWord {
        word: "white",
        letter: "W",
        key: "w",
    },
    SocketWord {
        word: "abyssal",
        letter: "A",
        key: "a",
    },
];

/// The words a query names a socket by, in the table's order.
pub fn words() -> Vec<&'static str> {
    SOCKET_WORDS.iter().map(|s| s.word).collect()
}

/// GGG's letter for a socket word, in any case.
pub fn letter(word: &str) -> Option<&'static str> {
    SOCKET_WORDS
        .iter()
        .find(|s| s.word.eq_ignore_ascii_case(word))
        .map(|s| s.letter)
}

/// The binder's `sockets.<word>` fields, one a row of the table, made
/// once (`bind::fields`).
pub(crate) fn field_defs() -> &'static [FieldDef] {
    static NAMES: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
        SOCKET_WORDS
            .iter()
            .map(|s| {
                (
                    format!("sockets.{}", s.word),
                    format!(
                        "how many {} sockets the item has, over every link group",
                        s.word
                    ),
                )
            })
            .collect()
    });
    static DEFS: LazyLock<Vec<FieldDef>> = LazyLock::new(|| {
        SOCKET_WORDS
            .iter()
            .zip(NAMES.iter())
            .map(|(s, (name, what))| FieldDef {
                name: name.as_str(),
                thing: Thing::SocketColour(s.letter),
                kind: Kind::Number,
                what: what.as_str(),
            })
            .collect()
    });
    &DEFS
}

/// A count over the sockets, as far as it was read (the module doc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Counted {
    /// The item has no such count: no collection, or — for `links` — no
    /// link group.
    Absent,
    /// At least `low` and at most `high`; established when the two are one.
    Range { low: usize, high: usize },
}

impl Counted {
    /// The number, where it is established.
    pub fn value(self) -> Option<usize> {
        match self {
            Counted::Range { low, high } if low == high => Some(low),
            _ => None,
        }
    }

    /// What was read of it, where it is not established: the floor.
    pub fn floor(self) -> Option<usize> {
        match self {
            Counted::Range { low, .. } => Some(low),
            Counted::Absent => None,
        }
    }

    /// A comparison over the interval: true when every count in it
    /// satisfies the test, false when none does, undecided otherwise.
    /// Absent is false (C93).
    pub(crate) fn truth(self, test: &NumTest) -> Truth {
        let Counted::Range { low, high } = self else {
            return Truth::False;
        };
        let holds = |n: usize| test.holds(n as f64);
        let (at_low, at_high) = (holds(low), holds(high));
        match test {
            // monotone in n: the ends decide
            NumTest::Cmp(Op::Gt | Op::Ge, _) => {
                if at_low {
                    Truth::True
                } else if at_high {
                    Truth::Undecided
                } else {
                    Truth::False
                }
            }
            NumTest::Cmp(Op::Lt | Op::Le, _) => {
                if at_high {
                    Truth::True
                } else if at_low {
                    Truth::Undecided
                } else {
                    Truth::False
                }
            }
            // `=` and a range hold on an interval of counts: all of it, or
            // none of it, or some
            _ => {
                if (low..=high).all(holds) {
                    Truth::True
                } else if (low..=high).any(holds) {
                    Truth::Undecided
                } else {
                    Truth::False
                }
            }
        }
    }
}

/// The sockets of one item, read for counting: the collection, how many
/// elements were no socket, and the sockets by group.
pub struct Groups<'a> {
    sockets: &'a [Socket],
    /// Elements that were no socket: each may be a socket of any colour
    /// in any group.
    junk: usize,
    /// Sockets whose group could not be read: each may sit in any group,
    /// or make one of its own.
    unplaced: usize,
    /// The groups GGG numbered, in order of their number: (group, the
    /// sockets in it by their place in the collection).
    groups: Vec<(i64, Vec<usize>)>,
}

/// One link group's counts, as far as they were read: the floor is what
/// its own sockets say, the ceiling adds every socket that may belong to
/// it (the module doc).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GroupCounts {
    /// The sockets of the group, by their place in the collection; empty
    /// for the group that unplaced sockets may make of their own.
    pub members: Vec<usize>,
    pub size: Counted,
    /// By GGG's letter, the letters the group's sockets carry, and every
    /// word's of the table each with a count even when zero.
    colours: Vec<(String, Counted)>,
}

impl GroupCounts {
    /// How many sockets of GGG's colour `letter` the group holds.
    pub fn colour(&self, letter: &str) -> Counted {
        self.colours
            .iter()
            .find(|(l, _)| l == letter)
            .map_or(Counted::Absent, |(_, n)| *n)
    }
}

/// The collection of an item, or none where the body carries no
/// `sockets` — the array itself being unread is the same absence, and
/// what says so is the deriver's unread part.
pub fn groups(item: &Item) -> Option<Groups<'_>> {
    let sockets = item.sockets.as_deref()?;
    let junk = item
        .unread
        .iter()
        .filter(|u| u.part == Part::Sockets)
        .count();
    let mut groups: Vec<(i64, Vec<usize>)> = Vec::new();
    let mut unplaced = 0;
    for (i, socket) in sockets.iter().enumerate() {
        match socket.group {
            None => unplaced += 1,
            Some(group) => match groups.iter_mut().find(|(g, _)| *g == group) {
                Some((_, members)) => members.push(i),
                None => groups.push((group, vec![i])),
            },
        }
    }
    groups.sort_by_key(|(group, _)| *group);
    Some(Groups {
        sockets,
        junk,
        unplaced,
        groups,
    })
}

impl Groups<'_> {
    /// `sockets`: how many sockets the item has.
    pub fn count(&self) -> Counted {
        Counted::Range {
            low: self.sockets.len(),
            high: self.sockets.len() + self.junk,
        }
    }

    /// `sockets.<word>`: how many of GGG's letter `letter` over the item.
    pub fn colour(&self, letter: &str) -> Counted {
        let known = self
            .sockets
            .iter()
            .filter(|s| s.colour.as_deref() == Some(letter))
            .count();
        let unread = self.sockets.iter().filter(|s| s.colour_unread).count();
        Counted::Range {
            low: known,
            high: known + unread + self.junk,
        }
    }

    /// Whether the item is known to have a link group: one GGG numbered,
    /// or a socket whose group is unread, which sits in one whatever it is.
    pub fn has_group(&self) -> bool {
        !self.groups.is_empty() || self.unplaced > 0
    }

    /// Whether the item may have a link group and is not known to: an
    /// element that may be a socket, and nothing that is one.
    pub fn may_have_group(&self) -> bool {
        !self.has_group() && self.junk > 0
    }

    /// The sockets whose group is unread that may add to a colour's count
    /// of a group they are not already members of: those of that colour,
    /// and those whose colour is unread too (else the socket alone in its
    /// collection is counted twice).
    fn unplaced_of(&self, letter: &str, members: &[usize]) -> usize {
        self.sockets
            .iter()
            .enumerate()
            .filter(|(i, s)| {
                s.group.is_none()
                    && !members.contains(i)
                    && (s.colour.as_deref() == Some(letter) || s.colour_unread)
            })
            .count()
    }

    /// `links`: the size of the largest link group. Absent while no socket
    /// is known and no element may be one; at least one while a socket is.
    pub fn links(&self) -> Counted {
        let may_join = self.unplaced + self.junk;
        match self.groups.iter().map(|(_, members)| members.len()).max() {
            Some(largest) => Counted::Range {
                low: largest,
                high: largest + may_join,
            },
            None if self.unplaced > 0 => Counted::Range {
                low: 1,
                high: may_join,
            },
            None if self.junk > 0 => Counted::Range {
                low: 0,
                high: self.junk,
            },
            None => Counted::Absent,
        }
    }

    /// Every link group the item may have, each with its counts: the
    /// groups GGG numbered, a socket whose group is unread on its own
    /// where it is the whole collection (a group of one, exactly), and —
    /// where a socket may sit in any group — one more, of the sockets that
    /// may make a group of their own, whose members are none since it may
    /// not exist.
    pub(crate) fn each(&self) -> Vec<GroupCounts> {
        let may_join = self.unplaced + self.junk;
        let letters = |of: &[&Socket]| {
            let mut letters: Vec<String> =
                SOCKET_WORDS.iter().map(|s| s.letter.to_string()).collect();
            for socket in of {
                if let Some(colour) = &socket.colour
                    && !letters.contains(colour)
                {
                    letters.push(colour.clone());
                }
            }
            letters
        };
        let counts = |members: &[usize], others: usize| {
            let of: Vec<&Socket> = members.iter().map(|i| &self.sockets[*i]).collect();
            let unread = of.iter().filter(|s| s.colour_unread).count();
            let colours = letters(&of)
                .into_iter()
                .map(|letter| {
                    let known = of
                        .iter()
                        .filter(|s| s.colour.as_deref() == Some(&*letter))
                        .count();
                    let high = known + unread + self.unplaced_of(&letter, members) + self.junk;
                    (letter, Counted::Range { low: known, high })
                })
                .collect();
            GroupCounts {
                members: members.to_vec(),
                size: Counted::Range {
                    low: of.len(),
                    high: of.len() + others,
                },
                colours,
            }
        };
        if self.groups.is_empty() && self.unplaced == 1 && self.junk == 0 {
            let alone = self
                .sockets
                .iter()
                .position(|s| s.group.is_none())
                .unwrap_or(0);
            return vec![counts(&[alone], 0)];
        }
        let mut out: Vec<GroupCounts> = self
            .groups
            .iter()
            .map(|(_, members)| counts(members, may_join))
            .collect();
        if may_join > 0 {
            out.push(GroupCounts {
                members: Vec::new(),
                size: Counted::Range {
                    low: 1,
                    high: may_join,
                },
                colours: SOCKET_WORDS
                    .iter()
                    .map(|s| {
                        let high = self.unplaced_of(s.letter, &[]) + self.junk;
                        (s.letter.to_string(), Counted::Range { low: 0, high })
                    })
                    .collect(),
            });
        }
        out
    }

    /// One group's sockets as the game shows them: `R-R-G`.
    pub fn group_text(&self, members: &[usize]) -> String {
        members
            .iter()
            .map(|i| socket_text(&self.sockets[*i]))
            .collect::<Vec<_>>()
            .join("-")
    }

    /// The collection as the game shows it (the module doc, "The layout").
    pub fn layout(&self) -> String {
        let mut parts: Vec<String> = self
            .groups
            .iter()
            .map(|(_, members)| self.group_text(members))
            .collect();
        let unplaced: Vec<String> = self
            .sockets
            .iter()
            .filter(|s| s.group.is_none())
            .map(socket_text)
            .collect();
        if !unplaced.is_empty() {
            parts.push(format!("{} (group unread)", unplaced.join(" ")));
        }
        if self.junk > 0 {
            parts.push(format!(
                "{} {} unread",
                self.junk,
                if self.junk == 1 {
                    "element"
                } else {
                    "elements"
                }
            ));
        }
        parts.join(" ")
    }
}

fn socket_text(socket: &Socket) -> String {
    match (&socket.colour, socket.colour_unread, &socket.kind) {
        (Some(colour), _, _) => colour.clone(),
        (None, true, _) => "?".to_string(),
        (None, false, Some(kind)) => kind.clone(),
        (None, false, None) => "?".to_string(),
    }
}

/// The collection as the game shows it, for an item; none where the body
/// carries no `sockets`.
pub fn layout(item: &Item) -> Option<String> {
    groups(item).map(|g| g.layout())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_comparison_over_an_interval_is_decided_where_the_whole_interval_agrees() {
        let cmp = |op: Op, n: i64| NumTest::Cmp(op, n as f64);
        let two_to_four = Counted::Range { low: 2, high: 4 };
        assert_eq!(two_to_four.truth(&cmp(Op::Ge, 2)), Truth::True);
        assert_eq!(two_to_four.truth(&cmp(Op::Ge, 3)), Truth::Undecided);
        assert_eq!(two_to_four.truth(&cmp(Op::Ge, 5)), Truth::False);
        assert_eq!(two_to_four.truth(&cmp(Op::Le, 4)), Truth::True);
        assert_eq!(two_to_four.truth(&cmp(Op::Lt, 4)), Truth::Undecided);
        assert_eq!(two_to_four.truth(&cmp(Op::Lt, 2)), Truth::False);
        assert_eq!(two_to_four.truth(&cmp(Op::Eq, 3)), Truth::Undecided);
        assert_eq!(two_to_four.truth(&cmp(Op::Eq, 7)), Truth::False);
        assert_eq!(
            Counted::Range { low: 3, high: 3 }.truth(&cmp(Op::Eq, 3)),
            Truth::True
        );
        let range = |from: i64, to: i64| NumTest::Range(Some(from as f64), Some(to as f64));
        assert_eq!(two_to_four.truth(&range(1, 6)), Truth::True);
        assert_eq!(two_to_four.truth(&range(3, 6)), Truth::Undecided);
        assert_eq!(two_to_four.truth(&range(5, 6)), Truth::False);
        assert_eq!(Counted::Absent.truth(&cmp(Op::Le, 9)), Truth::False);
    }
}
