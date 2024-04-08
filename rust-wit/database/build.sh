cd ./guest/ && cargo build --target wasm32-wasi
cd ..

wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm \
	-o guest-component.wasm \
	--adapt ./wasi_snapshot_preview1.reactor.wasm

WASMTIME_BACKTRACE_DETAILS=1 RUST_LOG=wasi_common=trace cargo run
