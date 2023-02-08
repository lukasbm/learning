all: build

build: target/debug/libfibonacci.a app.c
	gcc app.c -o app -lfibonacci -L ./target/debug

target/debug/libfibonacci.a: fibonacci.rs Cargo.toml
	cargo build

clean:
	rm -rf ./target/
	rm -f app

run: build
	./app
