//! The terminal as a surface: how a failure reaches the person at it —
//! the words of a fault, the accounts and ways offered after it, the
//! `--json` failure document, a usage refusal — and the printed commands
//! those name, quoted for the shell (C156) and rebuilt from the command
//! line as parsed.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use crate::printed::Printed as _;

use acquisition_verbs::Fault;
use serde_json::json;

use crate::cli::Cli;
use crate::daemon_cmd::mismatch_remedy;
use crate::{plan_cmd, saved_cmd, search_cmd};

/// `n` and its noun, `one` for exactly one and `many` otherwise: the
/// count words every text of this crate prints (#91: five copies).
pub(crate) fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// The failure was already printed in the command's own output format (a
/// failed job's `--json` outcome); main only sets the exit code.
#[derive(Debug)]
pub(crate) struct AlreadyReported;

impl std::fmt::Display for AlreadyReported {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("already reported")
    }
}

impl std::error::Error for AlreadyReported {}

/// A command line the parser refuses, under `--json`: the same one
/// document on stdout as any other failure (C11), `kind: "usage"`, with
/// the parser's own exit 2 — the command never ran. Help and version are
/// not failures and print as they always do; so does a refusal without
/// `--json`. `--json` is read from the raw arguments, since nothing was
/// parsed: any argument spelled so before a bare `--`.
pub(crate) fn refuse_usage(e: clap::Error) -> ! {
    use clap::error::ErrorKind as Parse;
    let asked_json = std::env::args()
        .skip(1)
        .take_while(|a| a != "--")
        .any(|a| a == "--json");
    let informative = matches!(
        e.kind(),
        Parse::DisplayHelp
            | Parse::DisplayVersion
            | Parse::DisplayHelpOnMissingArgumentOrSubcommand
    );
    if informative {
        e.exit();
    }
    if !asked_json {
        // the parser's refusal, then what a pasted command met (C156): a
        // cmd.exe paste most often fails here, its quoted words split
        let _ = e.print();
        if let Some(hint) = pasted_shell_hint() {
            eprintln!("{hint}");
        }
        std::process::exit(e.exit_code());
    }
    let message = e.render().to_string();
    let message = message.trim().trim_start_matches("error: ");
    println!("{}", json!({ "error": message, "kind": "usage" }));
    std::process::exit(2);
}

/// A layer fault in this frontend's words (C11, C116): the cause the
/// fault says, and the remedy in the CLI's own verbs — `--account`, `acq
/// auth`, `acq policy set`, `acq refresh --plan`. One arm per fault, so
/// a cause the layer adds cannot reach a person without its remedy.
pub(crate) fn fault_words(fault: &Fault) -> String {
    match fault {
        Fault::Environment(_)
        | Fault::Unopened(_)
        | Fault::Plan(_)
        | Fault::Policy(_)
        | Fault::Door(_)
        | Fault::Unresponsive(_)
        | Fault::Daemon(_)
        | Fault::Transport(_)
        | Fault::Other(_) => fault.to_string(),
        Fault::NoAccounts { .. } => format!("{fault} — log in with `acq auth`"),
        Fault::UnknownAccount { .. } => format!("{fault} (see `acq accounts`)"),
        Fault::AmbiguousAccount { .. } => {
            format!("{fault}; name one with --account")
        }
        Fault::NoUuid { .. } => format!("{fault}; log in once with `acq auth` to fix it"),
        Fault::NoSyncPolicy { username } => format!(
            "{fault} — declare one first, e.g. `{}`",
            plan_cmd::policy_set_line(username, plan_cmd::POLICY_EXAMPLE)
        ),
        Fault::NotCovered {
            covered, username, ..
        } => match covered.len() {
            0 => format!("{fault}: `acq policy set` covers a league"),
            n if n <= acquisition_store::bound::LISTED => {
                format!("{fault}: name one with `--league`")
            }
            _ => format!(
                "{fault}: name one with `--league`; `{}` lists them all",
                printed_line(&policy_show_words(username))
            ),
        },
        // the plan compiled again as the account and in the scope the
        // apply ran in, so it runs as printed (fault 1's rule)
        Fault::Replan {
            username,
            realm,
            league,
            ..
        } => format!(
            "{fault} with `{}`",
            printed_line(&way_words(&acquisition_verbs::refusal::Way::RefreshPlan {
                account: username.clone(),
                realm: *realm,
                league: league.clone(),
            }))
        ),
        Fault::Spend(_) => fault.to_string(),
        // A merge reads the stored row to merge into, so the retry is a
        // whole write naming the revision; the read-back as the account it
        // was refused for, as every printed command names it (#97).
        Fault::UnreadablePolicy {
            username, revision, ..
        } => format!(
            "{fault} — `--replace --if-revision {revision}` writes a whole policy over it \
             deliberately; `{}` names it",
            printed_line(&policy_show_words(username))
        ),
        Fault::NoDaemon { because: None, .. } => {
            "daemon is not running (a job command or `acq auth` starts one)".into()
        }
        Fault::NoDaemon { .. } => fault.to_string(),
        Fault::NoJobOnDisk { .. } => {
            format!("{fault} (a job command or `acq auth` starts one)")
        }
        Fault::Unfinished { id, .. } => format!(
            "{fault}: a job command starts one that resumes it, and `acq result {id} --wait` \
             then waits for it"
        ),
        Fault::AnotherDaemon(found) => format!("{fault}; {}", mismatch_remedy(found)),
    }
}

/// The failure's words: its chain as anyhow prints it, a layer fault in
/// the CLI's words where the chain holds one.
pub(crate) fn failure_words(e: &anyhow::Error) -> String {
    failure_words_by(e, fault_words)
}

/// [`failure_words`] with a layer fault in `words`: a surface of this
/// crate that spells some faults' remedies otherwise (the web's pages).
pub(crate) fn failure_words_by(e: &anyhow::Error, words: impl Fn(&Fault) -> String) -> String {
    let mut parts = Vec::new();
    for link in e.chain() {
        if let Some(fault) = link.downcast_ref::<Fault>() {
            parts.push(words(fault));
            return parts.join(": ");
        }
        parts.push(link.to_string());
    }
    format!("{e:#}")
}

/// Under a text refusal whose JSON offers accounts (#168's text half;
/// the answer-facts seat, CLI run 1 call 11; the review of round C,
/// finding 4) — an account to pick from those known, or one no login
/// made, with the names known — one ready command per account: the
/// offers of the refusal document `surface` prints (`refusal::document`,
/// never a second construction), each running as printed (C156): a
/// line as it stands, and a request read whole (`--request`, `--next`)
/// as the request it offers — rebound to the account, its continuation
/// reset (`refusal::again`), never the file beside a scope naming
/// another (the review of round C's fix round, finding 1) — spelled by
/// the search's one command of a request ([`offered_command`]); the
/// first of the store's bound (`acquisition_store::bound::LISTED`), the
/// rest counted with the command that lists every account. Every text
/// path that reports such a refusal prints it: main's, and `search
/// --request`'s own. Nothing for any other refusal.
pub(crate) fn ready_accounts(e: &anyhow::Error, surface: &Terminal) -> String {
    let picking = e.chain().any(|c| {
        matches!(
            c.downcast_ref::<Fault>(),
            Some(Fault::AmbiguousAccount { .. } | Fault::UnknownAccount { .. })
        )
    });
    if !picking {
        return String::new();
    }
    let doc = acquisition_verbs::refusal::document(e, surface);
    let lines: Vec<String> = doc["offers"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|o| offered_command(&o["request"]))
        .collect();
    let (shown, left) = acquisition_store::bound::bounded(lines, None, false);
    let mut out: String = shown.iter().map(|l| format!("  `{l}`\n")).collect();
    if left > 0 {
        out.push_str(&format!(
            "  and {left} more: `acq accounts --expand` lists every account\n"
        ));
    }
    out
}

/// Under a text refusal whose words say its ways on in the request's
/// words (C143) — a pricing refusal's, a refused body's — each way as
/// this terminal runs it: the offers of the refusal document `surface`
/// prints (`refusal::document`, never a second construction), a line
/// each, what it is and then its command, as a search refusal's ways
/// print. Nothing for any other refusal.
pub(crate) fn ready_ways(e: &anyhow::Error, surface: &Terminal) -> String {
    let said = acquisition_plan::pricing::Refused::find(e).is_some()
        || e.chain()
            .any(|c| c.is::<acquisition_verbs::refusal::Pointed>());
    if !said {
        return String::new();
    }
    let doc = acquisition_verbs::refusal::document(e, surface);
    doc["offers"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|o| {
            Some(format!(
                "    {}: {}\n",
                o["says"].as_str()?,
                offered_command(&o["request"])?
            ))
        })
        .collect()
}

/// An offer's request as the command a terminal prints: a line as it
/// stands; a request object as `printed.rs` spells it — what a
/// zero answer prints for its own offers — a query given as a tree alone
/// printed as its text, since that command has no tree of its own. None
/// for a request that does not read or bind: no command to print.
fn offered_command(offered: &serde_json::Value) -> Option<String> {
    if let Some(line) = offered.as_str() {
        return Some(line.to_string());
    }
    let mut request = acquisition_search::answer::Asked::read(offered)
        .ok()?
        .request;
    if request.query.text.is_none() && request.query.trade.is_none() {
        request.query.text = Some(request.bind().ok()?.text());
    }
    Some(request.command())
}

/// A failure's `--json` document (C11): the one refusal shape every
/// surface prints (`acquisition_verbs::refusal::document`; round 4 of
/// the 2g build), its words this terminal's and its offers command lines
/// ([`Terminal`]), this command's own repeated as another account.
pub(crate) fn error_report(e: &anyhow::Error) -> serde_json::Value {
    acquisition_verbs::refusal::document(e, &Terminal::this_command())
}

/// This terminal's side of the one refusal shape: its words
/// ([`failure_words`]), and an offer as the line a shell runs — a
/// search's as the request `acq search --request` reads, as a search's
/// answer holds one — with the scope it carries spelled in flags; the
/// command refused, to repeat as another account ([`Repeat`]).
pub(crate) struct Terminal {
    repeat: Repeat,
}

/// What "the same request as another account" is at this terminal, made
/// from the command as parsed, never from its words as typed (round 4's
/// review, findings 1 and 2).
pub(crate) enum Repeat {
    /// The parsed command's arguments, each given on the command line,
    /// as a shell takes them: `acq --account <name>` is put before them.
    Line(Vec<String>),
    /// A request read whole (`--request`, `--next`), as it was read —
    /// the request or the object holding it — repeated by the verbs' one
    /// rule (`refusal::again`): what `acq search --request` reads.
    Request(serde_json::Value),
    /// Nothing to repeat: a page, or a command that read its input from
    /// stdin, which a line repeated would not carry.
    None,
}

impl Terminal {
    /// The command this process was run as, parsed again by the parser
    /// that ran it.
    pub(crate) fn this_command() -> Terminal {
        Terminal {
            repeat: parsed_line().map_or(Repeat::None, Repeat::Line),
        }
    }

    /// A request read whole, refused: offered again as itself, the
    /// envelope it was read in kept.
    pub(crate) fn asking(envelope: serde_json::Value) -> Terminal {
        Terminal {
            repeat: Repeat::Request(envelope),
        }
    }

    /// A write's receipt: its one offer is the undo, a command line made
    /// from the receipt, and nothing is repeated.
    pub(crate) fn writing() -> Terminal {
        Terminal {
            repeat: Repeat::None,
        }
    }
}

/// `acq policy show` as `account`: what lists a policy's leagues past the
/// ten a refusal or a receipt names (C53).
pub(crate) fn policy_show_words(account: &str) -> Vec<String> {
    [
        vec!["acq".to_string(), "policy".into(), "show".into()],
        option_words("--account", account),
    ]
    .concat()
}

/// A scope option and its value as the words of a printed command: two
/// words, or one, `--league=-L`, where the value starts with `-`, which
/// the parser would otherwise read as an option of its own (the review
/// of the league seat's fix round, finding 3). Quoted after, as every
/// word is ([`printed_line`]).
pub(crate) fn option_words(name: &str, value: &str) -> Vec<String> {
    match value.starts_with('-') {
        true => vec![format!("{name}={value}")],
        false => vec![name.to_string(), value.to_string()],
    }
}

/// A command line as this terminal prints it: each word quoted for the
/// declared shell (C156), the one quoting every printed command takes.
pub(crate) fn printed_line(words: &[String]) -> String {
    words
        .iter()
        .map(|w| acquisition_search::answer::shell_quoted(w))
        .collect::<Vec<_>>()
        .join(" ")
}

/// A way's words as this terminal spells it, for a line a text answer
/// prints: the words its offer would send ([`Terminal`]'s `argv`), so a
/// printed next step and an offer are one spelling.
pub(crate) fn way_words(way: &acquisition_verbs::refusal::Way) -> Vec<String> {
    use acquisition_verbs::refusal::Surface as _;
    Terminal::writing().argv(way).unwrap_or_default()
}

impl acquisition_verbs::refusal::Surface for Terminal {
    fn words(&self, e: &anyhow::Error) -> String {
        failure_words(e)
    }

    fn request(
        &self,
        way: &acquisition_verbs::refusal::Way,
    ) -> Option<(serde_json::Value, Option<&'static str>)> {
        use acquisition_verbs::refusal::Way;
        match (way, &self.repeat) {
            (Way::Again { account }, Repeat::Request(envelope)) => {
                let (envelope, _) = acquisition_verbs::refusal::again(envelope, account)?;
                return Some((envelope, None));
            }
            (Way::Search(request), _) => {
                return Some((serde_json::to_value(request).ok()?, None));
            }
            _ => {}
        }
        let words = self.argv(way)?;
        Some((json!(printed_line(&words)), None))
    }

    /// The command line a way is sent back as, word by word and unquoted
    /// (C156: beside the line, what a program runs with no shell); the
    /// line is these words, each quoted for the declared shell. None for
    /// a way this surface sends as a request object, or not at all.
    fn argv(&self, way: &acquisition_verbs::refusal::Way) -> Option<Vec<String>> {
        use acquisition_verbs::refusal::Way;
        let words = |line: &[&str]| line.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        let scope = |account: &str, realm: &acquisition_protocol::realm::Realm, league: &str| {
            [
                option_words("--account", account),
                option_words("--realm", realm.as_str()),
                option_words("--league", league),
            ]
            .concat()
        };
        Some(match way {
            Way::Again { account } => match &self.repeat {
                Repeat::Line(line) => [words(&["acq"]), option_words("--account", account)]
                    .concat()
                    .into_iter()
                    .chain(line.iter().cloned())
                    .collect(),
                Repeat::Request(_) | Repeat::None => return None,
            },
            Way::Search(_) => return None,
            // the index bare; each name a value of its own, `=` joined so
            // none is read as a flag and a pattern's commas stay its own
            Way::Describe { names } if names.is_empty() => words(&["acq", "search", "--describe"]),
            Way::Describe { names } => words(&["acq", "search"])
                .into_iter()
                .chain(names.iter().map(|name| format!("--describe={name}")))
                .collect(),
            Way::Tabs {
                account,
                realm,
                league,
            } => [words(&["acq", "tabs"]), scope(account, realm, league)].concat(),
            Way::RefreshPlan {
                account,
                realm,
                league,
            } => [
                words(&["acq", "refresh", "--plan"]),
                scope(account, realm, league),
            ]
            .concat(),
            // a reviewed file's scope is the plan's own, named all the
            // same: the apply refuses a flag the plan does not match
            Way::RefreshApply {
                account,
                realm,
                league,
                plan,
            } => [
                words(&[
                    "acq",
                    "refresh",
                    &match plan {
                        Some(path) => format!("--apply={path}"),
                        None => "--apply".to_string(),
                    },
                ]),
                scope(account, realm, league),
            ]
            .concat(),
            Way::PriceShow {
                target,
                account,
                realm,
                league,
            } => [
                words(&["acq", "price", "show", target]),
                option_words("--account", account),
                realm
                    .map(|r| option_words("--realm", r.as_str()))
                    .unwrap_or_default(),
                league
                    .as_deref()
                    .map(|l| option_words("--league", l))
                    .unwrap_or_default(),
            ]
            .concat(),
            Way::PriceStatus {
                account,
                realm,
                league,
            } => [
                words(&["acq", "price", "status", "--expand"]),
                scope(account, realm, league),
            ]
            .concat(),
            // a receipt's undo, as its text receipt prints it: the price
            // words after the target, the revision last
            Way::PriceSet {
                target,
                account,
                kind,
                amount,
                currency,
                if_revision,
            } => {
                let mut line = [
                    words(&["acq", "price", "set"]),
                    option_words("--account", account),
                    words(&[target, kind]),
                ]
                .concat();
                line.extend([amount, currency].into_iter().flatten().cloned());
                // a create is 0 here, since naming none at a terminal
                // replaces whatever stands
                line.extend(words(&[
                    "--if-revision",
                    &if_revision.unwrap_or(0).to_string(),
                ]));
                line
            }
            Way::PriceClear {
                target,
                account,
                if_revision,
            } => [
                words(&["acq", "price", "clear"]),
                option_words("--account", account),
                words(&[target, "--if-revision", &if_revision.to_string()]),
            ]
            .concat(),
            Way::PolicySet {
                account,
                value,
                if_revision,
            } => [
                words(&["acq", "policy", "set"]),
                option_words("--account", account),
                words(&[
                    "--replace",
                    "--if-revision",
                    &if_revision.to_string(),
                    &value.to_string(),
                ]),
            ]
            .concat(),
            // a failed job's way on (C165): the job as `acq submit` takes
            // it, its params whole
            Way::Submit {
                account,
                kind,
                params,
            } => [
                words(&["acq"]),
                account
                    .as_deref()
                    .map(|a| option_words("--account", a))
                    .unwrap_or_default(),
                words(&["submit", kind, "--params", &params.to_string()]),
            ]
            .concat(),
            Way::Result { id } => words(&["acq", "result", &id.to_string(), "--json"]),
            Way::Currencies => words(&["acq", "reference", "currency"]),
            Way::RefusedBodies { account } => [
                words(&["acq", "store", "refused"]),
                option_words("--account", account),
            ]
            .concat(),
            Way::SavedSearches { expand } => match expand {
                true => words(&["acq", "saved", "list", "--expand"]),
                false => words(&["acq", "saved", "list"]),
            },
            Way::SavedSearchShow { id } => words(&["acq", "saved", "show", id]),
            Way::SavedSearchRun { id, account } => [
                words(&["acq"]),
                account
                    .as_deref()
                    .map(|a| option_words("--account", a))
                    .unwrap_or_default(),
                words(&["search", "--saved", id]),
            ]
            .concat(),
            Way::SavedSearchSet(set) => saved_cmd::set_words(set),
            Way::SavedSearchClear { id, if_revision } => words(&[
                "acq",
                "saved",
                "clear",
                id,
                "--if-revision",
                &if_revision.to_string(),
            ]),
        })
    }

    fn reading(&self, offered: &acquisition_search::error::Offered) -> String {
        search_cmd::terminal_reading(offered)
    }

    fn repeats_from_its_start(&self) -> bool {
        match &self.repeat {
            Repeat::Request(envelope) => acquisition_verbs::refusal::again(envelope, "")
                .is_some_and(|(_, restarted)| restarted),
            Repeat::Line(_) | Repeat::None => false,
        }
    }
}

/// This process's command, parsed again by the parser that ran it, as
/// the words a shell takes after `acq`: every argument given on the
/// command line but `--account`, at its own level — a global's first, a
/// subcommand's after its name, an option as `--name=value`, positionals
/// last behind `--` where one starts with `-`. None where any value read
/// stdin (`-`), or a request is read whole (`--request`, `--next`: its
/// refusal offers the request itself, [`Terminal::asking`]).
fn parsed_line() -> Option<Vec<String>> {
    let cmd = <Cli as clap::CommandFactory>::command();
    let matches = cmd.clone().try_get_matches_from(std::env::args_os()).ok()?;
    let mut words = Vec::new();
    parsed_level(&cmd, &matches, true, &mut words)?;
    Some(words)
}

/// One level of [`parsed_line`]: its arguments, then its subcommand's.
fn parsed_level(
    cmd: &clap::Command,
    m: &clap::ArgMatches,
    top: bool,
    out: &mut Vec<String>,
) -> Option<()> {
    let mut positionals = Vec::new();
    for arg in cmd.get_arguments() {
        let id = arg.get_id().as_str();
        if id == "account" || (arg.is_global_set() && !top) {
            continue;
        }
        if m.value_source(id) != Some(clap::parser::ValueSource::CommandLine) {
            continue;
        }
        if matches!(id, "request" | "next") {
            return None;
        }
        let named = match (arg.get_long(), arg.get_short()) {
            (Some(long), _) => format!("--{long}"),
            (None, Some(short)) => format!("-{short}"),
            (None, None) => String::new(),
        };
        if !arg.get_action().takes_values() {
            let times = match arg.get_action() {
                clap::ArgAction::Count => usize::from(m.get_count(id)),
                _ => 1,
            };
            out.extend(std::iter::repeat_n(named, times));
            continue;
        }
        let values: Vec<String> = m
            .get_raw(id)?
            .map(|v| v.to_str().map(str::to_string))
            .collect::<Option<_>>()?;
        if values.iter().any(|v| v == "-") {
            return None;
        }
        if arg.is_positional() {
            positionals.extend(values);
        } else if arg.get_long().is_some() {
            out.extend(values.into_iter().map(|v| format!("{named}={v}")));
        } else {
            for v in values {
                out.push(named.clone());
                out.push(v);
            }
        }
    }
    if positionals.iter().any(|v| v.starts_with('-')) {
        out.push("--".into());
    }
    out.extend(positionals);
    if let Some((name, sub)) = m.subcommand() {
        out.push(name.to_string());
        parsed_level(cmd.find_subcommand(name)?, sub, false, out)?;
    }
    Some(())
}

/// [`shell_hint`] over this command's own words, under the shell the
/// environment says the printed commands are for.
pub(crate) fn pasted_shell_hint() -> Option<&'static str> {
    use acquisition_search::answer::Shell;
    shell_hint(
        std::env::args().skip(1),
        Shell::of_environment() == Shell::Pwsh,
    )
}

/// C156: where the commands acq prints are for PowerShell 7.3 or newer,
/// declared or detected, a failed command whose words carry what another
/// Windows shell leaves of a printed one says so after its refusal, and
/// names the declaration that quotes them for that shell, in its own
/// syntax: a word still in single quotes, which cmd.exe reads as no
/// quote, or JSON with no double quote left, which Windows PowerShell 5.1
/// strips from an argument. Elsewhere, and on any other failure, nothing.
fn shell_hint(args: impl Iterator<Item = String>, powershell: bool) -> Option<&'static str> {
    if !powershell {
        return None;
    }
    let mut hint = None;
    for arg in args {
        if arg.starts_with('\'') {
            return Some(
                "a word still holds its single quotes, as cmd.exe passes them: \
                 the commands acq prints are for PowerShell 7.3 or newer; \
                 in cmd.exe, `set ACQ_SHELL=cmd` quotes them for it",
            );
        }
        if arg.starts_with('{') && arg.contains(':') && !arg.contains('"') {
            hint = Some(
                "the JSON reached acq with no double quotes, as Windows PowerShell 5.1 \
                 passes them: the commands acq prints are for PowerShell 7.3 or newer; \
                 in Windows PowerShell 5.1, `$env:ACQ_SHELL = 'powershell5'` quotes them for it",
            );
        }
    }
    hint
}

#[cfg(test)]
mod tests {
    use super::*;

    /// C156: a refusal under PowerShell 7.3, declared or detected, names
    /// the shell a pasted command met, from what that shell left of its
    /// words, and the declaration for that shell; under any other shell,
    /// and for words no shell mangled, nothing.
    #[test]
    fn c156_a_refusal_names_the_windows_shell_a_pasted_command_met() {
        let words = |w: &[&str]| {
            w.iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .into_iter()
        };
        let cmd = shell_hint(words(&["search", "--account", "'Alice#1234'"]), true).unwrap();
        assert!(
            cmd.contains("cmd.exe")
                && cmd.contains("PowerShell 7.3")
                && cmd.contains("`set ACQ_SHELL=cmd`"),
            "{cmd}"
        );
        let ps5 = shell_hint(words(&["policy", "set", "{version:3,realms:{}}"]), true).unwrap();
        assert!(
            ps5.contains("Windows PowerShell 5.1")
                && ps5.contains("PowerShell 7.3")
                && ps5.contains("`$env:ACQ_SHELL = 'powershell5'`"),
            "{ps5}"
        );
        assert_eq!(
            shell_hint(words(&["policy", "set", r#"{"version":3}"#]), true),
            None
        );
        assert_eq!(shell_hint(words(&["search", "class=Rings"]), true), None);
        assert_eq!(shell_hint(words(&["search", "'x'", "{a:b}"]), false), None);
    }

    /// The unreadable policy's remedy names both flags the retry needs:
    /// the merge would read the row it cannot; and its read-back names
    /// the account it was refused for (#97).
    #[test]
    fn the_unreadable_policy_remedy_names_a_whole_write_at_its_revision() {
        let words = fault_words(&Fault::UnreadablePolicy {
            username: "Alice#1234".into(),
            revision: 3,
            detail: "newer".into(),
        });
        assert!(words.contains("`--replace --if-revision 3`"), "{words}");
        assert!(
            words.contains("`acq policy show --account 'Alice#1234'` names it"),
            "{words}"
        );
    }
}
