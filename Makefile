.PHONY: install

build-release:
	cargo build --release

install:
	cargo install --path .

fix:
	cargo fmt

check:
	cargo check
	cargo clippy

test:
	cargo test
