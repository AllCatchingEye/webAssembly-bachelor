rm -rf ./guest
componentize-py -d ../wit/ -w host-webserver bindings ./guest
componentize-py -d ../wit/ -w host-webserver componentize app -o ../out/host-webserver.wasm

python3 -m wasmtime.bindgen ../out/host-webserver.wasm --out-dir host
