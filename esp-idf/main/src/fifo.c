#include "fifo.h"
#include "pthread.h"
#include "stdio.h"

void fifo_init() {
  queue.read = 0;
  queue.write = 0;
}

int isFull() {
  printf("Checking if full: read = %d, write = %d\n", queue.read, queue.write);
  return ((queue.write + 1) % BUFFER_SIZE == queue.read);
}

int isEmpty() {
  printf("Checking if empty: read = %d, write = %d\n", queue.read, queue.write);
  return (queue.read == queue.write);
}

int put(char *msg) {
  printf("Attempting to put message: %s\n", msg);
  if (isFull()) {
    printf("Error: Queue is full.\n");
    return -1;
  }
  queue.data[queue.write] = msg;
  queue.write = (queue.write + 1) % BUFFER_SIZE;
  printf("Message put successfully. New write index: %d\n", queue.write);
  return 0;
}

char *get() {
  printf("Attempting to get message\n");
  if (isEmpty()) {
    printf("Error: Queue is empty.\n");
    return NULL;
  }
  char *msg = queue.data[queue.read];
  queue.read = (queue.read + 1) % BUFFER_SIZE;
  printf("Message retrieved: %s. New read index: %d\n", msg, queue.read);
  return msg;
}

void fifo_init_wrapper(wasm_exec_env_t exec_env) { fifo_init(); }
int put_wrapper(wasm_exec_env_t exec_env, char *msg) { return put(msg); }
