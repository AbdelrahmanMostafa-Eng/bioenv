.PHONY: all build test lint audit clean install

all: build test lint

build:
	cargo build --release

test:
	cargo test

lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

audit:
	cargo audit

clean:
	cargo clean

install:
	cargo install --path .

run:
	cargo run --
