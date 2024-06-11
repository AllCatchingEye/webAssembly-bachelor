wit-bindgen tiny-go ../wit --world frontend --out-dir=gen

tinygo build -o add.wasm -target=wasi add.go
COMPONENT_ADAPTER_REACTOR=../wasi_snapshot_preview1.command.wasm
wasm-tools component embed --world frontend ../wit add.wasm -o add.embed.wasm
wasm-tools component new -o add.component.wasm --adapt wasi_snapshot_preview1="$COMPONENT_ADAPTER_REACTOR" add.embed.wasm

wasi-virt add.component.wasm --stderr=allow --debug --allow-sockets -o add.virt.wasm
