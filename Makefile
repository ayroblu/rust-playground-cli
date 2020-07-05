# Example from
# https://tech.davis-hansson.com/p/make/
# Also consider reference at: http://www.gnu.org/software/make/manual/

## Initial setup
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

## ------------------------- Main part of the build file
# Default - top level rule is what gets ran when you run just `make`
build:
> cargo build --release
.PHONY: build

run:
> cargo run -q
.PHONY: run

doc:
> cargo doc --open
.PHONY: doc
