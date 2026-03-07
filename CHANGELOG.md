# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
