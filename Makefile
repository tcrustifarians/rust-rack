all: lib/librust_rack.bundle

lib/librust_rack.bundle: target/debug/librack.dylib
	ld -bundle -o lib/rust_rack.bundle target/debug/librack.dylib

target/debug/librack.dylib:
	cargo build

clean:
	rm -f lib/rust_rack.bundle
	cargo clean
