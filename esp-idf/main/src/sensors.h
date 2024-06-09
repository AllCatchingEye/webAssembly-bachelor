#ifndef SENSORS_H
#define SENSORS_H
#include "wasm_export.h"
#include <stdint.h>

#endif

void initilize_sensors();

int read_temperature_native(wasm_exec_env_t exec_env);
int read_humidity_native(wasm_exec_env_t exec_env);
int read_status_native(wasm_exec_env_t exec_env);

void build_message_native(wasm_exec_env_t exec_env, char *buffer,
                          int buffer_len, int temperature, int humidity);
