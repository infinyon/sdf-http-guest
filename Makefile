GUEST_LIB=sdf-http

WASM_TARGET=wasm32-wasi

build:
	cargo build -p $(GUEST_LIB) --target $(WASM_TARGET)
