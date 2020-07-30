use anyhow::anyhow;
use seamless::{anyhow, libc};
use seamless::{is_localhost, local_call, remote_call};

#[no_mangle]
extern "C" fn foreign_call(
    host: *const libc::c_char,
    caller_lang_name: *const libc::c_char,
    callee_lang_name: *const libc::c_char,
    file: *const libc::c_char,
    func_name: *const libc::c_char,
    return_buffer: *mut libc::c_char,
    return_buffer_size: libc::size_t,
    argv: *const *const libc::c_void,
    argc: libc::size_t,
) -> libc::c_int {
    let result = || {
        use std::ffi::CStr;
        let host = unsafe { CStr::from_ptr(host).to_str()? };
        let caller_lang_name = unsafe { CStr::from_ptr(caller_lang_name).to_str()? };
        let lang_name = unsafe { CStr::from_ptr(callee_lang_name).to_str()? };
        let file = unsafe { CStr::from_ptr(file).to_str()? };
        let func_name = unsafe { CStr::from_ptr(func_name).to_str()? };
        let argv = unsafe { std::slice::from_raw_parts(argv, argc) };

        if is_localhost(host) {
            local_call(caller_lang_name, lang_name, file, func_name, argv)
        } else {
            remote_call(host, caller_lang_name, lang_name, file, func_name, argv)
        }
    };
    match result() {
        Ok(val) => {
            if val.len() > return_buffer_size {
                println!("foreign call error: {}", anyhow!("return buffer too small"));
                return 2;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(val.as_ptr(), return_buffer as *mut u8, val.len());
            }
            0
        }
        Err(e) => {
            println!("foreign call error: {}", e);
            1
        }
    }
}
