# agentctl

CLI for agent hub validation, index generation, and skill management.
Implements the hub formats defined in [agent-foundation](https://github.com/geronimo-iia/agent-foundation).

See [docs/roadmap.md](docs/roadmap.md) for current development plans and upcoming features.

## Install

**Homebrew** (macOS/Linux):
```bash
brew tap geronimo-iia/agent
brew install agentctl
```

**cargo-binstall** (installs pre-built binary):
```bash
cargo binstall agent-ctl
```

**cargo** (builds from source):
```bash
cargo install agent-ctl
```

**Binary download**: grab the latest release from [GitHub Releases](https://github.com/geronimo-iia/agentctl/releases).

## Quickstart

Validate a hub directory:
```bash
agentctl hub validate --type docs --path ./my-hub
agentctl hub validate --type skills --path ./my-skills
```

Generate `index.json`:
```bash
agentctl hub generate --type docs --path ./my-hub --output index.json
agentctl hub generate --type skills --path ./my-skills --hub-id my-hub --output index.json
```

## Usage

```
agentctl --help
agentctl hub --help
```

### Validation output

On success:
```
✓ Validation passed
```

On failure (exit code 1):
```
  ✗ skills/my-skill/SKILL.md:1: missing required field: name
  ✗ skills/my-skill/SKILL.md:1: missing required field: description
✗ Validation failed (2 error(s))
```

### Generated index.json (skills)

```json
{
  "hub_id": "my-hub",
  "generated_at": "2026-07-14T10:00:00Z",
  "skills": [
    {
      "name": "python-scaffold",
      "description": "Scaffold a Python project",
      "path": "python-scaffold",
      "commit_hash": "abc1234"
    }
  ]
}
```

## Config Management

Inspect and modify `~/.agentctl/config.json` without editing JSON by hand.

```bash
# create a default config file (errors if already exists)
agentctl config init
agentctl config init --force   # overwrite existing

# print the config file path
agentctl config path

# print the full config as pretty JSON
agentctl config show

# get / set scalar values
agentctl config get skills_root
agentctl config set skills_root ~/.agent/skills
```

**Supported keys**: `skills_root` — overrides the default skill install root. Hub entries are managed via `hub add/remove/enable/disable`.

## Hub Registry

Register and manage hub sources in `~/.agentctl/config.json`:

```bash
agentctl hub add --type skills agent-foundation https://raw.githubusercontent.com/geronimo-iia/agent-foundation/main/skills/index.json
agentctl hub add --type docs agent-foundation https://raw.githubusercontent.com/geronimo-iia/agent-foundation/main/docs/index.json
agentctl hub list
agentctl hub disable agent-foundation
agentctl hub enable agent-foundation
agentctl hub remove agent-foundation
agentctl hub refresh agent-foundation   # force-refresh one hub
agentctl hub refresh --force agent-foundation  # bypass cache entirely
agentctl hub refresh                    # refresh all enabled hubs
agentctl hub refresh --force            # force refresh all hubs
```

## Skill Management

Install, list, remove, and update skills from registered hubs.

```bash
# install from the first enabled skill hub
agentctl skill install python-scaffold

# install from a specific hub
agentctl skill install python-scaffold --hub agent-foundation

# install into a named mode (path: ~/.agent/skills-dev/)
agentctl skill install python-scaffold --mode dev

# list installed skills
agentctl skill list

# remove a skill
agentctl skill remove python-scaffold --hub agent-foundation

# update a skill (errors if no update lifecycle — use --force to reinstall)
agentctl skill update python-scaffold
agentctl skill update python-scaffold --force

# update all installed skills
agentctl skill update

# export current skill installations for backup/sharing
agentctl skill export > skills.lock.json
```

**Global flags:**
- `-q` / `--quiet` — suppress all step output; implies `--yes`
- `-y` / `--yes` — auto-approve all `requires_approval` lifecycle steps

Skills are installed to `~/.agent/skills/<name>/` and tracked in `~/.agentctl/skills.lock.json`.
Each skill may include a `lifecycle.yaml` with `install`, `update`, and `uninstall` sections.

## agentctl.toml

Optional file at the hub root. When present, values are used as defaults; CLI flags always take precedence.

```toml
[hub]
id = "agent-foundation"  # overrides --hub-id on generate

[generate]
# Replaces the default exclusion list when present.
# Supports filename patterns, directory patterns, and path wildcards.
ignore = [
  "README.md",              # exact filename match
  "CHANGELOG.md",
  "CONTRIBUTING.md",
  "ARCHIVED.md",
  "draft-*.md",             # filename wildcard
  "rules/templates/",       # directory pattern (excludes all files in directory)
  "docs/internal/*.md",     # path wildcard (specific pattern in directory)
]
```

**Pattern types:**
- **Filename patterns**: `"README.md"` (exact), `"draft-*.md"` (wildcard) — matched case-insensitively at any path level
- **Directory patterns**: `"templates/"`, `"rules/templates/"` — excludes all files in the specified directory
- **Path wildcards**: `"rules/*.md"`, `"docs/internal/*.md"` — excludes files matching the pattern in the specific directory path

When `agentctl.toml` is absent or has no `[generate] ignore` key, the default exclusions apply: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `LICENSE*`, `ARCHIVED.md`.

See [docs/hub-config.md](docs/hub-config.md) for full details including cache layout and TTL behaviour.

## CI Integration

Example GitHub Actions workflow for a docs or skills hub:

```yaml
name: Publish Index

on:
  push:
    branches: [main]

jobs:
  publish:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4

      - name: Install agentctl
        run: |
          curl -sSL https://github.com/geronimo-iia/agentctl/releases/latest/download/x86_64-unknown-linux-gnu.tar.gz | tar xz -C /usr/local/bin

      - name: Validate
        run: agentctl hub validate --type docs --path .

      - name: Generate index.json
        run: agentctl hub generate --type docs --path . --output index.json

      - name: Commit index.json
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add index.json
          git diff --staged --quiet || git commit -m "chore: regenerate index.json [skip ci]"
          git push
```

Replace `--type docs` with `--type skills` for a skills hub.

## License

MIT OR Apache-2.0
