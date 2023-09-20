release ?=
RUST_BACKTRACE := 1

dev:
	cargo run -- --dev dummy dummy

prod:
	cargo run -- dummy dummy

build:
	cargo build $(release)

test:
	cargo test $(release) --workspace --exclude integration_tests

integration:
	cargo test $(release) -p integration_tests

clippy:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt --all -- --check

udeps:
	cargo udeps --all-targets

check: test clippy fmt udeps

install:
	rustup update
	cargo install cargo-udeps
	rustup component add clippy
	rustup component add rustfmt
