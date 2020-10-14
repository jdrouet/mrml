install-clippy:
	rustup component add clippy

install-rustfmt:
	rustup component add rustfmt

install-tarpaulin:
	cargo install cargo-tarpaulin

ci-install-deps: install-clippy install-rustfmt install-tarpaulin
	make -C packages/wasm ci-install-deps

format:
	make -C packages/cli format
	make -C packages/library format
	make -C packages/wasm format

quality:
	cargo clippy -- -D warnings
