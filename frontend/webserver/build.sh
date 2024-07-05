rm -rf ./webserver/
componentize-py -d ../wit/ -w webserver bindings ./
componentize-py -d ../wit/ -w webserver componentize app -o ../out/webserver.wasm
