# agentctl

CLI for agent hub validation, index generation, and skill management.
Implements the hub formats defined in [agent-foundation](https://github.com/geronimo-iia/agent-foundation).

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
agentctl hub refresh                    # refresh all enabled hubs
```

## agentctl.toml

Optional file at the hub root to set the hub ID and ignore patterns:

```toml
[hub]
id = "agent-foundation"

[generate]
ignore = [
  "README.md",
  "CHANGELOG.md",
  "CONTRIBUTING.md",
  "ARCHIVED.md",
  "draft-*.md",
]
```

CLI flags take precedence over `agentctl.toml` values. See [docs/hub-config.md](docs/hub-config.md) for full details.

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
