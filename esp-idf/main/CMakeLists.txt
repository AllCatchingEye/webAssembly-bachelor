# Copyright (C) 2021 Intel Corporation and others.  All rights reserved.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

idf_component_register(
                    SRCS "main.c" "src/wasm_helper_functions.c" 
                    "src/esp_client.c" "src/wifi.c" "src/sensors.c" 
                    # "monitor_module/monitor_mod.h" "monitor_module/monitor_mod.c"
                    # "monitor_component/board.h" "monitor_component/monitor.c"
                    # "monitor_component/board.c" "monitor_component/board_component_type.o"
                    "src/fifo.c"
                    INCLUDE_DIRS "." "./src" "./wasm" "./monitor_component" "./monitor_module"
                    REQUIRES wamr esp32-DHT11 nvs_flash
                    PRIV_REQUIRES driver esp_http_client esp_wifi
                    )

