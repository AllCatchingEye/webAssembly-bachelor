#!/bin/bash

build_wasm() {
	echo "Building wasm for $1..."
	cd "$1"
	mkdir -p build && cd build
	cmake ..
	sudo make
	cd .. && cd ..
	echo "Done."
}

build_c_array() {
	build_wasm "$1"

	echo "Building c_array for $1..."
	python wasm_c_array.py "$2" "$3"
	echo "Done."
}

build_c_array ./add/ ./add/build/add.wasm add
build_c_array ./process_data/ ./process_data/build/process_data.wasm process_data
