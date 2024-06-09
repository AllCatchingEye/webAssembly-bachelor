cmake_minimum_required(VERSION 3.5)
project(process_data)

set(WASI_SDK_PATH /opt/wasi-sdk-21.0)
set(CMAKE_C_COMPILER ${WASI_SDK_PATH}/bin/clang-17)
set(CMAKE_CXX_COMPILER ${CMAKE_C_COMPILER})
set(CMAKE_C_FLAGS "\
    --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot \
    -Wl,--strip-all \
    -Os \
    -Wl,--export=${WASI_EXPORT} \
    -Wl,--export=monitor_sensors \
    -Wl,--no-entry \
    -Wl,--allow-undefined \
    -z stack-size=8192 \
    -Wl,--initial-memory=65536 \
    -Wl,--export=__heap_base \
    -Wl,--export=__data_end \
")
set(CMAKE_C_LINK_FLAGS ${CMAKE_C_FLAGS})

add_executable(process_data.wasm process_data.c)

add_custom_command(
    TARGET process_data.wasm
    POST_BUILD
    COMMAND cp process_data.wasm ..
)
