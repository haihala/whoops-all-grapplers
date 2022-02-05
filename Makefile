release ?=
RUST_BACKTRACE := 1

run:
	cargo run

build:
	cargo build $(release)

test:
	cargo test $(release)

clippy:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt --all -- --check

udeps:
	cargo udeps --all-targets

check: test clippy fmt udeps

install:
	cargo install cargo-udeps
