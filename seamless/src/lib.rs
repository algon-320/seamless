pub extern crate anyhow;
pub extern crate libc;
pub extern crate serde;
pub extern crate serde_cbor;
pub extern crate serde_derive;
pub extern crate serde_json;

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

pub mod language;
pub mod local;
pub mod remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Void,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Type {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Void,
}

pub fn type_of(val: &Value) -> Type {
    match val {
        Value::Int32(_) => Type::Int32,
        Value::Int64(_) => Type::Int64,
        Value::Uint32(_) => Type::Uint32,
        Value::Uint64(_) => Type::Uint64,
        Value::Void => Type::Void,
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Signature {
    pub argument_type: Vec<Type>,
    pub return_type: Type,
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref CALLABLE_FUNCTIONS: Result<serde_json::Value> = {
        // TODO: make JSON path configurable
        let json = std::fs::read_to_string("./callable_functions.json")?;
        Ok(serde_json::from_str(&json)?)
    };
}
