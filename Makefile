.DEFAULT_GOAL := help

.PHONY: build
build: ## build everything
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: test
test: build ## test everything
	set -ev; \
	cargo test

.PHONY: clean
clean: ## remove build artifacts
	rm -rf target

help:
	@awk -F":.*## " '$$2&&$$1~/^[a-zA-Z_%-]+/{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
