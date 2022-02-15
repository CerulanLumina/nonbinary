TARGET_RUST := libnonbinary.rlib
BUILD_DIR := ./target/release
.DEFAULT_GOAL := all

$(BUILD_DIR)/$(TARGET_RUST):
	@echo Building Rust targets libnonbinary.rlib, libnonbinary.so, libnonbinary.a
	@cargo build --release


headers/nonbinary.h:
	@echo Generating headers...
	@cbindgen --config cbindgen.toml --output headers/nonbinary.h

.PHONY: clean all
all: $(BUILD_DIR)/$(TARGET_RUST) headers/nonbinary.h
clean:
	cargo clean
	rm -f headers/nonbinary.h
