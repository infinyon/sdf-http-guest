update-deps:
	wit-deps update


VALUE_GUEST_PRJ=value-simple-guest

build-value-guest:
	make -C $(TEST_GUEST)/$(VALUE_GUEST_PRJ) build

.PHONY: test
test: 
	WASM_DIR=$(WASM_TARGET_BUILD) cargo test -p sdf-http-test -- --nocapture

