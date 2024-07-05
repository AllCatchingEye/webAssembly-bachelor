go mod init example.com

wit-bindgen tiny-go ../add.wit --world example --out-dir=gen

tinygo build -o add.wasm -target=wasi add.go

COMPONENT_ADAPTER_REACTOR=../../wasi_snapshot_preview1.reactor.wasm
wasm-tools component embed --world example ../add.wit add.wasm -o add.embed.wasm
wasm-tools component new -o ../components/go-add-component.wasm --adapt wasi_snapshot_preview1=$COMPONENT_ADAPTER_REACTOR add.embed.wasm
