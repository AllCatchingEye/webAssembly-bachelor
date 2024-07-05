cd ./guest/ && cargo build --target wasm32-wasi
cd ..

wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm \
	-o guest-component.wasm \
	--adapt ../wasi_snapshot_preview1.reactor.wasm

# wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm \
# 	-o virt-guest-component.wasm \
# 	--adapt ./wasi_snapshot_preview1.proxy.wasm

# wasi-virt ./guest-component.wasm \
# 	--allow-all \
# 	-e WASMTIME_BACKTRACE_DETAILS=1 \
# 	-o virt-guest-component.wasm

RUST_LOG=wasi_common=trace sudo cargo run
