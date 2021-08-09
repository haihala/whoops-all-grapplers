release ?=

build:
	cargo build $(release)

test:
	cargo test $(release)

clippy:
	cargo +nightly clippy --all

fmt:
	cargo +nightly fmt --all -- --check

all: build test clippy fmt
