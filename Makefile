PROJECT_NAME := $(shell basename "$(PWD)")
ARGS := -d $(PWD)/dest
FILEPATH := rbkkl/donum/README.md

.PHONY: run
run:
	cargo build
	./target/debug/$(PROJECT_NAME) $(ARGS) $(FILEPATH)
