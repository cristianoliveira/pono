.PHONY: help
help: ## Lists the available commands. Add a comment with '##' to describe a command.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST)\
		| sort\
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Run a release build.
	@cargo build --release

.PHONY: test
test: fmt ## Run all the tests.
	@cargo test --features "test-all"

.PHONY: install
install: test ## Install locally after running tests.
	@cargo install --path .

.PHONY: fmt
fmt: ## Formats the code.
	@cargo fmt -v
	
.PHONY: nix-checks
nix-checks: ## Run a nit check on the nix files.
	# used in .github/workflows/on-push-nixbuild.yml
	@nix flake check

.PHONY: nix-build-local
nix-build-local: ## Run a local nix build. Used for development and CI.
	# used in .github/workflows/on-push-nixbuild.yml
	@nix build .#pono-local --verbose

.PHONY: nix-build
nix-build: ## Run a nix build. Used for development and CI.
	# used in .github/workflows/on-push-nixbuild.yml
	@nix build .#pono --verbose

.PHONY: nix-release
nix-release: ## Generate new hash for nix package based on the latest version.
	scripts/bump-nix
