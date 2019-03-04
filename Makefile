run: clean build
	./example/hello

clean:
	cargo clean
	rm -f ./example/hello

build:
	cargo build --release
	gcc -o ./example/hello ./example/hello.c target/release/libffi.a
