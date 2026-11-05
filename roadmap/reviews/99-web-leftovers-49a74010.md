49a74010^..c0d4f107 — the brief’s done-when is not fully met: the new account-lifetime path lacks its required boundary pin.

- **P2 — [web/mod.rs:166](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:166), shape 1:** The catch also starts the server for an unknown explicit account, contrary to Do 1’s terminal-refusal requirement. Concrete input: `acq --account Typo web` against an empty persisted index; `Index::resolve` returns `Empty` before examining the client’s selector, producing `NoAccounts`. After Alice logs in, reloading instead produces `unknown_account`, so the advertised recovery does not work. Smallest fix: defer `NoAccounts` only when no selector was supplied, and pin the explicit-selector case.

- **P3 — [web.rs:505](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/web.rs:505), shape 2:** The new test pins account discovery but never verifies that the discovered account remains fixed after another login. Concrete persisted-input sequence: empty index → Alice → first successful page → Bob also appears → reload or submit. The existing lifetime test starts with Alice already present, leaving this new path uncovered. Smallest fix: start that existing lifetime test before Alice’s login and exercise the no-account page first.

- **Not checked:** Gate results, test counts, and the form-edit demonstration are **not verified**; execution was prohibited. No live browser or upstream GGG behavior was exercised.

- **Nitpicks:** Neither finding; the second is a missing required pin rather than an observed runtime failure.

- **Brief:** Do 1 incorrectly calls an explicit selector against an empty index `unknown_account`; the existing contract and test require `not_logged_in`, so preserve that kind when restoring the terminal refusal.

- **Plan/prompt:** Nothing else impeded the review.
