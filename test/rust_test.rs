#![crate_type = "cdylib"]

#[no_mangle]
extern "C" fn fib(n: i32) -> i64 {
    if n < 2 {
        n as i64
    } else {
        fib(n - 1) + fib(n - 2)
    }
}