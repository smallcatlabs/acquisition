# The job ledger prunes only when a daemon starts, and holds every fetched body

Labels: area:daemon area:store trigger:step

`db.prune` is called from `restore_jobs` alone (`crates/acquisition-daemon/src/daemon/queue.rs:326`). A done `stash` job's payload is the whole body (`kinds.rs:629-637`). On the owner's disk `daemon.db` is 69.6 MB, 60 MB of it 3,498 such bodies that the facts store also holds. The thesis that would have removed the cause died (`findings/persistence.md`); this is the defect as it stands.
