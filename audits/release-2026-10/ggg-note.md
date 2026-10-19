# The note to GGG — draft

A draft for release step 5 (`RELEASE-SLICE.md`, "Final state"), under
the registration's framing in `CONTEXT.md`, invariant 4: "Acquisition
1.0, same registration, new capabilities". Sent by the owner, in his
own words, before anything public; this text is a starting point and
nothing here was sent. The facts are the code's and the ground
truth's (`docs/design/network-ground-truth.md`): the user agent and
the scopes are `crates/acquisition-daemon/src/provider.rs`, the
single sender is `CONTEXT.md`, invariant 1. To the support thread the
earlier OAuth correspondence ran in, if it is still open; otherwise
to the developer contact the registration names.

---

Subject: Acquisition 1.0 — same registration, new capabilities

Hi,

A heads-up before I publish anything: I am about to put out the first
release of a rewrite of Acquisition, the stash and character
management tool registered with you as the OAuth client
`acquisition`. Nothing about the registration changes. Same client
id, same PKCE loopback flow, same user agent shape,
`acquisition/<version> (contact: gerwaric@gmail.com)`, with the
version segment moving to `1.0.0-alpha.1`. The existing desktop app
keeps running beside it under the same registration.

What is new:

- The rewrite is a small local daemon that is the only thing that
  talks to your API. Every request, including the OAuth token
  exchange and refresh, goes through one rate limiter that reads your
  `X-Rate-Limit-*` headers as the source of truth and waits out every
  hold before sending again. Nothing retries through a block. It
  calls the same endpoints the current app does (profile, leagues,
  stashes and characters), and asks for the current app's scopes plus
  `account:profile`.
- Beside the command line there is an MCP server, so a user can let an
  AI agent search, price and refresh their own stash. The agent has no
  path to your API except through that same daemon and limiter, so
  its traffic is bounded exactly as a person's is, under the user's
  own login. Logging in stays a person's act in a browser; the agent
  cannot start a session.
- It does not touch the trade site, the forums or the legacy website
  endpoints. Shop pages are rendered as text for the user to paste by
  hand.

It is an alpha, published as a GitHub pre-release with direct
downloads only, so the number of users will be small at first. If
anything about this looks wrong to you, or you would rather it be
done differently, I would much rather hear it now than after people
are using it.

Thanks again for the help with the OAuth HEAD requests and the
rate-limit headers over the last couple of years; that work is what
made the limiter possible.

Tom
