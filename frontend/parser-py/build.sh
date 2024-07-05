rm -rf ./parser
componentize-py -d ../wit/ -w parser bindings ./
componentize-py -d ../wit/ -w parser componentize app -o ../out/client.wasm
