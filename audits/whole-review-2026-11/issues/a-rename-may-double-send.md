# Jobs queued under an old username may escape a plan's spent check

Labels: area:plans area:network trigger:step

`plan.unspent` compares a job's `account` with the current username while the plan is keyed by `account_uuid` (`crates/acquisition-plan/src/lib.rs:1345`). After a rename at GGG, a job still queued under the old name would not count as held, and the apply could send it again. Inferred by the persistence investigator, not run. Done when a test shows it or shows why not. Bears on portfolio page 5.
