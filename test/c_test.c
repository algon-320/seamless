#include <stdio.h>
#include <stddef.h>
#include <stdint.h>

void my_func(int x)
{
  printf("my_func: x = %d\n", x);
}

int64_t my_func2(int x, int64_t y)
{
  printf("my_func2: x = %d, y = %ld\n", x, y);
  return (int64_t)x + y;
}
