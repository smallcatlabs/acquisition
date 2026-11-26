558195d6^..ea7e979d — the brief’s done-when does not fully hold: the promised read-leaf coverage is incomplete.

- **P2 — `crates/acquisition-cli/tests/store_answers.rs:486`, shape 1:** The “every read leaf” test omits `auth status` and `diagnose`, although both answer from persisted store/index data without a daemon. Over `Seeded::owner()`, neither command is exercised by this pin; the separate diagnosis test uses Alice alone. Include both leaves and assert their answers and daemon absence.

- **P3 — `crates/acquisition-cli/tests/search_json.rs:2631`, shape 7:** The converted tests still repeat fixture-owned query counts across frontends, contrary to decision 2. The same stored world and trade query expect literal `3` here and in `crates/acquisition-mcp/tests/search_tools.rs:751`; adding another qualifying item requires editing both assertions. Hold that expected count beside the shared table and consume it through both doors.

- **P3 — `crates/acquisition-client/tests/harness/seeded.rs:75`, shape 11:** The `EARLY` comment records pre-conversion history (“before this table”) in code. That explanation belongs in the commit record; remove the historical clause and retain the current requirement that subsequent fetches use later instants.

- **Not checked:** Example execution, tests, gate status, and runtime daemon absence are **not verified**, as requested. No production source or HTTP transport path changed.
- **Nitpicks:** The historical comment is a nitpick; the duplicated count is a small maintenance defect.
- **Brief:** Explicitly enumerate diagnostic and session-status reads in the required leaf set.
- **Plan:** No separate plan was supplied; the brief and commits were sufficient.
- **Prompt:** Nothing impeded the review.
