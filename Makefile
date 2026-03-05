# Makefile for Nvisy SDK for Rust

# URLs
SERVER_URL        ?= https://api.nvisy.com
SERVER_LOCAL_URL  ?= http://127.0.0.1:8080
RUNTIME_URL       ?= http://localhost:8080
RUNTIME_LOCAL_URL ?= http://localhost:8080

SERVER_OPENAPI    = $(SERVER_URL)/openapi.json
RUNTIME_OPENAPI   = $(RUNTIME_URL)/api/v1/openapi.json

GENERATED_DIR     = generated

# Logging
define log
	@echo "[$(shell date '+%H:%M:%S')] $(1)"
endef

# Default target
.PHONY: help
help:
	@echo "  make check                  : Run all checks (fmt, clippy, test, doc)"
	@echo "  make clean                  : Remove build artifacts"
	@echo "  make generate-server        : Generate types from server OpenAPI"
	@echo "  make generate-server-local  : Generate types from local server"
	@echo "  make generate-runtime       : Generate types from runtime OpenAPI"
	@echo "  make generate-runtime-local : Generate types from local runtime"

# Run all checks: format, lint, test, docs
.PHONY: check
check:
	$(call log,Checking formatting...)
	@cargo fmt --check
	$(call log,Running clippy...)
	@cargo clippy -- -D warnings
	$(call log,Running tests...)
	@cargo test
	$(call log,Building docs...)
	@cargo doc --no-deps
	$(call log,All checks passed)

# Remove build artifacts and generated files
.PHONY: clean
clean:
	$(call log,Cleaning...)
	@cargo clean
	@rm -rf $(GENERATED_DIR)/
	$(call log,Clean complete)

# Generate reference types from server OpenAPI spec
.PHONY: generate-server
generate-server:
	$(call log,Fetching server OpenAPI schema from $(SERVER_OPENAPI)...)
	@mkdir -p $(GENERATED_DIR)
	@curl -sf $(SERVER_OPENAPI) -o $(GENERATED_DIR)/openapi-server.json
	$(call log,Generating types...)
	@openapi-generator generate -i $(GENERATED_DIR)/openapi-server.json -g rust -o $(GENERATED_DIR)/nvisy-api --package-name nvisy-api
	@cargo fmt
	$(call log,Server types generated)

# Generate reference types from local server OpenAPI spec
.PHONY: generate-server-local
generate-server-local:
	$(call log,Fetching server OpenAPI schema from $(SERVER_LOCAL_URL)...)
	@mkdir -p $(GENERATED_DIR)
	@curl -sf $(SERVER_LOCAL_URL)/openapi.json -o $(GENERATED_DIR)/openapi-server.json
	$(call log,Generating types...)
	@openapi-generator generate -i $(GENERATED_DIR)/openapi-server.json -g rust -o $(GENERATED_DIR)/nvisy-api --package-name nvisy-api
	@cargo fmt
	$(call log,Server types generated)

# Generate reference types from runtime OpenAPI spec
.PHONY: generate-runtime
generate-runtime:
	$(call log,Fetching runtime OpenAPI schema from $(RUNTIME_OPENAPI)...)
	@mkdir -p $(GENERATED_DIR)
	@curl -sf $(RUNTIME_OPENAPI) -o $(GENERATED_DIR)/openapi-runtime.json
	$(call log,Generating types...)
	@openapi-generator generate -i $(GENERATED_DIR)/openapi-runtime.json -g rust -o $(GENERATED_DIR)/nvisy-rt-api --package-name nvisy-rt-api
	@cargo fmt
	$(call log,Runtime types generated)

# Generate reference types from local runtime OpenAPI spec
.PHONY: generate-runtime-local
generate-runtime-local:
	$(call log,Fetching runtime OpenAPI schema from $(RUNTIME_LOCAL_URL)...)
	@mkdir -p $(GENERATED_DIR)
	@curl -sf $(RUNTIME_LOCAL_URL)/api/v1/openapi.json -o $(GENERATED_DIR)/openapi-runtime.json
	$(call log,Generating types...)
	@openapi-generator generate -i $(GENERATED_DIR)/openapi-runtime.json -g rust -o $(GENERATED_DIR)/nvisy-rt-api --package-name nvisy-rt-api
	@cargo fmt
	$(call log,Runtime types generated)
