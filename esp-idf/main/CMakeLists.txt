# Copyright (C) 2021 Intel Corporation and others.  All rights reserved.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

idf_component_register(
                    SRCS "main.c" "src/wasm_helper_functions.c" 
                    "src/esp_client.c" "src/wifi.c" "src/sensors.c" 
                    "wasm/process_data/process_data.h" "wasm/process_data/process_data.c"
                    "src/fifo.c"
                    INCLUDE_DIRS "." "./src" "./wasm"
                    REQUIRES wamr esp32-DHT11 nvs_flash
                    PRIV_REQUIRES driver esp_http_client esp_wifi
                    )

