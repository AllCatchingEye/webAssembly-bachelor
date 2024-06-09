#include "sensors.h"
#include "dht11.h"
#include <stdio.h>
#include <sys/time.h>

void initilize_sensors() { DHT11_init(GPIO_NUM_4); }

int read_temperature_native(wasm_exec_env_t exec_env) {
  return DHT11_read().temperature;
}
int read_humidity_native(wasm_exec_env_t exec_env) {
  return DHT11_read().humidity;
}
int read_status_native(wasm_exec_env_t exec_env) { return DHT11_read().status; }

void build_message_native(wasm_exec_env_t exec_env, char *buffer,
                          int buffer_len, int temperature, int humidity) {
  snprintf(buffer, buffer_len,
           "{\"message_type\": \"dht11\", \"operation\": \"Insert\", "
           "\"temperature\": %d, \"humidity\": %d}",
           temperature, humidity);
}
