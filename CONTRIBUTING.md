# Contributing

This file is a pointer, not a rulebook. The rules for changing
Acquisition live in one place, and it is not here.

## Start at AGENTS.md

Read `AGENTS.md`, and what it tells you to read, before you change
anything. It is written for coding agents, and a person follows the
same path. Where this file and `AGENTS.md` ever disagree, `AGENTS.md`
is right.

## The gate

Every change keeps the quality gate green: the commands under "Quality
gate" in `AGENTS.md`. Run them before you open a pull request.

## AI-assisted pull requests

Most of this project is written with AI assistance, and a pull request
is held to the same rule as my own commits:

- You, the person who submits it, are its author, and you answer for
  every line of it, however it was written.
- Disclose the assistance with an `Assisted-by:` trailer on your
  commits. This repository's commit-msg hook writes Claude's trailer
  that way; `AGENTS.md` says how to turn it on.
- The gate decides, for an assisted change as for any other.

## Issues

The backlog is this repository's GitHub issues, each labelled `area:*`
(the file under `decisions/` it belongs to) and `trigger:*` (what would
start it: a user's report, a seat run, a second consumer, the GUI, a
live run). What I have chosen not to do is a "Not doing" line at the
foot of the area's file in `decisions/`, with the trigger that would
reopen it, or refused outright. A bug report is welcome as an issue.
