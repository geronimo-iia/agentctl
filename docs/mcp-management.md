# MCP Server Management

Implementation guide for `agentctl mcp` commands for managing Model Context Protocol servers.

## Commands

### Install

```bash
agentctl mcp install <hub_id:server-name>
agentctl mcp install <hub_id:server-name> --as <custom-name>
```

**Purpose**: Install standalone MCP server from hub

**Process**:
1. Fetch hub's cached `index.json` (MCP server hub)
2. Locate server entry by name
3. Download WASM module from URL
4. Verify SHA-256 hash matches index
5. Copy to `~/.agentctl/mcp/<server-name>.wasm`
6. Register server in MCP registry
7. Add entry to `mcp.lock.json`

**Security**: Hash verification ensures integrity, WASM provides sandboxing

### Uninstall

```bash
agentctl mcp uninstall <server-name>
```

**Process**:
1. Confirm with user
2. Unregister from MCP registry
3. Remove WASM file
4. Remove lock file entry

### List

```bash
agentctl mcp list
agentctl mcp list --hub <hub_id>
agentctl mcp list --bundled
```

**Shows**:
- Standalone MCP servers from lock file
- Skill-bundled MCP servers (from installed skills)
- Server capabilities (tools, resources, prompts)
- Server status (active, inactive, error)

### Info

```bash
agentctl mcp info <server-name>
```

**Shows**:
- Server metadata (name, version, description)
- Capabilities provided
- Tools available
- Source (hub or skill-bundled)
- Installation path

### Update

```bash
agentctl mcp update <server-name>
agentctl mcp update --all
```

**Process**:
1. Fetch latest hub `index.json`
2. Compare versions (semver)
3. If newer: download new WASM, verify hash, replace file
4. Update lock file

### Test

```bash
agentctl mcp test <server-name>
```

**Purpose**: Verify MCP server loads and responds correctly

**Process**:
1. Load WASM module
2. Send `initialize` request
3. Send `tools/list` request
4. Report capabilities and status

## MCP Server Sources

### Standalone Servers

**Source**: MCP server hubs (dedicated repositories)
**Index**: `index.json` following [mcp-index.json schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/mcp-index.json)
**Installation**: `agentctl mcp install`
**Location**: `~/.agentctl/mcp/`

### Skill-Bundled Servers

**Source**: Skills with `assets/*.wasm` files
**Index**: Declared in skill's `index.json` via `mcp_servers` array
**Installation**: Automatic during `agentctl skill install`
**Location**: `~/.agentctl/skills/<skill-name>/assets/`

**Note**: Skill-bundled servers are managed via skill lifecycle, not directly via `agentctl mcp` commands.

## Configuration

**MCP directory**: `~/.agentctl/mcp/`
**Lock file**: `~/.agentctl/mcp.lock.json`
**Hub config**: `~/.agentctl/config.json` (mcp_hubs array)
**Registry**: In-memory MCP server registry for runtime

## Lock File Format

```json
{
  "version": "1.0",
  "servers": {
    "python-tools": {
      "hub_id": "official-mcp",
      "name": "python-tools",
      "version": "1.2.0",
      "wasm_url": "https://hub.example.com/python-tools-1.2.0.wasm",
      "wasm_hash": "sha256:abc123...",
      "installed_at": "2025-01-16T10:30:00Z"
    }
  }
}
```

## Security Model

**WASM Sandboxing**: All MCP servers run in WASM sandbox with capability-based security
**Hash Verification**: SHA-256 hash checked on download
**No Network Access**: WASM modules cannot make network calls (unless explicitly granted)
**Filesystem Isolation**: Access only to explicitly granted paths
**Audit Logging**: All tool invocations logged

See [MCP WASM specification](https://github.com/geronimo-iia/agent-foundation/blob/main/tools/mcp-wasm.md) for security details.

## References

- [MCP Protocol Specification](https://github.com/geronimo-iia/agent-foundation/blob/main/tools/mcp-protocol.md)
- [MCP WASM Deployment](https://github.com/geronimo-iia/agent-foundation/blob/main/tools/mcp-wasm.md)
- [MCP Index Schema](https://github.com/geronimo-iia/agent-foundation/blob/main/schemas/mcp-index.json)
- [MCP Execution Policy](https://github.com/geronimo-iia/agent-foundation/blob/main/tools/execution-policy.md)
