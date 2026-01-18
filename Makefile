# Makefile for Nvisy.com SDK for Rust

# Make-level logger (evaluated by make; does not invoke the shell)
define make-log
$(info [$(shell date '+%Y-%m-%d %H:%M:%S')] [MAKE] [$(MAKECMDGOALS)] $(1))
endef

# Default target
.PHONY: help
help:
	$(call make-log,Available targets:)
	@echo "  clean          - Remove build artifacts and temporary files"
	@echo "  check-remote   - Check connection to production API"
	@echo "  check-local    - Check connection to local API server"
	@echo "  generate       - Generate Rust types from OpenAPI specification"
	@echo "  generate-local - Generate Rust types from local API server"

# Clean build artifacts and temporary files
.PHONY: clean
clean:
	$(call make-log,Cleaning build artifacts...)
	@rm -rf target/
	@rm -f openapi.json
	@rm -rf nvisy-api/
	$(call make-log,Clean complete)

# Check connection to production API
.PHONY: check-remote
check-remote:
	$(call make-log,Checking connection to production API...)
	@curl -sf https://api.nvisy.com/openapi.json > /dev/null || (echo "Error: Cannot connect to https://api.nvisy.com/openapi.json" && exit 1)
	$(call make-log,Connection successful)

# Check connection to local API server
.PHONY: check-local
check-local:
	$(call make-log,Checking connection to local API server...)
	@curl -sf http://127.0.0.1:8080/api/openapi.json > /dev/null || (echo "Error: Cannot connect to http://127.0.0.1:8080/api/openapi.json" && exit 1)
	$(call make-log,Connection successful)

# Generate Rust types from OpenAPI specification
.PHONY: generate
generate: check-remote
	$(call make-log,Fetching OpenAPI schema...)
	@curl -s https://api.nvisy.com/openapi.json -o openapi.json
	$(call make-log,Generating Rust types from OpenAPI specification...)
	@openapi-generator generate -i openapi.json -g rust -o nvisy-api --package-name nvisy-api
	$(call make-log,Formatting generated files...)
	@cargo fmt
	$(call make-log,Building project...)
	@cargo build
	$(call make-log,Type generation complete)

# Generate Rust types from local API server
.PHONY: generate-local
generate-local: check-local
	$(call make-log,Fetching OpenAPI schema...)
	@curl -s http://127.0.0.1:8080/api/openapi.json -o openapi.json
	$(call make-log,Generating Rust types from local API server...)
	@openapi-generator generate -i openapi.json -g rust -o nvisy-api --package-name nvisy-api
	$(call make-log,Formatting generated files...)
	@cargo fmt
	$(call make-log,Building project...)
	@cargo build
	$(call make-log,Type generation complete)
