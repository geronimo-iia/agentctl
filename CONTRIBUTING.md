# Contributing

## Prerequisites

- Rust stable (`rustup install stable`)
- `cargo audit` (`cargo install cargo-audit`)

## Development

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt
```

## Commit format

Follow [semantic commits](https://www.conventionalcommits.org/):
```
feat(hub): add validate command
fix(generate): correct commit hash for nested paths
docs: update README install instructions
```

## Pull requests

- One concern per PR
- All CI checks must pass (`fmt`, `clippy`, `test`, `audit`)
- Update `CHANGELOG.md` under `[Unreleased]`

## Releases

For maintainers releasing new versions, follow the complete [release process](docs/release.md).
