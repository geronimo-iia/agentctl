---
title: "Phase 5.5 — Hub Refresh Enhancement (Completed)"
summary: "Add --force flag to hub refresh command for bypassing cache entirely - completed in v0.5.1"
read_when:
  - Understanding Phase 5.5 implementation details
  - Reviewing completed force refresh functionality
status: archived
last_updated: "2026-01-16"
---

# Phase 5.5 — Hub Refresh Enhancement (Completed)

**Status**: ✅ Completed in v0.5.1 (2026-01-16)

## Goal

Add --force flag to hub refresh command for bypassing cache entirely.

## Implementation

Simple and effective enhancement that addresses cache staleness issues:

- Add --force flag to hub refresh command
- Force flag deletes cache directory before refresh
- Ensures fresh fetch from remote index URL
- Useful for development and troubleshooting

## Commands Delivered

```bash
agentctl hub refresh --force [<id>]                 # force refresh bypassing cache
```

## Exit Criteria Met

- ✅ `agentctl hub refresh --force` deletes cache and fetches fresh index
- ✅ Works with both single hub and all hubs refresh
- ✅ Tests covering force refresh functionality (`tests/hub_force_refresh_test.rs` with 3 test cases)
- ✅ `README.md` updated with --force flag examples
- ✅ `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` pass
- ✅ `CHANGELOG.md` updated, tag `v0.5.1` released

## Technical Details

### Implementation Files

- `src/cli.rs` — Added `force: bool` field to `HubAction::Refresh`
- `src/main.rs` — Added force parameter handling in refresh command
- `src/hub/registry.rs` — Added `refresh_one_force()` and `refresh_all_force()` functions
- `tests/hub_force_refresh_test.rs` — 3 comprehensive tests

### Test Coverage

1. **Force refresh single hub** — deletes cache and fetches fresh index
2. **Force refresh all hubs** — works with multiple enabled hubs
3. **Nonexistent hub error** — proper error handling for missing hubs

### Code Quality

- All formatting checks pass (`cargo fmt`)
- No clippy warnings (`cargo clippy -- -D warnings`)
- No security vulnerabilities (`cargo audit`)
- 79 total tests pass (45 unit + 34 integration)

## Usage Examples

```bash
# Force refresh specific hub (bypasses cache entirely)
agentctl hub refresh --force agent-skills

# Force refresh all enabled hubs
agentctl hub refresh --force

# Regular refresh (uses TTL-based caching)
agentctl hub refresh agent-skills
```

## Problem Solved

This enhancement addresses the issue where `hub refresh` wasn't sufficient to bypass stale cache when:
- CI pipelines are still updating remote index.json
- Development scenarios require immediate cache invalidation
- Troubleshooting cache-related issues

The `--force` flag provides a reliable way to ensure fresh data by completely removing the cache directory before fetching.

## Next Phase

Phase 6 — Skill Import will add the ability to import and install skills from exported lock files, complementing the export functionality from Phase 5.