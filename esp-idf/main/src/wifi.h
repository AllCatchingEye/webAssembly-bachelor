#include "esp_wifi.h"
#include "wasm_export.h"

#define TRUE 1
#define FALSE 0

extern int connected_to_wifi;

void wifi_connect();
int get_wifi_status();
int get_wifi_status_wrapper(wasm_exec_env_t exec_env);
