.DEFAULT_GOAL := build

PYTHON ?= python3
VENV ?= target/python-venv
DIST_DIR ?= target/python-dist
MATURIN_VERSION ?= 1.16.0
RELEASE_REPO ?= sanjay7178/text-processing-rs
RELEASE_TAG ?=
BUMP ?= auto

ifeq ($(OS),Windows_NT)
VENV_PYTHON := $(VENV)/Scripts/python.exe
else
VENV_PYTHON := $(VENV)/bin/python
endif

PYTHON_WHEEL_FEATURES := python,pyo3/abi3-py39

export RELEASE_REPO RELEASE_TAG BUMP

.PHONY: setup build check install run test next-version release release-dry-run

setup: $(VENV)/.build-tools

$(VENV)/.build-tools: Makefile pyproject.toml
	"$(PYTHON)" -m venv "$(VENV)"
	"$(VENV_PYTHON)" -m pip install "maturin==$(MATURIN_VERSION)" twine
	touch "$@"

build: setup
	"$(VENV_PYTHON)" -m maturin build --locked --release --sdist --compatibility pypi \
		--features "$(PYTHON_WHEEL_FEATURES)" --interpreter "$(VENV_PYTHON)" --out "$(DIST_DIR)"

check: build
	"$(VENV_PYTHON)" -m twine check --strict "$(DIST_DIR)"/*.whl "$(DIST_DIR)"/*.tar.gz

install: setup
	VIRTUAL_ENV="$(abspath $(VENV))" "$(VENV_PYTHON)" -m maturin develop --locked --release \
		--features "$(PYTHON_WHEEL_FEATURES)"

run: install
	"$(VENV_PYTHON)" -c 'from text_processing_rs import itn_normalize_sentence_lang; print(itn_normalize_sentence_lang("I have twenty one apples"))'
	"$(VENV_PYTHON)" -c 'from text_processing_rs import tn_normalize_sentence_lang; print(tn_normalize_sentence_lang("I have 21 apples"))'

test: install
	cargo test --locked
	"$(VENV_PYTHON)" scripts/check-python-wheel.py

next-version:
	@"$(PYTHON)" scripts/predict-python-version.py --repo "$$RELEASE_REPO" --bump "$$BUMP"

release release-dry-run:
	@set -eu; \
	version=$$(cargo metadata --locked --no-deps --format-version 1 | \
		"$(PYTHON)" -c 'import json, sys; print(json.load(sys.stdin)["packages"][0]["version"])'); \
	tag="$${RELEASE_TAG:-v$$version}"; \
	case "$$tag" in \
		v"$$version"|v"$$version"-*) ;; \
		*) printf 'RELEASE_TAG must start with v%s and match the package version.\n' "$$version" >&2; exit 1 ;; \
	esac; \
	git check-ref-format "refs/tags/$$tag"; \
	commit=$$(git rev-parse HEAD); \
	printf 'Repository  %s\nVersion     %s\nTag         %s\nCommit      %s\n' "$$RELEASE_REPO" "$$version" "$$tag" "$$commit"; \
	refs=$$(git ls-remote --tags "https://github.com/$$RELEASE_REPO.git" "refs/tags/$$tag" "refs/tags/$$tag^{}"); \
	tag_commit=$$(printf '%s\n' "$$refs" | awk 'END { print $$1 }'); \
	if [ -n "$$tag_commit" ] && [ "$$tag_commit" != "$$commit" ]; then \
		printf 'Tag %s points to a different commit. Choose a fresh RELEASE_TAG, such as v%s-1.\n' "$$tag" "$$version" >&2; \
		exit 1; \
	fi; \
	if ! gh api "repos/$$RELEASE_REPO/contents/.github/workflows/python-release.yml?ref=$$commit" --silent; then \
		printf 'The release commit must contain the Python workflow on %s.\n' "$$RELEASE_REPO" >&2; \
		exit 1; \
	fi; \
	if [ "$@" = release ]; then \
		gh release create "$$tag" --repo "$$RELEASE_REPO" --target "$$commit" \
			--title "Python package $$version" --notes "The Python release workflow builds and attaches the distribution files."; \
	else \
		printf 'Dry run passed for %s.\n' "$$tag"; \
	fi
