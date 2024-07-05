rm -rf ./plotter_guest
componentize-py -d ../wit/ -w plotter bindings plotter_guest
componentize-py -d ../wit/ -w wasi:cli/command@0.2.0 componentize plotter -o ../out/plotter.wasm
wasmtime run ../out/plotter.wasm
