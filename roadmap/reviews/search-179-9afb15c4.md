9afb15c4 — the issue’s done-when is implemented: fixed case rows and a general correction to the property; execution not verified.

- Findings: none. The recursive rule preserves term order and existing leaf checks; both rows explicitly assert the probe’s changed result and call the shared property.
- Not checked: runtime reproduction, seeded runs, or gate results, because execution was prohibited.
- Nitpicks: none.
- Issue: the incomplete shrunk input prevents comparison against the exact original body; include the full input in future reports.
- Plan/prompt: no other impediments or suggested changes.
