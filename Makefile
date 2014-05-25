RUSTC ?= rustc
RUSTDOC ?= rustdoc

.PHONY: all build doc interactive tests clean
all: build interactive tests doc
build:
	mkdir -p target
	cd target && $(RUSTC) ../src/script.rs -g -L .
tests:
	$(RUSTC) src/bin/tests.rs -g -L target -o target/tests
interactive:
	$(RUSTC) src/bin/interactive.rs -g -L target -o target/interactive
doc:
	$(RUSTDOC) src/script.rs -o doc
clean:
	rm -rf target/*
