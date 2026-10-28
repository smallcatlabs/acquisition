//! C53 (ages are text, epochs are JSON, and an age the text prints stands
//! in words beside its epoch in the JSON): the one vocabulary every age
//! is said in, on every surface, held once beside [`crate::now`].
//!
//! One table, [`span`]: `45s`, `12m`, `5h`, `9d`, the unit changing at
//! 90 seconds, 90 minutes and 48 hours. Its readings: [`ago`], an epoch's
//! age at a moment (`6d ago`, or `never` for an epoch that is absent),
//! which the CLI's text prints and the JSON carries; and [`before`], an
//! epoch's age at another epoch it is measured from (`3h before
//! generated_at`), for a document whose text reads an age against its
//! own time rather than the answer's. The CLI's `ago` and `dur` and the
//! verbs' ages call these, so a golden that changes changes once.
//!
//! The words stand in a document under a sibling key named for the epoch
//! they are made from, the epoch's own name and `_age` (`fetched_at` and
//! `fetched_at_age`), made at the answer's own time: the epoch is the
//! fact a program keeps, the words what the answer says of it when it
//! was made (step 2g, round 6).

/// What an age's words are, said once for every surface that glosses a
/// document's keys (C146: the MCP's `describe` entry `ages`).
pub const SAID: &str = "beside a time (seconds since 1970, UTC) that the text says as an age, the age stands in words under the time's own name and `_age` (`fetched_at_age` beside `fetched_at`): `45s ago`, `12m ago`, `5h ago` or `9d ago` — the unit changes at 90 seconds, 90 minutes and 48 hours, rounded down — or `never` where the time is null. The words are made when the answer was, so they are that answer's and go stale with it; the time is the fact. Where the text measures an age from a time of the document's own instead of the answer's, the words name that time: a plan's listing ages read `3h before generated_at`";

/// A span of seconds in the vocabulary: `45s` under 90 seconds, `12m`
/// under 90 minutes, `5h` under 48 hours, else `9d`; whole units,
/// rounded down, a negative span read as none.
pub fn span(seconds: i64) -> String {
    let s = seconds.max(0);
    if s < 90 {
        format!("{s}s")
    } else if s < 5400 {
        format!("{}m", s / 60)
    } else if s < 172_800 {
        format!("{}h", s / 3600)
    } else {
        format!("{}d", s / 86400)
    }
}

/// An epoch's age at `now`: `{span} ago`, or `never` where there is no
/// epoch. An epoch after `now` (a clock set back) reads `0s ago`.
pub fn ago(now: i64, at: Option<i64>) -> String {
    match at {
        None => "never".into(),
        Some(at) => format!("{} ago", span(now - at)),
    }
}

/// An epoch's age at another epoch of the same document, named by its
/// field: `{span} before {basis}`, or `never` where there is no epoch.
pub fn before(basis: &str, basis_at: i64, at: Option<i64>) -> String {
    match at {
        None => "never".into(),
        Some(at) => format!("{} before {basis}", span(basis_at - at)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vocabulary the CLI printed before step 2g's round 6 held it
    /// here, boundary by boundary: the unit changes at 90 seconds, 90
    /// minutes and 48 hours, rounding down.
    #[test]
    fn c53_an_age_is_said_in_one_vocabulary() {
        for (seconds, said) in [
            (0, "0s"),
            (45, "45s"),
            (89, "89s"),
            (90, "1m"),
            (5399, "89m"),
            (5400, "1h"),
            (172_799, "47h"),
            (172_800, "2d"),
            (9 * 86_400 + 5, "9d"),
            (-30, "0s"),
        ] {
            assert_eq!(span(seconds), said, "{seconds} seconds");
        }
    }

    /// The two readings of the one table: an epoch's age at the answer's
    /// time, `never` for none, and its age at another epoch, named.
    #[test]
    fn c53_an_epoch_reads_as_its_age_at_a_moment_or_never() {
        let now = 1_000_000;
        assert_eq!(ago(now, Some(now - 6 * 86_400)), "6d ago");
        assert_eq!(ago(now, Some(now - 45)), "45s ago");
        assert_eq!(ago(now, None), "never");
        assert_eq!(ago(now, Some(now + 60)), "0s ago", "a clock set back");
        assert_eq!(
            before("generated_at", now, Some(now - 3 * 3600)),
            "3h before generated_at"
        );
        assert_eq!(before("generated_at", now, None), "never");
    }
}
