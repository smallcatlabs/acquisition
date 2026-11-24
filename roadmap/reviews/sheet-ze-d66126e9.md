d66126e963f61af72d129f086626ef3804db0855 — The issue’s done-when is not yet met: this commit prepares the sitting; the captures and general translation fix remain outstanding.

- **P3 — tools/trade-sheet.py:2871, shape 1:** ze01 and ze03 interpret empty results as evidence that bounds skip the singular form, although their unbounded controls are historical captures. A current upstream response with zero matches because qualifying listings disappeared would produce the same observation. Make “none” inconclusive unless a contemporaneous control still returns the singular form; apply this qualification to both generated sheet rows.
- **P3 — tools/trade-sheet.py:2881, shape 1:** ze04’s interpretation expects exactly two sockets although its query supplies only `min: 2`. An upstream item displaying `Has 3 Abyssal Sockets` satisfies the query but falls outside the stated interpretation. Say “two or more” and judge the displayed number against the lower bound, in both generator and sheet.

- **Not checked:** execution, generated-link decoding, page generation, self-tests, and gate results — not verified, as execution was prohibited. The owner’s pending sitting is not a finding against this preparation commit.
- **Nitpicks:** Both findings concern interpretation wording; the first requires listings to have changed, and the second requires a returned value above two.
- **Brief:** The issue combines preparing evidence, collecting it, and implementing the fix; naming the preparation milestone would make this commit’s acceptance criterion clearer.
- **Plan:** No separate plan was supplied.
- **Prompt:** Reviewing the completed range would permit a definitive judgment of the issue’s full done-when.
