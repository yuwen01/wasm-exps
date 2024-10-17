.PHONY: wasm

wasm:
	cargo build --target wasm32-wasip1 --no-default-features --release
	wasmtime target/wasm32-wasip1/release/fibonacci-script.wasm
