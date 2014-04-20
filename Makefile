.PHONY: all build lib llvm doc tests interactive clean
all: lib build tests interactive doc
lib:
	cd lib/llvm && make build
	mv lib/llvm/target/libllvm* target
build:
	rustc src/script.rs -L target --out-dir target
tests:
	rustc src/bin/tests.rs -L target --out-dir target
interactive:
	rustc src/bin/interactive.rs -L target --out-dir target
doc:
	rustdoc src/script.rs -o doc
clean:
	rm -rf target/*