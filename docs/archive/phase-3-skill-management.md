---
title: "Phase 3 — Skill Management"
summary: "Completed implementation of skill install/list/remove/update with lifecycle execution."
status: archived
archived_date: "2026-07-16"
archived_reason: "Phase completed successfully, functionality integrated into agentctl v0.3.0"
---

# Phase 3 — Skill Management ✅ COMPLETED

**Goal**: Install, list, and remove skills from registered hubs with lifecycle execution.

## Commands Implemented

```
agentctl [--quiet] [--yes] [--lock <path>]
agentctl skill install <name> [--hub <id>] [--mode <mode>] [--yes]
agentctl skill list
agentctl skill remove <name> [--hub <id>] [--yes]
agentctl skill update [<name>] [--hub <id>] [--yes] [--force]
```

## Key Features Delivered

- Install to `~/.agent/skills/` with configurable root via `skills_root`
- Lock file at `~/.agentctl/skills.lock.json` tracking installations
- Git clone with sparse checkout for efficient skill installation
- `lifecycle.yaml` execution with user approval prompts
- Variable resolution with built-ins and custom variables
- Global flags: `--quiet`, `--yes`, `--lock` for automation and testing

## Architecture

- `src/skill/mod.rs` — skill install/list/remove/update operations
- `src/skill/lifecycle.rs` — parse and execute lifecycle.yaml with approval
- `src/skill/vars.rs` — built-in + custom variable resolution
- `src/skill/lock.rs` — read/write skills.lock.json
- `tests/skill_integration.rs` — 11 integration tests
- `tests/common/mod.rs` — shared test helpers

## Lifecycle Execution

**Variable Resolution**:
- Built-ins: `SKILL_NAME`, `SKILL_PATH`, `HOME`, `PLATFORM`
- Custom vars from `variables:` block evaluated sequentially
- Simple `${VAR}` string replacement with error on undefined reference

**Execution Flow**:
1. Parse `lifecycle.yaml` into typed structs
2. Select section (install/update/uninstall) based on command
3. Filter steps by current platform
4. For each step: print description, prompt approval if required, execute via `sh -c`

**Approval System**:
- Injectable `Approver` trait for testability
- `--quiet` suppresses output and implies `--yes`
- `--yes` auto-approves all steps while keeping output visible

## Update Logic

- Compare versions between lock file and hub index
- Error if no `update` lifecycle section unless `--force` specified
- `--force` runs uninstall+install sequence for full reinstall
- Handles version conflicts and missing hubs gracefully

## Release Artifacts

- **Version**: v0.3.0
- **Tests**: 45 unit + 14 hub integration + 11 skill integration = 70 tests
- **Lock File**: JSON format tracking name, version, hub, commit_hash, path
- **Configuration**: Configurable `skills_root` with `~` expansion support

## Impact

- Enabled end-to-end skill installation and management
- Established lifecycle execution pattern for skill automation
- Created robust testing infrastructure with injectable dependencies
- Provided foundation for advanced skill management features

This phase successfully delivered the core skill management functionality, enabling users to install and manage skills from registered hubs with full lifecycle support.