#![crate_type = "cdylib"]

#[no_mangle]
extern "C" fn fib(n: i32) -> i64 {
    if n < 2 {
        n as i64
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[no_mangle]
extern "C" fn many_args(a: i32, b: u32, c: i64, d: u64) {
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
}