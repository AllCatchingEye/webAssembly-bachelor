# (cd parser-js && ./build.sh)
# (cd parser-py && ./build.sh)
(cd parser-go && ./build.sh)
(cd client && ./build.sh)
(cd webserver && ./build.sh)

wac plug ./out/webserver.wasm --plug ./out/parser.wasm --plug ./out/client.wasm -o ./out/plugged.wasm
cp -r ./out/plugged.wasm ./webserver/

(cd webserver && ./run.sh)
# wasm-tools compose ./out/webserver.wasm -d ./out/client.wasm -d ./out/parser.wasm -o ./out/composed.wasm

# sudo wasmtime run --wasi inherit-network plugged.wasm
# python3 -m wasmtime.bindgen ./out/plugged.wasm --out-dir ./webserver/webserver_host
