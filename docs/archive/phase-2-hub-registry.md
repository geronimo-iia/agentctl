---
title: "Phase 2 — Hub Registry"
summary: "Completed implementation of local hub registry with add/list/remove/enable/disable commands."
status: archived
archived_date: "2026-07-16"
archived_reason: "Phase completed successfully, functionality integrated into agentctl v0.2.0"
---

# Phase 2 — Hub Registry ✅ COMPLETED

**Goal**: Local hub registry so users can register and manage hub sources.

## Commands Implemented

```
agentctl hub add <id> <index_url> [--git-url <url>]
agentctl hub list
agentctl hub remove <id>
agentctl hub enable <id>
agentctl hub disable <id>
agentctl hub refresh [<id>|--all]
```

## Key Features Delivered

- Per-hub `agentctl.toml` config file with hub_id and ignore patterns
- Config file at `~/.agentctl/config.json` for hub registry
- Index cache at `~/.agentctl/cache/hubs/<id>/` with TTL (6h default)
- Auto-detect hub type from `index.json` type field
- HTTP fetch via `ureq` with stale cache fallback
- `--config` flag for test isolation and explicit config override

## Architecture

- `src/config.rs` — read/write `~/.agentctl/config.json`
- `src/hub/config.rs` — read `agentctl.toml` from hub root
- `src/hub/registry.rs` — hub registry operations
- `src/hub/cache.rs` — TTL-based index caching with stale fallback
- `tests/hub_integration.rs` — 14 integration tests

## agentctl.toml Format

```toml
[hub]
id = "agent-foundation"

[generate]
ignore = [
  "README.md",
  "draft-*.md",
]
```

## Cache Design

Filesystem-based TTL with no daemon:
- `~/.agentctl/cache/hubs/<id>/index.json` — cached hub index
- `~/.agentctl/cache/hubs/<id>/fetched_at` — RFC3339 timestamp
- Stale cache used with warning when network unavailable

## Release Artifacts

- **Version**: v0.2.0
- **Tests**: 28 unit + 14 integration tests
- **Documentation**: `docs/hub-config.md` with format specification
- **Examples**: `agentctl.toml` committed to agent-foundation and agent-skills

## Impact

- Enabled users to register and manage multiple hub sources
- Established caching infrastructure for offline operation
- Created foundation for skill installation from registered hubs
- Introduced per-hub configuration with ignore patterns

This phase successfully established the hub registry system and caching infrastructure needed for skill management.