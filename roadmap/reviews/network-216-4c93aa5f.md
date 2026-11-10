4c93aa5f — issue #216’s done-when conditions are satisfied by static inspection.

- Findings: none. The dependency guard and breakers cover the named HTTP paths; check 15 detects additional construction sites; the expanded body test exercises bearer GET, token POST and plain GET; invariant 1’s pin no longer names the issue.
- Not verified: gate exit statuses, test results and breaker execution; this review was read-only as requested.
- Nitpicks for unlikely cases: none.
- Brief: the issue provided sufficient scope and acceptance conditions.
- Plan: no separate plan was supplied or needed.
- Prompt: no impediments or changes suggested.
