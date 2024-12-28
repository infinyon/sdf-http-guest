GUEST_LIB=sdf-http

WASM_TARGET=wasm32-wasip2

build-guest:
	cargo build -p $(GUEST_LIB) --target $(WASM_TARGET)



HTTP_SERVER = http-test-server

debug-http:
	cargo run

build-http:
	cargo build --release -p $(HTTP_SERVER)

run-http:	build-http shutdown
	nohup target/release/$(HTTP_SERVER) 2>&1 > /tmp/http-test-server.log &

shutdown:
	killall http-test-server || true


EXPECTED="hello-world"

# this test is a simple curl test that checks if the server is working
.PHONY: test-http
test-http:	
	$(eval RESPONSE := $(shell curl -s localhost:3000/hello/world))
	@echo "response from server: $(RESPONSE)"
	@if [ "$(RESPONSE)" = "$(EXPECTED)" ]; then \
        echo "test worked"; \
    else \
        echo "test failed"; \
        exit 1; \
    fi


FLUVIO_OUTPUT=/tmp/fluvio_output.txt

# test dataflow, this assume fluvio is running
.PHONY: test-dataflow
test-dataflow:
	$(eval CURRENT_OFFSET := $(shell fluvio partition list --output json | jq '.[] | select(.name == "output-0").status.leader.leo')) 
	$(eval NEXT_OFFSET := $(shell echo $$(($(CURRENT_OFFSET) + 1))))
	@echo "current offset: $(CURRENT_OFFSET), next: $(NEXT_OFFSET)"
	$(eval RANDOM_NUMBER := $(shell echo $$RANDOM | awk '{print int($$1)}'))
	@echo "Random number: $(RANDOM_NUMBER)"
	@echo $(RANDOM_NUMBER) | fluvio produce input
	$(eval EXPECTED := $(shell echo "hello-${RANDOM_NUMBER}"))
	@echo "expected: $(EXPECTED)"
	@echo "waiting for data to be processed"
	@sleep 0.5


# validation must be run after test-dataflow
validate-dataflow: test-dataflow
	@fluvio consume output --start $(CURRENT_OFFSET) --end $(NEXT_OFFSET) -d 2>/dev/null
	$(eval OUTPUT := $(shell fluvio consume output --start $(CURRENT_OFFSET) --end $(NEXT_OFFSET) -d 2>/dev/null ))
	@echo "output: $(OUTPUT), expected: $(EXPECTED),"
	@if [ "$(OUTPUT)" = "$(EXPECTED)" ]; then \
		echo "test worked"; \
	else \
		echo "test failed"; \
		exit 1; \
	fi
