//! `auth`: the login flow, `auth status`'s text, and opening the
//! browser the login needs.
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use std::io::Write as _;
use std::time::Duration;

use acquisition_client::client::DaemonError;
use acquisition_protocol::protocol::{Request, Response};
use acquisition_protocol::provider::GGG;
use anyhow::{Result, bail};
use serde_json::json;

use crate::run::{Ctx, connect};

/// The interactive login flow: ask the daemon to start OAuth, hand the URL to
/// the browser, then poll auth status until the flow resolves. The wait is
/// the daemon's, with no deadline of the CLI's own: one once gave up while
/// the daemon still registered the session. The daemon bounds its browser
/// wait (`ACQ_LOGIN_WITHIN`) and its profile fetch; the credential
/// recording that lands the login (its lock and the keyring save) has no
/// bound, a standing concern of the keyring's reach (C148), so the loop
/// ends with the flow's own result, a transport error, or the user's
/// interrupt.
pub(crate) async fn login(cx: &Ctx, no_browser: bool, json: bool) -> Result<()> {
    let mut client = connect(cx).await?;
    let url = match client.request(&Request::AuthStart {}).await? {
        Response::AuthUrl { authorize_url } => authorize_url,
        Response::Error { kind, message } => return Err(DaemonError { kind, message }.into()),
        other => bail!("unexpected response: {other:?}"),
    };
    if json {
        // First of two JSON lines (the final auth status is the second), so
        // a scripted login can read the URL it must visit.
        println!("{}", json!({ "authorize_url": url }));
        std::io::stdout().flush().ok();
    } else {
        println!("To log in, open:\n\n  {url}\n");
    }
    if !no_browser && open_browser(&url) && !json {
        println!("(opened in your browser)");
    }
    if !json {
        println!("waiting for login to complete...");
    }
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        let status = client.request(&Request::AuthStatus {}).await?;
        let Response::Auth {
            pending,
            ref login_ok,
            ref login_error,
            ref keyring,
            ..
        } = status
        else {
            bail!("unexpected response: {status:?}");
        };
        if pending {
            continue;
        }
        // Only the flow's own terminal result counts: `logged_in` is
        // aggregate state, and another account's live session must not be
        // mistaken for this login succeeding.
        if let Some(error) = login_error {
            bail!("login failed: {error}");
        }
        let Some(user) = login_ok else {
            bail!("login did not complete (see daemon log)");
        };
        if json {
            return print_auth(&status, true);
        }
        println!("logged in as {user}");
        if keyring != "ok" {
            println!("warning: keyring {keyring}; session will not survive a daemon restart");
        }
        return Ok(());
    }
}

pub(crate) fn print_auth(status: &Response, json: bool) -> Result<()> {
    if json {
        if !matches!(status, Response::Auth { .. }) {
            bail!("unexpected response: {status:?}");
        }
        println!("{}", serde_json::to_string_pretty(status)?);
        return Ok(());
    }
    print!("{}", auth_text(status)?);
    Ok(())
}

/// What `auth status` and the dashboard say while the daemon's start
/// restores its sessions from the keyring (its keyring word `restoring`):
/// no session is live yet and none is lost.
pub(crate) const RESTORING_SESSIONS: &str =
    "restoring the sessions saved in the keyring — if a keychain prompt is open, answer it";

/// `auth status`'s text. While the daemon's start restores its sessions
/// from the keyring (its keyring word `restoring`), there is no session
/// yet and none is lost: the text says the restore is under way and that
/// a keychain prompt may be waiting, never "not logged in".
fn auth_text(status: &Response) -> Result<String> {
    use std::fmt::Write as _;
    let Response::Auth {
        logged_in,
        pending,
        keyring,
        provider,
        accounts,
        ..
    } = status
    else {
        bail!("unexpected response: {status:?}");
    };
    let mut out = String::new();
    if provider == GGG {
        writeln!(out, "provider: real GGG")?;
    }
    match (logged_in, pending) {
        (_, true) => writeln!(out, "login in progress (waiting on the browser)")?,
        (false, _) if keyring == "restoring" => writeln!(out, "{RESTORING_SESSIONS}")?,
        (false, _) => writeln!(out, "not logged in — run `acq auth`")?,
        (true, _) => {
            for a in accounts {
                let token = match a.access_expires_in_seconds {
                    Some(s) if s > 0 => {
                        format!(
                            "access token valid for ~{s}s (renewed on use; the login outlasts it)"
                        )
                    }
                    _ => "access token expired (renewed on next use; the login stands)".into(),
                };
                let kr = if a.keyring == "ok" {
                    String::new()
                } else {
                    format!("; keyring {}", a.keyring)
                };
                writeln!(out, "logged in as {}: {token}{kr}", a.username)?;
            }
        }
    }
    writeln!(out, "keyring: {keyring}")?;
    Ok(out)
}

pub(crate) fn open_browser(url: &str) -> bool {
    let opener = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    std::process::Command::new(opener)
        .arg(url)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `auth status` while the daemon's start restores its sessions from
    /// the keyring: no session yet, and the text says the restore is
    /// under way, never that the user is logged out.
    #[test]
    fn auth_status_while_the_restore_reads_says_so() {
        let status = Response::Auth {
            logged_in: false,
            pending: false,
            login_ok: None,
            login_error: None,
            username: None,
            access_expires_in_seconds: None,
            keyring: "restoring".into(),
            provider: "mock".into(),
            accounts: Vec::new(),
        };
        let text = auth_text(&status).unwrap();
        assert_eq!(
            text,
            "restoring the sessions saved in the keyring — if a keychain prompt is open, answer it\nkeyring: restoring\n"
        );
        assert!(!text.contains("not logged in"), "{text}");
    }
}
