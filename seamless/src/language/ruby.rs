use crate::language::Language;
use crate::{type_of, Type, Value};
use anyhow::Result;

use libc::{c_char, c_int, c_long, c_void};
use std::ffi::CString;

type VALUE = *mut c_void;
extern "C" {
    fn seamless_ruby_init();
    fn semaless_ruby_require(filename: *const c_char);
    fn seamless_ruby_finalize();
    fn seamless_ruby_call(func_name: *const c_char, argc: c_int, argv: *const VALUE) -> VALUE;
    fn seamless_ruby_int2fix(x: c_int) -> VALUE;
    fn seamless_ruby_long2fix(x: c_long) -> VALUE;
    fn seamless_ruby_num2int(x: VALUE) -> c_int;
    fn seamless_ruby_num2long(x: VALUE) -> c_long;
}

#[derive(Debug)]
pub struct Ruby;

impl Language for Ruby {
    fn call(&self, file: &str, func_name: &str, args: &[Value], ret_ty: Type) -> Result<Value> {
        let ret;
        unsafe {
            let file = CString::new(format!("./{}", file))?;
            let func_name = CString::new(func_name)?;

            seamless_ruby_init();
            semaless_ruby_require(file.as_ptr());
            {
                let args: Vec<_> = args
                    .iter()
                    .map(|arg| {
                        let mut buf = vec![0; Ruby.size_of(type_of(&arg))];
                        Ruby.serialize(&arg, buf.as_mut_ptr())?;
                        Ok(buf)
                    })
                    .collect::<Result<_>>()?;
                let argv: Vec<_> = args
                    .iter()
                    .map(|arg| *(arg.as_ptr() as *const i64) as VALUE)
                    .collect();

                let tmp: VALUE =
                    seamless_ruby_call(func_name.as_ptr(), argv.len() as c_int, argv.as_ptr());
                ret = Ruby.deserialize(ret_ty, tmp as *const u8)?;
            }
            seamless_ruby_finalize();
        }
        Ok(ret)
    }

    fn size_of(&self, ty: Type) -> usize {
        match ty {
            Type::Int32 | Type::Int64 | Type::Uint32 | Type::Uint64 => std::mem::size_of::<VALUE>(),
            Type::Void => 0,
        }
    }

    fn serialize(&self, value: &Value, bytes: *mut u8) -> Result<()> {
        let bytes = bytes as VALUE;
        match value {
            Value::Int32(v) => {
                let fix = unsafe { seamless_ruby_int2fix(*v) };
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        &fix as *const _ as *const u8,
                        bytes as *mut u8,
                        8,
                    )
                };
                Ok(())
            }
            Value::Int64(v) => {
                let fix = unsafe { seamless_ruby_long2fix(*v) };
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        &fix as *const _ as *const u8,
                        bytes as *mut u8,
                        8,
                    )
                };
                Ok(())
            }
            Value::Void => Ok(()),
            _ => todo!(),
        }
    }

    fn deserialize(&self, ty: Type, bytes: *const u8) -> Result<Value> {
        let bytes = bytes as VALUE;
        match ty {
            Type::Int32 => {
                let v = unsafe { seamless_ruby_num2int(bytes) };
                Ok(Value::Int32(v))
            }
            Type::Int64 => {
                let v = unsafe { seamless_ruby_num2long(bytes) };
                Ok(Value::Int64(v))
            }
            Type::Void => Ok(Value::Void),
            _ => todo!(),
        }
    }
}
