`4aaf3a50^..f36033f5` — the brief’s done-when does not fully hold; the close and issue dispositions are also absent from this range.

- **P1 — `crates/acquisition-cli/src/web/mod.rs:1875`, shape 7:** The item windows reacquire a corpus after the places document was completed, allowing one page to combine different persisted revisions. **Failing input:** a daemon refresh adds a matching item between these reads; the head reports two matches while its segment contains three, without a basis refusal. **Smallest fix:** retain the corpus used to establish the listing’s basis and build every segment from it; cover this interleaving in a test.

- **P2 — `crates/acquisition-cli/src/web/mod.rs:1354`:** The grouping links turn a case-insensitive league predicate into an exact store lookup using the client’s spelling. **Failing input:** `search?realm=pc&text=league%3Dstandard` matches persisted `Standard` items, but “by stash tab” leads to `league=standard`, where the listing incorrectly reports no tabs; characters have the same fault. **Smallest fix:** resolve the fixed league to its actual persisted spelling before constructing either link, preserving exact spelling where distinct variants exist, and test both destinations.

- **P2 — `crates/acquisition-cli/src/web/mod.rs:1944`:** The places page leaves `Head.basis` empty despite now rendering versioned query and window documents, violating the existing page-basis contract. **Failing reading:** `tabs?realm=pc&text=class%3Arings&open=t1` renders persisted search results without the `<meta name="basis">` that identifies their revision; this also affects opened places without a query. **Smallest fix:** propagate the documents’ basis into the head and extend `c16_every_page_s_basis_line_is_its_documents_own` to cover these pages.

Not checked: execution-dependent criteria—including tests, browser behavior, the gate and `--only doc`—are **not verified**, because execution was forbidden; hosted CI and issue state were not inspected.

Nitpicks for unlikely cases: none of the findings above.

Brief: include the close commit when requesting assessment of the issue-disposition criterion. Plan: no separate plan was supplied. Prompt: nothing else impeded the review.
