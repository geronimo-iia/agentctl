# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-07-15

### Added

- `agentctl skill install <name> [--hub <id>] [--mode <mode>]` — install skill from registered hub
- `agentctl skill list` — list installed skills from lock file
- `agentctl skill remove <name> --hub <id>` — run uninstall lifecycle, remove dir, update lock
- `agentctl skill update [<name>] [--hub <id>] [--force]` — update to latest version; errors if no `update` lifecycle section unless `--force` (reinstalls via uninstall+install)
- `--quiet` / `-q` global flag — suppress all step output, implies `--yes`
- `--yes` / `-y` global flag — auto-approve all `requires_approval` lifecycle steps
- `--lock <path>` global flag — override lock file path (default: `~/.agentctl/skills.lock.json`)
- `src/skill/mod.rs` — install/list/remove/update with git clone and dir copy
- `src/skill/lifecycle.rs` — `execute_lifecycle` and `execute_update` with injectable `Approver`/`Executor`
- `src/skill/vars.rs` — `${VAR}` expansion with built-ins (`SKILL_NAME`, `SKILL_PATH`, `HOME`, `PLATFORM`) and custom vars
- `src/skill/lock.rs` — `skills.lock.json` read/write
- `tests/common/mod.rs` — shared test helpers (`agentctl`, `fixture`, `with_config`, `with_lock`, `with_config_and_lock`)
- `tests/skill_integration.rs` — 11 integration tests
- 45 unit + 14 hub integration + 11 skill integration = 70 tests total

## [0.2.0] - 2026-07-15

### Added

- `agentctl hub add/list/remove/enable/disable` — local hub registry in `~/.agentctl/config.json`
- `agentctl hub refresh [<id>]` — force-refresh one or all enabled hub caches
- `--config <path>` global flag to override config file location
- `agentctl.toml` per-hub config — `[hub] id` and `[generate] ignore` patterns
- Filesystem TTL cache at `~/.agentctl/cache/hubs/<id>/` with stale-on-failure fallback
- `src/config.rs` — `Config::load_from` / `save_to` with fixture-based tests
- `src/hub/cache.rs` — `get_from` / `refresh_to` with injectable `Fetcher` for mock testing
- `src/hub/registry.rs` — registry operations with `refresh_one_with` / `refresh_all_with`
- `dirs` dependency for platform-correct home directory resolution
- 28 unit tests + 13 integration tests (up from 10)
- `docs/hub-config.md` — `agentctl.toml` format spec and cache design
- `agentctl.toml` committed to `agent-foundation` and `agent-skills` repos

## [0.1.0] - 2026-07-14

### Added

- `agentctl hub validate --type <skills|docs> --path <dir>` — validate hub files against schema
- `agentctl hub generate --type <skills|docs> --path <dir> --output index.json` — generate index.json
- Structured error messages with file path and line number on validation failures
- Hidden directory filtering (`.git`, etc.) in skills hub validation
- Flat hierarchy enforcement for skills (one level deep)
- Git commit hash per entry via libgit2
- Exit code 0/1 for CI use
- Cross-platform release binaries: Linux x86_64/ARM64, macOS x86_64/ARM64, Windows x86_64
- 10 tests: unit + integration, all using local fixtures
- Dependabot configured for Cargo and GitHub Actions (weekly)

### Fixed

- Clippy `bool_comparison` warning in hidden directory filter (`== false` → `!`)
