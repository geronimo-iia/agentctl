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

agentctl has successfully delivered core functionality through Phase 4.5:
- ✅ Hub validation and index generation (v0.1.0)
- ✅ Hub registry with caching (v0.2.0) 
- ✅ Skill management with lifecycle execution (v0.3.0)
- ✅ Config management commands (v0.4.0)
- ✅ Enhanced ignore patterns (v0.4.1)

## Completed Phases

Detailed documentation for completed phases is available in the archive:
- [Phase 1 — Hub Validation](archive/phase-1-hub-validation.md)
- [Phase 2 — Hub Registry](archive/phase-2-hub-registry.md)
- [Phase 3 — Skill Management](archive/phase-3-skill-management.md)
- [Phase 4 — Config Management](archive/phase-4-config-management.md)
- [Phase 4.5 — Enhanced Ignore Patterns](archive/phase-4.5-enhanced-ignore.md)

---

## Phase 5 — Enhanced Skill Management

**Goal**: Add advanced skill management features for portability and local development.

### Commands

```
agentctl skill export > skills.lock.json             # export current lock file
agentctl skill import skills.lock.json              # import and install from lock file
agentctl skill outdated                             # list outdated skills
agentctl skill dev <path>                           # link local skill for development
```

### Implementation

**Import/Export**:
- Export copies existing `skills.lock.json` to stdout
- Import installs all skills from provided lock file
- Reproducible installs using existing lock file format

**Local development**:
- Link local skill directories for development
- Hot-reload skill changes without reinstall
- Validate local skills against schema

### New modules

- `src/skill/dev.rs` — local development support
- Extend `src/skill/mod.rs` — export/import and outdated detection

### Steps

1. Implement export/import:
   - Export command outputs current lock file JSON
   - Import command reads lock file and installs all listed skills
   - Handle version conflicts and missing hubs gracefully

2. Add outdated detection:
   - Compare lock file versions with hub index versions
   - List skills with newer versions available

3. Add local development support:
   - Create `src/skill/dev.rs` for local skill linking
   - Support symlinks to local skill directories
   - Add validation for local skills

4. Wire CLI:
   - Extend `SkillAction` enum with new commands
   - Implement dispatch for all new actions
   - Add comprehensive help and examples

5. Add comprehensive tests:
   - Extend `tests/skill_integration.rs` with new features
   - Add export/import round-trip tests
   - Test local development workflow

6. Update documentation:
   - Update `docs/skill-management.md` with new features
   - Add development workflow guide
   - Update README with all skill commands

### Exit criteria

- [ ] `agentctl skill export` outputs current lock file JSON to stdout
- [ ] `agentctl skill import skills.lock.json` installs all skills from lock file
- [ ] `agentctl skill outdated` lists skills with newer versions available
- [ ] `agentctl skill dev <path>` links local skill for development
- [ ] Export/import uses existing lock file format for reproducibility
- [ ] Local development mode with hot-reload capability
- [ ] Enhanced `tests/skill_integration.rs` covering all new features
- [ ] Updated `docs/skill-management.md` with complete feature guide
- [ ] `README.md` updated with all skill management examples
- [ ] `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` pass
- [ ] `CHANGELOG.md` updated, tag `v0.5.0` → release per [release process](docs/release.md)

---

## Future Phases (Deferred)

### Phase 6 — MCP Management (If Relevant)

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
