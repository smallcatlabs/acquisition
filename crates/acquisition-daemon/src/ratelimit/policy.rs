//! The header contract (N5): GGG's `X-Rate-Limit-*` and `Retry-After`
//! headers read into policies and holds, the caps on what they may say, and
//! the header snapshots kept for logs, job payloads and the dashboard.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

/// Product policy: longer server-requested pauses are contained as terminal
/// failures rather than silently parking work for an unobserved duration.
pub const RETRY_AFTER_CAP_SECS: u64 = 900;
/// Operational ceiling for any policy/state window period. The largest
/// observed period is 1,800s (N23); a full day leaves ample dynamic-policy
/// headroom while containing values that cannot represent a useful daemon
/// pacing decision (D8/N9).
pub const MAX_POLICY_PERIOD_SECS: u64 = 24 * 60 * 60;
/// Operational ceiling for declared and active restriction durations. The
/// largest observed restriction is 600s (N23); as with periods, a full day
/// contains unexpected input without narrowing any known policy.
pub const MAX_POLICY_RESTRICTION_SECS: u64 = 24 * 60 * 60;

// ---- the header contract (N5) ---------------------------------------------

/// One `hits:period:restriction` triplet from `X-Rate-Limit-<rule>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Window {
    pub max_hits: u32,
    pub period_secs: u64,
    pub restriction_secs: u64,
}

/// One `current-hits:period:restricted-for` triplet from
/// `X-Rate-Limit-<rule>-State`. Post-increment: includes the request that
/// carried it (N25).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowState {
    pub hits: u32,
    pub period_secs: u64,
    pub restricted_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub limits: Vec<Window>,
    pub state: Vec<WindowState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub rules: Vec<Rule>,
}

impl Policy {
    /// Parse one Full N5 header set. Every malformed or partial input is a
    /// value-level error: callers never receive a policy that can only pace
    /// part of the server's counter shape (D8/N20).
    pub fn parse(get: impl Fn(&str) -> Option<String>) -> Result<Policy, PolicyParseError> {
        let name = required(&get, "x-rate-limit-policy")?;
        if name.trim().is_empty() {
            return Err(PolicyParseError::EmptyPolicyName);
        }
        let names = required(&get, "x-rate-limit-rules")?;
        if names.trim().is_empty() {
            return Err(PolicyParseError::EmptyRules);
        }
        let mut rules = Vec::new();
        for raw_rule in names.split(',') {
            let rule = raw_rule.trim();
            if rule.is_empty() {
                return Err(PolicyParseError::EmptyRuleName);
            }
            let key = rule.to_ascii_lowercase();
            let limit_header = format!("x-rate-limit-{key}");
            let state_header = format!("x-rate-limit-{key}-state");
            let limits = parse_limits(&limit_header, &required(&get, &limit_header)?)?;
            let state = parse_state(&state_header, &required(&get, &state_header)?)?;
            if limits.len() != state.len() {
                return Err(PolicyParseError::WindowCountMismatch {
                    rule: rule.to_string(),
                    limits: limits.len(),
                    state: state.len(),
                });
            }
            for (index, (limit, status)) in limits.iter().zip(&state).enumerate() {
                if limit.period_secs != status.period_secs {
                    return Err(PolicyParseError::PeriodMismatch {
                        rule: rule.to_string(),
                        index,
                        limit: limit.period_secs,
                        state: status.period_secs,
                    });
                }
            }
            rules.push(Rule {
                name: rule.to_string(),
                limits,
                state,
            });
        }
        if rules.is_empty() {
            return Err(PolicyParseError::EmptyRules);
        }
        Ok(Policy {
            name: name.trim().to_string(),
            rules,
        })
    }

    pub fn from_headers(headers: &reqwest::header::HeaderMap) -> Result<Policy, PolicyParseError> {
        Policy::parse(|name| {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_string)
        })
    }

    /// Ordered rule names and ordered window periods identify which counter
    /// set a policy's local history describes (F65). Limit values may change
    /// dynamically within that shape without invalidating the history.
    pub(super) fn has_same_shape(&self, other: &Policy) -> bool {
        self.rules.len() == other.rules.len()
            && self.rules.iter().zip(&other.rules).all(|(left, right)| {
                left.name == right.name
                    && left
                        .limits
                        .iter()
                        .map(|w| w.period_secs)
                        .eq(right.limits.iter().map(|w| w.period_secs))
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyParseError {
    MissingHeader {
        name: String,
    },
    EmptyPolicyName,
    EmptyRules,
    EmptyRuleName,
    MalformedTriplet {
        header: String,
        raw: String,
    },
    OutOfRangeTriplet {
        header: String,
        raw: String,
    },
    WindowCountMismatch {
        rule: String,
        limits: usize,
        state: usize,
    },
    PeriodMismatch {
        rule: String,
        index: usize,
        limit: u64,
        state: u64,
    },
}

/// #113, a log line's words are values the daemon chose: a rule GGG
/// names, as one of its documented scopes, else "a rule outside GGG's".
/// The limiter keys on the name it accepted; the words never carry it.
pub fn rule_label(rule: &str) -> &'static str {
    match rule.trim() {
        "Ip" => "Ip",
        "Account" => "Account",
        "Client" => "Client",
        _ => "a rule outside GGG's",
    }
}

/// #113: a rate-limit header's name as words: the two fixed ones as they
/// are, a rule's limits or state header by the rule's label, since its
/// name embeds the rule GGG named.
fn header_label(header: &str) -> String {
    match header {
        "x-rate-limit-policy" | "x-rate-limit-rules" => header.to_string(),
        _ => {
            let rule = header.strip_prefix("x-rate-limit-").unwrap_or(header);
            match rule.strip_suffix("-state") {
                Some(rule) => format!("the state header of rule {}", rule_label(&capitalise(rule))),
                None => format!(
                    "the limits header of rule {}",
                    rule_label(&capitalise(rule))
                ),
            }
        }
    }
}

/// A lowercased header key's rule back in GGG's spelling (`account` →
/// `Account`), for [`rule_label`].
fn capitalise(rule: &str) -> String {
    let mut chars = rule.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

impl fmt::Display for PolicyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolicyParseError::MissingHeader { name } => write!(f, "missing {}", header_label(name)),
            PolicyParseError::EmptyPolicyName => write!(f, "empty X-Rate-Limit-Policy"),
            PolicyParseError::EmptyRules => write!(f, "empty X-Rate-Limit-Rules"),
            PolicyParseError::EmptyRuleName => write!(f, "empty rule name"),
            // #113: the triplet's text is not quoted; a log line carries
            // only what the daemon parsed.
            PolicyParseError::MalformedTriplet { header, .. } => {
                write!(f, "malformed triplet in {}", header_label(header))
            }
            PolicyParseError::OutOfRangeTriplet { header, .. } => {
                write!(f, "out-of-range triplet in {}", header_label(header))
            }
            PolicyParseError::WindowCountMismatch {
                rule,
                limits,
                state,
            } => write!(
                f,
                "rule {} has {limits} limit windows but {state} state windows",
                rule_label(rule)
            ),
            PolicyParseError::PeriodMismatch {
                rule,
                index,
                limit,
                state,
            } => write!(
                f,
                "rule {} window {index} has limit period {limit} but state period {state}",
                rule_label(rule)
            ),
        }
    }
}

impl std::error::Error for PolicyParseError {}

fn required(get: &impl Fn(&str) -> Option<String>, name: &str) -> Result<String, PolicyParseError> {
    get(name).ok_or_else(|| PolicyParseError::MissingHeader {
        name: name.to_string(),
    })
}

fn parse_integer(raw: &str) -> Option<u64> {
    if raw.is_empty() || !raw.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    raw.parse().ok()
}

/// `Retry-After` is retry authority, not part of the Full policy grammar.
/// Keeping this result total and separate lets a malformed policy still
/// install the conservative hold required for a retryable 429 (D8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryAfter {
    Acceptable { seconds: u64 },
    Missing,
    Malformed { raw: String },
    Negative { raw: String },
    OverCap { raw: String },
}

impl RetryAfter {
    pub fn seconds(&self) -> Option<u64> {
        match self {
            RetryAfter::Acceptable { seconds } => Some(*seconds),
            RetryAfter::Missing
            | RetryAfter::Malformed { .. }
            | RetryAfter::Negative { .. }
            | RetryAfter::OverCap { .. } => None,
        }
    }

    /// The number of seconds the header itself carried, for the status
    /// (invariant 2: report what the server said, never the hold's own
    /// arithmetic). An over-cap value is reported as sent; a missing,
    /// malformed or negative one, or one too large to count, carried none.
    pub fn carried_secs(&self) -> Option<u64> {
        match self {
            RetryAfter::Acceptable { seconds } => Some(*seconds),
            RetryAfter::OverCap { raw } => raw.trim().parse().ok(),
            RetryAfter::Missing | RetryAfter::Malformed { .. } | RetryAfter::Negative { .. } => {
                None
            }
        }
    }

    pub fn is_acceptable(&self) -> bool {
        matches!(self, RetryAfter::Acceptable { .. })
    }
}

impl fmt::Display for RetryAfter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RetryAfter::Acceptable { seconds } => write!(f, "{seconds}s"),
            RetryAfter::Missing => write!(f, "missing Retry-After"),
            // #113: a log line never carries the header's text; a
            // malformed, negative or over-cap value is named, not quoted.
            // An over-cap value is all digits but need not fit a number
            // (one past `u64` never parses); `carried_secs` reads it for
            // the status alone.
            RetryAfter::Malformed { .. } => write!(f, "malformed Retry-After"),
            RetryAfter::Negative { .. } => write!(f, "negative Retry-After"),
            RetryAfter::OverCap { .. } => write!(
                f,
                "a Retry-After over the {RETRY_AFTER_CAP_SECS}s product cap"
            ),
        }
    }
}

pub fn parse_retry_after(get: impl Fn(&str) -> Option<String>) -> RetryAfter {
    let Some(raw) = get("retry-after") else {
        return RetryAfter::Missing;
    };
    let value = raw.trim();
    if value.starts_with('-') {
        return RetryAfter::Negative { raw };
    }
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return RetryAfter::Malformed { raw };
    }
    match value.parse::<u64>() {
        Ok(seconds) if seconds <= RETRY_AFTER_CAP_SECS => RetryAfter::Acceptable { seconds },
        Ok(_) | Err(_) => RetryAfter::OverCap { raw },
    }
}

pub fn retry_after_from_headers(headers: &reqwest::header::HeaderMap) -> RetryAfter {
    parse_retry_after(|name| {
        headers.get(name).map(|value| {
            value
                .to_str()
                .map(str::to_string)
                .unwrap_or_else(|_| "<non-utf8>".into())
        })
    })
}

fn parse_triplet(header: &str, raw: &str) -> Result<[u64; 3], PolicyParseError> {
    let mut fields = raw.trim().split(':');
    let parsed = [fields.next(), fields.next(), fields.next()];
    if fields.next().is_some() || parsed.iter().any(Option::is_none) {
        return Err(PolicyParseError::MalformedTriplet {
            header: header.to_string(),
            raw: raw.to_string(),
        });
    }
    let parsed = parsed.map(|field| field.and_then(|value| parse_integer(value.trim())));
    let [Some(first), Some(second), Some(third)] = parsed else {
        return Err(PolicyParseError::MalformedTriplet {
            header: header.to_string(),
            raw: raw.to_string(),
        });
    };
    Ok([first, second, third])
}

fn parse_limits(header: &str, raw: &str) -> Result<Vec<Window>, PolicyParseError> {
    raw.split(',')
        .map(|triplet| {
            let [max_hits, period_secs, restriction_secs] = parse_triplet(header, triplet)?;
            let max_hits =
                u32::try_from(max_hits).map_err(|_| PolicyParseError::OutOfRangeTriplet {
                    header: header.to_string(),
                    raw: triplet.to_string(),
                })?;
            if max_hits == 0
                || period_secs == 0
                || period_secs > MAX_POLICY_PERIOD_SECS
                || restriction_secs > MAX_POLICY_RESTRICTION_SECS
            {
                return Err(PolicyParseError::OutOfRangeTriplet {
                    header: header.to_string(),
                    raw: triplet.to_string(),
                });
            }
            Ok(Window {
                max_hits,
                period_secs,
                restriction_secs,
            })
        })
        .collect()
}

fn parse_state(header: &str, raw: &str) -> Result<Vec<WindowState>, PolicyParseError> {
    raw.split(',')
        .map(|triplet| {
            let [hits, period_secs, restricted_secs] = parse_triplet(header, triplet)?;
            let hits = u32::try_from(hits).map_err(|_| PolicyParseError::OutOfRangeTriplet {
                header: header.to_string(),
                raw: triplet.to_string(),
            })?;
            if period_secs == 0
                || period_secs > MAX_POLICY_PERIOD_SECS
                || restricted_secs > MAX_POLICY_RESTRICTION_SECS
            {
                return Err(PolicyParseError::OutOfRangeTriplet {
                    header: header.to_string(),
                    raw: triplet.to_string(),
                });
            }
            Ok(WindowState {
                hits,
                period_secs,
                restricted_secs,
            })
        })
        .collect()
}

/// The response headers worth keeping from a non-2xx: everything the rate
/// snapshot keeps plus the ones that say *who* answered (Cloudflare or the
/// origin) and *what* it sent. A HEAD has no body, so for a probe these
/// headers are the only evidence a failure leaves. Never `set-cookie`.
/// A 3xx's `location` is kept with its query and fragment reduced (C130).
pub fn response_headers_snapshot(headers: &reqwest::header::HeaderMap) -> serde_json::Value {
    const KEEP: &[&str] = &[
        "location",
        "retry-after",
        "cf-ray",
        "cf-cache-status",
        "cf-mitigated",
        "content-type",
        "content-length",
        "server",
        "date",
        "www-authenticate",
        "x-request-id",
    ];
    let mut map = serde_json::Map::new();
    for (name, value) in headers {
        let name = name.as_str();
        if name.starts_with("x-rate-limit") || KEEP.contains(&name) {
            let value = match value.to_str() {
                Err(_) => "<non-utf8>".to_string(),
                Ok(value) if name == "location" => redacted_location(value),
                Ok(value) => value.to_string(),
            };
            map.insert(name.into(), value.into());
        }
    }
    serde_json::Value::Object(map)
}

/// The longest origin and path a redacted `Location` keeps, in
/// characters, before it is cut with a mark. A host is at most 253
/// characters, and a login or challenge page's origin and path fits well
/// inside 512 with room for a long path; past that, more of it says
/// nothing a reader needs, and the journal line and the failure text stay
/// bounded whatever a server sends.
pub(crate) const LOCATION_CHARS: usize = 512;

/// A `Location` as the daemon may record or report it (C130): the target
/// is shown only redacted, wherever it lands — the journal's header
/// snapshot, a job's failure, a token request's refusal. Every form is
/// resolved the way a client would resolve it, against an `https` base,
/// so a scheme-relative `//host/…` and an absolute URL meet the same
/// parser. What is kept is the origin (scheme, host, port) and the path,
/// cut at [`LOCATION_CHARS`]; user information is dropped, and a query or
/// a fragment, where a code or a token can ride, is replaced by a mark
/// that says one was there. A scheme with no host (`data:`, `mailto:`,
/// `urn:`) carries its content where a host would be, so it is shown as
/// the scheme and a mark. A scheme-relative reference keeps its leading
/// `//`, a path-relative one is the path it resolves to from the root,
/// and one the parser refuses is a mark alone.
pub(crate) fn redacted_location(location: &str) -> String {
    const BASES: [&str; 2] = ["https://a.invalid/", "https://b.invalid/"];
    let resolved = BASES.map(|base| {
        url::Url::parse(base)
            .and_then(|base| base.join(location))
            .ok()
    });
    let [Some(url), Some(other)] = resolved else {
        return "<unreadable location>".into();
    };
    // Against two bases a relative reference takes each base's host; one
    // that names its own host (absolute or scheme-relative) keeps it.
    let names_host = url.host_str() == other.host_str();
    let absolute = url::Url::parse(location).is_ok();
    let origin = match (url.host_str(), names_host) {
        (None, _) => return format!("{}:…", url.scheme()),
        (Some(host), true) => {
            let port = url
                .port()
                .map(|port| format!(":{port}"))
                .unwrap_or_default();
            let scheme = if absolute {
                format!("{}:", url.scheme())
            } else {
                String::new()
            };
            format!("{scheme}//{host}{port}")
        }
        (Some(_), false) => String::new(),
    };
    let mut kept = format!("{origin}{}", url.path());
    if kept.chars().count() > LOCATION_CHARS {
        kept = kept.chars().take(LOCATION_CHARS).collect();
        kept.push('…');
    }
    if url.query().is_some() {
        kept.push_str("?…");
    }
    if url.fragment().is_some() {
        kept.push_str("#…");
    }
    kept
}

/// What stands for a 3xx's body wherever a caller would show one (C130):
/// where it pointed, redacted. The body itself is never kept.
pub(crate) fn redirect_evidence(headers: &reqwest::header::HeaderMap) -> String {
    let target = match headers.get(reqwest::header::LOCATION).map(|v| v.to_str()) {
        Some(Ok(location)) => redacted_location(location),
        Some(Err(_)) => "<non-utf8>".into(),
        None => "no location".into(),
    };
    format!("<a redirect's body is not kept; it pointed to {target}>")
}

/// #113, a log line's words are values the daemon chose or parsed into
/// its own types, never upstream text judged safe by its shape: the rate
/// line a log or a refusal carries, rendered from `rate`, a
/// [`rate_limit_snapshot`], through [`Policy::parse`] and
/// [`parse_retry_after`]. Each rule by its [`rule_label`], each window's
/// numbers, and the `Retry-After` the parse read; never the policy's
/// name, which a line names by the route the daemon keys it on; headers
/// that do not parse are "unreadable rate headers" and never their text;
/// none at all is "no rate headers". The snapshot itself, raw, stays the
/// journal's and a job's payload (GGG's contract evidence).
pub fn rate_words(rate: &serde_json::Value) -> String {
    let get = |name: &str| rate.get(name).and_then(|v| v.as_str()).map(str::to_string);
    let has_rate = rate
        .as_object()
        .is_some_and(|map| map.keys().any(|name| name.starts_with("x-rate-limit")));
    let mut words = if !has_rate {
        "no rate headers".to_string()
    } else {
        match Policy::parse(get) {
            Ok(policy) => {
                let rules: Vec<String> = policy
                    .rules
                    .iter()
                    .map(|rule| {
                        let windows: Vec<String> = rule
                            .limits
                            .iter()
                            .zip(&rule.state)
                            .map(|(limit, state)| {
                                let held = if state.restricted_secs > 0 {
                                    format!(", restricted {}s", state.restricted_secs)
                                } else {
                                    String::new()
                                };
                                format!(
                                    "{}/{} per {}s{held}",
                                    state.hits, limit.max_hits, limit.period_secs
                                )
                            })
                            .collect();
                        format!("{} {}", rule_label(&rule.name), windows.join(", "))
                    })
                    .collect();
                format!("rate {}", rules.join("; "))
            }
            Err(_) => "unreadable rate headers".to_string(),
        }
    };
    let retry_after = parse_retry_after(get);
    if !matches!(retry_after, RetryAfter::Missing) {
        words.push_str(&format!("; Retry-After {retry_after}"));
    }
    words
}

/// The `X-Rate-Limit-*` (+ `Retry-After`) headers as a JSON object, raw:
/// for the journal, job payloads and the dashboard. A log line renders
/// it through [`rate_words`].
pub fn rate_limit_snapshot(headers: &reqwest::header::HeaderMap) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (name, value) in headers {
        let name = name.as_str();
        if name.starts_with("x-rate-limit") || name == "retry-after" {
            map.insert(name.into(), value.to_str().unwrap_or("<non-utf8>").into());
        }
    }
    serde_json::Value::Object(map)
}
