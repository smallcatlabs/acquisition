//! The words a frontend says this crate's documents in: an answer's, a
//! shown item's, a count's, a site search's and the language's index's
//! fragments, each a function of the document it reads (C53, text is a
//! function of the JSON) and the moment it is said at, made once here so
//! every door that links the crate says them alike (C46, shared semantics
//! in Rust). A fragment names no frontend's flag or printed command
//! (C143): a door assembles its own ways on around them.

/// `n` and its noun, `one` for exactly one and `many` otherwise: the
/// count words every text prints, from one copy.
pub fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}
