# Technology Choice: Rust

## Decision

**agentctl** is implemented in Rust.

## Rationale

### Primary Reasons

**Single Binary Distribution**
- No runtime dependencies (Python, Node.js, etc.)
- Simple installation: download and run
- Easy packaging for system package managers (brew, apt, cargo)

**Performance**
- Fast startup time for CLI commands
- Efficient git operations via libgit2
- Low memory footprint

**Cross-Platform**
- Native compilation for Linux, macOS, Windows
- Consistent behavior across platforms
- Easy cross-compilation

### Technical Benefits

**Strong Ecosystem**
- `clap`: Excellent CLI argument parsing with derive macros
- `serde`: Robust JSON/TOML serialization
- `tokio`: Async runtime for network operations
- `git2`: Rust bindings for libgit2

**Type Safety**
- Compile-time error detection
- No runtime type errors
- Clear error handling with Result types

**Maintainability**
- Explicit error handling
- Strong type system prevents bugs
- Cargo ecosystem for dependencies

## Alternative Considered: Python

**Pros**:
- Could reuse agent-hub-indexer validation code
- Faster prototyping
- Same toolchain as agent-hub-indexer

**Cons**:
- Requires Python runtime on user machines
- Slower startup time
- More complex distribution (pip, uv, system packages)

**Decision**: Different tools, different audiences
- **agent-hub-indexer**: CI tool for hub maintainers (Python acceptable)
- **agentctl**: End-user CLI tool (Rust better fit)

## Implementation Standards

Follow [agent-software/rust](https://github.com/geronimo-iia/agent-software/tree/main/rust) specifications:
- [Project Structure & Tools](https://github.com/geronimo-iia/agent-software/blob/main/rust/project-structure-tools.md)
- [CI/CD](https://github.com/geronimo-iia/agent-software/blob/main/rust/ci-cd.md)

## Key Dependencies

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }      # CLI parsing
serde = { version = "1.0", features = ["derive"] }     # Serialization
serde_json = "1.0"                                     # JSON handling
toml = "0.8"                                           # TOML config
git2 = "0.18"                                          # Git operations
anyhow = "1.0"                                         # Error handling
reqwest = { version = "0.11", features = ["json"] }    # HTTP client
tokio = { version = "1.0", features = ["full"] }       # Async runtime
```

## Project Structure

Binary project with workspace organization:
```
agentctl/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point
│   ├── skill/            # Skill management
│   ├── hub/              # Hub management
│   └── common/           # Shared utilities
├── tests/
└── docs/
```

See [rust/project-structure-tools.md](https://github.com/geronimo-iia/agent-software/blob/main/rust/project-structure-tools.md) for details.
