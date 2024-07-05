wit-bindgen c ./monitor.wit

rm -rf ./build/
mkdir ./build/

cd ./build/ || exit
cmake ..
make

rm ../../esp-idf/main/wasm/monitor-core.wasm
mv monitor-core.wasm ../../esp-idf/main/wasm/

wasm-tools component new monitor-core.wasm -o monitor-component.wasm
