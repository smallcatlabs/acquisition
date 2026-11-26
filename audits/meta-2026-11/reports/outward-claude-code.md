Claude Code: what is available now (research as of 25 Nov 2026), mapped to your method

Evidence: I read the official pages on code.claude.com listed below. The changelog I could reach covers 2.1.327 to 2.1.332 (17 to 25 Nov). I could not page back to September or October, so older version notes come from the feature pages. Items marked "unverified" were not in any page I read.

**1. Custom subagents** (https://code.claude.com/docs/en/sub-agents.md)
- What: Markdown files whose body is the system prompt. Claude delegates by `description`, by @-mention, or session-wide with `claude --agent <name>`.
- Config: `.claude/agents/*.md` (also `~/.claude/agents/`, the `--agents` JSON flag, managed settings). Fields: `name`, `description` (required); `tools`, `disallowedTools`; `model` (`haiku`, `sonnet`, `opus`, `fable`, a full ID, or `inherit`); `effort`; `permissionMode`; `isolation: worktree`; `skills` (preloaded); `memory: user|project|local`; `background`; `maxTurns`; `hooks`; `mcpServers`. Settings: `permissions.deny: ["Agent(name)"]`; env `CLAUDE_CODE_SUBAGENT_MODEL`, `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` (default 20).
- Background and writes: Claude-spawned subagents run in the background by default, with a smaller tool set. They can write files if their tools allow it. Bash still writes unless denied.
- Limits: `permissionMode` is ignored for plugin subagents. A subagent in its own worktree reads CLAUDE.md from the main conversation, not from the worktree.
- Replaces: hand-written worker prompts and role definitions, and per-role model choice.

**2. Workflows** (https://code.claude.com/docs/en/workflows.md)
- What: a JavaScript script that orchestrates subagents with `agent(prompt, {schema, label, stallMs})`, `pipeline()`, `parallel()`, `phase()`, `log()`, and an `args` global. Intermediate results stay in script variables. Runs go in the background and are managed from `/workflows` (pause `p`, stop `x`, save `s`).
- Triggers: ask for "a workflow" or use the `ultracode` keyword. `/effort ultracode` makes every task a workflow. Saved scripts live in `.claude/workflows/` (shared with the repo) or `~/.claude/workflows/`. Plugins can ship them.
- Limits: no user input mid-run, and no direct shell or filesystem access (agents do that). Default concurrency is 16 agents, capped at the CPU count. `CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS` takes 1 to 256; setting it to 1 matches one build at a time. Hard caps: 1,000 agents per run, 4,096 items per call. Runs need a `Workflow` allow rule in `-p` mode.
- Gate-review-fix: the docs show a "keep fixing until a check passes" loop, and `schema` gives structured verdicts. Sign-off between stages is impossible inside one run, so rulings and cold review must stay outside. The docs describe only Claude agents. Having an agent shell out to another vendor's CLI is my inference, not documented.
- Resume: relaunching in the same session replays completed agents from saved results.
- Replaces: the orchestrating session's dispatch-collect-retry loop.

**3. Skills** (https://code.claude.com/docs/en/skills.md)
- Frontmatter: `name`, `description`, `when_to_use`, `argument-hint`, `arguments`, `disable-model-invocation`, `user-invocable`, `allowed-tools`, `model`, `effort`, `context: fork`, `agent`, `background`, `hooks`, `paths`, `shell`.
- Dynamic context: `` !`cmd` `` and fenced ```! blocks run before Claude sees the skill. A non-zero exit aborts the skill. `PreToolUse` does not fire for them. Use `PreSkillShellCommand`, added in 2.1.332.
- Bundled scripts run via `${CLAUDE_SKILL_DIR}`. Arguments are `$ARGUMENTS`, `$N`, `$name`.
- Limits: keep SKILL.md under 500 lines. After compaction, 5,000 tokens per skill and 25,000 in total are re-injected. Every skill listing costs context each turn.
- Plugins: `skills/` in the plugin. `optionalDependencies` in manifests (2.1.331). Eval and `/skill-doctor` come from the embedded reference, which says there is no public docs page yet; send gaps to `/feedback`.
- Replaces: procedure documents, and procedures that flood the main context (use `context: fork`).

**4. Hooks** (https://code.claude.com/docs/en/hooks.md; guide https://code.claude.com/docs/en/hooks-guide.md)
- Events: 34 in total. Blocking events include `PreToolUse`, `PermissionRequest`, `Stop`, `SubagentStop`, `UserPromptSubmit`, `PreSkillShellCommand`, `PreCompact`, `TaskCreated`, `TaskCompleted`, `TeammateIdle`, `ConfigChange`. Non-blocking include `SessionStart`, `PostToolUse`, `PostCompact`, `SessionEnd`, `SubagentStart`.
- Types: `command`, `http`, `mcp_tool`, `prompt`, `agent` (experimental).
- Config: `hooks` in settings files, or in skill and agent frontmatter. Keys: `matcher`, `if` (a permission rule such as `Bash(git push *)`), `timeout` (600 s default for commands), `onFailure: "block"` (2.1.295), `async`, `once`.
- Blocking: exit 2, or JSON `permissionDecision: "deny"` on `PreToolUse`, or `decision: "block"` on `Stop`. `if` is best-effort, so use deny rules for hard limits.
- Your cases: a `PreToolUse` on `git push` that refuses unless a gate-result file is fresh, with `onFailure: "block"`. A `Stop` hook that keeps the session working while the gate is red. The guide describes a block cap, so read it first. A `SessionStart` hook for step state, and a `PostCompact` hook to re-inject it.
- Replaces: rules in CLAUDE.md that depend on discipline.

**5. Scheduled tasks (formerly routines) and cloud sessions** (https://code.claude.com/docs/en/routines.md; local `/loop` https://code.claude.com/docs/en/scheduled-tasks.md)
- What: cloud sessions on Anthropic infrastructure, started by a schedule, an HTTP POST to `/fire`, or GitHub events. Research preview.
- Config: created at claude.ai/code/routines, with `/schedule`, or from the desktop app. Each task has a prompt, repositories, a cloud environment (network level, variables, cached setup script), and connectors (all included by default).
- Repo access: each run clones the repos from the default branch and pushes `claude/`-prefixed branches. Branch protection and rulesets govern pushes. Runs start in auto mode. Commits carry the connected GitHub user; the page does not mention signing, so your re-commit step stands.
- Limits: cloud minimum interval 1 hour (desktop 1 minute). Account cap 100 scheduled runs per hour. Run-now and API cap 30 per task per hour. A session cannot reach your Mac's store or local toolchain unless its setup script installs them.
- `/loop` is session-scoped. Recurring tasks now expire after 30 days (7 before 2.1.330). Maximum 50 per session.
- Replaces: manual nightly checks and PR babysitting. Not a replacement for the Mac gate.

**6. Side sessions, messaging, agent teams, worktrees** (https://code.claude.com/docs/en/cross-session-messaging.md; https://code.claude.com/docs/en/agent-view.md; https://code.claude.com/docs/en/agent-teams.md; https://code.claude.com/docs/en/worktrees.md)
- Start: `claude --bg "<prompt>"`, `/background` (`/bg`), `/fork`, and the `claude agents` view (research preview). Background sessions run locally under a supervisor.
- Messaging: Claude uses `ListAgents` and `SendMessage`. `/list-agents` shows the peers. Same-machine delivery goes over a local socket. Messages are plain text, cannot approve permissions, and count toward usage. `notify_when_idle` sends one notice, and it expires after 12 hours. Inbound control is `crossSessionInbound: accept|hold|refuse`. Requires 2.1.224 or later.
- Agent teams: experimental. Enable with `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`. One team per session, no nested teams, no resumption of in-process teammates, and much higher token use.
- Worktrees: `claude --worktree <name>` creates `.claude/worktrees/<name>` on `worktree-<name>`. `.worktreeinclude` copies ignored files. `worktree.baseRef` is `fresh` (default) or `head`.

**7. Memory** (https://code.claude.com/docs/en/memory.md)
- Files: managed policy, project `CLAUDE.md` or `.claude/CLAUDE.md` (checked in, so every clone and machine sees it), `~/.claude/CLAUDE.md`, `CLAUDE.local.md`. `AGENTS.md` can stand in for CLAUDE.md. `@path` imports work.
- Rules: `.claude/rules/*.md` with `paths:` load when matching files are read. Compaction summarises them away, so move anything that must persist to root CLAUDE.md.
- Auto memory: Claude-written, machine-local, not in the repo. The first 200 lines or 25 KB of `MEMORY.md` load.
- Sharing: only checked-in files reach every machine. Move orchestration know-how out of private memory into a checked-in skill, workflow or subagent.

**8. Model and effort** (https://code.claude.com/docs/en/model-config.md; https://code.claude.com/docs/en/fast-mode.md)
- Aliases: `opus` (Opus 5.5), `sonnet` (Sonnet 5.5), `haiku` (Haiku 5.5), `fable`, `opusplan`, and `[1m]` 1M variants. Priority: `/model`, then `--model`, then `ANTHROPIC_MODEL`, then settings.
- Per-task: subagent and skill `model` and `effort`; `CLAUDE_CODE_SUBAGENT_MODEL`; `effortLevel`; `/effort`. Opus 5.5 defaults to `medium`.
- Routing: no automatic routing by task. `fallbackModel` covers availability only. `availableModels` is a managed allowlist.
- Fast mode: `/fast`, or the `fastMode` setting. Up to 2.5x faster at a higher price, $8/$40 per MTok on Opus 5.5. Fast mode for Opus 4.8 was deprecated 30 Oct and is removed 30 Nov 2026.

**9. Context** (https://code.claude.com/docs/en/context-window.md)
- Auto-compaction near the limit. `/compact <focus>`, `/autocompact 500k`, `/context`, `/clear`, `/rewind`.
- After compaction: root CLAUDE.md and unscoped rules are re-read, up to five recent files are re-read, skill bodies are capped, background agents keep running. `PreCompact` and `PostCompact` hooks fire.
- 1M context for Sonnet 5+, Opus 4.6+, Haiku 5.5 and Fable via `[1m]`. Pricing not checked here.
- Automatic clearing of old tool results: unverified.

**10. MCP** (https://code.claude.com/docs/en/mcp.md)
- Project `.mcp.json` is committable. Approval prompts apply in interactive mode. `disabledMcpjsonServers` and `--strict-mcp-config` (2.1.246) block servers. Skills from MCP servers are beta (2.1.331).
- Tool search is on by default. The section listing its values was truncated, so exact settings are unverified.
- Using the product you build from inside a session: the page has no specific guidance, so this is unverified.

**11. Code review** (https://code.claude.com/docs/en/code-review.md; https://code.claude.com/docs/en/ultrareview.md; https://code.claude.com/docs/en/github-actions.md)
- `/code-review` runs locally on any plan. Since 2.1.328 it reports bugs and CLAUDE.md violations on most models; use `/simplify` for cleanups.
- `/code-review ultra` (alias `/ultrareview`) runs a cloud fleet with independent verification. It takes 5 to 15 minutes. Three free runs per account, then roughly $5 to $25 per review in usage credits. Needs a claude.ai login, so it is not available on Bedrock, Foundry or Agent Platform. `claude ultrareview` is the scriptable form.
- Code Review (cloud, PR-triggered, Team and Enterprise): reads `REVIEW.md` from the base branch, posts inline comments, is read-only, and does not push.
- GitHub Actions: `anthropics/claude-code-action@v1` with `prompt` and `claude_args`. Its GitHub App holds broad permissions.
- Auto-fix of CI on PRs: the steward and ship pages exist (https://code.claude.com/docs/en/pr-steward.md, https://code.claude.com/docs/en/ship-command.md), but I did not read them.
- These cannot replace your cold review. They are the same vendor reviewing its own build, not an outside review.

**12. Headless and SDK** (https://code.claude.com/docs/en/headless.md)
- `claude -p` with `--bare` skips hooks, skills, CLAUDE.md, MCP and auto memory unless you pass them back. The docs say `--bare` will become the default for `-p`.
- Output: `--output-format text|json|stream-json`, `--json-schema`, `--verbose --include-partial-messages`. Also `--permission-mode auto|dontAsk`, `--allowedTools`, `--permission-prompts none` (2.1.259), `--max-budget-usd`, `--max-turns`, `--settings`, `--agents`.
- Blind-agent test: `claude --bare -p` with a settings file that has an allowlist, the brief as the prompt, and stream-json as the transcript. `parent_tool_use_id` links subagent messages to their parents. `total_cost_usd` appears in json output.
- SDK packages: Python `claude-agent-sdk`, TypeScript `@anthropic-ai/claude-agent-sdk`.

**13. Recent changes and deprecations** (changelog: https://code.claude.com/docs/en/changelog.md)
- 2.1.332 (25 Nov): `PreSkillShellCommand`; MCP skills (beta); `--model default` steps down to an older model if needed.
- 2.1.331 (23 Nov): plugin `optionalDependencies`. Managed `extraKnownMarketplaces` auto-update by default on or after 30 Nov 2026 unless `"autoUpdate": false`. Relevant to your setting.
- 2.1.330 (20 Nov): scheduled tasks and `/loop` run 30 days.
- 2.1.329 (19 Nov): `/tasks` lists scheduled tasks. Most saved tasks need rescheduling in new signed-in sessions.
- 2.1.328 (19 Nov): `onFailure` for `mcp_tool`; `/code-review` scope change.
- Deprecated: fast mode on Opus 4.8 (removed 30 Nov 2026); the automatic `max_tokens` retry (removed 2.1.329).
- Auto mode is the default for interactive terminal and VS Code sessions from 2.1.283 (https://code.claude.com/docs/en/permission-modes.md). Check `permissions.defaultMode` in your settings.

**Ranked: five features most likely to remove hand-rolled process weight**
1. **Hooks.** They turn the gate-before-push and never-end-red rules into enforcement, which fits your "structure over discipline" principle. Warn: `if` is best-effort, so use deny rules for hard limits. A Stop hook that runs the full gate every turn is slow, and a Stop hook can loop.
2. **Subagent definitions** (`model`, `effort`, `tools`, `isolation: worktree`, `memory`, `skills`). Your worker roles become checked-in files, and mechanical work can go to `haiku` at low effort. Warn: worktree subagents read the main CLAUDE.md.
3. **Workflows.** A build-check-fix loop with `schema` and resume is a script, not a hand-run session. Set `CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS=1` to match one build at a time. Warn: no sign-off mid-run, so rulings and the cold review stay outside, and agents are Claude only as documented.
4. **Skills with `context: fork`, `paths` and dynamic context, plus `claude plugin eval` and `/skill-doctor`.** Procedures become measurable units. Warn: each skill listing costs context every turn.
5. **Background sessions with `ListAgents` and `SendMessage`, plus worktrees.** Worker sessions can be started and checked without hand prompts. Warn: messages count toward usage and cannot approve, and agent teams are experimental and token-heavy.

**Warn against**
- Code Review or ultrareview as a stand-in for the cross-vendor cold review.
- Cloud scheduled tasks as the gate runner. They get a fresh clone and no local store, and the feature is in research preview.
- Auto or bypass modes used to skip gate or push prompts. Keep the push-to-main refusal as a deny rule.
- Fast mode as a default.
