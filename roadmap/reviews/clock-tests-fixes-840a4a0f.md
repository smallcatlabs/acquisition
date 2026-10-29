fafa4969^..840a4a0f — the fix round’s source requirements hold: both prior findings are addressed, with semantic assertions preserved.

- Findings: none. All six performance bounds become diagnostics; the event window exceeds CI’s configured ceiling, and the test still asserts the exact fractional command.
- Execution evidence—gate status, test count, and runtime checks—not verified, as requested.
- Not checked: external CI or issue #131’s body, because this review judges committed evidence; unchanged network and credential paths were not re-audited. No production code, goldens, or reference documents changed.
- Nitpicks: none.
- Brief: the amended goal resolves the earlier conflict over deferred instances.
- Plan: no obstacle.
- Prompt: no change suggested.
