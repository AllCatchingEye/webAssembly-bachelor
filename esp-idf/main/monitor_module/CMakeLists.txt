cmake_minimum_required(VERSION 3.5)
project(monitor_module)

set(WASI_SDK_PATH /opt/wasi-sdk-21.0)
set(CMAKE_C_COMPILER ${WASI_SDK_PATH}/bin/clang-17)
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

add_executable(monitor_module.wasm monitor_mod.c)

add_custom_command(
    TARGET monitor_module.wasm
    POST_BUILD
    COMMAND cp monitor_module.wasm ..
)
