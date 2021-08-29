release ?=

run:
	cargo run

build:
	cargo build $(release)

test:
	cargo test $(release)

clippy:
	cargo +nightly clippy --all

fmt:
	cargo +nightly fmt --all -- --check

check: build test clippy fmt
