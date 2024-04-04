(cd ../adder && cargo component build --release)
(cd ../server && cargo component build --release)
cargo component build --release

wasm-tools compose --config ./config.yml target/wasm32-wasi/release/app.wasm \
	-d ../adder/target/wasm32-wasi/release/adder.wasm \
	-d ../server/target/wasm32-wasi/release/server.wasm \
	-o out.wasm

chmod +x out.wasm

sudo wasmtime run ./out.wasm
