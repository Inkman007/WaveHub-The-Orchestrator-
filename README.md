# WaveHub — The Orchestrator

[![CI](https://github.com/your-org/WaveHub-The-Orchestrator/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/WaveHub-The-Orchestrator/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar-7B2FBE?logo=stellar)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Smart%20Contract-Soroban-blueviolet)](https://soroban.stellar.org)

> **WaveHub** is an administrative smart contract deployed on the **Stellar** blockchain via the **Soroban** smart contract platform. It defines and manages reward cycles called *Waves* — each Wave is a fixed time window during which a reward pool is reserved for future distribution.

---

## Table of contents

- [Overview](#overview)
- [How it works](#how-it-works)
- [Architecture](#architecture)
- [Data model](#data-model)
- [Contract interface](#contract-interface)
- [Error reference](#error-reference)
- [File structure](#file-structure)
- [Prerequisites](#prerequisites)
- [Quickstart](#quickstart)
- [Building](#building)
- [Testing](#testing)
- [Deployment](#deployment)
- [Invoking the contract](#invoking-the-contract)
- [Scripts](#scripts)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)
- [Future improvements](#future-improvements)

---

## Overview

WaveHub is built with [Soroban](https://soroban.stellar.org) — Stellar's native smart contract platform — and compiled to **WebAssembly (WASM)**. It runs entirely on-chain on the Stellar network.

Key responsibilities:

- Record start/end timestamps and pool size for each Wave
- Restrict Wave creation and finalization to the contract owner
- Emit on-chain events on Wave creation and finalization
- Return typed, numeric error codes for all failure paths

---

## How it works

```
┌─────────────────────────────────────────────────────────────┐
│                        Stellar Network                       │
│                                                             │
│   Owner Keypair                                             │
│       │                                                     │
│       │  stellar contract invoke                            │
│       ▼                                                     │
│  ┌─────────────┐    require_auth()    ┌──────────────────┐  │
│  │   Caller    │ ──────────────────▶  │  WaveHub Contract │  │
│  └─────────────┘                      │                  │  │
│                                       │  validate inputs │  │
│                                       │       │          │  │
│                                       │       ▼          │  │
│                                       │  write to ledger │  │
│                                       │       │          │  │
│                                       │       ▼          │  │
│                                       │  emit WaveNew    │  │
│                                       │  return wave_id  │  │
│                                       └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## Architecture

The contract is split into focused modules:

```
src/
├── lib.rs          ← crate root; re-exports and test suite
├── contract.rs     ← #[contractimpl] — all public entry points
├── types.rs        ← Wave struct and DataKey storage enum
├── errors.rs       ← WaveHubError typed error enum
└── storage.rs      ← typed helpers wrapping env.storage()
```

### Storage layout

| Key | Storage tier | Type | Description |
|---|---|---|---|
| `"OWNER"` | `instance` | `Address` | Contract owner |
| `"WCOUNT"` | `instance` | `u64` | Running wave counter |
| `DataKey::Wave(id)` | `persistent` | `Wave` | Wave data by ID |

`instance` storage lives as long as the contract. `persistent` storage survives ledger archival and is the correct tier for long-lived Wave records.

---

## Data model

### Wave

```rust
// contracts/wavehub/src/types.rs

#[contracttype]
pub struct Wave {
    pub total_pool: i128,   // token amount reserved for this Wave
    pub start_time: u64,    // Stellar ledger timestamp at creation
    pub end_time:   u64,    // start_time + duration (seconds)
    pub finalized:  bool,   // true once the Wave has been closed
}
```

### DataKey

```rust
#[contracttype]
pub enum DataKey {
    Wave(u64),  // keyed by wave ID
}
```

### WaveHubError

```rust
// contracts/wavehub/src/errors.rs

#[contracterror]
#[repr(u32)]
pub enum WaveHubError {
    AlreadyInitialized = 1,
    NotInitialized     = 2,
    Unauthorized       = 3,
    InvalidDuration    = 4,
    InvalidPool        = 5,
    WaveNotFound       = 6,
    AlreadyFinalized   = 7,
    DurationOverflow   = 8,
}
```

`#[contracterror]` serialises each variant as its `u32` discriminant on-chain, making errors inspectable from any Stellar SDK or the CLI.

---

## Contract interface

```rust
// contracts/wavehub/src/contract.rs

impl WaveHubContract {
    /// Set the contract owner. Can only be called once.
    pub fn initialize(env: Env, owner: Address) -> Result<(), WaveHubError>;

    /// Create a new Wave. `duration` is in seconds; `total_pool` is in stroops.
    /// Returns the new wave ID (1-indexed, auto-incrementing).
    pub fn create_wave(
        env: Env,
        caller: Address,
        duration: u64,
        total_pool: i128,
    ) -> Result<u64, WaveHubError>;

    /// Mark a Wave as finalized. Idempotent calls are rejected.
    pub fn finalize_wave(env: Env, caller: Address, wave_id: u64) -> Result<(), WaveHubError>;

    /// Fetch a Wave by its ID.
    pub fn get_wave(env: Env, wave_id: u64) -> Result<Wave, WaveHubError>;

    /// Return the total number of Waves ever created.
    pub fn wave_count(env: Env) -> u64;

    /// Return the contract owner address.
    pub fn owner(env: Env) -> Result<Address, WaveHubError>;
}
```

### Events emitted

| Topic | Data | Trigger |
|---|---|---|
| `("WaveNew", wave_id)` | `(total_pool, start_time, end_time)` | `create_wave` succeeds |
| `("WaveFin", wave_id)` | `()` | `finalize_wave` succeeds |

---

## Error reference

| Code | Variant | Trigger |
|---|---|---|
| 1 | `AlreadyInitialized` | `initialize` called more than once |
| 2 | `NotInitialized` | Any function called before `initialize` |
| 3 | `Unauthorized` | Caller is not the owner |
| 4 | `InvalidDuration` | `duration == 0` |
| 5 | `InvalidPool` | `total_pool <= 0` |
| 6 | `WaveNotFound` | No Wave exists for the given ID |
| 7 | `AlreadyFinalized` | Wave is already finalized |
| 8 | `DurationOverflow` | `start_time + duration` overflows `u64` |

---

## File structure

```
WaveHub-The-Orchestrator/
├── .github/
│   ├── workflows/
│   │   └── ci.yml                  # CI: fmt → clippy → test → WASM build
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml
│   │   └── feature_request.yml
│   └── PULL_REQUEST_TEMPLATE.md
├── contracts/
│   └── wavehub/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs              # crate root + full test suite
│           ├── contract.rs         # #[contractimpl] entry points
│           ├── types.rs            # Wave struct, DataKey enum
│           ├── errors.rs           # WaveHubError enum
│           └── storage.rs          # typed storage helpers
├── scripts/
│   ├── deploy.sh                   # build WASM + deploy to Stellar
│   └── invoke.sh                   # call contract functions via stellar-cli
├── .gitignore
├── .rustfmt.toml
├── clippy.toml
├── Cargo.toml                      # workspace root
├── Makefile
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE
├── SECURITY.md
└── README.md
```

---

## Prerequisites

| Tool | Install |
|---|---|
| Rust (stable) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| WASM target | `rustup target add wasm32-unknown-unknown` |
| Stellar CLI | `cargo install --locked stellar-cli --features opt` |

Verify your setup:

```bash
rustc --version          # rustc 1.74+
stellar --version        # stellar 21+
```

Generate and fund a Stellar identity on testnet:

```bash
stellar keys generate default --network testnet
stellar keys fund default --network testnet

# Confirm the address
stellar keys address default
```

---

## Quickstart

```bash
# 1. Clone
git clone https://github.com/your-org/WaveHub-The-Orchestrator.git
cd WaveHub-The-Orchestrator

# 2. Build
make build

# 3. Test
make test

# 4. Deploy to testnet
make deploy NETWORK=testnet
# → prints: Contract ID: C...

# 5. Initialize
export CONTRACT_ID=<contract-id-from-step-4>
./scripts/invoke.sh initialize \
  --owner $(stellar keys address default)

# 6. Create your first Wave (1 hour, 1 000 000 stroops)
./scripts/invoke.sh create_wave \
  --caller $(stellar keys address default) \
  --duration 3600 \
  --total_pool 1000000
# → returns: 1
```

---

## Building

Compile the contract to a WASM artifact:

```bash
make build
# equivalent to:
cargo build \
  --manifest-path contracts/wavehub/Cargo.toml \
  --target wasm32-unknown-unknown \
  --release
```

The optimised WASM is written to:

```
contracts/wavehub/target/wasm32-unknown-unknown/release/wavehub.wasm
```

---

## Testing

Tests use the `soroban-sdk` test utilities and run entirely off-chain — no network required.

```bash
make test
# equivalent to:
cargo test --manifest-path contracts/wavehub/Cargo.toml
```

### Test coverage

| Test | What it verifies |
|---|---|
| `test_create_wave` | Owner creates a Wave; all fields are correct |
| `test_wave_count_increments` | Counter increments correctly across multiple Waves |
| `test_non_owner_rejected` | Non-owner call returns `Unauthorized` (error code 3) |
| `test_zero_duration_rejected` | Zero duration returns `InvalidDuration` (error code 4) |
| `test_zero_pool_rejected` | Zero pool returns `InvalidPool` (error code 5) |
| `test_finalize_wave` | Owner finalizes a Wave; `finalized` becomes `true` |
| `test_double_finalize_rejected` | Second finalize returns `AlreadyFinalized` (error code 7) |
| `test_double_initialize_rejected` | Second init returns `AlreadyInitialized` (error code 1) |

### Example test

```rust
#[test]
fn test_create_wave() {
    let (_env, owner, client) = setup();
    let wave_id = client.create_wave(&owner, &3600, &1_000_000).unwrap();
    assert_eq!(wave_id, 1);

    let wave = client.get_wave(&1).unwrap();
    assert_eq!(wave.total_pool, 1_000_000);
    assert!(!wave.finalized);
    assert_eq!(wave.end_time - wave.start_time, 3600);
}

#[test]
fn test_non_owner_rejected() {
    let (env, _owner, client) = setup();
    let other = Address::generate(&env);
    let err = client.create_wave(&other, &3600, &1_000_000).unwrap_err();
    assert_eq!(err, WaveHubError::Unauthorized.into());
}
```

### Lint and format

```bash
make fmt        # auto-format with rustfmt
make lint       # clippy with -D warnings
make fmt-check  # CI-safe format check (no writes)
```

---

## Deployment

### Testnet

```bash
make deploy NETWORK=testnet
```

### Mainnet

> ⚠️ Deploying to mainnet costs real XLM. Ensure your identity is funded and the contract has been thoroughly tested.

```bash
make deploy NETWORK=mainnet
```

### Manual deploy

```bash
stellar contract deploy \
  --wasm contracts/wavehub/target/wasm32-unknown-unknown/release/wavehub.wasm \
  --network testnet \
  --source default
```

The command prints the **Contract ID** (a `C...` address). Export it for subsequent calls:

```bash
export CONTRACT_ID=CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

---

## Invoking the contract

All invocations use `scripts/invoke.sh` or `stellar contract invoke` directly.

```bash
export CONTRACT_ID=<your-contract-id>
export NETWORK=testnet          # default
export SOURCE=default           # Stellar identity to sign with
```

### initialize

Set the contract owner. Must be called once before any other function.

```bash
./scripts/invoke.sh initialize \
  --owner $(stellar keys address default)
```

### create_wave

Create a new Wave. Returns the wave ID.

```bash
./scripts/invoke.sh create_wave \
  --caller $(stellar keys address default) \
  --duration 3600 \
  --total_pool 1000000
# Output: 1
```

| Parameter | Type | Description |
|---|---|---|
| `caller` | `Address` | Must match the contract owner |
| `duration` | `u64` | Wave length in seconds (must be > 0) |
| `total_pool` | `i128` | Pool size in stroops (must be > 0) |

### get_wave

Fetch a Wave by ID.

```bash
./scripts/invoke.sh get_wave --wave_id 1
# Output:
# {
#   "total_pool": 1000000,
#   "start_time": 1746820800,
#   "end_time":   1746824400,
#   "finalized":  false
# }
```

### finalize_wave

Close a Wave. Cannot be reversed.

```bash
./scripts/invoke.sh finalize_wave \
  --caller $(stellar keys address default) \
  --wave_id 1
```

### wave_count

Return the total number of Waves created.

```bash
./scripts/invoke.sh wave_count
# Output: 1
```

### owner

Return the contract owner address.

```bash
./scripts/invoke.sh owner
# Output: G...
```

---

## Scripts

| Script | Usage | Description |
|---|---|---|
| `scripts/deploy.sh` | `./scripts/deploy.sh [network]` | Build WASM and deploy to Stellar |
| `scripts/invoke.sh` | `CONTRACT_ID=<id> ./scripts/invoke.sh <fn> [args]` | Invoke any contract function |

Environment variables:

| Variable | Default | Description |
|---|---|---|
| `NETWORK` | `testnet` | Stellar network to target |
| `SOURCE` | `default` | Stellar CLI identity to sign with |
| `CONTRACT_ID` | _(required for invoke)_ | Deployed contract address |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow, code style guide, and commit message conventions.

---

## Security

See [SECURITY.md](SECURITY.md) for the responsible disclosure policy. Do not open public issues for security vulnerabilities.

---

## License

[MIT](LICENSE) © 2026 WaveHub Contributors

---

## Future improvements

- **SAC integration** — use a Stellar Asset Contract to lock pool funds on-chain at Wave creation
- **`withdraw` function** — allow the owner to recover pool funds after finalization
- **Multi-admin / RBAC** — role-based access control for larger teams
- **Wave expiry enforcement** — reject `finalize_wave` calls before `end_time` is reached
