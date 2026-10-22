#!/usr/bin/env bash
# trade-check.sh — the sitting's tools, run as their own self-tests. Moved
# here whole from `tools/docs-check.sh` (step 1e of the roadmap), keeping
# its number:
#
#   9. The sitting's tools: the sheet is what its script composes — a sat
#      search is never composed otherwise, and the sheet is appended to
#      and never edited (`tools/trade-sheet.py --verify`) — and a fetch is
#      split to the search it follows, in that search's league
#      (`tools/trade-split.py --self-test`). Both from the outside review
#      of the trade import's step 7: round v drifted with an input the
#      sitting itself updated, and a fetch was found a row by its query.
#      And the shipped category table is what its committed sources give
#      (`tools/category-table.py --check`): the reading that closed step
#      7d found it six citations behind the grouping members it is
#      written from, which nothing had checked. And from the outside
#      review of 606b4a17: a generator refuses an absent input rather
#      than reading what is there (`tools/sources.py --self-test`; and
#      category's `expected.py --self-test`, which runs it without its
#      clones and reads a zero beside its own league alone); a capture
#      is read by one reader, which counts a listing sent null and
#      refuses one its search did not return (`tools/trade_query.py
#      --self-test`); and the splitter writes a capture whole. And from
#      the outside review of 555c4f58: a raw directory is held to its
#      inventory, so a missing search or page is a refusal and never a
#      smaller extract (`sources.py`); no script joins a local capture to
#      a committed extract — the extract's two files are paired in one
#      place (`trade_query.py`); and a unit is staged, read back and put
#      in place with the old one set aside until it is (`trade-split.py`).
#
# Exit 1 on any failure; the report names each offender.
set -euo pipefail
cd "$(dirname "$0")/.."

fail=0

# ---- 9. the sitting's tools ------------------------------------------------
for check in "sheet tools/trade-sheet.py --verify" "split tools/trade-split.py --self-test" \
             "category tools/category-table.py --check" "sources tools/sources.py --self-test" \
             "capture tools/trade_query.py --self-test" \
             "expected search/category/scripts/expected.py --self-test" \
             "gamediff tools/game-diff.py --self-test" "drift tools/game-diff.py --check"; do
  read -r name tool flag <<<"$check"
  if said=$(python3 -I "$tool" "$flag" 2>&1); then
    printf 'ok      %-13s %s\n' "$name" "$(tail -1 <<<"$said" | sed 's/^ok *//')"
  else
    printf '%s\n' "$said" | sed "s/^/$(printf '%-7s ' "$(tr a-z A-Z <<<"$name")")/"
    fail=1
  fi
done

exit $fail
