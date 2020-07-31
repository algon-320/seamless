use crate::{Type, Value};
use anyhow::{anyhow, Result};

mod c;
mod ruby;
mod rust;

pub trait Language {
    fn call(&self, file: &str, func_name: &str, args: &[Value], ret_ty: Type) -> Result<Value>;
    fn size_of(&self, ty: Type) -> usize;
    fn serialize(&self, value: &Value, bytes: *mut u8) -> Result<()>;
    fn deserialize(&self, ty: Type, bytes: *const u8) -> Result<Value>;
}

pub fn from_name(name: &str) -> Result<&'static dyn Language> {
    match name {
        "C" | "c" => Ok(&c::C),
        "Rust" | "rust" => Ok(&rust::Rust),
        "Ruby" | "ruby" => Ok(&ruby::Ruby),
        _ => Err(anyhow!("unsupported language")),
    }
}

fn size_of(ty: Type) -> usize {
    match ty {
        Type::Int32 => std::mem::size_of::<i32>(),
        Type::Int64 => std::mem::size_of::<i64>(),
        Type::Uint32 => std::mem::size_of::<u32>(),
        Type::Uint64 => std::mem::size_of::<u64>(),
        Type::Void => 0,
    }
}

use libffi::low::*;

fn ffi_type_of(v: &Value) -> *mut ffi_type {
    unsafe {
        (match v {
            Value::Int32(_) => &mut types::sint32,
            Value::Int64(_) => &mut types::sint64,
            Value::Uint32(_) => &mut types::uint32,
            Value::Uint64(_) => &mut types::uint64,
            Value::Void => &mut types::void,
        }) as *mut ffi_type
    }
}
fn ffi_type_map(v: &Type) -> *mut ffi_type {
    unsafe {
        (match v {
            Type::Int32 => &mut types::sint32,
            Type::Int64 => &mut types::sint64,
            Type::Uint32 => &mut types::uint32,
            Type::Uint64 => &mut types::uint64,
            Type::Void => &mut types::void,
        }) as *mut ffi_type
    }
}
