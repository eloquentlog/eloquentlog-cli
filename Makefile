# setup -- {{{
setup\:vendor:  ## Install cargo vendor and run it
	@mkdir -p .cargo
	@which cargo-vendor >/dev/null 2>&1 || cargo install \
		cargo-vendor --force
	@cargo vendor > .cargo/config
.PHONY: setup\:vendor

setup\:tool:  ## Install development tools
# for cargo-husky
	@mkdir -p .git/hooks
.PHONY: setup\:tool

setup\:all: | setup\:tool setup\:vendor  ## Setup vendor and tool both [alias: setup]
.PHONY: setup\:all

setup: | setup\:all
.PHONY: setup
# }}}

# verify -- {{{
verify\:check:  ## Check rust syntax [alias: check]
	@cargo check --all --verbose
.PHONY: veryf\:check

check: | verify\:check
.PHONY: check

verify\:format:  ## Check format without changes [alias: verify:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: verify\:format

verify\:fmt: | verify\:format
.PHONY: verify\:fmt

format: | verify\:format
.PHONY: format

fmt: | verify\:format
.PHONY: fmt

verify\:lint:  ## Check style using clippy [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: | verify\:lint
.PHONY: lint

verify\:all: | verify\:check verify\:format verify\:lint  ## Check code using all verify:xxx targets [alias: verify]
.PHONY: verify\:all

verify: | verify\:all
.PHONY: verify
# }}}

# test -- {{{
test\:lib:  ## Run tests in lib [alias: test:unit]
	@cargo test --lib
.PHONY: test\:lib

test\:unit: | test\:lib
.PHONY: test\:unit

test\:bin:  ## Run tests for eloquentlog command
	@cargo test --bin eloquentlog
.PHONY: test\:bin

test\:doc:  ## Run tests in doc
	@cargo test --doc
.PHONY: test\:doc

test\:integration:  ## Run integration tests
	@cargo test --test integration
.PHONY: test\:integration

test\:all:  ## Run unit tests and integration tests [alias: test]
	@cargo test --tests
.PHONY: test\:all

test: | test\:all
.PHONY: test
# }}}

# coverage -- {{{
coverage:  ## Generate coverage report from tests for lib using kcov [alias: cov]
	@cargo test --lib --no-run
	@./.tool/setup-kcov
	./.tool/get-covered libeloquentlog
.PHONY: coverage

cov: | coverage
.PHONY: cov
# }}}

# build -- {{{
build\:debug:  ## Run debug build [alias: build]
	cargo build
.PHONY: build\:debug

build: | build\:debug
.PHONY: build

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release
# }}}

# other utilities -- {{{
doc:
	@cargo doc --offline --document-private-items --lib --open
.PHONY: doc

clean:  ## Tidy up
	@cargo clean
.PHONY: clean

package:  ## Create package
	@cargo package
.PHONY: package

install:  ## Install eloquentlog command into the directory same with cargo
	@cargo install --path . --force
.PHONY: install

uninstall:  ## Uninstall eloquentlog command
	@cargo uninstall eloquentlog-cli
.PHONY: uninstall

help:  ## Display this message
	@grep --extended-regexp '^[0-9a-z\:\\]+: ' $(MAKEFILE_LIST) | \
		grep --extended-regexp '  ## ' | \
		sed --expression='s/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ":  ## "}; \
				{printf "\033[38;05;222m%-17s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help
# }}}

.DEFAULT_GOAL = test\:all
default: test\:all
