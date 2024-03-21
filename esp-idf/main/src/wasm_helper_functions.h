#include "bh_platform.h"
#include "wasm_export.h"
#include <stdint.h>
#include <unistd.h>

#define LOG_TAG "wamr"

typedef struct {
  wasm_module_inst_t wasm_module_inst;
  wasm_module_t wasm_module;
  uint32 stack_size;
  uint32 heap_size;
} wasm_t;

typedef struct {
  uint8_t *wasm_file_buf;
  unsigned wasm_file_buf_size;
} wasm_file_t;

wasm_t initilize_wasm();

wasm_file_t initilize_wasm_file(uint8_t *data, size_t size);

RuntimeInitArgs wasm_init_args();

void wasm_start(wasm_t *wasm, wasm_file_t *wasm_file);

static void *app_instance_main(wasm_module_inst_t module_inst);

int wasm_run_func(wasm_t *wasm, char *func_name, int argc, uint32 args[]);

void wasm_end(wasm_t *wasm);
