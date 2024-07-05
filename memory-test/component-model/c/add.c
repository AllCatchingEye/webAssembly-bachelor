#include "example.h"
// #include "stdio.h"

int32_t exports_example_component_adder_add(int32_t x, int32_t y) {
  int result = x + y;
  // printf("Result of %d + %d: %d", x, y, result);
  return result;
}
