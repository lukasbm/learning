all: build

build: target/debug/libfibonacci.so app.c
	gcc app.c -o app -lfibonacci -L ./target/debug

target/debug/libfibonacci.so: fibonacci.rs Cargo.toml
	cargo build

clean:
	rm -rf ./target/
	rm -f app


run: build
	LD_LIBRARY_PATH=target/debug ./app
