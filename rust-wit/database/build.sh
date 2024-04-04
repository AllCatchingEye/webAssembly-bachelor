cd ./guest-component/ && cargo component build
cd ..

# wasm-tools component new ./target/wasm32-wasi/debug/guest_component.wasm \
# 	-o guest-component.wasm \
# 	--adapt ./wasi_snapshot_preview1.reactor.wasm

cargo run
