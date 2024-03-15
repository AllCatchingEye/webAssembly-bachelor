#include "process_data.h"
#include "arpa/inet.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"
#include "sys/socket.h"
#include "unistd.h"

#ifdef __wasi__
#include <wasi_socket_ext.h>
#endif

#define PORT 34254

static void init_sockaddr_inet(struct sockaddr_in *addr) {
  /* 127.0.0.1:1234 */
  addr->sin_family = AF_INET;
  addr->sin_port = htons(1234);
  addr->sin_addr.s_addr = htonl(INADDR_LOOPBACK);
}

int send_values(int temperature, int humidity) {
  int socket_fd, ret, af;
  char buffer[1024] = {0};
  socklen_t serverlen;
  struct sockaddr_storage server_address = {0};
  const char *message = "Hello from client";

  af = AF_INET;
  init_sockaddr_inet((struct sockaddr_in *)&server_address);
  serverlen = sizeof(struct sockaddr_in);

  printf("[Client] Create socket\n");
  socket_fd = socket(af, SOCK_DGRAM, 0);
  if (socket_fd == -1) {
    perror("Create socket failed");
    return EXIT_FAILURE;
  }

  printf("[Client] Client send\n");
  ret = sendto(socket_fd, message, strlen(message), 0,
               (struct sockaddr *)&server_address, serverlen);
  if (ret < 0) {
    close(socket_fd);
    perror("Send failed");
    return EXIT_FAILURE;
  }

  printf("[Client] Client receive\n");
  serverlen = sizeof(server_address);
  /* make sure there is space for the string terminator */
  ret = recvfrom(socket_fd, buffer, sizeof(buffer) - 1, 0,
                 (struct sockaddr *)&server_address, &serverlen);

  if (ret > 0) {
    buffer[ret] = '\0';
    printf("[Client] Buffer recieved: %s\n", buffer);
  }

  close(socket_fd);
  printf("[Client] BYE \n");
  return EXIT_SUCCESS;
}

void process_sensor_values(int temperature, int humidity, int status) {
  check_status(status);

  process_temperature(temperature);
  process_humidity(humidity);

  send_values(temperature, humidity);
}

void check_status(int status) {
  switch (status) {
  case -1:
    printf("Status code: %d - DHT11_TIMEOUT_ERROR", status);
    return;
  case -2:
    printf("Status code: %d - DHT11_CRC_ERROR", status);
    return;
  default:
    break;
  }
}

void process_humidity(int humidity) {
  if (humidity > 60) {
    printf("Humidity is too high with %d. Ventilate the room.\n", humidity);
  } else if (humidity < 40) {
    printf("Humidity is too low with %d. Moisturize the room.\n", humidity);
  } else {
    printf("Humidity is at a good value with %d.\n", humidity);
  }
}

void process_temperature(int temperature) {
  if (temperature > 30) {
    printf("Temperature is too high with %d. Cool the room.\n", temperature);
  } else if (temperature < 20) {
    printf("Temperature is too low with %d. Heat the room.\n", temperature);
  } else {
    printf("Temperature is at a good value with %d.\n", temperature);
  }
}
