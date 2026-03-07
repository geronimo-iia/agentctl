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
