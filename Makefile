# Makefile

.DEFAULT_GOAL := all

BIN_NAME := lockjaw
CONFIG_SCRIPT := configure

all: $(CONFIG_SCRIPT) build strip

$(CONFIG_SCRIPT):
	chmod +x $(CONFIG_SCRIPT)
	./$(CONFIG_SCRIPT)

build: $(CONFIG_SCRIPT)
	RUSTFLAGS="-C opt-level=3 -C panic=abort -C target-cpu=native" cargo build --release

strip:
	strip target/release/$(BIN_NAME)

install: build strip
	sudo cp ./target/release/$(BIN_NAME) /usr/bin/$(BIN_NAME)
	cargo clean

uninstall:
	sudo rm /usr/bin/$(BIN_NAME)

clean:
	cargo clean

run:
	cargo run --release

.PHONY: all $(CONFIG_SCRIPT) build strip install uninstall clean run
