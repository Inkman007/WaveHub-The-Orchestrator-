# Changelog

All notable changes to WaveHub will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.1.0] — 2026-05-09

### Added
- Initial Soroban smart contract implementation
- `initialize`, `create_wave`, `finalize_wave`, `get_wave`, `wave_count`, `owner` functions
- Typed error enum (`WaveHubError`) with `contracterror` macro
- Modular source layout: `contract.rs`, `types.rs`, `errors.rs`, `storage.rs`
- Full unit test suite (7 tests)
- CI workflow via GitHub Actions
- Deploy and invoke shell scripts
- `Makefile` with `build`, `test`, `lint`, `fmt`, `coverage`, `deploy`, `clean` targets
