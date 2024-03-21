#ifndef SENSORS_H
#define SENSORS_H

typedef struct {
  int temperature;
  int humidity;
  int status;
} dht11_values_t;

#endif

void initilize_sensors();

dht11_values_t read_dht11_sensor();
