GDB ?= arm-none-eabi-gdb
TARGET ?= thumbv7em-none-eabihf
ZISSOU ?= target/$(TARGET)/release/zissou


zissou $(ZISSOU): build


bloat: build
	cargo bloat $(BLOAT_ARGS) -n 50 --target $(TARGET)

build:
	rustup install stable
	rustup component add --toolchain stable llvm-tools-preview
	rustup target add --toolchain stable $(TARGET)
	cargo +stable build --release

clean:
	cargo clean

# clippy:
# 	rustup component add clippy
# 	cargo clippy

flash:  $(ZISSOU)
	$(GDB) -q -x flash.gdb $(ZISSOU)

fmt:
	rustup install nightly
	rustup component add --toolchain nightly rustfmt
	cargo +nightly fmt

openocd:
	openocd

# NB: needs separate OpenOCD running
run: build
	cargo +stable run --release

size: build
	cargo size --bin zissou --release


