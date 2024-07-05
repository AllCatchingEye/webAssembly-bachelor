/*
 * Copyright (C) 2019-21 Intel Corporation and others.  All rights reserved.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include "bh_platform.h"
#include "esp_log.h"
#include "freertos/queue.h"
#include "freertos/task.h"
#include "lib_export.h"
#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

#include "src/esp_client.h"
#include "src/fifo.h"
#include "src/sensors.h"
#include "src/wasm_helper_functions.h"
#include "src/wifi.h"

#include "c_arrays/add.h"
#include "c_arrays/add_core.h"
#include "c_arrays/monitor.h"
#include "c_arrays/process_data.h"
#include "c_arrays/test_wasm.h"

// #ifdef CONFIG_IDF_TARGET_ESP32S3
#define IWASM_MAIN_STACK_SIZE 5120
// #else
// #define IWASM_MAIN_STACK_SIZE 4096
// #endif

void *iwasm_main(void *arg) {
  (void)arg; /* unused */

  int stack_size = 8 * 1024;
  int heap_size = 8 * 1024;

  wasm_t wasm = initilize_wasm(stack_size, heap_size);

  static NativeSymbol native_symbols[] = {
      // {"start_server", start_server_wrapper, "()"},
      {"exports_esp_monitor_native_functions_initilize_sensors",
       initilize_sensors_native, "()"},
      {"exports_esp_monitor_native_functions_read_temperature",
       read_temperature_native, "()i"},
      {"exports_esp_monitor_native_functions_read_humidity",
       read_humidity_native, "()i"},
      {"exports_esp_monitor_native_functions_read_status", read_status_native,
       "()i"},
      {"exports_esp_monitor_native_functions_build_message",
       build_message_native, "(*~ii)"},
      {"exports_esp_monitor_native_functions_get_wifi_status",
       get_wifi_status_wrapper, "()i"},
      {"exports_esp_monitor_native_functions_put", put_wrapper, "($)i"},
      {"exports_esp_monitor_native_functions_fifo_init", fifo_init_wrapper,
       "()"},
  };

  wasm_file_t wasm_file =
      initilize_wasm_file((uint8_t *)monitor, sizeof(monitor));

  // void *ret;
  RuntimeInitArgs init_args;

  /* configure memory allocation */
  memset(&init_args, 0, sizeof(RuntimeInitArgs));
#if WASM_ENABLE_GLOBAL_HEAP_POOL == 0
  init_args.mem_alloc_type = Alloc_With_Allocator;
  init_args.mem_alloc_option.allocator.malloc_func = (void *)os_malloc;
  init_args.mem_alloc_option.allocator.realloc_func = (void *)os_realloc;
  init_args.mem_alloc_option.allocator.free_func = (void *)os_free;
#else
#error The usage of a global heap pool is not implemented yet for esp-idf.
#endif

  char error_buf[128];
  ESP_LOGI(LOG_TAG, "Initialize WASM runtime");
  /* initialize runtime environment */
  if (!wasm_runtime_full_init(&init_args)) {
    ESP_LOGE(LOG_TAG, "Init runtime failed.");
    return NULL;
  }

  int n_native_symbols = sizeof(native_symbols) / sizeof(NativeSymbol);
  ESP_LOGI(LOG_TAG, "Number of native symbols: %d\n", n_native_symbols);
  if (!wasm_runtime_register_natives("env", native_symbols, n_native_symbols)) {

    ESP_LOGE(LOG_TAG, "Registering native functions failed");
    return NULL;
  }

  /* load WASM module */
  if (!(wasm.wasm_module = wasm_runtime_load(wasm_file.wasm_file_buf,
                                             wasm_file.wasm_file_buf_size,
                                             error_buf, sizeof(error_buf)))) {
    ESP_LOGE(LOG_TAG, "Error in wasm_runtime_load: %s", error_buf);
  }

  ESP_LOGI(LOG_TAG, "Instantiate WASM runtime");
  if (!(wasm.wasm_module_inst = wasm_runtime_instantiate(
            wasm.wasm_module, wasm.stack_size, // stack size
            wasm.heap_size,                    // heap size
            error_buf, sizeof(error_buf)))) {
    ESP_LOGE(LOG_TAG, "Error while instantiating: %s", error_buf);
  }

  ESP_LOGI(LOG_TAG, "Connecting to wifi...");
  wifi_connect();

  while (get_wifi_status() != 1) {
    ESP_LOGI(LOG_TAG, "Waiting for connection...");
    sleep(1);
  }

  pthread_t tcp_thread;
  int res;

  pthread_attr_t tcp_attr;
  pthread_attr_init(&tcp_attr);
  pthread_attr_setdetachstate(&tcp_attr, PTHREAD_CREATE_JOINABLE);
  pthread_attr_setstacksize(&tcp_attr, IWASM_MAIN_STACK_SIZE);

  res = pthread_create(&tcp_thread, &tcp_attr, tcp_client, (void *)NULL);
  assert(res == 0);

  ESP_LOGI(LOG_TAG, "Starting sensor monitor");
  uint32 args[1];

  wasm_run_func(&wasm, "esp:monitor/monitor@0.1.0#run", 0, args);

  res = pthread_join(tcp_thread, NULL);
  assert(res == 0);

  wasm_end(&wasm);

  return NULL;
}

void app_main(void) {
  pthread_t t;
  int res;

  pthread_attr_t tattr;
  pthread_attr_init(&tattr);
  pthread_attr_setdetachstate(&tattr, PTHREAD_CREATE_JOINABLE);
  pthread_attr_setstacksize(&tattr, IWASM_MAIN_STACK_SIZE);

  res = pthread_create(&t, &tattr, iwasm_main, (void *)NULL);
  assert(res == 0);

  res = pthread_join(t, NULL);
  assert(res == 0);

  ESP_LOGI(LOG_TAG, "Exiting...");
}
