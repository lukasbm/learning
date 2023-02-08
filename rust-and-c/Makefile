all: build

build: target/debug/libfibonacci.so
	g++ app.c -L ./target/debug/libfibonacci.so -o app

target/debug/libfibonacci.so: fibonacci.rs Cargo.toml
	cargo build

clean:
	rm -rf target
