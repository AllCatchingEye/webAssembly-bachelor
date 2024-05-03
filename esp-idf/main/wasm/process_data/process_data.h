#include "wasm_export.h"

#define MAX_TEMPERATURE 50
#define MAX_HUMIDITY 100

#define MIN_TEMPERATURE 0
#define MIN_HUMIDITY 0

#define BUFFER_SIZE 10

typedef struct {
  char *data[BUFFER_SIZE];
  int read;
  int write;
} FifoQueue_t;

FifoQueue_t fifo_init();
int isFull(FifoQueue_t queue);
int isEmpty(FifoQueue_t queue);

int put(FifoQueue_t *queue, char *msg);
char *get(FifoQueue_t *queue);

void process_sensor_values(wasm_exec_env_t exec_env, FifoQueue_t *queue,
                           int temperature, int humidity, int status);

int valid_values(int temperature, int humidity);
int valid_status(int status);

char *build_message(int temperature, int humidity);

/* void process_temperature(int temperature); */
/* void process_humidity(int humidity); */
