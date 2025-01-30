ARCH := $(shell uname -m | tr '[:upper:]' '[:lower:]' | sed 's/arm64/aarch64/')
OS := $(shell uname -s | tr '[:upper:]' '[:lower:]')
OS_TYPE ?= debian
WORK_DIR := $(shell pwd)
DIST_DIR := $(WORK_DIR)/dist
TARGET ?= debug
CARGO_FLAGS := $(if $(filter $(TARGET),release),--release,)

clean-cargo:
	cargo clean

clean-dist:
	rm -rfv $(DIST_DIR)

clean: clean-cargo clean-dist

check:
	cargo check $(CARGO_FLAGS)

format:
	cargo fmt --check

lint:
	cargo clippy -- -D warnings

build:
	cargo build $(CARGO_FLAGS)

dist: build
	mkdir -pv $(DIST_DIR)
	tar -czvf $(DIST_DIR)/harm-$(ARCH)-$(OS).tar.gz \
		-C $(WORK_DIR)/target/$(TARGET) harm
