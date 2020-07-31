pub extern crate anyhow;
pub extern crate libc;
pub extern crate serde;
pub extern crate serde_cbor;
pub extern crate serde_derive;
pub extern crate serde_json;

use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

pub mod language;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    // BigInt(rug::Integer),
    Void,
}
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Type {
    Int32,
    Int64,
    Uint32,
    Uint64,
    // BigInt,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFunction {
    pub caller_lang_name: String,
    pub callee_lang_name: String,
    pub file: String,
    pub func_name: String,
    pub args: Vec<Value>,
}

pub fn remote_call(
    host: &str,
    caller_lang_name: &str,
    callee_lang_name: &str,
    file: &str,
    func_name: &str,
    args: &[*const libc::c_void],
) -> Result<Box<[u8]>> {
    let caller_lang = language::from_name(caller_lang_name)?;

    let sig = || -> Result<Signature> {
        let json = CALLABLE_FUNCTIONS.as_ref().map_err(|e| anyhow!(e))?;
        let sig_json = json[callee_lang_name][file][func_name].clone();
        if sig_json.is_null() {
            bail!("function \"{}\" not found", func_name)
        }
        let sig: Signature = serde_json::from_value(sig_json)?;
        Ok(sig)
    }()
    .map_err(|e| anyhow!("callable_functions.json: {}", e))?;
    if args.len() < sig.argument_type.len() {
        bail!("too few arguments");
    }
    let args: Vec<_> = sig
        .argument_type
        .into_iter()
        .zip(args.iter())
        .map(|(ty, argv)| caller_lang.deserialize(ty, (*argv) as *const u8))
        .collect::<Result<_>>()?;

    let function = RemoteFunction {
        caller_lang_name: caller_lang_name.to_owned(),
        callee_lang_name: callee_lang_name.to_owned(),
        file: file.to_owned(),
        func_name: func_name.to_owned(),
        args,
    };
    let bytes = serde_cbor::to_vec(&function)?;

    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpStream};

    let val = match TcpStream::connect(host) {
        Ok(mut stream) => {
            println!("Successfully connected to server: {}", host);

            // 1. send length (big endian)
            stream.write_all(&usize::to_be_bytes(bytes.len())).unwrap();
            // 2. send contents
            let mut sent = 0;
            while sent < bytes.len() {
                let add = stream.write(&bytes[sent..]).unwrap();
                sent += add;
            }
            println!("Sent function info, awaiting reply...");

            let mut length = [0u8; 8];
            match stream.read_exact(&mut length) {
                Ok(_) => {}
                Err(_) => bail!("remote call failed"),
            }
            let length = usize::from_be_bytes(length);
            let mut read = 0;
            let mut bytes = Vec::new();
            let mut buffer = vec![0; 1024];
            while read < length {
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        break;
                    }
                    Ok(size) => {
                        bytes.extend_from_slice(&buffer[..size]);
                        read += size;
                    }
                    Err(_) => {
                        println!(
                            "An error occurred, terminating connection with {}",
                            stream.peer_addr().unwrap()
                        );
                        stream.shutdown(Shutdown::Both).unwrap();
                        break;
                    }
                }
            }
            stream.shutdown(Shutdown::Both).unwrap();
            let res: Result<Value, String> = serde_cbor::from_slice(&bytes)?;
            res
        }
        Err(e) => bail!("Failed to connect: {}", e),
    }
    .map_err(|e| anyhow!(e))?;

    {
        let mut buf = vec![0; caller_lang.size_of(type_of(&val))];
        caller_lang.serialize(&val, buf.as_mut_ptr())?;
        Ok(buf.into_boxed_slice())
    }
}

pub fn local_call(
    caller_lang_name: &str,
    callee_lang_name: &str,
    file: &str,
    func_name: &str,
    args: &[*const libc::c_void],
) -> Result<Box<[u8]>> {
    let caller_lang = language::from_name(caller_lang_name)?;
    let callee_lang = language::from_name(callee_lang_name)?;

    let sig = || -> Result<Signature> {
        let json = CALLABLE_FUNCTIONS.as_ref().map_err(|e| anyhow!(e))?;
        let sig_json = json[callee_lang_name][file][func_name].clone();
        if sig_json.is_null() {
            bail!("function \"{}\" not found", func_name)
        }
        let sig: Signature = serde_json::from_value(sig_json)?;
        Ok(sig)
    }()
    .map_err(|e| anyhow!("callable_functions.json: {}", e))?;

    if args.len() < sig.argument_type.len() {
        bail!("too few arguments");
    }
    let args: Vec<_> = sig
        .argument_type
        .into_iter()
        .zip(args.iter())
        .map(|(ty, argv)| caller_lang.deserialize(ty, (*argv) as *const u8))
        .collect::<Result<_>>()?;
    let ret = callee_lang.call(file, func_name, &args, sig.return_type)?;

    {
        let mut buf = vec![0; caller_lang.size_of(type_of(&ret))];
        caller_lang.serialize(&ret, buf.as_mut_ptr())?;
        Ok(buf.into_boxed_slice())
    }
}

pub fn is_localhost(host: &str) -> bool {
    if host == "localhost" {
        true
    } else {
        false
    }
}
