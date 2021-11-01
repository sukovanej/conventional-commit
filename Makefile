.PHONY: install

build-release:
	cargo build --release

current_git_command = $(git config --global alias.cc)

install:
	@ echo "Building..."
	@ cargo install --path . 1> /dev/null && \
		echo "Succesfully built!" && sh setup-git-alias.sh || (echo "Build failed" && exit 1)

fix:
	cargo fmt

check:
	cargo check
	cargo clippy

test:
	cargo test
