/* HTTP GET Example using plain POSIX sockets

   This example code is in the Public Domain (or CC0 licensed, at your option.)

   Unless required by applicable law or agreed to in writing, this
   software is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
   CONDITIONS OF ANY KIND, either express or implied.
*/
#include "esp_event.h"
#include "esp_log.h"
#include "freertos/task.h"
#include <stdio.h>
#include <string.h>

#include "lwip/netdb.h"
#include "lwip/sockets.h"

#include "sensors.h"

#define MAX_HTTP_OUTPUT_BUFFER 2048

/* Constants that aren't configurable in menuconfig */
#define WEB_SERVER "192.168.0.217"
#define WEB_PORT "8000"
#define WEB_PATH "/dht11_values"

static const char *TAG = "ESP_CLIENT";

static void http_post_task(void *pvParameters) {
  dht11_values_t *dht11_values = (dht11_values_t *)pvParameters;

  char *query_string;
  asprintf(&query_string, "temperature=%d&humidity=%d",
           dht11_values->temperature, dht11_values->humidity);
  int content_length = strlen(query_string);

  char *request_string = "POST " WEB_PATH " HTTP/1.1\r\n"
                         "Host: " WEB_SERVER ":" WEB_PORT "\r\n"
                         "User-Agent: esp-idf/1.0 esp32\r\n"
                         "Content-Type: application/x-www-form-urlencoded\r\n"
                         "Connection: close\r\n"
                         "Content-Length: %d\r\n"
                         "\r\n"
                         "%s\r\n";
  char *REQUEST;
  asprintf(&REQUEST, request_string, content_length, query_string);
  ESP_LOGI(TAG, "Created Request:\n%s", REQUEST);

  const struct addrinfo hints = {
      .ai_family = AF_INET,
      .ai_socktype = SOCK_STREAM,
  };
  struct addrinfo *res;
  struct in_addr *addr;
  int s, r;
  char recv_buf[64];

  int err = getaddrinfo(WEB_SERVER, WEB_PORT, &hints, &res);

  if (err != 0 || res == NULL) {
    ESP_LOGE(TAG, "DNS lookup failed err=%d res=%p", err, res);
    vTaskDelay(1000 / portTICK_PERIOD_MS);
  }

  /* Code to print the resolved IP.

     Note: inet_ntoa is non-reentrant, look at ipaddr_ntoa_r for "real" code */
  addr = &((struct sockaddr_in *)res->ai_addr)->sin_addr;
  ESP_LOGI(TAG, "DNS lookup succeeded. IP=%s", inet_ntoa(*addr));

  s = socket(res->ai_family, res->ai_socktype, 0);
  if (s < 0) {
    ESP_LOGE(TAG, "... Failed to allocate socket.");
    freeaddrinfo(res);
    vTaskDelay(1000 / portTICK_PERIOD_MS);
  }
  ESP_LOGI(TAG, "... allocated socket");

  if (connect(s, res->ai_addr, res->ai_addrlen) != 0) {
    ESP_LOGE(TAG, "... socket connect failed errno=%d", errno);
    close(s);
    freeaddrinfo(res);
    vTaskDelay(4000 / portTICK_PERIOD_MS);
  }

  ESP_LOGI(TAG, "... connected");
  freeaddrinfo(res);

  if (write(s, REQUEST, strlen(REQUEST)) < 0) {
    ESP_LOGE(TAG, "... socket send failed");
    close(s);
    vTaskDelay(4000 / portTICK_PERIOD_MS);
  }
  ESP_LOGI(TAG, "... socket send success");

  struct timeval receiving_timeout;
  receiving_timeout.tv_sec = 5;
  receiving_timeout.tv_usec = 0;
  if (setsockopt(s, SOL_SOCKET, SO_RCVTIMEO, &receiving_timeout,
                 sizeof(receiving_timeout)) < 0) {
    ESP_LOGE(TAG, "... failed to set socket receiving timeout");
    close(s);
    vTaskDelay(4000 / portTICK_PERIOD_MS);
  }
  ESP_LOGI(TAG, "... set socket receiving timeout success");

  /* Read HTTP response */
  do {
    bzero(recv_buf, sizeof(recv_buf));
    r = read(s, recv_buf, sizeof(recv_buf) - 1);
    for (int i = 0; i < r; i++) {
      putchar(recv_buf[i]);
    }
  } while (r > 0);

  ESP_LOGI(TAG, "... done reading from socket. Last read return=%d errno=%d.",
           r, errno);
  close(s);
  for (int countdown = 10; countdown >= 0; countdown--) {
    ESP_LOGI(TAG, "%d... ", countdown);
    vTaskDelay(1000 / portTICK_PERIOD_MS);
  }

  vTaskDelete(NULL);
}

void send_sensor_values(dht11_values_t dht11_values) {
  xTaskCreate(&http_post_task, "http_post_task", 4096, &dht11_values, 5, NULL);
}
