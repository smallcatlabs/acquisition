## Rows
| id | class | M | restated | conf | else | who is promised what |
| C1 | edge | - | - | high | - | every crate: which crate may link which, and the daemon alone reaches GGG |
| C2 | edge | - | - | high | - | every frontend: the daemon owns shared state, reached only over local IPC |
| C4 | edge | 2 | - | medium | design | every frontend: a GGG request is a job it submits and follows, never a blocking call |
| C12 | edge | - | - | high | - | every frontend and the owner: a frontend consumes only the protocol and the store's read API; a third channel is ruled first |
| C14 | ggg | 2 | 2 | medium | scope | GGG and agents: agent traffic is allowed, through the one daemon gate |
| C15 | edge | 1 | - | medium | mech | every frontend: the core is async with no runtime, and each frontend brings its own |
| C34 | edge | - | - | medium | data | the owner's intent and every crate: facts, intent, derivations and effects each have one mutation path; the daemon is blind to intent and schedules nothing |
| C35 | data | 2,4 | - | high | - | the owner: annotations are irreplaceable, and no fact-side event deletes intent |
| C38 | ggg | - | - | medium | design | the user and GGG: applying a plan sends exactly its listed actions or a subset, never more |
| C46 | edge | - | - | high | - | every frontend: shared semantics are Rust; a frontend is an adapter and evaluates nothing itself |
| C47 | cross | 3 | - | medium | mech | every crate and the persisted queue: malformed external input is a structured error, never a panic |
| C48 | edge | - | - | high | - | every frontend: the SQLite schema is no surface, and there is no cached search service |
| C79 | scope | 4 | - | high | - | GGG and the owner: unsanctioned surfaces are never runtime dependencies, and the daemon never touches them |
| C110 | rule | - | - | medium | scope | the owner: the Rust tree stays relicensable by him; nothing derives from others' C++ expression |
| C120 | scope | 2 | - | high | - | GGG and users: the tool never posts to the forum or the shop and never carries POESESSID |
| C122 | check | 3,4,5,6 | - | high | - | later sessions: where a behaviour is tested, how goldens regenerate, how test processes are isolated and run |
| C153 | run | - | - | medium | check | later sessions: a finding joins an existing surface before it becomes a file; a close lists what it retired |
| C154 | run | - | - | high | - | later sessions and the owner: a retirement names what holds the property now |
| C155 | ggg | 3 | - | medium | check | the owner and the registration: no pull request or stranger's workflow runs on his hardware |
| C3 | design | 3,4 | 3,4 | medium | edge | a user: the daemon starts when needed and exits when idle, with no ceremony |
| C5 | mech | 1 | - | high | - | nobody outside the daemon: the queue is a priority queue |
| C6 | mech | 1,2,3,4,5,7,8 | 8 | medium | data | the user: submitted jobs survive a restart; a daemon that cannot open its queue refuses to start |
| C7 | ggg | - | - | high | - | the user and GGG: job params never carry a secret; tokens never enter or leave the daemon through a job |
| C127 | ggg | 2 | - | high | - | GGG: no client value steers a send off its route or reaches a URL or timer unjudged |
| C164 | ggg | - | - | high | - | GGG: a stash job whose 404 the stored listing predicts is refused and never sent |
| C165 | design | - | - | medium | edge | an agent: a job's record carries its times and a failure's kind from a closed list |
| C8 | edge | - | - | medium | design | every frontend: the protocol carries a subscribe channel of job events |
| C9 | edge | - | - | medium | mech | every frontend: the protocol is JSON lines over a local socket or named pipe |
| C85 | edge | - | - | high | - | every client: one request per connection, events as hints, a frame bound, and a stable hello plane |
| C166 | design | 3 | - | medium | edge | a client: the job list is paged and every answer fits its frame |
| C152 | design | - | - | medium | edge | a frontend: the journal's path stands in its own field, never inside prose |
| C10 | edge | - | - | medium | design | every client: a mismatched daemon is replaced by a use verb, never negotiated; observers and the MCP never replace |
| C84 | edge | 1,5 | - | medium | mech | every client: a daemon is used only when its contract and artifact match what the client would spawn |
| C83 | edge | 3,6,7,8 | 6 | medium | ggg | every client and GGG: one daemon holds one world; a client refuses a daemon on another world |
| C112 | edge | - | - | medium | ggg | the OS user: only that user can serve or reach the daemon's endpoint |
| C132 | ggg | 2,3,4,5 | 4,5 | medium | mech | GGG: one OS user's real-mode daemons share one lock, so two gates never exist |
| C86 | design | - | - | medium | edge | a user at any door: a wedged daemon is reported unresponsive, never absent, and nothing spawns over it |
| C147 | mech | 3,5 | - | medium | edge | a long-lived frontend: every door and ledger read acts on the world it is given |
| C137 | check | 3 | - | high | - | later sessions: a deadline is tested by forcing its expiry, never against the wall clock |
| C157 | check | - | - | medium | mech | later sessions: a lifecycle that waits is a pure machine, pinned by event scripts |
| C158 | check | - | - | high | - | later sessions: a daemon scenario crosses only the frontend boundary and moves its own clock |
| C160 | check | 1,2 | - | medium | mech | later sessions: every daemon timer sleeps on the one clock, so a stalled scenario fails at its park |
| C82 | edge | 4 | - | medium | design | a frontend and the user: a frontend spawns only its sibling acqd, one installation per world |
| C23 | design | 2 | - | medium | mech | a client: work of many requests is a parent job whose failure carries its children's tally as a field |
| C27 | design | 3 | - | medium | ggg | the user: a client that disappears cancels nothing; a job is reattached by id |
| C133 | ggg | 3,4,5,7 | 3 | medium | mech | GGG and the user: every exit drains in-flight refreshes, so no rotated token or journal line is lost |
| C134 | ggg | 3,5,6 | 6 | medium | mech | GGG: a body that breaks the daemon is sent once; a listener error never ends the process |
| C45 | design | 3 | - | high | - | a user or agent: with no daemon, an observer reads the persisted queue, never "no outstanding work" |
| C123 | scope | 3 | - | medium | data | the owner and other players: the mock serves only the owner's own or hand-written items, never another's body |
| C31 | ggg | 3 | 3 | high | - | GGG: many accounts share one daemon and one Cloudflare bound; Account rules are paced per account |
| C49 | mech | 2 | - | medium | edge | nobody outside the daemon: only the session layer interprets accounts |
| C50 | data | 4,5 | - | high | - | the owner's intent: an account's key is its stable uuid; a login whose profile fetch fails fails whole |
| C51 | design | 5 | - | medium | edge | a client: the daemon holds no default account; a job names exactly one |
| C135 | ggg | 3,4,5 | - | high | - | GGG: the access token rests in the keyring and is refreshed only at expiry |
| C136 | ggg | - | - | medium | edge | GGG and the daemon's own rules: no client can submit as the daemon |
| C144 | data | 4,5 | 4 | medium | ggg | the user: the daemon reports no success it did not earn, a store or keyring failure included |
| C148 | mech | 2,3,4,5,6 | - | medium | edge | every client: a keychain prompt never makes the daemon unresponsive |
| C11 | design | - | - | high | - | an agent: every CLI command has a JSON form, failures and usage errors included, with fixed exit codes |
| C13 | edge | - | - | medium | ggg | the owner and GGG: the MCP is an out-of-process thin client that never spawns or replaces a real-mode daemon |
| C16 | edge | 3 | 3 | medium | design | the user: the GUI is a loopback page that evaluates nothing and holds no state the server cannot rebuild |
| C52 | data | - | - | high | - | the owner: an agent never replaces intent without naming the revision it read |
| C53 | design | 8 | - | high | - | a reader or agent: three output levels, ten by default, ages in words beside epochs, additive JSON |
| C149 | design | 3,4 | 3 | high | - | an agent: every answer is one JSON object with its lists counted |
| C115 | design | - | - | high | - | an agent: an answer leads with the total and view before the terms block |
| C116 | cross | - | - | medium | design | every user: no registry id or store name stands in user-facing text |
| C121 | design | - | - | medium | cross | an agent: an account that cannot be picked is refused in one protocol word per mistake, on every surface |
| C138 | ggg | - | - | medium | edge | the user and GGG: no other web page can drive the local listener into sends |
| C139 | edge | 2 | 2 | high | - | the workspace: `acq web` is a CLI verb that links nothing the CLI does not |
| C191 | scope | - | - | medium | design | the owner: `acq web` stays a development surface; a desktop GUI is a separate frontend |
| C140 | ggg | - | - | high | - | GGG: item art is fetched by the user's browser, never by the daemon or tooling |
| C143 | design | - | - | high | - | every surface's reader: shared text speaks the request's words, never one frontend's flag |
| C146 | design | - | - | high | - | a reader: every printed count says what it counted, where from, and what to do next |
| C162 | mech | 2,3,4 | 3,4 | medium | cross | every surface: coverage and age are derived once, in the store |
| C163 | design | 5 | - | high | - | a reader: every time a document carries has its age in words beside it |
| C156 | design | - | - | high | - | a user at a shell: a printed command is quoted for the declared shell |
| C168 | data | - | - | high | - | the owner's intent: a page writes intent through the verbs under its revision, so a stale page is refused |
| C187 | check | 2,3 | - | high | - | later sessions: a page script's rule is held by a headless-browser gate stage that never skips |
| C17 | mech | 1 | 1 | medium | ggg | GGG: the limiter is a header-driven policy layer (invariant 2 said again) |
| C18 | ggg | 2 | - | medium | mech | GGG: limiter state is per policy, per account for Account rules, learned only from headers |
| C19 | ggg | - | - | high | - | GGG: a headerless counted send is paced as counted and never retried; a 429 holds its route |
| C20 | ggg | - | - | high | - | GGG: discovery is one visible HEAD probe per endpoint per lifetime |
| C21 | ggg | 1,2,3,4 | 1,2,3 | medium | mech | GGG: one global burst bound spans every send to its body's end, token traffic included |
| C22 | ggg | 2,6 | 2,6 | high | - | GGG: the C++ network design's properties stand, with three named adaptations |
| C24 | ggg | - | - | high | - | GGG: a rejected refresh grant is never sent again until a new login, across restarts |
| C87 | ggg | 3,4 | - | high | - | GGG: a rejected access token is never sent again |
| C25 | ggg | 6 | - | medium | design | GGG and the user: a halt fails no unsent job, so a rerun refetches nothing |
| C26 | ggg | 3,4 | 3,4 | high | - | GGG: a policy 429 is retried at most twice; a block, a 403 or a 503 never |
| C32 | ggg | 1,2 | - | high | - | GGG: only declared routes accept a headerless 2xx or go unprobed |
| C33 | check | - | - | high | - | later sessions: the limiter's spec is the test tables, not prose |
| C88 | ggg | 8,9 | 8,9 | medium | rule | GGG and the owner: the real provider is the default, and only a human at a terminal starts a real-mode daemon |
| C128 | ggg | 2 | - | high | - | GGG: a bearer goes only to the API origin |
| C129 | ggg | - | - | high | - | GGG: a Cloudflare answer stops every send, and a block's mark survives restarts |
| C130 | ggg | 4 | 4 | high | - | GGG: no redirect is followed; a 3xx is one send |
| C131 | ggg | - | - | high | - | GGG and the user: a token 2xx with unusable headers still installs its rotated tokens |
| C150 | ggg | 2,3,4 | 4 | medium | design | a user behind a proxy: the daemon's sends honour the environment's proxy |
| C159 | check | 4 | - | high | - | later sessions: every copy of the probe-exempt route set is checked equal to the daemon's |
| C161 | check | 1,3,4 | 1,4 | medium | ggg | later sessions: every send crosses one transport seam, in-process in scenarios |
| C169 | ggg | - | - | high | - | GGG and Cloudflare: the TLS stack presented never changes without a ruling and a live run |
| C190 | ggg | - | - | high | - | GGG: a run of invalid answers closes a route, with a doubling cooldown that outlives the daemon |
| C36 | edge | 6 | 6 | medium | design | the owner's intent and the daemon: the sync policy is intent a frontend's planner compiles, never a scheduler |
| C126 | design | - | - | medium | ggg | the user: a league missing from the fetched list is noted with its age, never refused |
| C37 | design | - | - | medium | data | the user: a policy's tab id covers its children |
| C39 | edge | - | - | high | - | the workspace: the planner is its own crate, linked by frontends only; the store gives neutral snapshots |
| C40 | design | - | - | medium | edge | a client: a quote is its own read-only request that reserves nothing |
| C41 | ggg | - | - | medium | edge | GGG and the user: store reads never start network traffic; only apply spends |
| C42 | ggg | - | - | medium | design | GGG and the user: a plan over its request budget is refused before any child is submitted |
| C43 | edge | 1,2,3 | - | medium | mech | the daemon: it admits plan vocabulary, never meaning, and stays plan-blind |
| C44 | data | 5,6 | 6 | medium | ggg | the owner's intent: a plan from a policy revision edited since is refused |
| C76 | design | 2 | 2 | medium | mech | the user: an explicit selection takes the same plan-and-apply door |
| C77 | design | - | - | medium | mech | the user: freshness is judged at compile, and a later refetch is no fault |
| C111 | ggg | 2,5 | - | medium | design | GGG and the user: a plan spends each request once; a replay sends nothing |
| C64 | data | 5 | - | high | - | the owner: manual prices are explicit rows, everything inherited is derived, and pricing sends nothing |
| C65 | data | - | - | high | - | the owner: every intent write carries its channel and claimed actor |
| C66 | data | 2,5 | - | high | - | the owner: an intent value that does not parse never lands, and nothing is upgraded silently |
| C67 | data | 2 | - | medium | design | the owner's prices: the shape of a buyout's target, type and amount |
| C68 | edge | - | - | medium | rule | every surface and the owner: reference data is a fifth input, reviewed and versioned, committed by a human |
| C69 | design | - | - | medium | mech | the user: a listing shows both sides, their relation and the effective price |
| C70 | design | - | - | medium | data | the user: a priced tab covers its children, and more specific rows override |
| C71 | data | 3 | - | medium | design | the owner: a batch of prices lands atomically through a plan (unbuilt) |
| C72 | data | - | - | high | - | the owner: pricing never rewrites the sync policy; the relation is reported |
| C73 | data | 7 | - | high | - | the owner: the legacy import never overwrites a row and accounts for every source row (unbuilt) |
| C74 | design | 4 | 4 | medium | scope | the user: the shop page is a pure derivation that counts every omission and sends nothing |
| C75 | mech | 1,2,3 | - | medium | edge | nobody outside the plan crate: pricing is a module there until a split buys a property |
| C78 | data | 2,4 | - | high | - | the owner: every price write can be undone from its receipt or returned prior |
| C81 | design | - | - | medium | data | the user: the effective price is the most specific statement and never contradicts the site |
| C80 | design | - | - | medium | mech | the user: a nested item's game side reads the tab a user can name |
| C170 | rule | - | - | high | - | the owner: which classes he rules on, and what a session decides |
| C171 | rule | - | - | high | - | the owner: when a session commits and pushes without asking |
| C179 | rule | - | - | high | - | the owner: how a decision in his classes reaches him |
| C181 | run | - | - | medium | rule | the owner: a milestone's date never cuts engineering |
| C172 | rule | - | - | high | - | the owner: which model plans and builds, which designs, and that he rules |
| C185 | run | - | - | medium | rule | the owner: one build at a time, and none while he sits at the machine |
| C183 | run | - | - | medium | rule | the owner and later sessions: project rules live in the tree, which governs over memory |
| C189 | rule | - | - | high | - | the owner: what a live run is, and that his own use is none |
| C173 | rule | - | - | medium | run | the owner: a step's moves; a design brief is his to rule before anything is built |
| C174 | run | - | - | high | - | builders: what a brief carries |
| C178 | run | - | - | high | - | later sessions: a close is one commit, and git log is the ledger |
| C180 | run | - | - | medium | check | every session: the road's state file, rewritten at each close, under a cap |
| C175 | rule | - | - | high | - | the owner: which rounds get a cold review |
| C176 | rule | - | - | high | - | the owner: how many reviews a thing gets |
| C177 | rule | - | - | medium | run | the owner: who runs a cold review and how its answer is kept |
| C182 | run | - | - | medium | rule | later sessions: structure before a rule, and a rule retires by naming its holder |
| C192 | run | - | - | medium | rule | later sessions and the owner: an entry states a property, and its mechanism lives on the code |
| C89 | edge | - | - | high | - | the workspace: search is its own crate over the store's read API, linked by neither the daemon nor the planner |
| C103 | edge | 4 | 4 | medium | mech | the store and search crates: one deriver per fact, place from the store, what a body says in search |
| C90 | design | 6 | 6 | high | - | a user or agent: a line is named by its displayed template |
| C91 | design | - | - | high | - | a user or agent: a query is one composable typed tree with no silent modes |
| C104 | design | - | - | high | - | a user or agent: text and tree are one value, both on every answer |
| C109 | design | - | - | high | - | a user or agent: a query nests at most 32 deep, deeper is refused |
| C92 | design | - | - | high | - | a user or agent: a comparison binds one occurrence; a sum is asked for by name |
| C93 | design | - | - | high | - | a user or agent: absent is false, and undecided is counted apart |
| C94 | design | - | - | high | - | a user or agent: a named total is defined once in reviewed reference data |
| C95 | design | - | - | high | - | a user or agent: the counts view has two shapes and one optional sum |
| C105 | design | - | - | high | - | a user or agent: a count accounts for every matching item |
| C96 | design | - | - | high | - | a user or agent: realm is the search's scope, never a term |
| C97 | design | - | - | high | - | a user or agent: vocabulary discovery is the search's own read |
| C98 | design | 3,5 | - | medium | edge | every consumer: every answer is labelled with one consistent basis, never a stale cache |
| C100 | design | - | - | high | - | a user or agent: one answer shape, and every printed id is accepted back |
| C101 | design | - | - | medium | rule | a user or agent: a missing reach is a named field, never a general door |
| C102 | design | 2 | - | medium | scope | a user or agent: each search limit is stated at the answer; the search never judges, fetches or acts |
| C106 | scope | 4 | - | medium | design | the user: reference data never embeds judgment |
| C107 | design | - | - | medium | scope | the user: the search never says "legacy"; a source's label may be a field |
| C117 | design | - | - | medium | scope | an agent: tab facts are fields; whether the site lists an item is the agent's inference |
| C118 | design | - | - | high | - | a reader: an answer says the age of its subject and of each row |
| C125 | check | 2 | - | high | - | later sessions: a table drawn from the game's export names its pin, and the gate refuses drift |
| C28 | edge | 2,4 | - | high | - | the workspace: persistence is a library; the daemon writes facts, frontends read facts and write intent |
| C29 | mech | 2 | - | medium | data | the store's readers: `items` is the one place an item is found |
| C30 | data | 2 | - | medium | ggg | the user: a refused body is kept as evidence, never a fact and never destroyed |
| C145 | mech | 4 | - | medium | ggg | the store's readers: a wrong-typed key refuses the body instead of retiring a tab's facts |
| C54 | mech | - | - | medium | data | the store's readers: a listing owns membership, a fetch owns contents and never revives |
| C55 | data | - | - | high | - | the owner's intent: a character is identified by its id, never its name |
| C56 | mech | - | - | medium | design | the store's readers: each item records its source array; an unknown array trips a counter |
| C57 | mech | 2 | - | medium | design | the store's readers: a PoE2 granted skill stays with its host, never an item |
| C108 | edge | 5 | 5 | high | - | the search crate: the store gives one snapshot read and a revision no write slips past |
| C124 | data | 2,3 | 3 | high | - | the owner: a tool that presents itself as a read never migrates or writes his store |
| C193 | edge | 6 | - | medium | data | the owner's store and GGG: a frontend writes facts only through the verbs' one door, which never starts a daemon |
| C167 | data | 1 | - | medium | edge | the owner: user-scoped intent has its own file, written under revision |
| C194 | data | - | - | medium | design | the owner: a saved search keeps what was asked, never only its translation |
| C151 | data | - | - | high | - | the owner: an unreadable intent row is named, never dropped, and can be cleared |
| C58 | cross | 2,3,4,5,6 | - | medium | ggg | every crate and GGG: realm is a coordinate above league everywhere, pc by omission on the wire |
| C59 | ggg | 1 | - | high | - | GGG: no URL shape nobody has observed is ever sent |
| C60 | mech | - | - | medium | data | the store's readers: realm is stamped from the request, and removal never over-retires |
| C61 | ggg | - | - | medium | design | GGG: a deleted or expired character is never fetched |
| C62 | mech | 2,3 | - | medium | ggg | the planner: a character changed when a newer listing differs in experience or league |
| C63 | design | - | - | medium | data | a user: a character's id prints beside its name; a policy stores what was typed |
| C188 | design | 1 | - | medium | mech | a reader: the answer names each realm's newest game release its characters state |
| C99 | design | 8 | - | medium | scope | a user or agent: translation to and from the site carries a report, and a remainder is never run as equivalent |
| C113 | design | - | - | high | - | a user or agent: a site search is a third spelling of a query, decoded offline and labelled |
| C114 | design | - | - | high | - | a user or agent: each site property filter is a named field from one table |
| C119 | design | - | - | high | - | a user or agent: an ambiguous clause says what the other reading adds or drops |
| C141 | design | - | - | high | - | an agent or a page: the league may ride beside the query |
| C142 | design | - | - | high | - | an agent or a page: the envelope may name the defence reading |
| C195 | design | 4 | 4 | medium | scope | a user: a site search goes back as a link exactly as asked, and Acquisition sends nothing |

## Where the rule did not decide

- C4, C8, C9, C165, C166, C152, C40, C10, C86: edge or design. The edge class covers "that a contract exists between two of them and how it may change", and design covers "the words or the shape of an answer". A protocol message's shape is both a contract between processes and an agent-facing shape.
- C34: edge or data. "Take the first class that fits" lists data before edge, but "the headline's first" makes it a layer boundary whose *Why:* is data.
- C38, C41, C42, C111: ggg or design. "What the daemon sends ... how often" fits a plan that bounds sends, but the promisee is the user's reviewed plan, an apply verb's semantics.
- C47, C116, C58, C162: cross or not. The test is "a new frontend or crate could break it and no existing test or check would fail". C47's lint covers named crates only, and C116's `no_ids.rs` walks only the CLI, so I called both cross. C162's derivation lives in the shared store, so I did not call it cross, although a new surface could still derive its own.
- C6, C29, C54, C56, C60, C145: data or mech. The data class names "a fact a user cannot refetch". GGG facts are refetchable, so store-correctness entries fell to mech, even where a wrong fact is "silently changed". C30 stayed data because a refused body is evidence a refetch may not reproduce.
- C13, C88, C155: ggg, edge, rule or check. C88 is both "what a live run is" (rule) and what the daemon sends (ggg). C155's subject is a workflow (check), but its promise guards the machine holding the registration.
- C17, C21: these are nearly all restatement of invariants 1 and 2, and "a promise that is only a restatement gives no class" leaves them session-class (mech). But C21 is where the invariant's gate is defined, so I kept it ggg at medium.
- C3, C27, C51: design or edge. The daemon lifecycle and account selection are user-facing behaviour, but they also set what a client may expect of the daemon process.
- C68: edge or rule. It is a fifth input (a layer) and also "a human commits" (who decides).
- C102, C106, C107, C117, C99, C195, C74: scope or design. The brief's line "the product's, not a verb's" is hard to apply when the search crate is the product's only judge of items. I took scope only for C106, which binds every reference table.
- C153, C181, C183, C185, C173, C177, C180, C182, C192: run or rule. "Who decides, how many reviews ... when a session may commit or push" names rule's items exactly, and these sit beside them without being one.
- C157, C160, C161: check or mech or ggg. Each mixes a code design rule (a machine, one clock, one seam) with how tests are written, and C161 restates invariant 1 besides.

## Noticed

- C101 clause 6 ("a change to what a user types is recorded as a ruling before the code, under the owner's delegation") conflicts with C170 clause 4 and AGENTS.md, which make agent-facing design the session's, recorded in the commit.
- The MCP's spawn rule is stated three times: C13 clause 2, C10 clause 6 and C88 clause 8. C3 clauses 3 and 4 restate C10 and C13. Invariant 1 is restated in C21, C22 clause 2, C130 clause 4, C150 clause 4 and C161.
- Several entries carry a build status as a clause: C71 and C78 ("Unbuilt: issue #48"), C73 clause 7, C193 clause 6, C99 clause 8, and C163's *Pinned:* ("Unbuilt, #164"). Under C192 these are mechanism or road state, not property.
- C33, C122, C137, C157, C158, C159, C160, C187 and C125 are of the check class, the same as the seven already retired, and most name a holder in their *Pinned:*. C122 is in AGENTS.md's capped cross-cutting list although it is check-class.
- C149 clause 4 is history ("taken at the alpha"), which AGENTS.md says the current documents should not carry.
- Many store entries (C29, C54, C56, C57, C60, C145) are owner's-class only if "facts" count as the owner's store. The brief's data class excludes refetchable facts, so the two readers may split on these more than on anything else.
- C17 consists only of a restatement of invariant 2. C18 repeats its clause 1 in other words.
- Under the strict rule, C5 is wholly mechanism (the only clause is M).
- I ran no `bothy.py show`. Every class was judged from the entries' text alone.
