# Security

## Supported versions

I support the latest release only. Fixes ship in the next release, the
way every other change does; I do not backport them to older releases.
If you run an older one, update before you report.

## What I protect

Two things matter most here:

- **Your refresh tokens.** Acquisition keeps them in your operating
  system's keyring, never in plain text on disk, and sends them nowhere
  but Grinding Gear Games.
- **The project's relationship with GGG**: its OAuth registration and
  its standing under their rate limits.

A flaw that puts either at risk is the report I most want.

## Reporting a vulnerability

Report it privately, through GitHub's private vulnerability reporting
on this repository: the **Security** tab, then **Report a
vulnerability**. Please do not open a public issue for it.
