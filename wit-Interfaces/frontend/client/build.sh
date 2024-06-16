rm -rf ./tcp_client
componentize-py -d ../wit/ -w tcp-client bindings ./
componentize-py -d ../wit/ -w tcp-client componentize app -o ../out/client.wasm
