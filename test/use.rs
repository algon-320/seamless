use std::ffi::c_void;

extern "C" {
    fn foreign_call(
        host: *const u8,
        caller_lang_name: *const u8,
        callee_lang_name: *const u8,
        file: *const u8,
        func_name: *const u8,
        return_buffer: *mut i8,
        return_buffer_size: usize,
        argv: *const *const c_void,
        argc: usize,
    ) -> i32;
}

fn main() {
    println!("---------------- my_func2 from Rust ----------------");

    let mut ret = 0i64;
    let args = [
        &123i32 as *const _ as *const c_void,
        &10000000000000000i64 as *const _ as *const c_void,
    ];
    if unsafe {
        foreign_call(
            "localhost\0".as_ptr(),
            "Rust\0".as_ptr(),
            "C\0".as_ptr(),
            "libc_test.so\0".as_ptr(),
            "my_func2\0".as_ptr(),
            &mut ret as *mut _ as *mut i8,
            8,
            args.as_ptr() as *const _,
            2,
        )
    } != 0
    {
        panic!("error");
    }
    println!("ret = {}", ret);
}
