wit-bindgen c ./monitor.wit

rm -rf ./build/
mkdir ./build/

cd ./build/ || exit
cmake ..
make

cd .. || exit

# wasm-tools component new ./monitor-core.wasm -o monitor-component.wasm --adapt wasi_snapshot_preview1.reactor.wasm
