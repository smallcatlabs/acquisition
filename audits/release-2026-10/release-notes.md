# Release notes draft — v1.0.0-beta.1

For the GitHub release the tag drafts (`release.yml`), to paste as its
body and edit; a pre-release. Links point at the README's sections on
the tag's own tree. The owner's words replace these wherever he likes.

---

Acquisition downloads your Path of Exile stash tabs and characters to
your own machine, where you can search them offline, price what you
want to sell, and render the forum shop pages that list it. This is
the first release of the rewrite: the command line `acq`, the MCP
server `acq-mcp` that lets an AI agent do the same, and the daemon
`acqd` that sends every request to Grinding Gear Games under their
rate limits. The desktop app, acquisition-qt, keeps working beside
it.

**Install:** one archive per platform below, each holding the three
binaries, the licences and the third-party notices, with `SHA256SUMS`
beside them. The README's [Install](https://github.com/smallcatlabs/acquisition/blob/v1.0.0-beta.1/README.md#install)
section says what to do on macOS (signed, not notarized), Linux
(glibc 2.35 or newer) and Windows (unsigned), then
[Log in](https://github.com/smallcatlabs/acquisition/blob/v1.0.0-beta.1/README.md#log-in)
and, for an agent,
[Your agent](https://github.com/smallcatlabs/acquisition/blob/v1.0.0-beta.1/README.md#your-agent).

**What it does:** fetch under GGG's rate limits with a plan you read
first; search your items offline with a query language the tool
explains (`acq search --describe`); set prices by hand and read them
beside the game's notes; render forum shop pages to paste. Every
command answers as JSON for a program, and the 25 MCP tools answer
the same documents.

**What it does not do:** show market prices, post to the forum, touch
the game client, or run as a GUI. Path of Exile 2 is partial: GGG
offers no PoE2 stash tabs, so only characters are fetched. See the
README's "Known gaps".

**Reporting:** a security problem through GitHub's private reporting
(`SECURITY.md`); a bug or a request as an issue; a change,
`CONTRIBUTING.md`.

Licensed MIT OR Apache-2.0.
