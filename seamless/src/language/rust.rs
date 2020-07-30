use crate::language::Language;
use crate::{Type, Value};
use anyhow::Result;

#[derive(Debug)]
pub struct Rust;

use libc::{c_char, c_void, size_t};

fn size_of(ty: Type) -> usize {
    match ty {
        Type::Int32 => std::mem::size_of::<i32>(),
        Type::Int64 => std::mem::size_of::<i64>(),
        Type::Uint32 => std::mem::size_of::<u32>(),
        Type::Uint64 => std::mem::size_of::<u64>(),
        Type::Void => 0,
    }
}

impl Language for Rust {
    fn call(&self, file: &str, func_name: &str, args: &[Value], ret_ty: Type) -> Result<Value> {
        let lib = libloading::Library::new(file)?;
        let bridge_func_name = format!("{}_bridge", func_name);
        let args: Vec<_> = args
            .iter()
            .map(|arg| Rust.serialize(arg))
            .collect::<Result<_>>()?;

        let mut return_buf: Vec<_> = vec![0; size_of(ret_ty)];
        type return_ptr_t = *mut c_void;
        match args.len() {
            0 => unsafe {
                let func: libloading::Symbol<unsafe extern "C" fn(return_ptr_t) -> c_void> =
                    lib.get(bridge_func_name.as_bytes())?;
                func(return_buf.as_mut_ptr() as return_ptr_t)
            },
            1 => unsafe {
                let func: libloading::Symbol<
                    unsafe extern "C" fn(return_ptr_t, *const c_void) -> c_void,
                > = lib.get(bridge_func_name.as_bytes())?;
                func(
                    return_buf.as_mut_ptr() as return_ptr_t,
                    args[0].as_ptr() as *const c_void,
                )
            },
            2 => unsafe {
                let func: libloading::Symbol<
                    unsafe extern "C" fn(return_ptr_t, *const c_void, *const c_void) -> c_void,
                > = lib.get(bridge_func_name.as_bytes())?;
                func(
                    return_buf.as_mut_ptr() as return_ptr_t,
                    args[0].as_ptr() as *const c_void,
                    args[1].as_ptr() as *const c_void,
                )
            },
            _ => panic!("too many arguments"),
        };
        let ret_val = Rust.deserialize(ret_ty, return_buf.as_ptr())?;
        Ok(ret_val)
    }

    fn serialize(&self, value: &Value) -> Result<Vec<u8>> {
        macro_rules! ser_primitive_integer {
            ($v:expr, $T:ty) => {{
                Ok(<$T>::to_ne_bytes(*$v).to_vec())
            }};
        }
        match value {
            Value::Int32(v) => ser_primitive_integer!(v, i32),
            Value::Int64(v) => ser_primitive_integer!(v, i64),
            Value::Uint32(v) => ser_primitive_integer!(v, u32),
            Value::Uint64(v) => ser_primitive_integer!(v, u64),
            Value::Void => Ok(Vec::new()),
        }
    }

    fn deserialize(&self, ty: Type, bytes: *const u8) -> Result<Value> {
        use std::convert::TryInto;
        macro_rules! deser_primitive_integer {
            ($T:ty, $p:path) => {{
                let bytes = unsafe { std::slice::from_raw_parts(bytes, size_of(ty)) };
                Ok($p(<$T>::from_ne_bytes(bytes.try_into().unwrap())))
            }};
        }
        match ty {
            Type::Int32 => deser_primitive_integer!(i32, Value::Int32),
            Type::Int64 => deser_primitive_integer!(i64, Value::Int64),
            Type::Uint32 => deser_primitive_integer!(u32, Value::Uint32),
            Type::Uint64 => deser_primitive_integer!(u64, Value::Uint64),
            Type::Void => Ok(Value::Void),
        }
    }
}
