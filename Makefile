release ?=

run:
	cargo run

build:
	cargo build $(release)

test:
	cargo test $(release)

clippy:
	cargo clippy --all

fmt:
	cargo fmt --all -- --check

check: build test clippy fmt
