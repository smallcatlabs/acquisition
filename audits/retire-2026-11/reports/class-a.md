## Rows
| id | class | M | restated | conf | else | who is promised what |
| C1 | edge | - | - | high | - | every crate: which crate links which, and the daemon alone sends to GGG |
| C2 | edge | - | - | high | - | the frontends: the daemon owns shared state, and they reach it only over local IPC |
| C4 | edge | - | - | medium | design | the frontends: an API request is a job with an id, a state and a priority, never a blocking call |
| C12 | edge | - | - | high | - | the frontends: exactly two surfaces, the protocol and the store's read API, and a third is ruled first |
| C14 | ggg | 2 | 2 | high | - | GGG: agent traffic is allowed, but only through the daemon's one gate |
| C15 | edge | 1 | - | medium | mech | the frontends: the core exposes async functions and each frontend provides the runtime |
| C34 | data | - | - | medium | edge | the owner's intent: each layer has one mutation path, and the daemon never sees intent or schedules work |
| C35 | data | 2,4 | - | high | - | the owner's intent: annotations are irreplaceable, and no fact-side event deletes them |
| C38 | ggg | - | - | medium | design | the user and GGG: an applied plan sends exactly its reviewed actions or fewer, never more |
| C46 | edge | - | - | high | - | every frontend: shared semantics live in Rust, and a frontend is only an adapter |
| C47 | cross | 3 | - | medium | mech | every crate and client: malformed input is a structured error with a stable kind, never a panic |
| C48 | edge | - | - | medium | scope | the frontends: the SQLite schema and raw SQL are not a surface, and there is no cached search service |
| C79 | scope | 4 | - | medium | ggg | GGG and the owner: unsanctioned surfaces are governed inputs that nothing at runtime depends on and the daemon never touches |
| C110 | rule | 3 | - | medium | scope | the owner: the workspace stays MIT/Apache, and nothing in it is transcribed from others' C++ |
| C120 | scope | 2 | - | high | - | GGG and the user: the tool never posts to the forum or the shop, and sends no POESESSID traffic |
| C122 | check | 3,4,5,6 | - | high | - | later sessions: how tests are written, at what layer, and how the gate runs them |
| C153 | run | - | - | medium | check | later sessions: a finding joins an existing surface before it becomes a file, and a close lists what it retired |
| C154 | run | - | - | medium | check | later sessions: a test or check is retired only by a commit that names what now holds it |
| C155 | ggg | 3,4 | - | medium | rule | the owner and the registration: no pull request or stranger's workflow runs on the owner's hardware |
| C3 | design | 3,4 | 3,4 | medium | cross | the user: the daemon starts when a use needs it and leaves no stray process |
| C5 | mech | 1 | - | high | - | nobody outside the daemon: the queue is a priority queue |
| C6 | design | 1,2,3,4,5,7,8 | 8 | medium | data | a client: submitted jobs survive a daemon restart, and a daemon that cannot keep them refuses to start |
| C7 | ggg | - | - | high | - | the account's tokens: job params are public and never carry a secret, and tokens never enter from a client |
| C127 | ggg | 2,4 | - | high | - | GGG: no client string reaches a URL or timer unjudged, so no send leaves its limiter's route |
| C164 | ggg | - | - | high | - | GGG: no request is spent on a tab the stored listing already says is not there |
| C165 | edge | - | - | low | design | an agent or a client: a job says when it ran and what kind of failure it met, and the protocol change is additive |
| C8 | edge | - | - | medium | design | every frontend: the protocol carries a job-state event channel over the same socket |
| C9 | edge | - | - | medium | mech | every client: the protocol is JSON lines over a Unix socket or named pipe |
| C85 | edge | - | - | high | - | every client: connection, subscription, frame and version semantics, including that losses are visible |
| C166 | design | 3 | - | medium | edge | a client: the job list is paged, and no answer is lost to the frame bound |
| C152 | design | - | - | high | - | a frontend and its reader: the journal's path is a field of its own, never inside error prose |
| C10 | edge | 3 | - | high | - | every client: one version of the protocol, a mismatched daemon replaced only by a use verb, never by an observer or the MCP |
| C84 | edge | 1,4,5 | - | low | mech | a client: a mismatch of contract or of artifact is reported by name, and a rebuilt acqd is a mismatch |
| C83 | ggg | 6,7 | 6 | medium | edge | GGG and the owner's world: one daemon per store root, holding a lock, with durable state in the world |
| C112 | edge | - | - | medium | ggg | the OS user: only that user can serve or reach the daemon's derived endpoint |
| C132 | ggg | 2,3,4,5 | 4,5 | medium | mech | GGG: one real-mode lock per OS user, which no variable can move, so there are never two gates |
| C86 | design | - | - | low | ggg | the user: a daemon that does not answer hello is reported as unresponsive, and nothing spawns over it |
| C147 | mech | 3,4,5 | - | medium | edge | the frontend crates: a door takes the world it acts on, and a client reaps the child it spawns |
| C137 | check | 3,4 | - | high | - | later sessions: a deadline is tested by forcing expiry, never by a wall-clock bound |
| C157 | check | 1,2,3,7,8 | - | medium | mech | later sessions: a waiting lifecycle is a pure state machine pinned by event scripts |
| C158 | check | - | - | high | - | later sessions: a daemon scenario crosses only the frontend boundary and moves its own clock |
| C160 | check | 1,2,6 | 6 | high | - | later sessions: every daemon timer goes through the one clock, so a stalled scenario fails where it parks |
| C82 | edge | 4 | - | medium | design | a frontend and the user: it spawns only the acqd installed beside it, never one found on a path |
| C23 | design | 2 | - | medium | mech | a client: a many-request job is a parent that finishes with its children and gives their tally as a field |
| C27 | design | 3 | - | medium | ggg | the user: a client that disappears or is interrupted cancels nothing, and its jobs can be reattached |
| C133 | ggg | 3,4,5,7 | 3 | medium | mech | the account's tokens and GGG: every exit drains refreshes and logins in flight, so no rotated token is lost |
| C134 | ggg | 3,5,6 | 6 | medium | mech | GGG: a body that breaks the daemon is sent once, and a listener error never drops the world lock |
| C45 | design | - | - | high | - | an observer: with no daemon running, it reads the persisted queue, never "no outstanding work" |
| C123 | scope | 3,4 | 4 | low | data | the owner and the project: mock bodies are the owner's own items or hand-written, never the trade site's or another player's |
| C31 | ggg | 3 | 3 | high | - | GGG: many accounts share one daemon, and Account rules are paced per account |
| C49 | mech | 2 | - | high | - | nobody outside the daemon: only the session layer interprets an account |
| C50 | data | 3,4,5 | 5 | high | - | the owner's intent: an account's key is its stable uuid, and a login without one fails whole |
| C51 | design | - | - | medium | edge | a client: the daemon keeps no default account, and an ambiguous submit is refused with the list |
| C135 | ggg | 3,4,5 | - | high | - | GGG: the access token rests in the keyring, and a lifetime refreshes only at expiry |
| C136 | ggg | - | - | medium | mech | GGG: no client can submit as the daemon to get around a door keyed on the daemon's own work |
| C144 | ggg | 5 | - | medium | data | the user and the tokens: no success is reported that the store or keyring did not get, and a failed save leaves no stray entry |
| C148 | mech | 2,3,4,6 | - | high | - | the clients: keyring I/O never makes the daemon unresponsive |
| C11 | design | - | - | high | - | an agent at the CLI: every command and failure has a JSON form and stable exit codes |
| C13 | edge | - | - | high | - | the daemon and GGG: the MCP is a separate thin client that never spawns or replaces a real-mode daemon |
| C16 | edge | 3 | 3 | medium | design | the pages: the GUI is a loopback web page that holds only state rebuilt from its URL and basis |
| C52 | data | - | - | high | - | the owner's intent: an agent never overwrites intent without naming the revision it read |
| C53 | design | 8 | - | high | - | a reader: the decision view, the audit view and the JSON contract, and how lists, ages and failures read |
| C149 | design | - | - | high | - | a program: every answer and refusal is one JSON object, with counts beside its lists |
| C115 | design | - | - | high | - | an agent: an answer leads with the total and the view, and the terms block comes after |
| C116 | cross | - | - | medium | design | every user of every frontend: no rule id or store name appears in user-facing text |
| C121 | design | - | - | medium | cross | a program: an account that cannot be picked is refused in one word per mistake on every surface |
| C138 | ggg | 2 | 2 | low | design | the user and GGG: no other web page can drive the local web UI into sends |
| C139 | edge | 2 | 2 | medium | design | the crates: the web UI is a CLI verb and links nothing the CLI does not |
| C191 | scope | - | - | high | - | the owner: acq web stays a development surface, and a fast desktop GUI is a separate frontend |
| C140 | ggg | 2 | - | medium | scope | GGG: the browser fetches item art, never the daemon or the tooling, and a page renders without it |
| C143 | design | - | - | medium | cross | every reader: a shared text speaks the request's words, never one frontend's flags |
| C146 | design | 5 | 5 | high | - | a reader: a printed count says what it counts, where it came from and what to do next |
| C162 | mech | 2,3,4 | 3 | medium | design | the surfaces: coverage and age come from one derivation in the store |
| C163 | design | 5 | - | high | - | a reader: every time in a document has its age in words beside it |
| C156 | design | - | - | high | - | the user at a terminal: a printed command is quoted for the declared shell |
| C168 | data | - | - | medium | design | the owner's intent: a page writes intent only through the verbs, naming the revision it replaces |
| C187 | check | 2,3 | - | high | - | later sessions: a page script's presentation rule is held by a browser-driven gate check that never skips |
| C17 | mech | 1 | - | medium | ggg | nobody outside the limiter: the limiter is a custom policy layer over a simple enforcer |
| C18 | ggg | 2 | - | medium | mech | GGG: pacing is keyed per policy (per account for Account rules) and learned only from GGG's headers |
| C19 | ggg | - | - | high | - | GGG: a send answered without headers is paced as counted and never retried, and a 429 holds its route |
| C20 | ggg | - | - | high | - | GGG: endpoint discovery is one visible HEAD probe per endpoint per lifetime |
| C21 | mech | 1,2,3,4 | 1,2,3 | low | ggg | nobody beyond invariant 1: the dispatcher puts no global cap on job tasks |
| C22 | ggg | 2,3,6 | 2,6 | medium | rule | GGG: the Rust limiter keeps the C++ design's protective properties, and changing an adaptation is a design change |
| C24 | ggg | - | - | high | - | GGG: a rejected refresh grant is never re-sent until a new login, across restarts |
| C87 | ggg | 3 | - | high | - | GGG: a rejected access token is never sent again, and the next send is the refresh |
| C25 | ggg | - | - | medium | design | GGG and the user: a halt leaves unsent jobs waiting, so a rerun does not refetch everything |
| C26 | ggg | 3 | 3 | high | - | GGG: a 429 is retried at most twice behind the hold, and a 403 or 503 never |
| C32 | ggg | 1,2 | - | high | - | GGG: only declared routes accept a headerless 2xx or go without a probe, and the list is not generalised |
| C33 | check | - | - | high | - | later sessions: the limiter's spec is the test tables, drawn from the ground truth |
| C88 | ggg | 3,4,5,8 | 8 | medium | rule | GGG and the owner: the real provider is the default, and only a human at a terminal starts a real-mode daemon |
| C128 | ggg | 2 | - | high | - | GGG and the user: a bearer token goes only to the API origin |
| C129 | ggg | - | - | high | - | GGG: a Cloudflare answer stops all sends, and a block's mark holds every daemon until cleared |
| C130 | ggg | 4 | 4 | high | - | GGG and the tokens: no redirect is followed, and its target is shown only redacted |
| C131 | ggg | 2 | 2 | high | - | the session and GGG: a token 2xx with unusable headers still installs its tokens, and the route is held |
| C150 | design | 4 | 4 | medium | ggg | the user behind a proxy: the daemon honours the environment's proxy variables |
| C159 | check | 1 | 1 | high | - | later sessions: every copy of the probe-exempt route set is checked against the daemon's |
| C161 | check | 1,3,4 | 3,4 | medium | mech | later sessions: every provider exchange crosses one transport seam, and no scenario does real I/O |
| C169 | ggg | - | - | high | - | Cloudflare and GGG: the TLS stack presented stays the same, and changing it is ruled and live-run first |
| C190 | ggg | - | - | high | - | GGG: a run of invalid answers closes the route for a growing cooldown that outlives the daemon |
| C36 | design | 3,6 | 6 | medium | data | the user: the sync policy declares coverage and freshness, is no scheduler, and a set replaces only the realms it names |
| C126 | design | 3 | - | high | - | the user: a policy may name an unknown league, and the receipt says so with the list's age |
| C37 | design | - | - | high | - | the user: a tab id in the policy covers that tab and its children |
| C39 | edge | - | - | high | - | the crates: the planner is its own crate linked by frontends only, and the store gives neutral snapshots |
| C40 | edge | - | - | low | design | a client: a quote is its own read-only request that reserves nothing |
| C41 | ggg | 4 | - | medium | edge | GGG and the user: a read never starts network traffic, and only an asserted freshness fails |
| C42 | ggg | - | - | medium | design | GGG and the user: a plan over its request budget is refused before any child is submitted |
| C43 | edge | 1,2,3 | 3 | medium | mech | the daemon: apply is a plan-blind fan-out that admits vocabulary, not meaning |
| C44 | data | 5,6 | 6 | medium | design | the owner's intent: a plan from a policy is revoked by any later edit of that policy |
| C76 | design | 2 | 2 | high | - | the user: an explicit selection compiles to a plan through the same envelope and apply |
| C77 | design | - | - | high | - | the user: freshness is judged at compile, and a replan that refetches aged facts is correct |
| C111 | ggg | 5 | - | high | - | GGG: a plan spends each request once, and a replayed plan sends nothing |
| C64 | data | 2,5 | - | medium | design | the owner's intent: only explicit prices are stored, everything inherited is derived, and pricing never contacts the daemon |
| C65 | data | 3 | - | high | - | the owner's intent: every intent write records its channel and claimed actor |
| C66 | data | 2 | - | high | - | the owner's intent: intent values are typed, round-trip exactly, and are never silently upgraded |
| C67 | data | 2 | - | medium | design | the owner's intent: a buyout's target and value shape, keyed on the store's identity |
| C68 | data | 2 | 2 | medium | rule | intent that cites reference data: tables are reviewed and committed by a human, and a tag is never removed or reused |
| C69 | design | - | - | high | - | the user: a listing shows both resolutions, their relation and the effective price, with causes |
| C70 | design | - | - | high | - | the user: a priced tab covers its children, and a more specific row overrides it |
| C71 | data | 3 | - | medium | design | the owner's intent: a price batch applies atomically under preconditions that protect intent |
| C72 | data | - | - | high | - | the owner's intent: pricing never rewrites the sync policy, and their relation is only reported |
| C73 | data | - | - | high | - | the owner's legacy data: the import never overwrites a row and accounts for every source row |
| C74 | design | 4 | 4 | medium | scope | the user: shop render is a pure derivation that sends nothing and counts every omission |
| C75 | mech | 1,3 | - | medium | edge | later sessions: pricing is a module of the plan crate until a crate split buys an edge |
| C78 | data | 2,4 | - | high | - | the owner's intent: every price write can be undone from what it returns |
| C81 | design | 2 | 2 | high | - | the user: the effective price is the most specific statement, never contradicting what the site shows |
| C80 | design | - | - | high | - | the user: a nested item's game-side tab is the one a user can name and publish |
| C170 | rule | - | - | high | - | the owner: he rules on his classes, and a session decides and records everything else |
| C171 | rule | - | - | high | - | the owner: when a session commits and pushes, and how far an approved series runs |
| C179 | rule | - | - | high | - | the owner: how a decision in his classes reaches him, one issue each |
| C181 | run | - | - | medium | rule | the owner: a milestone date never cuts engineering |
| C172 | rule | - | - | high | - | the owner: which model plans, builds and designs, and that he rules |
| C185 | run | - | - | medium | rule | the owner: one build at a time, and none while he sits at the machine |
| C183 | run | - | - | medium | rule | the owner and other sessions: project rules live in the tree, which governs over memory |
| C189 | rule | - | - | high | - | the owner: what a live run is, and that his own use of the tool is not one |
| C173 | rule | - | - | medium | run | the owner and sessions: the six moves of a step, with a design brief ruled before building |
| C174 | run | - | - | high | - | builders: a brief carries rules and commands, never their output |
| C178 | run | - | - | high | - | later sessions: a step's close is one commit, and git log is the ledger |
| C180 | run | - | - | high | - | every session: the roadmap's README holds the current state only, under a cap |
| C175 | rule | - | - | high | - | the owner: which rounds get a cold review |
| C176 | rule | - | - | high | - | the owner: how many reviews a build or a brief gets, and when the step stops for him |
| C177 | rule | - | - | medium | run | the owner: who runs a cold review, with what input, and that it is saved whole |
| C182 | run | - | - | medium | rule | later sessions: a rule is added only when no structure can hold the fault |
| C192 | run | - | - | medium | rule | the owner and later sessions: an entry states a property, and its mechanism lives with the code |
| C89 | edge | 4 | 4 | high | - | the crates: search is its own crate over the store's read API, linking no daemon, client or HTTP |
| C103 | edge | 3,4 | 4 | medium | mech | the crates: each fact has one deriver, place from the store's read and body facts in search |
| C90 | design | 6 | 6 | high | - | a user or agent: a line is named by its displayed template, and a rename is a new line |
| C91 | design | - | - | high | - | a user or agent: a query is one composable typed tree with no silent modes |
| C104 | design | - | - | high | - | a user or agent: every surface carries the text, accepts the tree, and returns both |
| C109 | design | - | - | high | - | a user or agent: a query nests at most 32 deep and deeper is refused by name |
| C92 | design | - | - | high | - | a user or agent: a comparison binds one occurrence, and the item's sum is asked for by name |
| C93 | design | - | - | high | - | a user or agent: absent is false, undecided is counted apart, never merged |
| C94 | design | - | - | high | - | a user or agent: a named total is defined once in reviewed data and never declines silently |
| C95 | design | - | - | high | - | a user or agent: the counts view has two shapes and one optional sum, with no aggregation language |
| C105 | design | - | - | high | - | a user or agent: a count accounts for every matching item |
| C96 | design | - | - | high | - | a user or agent: realm is the search's stated scope, never a term or silently spanned |
| C97 | design | - | - | high | - | a user or agent: the vocabulary is a read of the corpus through the same call as search |
| C98 | design | 1,3 | 1 | medium | edge | a user or agent: every answer is labelled with one consistent basis, and a held corpus is reused only while it matches |
| C100 | design | - | - | high | - | a user or agent: one answer shape, compact rows, and every printed id accepted back |
| C101 | rule | - | - | low | design | a user or agent: a missing reach is a field, never a general door; and a change to what a user types is ruled first |
| C102 | design | 2,6 | 6 | medium | scope | a user or agent: each limit of the search is stated at the answer |
| C106 | scope | 4 | 4 | medium | design | the user: reference data holds game data and convention, never the tool's judgment |
| C107 | design | - | - | medium | scope | the user: "legacy" is the user's word, never the search's |
| C117 | design | - | - | medium | scope | an agent: a tab's GGG facts are searchable fields, and whether the site lists an item is the agent's inference |
| C118 | design | - | - | high | - | a reader: an answer says how old its subject and rows are and what it could not see |
| C125 | check | 2,3 | - | high | - | later sessions: a table drawn from the game's export names its pin, and the gate refuses drift |
| C28 | edge | 4 | - | high | - | the crates: persistence is a shared library, the daemon writes facts and serves no query |
| C29 | mech | 2 | - | medium | data | the store's readers: bodies are kept as received, and items is the one place to look for an item |
| C30 | data | - | - | medium | ggg | the owner's store: a refused body is kept as evidence, never ingested as a fact |
| C145 | data | 4 | - | medium | mech | the owner's store: a wrong-typed key refuses the body rather than reading as a cleared tab |
| C54 | data | 6 | - | medium | mech | the owner's store: a listing owns membership, and a late fetch never revives what a listing retired |
| C55 | data | - | - | medium | design | the owner's intent: a character's identity is its id and its name only its address |
| C56 | mech | - | - | medium | design | the store's readers: every item row records its source array, and an unknown array is counted, not lifted |
| C57 | mech | 2 | - | medium | data | the store's readers: a PoE2 granted skill stays in its host, never an item row |
| C108 | edge | 3,5 | 5 | high | - | the search crate: the store gives one snapshot read and a revision no write slips past |
| C124 | data | 3 | 3 | high | - | the owner's store: a tool that says it reads opens read-only and never migrates |
| C193 | edge | 6 | - | high | - | the frontends and GGG: fact writes go through one verbs door that never starts a daemon or races one |
| C167 | data | 3 | - | medium | mech | the owner's intent: user-scoped intent lives per world in its own file, written under compare-and-swap |
| C194 | data | - | - | medium | design | the user's saved search: the row keeps what was asked, and its translation is derived at read |
| C151 | data | - | - | high | - | the owner's intent: an unreadable annotation row is named, never dropped, and can still be cleared |
| C58 | ggg | 2,3,4,5,6 | - | medium | cross | GGG and every layer: realm stands above league everywhere, and pc is sent by omission |
| C59 | ggg | 1 | - | high | - | GGG: no URL shape is sent that has not been observed for its realm |
| C60 | data | - | - | medium | mech | the owner's store: a realm's removal never retires another realm's rows |
| C61 | ggg | - | - | high | - | GGG: a deleted or expired character is never fetched |
| C62 | design | 2,3 | - | medium | ggg | the user: a character counts as changed when a newer listing shows different experience or league |
| C63 | design | - | - | high | - | the user: the characters listing prints the id, and the policy keeps what was typed |
| C188 | design | 1 | - | medium | mech | a reader: the store gives the newest game release each realm's characters state |
| C99 | design | 8 | 8 | high | - | a user or agent: the trade translation reports every clause it could not carry |
| C113 | design | 6 | 6 | high | - | a user or agent: a PoE1 site search decodes offline into a labelled query |
| C114 | design | - | - | high | - | a user or agent: every numeric property the site filters on is a named field |
| C119 | design | - | - | high | - | a user or agent: an ambiguous clause says what the other reading would add or drop |
| C141 | design | - | - | high | - | an agent or a page: the league may ride beside the query |
| C142 | design | - | - | high | - | an agent or a page: the envelope may name the defence reading |
| C195 | design | 4 | 4 | medium | scope | a person: a site search goes back to the site as asked, as a link a person opens |

## Where the rule did not decide

- C34: data or edge. "Strip the entry to its own promises, the headline's first" points to edge, since the headline is about mutation paths. "Take the first class that fits" points to data, since the *Why:* is that intent can never be clobbered. I took data.
- C38: ggg or design. A binding plan bounds what the daemon sends, which is ggg ("what the daemon sends ... how often"). But the person promised is the reviewing user. "Who is promised" does not settle it.
- C79, C106: scope or ggg. "A surface it will not touch" is named under scope, but ggg ("the relationship") comes first in order, and C79's *Why:* is the relationship. I took scope because the brief's own example of scope is a surface.
- C110: rule or scope. A licence is not in any class's list. It is the owner's to keep, so I took rule.
- C155: ggg or rule. The *Why:* is the registration, but the promise is about CI hardware and outside contributors. No class names CI security.
- C165, C40: edge or design. C165's "Additive: no field renamed or removed" is a contract-change clause on the protocol ("how it may change", which is edge). Everything else in it is answer shape. I applied "two classes take the owner's one", but low. I read C53's and C146's additive clauses as agent-facing JSON, so design, because the agent is not a crate or process. That distinction is mine, not the brief's.
- C3, C6, C86, C27, C51: design, mech or edge. These are daemon lifecycle promises a client sees through the protocol. "Contract between two of them" could make every protocol behaviour edge. I took design wherever the promise is what an agent or user meets.
- C21: its clauses 1–3 restate invariant 1 almost word for word ("one send-lifetime global gate", OAuth inside, authorize outside). Clause 4 is mechanism. The rule says a restatement gives no class, so nothing of the owner's is left, and I took mech, low. If invariant 1 was drafted from C21, the restatement runs the other way.
- C101: clause 6 ("a change to what a user types is recorded as a ruling before the code, under the owner's delegation") is a who-decides clause, which is rule. That would make the entry the owner's under "two classes take the owner's one". But the rest is plainly design, and C170 says the opposite (a query word is the session's). I took rule, low.
- C58: clause 7 (pc expressed by omission on the wire) is ggg. The rest is realm carried through every layer, which is cross or mechanism. The rule took ggg.
- C116, C121, C143: cross or design. The test is "a new frontend could break it and no existing test would fail". C116's pin is the CLI's no_ids.rs alone, so I took cross. C121 and C143 have words held in shared crates or the MCP harness, so I took design. I did not open the tests.
- C138: the listener's security (loopback, token, Host/Origin check) has no class of its own. Its *Why:* is GGG sends through apply, so ggg, low.
- C147, C148, C162: mech or edge/design. Each states a crate API or a lock discipline whose only outside promise is responsiveness or agreement across surfaces.
- C181, C183, C185, C182, C192, C177, C173: run or rule. "What only people keep" lists who decides, reviews, commit/push and live runs. These sit beside that list (the owner at his machine, milestone dates, where rules live, who runs a review).

## Noticed

- C101 clause 6 conflicts with C170 and AGENTS.md: a query word is agent-facing design, which is the session's. Whichever survives, the other is stale.
- C21's clauses 1–3 duplicate invariant 1. One of the two is redundant.
- Restatements are common. C3 (C10, C13), C83/C132 (C112, C31), C88 (C13), C89 clause 4 (C1 clause 8), C16/C139/C138 (C46, C12, C1, C88), C76 (C44), C43 (C38), C108/C103 (each other) and C113/C90 (C99, C106) each repeat an id they cite.
- C88 and C13 both state that the MCP never spawns in real mode, and C3 states it a third time.
- C133's figures (15 s drain, 3 s) and C86's (5 s hello) are numbers a user can feel. I marked C133's as M and C86's as P. A second reader may split these the other way.
- C187 describes a gate stage. Gate stages are named among the owner's classes in C170, but the seven retired rulings include gate-stage descriptions (C196, C197) classed as check. I followed the retired ones.
- C71, C73, C78, C163 and C193 carry "Unbuilt" clauses. C73 waits on an owner's ruling ("not doing until the owner rules the replace").
- I judged every entry from the script's text. I ran no `bothy.py show` and opened no test.
