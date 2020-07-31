use seamless::libc;
use seamless::{local, remote};
use std::ffi::CStr;

#[no_mangle]
extern "C" fn foreign_call(
    host: *const libc::c_char,
    caller_lang_name: *const libc::c_char,
    callee_lang_name: *const libc::c_char,
    file: *const libc::c_char,
    func_name: *const libc::c_char,
    return_buffer: *mut libc::c_void,
    return_buffer_size: libc::size_t,
    argv: *const *const libc::c_void,
    argc: libc::size_t,
) -> libc::c_int {
    let result = || {
        let is_local = host.is_null();
        let caller_lang_name = unsafe { CStr::from_ptr(caller_lang_name).to_str()? };
        let lang_name = unsafe { CStr::from_ptr(callee_lang_name).to_str()? };
        let file = unsafe { CStr::from_ptr(file).to_str()? };
        let func_name = unsafe { CStr::from_ptr(func_name).to_str()? };
        let argv = unsafe { std::slice::from_raw_parts(argv, argc) };
        let ret =
            unsafe { std::slice::from_raw_parts_mut(return_buffer as *mut u8, return_buffer_size) };

        if is_local {
            local::call(caller_lang_name, lang_name, file, func_name, argv, ret)
        } else {
            let host = unsafe { CStr::from_ptr(host).to_str()? };
            remote::call(
                &host.parse()?,
                caller_lang_name,
                lang_name,
                file,
                func_name,
                argv,
                ret,
            )
        }
    };
    match result() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("foreign call error: {}", e);
            1
        }
    }
}
