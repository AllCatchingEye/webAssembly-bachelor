#include "esp_http_client.h"
#include "sensors.h"

void test_tcp(QueueHandle_t queue);
void *tcp_client(void *arg);
