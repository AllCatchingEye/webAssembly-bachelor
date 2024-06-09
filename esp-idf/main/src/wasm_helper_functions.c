
#include "wasm_helper_functions.h"
#include "esp_log.h"
#include "lib_export.h"
#include <stdio.h>

wasm_file_t initilize_wasm_file(uint8_t *data, size_t size) {
  wasm_file_t wasm_file;
  wasm_file.wasm_file_buf = data;
  wasm_file.wasm_file_buf_size = size;

  return wasm_file;
}

wasm_t initilize_wasm(int stack_size, int heap_size) {
  wasm_t wasm;
  wasm.wasm_module = NULL;
  wasm.wasm_module_inst = NULL;
  wasm.stack_size = stack_size;
  wasm.heap_size = heap_size;

  return wasm;
}

RuntimeInitArgs wasm_init_args() {
  // void *ret;
  RuntimeInitArgs init_args;

  /* configure memory allocation */
  memset(&init_args, 0, sizeof(RuntimeInitArgs));
#if WASM_ENABLE_GLOBAL_HEAP_POOL == 0
  init_args.mem_alloc_type = Alloc_With_Allocator;
  init_args.mem_alloc_option.allocator.malloc_func = (void *)os_malloc;
  init_args.mem_alloc_option.allocator.realloc_func = (void *)os_realloc;
  init_args.mem_alloc_option.allocator.free_func = (void *)os_free;
#else
#error The usage of a global heap pool is not implemented yet for esp-idf.
#endif

  return init_args;
}

void wasm_start(wasm_t *wasm, wasm_file_t *wasm_file,
                NativeSymbol native_symbols) {
  /* setup variables for instantiating and running the wasm module */

  char error_buf[128];

  RuntimeInitArgs init_args = wasm_init_args();

  ESP_LOGI(LOG_TAG, "Initialize WASM runtime");
  /* initialize runtime environment */
  if (!wasm_runtime_full_init(&init_args)) {
    ESP_LOGE(LOG_TAG, "Init runtime failed.");
    return;
  }

  // int n_native_symbols = sizeof(native_symbols) / sizeof(NativeSymbol);
  // ESP_LOGI(LOG_TAG, "Number of native symbols: %d\n", n_native_symbols);
  // if (!wasm_runtime_register_natives("env", native_symbols,
  // n_native_symbols)) {
  //
  //   ESP_LOGE(LOG_TAG, "Registering native functions failed");
  //   return;
  // }

  /* load WASM module */
  if (!(wasm->wasm_module = wasm_runtime_load(wasm_file->wasm_file_buf,
                                              wasm_file->wasm_file_buf_size,
                                              error_buf, sizeof(error_buf)))) {
    ESP_LOGE(LOG_TAG, "Error in wasm_runtime_load: %s", error_buf);
  }

  ESP_LOGI(LOG_TAG, "Instantiate WASM runtime");
  if (!(wasm->wasm_module_inst = wasm_runtime_instantiate(
            wasm->wasm_module, wasm->stack_size, // stack size
            wasm->heap_size,                     // heap size
            error_buf, sizeof(error_buf)))) {
    ESP_LOGE(LOG_TAG, "Error while instantiating: %s", error_buf);
  }
}

static void *app_instance_main(wasm_module_inst_t module_inst) {
  const char *exception;

  ESP_LOGI(LOG_TAG, "Loading main...");
  wasm_application_execute_main(module_inst, 0, NULL);
  if ((exception = wasm_runtime_get_exception(module_inst)))
    printf("%s\n", exception);
  return NULL;
}

int wasm_run_func(wasm_t *wasm, char *func_name, int argc, uint32 args[]) {
  ESP_LOGI(LOG_TAG, "Lookup func %s", func_name);
  /* lookup a WASM function by its name
     The function signature can NULL here */
  wasm_function_inst_t func =
      wasm_runtime_lookup_function(wasm->wasm_module_inst, func_name);

  /* creat an execution environment to execute the WASM functions */
  ESP_LOGI(LOG_TAG, "Create exec env for func %s", func_name);
  wasm_exec_env_t exec_env =
      wasm_runtime_create_exec_env(wasm->wasm_module_inst, wasm->stack_size);

  /* call the WASM function */
  ESP_LOGI(LOG_TAG, "Calling wasm function %s", func_name);
  if (!wasm_runtime_call_wasm(exec_env, func, argc, args)) {
    /* exception is thrown if call fails */
    printf("Error: %s\n", wasm_runtime_get_exception(wasm->wasm_module_inst));
  }

  wasm_runtime_destroy_exec_env(exec_env);

  return args[0];
}

void wasm_end(wasm_t *wasm) {
  /* destroy the module instance */
  ESP_LOGI(LOG_TAG, "Deinstantiate WASM runtime");
  wasm_runtime_deinstantiate(wasm->wasm_module_inst);

  /* unload the module */
  ESP_LOGI(LOG_TAG, "Unload WASM module");
  wasm_runtime_unload(wasm->wasm_module);

  /* destroy runtime environment */
  ESP_LOGI(LOG_TAG, "Destroy WASM runtime");
  wasm_runtime_destroy();
}
