wit-bindgen c ./wit/add.wit

CC=/opt/wasi-sdk-22.0/bin/clang
$CC add.c example.c example_component_type.o -o add-core.wasm -mexec-model=reactor

wasm-tools component new add-core.wasm -o add-component.wasm
