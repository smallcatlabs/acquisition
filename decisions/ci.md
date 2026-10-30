# Decisions — CI, release and the runners

Part of the decision registry (`AGENTS.md`, "Cross-cutting rulings", whose rules apply unchanged): the rulings that bind where and how the repository's workflows run — `.github/workflows/gate-linux.yml`, `gate-windows.yml`, `weekly.yml`, `release.yml` — and the runners they run on. Read this before touching a workflow.

## Runners

- **C155 — A self-hosted runner takes only a push to `main`; a pull request, a fork's above all, never reaches the owner's hardware.** A gate job runs in a KVM guest on the owner's workstation, an image of its hosted runner's OS, as a user with no admin right, no host mount and no secret, and only while a per-platform repository variable (`SELF_HOSTED_LINUX`) says so; unset, every job is hosted. A scheduled run stays hosted, and every outside contributor's run waits for the owner's approval. *Why:* the repository is public, and a workflow a stranger can edit must never run on the machine that holds the registration. *Pinned:* nothing yet.
