#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include "seamless_bridge.h"

void my_func(int x)
{
  printf("my_func: x = %d\n", x);
}

int64_t my_func2(int x, int64_t y)
{
  printf("my_func2: x = %d, y = %ld\n", x, y);
  int64_t ret = (int64_t)x + y;
  printf("my_func2: ret = %ld\n", ret);
  return ret;
}

// for seamless
def_bridge_void_1(my_func, int);
def_bridge_2(my_func2, int64_t, int, int64_t);