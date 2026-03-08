---
title: "agentctl Roadmap"
summary: "Phased implementation plan for agentctl, the end-user CLI for agent hub management."
read_when:
  - Planning agentctl development
  - Deciding what to implement next
  - Understanding scope and sequencing
status: draft
last_updated: "2026-07-15"
---

# agentctl Roadmap

## Goal

Ship a first version of `agentctl` that covers hub validation and index generation before publishing `agent-foundation`, `agent-software`, and skill hubs. This unblocks hub maintainers and establishes the CLI contract early.

---

## Phase 1 — Hub Validate & Generate (MVP) ✅ DONE

**Goal**: Parity with `agent-hub-indexer` as a Rust binary. This is the gate before publishing hubs.

### Commands

```
agentctl --help
agentctl hub --help
agentctl hub validate --type <skills|docs> --path <dir>
agentctl hub generate --type <skills|docs> --path <dir> --output index.json
```

`--help` is generated automatically by `clap` from doc comments on every command and argument. All structs and fields in `src/cli.rs` must have doc comments.

### Schemas

Implementation must conform to the JSON schemas in `agent-foundation/schemas/`:

- [`skills-index.json`](../../agent-foundation/schemas/skills-index.json) — output schema for `hub generate --type skills`
- [`docs-index.json`](../../agent-foundation/schemas/docs-index.json) — output schema for `hub generate --type docs`
- [`agentctl-config.json`](../../agent-foundation/schemas/agentctl-config.json) — config schema (used from Phase 2)
- [`skills-lock.json`](../../agent-foundation/schemas/skills-lock.json) — lock file schema (used from Phase 3)

`src/hub/schema.rs` types must match these schemas exactly.

### Scope

- Parse SKILL.md YAML frontmatter — validate `name`, `description` required fields
- Parse .md YAML frontmatter — validate `title`, `summary`, `status`, `last_updated`, `read_when` required fields
- Flat hierarchy enforcement for skills (one level deep)
- Generate `index.json` matching current schema (type, version, entries, metadata)
- Git commit hash per entry via `git2`
- Exit code 0/1 for CI use
- `--version` flag (wired from `Cargo.toml` version via `clap`)
- Structured error messages with file path + line number on validation failures
- All CLI commands and arguments have doc comments (`///`) for `--help` output

### Project structure

Follows [agent-software/rust/project-structure-tools.md](../../agent-software/rust/project-structure-tools.md) binary project layout:

```
agentctl/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
├── rustfmt.toml
├── clippy.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   └── hub/
│       ├── mod.rs
│       ├── schema.rs
│       ├── validate.rs
│       └── generate.rs
└── tests/
    └── hub_integration.rs
```

### Cargo.toml

```toml
[package]
name = "agent-ctl"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"

[[bin]]
name = "agentctl"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
git2 = "0.18"
anyhow = "1.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### CI

Follows [agent-software/rust/ci-cd.md](../../agent-software/rust/ci-cd.md):

- `.github/workflows/ci.yml` — `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo audit`
- `.github/workflows/release.yml` — tag-triggered cross-platform builds, GitHub release with binary artifacts

**Release targets**:

| OS | Architecture | Target |
|---|---|---|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Linux | ARM64 | `aarch64-unknown-linux-gnu` |
| macOS | x86_64 | `x86_64-apple-darwin` |
| macOS | ARM64 (Apple Silicon) | `aarch64-apple-darwin` |
| Windows | x86_64 | `x86_64-pc-windows-msvc` |

Linux ARM64 cross-compilation requires `cross` (Docker-based) or `cross-rs/cross` GitHub Action.

### Exit criteria

- [x] `agentctl hub validate` and `agentctl hub generate` pass on `agent-foundation`
- [x] `agentctl --help`, `agentctl hub --help`, `agentctl --version` all produce correct output
- [x] 10 tests pass: 1 unit (`hidden_dirs_are_ignored`) + 9 integration (`version_flag`, `help_flag`, `hub_help_flag`, `validate_skills_hub_valid`, `validate_skills_hub_rejects_bad_frontmatter`, `validate_skills_hub_ignores_git_dir`, `validate_docs_hub_valid`, `validate_docs_hub_rejects_missing_fields`, `generate_docs_index`) — all using local fixtures in `tests/fixtures/`
- [x] `cargo fmt`, `cargo clippy -- -D warnings` pass
- [x] `agent-foundation/.github/workflows/publish.yml` updated to use `agentctl`
- [x] `agent-hub-indexer` archived under `archive/agent-hub-indexer`
- [x] `CHANGELOG.md`, `README.md`, `CONTRIBUTING.md` written
- [x] `LICENSE-MIT`, `LICENSE-APACHE` (Jerome Guibert, 2026)
- [x] CI workflows: `ci.yml`, `release.yml` with all 5 targets
- [x] `cargo audit` passes with no vulnerabilities (124 dependencies scanned)
- [x] `hub validate --type skills` tested with local fixtures (valid + invalid + `.git` ignored)
- [x] Create GitHub repo, push code (`git@github.com:geronimo-iia/agentctl.git`)
- [x] Add `CARGO_TOKEN` secret (crates.io → Account Settings → API Tokens)
- [x] CI passes on `main` (fmt, clippy, tests, audit)
- [x] Tag `v0.1.0` → triggers release workflow → binaries + crates.io publish
- [x] Create `homebrew-agent` repo with `agentctl.rb` (`brew tap geronimo-iia/agent`)

### homebrew-agent setup

Requires the `v0.1.0` GitHub release to exist first (tarball SHA256 is only available post-release).

**1. Get the SHA256**

```sh
curl -sL https://github.com/<user>/agentctl/archive/refs/tags/v0.1.0.tar.gz | shasum -a 256
```

**2. Create the repo**

- Repo name: `homebrew-agent` (enables `brew tap <user>/agent`)
- Public repo, no CI needed

**3. Create `Formula/agentctl.rb`**

```ruby
class Agentctl < Formula
  desc "CLI for agent hub validation, index generation, and skill management"
  homepage "https://github.com/<user>/agentctl"
  url "https://github.com/<user>/agentctl/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "<sha256-from-step-1>"
  license "MIT OR Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/agentctl --version")
  end
end
```

**4. Install and verify**

```sh
brew tap <user>/agent
brew install agentctl
agentctl --version  # should print 0.1.0
```

**5. Update roadmap exit criterion** — mark this item `[x]` and record the tap URL in `README.md`.

---

## Phase 2 — Hub Registry (add/list/remove)

**Goal**: Local hub registry so users can register and manage hub sources.

### Commands

```
agentctl hub add <id> <index_url> [--git-url <url>]
agentctl hub list
agentctl hub remove <id>
agentctl hub enable <id>
agentctl hub disable <id>
agentctl hub refresh [<id>|--all]
```

### Scope

- Per-hub `agentctl.toml` config file — `hub_id`, ignore patterns, overrides CLI flags when present
- Config file at `~/.agentctl/config.json` (skill_hubs + doc_hubs arrays)
- Index cache at `~/.agentctl/cache/hubs/<id>/index.json` with TTL (default 6h)
- Auto-detect hub type from `index.json` `type` field
- HTTP fetch via `ureq` (synchronous — no async runtime needed for a CLI tool)
- Stale cache used with warning when network unavailable
- `hub refresh` with no args or `--all` refreshes all enabled hubs

### agentctl.toml (per-hub)

Optional file at hub root. CLI flags take precedence over file values.

```toml
[hub]
id = "agent-foundation"  # overrides --hub-id

[generate]
# Replaces default exclusion list when present
ignore = [
  "README.md",
  "CHANGELOG.md",
  "CONTRIBUTING.md",
  "ARCHIVED.md",
  "draft-*.md",
]
```

**Default exclusions** (applied when no `agentctl.toml` or no `[generate] ignore` key):
```
README.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE*, ARCHIVED.md
```

Matching is case-insensitive on filename only. Applies to both validate and generate.

### Index cache design

Filesystem-based TTL — no daemon, no background process.

**Layout:**
```
~/.agentctl/cache/hubs/<id>/
├── index.json     # cached hub index
└── fetched_at     # RFC3339 timestamp of last successful fetch
```

**On any command that needs the index** (search, install, refresh):
1. Read `fetched_at` — if missing or age > `ttl_hours`, fetch `index_url` and overwrite both files
2. Fetch fails + `index.json` exists → use stale cache, print warning to stderr
3. Fetch fails + no cache → error out

**`hub refresh [<id>|--all]`** — forces fetch regardless of TTL

**`hub add`** — fetches immediately to validate `index_url` is reachable and `type` field is valid

### New modules

- `src/hub/config.rs` — read `agentctl.toml` from hub root (hub_id, ignore patterns)
- `src/config.rs` — read/write `~/.agentctl/config.json`
- `src/hub/registry.rs` — add/list/remove/enable/disable
- `src/hub/cache.rs` — TTL-based index caching

Add to `Cargo.toml`: `ureq = { version = "2", features = ["json"] }`, `toml = "0.8"`

**Rationale**: `ureq` is synchronous, no `tokio` runtime, compiles fast, keeps binary small. A CLI tool runs one command and exits — async adds no value here.

### Exit criteria

- [x] `agentctl hub add/list/remove/enable/disable` implemented and tested
- [x] `agentctl hub refresh [<id>|--all]` implemented — refreshes one or all enabled hubs
- [x] Stale cache used with warning when network unavailable
- [x] `agentctl.toml` read at hub root — `hub_id` and `ignore` respected by validate + generate
- [x] Default exclusion list applied when no `agentctl.toml`
- [x] CLI flags take precedence over `agentctl.toml` values
- [x] `~/.agentctl/config.json` read/write working
- [x] `--config` flag added for test isolation and explicit config path override
- [x] No dead code — `Config::load`/`save` wrappers removed, all `#[allow(dead_code)]` eliminated
- [x] Index cache with TTL at `~/.agentctl/cache/hubs/<id>/index.json`
- [x] Tests for registry commands, `agentctl.toml` loading, default exclusions, CLI override
- [x] 28 unit tests + 14 integration tests passing
- [x] `docs/hub-config.md` — `agentctl.toml` format spec and cache design
- [x] `README.md` updated with `agentctl.toml` section and example
- [x] Example `agentctl.toml` committed to `agent-foundation` and `agent-skills` repos
- [x] `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` pass
- [x] `CHANGELOG.md` updated, tag `v0.2.0` → release

---

## Phase 3 — Skill Management

**Goal**: Install, list, and remove skills from registered hubs.

### Commands

```
agentctl skill search <query>
agentctl skill install <name> [--hub <id>] [--mode <mode>]
agentctl skill list
agentctl skill remove <name>
agentctl skill update [<name>]
```

### Scope

- Search across cached hub indexes
- Install to `~/.agent/skills/` or `~/.agent/skills-{mode}/`
- Lock file at `~/.agentctl/skills.lock` (name, version, hub, commit_hash, path)
- Git clone or sparse checkout for install
- `lifecycle.yaml` execution with user approval prompt

---

## Phase 4 — Doc Hub & MCP Management

Covered in [hub-management.md](hub-management.md), [mcp-management.md](mcp-management.md), [skill-management.md](skill-management.md).

---

## Sequencing Rationale

Phase 1 is the only hard dependency before publishing hubs — it replaces `agent-hub-indexer` for local use and CI. Phases 2–4 can follow independently once hubs are live.

`agentctl` replaces `agent-hub-indexer` entirely — for local use, CI, and hub publishing. `agent-hub-indexer` is archived under `archive/agent-hub-indexer`.

## Standards

- Project structure: [agent-software/rust/project-structure-tools.md](../../agent-software/rust/project-structure-tools.md)
- CI/CD: [agent-software/rust/ci-cd.md](../../agent-software/rust/ci-cd.md)
- Commits: [agent-software/version-control-release/git-commit-semantic.md](../../agent-software/version-control-release/git-commit-semantic.md)
- Versioning: [agent-software/version-control-release/semantic-versioning.md](../../agent-software/version-control-release/semantic-versioning.md)
