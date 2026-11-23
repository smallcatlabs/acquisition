//! `from`, the page a request left (the landing rule,
//! `roadmap/briefs/4-full-1-form-design.md` §5): a typed value, never a
//! string passed about. [`FromPage::here`] is its one maker, from a place's
//! route and its fields; [`taken`] its one reader, which takes it out of a
//! request's fields first, so no fold, verb or JSON answer meets it (as
//! `switched` takes the switch).
//!
//! A route is a place or a step. A place is a page a reader stands on (the
//! search page, `tabs`, `characters`, `saved`, `jobs`, an item's page): its
//! `from` is itself, its route and its fields less `from`, and a `from` its
//! URL carries is read only as its own way back. A step (`plan`,
//! `saved/open`, the clear's ask and every POST) hands on the `from` it was
//! given, and one reached with none has none. So `from` never nests, and no
//! way back leads to a step: a `from` that names a step, no place of this
//! server, or holds a token, a scheme or a `from` of its own is a usage
//! refusal (C47), never dropped. *Declined:* the browser's `Referer`, which
//! a browser may trim unseen by any test, while the landing page's URL must
//! carry its way back anyway (C16, a page is its URL).

use axum::extract::Query;
use axum::http::Uri;

use super::{Failure, Fields, Web};

/// The field every leaving form and every link to a step carries.
pub(crate) const FROM: &str = "from";

/// A place of this server: a page a reader stands on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Place {
    /// The empty search page, `.`.
    Home,
    Search,
    Tabs,
    Characters,
    Saved,
    Jobs,
    /// An item's page, by its id.
    Item(String),
}

impl Place {
    /// The route the place is served at, under the token; the home page's
    /// is the token's own path.
    fn route(&self) -> String {
        match self {
            Place::Home => String::new(),
            Place::Search => "search".to_string(),
            Place::Tabs => "tabs".to_string(),
            Place::Characters => "characters".to_string(),
            Place::Saved => "saved".to_string(),
            Place::Jobs => "jobs".to_string(),
            Place::Item(id) => format!("item/{}", super::enc(id)),
        }
    }

    /// The place as its way back names it.
    pub(crate) fn words(&self) -> &'static str {
        match self {
            Place::Home | Place::Search => "the search",
            Place::Tabs => "the stash tabs",
            Place::Characters => "the characters",
            Place::Saved => "the saved searches",
            Place::Jobs => "the jobs",
            Place::Item(_) => "the item",
        }
    }

    /// The place a route names, none where it names a step or nothing of
    /// this server.
    fn of(route: &str) -> Option<Place> {
        Some(match route {
            "." => Place::Home,
            "search" => Place::Search,
            "tabs" => Place::Tabs,
            "characters" => Place::Characters,
            "saved" => Place::Saved,
            "jobs" => Place::Jobs,
            _ => {
                let id = route.strip_prefix("item/")?;
                if id.is_empty() || id.contains('/') {
                    return None;
                }
                Place::Item(id.to_string())
            }
        })
    }

    /// The route as a `from` writes it: the home page's as `.`.
    fn word(&self) -> String {
        match self {
            Place::Home => ".".to_string(),
            place => place.route(),
        }
    }
}

/// A page a request left: a place of this server and its fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FromPage {
    place: Place,
    fields: Fields,
}

impl FromPage {
    /// The one maker: `place` as its request asked it, its fields less
    /// any `from` (a place's `from` is itself, never what it was reached
    /// from).
    pub(crate) fn here(place: Place, fields: &[(String, String)]) -> FromPage {
        FromPage {
            place,
            fields: fields.iter().filter(|(k, _)| k != FROM).cloned().collect(),
        }
    }

    pub(crate) fn place(&self) -> &Place {
        &self.place
    }

    /// Whether this is a page of `route`, whatever its fields.
    pub(crate) fn is_route(&self, route: &str) -> bool {
        self.place.route() == route
    }

    /// The value its hidden field and its link carry: the route and its
    /// query, no token (`tabs?realm=pc&league=Standard`).
    pub(crate) fn word(&self) -> String {
        let query: Vec<String> = self
            .fields
            .iter()
            .map(|(k, v)| format!("{}={}", super::enc(k), super::enc(v)))
            .collect();
        match query.is_empty() {
            true => self.place.word(),
            false => format!("{}?{}", self.place.word(), query.join("&")),
        }
    }

    /// The page itself, as a link under the token.
    pub(crate) fn href(&self, web: &Web) -> String {
        let fields: Vec<(&str, String)> = self
            .fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect();
        web.link(&self.place.route(), &fields)
    }

    /// The one reader: a `from` as a page sent it, or a usage refusal
    /// saying what it must be.
    fn read(word: &str) -> Result<FromPage, Failure> {
        let refused = |why: &str| {
            Failure::Usage(format!(
                "`from` {word:?} is no page to go back to: {why}; it names a page of this server \
                 by its route and fields, as a page writes it"
            ))
        };
        let (route, query) = match word.split_once('?') {
            Some((route, query)) => (route, query),
            None => (word, ""),
        };
        if route.contains(':') || route.starts_with('/') {
            return Err(refused(
                "a page is named by its route alone, no scheme, host or token",
            ));
        }
        let place = match Place::of(route) {
            Some(place) => place,
            None if STEPS.contains(&route) => {
                return Err(refused("a way back leads to a page, never to a step"));
            }
            None => return Err(refused("it names no page of this server")),
        };
        let uri: Uri = format!("/?{query}")
            .parse()
            .map_err(|_| refused("its fields are no query"))?;
        let Query(fields) =
            Query::<Fields>::try_from_uri(&uri).map_err(|_| refused("its fields are no query"))?;
        if fields.iter().any(|(k, _)| k == FROM) {
            return Err(refused("a way back holds no way back of its own"));
        }
        Ok(FromPage { place, fields })
    }
}

/// The routes that are steps: a `from` naming one is refused by its own
/// words.
const STEPS: [&str; 5] = ["plan", "apply", "leagues", "saved/open", "saved/clear"];

/// The one reader, met first by every place and every step: `from` taken
/// out of a request's fields and read, at most one, or a usage refusal.
pub(crate) fn taken(fields: Fields) -> Result<(Fields, Option<FromPage>), Failure> {
    let (from, fields): (Fields, Fields) = fields.into_iter().partition(|(k, _)| k == FROM);
    match from.as_slice() {
        [] => Ok((fields, None)),
        [(_, word)] => Ok((fields, Some(FromPage::read(word)?))),
        _ => Err(Failure::Usage(
            "a request names one page it came from, `from` once".to_string(),
        )),
    }
}

/// `link`, a link to a step, carrying `from` where there is one: the one
/// maker of such a link's `from`, as [`super::page::post_form`] is of a
/// form's.
pub(crate) fn onward(link: &str, from: Option<&FromPage>) -> String {
    match from {
        Some(from) => {
            let joint = if link.contains('?') { '&' } else { '?' };
            format!("{link}{joint}{FROM}={}", super::enc(&from.word()))
        }
        None => link.to_string(),
    }
}

/// What a request carries of the page it left, read by [`taken`]: the
/// `from` its forms and its links to a step hand on (`onward`), and the way
/// back its page draws (`back`). A place hands on itself and draws the
/// `from` its URL carries; a step hands on and draws the one it was given.
#[derive(Clone, Debug, Default)]
pub(crate) struct Here {
    pub onward: Option<FromPage>,
    pub back: Option<FromPage>,
}

impl Here {
    /// A request read nothing of yet (its `from` itself refused).
    pub(crate) fn none() -> Here {
        Here::default()
    }

    /// A place: itself onward, the `from` its URL carried as its way
    /// back.
    pub(crate) fn place(itself: FromPage, back: Option<FromPage>) -> Here {
        Here {
            onward: Some(itself),
            back,
        }
    }

    /// A step: the `from` it was given, onward and back alike.
    pub(crate) fn step(given: Option<FromPage>) -> Here {
        Here {
            onward: given.clone(),
            back: given,
        }
    }

    pub(crate) fn onward(&self) -> Option<&FromPage> {
        self.onward.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(word: &str) -> Result<FromPage, String> {
        FromPage::read(word).map_err(|f| match f {
            Failure::Usage(words) => words,
            _ => "not a usage refusal".to_string(),
        })
    }

    #[test]
    fn a_from_reads_back_as_the_place_its_maker_wrote() {
        let fields: Fields = vec![
            ("realm".into(), "pc".into()),
            ("league".into(), "Mercenaries & Co".into()),
            ("tab".into(), "a1".into()),
            ("from".into(), "jobs".into()),
        ];
        for place in [
            Place::Home,
            Place::Search,
            Place::Tabs,
            Place::Characters,
            Place::Saved,
            Place::Jobs,
            Place::Item("ab12".into()),
        ] {
            let made = FromPage::here(place.clone(), &fields);
            assert!(!made.word().contains("from="), "{}", made.word());
            assert_eq!(read(&made.word()), Ok(made), "{place:?}");
        }
    }

    #[test]
    fn a_from_naming_a_step_no_place_a_token_a_url_or_a_from_is_refused_as_usage() {
        for (word, why) in [
            ("plan?realm=pc", "never to a step"),
            ("saved/clear?id=0000000000a1", "never to a step"),
            ("saved/open?id=0000000000a1", "never to a step"),
            ("apply", "never to a step"),
            ("leagues", "never to a step"),
            ("", "no page of this server"),
            ("nowhere", "no page of this server"),
            ("item/a/b", "no page of this server"),
            ("abc123token/search", "no page of this server"),
            ("/abc123token/search", "no scheme, host or token"),
            ("https://example.com/search", "no scheme, host or token"),
            ("//example.com/search", "no scheme, host or token"),
            ("jobs?from=tabs", "no way back of its own"),
        ] {
            let words = read(word).expect_err(word);
            assert!(words.contains(why), "{word}: {words}");
        }
        let twice: Fields = vec![
            ("from".into(), "jobs".into()),
            ("from".into(), "saved".into()),
        ];
        assert!(matches!(taken(twice), Err(Failure::Usage(_))));
    }
}
