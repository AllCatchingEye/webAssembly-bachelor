cmake_minimum_required(VERSION 3.5)
project(client)

set(WASI_SDK_PATH /opt/wasi-sdk-20.0)
set(CMAKE_C_COMPILER ${WASI_SDK_PATH}/bin/clang-16)
set(CMAKE_CXX_COMPILER ${CMAKE_C_COMPILER})
set(CMAKE_C_FLAGS "\
    --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot \
    -Wl,--strip-all \
    -Os \
    -Wl,--export=${WASI_EXPORT} \
    -Wl,--export= \
    -Wl,--no-entry \
    -Wl,--allow-undefined \
    -z stack-size=8192 \
    -Wl,--initial-memory=65536 \
    -Wl,--export=__heap_base \
    -Wl,--export=__data_end \
    -mexec-model=reactor \
    -L/usr/local/lib \
    -I/usr/local/include \
")
set(CMAKE_C_LINK_FLAGS "${CMAKE_C_FLAGS} -lcurl")


add_executable(client.wasm client.c)


add_custom_command(
    TARGET client.wasm
    POST_BUILD
    COMMAND cp client.wasm /output
)
