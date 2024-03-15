/*
 * Copyright (C) 2019-21 Intel Corporation and others.  All rights reserved.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include "bh_platform.h"
#include "driver/gpio.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "test_wasm.h"
#include "wasm_export.h"
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>

#include "c_arrays/process_data.h"

#include "dht11.h"
#include "esp_log.h"

#ifdef CONFIG_IDF_TARGET_ESP32S3
#define IWASM_MAIN_STACK_SIZE 5120
#else
#define IWASM_MAIN_STACK_SIZE 4096
#endif

#define LOG_TAG "wamr"

struct wasm_struct {
  wasm_module_inst_t wasm_module_inst;
  wasm_module_t wasm_module;
  uint32 stack_size;
  uint32 heap_size;
};
typedef struct wasm_struct Wasm;

struct wasm_file {
  uint8_t *wasm_file_buf;
  unsigned wasm_file_buf_size;
};
typedef struct wasm_file Wasm_file;

// static void *
// app_instance_main(wasm_module_inst_t module_inst)
// {
//     const char *exception;
//
//     ESP_LOGI(LOG_TAG, "Loading main...");
//     wasm_application_execute_main(module_inst, 0, NULL);
//     if ((exception = wasm_runtime_get_exception(module_inst)))
//         printf("%s\n", exception);
//     return NULL;
// }

int wasm_run_func(Wasm *wasm, char *func_name, uint32 args[]) {
  ESP_LOGI(LOG_TAG, "lookup func square...");
  /* lookup a WASM function by its name
     The function signature can NULL here */
  wasm_function_inst_t func =
      wasm_runtime_lookup_function(wasm->wasm_module_inst, func_name, NULL);

  /* creat an execution environment to execute the WASM functions */
  ESP_LOGI(LOG_TAG, "create exec env for func");
  wasm_exec_env_t exec_env =
      wasm_runtime_create_exec_env(wasm->wasm_module_inst, wasm->stack_size);

  /* call the WASM function */
  ESP_LOGI(LOG_TAG, "Calling wasm function square...");
  if (wasm_runtime_call_wasm(exec_env, func, 3, args)) {
    /* the return value is stored in argv[0] */
    printf("Function return: %ld\n", args[0]);
  } else {
    /* exception is thrown if call fails */
    printf("%s\n", wasm_runtime_get_exception(wasm->wasm_module_inst));
  }

  wasm_runtime_destroy_exec_env(exec_env);

  return args[0];
}

void wasm_end(Wasm *wasm) {
  /* destroy the module instance */
  ESP_LOGI(LOG_TAG, "Deinstantiate WASM runtime");
  wasm_runtime_deinstantiate(wasm->wasm_module_inst);

  /* unload the module */
  ESP_LOGI(LOG_TAG, "Unload WASM module");
  wasm_runtime_unload(wasm->wasm_module);

  /* destroy runtime environment */
  ESP_LOGI(LOG_TAG, "Destroy WASM runtime");
  wasm_runtime_destroy();
}

RuntimeInitArgs wasm_init_args() {
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

  return init_args;
}

void wasm_start(Wasm *wasm, Wasm_file *wasm_file) {
  /* setup variables for instantiating and running the wasm module */

  char error_buf[128];

  RuntimeInitArgs init_args = wasm_init_args();

  ESP_LOGI(LOG_TAG, "Initialize WASM runtime");
  /* initialize runtime environment */
  if (!wasm_runtime_full_init(&init_args)) {
    ESP_LOGE(LOG_TAG, "Init runtime failed.");
    return;
  }

  /* load WASM module */
  if (!(wasm->wasm_module = wasm_runtime_load(wasm_file->wasm_file_buf,
                                              wasm_file->wasm_file_buf_size,
                                              error_buf, sizeof(error_buf)))) {
    ESP_LOGE(LOG_TAG, "Error in wasm_runtime_load: %s", error_buf);
  }

  ESP_LOGI(LOG_TAG, "Instantiate WASM runtime");
  if (!(wasm->wasm_module_inst = wasm_runtime_instantiate(
            wasm->wasm_module, wasm->stack_size, // stack size
            wasm->heap_size,                     // heap size
            error_buf, sizeof(error_buf)))) {
    ESP_LOGE(LOG_TAG, "Error while instantiating: %s", error_buf);
  }
}

void *iwasm_main(void *arg) {
  (void)arg; /* unused */

  Wasm wasm;
  wasm.wasm_module = NULL;
  wasm.wasm_module_inst = NULL;
  wasm.stack_size = 32 * 1024;
  wasm.heap_size = 32 * 1024;

  Wasm_file wasm_file;
  wasm_file.wasm_file_buf = (uint8_t *)process_data;
  wasm_file.wasm_file_buf_size = sizeof(process_data);

  wasm_start(&wasm, &wasm_file);

  // read dht11 sensor values
  DHT11_init(GPIO_NUM_4);

  while (1) {
    int temperature = DHT11_read().temperature;
    int humidity = DHT11_read().humidity;
    int status = DHT11_read().status;

    uint32 args[3] = {temperature, humidity, status};
    wasm_run_func(&wasm, "process_sensor_values", args);

    sleep(1);
  }

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
