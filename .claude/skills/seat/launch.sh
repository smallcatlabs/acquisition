#!/bin/bash
# launch.sh — start a seat's runner, or continue it for its debrief (the seat skill).
#
#   launch.sh WORLD MODEL DOOR PROMPT-FILE [SESSION-ID]
#
# A headless Claude Code session in WORLD, never a subagent of the session that
# made the world: a subagent starts in the repository with its instructions, the
# owner's memory index and the git log in its context, and is not blind. Started
# in WORLD, outside the repository, it loads no project instructions, no memory
# and no git status (the owner's global CLAUDE.md alone). The rails: anything not
# allowed below is denied without a prompt (dontAsk); it reads inside WORLD and
# runs WORLD's door (DOOR: seat.py or seat-mcp.py) and nothing else; no MCP
# server, no user settings, hook or allow-rule; no agent, skill, edit, write or
# web tool. With SESSION-ID it resumes that session (the debrief); without, it
# starts one. Prints the result JSON on stdout: session_id, result, denials.
set -euo pipefail
world=$(cd "$1" && pwd) model=$2 door=$3 resume=${5:-}
[[ -f $world/$door ]] || { echo "launch: no $door in $world" >&2; exit 2; }
text=$(cat "$4")  # read before the cd: a relative PROMPT-FILE is the caller's
[[ -n $text ]] || { echo "launch: $4 is empty" >&2; exit 2; }
cd "$world"
args=(-p --model "$model" --permission-mode dontAsk --strict-mcp-config --setting-sources project
      --allowedTools "Read($world/**)" "Bash(python3 -I $world/$door:*)"
      --disallowedTools Agent Workflow Skill Write Edit NotebookEdit WebFetch WebSearch
      --output-format json)
[[ -n $resume ]] && args+=(--resume "$resume")
exec claude "${args[@]}" "$text" < /dev/null
