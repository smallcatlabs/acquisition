//! `acq saved`: the clap adapter of the verbs' saved search
//! (`acquisition_verbs::saved`; C167, user-scoped intent in the world's
//! `user.db`). No daemon; the file is the world's, so no login is needed,
//! and an account named only narrows the realms a new search naming none
//! settles over (C96, #247). `--json` prints the verbs' document whole (C11); the
//! text is a function of it (C53): a listing of ten or fewer, more
//! counted, the ids in the last column, and a receipt as `price set`
//! prints one, the next action and the undo last.
//!
//! Rulings: `decisions/frontends.md`, `decisions/store.md`.

use acquisition_plan::pricing::Expect;
use acquisition_verbs::refusal::{Arguments, Way};
use acquisition_verbs::saved::{self, SavedRow, SavedSearches, SavedWritten, Set};
use anyhow::Result;

use crate::run::Ctx;
use crate::terminal::{Terminal, option_words, plural, printed_line, way_words};

/// `acq saved list`: ten, `--limit` that many, `--expand` every one.
pub fn list(cx: &Ctx, limit: Option<usize>, expand: bool, json: bool) -> Result<()> {
    let listed = saved::list(&cx.frontend, limit, expand)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&listed)?);
        return Ok(());
    }
    print!("{}", list_text(&listed, cx.account.as_deref()));
    Ok(())
}

/// `acq saved show <ID|NAME>`.
pub fn show(cx: &Ctx, id_or_name: &str, json: bool) -> Result<()> {
    let row = saved::show(&cx.frontend, id_or_name)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&row)?);
        return Ok(());
    }
    print!("{}", show_text(&row, cx.account.as_deref()));
    Ok(())
}

/// `acq saved set`: one parse per spelling — a create is `--name` and the
/// query with no `--id`, an edit `--id` and what it changes, `--if-revision`
/// read through [`Expect::or`] under the human's default, the revision
/// read just before the write.
#[allow(clippy::too_many_arguments)]
pub fn set(
    cx: &Ctx,
    id: Option<String>,
    if_revision: Option<i64>,
    name: Option<String>,
    query: Option<String>,
    realm: Option<String>,
    membership: Option<String>,
    json: bool,
) -> Result<()> {
    let expect = match (&id, if_revision) {
        (None, None) => Expect::Create,
        (None, Some(_)) => {
            return Err(Arguments(
                "`--if-revision` names the revision of the saved search `--id` names; a new one \
                 names neither"
                    .into(),
            )
            .into());
        }
        (Some(_), revision) => Expect::or(revision, Expect::Current),
    };
    let set = Set {
        id,
        expect,
        name,
        text: query,
        realm,
        membership,
    };
    let written = saved::set(&cx.frontend, cx.account.as_deref(), &set, &mut saved::mint)?;
    print_written(&written, cx.account.as_deref(), json)
}

/// `acq saved clear <ID>`.
pub fn clear(cx: &Ctx, id: &str, if_revision: Option<i64>, json: bool) -> Result<()> {
    let written = saved::clear(&cx.frontend, id, Expect::or(if_revision, Expect::Current))?;
    print_written(&written, cx.account.as_deref(), json)
}

/// A write's receipt: `--json` the verbs' receipt whole, its undo spelled
/// as this terminal runs it; the text its lines, its run as `account`.
fn print_written(written: &SavedWritten, account: Option<&str>, json: bool) -> Result<()> {
    if json {
        let receipt = acquisition_verbs::receipt::saved(written, &Terminal::writing());
        println!("{}", serde_json::to_string_pretty(&receipt)?);
        return Ok(());
    }
    print!("{}", written_text(written, account));
    Ok(())
}

/// A saved search's write as the words of `acq saved set`: the id and the
/// revision first, the fields after, the query last; a query starting
/// with `-`, the language's not, takes a space inside its quotes, as
/// every route prints one.
pub(crate) fn set_words(set: &Set) -> Vec<String> {
    let mut words: Vec<String> = ["acq", "saved", "set"].map(str::to_string).to_vec();
    if let Some(id) = &set.id {
        words.extend(["--id".to_string(), id.clone()]);
    }
    match set.expect {
        Expect::Exactly(revision) => {
            words.extend(["--if-revision".to_string(), revision.to_string()])
        }
        Expect::Create if set.id.is_some() => {
            words.extend(["--if-revision".to_string(), "0".to_string()])
        }
        Expect::Create | Expect::Current => {}
    }
    for (flag, value) in [
        ("--name", &set.name),
        ("--realm", &set.realm),
        ("--membership", &set.membership),
    ] {
        if let Some(value) = value {
            words.extend(option_words(flag, value));
        }
    }
    if let Some(text) = &set.text {
        words.push(match text.starts_with('-') {
            true => format!(" {text}"),
            false => text.clone(),
        });
    }
    words
}

// ---- text, a function of the JSON --------------------------------------------

/// A row's scope as the text prints it: `pc live`.
fn scope_words(row: &saved::ReadRow) -> String {
    format!("{} {}", row.scope.realm, row.scope.membership)
}

/// The run of the saved search `id` as this terminal prints it, as the
/// account the command ran under where one was named (`--account`), so
/// it runs as printed with a second account known.
fn run_line(id: &str, account: Option<&str>) -> String {
    printed_line(&way_words(&Way::SavedSearchRun {
        id: id.to_string(),
        account: account.map(str::to_string),
    }))
}

/// The listing: one line a row, the name first and the id last (C53), the
/// rest counted with what lists them, the next action last, its run as
/// `account`.
fn list_text(listed: &SavedSearches, account: Option<&str>) -> String {
    if listed.total == 0 {
        return "no saved searches: `acq saved set --name NAME 'QUERY'` saves one\n".to_string();
    }
    let mut out = format!(
        "{} (the file at revision {}):\n",
        plural(listed.total, "saved search", "saved searches"),
        listed.revision
    );
    let width = listed
        .searches
        .iter()
        .filter_map(SavedRow::name)
        .map(|n| n.chars().count())
        .max()
        .unwrap_or(0);
    for row in &listed.searches {
        out.push_str(&match row {
            SavedRow::Read(row) => format!(
                "  {:<width$}  {:<11}  {}  (revision {})  {}\n",
                row.name,
                scope_words(row),
                row.text,
                row.revision,
                row.id
            ),
            SavedRow::Unreadable(row) => format!(
                "  {:<width$}  this build cannot read it: {}  (revision {})  {}\n",
                "?", row.unreadable, row.revision, row.id
            ),
        });
    }
    if listed.left_out > 0 {
        out.push_str(&format!(
            "{} of {} listed: `{}` lists every one\n",
            listed.searches.len(),
            listed.total,
            printed_line(&way_words(&Way::SavedSearches { expand: true }))
        ));
    }
    out.push_str(&format!(
        "next: `{}` runs one; `acq saved show ID` shows one\n",
        run_line("ID", account)
    ));
    out
}

/// One row, every field a line, the next actions last, its run as
/// `account`.
fn show_text(row: &SavedRow, account: Option<&str>) -> String {
    match row {
        SavedRow::Read(row) => format!(
            "{name}  ({id})\n  scope: {scope}\n  query: {text}\n  revision {revision}, written \
             {updated} via {via}{actor}, created {created}\nnext: `{run}` runs it; `{clear}` \
             clears it\n",
            name = row.name,
            id = row.id,
            scope = scope_words(row),
            text = row.text,
            revision = row.revision,
            updated = row.updated_at_age,
            via = row.written_via,
            actor = row
                .actor
                .as_deref()
                .map(|a| format!(" as {a}"))
                .unwrap_or_default(),
            created = row.created_at_age,
            run = run_line(&row.id, account),
            clear = printed_line(&way_words(&Way::SavedSearchClear {
                id: row.id.clone(),
                if_revision: row.revision,
            })),
        ),
        SavedRow::Unreadable(row) => format!(
            "{id}: this build cannot read it ({why}), at revision {revision}\nnext: `{clear}` \
             clears it\n",
            id = row.id,
            why = row.unreadable,
            revision = row.revision,
            clear = printed_line(&way_words(&Way::SavedSearchClear {
                id: row.id.clone(),
                if_revision: row.revision,
            })),
        ),
    }
}

/// A row in a receipt's words: its name, scope and text, or why this
/// build cannot read it.
fn row_words(row: &SavedRow) -> String {
    match row {
        SavedRow::Read(row) => format!(
            "{} over {}: {} (revision {})",
            row.name,
            scope_words(row),
            row.text,
            row.revision
        ),
        SavedRow::Unreadable(row) => format!(
            "a row this build could not read (revision {}: {})",
            row.revision, row.unreadable
        ),
    }
}

/// The receipt as text (C53, C78): what the row is now and what it was,
/// then the next action and the undo, the receipt's own (`undo` or
/// `no_undo`), its run as `account`.
fn written_text(written: &SavedWritten, account: Option<&str>) -> String {
    let id = &written.target.id;
    let was = match &written.prior {
        Some(prior) => format!("was {}", row_words(prior)),
        None => "was unset".to_string(),
    };
    let undo = match written.undo() {
        Ok((way, says)) => format!("`{}` {says}", printed_line(&way_words(&way))),
        Err(why) => why,
    };
    match &written.written {
        Some(row) => format!(
            "saved search {id}: {}, {was}\nnext: `{}` runs it; {undo}\n",
            row_words(row),
            run_line(id, account)
        ),
        None => format!(
            "saved search {id}: cleared at revision {}, {was}\nnext: {undo}\n",
            written.revision
        ),
    }
}
