# AGENTS.md

## Project Context
- Repository type: Rust workspace for `peitho` (agent runtime), with supporting crates for config, audit, memory, database, and migrations.
- Runtime environment: local-first development with Docker Compose and PostgreSQL/pgvector.
- Current priorities: correctness, operability, and safe incremental delivery.

## Agent Operating Rules
- Read before changing: inspect relevant files and existing behavior before proposing edits.
- Keep scope tight: only touch files required to complete the current request.
- Preserve architecture boundaries: do not move responsibilities across crates unless explicitly requested.
- Prefer deterministic commands and outputs; avoid speculative or exploratory mutations.

## Atomic Change Discipline
- Every change must be as small as possible while still satisfying the request.
- Do not include extra considerations not required by the current task.
- Do not bundle unrelated cleanup, refactors, style-only edits, or opportunistic improvements.
- Every change unit must be atomic:
  - one intent,
  - one rationale,
  - one validation path.
- Before finalizing, perform an atomicity check:
  - verify each modified file is directly tied to the task,
  - verify no mixed intent in a single patch,
  - split work if intent is mixed.

## Large-Change Stop Policy
- If a pending change is too large, broad, or multi-intent, stop and warn the user.
- “Too large/non-atomic” includes:
  - edits spanning unrelated modules,
  - mixed feature + refactor in one change,
  - broad file churn without direct requirement linkage.
- When this condition is detected, ask the user to clean/split the workspace first.
- Continue only after explicit user consent to proceed.

## Git & Workspace Safety
- Never auto-commit or auto-push.
- Never revert user changes unless explicitly requested.
- Never use destructive git commands (for example `git reset --hard`, force checkout) unless explicitly requested.
- If unexpected unrelated modifications are detected during work, stop and ask the user how to proceed.

## Config and Secrets Policy
- Configuration must be environment-driven where applicable (for example `DATABASE_URL`, runtime flags).
- Never hardcode credentials, tokens, or secrets in source/config.
- `.env` is local-only and must not be committed.
- Keep `.env.example` updated when introducing or changing required environment variables.

## Rust, Migration, and Docker Quality Gates
- Rust changes:
  - run minimal impacted check: `cargo check -p <crate>`.
- Migration changes:
  - support PostgreSQL semantics expected by this repo,
  - include reversible `down`,
  - run `cargo check -p migration`.
- Compose/runtime changes:
  - keep values env-driven,
  - run `docker compose config`.

## Validation and Reporting Contract
- Validate only impacted modules by default (minimal check policy).
- Report clearly:
  - files changed,
  - commands run for validation,
  - known gaps or unverified risk areas.
- If validation is blocked (environment or dependency issue), state the blocker explicitly.

## Task Playbooks

### Config Update
- Change only required config structs/fields and env mappings.
- Update `.env.example` if required vars change.
- Validate impacted crate with `cargo check -p peitho-config` (or relevant crate).

### Migration Update
- Add/modify migration with explicit `up` and `down`.
- Keep schema changes focused on the requested model.
- Validate with `cargo check -p migration`.

### Compose/Infra Update
- Move values to env variables when possible.
- Avoid embedding credentials in compose.
- Validate with `docker compose config`.

### Audit/Event Model Update
- Add only required enums/fields/events.
- Keep serialized naming stable and machine-readable.
- Validate impacted crate with `cargo check -p peitho-audit`.
