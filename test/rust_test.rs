#![crate_type = "cdylib"]

fn fib(n: i32) -> i64 {
    if n < 2 {
        n as i64
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// for seamless
use std::ffi::c_void;
#[no_mangle]
unsafe extern "C" fn fib_bridge(ret_buf: *mut c_void, n: *const c_void) {
    *(ret_buf as *mut i64) = fib(*(n as *const i32));
}
