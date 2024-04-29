#ifndef PROCESS_DATA_H
#define PROCESS_DATA_H

#include "../wasm/process_data/process_data.h"

#endif /* HEADER_FILE_NAME_H */

#include "esp_http_client.h"
#include "sensors.h"

void test_tcp(FifoQueue_t *queue);
void *tcp_client(void *arg);
