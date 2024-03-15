#include "process_data.h"
#include "stdio.h"

void process_sensor_values(int temperature, int humidity, int status) {
  check_status(status);

  process_temperature(temperature);
  process_humidity(humidity);
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
