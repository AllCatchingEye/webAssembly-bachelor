#include "sensors.h"
#include "dht11.h"
#include <sys/time.h>

void initilize_sensors() { DHT11_init(GPIO_NUM_4); }

dht11_values_t read_dht11_sensor() {
  dht11_values_t dht11_values;
  dht11_values.temperature = DHT11_read().temperature;
  dht11_values.humidity = DHT11_read().humidity;
  dht11_values.status = DHT11_read().status;

  return dht11_values;
}
