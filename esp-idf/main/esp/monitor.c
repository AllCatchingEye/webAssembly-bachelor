#include "board.h"
#include "stdio.h"
#include <stdint.h>
#include <unistd.h>

int exports_esp_monitor_monitor_run() {
  printf("Initializing FIFO...\n");
  exports_esp_monitor_native_functions_fifo_init();
  exports_esp_monitor_native_functions_initilize_sensors();

  printf("Starting tcp server...\n");
  // start_server();

  int sleep_interval = 100000;
  while (1) {
    printf("Checking WiFi status...\n");
    int wifi_status = 2;
    wifi_status = exports_esp_monitor_native_functions_get_wifi_status();
    printf("Wifi status is: %d", wifi_status);
    if (wifi_status == 1) {
      printf("WiFi connected. Reading sensor values...\n");
      exports_esp_monitor_monitor_dht11_sensor_values_t dht11_values;
      exports_esp_monitor_monitor_read_dht11_sensor(&dht11_values);
      printf("Sensor values read:\n");
      printf("Temperature: %d, Humidity: %d, Status: %d\n",
             dht11_values.temperature, dht11_values.humidity,
             dht11_values.status);

      if (exports_esp_monitor_monitor_valid_values(&dht11_values) &&
          exports_esp_monitor_monitor_valid_status(&dht11_values)) {
        printf("Sensor values are valid. Building message...\n");
        char msg[200];
        uintptr_t msg_ptr = (uintptr_t)&msg;
        exports_esp_monitor_native_functions_build_message(
            msg_ptr, sizeof(msg), dht11_values.temperature,
            dht11_values.humidity);

        printf("Message built:\n%s\n", msg);
        printf("Putting message into queue...\n");
        exports_esp_monitor_native_functions_put(msg_ptr);
      } else {
        printf("Sensor values are invalid or status is not ok. Skipping "
               "sending...\n");
      }
    } else {
      printf("WiFi not connected. Skipping sensor read...\n");
    }

    printf("Sleeping for %d seconds...\n", sleep_interval);
    sleep(sleep_interval);
  }
}

void exports_esp_monitor_monitor_read_dht11_sensor(
    exports_esp_monitor_monitor_dht11_sensor_values_t *ret) {
  printf("Reading DHT11 sensor values...\n");
  ret->temperature = exports_esp_monitor_native_functions_read_temperature();
  ret->humidity = exports_esp_monitor_native_functions_read_humidity();
  ret->status = exports_esp_monitor_native_functions_read_status();
  printf("Sensor read complete: Temperature: %d, Humidity: %d, Status: %d\n",
         ret->temperature, ret->humidity, ret->status);
}

int exports_esp_monitor_monitor_valid_values(
    exports_esp_monitor_monitor_dht11_sensor_values_t *values) {
  const int max_temperature = 100;
  const int max_humidity = 100;
  const int min_temperature = -30;
  const int min_humidity = 0;
  printf("Validating sensor values...\n");
  if (values->temperature > max_temperature ||
      values->humidity > max_humidity) {
    printf("Invalid values: Temperature: %d, Humidity: %d\n",
           values->temperature, values->humidity);
    return 0;
  } else if (values->temperature < min_temperature ||
             values->humidity < min_humidity) {
    printf("Invalid values: Temperature: %d, Humidity: %d\n",
           values->temperature, values->humidity);
    return 0;
  } else {
    printf("Values are valid: Temperature: %d, Humidity: %d\n",
           values->temperature, values->humidity);
    return 1;
  }
}

int exports_esp_monitor_monitor_valid_status(
    exports_esp_monitor_monitor_dht11_sensor_values_t *values) {
  printf("Validating sensor status...\n");
  switch (values->status) {
  case -1:
    printf("Status code: %d - DHT11_TIMEOUT_ERROR\n", values->status);
    return 0;
  case -2:
    printf("Status code: %d - DHT11_CRC_ERROR\n", values->status);
    return 0;
  default:
    printf("Status code: %d - Status OK\n", values->status);
    return 1;
  }
}
