.PHONY: all build test run fmt ci
all: build

build:
	cargo build --workspace

test:
	cargo test --workspace

run:
	cargo run -p owg-server

fmt:
	rustfmt --edition 2021 --emit files $(shell fd -e rs)

ci:
	cargo test --workspace --all-features
