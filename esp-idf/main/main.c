/*
 * Copyright (C) 2019-21 Intel Corporation and others.  All rights reserved.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include "bh_platform.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "wasm_export.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

#include "src/esp_client.h"
#include "src/sensors.h"
#include "src/wasm_helper_functions.h"
#include "src/wifi.h"

#include "c_arrays/process_data.h"
#include "c_arrays/test_wasm.h"

#ifdef CONFIG_IDF_TARGET_ESP32S3
#define IWASM_MAIN_STACK_SIZE 5120
#else
#define IWASM_MAIN_STACK_SIZE 4096
#endif

void *iwasm_main(void *arg) {
  (void)arg; /* unused */

  wasm_t wasm = initilize_wasm();
  wasm_file_t wasm_file =
      initilize_wasm_file((uint8_t *)process_data, sizeof(process_data));

  wasm_start(&wasm, &wasm_file);

  initilize_sensors();

  wifi_connect();

  int sleep_interval = 1;
  while (1) {
    dht11_values_t dht11_values = read_dht11_sensor();

    uint32 args[3] = {dht11_values.temperature, dht11_values.humidity,
                      dht11_values.status};
    wasm_run_func(&wasm, "process_sensor_values", 3, args);

    if (connected_to_wifi == true) {
      ESP_LOGI(LOG_TAG, "Connected to wifi, sending sensor values");
      send_sensor_values(dht11_values);
    } else {
      ESP_LOGI(LOG_TAG, "Can't send sensor values, not connected to wifi");
    }

    sleep(sleep_interval);
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
