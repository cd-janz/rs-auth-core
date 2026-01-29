CARGO = cargo
BINARY_NAME = auth_core

CYAN  = \033[0;36m
RESET = \033[0m

.PHONY: all build run check clean test test-int test-e2e help

all: help

build: ## Compile the project and generate gRPC files
	@echo "$(CYAN)Compiling project...$(RESET)"
	$(CARGO) build

run: ## Run the server (REST and gRPC)
	@echo "$(CYAN)Starting servers...$(RESET)"
	$(CARGO) run

check: ## Check that the code compiles without generating binaries
	$(CARGO) check

clean: ## Clean build files and target/ directory
	$(CARGO) clean

test: ## Run all tests using nextest
	@echo "$(CYAN)🚀 Running all tests with nextest...$(RESET)"
	$(CARGO) nextest run

test-int: ## Run only INTEGRATION tests
	@echo "$(CYAN)🧪 Running integration tests...$(RESET)"
	$(CARGO) nextest run --test integration_test

test-e2e: ## Run only E2E tests
	@echo "$(CYAN)🏁 Running E2E tests...$(RESET)"
	$(CARGO) nextest run --test e2e_test

## 📋 Help
help: ## Show this help menu
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(CYAN)%-15s$(RESET) %s\n", $$1, $$2}'