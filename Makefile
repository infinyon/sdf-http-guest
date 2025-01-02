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


EXPECTED_GET=hello-world

# this test is a simple curl test that checks if the server is working
.PHONY: test-get
test-get:
	$(eval RESPONSE := $(shell curl -s localhost:3000/hello/world))
	@echo "response from server: $(RESPONSE)"
	@if [ "$(RESPONSE)" = "$(EXPECTED_GET)" ]; then \
        echo "test worked"; \
    else \
        echo "test failed"; \
        exit 1; \
    fi

EXPECTED_POST={code:0,message:u123}

.PHONY: test-post
test-post:
	$(eval RESPONSE := $(shell curl -s -X POST -d '{"name":"u123"}' -H "Authorization: Bearer 123" -H "Content-Type: application/json" localhost:3000/create))
	@echo "response from server: $(RESPONSE)"
	@if [ "$(RESPONSE)" = "$(EXPECTED_POST)" ]; then \
        echo "test worked"; \
    else \
        echo "test failed"; \
        exit 1; \
    fi



FLUVIO_OUTPUT=/tmp/fluvio_output.txt
GET_INPUT_TOPIC=input-get
GET_OUT_TOPIC=output-get

# test dataflow, this assume fluvio is running
.PHONY: test-df-get
test-df-get:
	$(eval CURRENT_OFFSET := $(shell fluvio partition list --output json | jq '.[] | select(.name == "$(GET_OUT_TOPIC)-0").status.leader.leo')) 
	$(eval NEXT_OFFSET := $(shell echo $$(($(CURRENT_OFFSET) + 1))))
	@echo "current offset: $(CURRENT_OFFSET), next: $(NEXT_OFFSET)"
	$(eval RANDOM_NUMBER := $(shell echo $$RANDOM | awk '{print int($$1)}'))
	@echo "Random number: $(RANDOM_NUMBER)"
	$(eval EXPECTED_GET_DF := $(shell echo "hello-${RANDOM_NUMBER}"))
	@echo $(RANDOM_NUMBER) | fluvio produce $(GET_INPUT_TOPIC)
	@echo "expected: $(EXPECTED_GET_DF)"
	@echo "waiting for data to be processed"
	@sleep 0.5


# validation must be run after test-dataflow
validate-df-get: test-df-get
	@fluvio consume $(GET_OUT_TOPIC) --start $(CURRENT_OFFSET) --end $(NEXT_OFFSET) -d 2>/dev/null
	$(eval OUTPUT := $(shell fluvio consume $(GET_OUT_TOPIC) --start $(CURRENT_OFFSET) --end $(NEXT_OFFSET) -d 2>/dev/null ))
	@echo "output: $(OUTPUT), expected: $(EXPECTED_GET_DF),"
	@if [ "$(OUTPUT)" = "$(EXPECTED_GET_DF)" ]; then \
		echo "test worked"; \
	else \
		echo "test failed"; \
		exit 1; \
	fi



POST_INPUT_TOPIC=input-post
POST_OUT_TOPIC=output-post

# test dataflow, this assume fluvio is running
.PHONY: test-df-post
test-df-post:
	$(eval CURRENT_OFFSET := $(shell fluvio partition list --output json | jq '.[] | select(.name == "$(POST_OUT_TOPIC)-0").status.leader.leo')) 
	$(eval NEXT_OFFSET := $(shell echo $$(($(CURRENT_OFFSET) + 1))))
	@echo "current offset: $(CURRENT_OFFSET), next: $(NEXT_OFFSET)"
	$(eval RANDOM_NUMBER := $(shell echo $$RANDOM | awk '{print int($$1)}'))
	@echo "Random number: $(RANDOM_NUMBER)"
	$(eval EXPECTED_POST_DF := $(shell echo "{\"code\":0,\"message\":\"${RANDOM_NUMBER}\"}"))
	@echo $(RANDOM_NUMBER) | fluvio produce $(POST_INPUT_TOPIC)
	@echo "expected: $(EXPECTED_POST_DF)"
	@echo "waiting for data to be processed"
	@sleep 0.5


# validation must be run after test-dataflow
validate-df-post: test-df-post
	@fluvio consume $(POST_OUT_TOPIC) --start $(CURRENT_OFFSET) --end $(NEXT_OFFSET) -d 2>/dev/null
	$(eval OUTPUT := $(shell fluvio consume $(POST_OUT_TOPIC) --start $(CURRENT_OFFSET) --end $(NEXT_OFFSET) -d 2>/dev/null ))
	@echo "output: $(OUTPUT), expected: $(EXPECTED_POST_DF),"
	@if [ "$(OUTPUT)" = "$(EXPECTED_POST_DF)" ]; then \
		echo "test worked"; \
	else \
		echo "test failed"; \
		exit 1; \
	fi