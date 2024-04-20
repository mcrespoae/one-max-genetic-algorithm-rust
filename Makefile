# Makefile
.PHONY: run build run-preview build-preview test clean
run: build
	cargo run --release

build:
	cargo build --release

run-preview: build-preview
	cargo run

build-preview:
	cargo build

test:
	cargo test
	cargo test -- --ignored

clean:
	cargo clean