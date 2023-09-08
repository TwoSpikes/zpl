.PHONY: install build

install: build
	install ./target/release/zpl /bin/
build: main.rs
	cargo build --release
