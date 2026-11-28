d215e53c — the fix-round done-when does not fully hold: the restore-verb finding remains partly unfixed.

Findings:

- **P3 — crates/acquisition-cli/src/main.rs:90, shapes 3 and 4:** The restore hint reaches only failures reported by `main`. With persisted facts at schema 7, `acq --json search --request request.json`, where the client request contains `{"scope":{"account":"Bob#9"}}`, reports through `search_cmd::open_as_asked` and returns `AlreadyReported`, so neither text nor JSON names the restore verb. Apply the remedy through every CLI refusal path that can encounter this fault, preserving the request’s account, and pin this sibling path.

- **P3 — crates/acquisition-verbs/src/backup.rs:134:** Adding required `unreadable` fields changes the persisted manifest format while leaving `FORMAT` at 1. A valid backup made by the preceding build containing intent or saved searches now fails with “missing field `unreadable`”; conversely, the preceding reader rejects the new fields. Preserve compatibility within a format, or version the changed format and provide an actionable compatibility refusal.

- **P3 — crates/acquisition-verbs/src/backup.rs:179:** Removing `Standing.standing` breaks the additive JSON contract. A consumer of the preceding build’s `standing[0].standing` now receives no value after the same mixed-restore request. Retain the field as a compatibility alias; simplifying the internal representation need not remove a serialized key.

The last two findings share a cause: changing an existing serialized contract without compatibility handling.

Execution evidence, including tests and gate status: **not verified**, as requested.

I did not build, execute tests or commands under review, or audit unrelated GGG send paths and upstream-response handling; the affected operations consume client requests and persisted local files.

Nitpicks for unlikely cases: the two compatibility findings chiefly affect backups or consumers created between this unfinished step’s build and fix commits.

Brief: compatibility expectations for backups made by interim builds could be explicit. Plan: no separate plan was supplied. Prompt: nothing impeded the review.
