cmake_minimum_required(VERSION 3.5)
project(udp_client)

include($ENV{WAMR_PATH}/core/iwasm/libraries/lib-socket/lib_socket_wasi.cmake)

set(WASI_SDK_PATH /opt/wasi-sdk-21.0)
set(CMAKE_C_COMPILER ${WASI_SDK_PATH}/bin/clang-17)
set(CMAKE_CXX_COMPILER ${CMAKE_C_COMPILER})
set(CMAKE_C_FLAGS "\
    --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot \
    -Wl,--strip-all \
    -Os \
    -Wl,--export=${WASI_EXPORT} \
    -Wl,--export=send_udp_message \
    -Wl,--no-entry \
    -Wl,--allow-undefined \
    -z stack-size=8192 \
    -Wl,--initial-memory=65536 \
    -Wl,--export=__heap_base \
    -Wl,--export=__data_end \
")
set(CMAKE_C_LINK_FLAGS "${CMAKE_C_FLAGS}")

set(WAMR_BUILD_LIBC_BUILTIN 1)
set(WAMR_BUILD_LIBC_WASI 1)
set(WAMR_BUILD_LIB_PTHREAD 1)

add_executable(udp_client.wasm udp_client.c)
target_include_directories(udp_client.wasm PRIVATE ./inc)
target_compile_options(udp_client.wasm INTERFACE -pthread)
target_link_libraries(udp_client.wasm socket_wasi_ext)

add_custom_command(
    TARGET udp_client.wasm
    POST_BUILD
    COMMAND cp udp_client.wasm /output
)
