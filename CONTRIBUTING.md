# Contributing to WaveHub

Thank you for your interest in contributing.

## Prerequisites

- Rust stable toolchain + `wasm32-unknown-unknown` target
- `stellar-cli` installed

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

## Workflow

1. Fork the repository and create a branch from `main`.
2. Make your changes.
3. Ensure all checks pass:
   ```bash
   make fmt    # auto-format
   make lint   # clippy
   make test   # unit tests
   make build  # WASM build
   ```
4. Update `CHANGELOG.md` under `[Unreleased]`.
5. Open a pull request using the provided template.

## Code style

- All code must pass `cargo fmt` and `cargo clippy -- -D warnings`.
- New behaviour must be covered by tests.
- Panic messages must be lowercase and descriptive (they surface as contract errors).

## Commit messages

Use the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
feat: add withdraw function
fix: prevent double-finalization race
docs: update deploy instructions
```

## Security

Please do **not** open public issues for security vulnerabilities.  
See [SECURITY.md](SECURITY.md) for the responsible disclosure process.
