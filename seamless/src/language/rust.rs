use crate::language::Language;
use crate::{Type, Value};
use anyhow::Result;

use super::c::C;
use super::size_of;
use libc::c_void;

#[derive(Debug)]
pub struct Rust;

// the same as C
impl Language for Rust {
    fn call(&self, file: &str, func_name: &str, args: &[Value], ret_ty: Type) -> Result<Value> {
        C.call(file, func_name, args, ret_ty)
    }
    fn size_of(&self, ty: Type) -> usize {
        size_of(ty)
    }
    fn serialize(&self, value: &Value, bytes: *mut c_void) -> Result<()> {
        C.serialize(value, bytes)
    }
    fn deserialize(&self, ty: Type, bytes: *const c_void) -> Result<Value> {
        C.deserialize(ty, bytes)
    }
}
