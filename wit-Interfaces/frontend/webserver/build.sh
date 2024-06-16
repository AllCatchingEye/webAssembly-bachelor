rm -rf ./webserver/
componentize-py -d ../wit/ -w webserver bindings ./
componentize-py -d ../wit/ -w webserver componentize --stub-wasi app -o ../out/webserver.wasm
