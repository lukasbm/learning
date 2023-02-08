#include "fibonacci.h"
#include <stdio.h>

int main() {
  for (int32_t i = 1; i < 10; i++) {
    uint64_t f = fibonacci(i);
    printf("fib %i: %li\n", i, f);
  }
  return 0;
}
