cmake_minimum_required(VERSION 3.5)
project(monitor)

set(WASI_SDK_PATH /opt/wasi-sdk-22.0)
set(CMAKE_C_COMPILER ${WASI_SDK_PATH}/bin/clang-18)
set(CMAKE_CXX_COMPILER ${CMAKE_C_COMPILER})
set(CMAKE_C_FLAGS "\
    --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot \
    -Wl,--strip-all \
    -Os \
    -Wl,--export=${WASI_EXPORT} \
    -Wl,--export=all \
    -Wl,--no-entry \
    -Wl,--allow-undefined \
    -z stack-size=8192 \
    -Wl,--initial-memory=65536 \
    -Wl,--export=__heap_base \
    -Wl,--export=__data_end \
")
set(CMAKE_C_LINK_FLAGS ${CMAKE_C_FLAGS})

add_executable(monitor-core.wasm monitor.c board.c board_component_type.o)

add_custom_command(
    TARGET monitor-core.wasm
    POST_BUILD
    COMMAND cp monitor-core.wasm ..
)
