.PHONY: install build

install: build
	install ./target/release/zpl ${PREFIX}/../usr/bin/
build: main.rs
	cargo build --release
