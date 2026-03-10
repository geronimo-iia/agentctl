---
title: "agentctl Roadmap"
summary: "Current and future implementation phases for agentctl, the end-user CLI for agent hub management."
read_when:
  - Planning agentctl development
  - Deciding what to implement next
  - Understanding scope and sequencing
status: draft
last_updated: "2026-07-16"
---

# agentctl Roadmap

## Current Status

agentctl has successfully delivered core functionality through Phase 5:
- ✅ Hub validation and index generation (v0.1.0)
- ✅ Hub registry with caching (v0.2.0) 
- ✅ Skill management with lifecycle execution (v0.3.0)
- ✅ Config management commands (v0.4.0)
- ✅ Enhanced ignore patterns (v0.4.1)
- ✅ Skill export (v0.5.0)

## Completed Phases

Detailed documentation for completed phases is available in the archive:
- [Phase 1 — Hub Validation](archive/phase-1-hub-validation.md)
- [Phase 2 — Hub Registry](archive/phase-2-hub-registry.md)
- [Phase 3 — Skill Management](archive/phase-3-skill-management.md)
- [Phase 4 — Config Management](archive/phase-4-config-management.md)
- [Phase 4.5 — Enhanced Ignore Patterns](archive/phase-4.5-enhanced-ignore.md)
- [Phase 5 — Skill Export](archive/phase-5-skill-export.md)

---

## Phase 5.5 — Hub Refresh Enhancement

**Goal**: Add --force flag to hub refresh command for bypassing cache entirely.

### Commands

```
agentctl hub refresh --force [<id>]                 # force refresh bypassing cache
```

### Implementation

- Add --force flag to hub refresh command
- Force flag deletes cache directory before refresh
- Ensures fresh fetch from remote index URL
- Useful for development and troubleshooting

### Exit criteria

- [ ] `agentctl hub refresh --force` deletes cache and fetches fresh index
- [ ] Works with both single hub and all hubs refresh
- [ ] Tests covering force refresh functionality
- [ ] `README.md` updated with --force flag example
- [ ] `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` pass
- [ ] `CHANGELOG.md` updated, tag `v0.5.1` → release per [release process](docs/release.md)

---

## Phase 6 — Skill Import

**Goal**: Import and install skills from exported lock files.

### Commands

```
agentctl skill import skills.lock.json              # import and install from lock file
```

### Implementation

- Import reads lock file and installs all listed skills
- Handle version conflicts and missing hubs gracefully
- Reproducible installs using existing infrastructure

### Exit criteria

- [ ] `agentctl skill import skills.lock.json` installs all skills from lock file
- [ ] Handles version conflicts and missing hubs gracefully
- [ ] Export/import round-trip tests
- [ ] `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` pass
- [ ] `CHANGELOG.md` updated, tag `v0.6.0` → release per [release process](docs/release.md)

---

## Phase 7 — Skill Outdated Detection

**Goal**: List skills with newer versions available.

### Commands

```
agentctl skill outdated                             # list outdated skills
```

### Implementation

- Compare lock file versions with hub index versions
- List skills with newer versions available
- Show current vs available versions

### Exit criteria

- [ ] `agentctl skill outdated` lists skills with newer versions available
- [ ] Shows current vs available version information
- [ ] Tests covering outdated detection
- [ ] `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` pass
- [ ] `CHANGELOG.md` updated, tag `v0.7.0` → release per [release process](docs/release.md)

---

## Phase 8 — Local Skill Development

**Goal**: Link local skill directories for development.

### Commands

```
agentctl skill dev <path>                           # link local skill for development
```

### Implementation

- Link local skill directories for development
- Hot-reload skill changes without reinstall
- Validate local skills against schema

### New modules

- `src/skill/dev.rs` — local development support

### Exit criteria

- [ ] `agentctl skill dev <path>` links local skill for development
- [ ] Local development mode with hot-reload capability
- [ ] Validation for local skills
- [ ] Tests covering local development workflow
- [ ] Updated `docs/skill-management.md` with complete feature guide
- [ ] `README.md` updated with all skill management examples
- [ ] `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` pass
- [ ] `CHANGELOG.md` updated, tag `v0.8.0` → release per [release process](docs/release.md)

---

## Future Phases (Deferred)

### Phase 9 — MCP Management (If Relevant)

**Status**: Deferred pending MCP ecosystem maturity

**Scope**: WASM-based MCP server management with sandboxing

**Commands**: `agentctl mcp install/list/update/test`

**Implementation**: See [mcp-management.md](mcp-management.md) for detailed specification

**Decision point**: Revisit when MCP WASM ecosystem and security model are established

---

## Sequencing Rationale

Phase 1 is the only hard dependency before publishing hubs — it replaces `agent-hub-indexer` for local use and CI. Phases 2–4 can follow independently once hubs are live.

`agentctl` replaces `agent-hub-indexer` entirely — for local use, CI, and hub publishing. `agent-hub-indexer` is archived under `archive/agent-hub-indexer`.

## Standards

- Project structure: [agent-software/rust/project-structure-tools.md](../../agent-software/rust/project-structure-tools.md)
- CI/CD: [agent-software/rust/ci-cd.md](../../agent-software/rust/ci-cd.md)
- Commits: [agent-software/version-control-release/git-commit-semantic.md](../../agent-software/version-control-release/git-commit-semantic.md)
- Versioning: [agent-software/version-control-release/semantic-versioning.md](../../agent-software/version-control-release/semantic-versioning.md)
