#!/bin/bash

clean_build() {
	cd "$1" || exit
	echo "Cleaning build in $1 ..."
	sudo rm -rf build
	echo "Done."
	cd ..
}

build_wasm() {
	echo "Building wasm for $1 ..."
	cd "$1" || exit
	mkdir -p build && cd build || exit

	if [ -z "$2" ]; then
		cmake ..
	else
		local COMMAND_OUTPUT="cmake $2 .."
		echo "Running cmake command: $COMMAND_OUTPUT"
		cmake $2 ..
	fi

	sudo make
	cd .. && cd ..
	echo "Done."
}

build_c_array() {
	clean_build "$1"

	if [ -z "$2" ]; then
		build_wasm "$1"
	else
		build_wasm "$1" "$4"
	fi

	echo "Building c_array for $1..."
	python wasm_c_array.py "$2" "$3"
	echo "Done."
}

# build_c_array ./add/ ./add/build/add.wasm add
build_c_array ./process_data/ ./process_data/build/process_data.wasm process_data
python wasm_c_array.py ./add-app.wasm add_app
