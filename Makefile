NETWORK    ?= testnet
WASM       := contracts/wavehub/target/wasm32-unknown-unknown/release/wavehub.wasm
CARGO_MANIFEST := --manifest-path contracts/wavehub/Cargo.toml

.PHONY: build test lint fmt coverage deploy clean

## Build the WASM artifact
build:
	cargo build $(CARGO_MANIFEST) --target wasm32-unknown-unknown --release

## Run unit tests
test:
	cargo test $(CARGO_MANIFEST)

## Run clippy linter
lint:
	cargo clippy --all-targets --all-features -- -D warnings

## Auto-format all source files
fmt:
	cargo fmt --all

## Check formatting without modifying files
fmt-check:
	cargo fmt --all -- --check

## Generate test coverage report (requires cargo-llvm-cov)
coverage:
	cargo llvm-cov $(CARGO_MANIFEST) --lcov --output-path lcov.info
	@echo "Coverage report written to lcov.info"

## Deploy to Stellar network (default: testnet)
deploy: build
	./scripts/deploy.sh $(NETWORK)

## Remove build artifacts
clean:
	cargo clean $(CARGO_MANIFEST)
	rm -f lcov.info
