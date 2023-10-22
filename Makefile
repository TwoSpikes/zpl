.PHONY: install build

install: build
	install ./target/release/zpl ${PREFIX}/../usr/bin/
build:
	cargo build --release
