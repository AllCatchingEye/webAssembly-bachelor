rm -rf ./guest
componentize-py -d ./webserver.wit -w test bindings ./guest
componentize-py -d ./webserver.wit -w test componentize --stub-wasi app -o host-webserver.wasm

python3 -m wasmtime.bindgen host-webserver.wasm --out-dir host_part
