#include "process_data.h"
#include "stdio.h"
#include <unistd.h>

dht11_values_t read_dht11_sensor() {
  printf("Reading DHT11 sensor values...\n");
  dht11_values_t dht11_values;
  dht11_values.temperature = read_temperature();
  dht11_values.humidity = read_humidity();
  dht11_values.status = read_status();
  printf("Sensor read complete: Temperature: %d, Humidity: %d, Status: %d\n",
         dht11_values.temperature, dht11_values.humidity, dht11_values.status);
  return dht11_values;
}

void monitor_sensors() {
  printf("Initializing FIFO...\n");
  fifo_init();

  // printf("Starting tcp server...\n");
  // start_server();

  int sleep_interval = 100000;
  while (1) {
    printf("Checking WiFi status...\n");
    if (get_wifi_status() == TRUE) {
      printf("WiFi connected. Reading sensor values...\n");
      dht11_values_t dht11_values = read_dht11_sensor();
      printf("Sensor values read:\n");
      printf("Temperature: %d, Humidity: %d, Status: %d\n",
             dht11_values.temperature, dht11_values.humidity,
             dht11_values.status);

      if (valid_values(dht11_values) && valid_status(dht11_values)) {
        printf("Sensor values are valid. Building message...\n");
        char msg[200];
        build_message(msg, sizeof(msg), dht11_values.temperature,
                      dht11_values.humidity);

        printf("Message built:\n%s\n", msg);
        printf("Putting message into queue...\n");
        put(msg);
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

int valid_values(dht11_values_t dht11_values) {
  printf("Validating sensor values...\n");
  if (dht11_values.temperature > MAX_TEMPERATURE ||
      dht11_values.humidity > MAX_HUMIDITY) {
    printf("Invalid values: Temperature: %d, Humidity: %d\n",
           dht11_values.temperature, dht11_values.humidity);
    return 0;
  } else if (dht11_values.temperature < MIN_TEMPERATURE ||
             dht11_values.humidity < MIN_HUMIDITY) {
    printf("Invalid values: Temperature: %d, Humidity: %d\n",
           dht11_values.temperature, dht11_values.humidity);
    return 0;
  } else {
    printf("Values are valid: Temperature: %d, Humidity: %d\n",
           dht11_values.temperature, dht11_values.humidity);
    return 1;
  }
}

int valid_status(dht11_values_t dht11_values) {
  printf("Validating sensor status...\n");
  switch (dht11_values.status) {
  case -1:
    printf("Status code: %d - DHT11_TIMEOUT_ERROR\n", dht11_values.status);
    return 0;
  case -2:
    printf("Status code: %d - DHT11_CRC_ERROR\n", dht11_values.status);
    return 0;
  default:
    printf("Status code: %d - Status OK\n", dht11_values.status);
    return 1;
  }
}
