use crate::language::Language;
use crate::{type_of, Type, Value};
use anyhow::Result;

#[derive(Debug)]
pub struct Rust;

use super::{ffi_type_map, ffi_type_of, size_of};
use libc::c_void;
use libffi::low::*;

impl Language for Rust {
    fn call(&self, file: &str, func_name: &str, args: &[Value], ret_ty: Type) -> Result<Value> {
        let args_data: Vec<_> = args
            .iter()
            .map(|arg| {
                let mut buf = vec![0u8; Rust.size_of(type_of(arg))];
                Rust.serialize(arg, buf.as_mut_ptr() as *mut _)?;
                Ok(buf)
            })
            .collect::<Result<_>>()?;
        let arg_ptr: Vec<_> = args_data
            .iter()
            .map(|arg| arg.as_ptr() as *const c_void)
            .collect();

        let mut ret_buf = vec![0u8; size_of(ret_ty)];
        unsafe {
            let lib = libloading::Library::new(file)?;
            let func: libloading::Symbol<unsafe extern "C" fn() -> c_void> =
                lib.get(func_name.as_bytes())?;

            let mut args: Vec<_> = args.iter().map(ffi_type_of).collect();
            let mut cif: ffi_cif = Default::default();

            prep_cif(
                &mut cif,
                ffi_abi_FFI_DEFAULT_ABI,
                args.len(),
                ffi_type_map(&ret_ty),
                args.as_mut_ptr(),
            )
            .unwrap();

            libffi::raw::ffi_call(
                &mut cif as *mut ffi_cif,
                Some(*CodePtr(*func as *mut _).as_safe_fun()),
                ret_buf.as_mut_ptr() as *mut c_void,
                arg_ptr.as_ptr() as *mut *mut c_void,
            );
        };
        Rust.deserialize(ret_ty, ret_buf.as_ptr() as *const _)
    }

    fn size_of(&self, ty: Type) -> usize {
        size_of(ty)
    }

    fn serialize(&self, value: &Value, bytes: *mut c_void) -> Result<()> {
        macro_rules! ser_primitive_integer {
            ($v:expr, $T:ty) => {{
                let b = <$T>::to_ne_bytes(*$v);
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        b.as_ptr(),
                        bytes as *mut u8,
                        Rust.size_of(type_of(value)),
                    )
                };
                Ok(())
            }};
        }
        match value {
            Value::Int32(v) => ser_primitive_integer!(v, i32),
            Value::Int64(v) => ser_primitive_integer!(v, i64),
            Value::Uint32(v) => ser_primitive_integer!(v, u32),
            Value::Uint64(v) => ser_primitive_integer!(v, u64),
            Value::Void => Ok(()),
        }
    }

    fn deserialize(&self, ty: Type, bytes: *const c_void) -> Result<Value> {
        use std::convert::TryInto;
        macro_rules! deser_primitive_integer {
            ($T:ty, $p:path) => {{
                let bytes = unsafe { std::slice::from_raw_parts(bytes as *const u8, size_of(ty)) };
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
