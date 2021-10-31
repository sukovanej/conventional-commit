.PHONY: install

build-release:
	cargo build --release

install:
	cargo install --path .
