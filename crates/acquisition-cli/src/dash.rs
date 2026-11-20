//! `acq dash --json` — one snapshot of the daemon's `dashboard` request.
//!
//! Pure client: one request, printed whole. An observation (C10): it never
//! spawns or replaces the daemon it reads. The snapshot is its one form;
//! the same facts in words are `daemon status`, `jobs` and the jobs page.

use acquisition_protocol::protocol::Request;
use anyhow::{Result, bail};

use crate::run::attach;

pub async fn run(cx: &crate::run::Ctx, json: bool) -> Result<()> {
    if !json {
        bail!(
            "the dashboard is a JSON snapshot — run `acq dash --json`; in words, `acq daemon status` says the limiter and the rails, `acq jobs --watch` follows the queue, and `acq web` has a jobs page"
        );
    }
    let mut client = attach(cx).await?;
    let resp = client.request(&Request::Dashboard {}).await?;
    println!("{}", serde_json::to_string_pretty(&resp)?);
    Ok(())
}
