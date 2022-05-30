release ?=
RUST_BACKTRACE := 1

run:
	@echo "cargo run --features bevy/dynamic	# Speedup on non-windows"
	cargo run

build:
	cargo build $(release)

test:
	cargo nextest run

cargo-test:
	cargo test $(release)

integration:
	cargo test $(release) -- --ignored

clippy:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt --all -- --check

udeps:
	cargo udeps --all-targets

check: test clippy fmt udeps

install:
	cargo install cargo-udeps cargo-nextest
	rustup component add clippy
