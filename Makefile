NAMESPACE := eloquentlog
PACKAGE := $(NAMESPACE)-cli

COVERAGE := 'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'

# setup
setup\:vendor: ## Install cargo vendor and run it
	@mkdir -p .cargo
	@which cargo-vendor >/dev/null 2>&1 || cargo install \
		cargo-vendor --force
	@cargo vendor > .cargo/config
.PHONY: setup\:vendor

setup\:tool: ## Install development tools
	@mkdir -p .git/hooks
.PHONY: setup\:tool

setup\:all: setup\:tool setup\:vendor ## Setup vendor and tool both
.PHONY: setup\:all

setup: setup\:all ## Sysonym of setup:all
.PHONY: setup

# verify
verify\:check: ## Check rust syntax [alias: check]
	@cargo check --all --verbose
.PHONY: veryf\:check

check: verify\:check
.PHONY: check

verify\:format: ## Check format without changes [alias: verify:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: verify\:format

verify\:fmt: verify\:format
.PHONY: verify\:fmt

format: verify\:format
.PHONY: format

fmt: verify\:format
.PHONY: fmt

verify\:lint: ## Check style using clippy [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: verify\:lint
.PHONY: lint

verify\:all: verify\:check verify\:format verify\:lint  ## Check code using all verify targets
.PHONY: verify\:all

verify: verify\:check ## Synonym of verify:check
.PHONY: verify

# test
test\:lib: ## Run tests in lib
	@cargo test --lib
.PHONY: test\:lib

test\:bin: ## Run tests for eloquentlog command binary
	@cargo test --bin $(NAMESPACE)
.PHONY: test\:bin

test\:doc: ## Run doc tests
	@cargo test --doc
.PHONY: test\:doc

test\:e2e: ## Run e2e tests under test directory
	@cargo test --test e2e
.PHONY: test\:e2e

test\:all: test\:doc ## Run tests for doc, lib and e2e
	@cargo test --lib --test e2e
.PHONY: test\:all

test: test\:all ## Synonym of test:all
.PHONY: test

# coverage
_get_covered:
	result=($(DST_DIR)/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$(DST_DIR)/index.js*"; \
	fi; \
	file=($(DST_DIR)/debug/deps/$(MODULE)-*); \
	kcov --verify --include-path=$(SRC_DIR) $(DST_DIR) $${file[0]}; \
	grep 'index.html' $(DST_DIR)/index.js* | \
		grep --only-matching --extended-regexp $(COVERAGE)

coverage\:lib: ## Get coverage of test for lib [alias: cov:lib]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/lib"; \
	cargo test --lib --no-run --target-dir=$${target_dir}; \
	make -s SRC_DIR=$${dir}/src DST_DIR=$${target_dir} \
		MARKER='index.html' MODULE=lib$(NAMESPACE) _get_covered
.PHONY: coverage\:lib

cov\:lib: coverage\:lib
.PHONY: cov\:lib

coverage\:bin: ## Get coverage of tests for elquentlog command binary [alias: cov:bin]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/bin"; \
	cargo test --bin $(NAMESPACE) --no-run --target-dir=$${target_dir}; \
	make -s SRC_DIR=$${dir}/src DST_DIR=$${target_dir} \
		MARKER='index.html' MODULE=$(NAMESPACE) _get_covered
.PHONY: coverage\:lib

cov\:bin: coverage\:bin
.PHONY: cov\:bin

# NOTE:
# e2e requires also an actual binary of cli under the
# target/debug/deps directory.
coverage\:e2e: ## Get coverage of test for e2e [alias: cov:e2e]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/e2e"; \
	export CARGO_TARGET_DIR=$${target_dir}; \
	cargo test --test e2e --no-run --target-dir=$${target_dir}; \
	make -s SRC_DIR=$${dir}/src DST_DIR=$${target_dir} \
		MARKER='index.html' MODULE=e2e _get_covered
.PHONY: coverage\:e2e

cov\:e2e: coverage\:e2e
.PHONY: cov\:e2e

coverage\:all: coverage\:lib coverage\:bin coverage\:e2e ## Get coverage from all tests [alias: cov:all]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	output_dir="$${dir}/target/coverage"; \
	kcov --merge $${output_dir} $$output_dir/lib $$output_dir/bin $$output_dir/e2e; \
	grep '\[merged\]' $$output_dir/index.js* | \
		grep --only-matching --extended-regexp $(COVERAGE)
.PHONY: coverage\:all

cov\:all: | coverage\:all
.PHONY: cov\:all

coverage: coverage\:lib ## Synonym of coverage:lib [alias: cov]
.PHONY: cov

cov: coverage
.PHONY: cov

# build
build\:debug: ## Run debug build
	cargo build
.PHONY: build\:debug

build: build\:debug ## Synonym of build:debug
.PHONY: build

build\:release: ## Build release arfitacts
	cargo build --release
.PHONY: build\:release

# utility
document: ## Generate documentation website for lib [alias: doc]
	@cargo doc --offline --document-private-items --lib --open
.PHONY: document

doc: document
.PHONY: doc

clean: ## Tidy up
	@cargo clean
.PHONY: clean

package: ## Create package
	@cargo package
.PHONY: package

install: ## Install eloquentlog command into the directory same with cargo
	@cargo install --path . --force
.PHONY: install

uninstall: ## Uninstall eloquentlog command
	@cargo uninstall $(PACKAGE)
.PHONY: uninstall

runner-%: ## Run a CI job on local (on Docker)
	@set -uo pipefail; \
	job=$(subst runner-,,$@); \
	opt=""; \
	while read line; do \
		opt+=" --env $$(echo $$line | sed -E 's/^export //')"; \
	done < .env.ci; \
	gitlab-runner exec docker \
		--executor docker \
		--cache-dir /cache \
		--docker-volumes $$(pwd)/.cache/gitlab-runner:/cache \
		--docker-volumes /var/run/docker.sock:/var/run/docker.sock \
		$${opt} $${job}
.PHONY: runner

help: ## Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[-_0-9a-z\%\:\\ ]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' ## ' | \
		sed --expression='s/\( [-_0-9a-z\%\:\\ ]*\) #/ #/' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": ## "}; \
			{printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = test\:all
default: test\:all
