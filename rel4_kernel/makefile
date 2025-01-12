ARCH := riscv64

ifeq ($(ARCH), riscv64)
TARGET := riscv64imac-unknown-none-elf
else ifeq ($(ARCH), aarch64)
TARGET := aarch64-unknown-none-softfloat
endif

all: build 
	@echo $(ARCH)
env:
	rustup target add riscv64imac-unknown-none-elf
	rustup component add rust-src
build:
	cargo build --release --target $(TARGET)
run:
	cargo build --release --target $(TARGET)
clean:
	cargo clean
	rm -rf build
.PHONY: all build env run clean
