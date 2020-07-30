use crate::{Type, Value};
use anyhow::{anyhow, Result};

mod c;
mod rust;

pub trait Language {
    fn call(&self, file: &str, func_name: &str, args: &[Value], ret_ty: Type) -> Result<Value>;
    fn serialize(&self, value: &Value) -> Result<Vec<u8>>;
    fn deserialize(&self, ty: Type, bytes: *const u8) -> Result<Value>;
}

pub fn from_name(name: &str) -> Result<&'static dyn Language> {
    match name {
        "C" | "c" => Ok(&c::C),
        "Rust" | "rust" => Ok(&rust::Rust),
        _ => Err(anyhow!("unsupported language")),
    }
}
