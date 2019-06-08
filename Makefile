GDB ?= arm-none-eabi-gdb
TARGET ?= thumbv7em-none-eabihf
TOOLCHAIN ?= nightly  # ufmt's `uwrite!` needs nightly :/
# TOOLCHAIN ?= stable

ZISSOU ?= target/$(TARGET)/release/zissou
zissou $(ZISSOU): build


bloat: build
	cargo +nightly bloat $(BLOAT_ARGS) -n 50 --release

build:
	# rustup install stable
	# rustup component add --toolchain stable llvm-tools-preview
	# rustup target add --toolchain stable $(TARGET)
	# touch build.rs  # force reloading git-describe
	cargo +$(TOOLCHAIN) build --release
	# cargo +$(TOOLCHAIN) size --bin zissou --release

clean:
	cargo clean

docs:
	cargo doc # --open
	echo "Visit target/thumbv7em-none-eabihf/doc/settings.html"

# clippy:
# 	rustup component add clippy
# 	cargo clippy

# slightly faster than `flash-gdb`
flash flash-openocd: $(ZISSOU)
	openocd -f openocd.cfg -c "init; targets; reset halt; program $(ZISSOU) verify reset exit"

flash-gdb: $(ZISSOU)
	$(GDB) -q -x flash.gdb $(ZISSOU)

fmt:
	rustup install nightly
	rustup component add --toolchain nightly rustfmt
	cargo +nightly fmt

openocd:
	openocd

hil:
	pytest --quiet

pcscd:
	sudo LIBCCID_ifdLogLevel=0x000F pcscd --foreground --debug --apdu --color

# NB: needs separate OpenOCD running
run:
	cargo +$(TOOLCHAIN) run --release

run-debug:
	cargo +$(TOOLCHAIN) run

rustup:
	rustup install $(TOOLCHAIN)
	rustup component add --toolchain $(TOOLCHAIN) llvm-tools-preview
	rustup target add --toolchain $(TOOLCHAIN) $(TARGET)

size: build
	cargo +$(TOOLCHAIN) size --bin zissou --release

tail-zissouserial:
	scripts/tail-zissouserial

udev:
	sudo cp 70-zissou.rules /etc/udev/rules.d
	sudo udevadm control --reload-rules
	sudo udevadm trigger

_create-venv:
	python3 -m venv venv

venv: _create-venv update-venv
	# python3 -m venv venv
	# venv/bin/pip install -U pip
	# venv/bin/pip install -U -r requirements.txt

# re-run if dev or runtime dependencies change,
# or when adding new scripts
update-venv: venv
	venv/bin/pip install -U pip
	venv/bin/pip install -U -r requirements.txt
	venv/bin/pip install -U -r dev-requirements.txt
