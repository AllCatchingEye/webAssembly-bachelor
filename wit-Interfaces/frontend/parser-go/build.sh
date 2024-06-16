go mod init parser.com
wit-bindgen tiny-go ../wit/ --world parser --out-dir=gen

tinygo build -o parser.wasm -target=wasi parser.go
wasm-tools component embed --world parser ../wit/ parser.wasm -o parser.embed.wasm
wasm-tools component new -o ../out/parser.wasm \
	--adapt wasi_snapshot_preview1=../wasi_snapshot_preview1.reactor.wasm \
	parser.embed.wasm
