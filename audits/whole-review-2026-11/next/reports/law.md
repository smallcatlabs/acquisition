# Investigator's report: can "law" be drawn by constructors?

As returned by an Opus investigator on 2026-11-25, read-only, no cargo and no `acq`. The portfolio's page 10 ended: "The line cannot be drawn by crate. I do not have the rule that draws it." It was handed this idea to break: law is the code that can construct, or mutate, the few values that invariants and authorizations ride on; if each has one private constructor in one module, law is a short list of modules and code outside them cannot break the property. Its claims are its own.

## Verdict up front

The idea holds for three of the nine values, and those three are exactly where `tools/arch-check.sh` check 5 (the crate edges) already does the work. For the other six it fails, and the reason is concrete: the frontends link `acquisition-store` and `acquisition-plan` directly (`crates/acquisition-cli/Cargo.toml:23-24`, `crates/acquisition-mcp/Cargo.toml:19-20`), and the values those crates make have all-`pub` fields or hand out a writer. So a frontend can build or change these values without touching the module that defines them.

## Per value

| # | Value | Constructor(s) | Can a frontend construct it? | Held by |
|---|---|---|---|---|
| 1 | Send / permits | `SendPermit` is `pub(crate)` (`acquisition-daemon/src/gate.rs:166`); `ChokePoint::with_clock_and_rails` is `pub(crate)` (`ratelimit/choke.rs:267`); `Production::new` is `pub` (`acquisition-transport/src/lib.rs:111`) | No | Dependency edges (arch check 5: only the daemon links transport and reqwest). Inside the daemon, check 15 is a spelling grep ("guards a slip, not an adversary", `acquisition-transport/src/lib.rs:30-31`) |
| 2 | `RefreshPlan`, `PlanSelection`, `Selection` | Every field `pub`, with `Deserialize` (`acquisition-plan/src/lib.rs:1336-1381`, `1278-1284`, `573`). `from_value` (`:1743`) checks shape only: schema, realm/league consistency, re-serialises byte for byte | Yes. MCP takes the plan from a tool argument (`acquisition-mcp/src/server.rs:518`); the web takes it from a POST field (`acquisition-cli/src/web/mod.rs:3897-3901`) | `check_spendable` (`lib.rs:1535`): provider, account uuid, and policy revision only when present. A selection plan carries no revision and skips that check (`:1553-1556`) |
| 3 | `Account` | `pub struct Account { pub frontend, pub entry }` (`acquisition-verbs/src/frontend.rs:407-409`); `AccountEntry` all `pub` (`acquisition-store/src/index.rs:15-`) | Yes, as a struct literal. No frontend does so today (`git grep "Account {"` finds only `Fault` arms) | The door grep (`tools/arch-check.sh:555`) refuses `Index::load`, but not an `Account {…}` literal |
| 4 | Intent writes | `Annotations::put(..., expected_revision, ...)` is `pub` (`annotations.rs:1059`); `Account::intent()` is `pub` (`frontend.rs:431`) | Yes. `acquisition-cli/src/price_cmd.rs:1133` already holds a writable `Annotations` | The compare-and-swap stops races, not blind replaces: a caller can read the revision and pass it. `check_write` (`:1054-1058`) is real door-side law. The grep misses `Annotations::open(` (`annotations.rs:853`), since it names only `open_for` |
| 5 | Fact writes | `Store::record` is `pub` (`acquisition-store/src/lib.rs:1527`); `Account::facts()` opens it read-write (`frontend.rs:414`) | Yes. The web and MCP already call `.facts()` (`web/mod.rs:1345`, `server.rs:172`) | Nothing but review. C34's arch grep keeps the daemon away from intent, not frontends away from fact writes |
| 6 | `Secret` | Private field, private `expose` (`acquisition-daemon/src/auth.rs:52, 62`); `exposed_for_test` is `#[cfg(test)]` (`:80`) | No: the daemon crate is unlinkable by frontends | Type privacy plus check 5. The idea is already true here |
| 7 | Job submit | `Request` is a `pub` enum (`acquisition-protocol/src/protocol.rs:507`). `acq submit` sends any kind with any params (`acquisition-cli/src/cli.rs:311`) | Yes, by design | Checks where the daemon receives the job: the `KINDS` table, `admit_realm` / `admit_segments`, `validate_apply` (`daemon/kinds.rs:1129`). This is law by receiver, not by constructor, and it is sound |
| 8 | Web guard | One `guard` layer wraps every route (`web/mod.rs:683`, applied at `:662`). The token is a path prefix added by the `at()` closure, per route (`:635-656`). POST-only is written into each route (`get(...)`/`post(...)`) | Yes, a new route can skip it | Host/Origin and the response headers: one layer. The token and POST-only: per route. Its test file (`acquisition-cli/tests/web.rs`) was not opened |
| 9 | Place identity | `PriceTarget` is a `pub` enum of strings with `Deserialize` (`acquisition-plan/src/price.rs:109`); `FactAddress` is `pub` (`acquisition-store/src/lib.rs:2659`); tab ids in a selection are `Vec<String>` | Yes | Ids are bound to a league only when `plan_selection` compiles them against that league's snapshot (`lib.rs:1948-1960`). `apply` never re-binds them, so a hand-made envelope skips that step |

## Grouping

- **Already one private constructor:** 1 (send and permits) and 6 (secret). Both are held by the crate edge. Privacy inside the daemon adds little once no frontend can link the daemon.
- **Held by a grep or a test:** 3 (account), the "frontends open no store" part of 4 and 5, and the transport's single caller (check 15). All are spelling greps: an alias import (`use … Store as S`) or `Annotations::open(` passes them.
- **Open (only review would notice):** 2, 4, 5, the token and POST-only parts of 8, and 9.

Item 2 needs care (read, not inferred). A plan is not a capability. Any client can send the same requests through `acq submit` or protocol `Submit`, and the daemon admits them. So "plans are binding" (C38) is a promise to the user who reviewed the plan. It is not a security boundary, and a constructor cannot turn it into one.

## Smallest closing changes

| Open | Change | Small? |
|---|---|---|
| 2 | Make `RefreshPlan`'s fields private, with getters, so only the planner and `from_value` construct one. Optionally add a digest over the actions, checked at apply | Private fields: medium (many readers in renderers). A digest: small, but it proves only "unedited since printed", and a hand-written plan computes its own |
| 3 | Make `Account`'s fields private (`pub(crate)`), with accessors | Small |
| 4, 5 | Change `Account::facts()` to return a read-only handle (a `StoreRead` newtype, or `open_read_only`), keep `record` behind a daemon-only type, and move `put` behind the verbs | Medium. The store crate has to split its read type from its write type, and `store rebuild` (`store_cmd.rs:1113`) is a frontend writing facts today |
| 8 | Nest all routes under one `Router` mounted at `/{token}`, and add one layer that refuses non-GET on read routes | Small |
| 9 | Add a `TabRef` made only from a snapshot listing, which `RefreshAction::FetchTab` carries | Large in fact. The envelope is JSON that travels across MCP and HTML, so any bound value is re-read from text and needs re-checking at apply anyway |

## Where the idea fails outright

- **Properties of every new route:** the token prefix, POST-only, and no CORS. No value carries them; the router's shape and `tests/web.rs` do.
- **Wording:** "no id in user-facing text" (C116) and "no refusal echoes a value whole" (C127). These are held by goldens and review.
- **Values that come back as text:** anything that round-trips through JSON (a plan, a target, a tab id) is rebuilt by a parser. The parser becomes the constructor, and it can only check shape, not where the value came from.
- **Pacing and Cloudflare (invariants 2 and 3):** these are behaviour over time, not values. They are held by the spec tests in `ratelimit/tests/spec.rs` and `daemon/tests/dispatch.rs`.

## Verdict

Partly, and mostly a restatement. Where the rule draws a clean line (sends, tokens), check 5's crate edges already draw it. What the idea adds is a test to put to any value: "can a frontend make or change this?" Applied here, that test finds real gaps:

- `Account` and `RefreshPlan` have all-`pub` fields.
- Frontends hold a read-write `Store` and `Annotations`.
- The door grep misses `Annotations::open(` and struct literals.
- The token and POST-only are written route by route.

As a rule for which changes need cold review it does not work yet, for two reasons:

1. The list is not short. The modules that hold these constructors or checks (choke, limiter, policy, gate, transport, auth, kinds, plan `lib.rs`, pricing, verbs `refresh.rs` and `frontend.rs`, store `lib.rs`, annotations, user, index, `web/mod.rs`) come to about 20,000 lines of roughly 120,000 non-test `.rs` lines. That is about a sixth of the code, in roughly sixteen modules, three of them over 2,500 lines.
2. The rule is false today for 2, 4, 5, 8 and 9: code outside the list can break those properties.

The rule becomes usable only after the five closing changes, and even then three things stay outside it, each held by tests and review rather than by any constructor:

- the per-route web properties;
- the wording rules;
- the plan as reviewed intent.

So the honest form is: law is the constructor modules, plus the receivers' checks (the daemon's handling of a submitted job, `check_spendable`, `check_write`), plus the router's shape.
