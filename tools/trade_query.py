"""What the trade site does to a query on the way back, read in one place.

The site rewrites a query it answers: every `"disabled": false` is dropped,
keys are reordered, and its page posts `stats: []` as one `and` group with
no filters (round i, 2026-10-01), and a socket filter with every colour it
was not given sent `null` (u059, 2026-10-16): a key sent null is a key not
sent. A returned or posted query is the sheet's
row when the two are equal once `bare()` has read both. The splitter
(`trade-split.py`) and the extract (`trade-captures.py`) both compare
queries, and each read them its own way until 2026-10-01, when four of
round i's eight searches were refused by one tool after the other had been
taught: one reader of a query's shape, here, and no copy.
"""


def bare(node):
    """A query with every `"disabled": false` and every key sent null
    removed, and every stat group with no filters removed: a group of
    nothing is no group."""
    if isinstance(node, dict):
        out = {k: bare(v) for k, v in node.items() if not (k == "disabled" and v is False) and v is not None}
        if isinstance(out.get("stats"), list):
            out["stats"] = [g for g in out["stats"] if not (isinstance(g, dict) and g.get("filters") == [])]
        return out
    if isinstance(node, list):
        return [bare(v) for v in node]
    return node
