#!/usr/bin/env bash
# trade-check.sh — the sitting's tools, run as their own self-tests:
#
#   9. The sitting's tools: the sheet is what its script composes — a sat
#      search is never composed otherwise, and the sheet is appended to
#      and never edited (`tools/trade-sheet.py --verify`) — and a fetch is
#      split to the search it follows, in that search's league
#      (`tools/trade-split.py --self-test`): a round may drift with an
#      input the sitting itself updates, and a fetch found a row by its
#      query alone may be another league's. And the shipped category
#      table is what its committed sources give (`tools/category-table.py
#      --check`), or it falls behind the grouping members it is written
#      from. A generator refuses an absent input rather than reading what
#      is there (`tools/sources.py --self-test`; and category's
#      `expected.py --self-test`, which runs it without its clones and
#      reads a zero beside its own league alone); a capture is read by one
#      reader, which counts a listing sent null and refuses one its search
#      did not return (`tools/trade_query.py --self-test`); and the
#      splitter writes a capture whole. A raw directory is held to its
#      inventory, so a missing search or page is a refusal and never a
#      smaller extract (`sources.py`); no script joins a local capture to
#      a committed extract — the extract's two files are paired in one
#      place (`trade_query.py`); and a unit is staged, read back and put
#      in place with the old one set aside until it is (`trade-split.py`).
#      The totals and stat tables are what their committed inputs give,
#      read with no clone (`tools/totals-table.py --check`,
#      `tools/trade-stats-table.py --check`), and the site join keeps its
#      reason precedence (`tools/site-join.py --self-test`). The class,
#      defence and forms tables are what their generators give from the
#      `poe1` clone (C198, the gate checks the generated tables;
#      retired, its reason is this comment): their three `--check`s run,
#      as one stage, wherever the clone sits where `tools/sources.py`
#      places it (`ACQ_CLONES` moves it), and must pass, since CI has no
#      clone, so a local gate is the one place a table that has left its
#      generator's rules is seen, and those rules change in ordinary fix
#      work; with no clone there the stage is skipped with one line, as
#      the drift line is, so the CI runner, which has none, runs as
#      before; a clone off its pin fails each, by `sources.require`'s
#      refusal naming the pin (a bump regenerates the tables in the
#      order `tools/game-diff.py` prints). Between bumps the drift line
#      holds their headers to the pin (C125, the tables name their
#      export; retired, its rule and reason at `tools/game-diff.py`,
#      whose self-test and drift line run here). The category table's
#      `--tags` reads the clone too and is not here.
#
# Six stages are self-tests, each staging its own refusals on inputs it
# makes, and `gamediff`'s stage the `drift` line's too; the four that
# hold a committed file to its script (`sheet`, `category`, `totals`,
# `stats`), and the clone's three, have theirs staged by
# `tools/trade-check-breakers.sh`.
#
# Exit 1 on any failure; the report names each offender.
set -euo pipefail
cd "$(dirname "$0")/.."

fail=0

# ---- 9. the sitting's tools ------------------------------------------------
for check in "sheet tools/trade-sheet.py --verify" "split tools/trade-split.py --self-test" \
             "category tools/category-table.py --check" "totals tools/totals-table.py --check" \
             "stats tools/trade-stats-table.py --check" "sitejoin tools/site-join.py --self-test" \
             "sources tools/sources.py --self-test" \
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

# ---- the tables the clone gives (C198) -------------------------------------
# Where the clone is, read from `tools/sources.py`, never worked out here.
poe1=$(python3 -I -c 'import sys; sys.path.insert(0, "tools"); from sources import POE1; print(POE1)')
if [[ ! -e $poe1 ]]; then
  printf 'ok      %-13s %s\n' clone "no poe1 clone at $poe1: the class, defence and forms tables' checks are skipped"
else
  for check in "class tools/class-table.py" "defence tools/defence-table.py" "forms tools/trade-forms-table.py"; do
    read -r name tool <<<"$check"
    if said=$(python3 -I "$tool" --check 2>&1); then
      printf 'ok      %-13s %s\n' "$name" "$(tail -1 <<<"$said" | sed 's/^ok *//')"
    else
      printf '%s\n' "$said" | sed "s/^/$(printf '%-7s ' "$(tr a-z A-Z <<<"$name")")/"
      fail=1
    fi
  done
fi

exit $fail
