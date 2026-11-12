//! The values a query compares (4-full part 1, step 6.1;
//! `roadmap/briefs/4-full-1-step6-values.md`): what a rows view carries
//! on each row where it names none, and the one value that orders it
//! where it names no sort (#38).
//!
//! # As built
//!
//! - **A value is what a sort takes** (`bind::bind_sort`), found where
//!   the query compares one, in the order the query is written, each
//!   once by its canonical text (`print::print_value`):
//!   - `field op number` on a number field: the field;
//!   - a comparison on a computed value or a `sum( … )`: that value;
//!   - inside `line( … )`, each slot compared: `line(S).<slot>`, S the
//!     group's selector, its slot comparisons favoured away
//!     (`group::slots_compared`), so the value is the occurrence the
//!     comparison was on (C92), as a sort takes it, the largest;
//!   - nothing else: `linked( … )` has no value text that reaches it
//!     (C101, a missing reach is a field to add), `undecided( … )` asks
//!     whether a thing could be read and compares nothing, and `has:`,
//!     `is:`, text tests and constants compare no number.
//! - **Every branch contributes**: of an and, an or, a `holds`, a `not`.
//! - **A value's direction is best first by its operator**: `>` and `>=`
//!   largest first, `<` and `<=` smallest first. `=`, a value under a
//!   `not` (an item's or a group's) or under a `holds` with an upper
//!   bound, and a value compared both ways (a range) have none: a flipped
//!   direction would be a rule to learn to predict an order, and a column
//!   is sorted by asking. Two nots cancel, as they do in meaning, so a
//!   rewrite that cancels them changes no order (invariant 7).
//! - **The default order** is the one value's, where the query compares
//!   exactly one and it has a direction; several, or one with none, give
//!   the store's order.
//! - **Both are a pure function of the canonical query and the build**:
//!   nothing of the corpus, the request's other fields or any answer
//!   before enters them. So a continuation carries only what its request
//!   named, and the same continuation re-derives the same list and the
//!   same order at the basis it names, whose build is part of it (C98).

use crate::tree::{Collection, Node, Op, Value, ValueRef};

/// Which end of a value comes first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Largest,
    Smallest,
}

/// One value the query compares, by its canonical text, with its
/// direction where it has exactly one.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Compared {
    pub text: String,
    pub value: ValueRef,
    pub direction: Option<Direction>,
}

/// The values a query compares, in its order, each once.
pub(crate) fn compared(root: &Node) -> Vec<Compared> {
    let mut found: Vec<(ValueRef, Option<Direction>)> = Vec::new();
    node(
        root,
        Under {
            negated: false,
            capped: false,
        },
        &mut found,
    );
    let mut out: Vec<Compared> = Vec::new();
    for (value, direction) in found {
        let text = crate::print::print_value(&value);
        match out.iter_mut().find(|c| c.text == text) {
            Some(seen) => {
                if seen.direction != direction {
                    seen.direction = None;
                }
            }
            None => {
                if crate::tree::check_value(&value).is_err()
                    || crate::bind::bind_sort(&value).is_err()
                {
                    continue;
                }
                out.push(Compared {
                    text,
                    value,
                    direction,
                });
            }
        }
    }
    out
}

/// The order a query gives where no sort was asked: its one value, where
/// it compares exactly one and that value has a direction.
pub(crate) fn default_order(compared: &[Compared]) -> Option<(&Compared, Direction)> {
    match compared {
        [one] => one.direction.map(|d| (one, d)),
        _ => None,
    }
}

fn direction(op: Op, plain: bool) -> Option<Direction> {
    match (op, plain) {
        (Op::Gt | Op::Ge, true) => Some(Direction::Largest),
        (Op::Lt | Op::Le, true) => Some(Direction::Smallest),
        _ => None,
    }
}

pub(crate) fn compares_a_number(op: Op, value: &Value) -> bool {
    matches!(op, Op::Eq | Op::Gt | Op::Ge | Op::Lt | Op::Le)
        && matches!(value, Value::Number(_) | Value::Range { .. })
}

/// Where a comparison stands: under an odd number of nots, or under a
/// `holds` with an upper bound, no direction is the best one.
#[derive(Clone, Copy)]
struct Under {
    negated: bool,
    capped: bool,
}

impl Under {
    fn plain(self) -> bool {
        !self.negated && !self.capped
    }

    fn not(self) -> Under {
        Under {
            negated: !self.negated,
            ..self
        }
    }
}

fn node(at: &Node, under: Under, out: &mut Vec<(ValueRef, Option<Direction>)>) {
    match at {
        Node::All(children) | Node::Any(children) => {
            children.iter().for_each(|c| node(c, under, out));
        }
        Node::Holds { of, max, .. } => {
            let under = Under {
                capped: under.capped || max.is_some(),
                ..under
            };
            of.iter().for_each(|c| node(c, under, out));
        }
        Node::Not(inner) => node(inner, under.not(), out),
        Node::Test { field, op, value } if compares_a_number(*op, value) => {
            out.push((
                ValueRef::Field(field.clone()),
                direction(*op, under.plain()),
            ));
        }
        Node::Compare { value, op, rhs } if compares_a_number(*op, rhs) => {
            out.push((value.clone(), direction(*op, under.plain())));
        }
        Node::Members {
            of: Collection::Lines,
            where_,
        } => {
            for (value, op, negated) in crate::group::slots_compared(where_) {
                let under = match negated {
                    true => under.not(),
                    false => under,
                };
                out.push((value, direction(op, under.plain())));
            }
        }
        Node::Members {
            of: Collection::Links,
            ..
        }
        | Node::Undecided(_)
        | Node::Const(_)
        | Node::Test { .. }
        | Node::Compare { .. }
        | Node::Has(_)
        | Node::Is(_) => {}
    }
}
