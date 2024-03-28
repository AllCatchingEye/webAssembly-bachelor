cargo component build

wasm-tools compose ./target/wasm32-wasi/release/app.wasm \
	-d ../server/target/wasm32-wasi/release/server.wasm \
	-o ./out.wasm
