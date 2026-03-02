# Skill Management

Implementation guide for `agentctl skill` commands based on the [Skill Lifecycle Specification](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/lifecycle.md).

## Commands

### Install

```bash
agentctl skill install <hub_id:slug>
agentctl skill install <hub_id:slug> --as <custom-name>
```

**Implements**: [Available → Installed transition](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/lifecycle.md#available--installed)

**Process**:
1. Fetch hub's cached `index.json`
2. Locate skill entry by slug
3. Sparse-clone hub repo at pinned commit
4. Copy skill directory to `~/.agentctl/skills/<slug>/`
5. Validate `SKILL.md` frontmatter per [authoring-guide.md](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/authoring-guide.md)
6. Check `requires` gate (warn only)
7. Add entry to `skills.lock.json` per [skills-lock.json schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/skills-lock.json)

**Conflict handling**: See [Conflict Resolution](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/lifecycle.md#conflict-resolution)

### Uninstall

```bash
agentctl skill uninstall <hub_id:slug>
agentctl skill uninstall <local-name>
```

**Implements**: [Installed → Removed transition](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/lifecycle.md#installed--removed)

**Process**:
1. Confirm with user
2. Remove directory
3. Remove lock file entry (if hub skill)

### Update

```bash
agentctl skill update <hub_id:slug>
agentctl skill update --all
```

**Implements**: [Installed → Updated transition](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/lifecycle.md#installed--updated)

**Process**:
1. Fetch latest hub `index.json`
2. Compare versions (semver)
3. If newer: replace directory, update lock file

### List

```bash
agentctl skill list
agentctl skill list --hub <hub_id>
agentctl skill list --local
agentctl skill outdated
```

**Shows**:
- Hub skills from lock file
- Local skills from directory scan
- `requires` gate status per skill

### Import/Export

```bash
agentctl skill export > manifest.json
agentctl skill import manifest.json
```

**Enables**: [Portability and Reproducibility](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/lifecycle.md#portability-and-reproducibility)

## Configuration

**Skills directory**: `~/.agentctl/skills/`
**Lock file**: `~/.agentctl/skills.lock.json`
**Hub config**: `~/.agentctl/config.json`

## References

- [Skill Lifecycle Specification](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/lifecycle.md)
- [Skill Definition](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/definition.md)
- [Skill Authoring Guide](https://github.com/geronimo-iia/agent-foundation/blob/main/skills/authoring-guide.md)
- [Skills Lock Schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/skills-lock.json)
