build:
	rustup component add llvm-tools-preview
	rustup target add thumbv7em-none-eabihf
	cargo build --release

fmt:
	rustup component add rustfmt
	cargo fmt

clippy:
	rustup component add clippy
	cargo clippy

clean:
	cargo clean
