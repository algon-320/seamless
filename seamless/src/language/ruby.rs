use crate::language::Language;
use crate::{Type, Value};
use anyhow::Result;

#[derive(Debug)]
pub struct Ruby;

use libc::{c_char, c_void, size_t};

extern "C" {
    fn seamless_language_ruby_init();
    fn seamless_language_ruby_init();
}

impl Language for Ruby {
    fn call(&self, file: &str, func_name: &str, args: &[Value], ret_ty: Type) -> Result<Value> {
        todo!()
    }

    fn serialize(&self, value: &Value) -> Result<Vec<u8>> {
        todo!()
    }

    fn deserialize(&self, ty: Type, bytes: *const u8) -> Result<Value> {
        todo!()
    }
}
