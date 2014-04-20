.PHONY: all build clean
all: build
build:
	mkdir -p target
	rustc src/llvm.rs --out-dir target
clean:
	rm -rf target/*