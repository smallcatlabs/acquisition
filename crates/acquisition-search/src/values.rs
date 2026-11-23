//! The values a query compares: what a rows view carries on each row
//! where it names none. They are columns, never an order: rows stand
//! as they lie in their place unless a sort is named (`corpus.rs`, "The
//! order").
//!
//! # As built
//!
//! - **A value is what a sort takes** (`bind::bind_sort`), found where
//!   the query compares one, in the order the query is written, each
//!   once by its canonical text (`print::print_value`):
//!   - `field op number` on a number field: the field;
//!   - a comparison on a computed value or a `sum( … )`: that value;
//!   - inside `line( … )`, each slot compared: `line(S).<slot>`, S the
//!     whole group's selector, its slot comparisons favoured away
//!     (`group::slots_compared`): the occurrences any of the group's
//!     comparisons could be on (C92), as a sort takes it, the largest.
//!     A comparison inside one alternative of the group's or is named
//!     by the whole group's selector too, so
//!     `line(("# to maximum Life" arg1>=50) or "#% to Cold Resistance")`
//!     compares `line("# to maximum Life" or "#% to Cold Resistance").arg1`,
//!     which reads an item's cold line though the comparison binds only
//!     its life;
//!   - nothing else: `linked( … )` has no value text that reaches it
//!     (C101, a missing reach is a field to add), `undecided( … )` asks
//!     whether a thing could be read and compares nothing, and `has:`,
//!     `is:`, text tests and constants compare no number.
//! - **A value is its text as spelled.** Two are one where their
//!   canonical texts are one, and a selector keeps the grouping, the
//!   order and the doubled nots its group was written with: two groups
//!   that ask one question spelled two ways are two values. So
//!   `line(source=explicit "# to maximum Life" arg1>=5) or
//!   line("# to maximum Life" arg1>=3 source=explicit)` compares two,
//!   two columns where the same query written in one order gives one
//!   (#294). A sort named beside the query's values joins them, and
//!   named values are kept once, by the same text.
//! - **Every branch contributes**: of an and, an or, a `holds`, a `not`.
//! - **What the query compares is a pure function of the canonical
//!   query and the build**: nothing of the corpus, the request's other
//!   fields or any answer before enters it.
//! - **The default list** is what a rows view carries where it names no
//!   values: what the query compares, then `stack` where some match
//!   carries a stack and the list has room for it (`answer.rs`,
//!   `BoundView::stacked`), so a currency tab shows its counts wherever
//!   it is searched. Whether one does is asked over every match, never
//!   over the rows a window shows, so a window's columns do not change as
//!   it scrolls. The list is then a function of the canonical query, the
//!   build and the matches: a continuation carries only what its request
//!   named, and re-derives the same list at the basis it names, which
//!   fixes the build and the facts the matches are drawn from (C98).

use crate::tree::{Collection, Node, Op, Value, ValueRef};

/// One value the query compares, by its canonical text.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Compared {
    pub text: String,
    pub value: ValueRef,
}

/// The values a query compares, in its order, each once.
pub(crate) fn compared(root: &Node) -> Vec<Compared> {
    let mut found: Vec<ValueRef> = Vec::new();
    node(root, &mut found);
    let mut out: Vec<Compared> = Vec::new();
    for value in found {
        let text = crate::print::print_value(&value);
        if out.iter().any(|c| c.text == text)
            || crate::tree::check_value(&value).is_err()
            || crate::bind::bind_sort(&value).is_err()
        {
            continue;
        }
        out.push(Compared { text, value });
    }
    out
}

pub(crate) fn compares_a_number(op: Op, value: &Value) -> bool {
    matches!(op, Op::Eq | Op::Gt | Op::Ge | Op::Lt | Op::Le)
        && matches!(value, Value::Number(_) | Value::Range { .. })
}

fn node(at: &Node, out: &mut Vec<ValueRef>) {
    match at {
        Node::All(children) | Node::Any(children) => {
            children.iter().for_each(|c| node(c, out));
        }
        Node::Holds { of, .. } => of.iter().for_each(|c| node(c, out)),
        Node::Not(inner) => node(inner, out),
        Node::Test { field, op, value } if compares_a_number(*op, value) => {
            out.push(ValueRef::Field(field.clone()));
        }
        Node::Compare { value, op, rhs } if compares_a_number(*op, rhs) => {
            out.push(value.clone());
        }
        Node::Members {
            of: Collection::Lines,
            where_,
        } => out.extend(crate::group::slots_compared(where_)),
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
