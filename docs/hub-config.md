---
title: "Hub Configuration"
summary: "agentctl.toml format spec and index cache design for hub maintainers."
read_when:
  - Setting up a hub repository
  - Customising hub ID or ignore patterns
  - Understanding how the index cache works
status: draft
last_updated: "2026-07-15"
---

# Hub Configuration

## agentctl.toml

Optional file at the hub root. When present, values are used as defaults; CLI flags always take precedence.

```toml
[hub]
id = "agent-foundation"  # overrides --hub-id on generate

[generate]
# Replaces the default exclusion list when present.
# Matched case-insensitively against filename only (not path).
# Supports a single * wildcard anywhere in the pattern.
ignore = [
  "README.md",
  "CHANGELOG.md",
  "CONTRIBUTING.md",
  "ARCHIVED.md",
  "draft-*.md",
]
```

### Default exclusions

Applied when `agentctl.toml` is absent or has no `[generate] ignore` key:

```
README.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE*, ARCHIVED.md
```

Exclusions apply to both `hub validate` and `hub generate`.

### Precedence

```
CLI --hub-id  >  agentctl.toml [hub] id  >  "default"
CLI --output  >  agentctl.toml (not applicable)
```

## Index cache

Filesystem-based TTL cache — no daemon, no background process.

**Location:** `~/.agentctl/cache/hubs/<id>/`

```
~/.agentctl/cache/hubs/<id>/
├── index.json     # cached hub index
└── fetched_at     # RFC3339 timestamp of last successful fetch
```

**TTL behaviour:**

1. Read `fetched_at` — if missing or age > `ttl_hours`, fetch `index_url` and overwrite both files
2. Fetch fails + `index.json` exists → use stale cache, print warning to stderr
3. Fetch fails + no cache → error out

**Default TTL:** 6 hours (configurable per hub in `~/.agentctl/config.json`).

**Force refresh:**

```bash
agentctl hub refresh <id>   # refresh one hub
agentctl hub refresh        # refresh all enabled hubs
```

## Config file

`~/.agentctl/config.json` stores registered hubs:

```json
{
  "skill_hubs": [
    {
      "id": "agent-foundation",
      "index_url": "https://raw.githubusercontent.com/geronimo-iia/agent-foundation/main/skills/index.json",
      "git_url": "https://github.com/geronimo-iia/agent-foundation",
      "enabled": true,
      "ttl_hours": 12
    }
  ],
  "doc_hubs": []
}
```

Override the config path with `--config`:

```bash
agentctl --config /path/to/config.json hub list
```
