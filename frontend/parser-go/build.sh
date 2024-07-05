go mod init parser.com
wit-bindgen tiny-go ../wit/ --world parser --out-dir=gen

tinygo build -o ./gen/parser.wasm -target=wasi parser.go
wasm-tools component embed --world parser ../wit/ ./gen/parser.wasm -o ./gen/parser.embed.wasm
wasm-tools component new -o ../out/parser.wasm \
	--adapt wasi_snapshot_preview1=../wasi_snapshot_preview1.reactor.wasm \
	./gen/parser.embed.wasm
