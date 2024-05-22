BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)

WASM_TARGET=wasm32-wasi
WASM_PROFILE=debug
WASM_TARGET_BUILD=$(BASE_DIR)/target/$(WASM_TARGET)/$(WASM_PROFILE)

# name of test guest
TEST_WASM=test
ADOPTOR=adaptors/wasi_snapshot_preview1.reactor.wasm

build:
	cargo build -p sdf-http-test --target $(WASM_TARGET)
	wasm-tools component new $(WASM_TARGET_BUILD)/sdf_http_test.wasm -o $(WASM_TARGET_BUILD)/$(TEST_WASM).wasm --adapt $(ADOPTOR)

.PHONY: test
test: 
	WASM_FILE=$(WASM_TARGET_BUILD)/$(TEST_WASM).wasm cargo test -p sdf-http-test -- --nocapture

