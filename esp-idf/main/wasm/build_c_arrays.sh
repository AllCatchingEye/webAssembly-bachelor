#!/bin/bash

SCRIPT_SRC=$PWD

# build normal monitor module
cd ../monitor_module || exit

rm -rf build
mkdir -p build
cd build || exit

cmake ..
make
cd "$SCRIPT_SRC" || exit

# build monitor component using component model
cd ../monitor_component || exit
./build.sh
cd "$SCRIPT_SRC" || exit

# generate c arrays from wasm binaries
python3 wasm_c_array.py ../monitor_module/monitor_module.wasm monitor_module
python3 wasm_c_array.py ../monitor_component/monitor-core.wasm monitor_component
