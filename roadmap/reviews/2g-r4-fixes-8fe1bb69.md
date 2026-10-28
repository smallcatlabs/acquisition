`8fe1bb69`: both prior findings are fixed by one shared rule and pinned in source; the fix round introduces one recovery regression.

- **[P2] crates/acquisition-cli/src/search_cmd.rs:371 — Shape 3: recovery loses `--routes`, although the sibling `--next` path preserves it.** With `route.json` containing `{"request":{"scope":{"account":"Typo","realm":"pc"},"query":{"text":"class:currency"}},"count":12}`, run `acq --json search --request route.json --routes`: the parsed request gains `view.routes = true`, but recovery receives the original JSON, so replaying an offer silently omits requested count routes. Apply the accepted routes override inside the preserved envelope before passing it to `open_as_asked`, and pin replay with `--routes` present.

- **What I did not check:** execution evidence, including gate statuses and test counts, is **not verified**; no builds, tests, daemons or live calls were run, as requested.
- **Nitpicks:** none; the finding changes the recovered answer’s requested content.
- **Brief:** clarify that recovery preserves supported command-line overrides alongside the JSON envelope.
- **Plan:** no additional obstacle.
- **Prompt:** no change needed.
