/*
 * Copyright (C) 2019-21 Intel Corporation and others.  All rights reserved.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include "bh_platform.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/queue.h"
#include "freertos/task.h"
#include "lib_export.h"
#include "platform_common.h"
#include "wasm_export.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

#include "src/esp_client.h"
#include "src/sensors.h"
#include "src/wasm_helper_functions.h"
#include "src/wifi.h"

#ifndef PROCESS_DATA_H
#define PROCESS_DATA_H

#include "wasm/process_data/process_data.h"

#endif /* HEADER_FILE_NAME_H */

#include "c_arrays/add_app.h"
#include "c_arrays/process_data.h"
#include "c_arrays/test_wasm.h"

#ifdef CONFIG_IDF_TARGET_ESP32S3
#define IWASM_MAIN_STACK_SIZE 5120
#else
#define IWASM_MAIN_STACK_SIZE 4096
#endif

#define QUEUE_SIZE 4
#define MSG_SIZE 128

void *iwasm_main(void *arg) {
  (void)arg; /* unused */

  wasm_t wasm = initilize_wasm();

  static NativeSymbol native_symbols[] = {
      // EXPORT_WASM_API_WITH_SIG(process_sensor_values, "(riii)")
  };
  // wasm_file_t wasm_file =
  //     initilize_wasm_file((uint8_t *)process_data, sizeof(process_data));
  wasm_file_t wasm_file =
      initilize_wasm_file((uint8_t *)add_app, sizeof(add_app));
  wasm_start(&wasm, &wasm_file, native_symbols);

  initilize_sensors();

  wifi_connect();

  FifoQueue_t queue = fifo_init();

  // /* creat an execution environment to execute the WASM functions */
  // ESP_LOGI(LOG_TAG, "Create exec env for func %s", "process_sensor_values");
  // wasm_exec_env_t exec_env =
  //     wasm_runtime_create_exec_env(wasm.wasm_module_inst, wasm.stack_size);

  // pthread_t tcp_thread;
  // int res;
  //
  // pthread_attr_t tcp_attr;
  // pthread_attr_init(&tcp_attr);
  // pthread_attr_setdetachstate(&tcp_attr, PTHREAD_CREATE_JOINABLE);
  // pthread_attr_setstacksize(&tcp_attr, IWASM_MAIN_STACK_SIZE);
  // res = pthread_create(&tcp_thread, &tcp_attr, tcp_client, (void *)&queue);
  // assert(res == 0);

  int sleep_interval = 10;
  while (1) {
    // dht11_values_t dht11_values = read_dht11_sensor();

    // if (connected_to_wifi == true) {
    //   process_sensor_values(exec_env, &queue, dht11_values.temperature,
    //                         dht11_values.humidity, dht11_values.status);
    //   // test_tcp(&queue);
    //   ESP_LOGI(LOG_TAG, "Connected to wifi, sending sensor values");
    //   // send_sensor_values(dht11_values);
    // } else {
    //   ESP_LOGI(LOG_TAG, "Can't send sensor values, not connected to wifi");
    // }
    ESP_LOGI(LOG_TAG,
             "Trying to call add function from component add_component...");

    wasm_application_execute_main(wasm.wasm_module_inst, 0, NULL);

    sleep(sleep_interval);
  }

  // res = pthread_join(tcp_thread, NULL);
  // assert(res == 0);

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
