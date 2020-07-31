#include <stdio.h>
#include <stdint.h>
#include <stddef.h>

int foreign_call(
    const char *host, // NULL means local call
    const char *caller_language,
    const char *callee_language,
    const char *file, const char *funcname,
    char *return_buffer,
    size_t return_buffer_size,
    void **argv,
    size_t argc);

void call_my_func()
{
    printf("---------------- call_my_func ----------------\n");
    int32_t x = 123;
    void *argv[] = {&x};
    if (foreign_call(NULL, "C", "C", "libc_test.so", "my_func", NULL, 0, argv, 1) != 0)
    {
        printf("something wrong was happened\n");
    }
}
void call_my_func2()
{
    printf("---------------- call_my_func2 --------------\n");
    int64_t ret;
    int32_t x = 123;
    int64_t y = 10000000000000000;
    void *argv[] = {&x, &y};
    if (foreign_call(NULL, "C", "C", "libc_test.so", "my_func2", (char *)&ret, sizeof ret, argv, 2) != 0)
    {
        printf("something wrong was happened\n");
    }
    printf("received return = %ld\n", ret);
}
void call_fib()
{
    printf("---------------- call_fib --------------\n");
    int64_t ret;
    int32_t x = 10;
    void *argv[] = {&x};
    if (foreign_call(NULL, "C", "Rust", "librust_test.so", "fib", (char *)&ret, sizeof ret, argv, 1) != 0)
    {
        printf("something wrong was happened\n");
    }
    printf("received return = %ld\n", ret);
}
void call_many_args()
{
    printf("---------------- call_many_args --------------\n");
    int32_t a = 1;
    uint32_t b = 2;
    int64_t c = 3;
    uint64_t d = 4;
    void *argv[] = {&a, &b, &c, &d};
    if (foreign_call(NULL, "C", "Rust", "librust_test.so", "many_args", NULL, 0, argv, 4) != 0)
    {
        printf("something wrong was happened\n");
    }
}
void call_foo()
{
    printf("---------------- call_foo --------------\n");
    int32_t a = 123;
    int64_t b = 1000000000000;
    void *argv[] = {&a, &b};
    if (foreign_call(NULL, "C", "Ruby", "ruby_test.rb", "foo", NULL, 0, argv, 2) != 0)
    {
        printf("something wrong was happened\n");
    }
}

void call_remote_fib()
{
    printf("---------------- call_remote_fib --------------\n");
    int64_t ret;
    int32_t x = 20;
    void *argv[] = {&x};
    if (foreign_call("127.0.0.1:4444", "C", "Rust", "librust_test.so", "fib", (char *)&ret, sizeof ret, argv, 1) != 0)
    {
        printf("something wrong was happened\n");
    }
    printf("received return = %ld\n", ret);
}

int main(void)
{
    // C function
    call_my_func();
    call_my_func2();

    // Rust function
    call_fib();
    call_many_args();

    // Ruby function
    call_foo();

    // Remote function
    call_remote_fib();
}
