/*
 * SPDX-FileCopyrightText: 2022 Espressif Systems (Shanghai) CO LTD
 *
 * SPDX-License-Identifier: Unlicense OR CC0-1.0
 */
#include "esp_client.h"
#include "esp_log.h"
#include "esp_netif.h"
#include "sdkconfig.h"
#include <arpa/inet.h>
#include <errno.h>
#include <netdb.h> // struct addrinfo
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define HOST_IP_ADDR "192.168.0.217"
#define PORT 8080

static const char *TAG = "CLIENT";

void *tcp_client(void *arg) {
  FifoQueue_t *queue = (FifoQueue_t *)arg;

  char host_ip[] = HOST_IP_ADDR;

  struct sockaddr_in dest_addr;
  inet_pton(AF_INET, host_ip, &dest_addr.sin_addr);
  dest_addr.sin_family = AF_INET;
  dest_addr.sin_port = htons(PORT);
  int addr_family = AF_INET;
  int ip_protocol = IPPROTO_IP;

  while (1) {
    if (!isEmpty(*queue)) {
      char *payload = get(queue);

      ESP_LOGI(TAG, "Read message:\n%s", payload);

      ESP_LOGI(TAG, "Message in queue, creating socket...");
      int sock = socket(addr_family, SOCK_STREAM, ip_protocol);
      if (sock < 0) {
        ESP_LOGE(TAG, "Unable to create socket: errno %d", errno);
        return NULL;
      }
      ESP_LOGI(TAG, "Socket created, connecting to %s:%d", host_ip, PORT);

      int conn_err =
          connect(sock, (struct sockaddr *)&dest_addr, sizeof(dest_addr));
      if (conn_err != 0) {
        ESP_LOGE(TAG, "Socket unable to connect: errno %d", errno);
        continue;
      }
      ESP_LOGI(TAG, "Successfully connected");

      int send_err = send(sock, payload, strlen(payload), 0);
      if (send_err < 0) {
        ESP_LOGE(TAG, "Error occurred during sending: errno %d", errno);
      }

      ESP_LOGI(TAG, "Shutting down socket and restarting...");
      shutdown(sock, 0);
      close(sock);
    }
  }
  return NULL;
}

void test_tcp(FifoQueue_t *queue) {
  char *payload = "{\"message_type\": \"test\", \"operation\": \"Insert\", "
                  "\"name\": \"Bob\"}";
  put(queue, payload);

  payload = "{\"message_type\": \"test\", \"operation\": \"Insert\", \"name\": "
            "\"Alice\"}";
  put(queue, payload);

  payload = "{\"message_type\": \"test\", \"operation\": \"Delete\", \"name\": "
            "\"Bob\"}";
  put(queue, payload);
}
