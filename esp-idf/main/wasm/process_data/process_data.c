#include "process_data.h"
#include "stdio.h"

FifoQueue_t fifo_init() {
  FifoQueue_t queue;
  queue.read = 0;
  queue.write = 0;
  return queue;
}

int isFull(FifoQueue_t queue) {
  return ((queue.write + 1) % BUFFER_SIZE == queue.read);
}

int isEmpty(FifoQueue_t queue) { return (queue.read == queue.write); }

int put(FifoQueue_t *queue, char *msg) {
  if (isFull(*queue)) {
    printf("Error: Queue is full.\n");
    return -1;
  }
  queue->data[queue->write] = msg;
  queue->write = (queue->write + 1) % BUFFER_SIZE;
  return 0;
}

char *get(FifoQueue_t *queue) {
  if (isEmpty(*queue)) {
    printf("Error: Queue is empty.\n");
    return NULL;
  }
  char *msg = queue->data[queue->read];
  queue->read = (queue->read + 1) % BUFFER_SIZE;
  return msg;
}

void process_sensor_values(FifoQueue_t *queue, int temperature, int humidity,
                           int status) {
  if (valid_values(temperature, humidity) && valid_status(status)) {

    char *msg = build_message(temperature, humidity);
    put(queue, msg);
  }
}

int valid_values(int temperature, int humidity) {
  if (temperature > MAX_TEMPERATURE || humidity > MAX_HUMIDITY) {
    return 0;
  } else if (temperature < MIN_TEMPERATURE || humidity < MIN_HUMIDITY) {
    return 0;
  } else {
    return 1;
  }
}

int valid_status(int status) {
  switch (status) {
  case -1:
    printf("Status code: %d - DHT11_TIMEOUT_ERROR", status);
    return 0;
    break;
  case -2:
    printf("Status code: %d - DHT11_CRC_ERROR", status);
    return 0;
    break;
  default:
    return 1;
    break;
  }
}

char *build_message(int temperature, int humidity) {
  char *msg;
  asprintf(&msg,
           "\{\"message_type\": \"dht11\", \"operation\": \"Insert\", "
           "\"temperature\": \"%d\", \"humidity\": \"%d\"}",
           temperature, humidity);

  return msg;
}

// void process_humidity(int humidity) {
//   if (humidity > 60) {
//     printf("Humidity is too high with %d. Ventilate the room.\n", humidity);
//   } else if (humidity < 40) {
//     printf("Humidity is too low with %d. Moisturize the room.\n", humidity);
//   } else {
//     printf("Humidity is at a good value with %d.\n", humidity);
//   }
// }
//
// void process_temperature(int temperature) {
//   if (temperature > 30) {
//     printf("Temperature is too high with %d. Cool the room.\n", temperature);
//   } else if (temperature < 20) {
//     printf("Temperature is too low with %d. Heat the room.\n", temperature);
//   } else {
//     printf("Temperature is at a good value with %d.\n", temperature);
//   }
// }
