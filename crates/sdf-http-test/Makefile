BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)

WASM_TARGET ?= wasm32-wasi
WASM_PROFILE=debug
WASM_TARGET_BUILD=$(BASE_DIR)/target/$(WASM_TARGET)/$(WASM_PROFILE)

VALUE_GUEST_PRJ=value-simple-guest

build:
	cargo build --target $(WASM_TARGET)

.PHONY: test
test: 
	WASM_FILE=$(WASM_TARGET_BUILD) cargo test -p sdf-http-test -- --nocapture

