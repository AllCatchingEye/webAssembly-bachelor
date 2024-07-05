
/*** Message Queue ***/

#include "wasm_export.h"
#define BUFFER_SIZE 10

typedef struct {
  char *data[BUFFER_SIZE];
  int read;
  int write;
} FifoQueue_t;

static FifoQueue_t queue;

void fifo_init();

int isFull();
int isEmpty();

int put(char *msg);
char *get();

void fifo_init_wrapper(wasm_exec_env_t exec_env);
int put_wrapper(wasm_exec_env_t exec_env, char *msg);
