60ccd309 — done-when partially holds: mutation scope and both test gaps are addressed; measured after-cost is deferred.

- **P3 — .github/workflows/weekly.yml:11:** The required cost comparison is incomplete: the commit supplies four before-times and a projection, but explicitly postpones measured after-times to the close. Smallest fix: record the after-run identifier and all six shard wall times in the close commit.
- **Not verified:** gate results, mutation outcomes, and measured runtimes; this review only read committed code and affected callers, as requested. No behavioural defect found.
- **Nitpicks:** none; the missing comparison is an explicit acceptance condition.
- **Prompt change:** supply a range through the close commit when that commit will carry evidence needed for the done-when.
