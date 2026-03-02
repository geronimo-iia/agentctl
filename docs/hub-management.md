# Hub Management

Implementation guide for `agentctl hub` commands based on [Skill Hub](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/hub.md) and [Doc Hub](https://github.com/geronimo-iia/agent-foundation/blob/main/docs/hub.md) specifications.

## Commands

### Add Hub

```bash
agentctl hub add <hub_id> <index_url> [--git-url <url>]
```

**Adds hub to configuration**:
```json
{
  "id": "hub_id",
  "index_url": "https://example.com/index.json",
  "git_url": "https://github.com/org/hub",
  "enabled": true,
  "ttl_hours": 6
}
```

**Hub types**: Auto-detected from `index.json` `type` field (`skills` or `docs`)

### List Hubs

```bash
agentctl hub list
```

**Shows**: All configured hubs with status (enabled/disabled)

### Enable/Disable

```bash
agentctl hub enable <hub_id>
agentctl hub disable <hub_id>
```

**Effect**: Controls whether hub is included in search/install operations

### Remove Hub

```bash
agentctl hub remove <hub_id>
```

**Process**:
1. Confirm with user
2. Remove from configuration
3. Clear cached `index.json`
4. Optionally remove installed skills from this hub

### Validate Hub

```bash
agentctl hub validate --type <skills|docs> --path <hub-directory>
```

**Validates**:
- Skills: `SKILL.md` frontmatter per [authoring-guide.md](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/authoring-guide.md)
- Docs: Frontmatter per [authoring-guide.md](https://github.com/geronimo-iia/agent-foundation/blob/main/docs/authoring-guide.md)

### Generate Index

```bash
agentctl hub generate --type <skills|docs> --path <hub-directory> --output index.json
```

**Generates**:
- Skills: Per [skills-index.json schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/skills-index.json)
- Docs: Per [docs-index.json schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/docs-index.json)

**Naming convention**: See [hub.md](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/hub.md) - use `skills.json`/`docs.json` for mixed repos

## Index Caching

**Cache location**: `~/.agentctl/cache/hubs/<hub_id>/index.json`
**TTL**: Configurable per hub (default 6 hours)
**Refresh**: Automatic on TTL expiry or manual via `agentctl hub refresh <hub_id>`

## Configuration

**Hub config**: `~/.agentctl/config.json`

**Schema**: [agentctl-config.json](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/agentctl-config.json)

**Specification**: [Hub Configuration](https://github.com/geronimo-iia/agent-foundation/blob/main/docs/hub-configuration.md)

```json
{
  "skill_hubs": [
    {
      "id": "official",
      "index_url": "https://skills.example.com/index.json",
      "git_url": "https://github.com/org/skills",
      "enabled": true,
      "ttl_hours": 6
    }
  ],
  "doc_hubs": [
    {
      "id": "team-docs",
      "index_url": "https://docs.example.com/index.json",
      "git_url": "https://github.com/org/docs",
      "enabled": true,
      "ttl_hours": 24
    }
  ]
}
```

## References

- [Skill Hub Specification](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/hub.md)
- [Doc Hub Specification](https://github.com/geronimo-iia/agent-foundation/blob/main/docs/hub.md)
- [Hub Configuration](https://github.com/geronimo-iia/agent-foundation/blob/main/docs/hub-configuration.md)
- [Hub Config Schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/agentctl-config.json)
- [Skills Index Schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/skills-index.json)
- [Docs Index Schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/docs-index.json)
- [Skills Lock Schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/skills-lock.json)
