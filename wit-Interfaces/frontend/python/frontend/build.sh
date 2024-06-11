componentize-py -d wit/ -w wasi:cli/command@0.2.0 componentize app -o app.wasm

# wasi-virt app.wasm --allow-all -o virt.wasm
# wasm-tools compose app.wasm -d virt.wasm -o component.virt.wasm
