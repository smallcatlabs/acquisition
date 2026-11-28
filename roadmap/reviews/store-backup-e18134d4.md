aea988a2^..e18134d4 — the brief’s done-when does not fully hold.

Findings, most severe first:

- **P2 — crates/acquisition-verbs/src/backup.rs:239, shape 1:** Manifest counts silently exclude unreadable persisted intent rows. An otherwise sound SQLite file containing one malformed price or policy value can be reported as holding zero rows or no policy, although the backup retains it; `saved_held` repeats this for saved searches at line 254. Count readable and unreadable live rows together, and explicitly distinguish unreadable values from absence.

- **P2 — crates/acquisition-cli/tests/store_answers.rs:788, shapes 1 and 2:** The mock rehearsal does not establish the brief’s concurrent backup and full-document restoration requirement. The refresh can finish before backup starts, and the final assertions at line 880 compare only three counts: changed first-seen times, different event contents, or missing saved-search contents could pass. Establish the overlap and compare the backup’s documents with the restored documents, including unlimited events and removed items.

- **P3 — crates/acquisition-cli/src/store_cmd.rs:1186, shape 6:** A printed restore command fails for a client-supplied directory beginning with `-`. Creating one with `acq store backup -- -backup` prints `acq store restore -backup --account …`, which clap treats as an option. Put options before `--` and the directory after it, or print an equivalent path prefixed with `./`.

- **P3 — crates/acquisition-store/src/lib.rs:268, shape 4:** The below-floor refusal mentions restoration without naming the verb that decision 7 explicitly requires. A person encountering an old persisted facts file learns that “a backup … restored” helps but receives no `acq store restore` instruction. Supply that actionable spelling through the frontend’s remedy, preserving the shared store wording where necessary.

Execution evidence, including tests and gate status: **not verified**, as requested.

I did not audit unrelated GGG send paths or upstream-response handling; the affected backup and restore callers operate on local files.

Nitpicks for unlikely cases: the leading-hyphen directory command and the missing restore verb.

Brief, plan and prompt: the pinned brief and commit range were sufficient; no changes requested.
