#include <stdio.h>
#include <stdint.h>
#include <stddef.h>

int foreign_call(
    const char *host,
    const char *caller_language,
    const char *language,
    const char *file, const char *funcname,
    char *return_buffer,
    size_t return_buffer_size,
    void **argv,
    size_t argc);

void call_my_func()
{
    printf("---------------- call_my_func ----------------\n");
    int32_t x = 123;
    void *argv[] = {(void *)&x};
    if (foreign_call("localhost", "C", "C", "libc_test.so", "my_func", NULL, 0, argv, 1) != 0)
    {
        printf("something wrong was happend\n");
    }
}
void call_my_func2()
{
    printf("---------------- call_my_func2 --------------\n");
    char return_buff[8] = {0};
    int32_t x = 123;
    int64_t y = 10000000000000000;
    void *argv[] = {(void *)&x, (void *)&y};
    if (foreign_call("localhost", "C", "C", "libc_test.so", "my_func2", return_buff, 8, argv, 2) != 0)
    {
        printf("something wrong was happend\n");
    }
    printf("received return = %ld\n", *(int64_t *)return_buff);
}
void call_fib()
{
    printf("---------------- call_fib --------------\n");
    char return_buff[8] = {0};
    int32_t x = 10;
    void *argv[] = {(void *)&x};
    if (foreign_call("localhost", "C", "Rust", "librust_test.so", "fib", return_buff, 8, argv, 1) != 0)
    {
        printf("something wrong was happend\n");
    }
    printf("received return = %ld\n", *(int64_t *)return_buff);
}

void call_remote_fib()
{
    printf("---------------- call_remote_fib --------------\n");
    char return_buff[8] = {0};
    int32_t x = 20;
    void *argv[] = {(void *)&x};
    if (foreign_call("localhost:3333", "C", "Rust", "librust_test.so", "fib", return_buff, 8, argv, 1) != 0)
    {
        printf("something wrong was happend\n");
    }
    printf("received return = %ld\n", *(int64_t *)return_buff);
}

int main(void)
{
    call_my_func();
    call_my_func2();
    call_fib();
    call_remote_fib();
}