use super::{Signature, Value, CALLABLE_FUNCTIONS};
use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn send_bytes(stream: &mut TcpStream, bytes: &[u8]) -> Result<()> {
    // 1. send length (64bit, big endian)
    let length = u64::to_be_bytes(bytes.len() as u64);
    stream.write_all(&length)?;
    // 2. send contents
    stream.write_all(bytes)?;
    Ok(())
}

pub fn read_bytes(stream: &mut TcpStream) -> Result<Vec<u8>> {
    // 1. receive the length of bytes (64bit, big endian)
    let mut length = [0u8; std::mem::size_of::<u64>()];
    match stream.read_exact(&mut length) {
        Ok(_) => {}
        Err(_) => bail!("remote call failed"),
    }
    let length = u64::from_be_bytes(length) as usize;

    // 2. receive the whole contents
    let mut read = 0;
    let mut bytes = Vec::new();
    let mut buffer = [0; 1024];
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
                eprintln!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
    Ok(bytes)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteFunction {
    pub caller_lang_name: String,
    pub callee_lang_name: String,
    pub file: String,
    pub func_name: String,
    pub args: Vec<Value>,
}

pub fn call(
    host: &std::net::SocketAddr,
    caller_lang_name: &str,
    callee_lang_name: &str,
    file: &str,
    func_name: &str,
    args: &[*const libc::c_void],
    return_buf: &mut [u8],
) -> Result<()> {
    let caller_lang = crate::language::from_name(caller_lang_name)?;

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
    if return_buf.len() < caller_lang.size_of(sig.return_type) {
        bail!("return buffer too small");
    }

    let args: Vec<_> = sig
        .argument_type
        .into_iter()
        .zip(args.iter())
        .map(|(ty, argv)| caller_lang.deserialize(ty, (*argv) as *const _))
        .collect::<Result<_>>()?;

    let function = RemoteFunction {
        caller_lang_name: caller_lang_name.to_owned(),
        callee_lang_name: callee_lang_name.to_owned(),
        file: file.to_owned(),
        func_name: func_name.to_owned(),
        args,
    };
    let bytes = serde_cbor::to_vec(&function)?;

    let val = match TcpStream::connect(host) {
        Ok(mut stream) => {
            send_bytes(&mut stream, &bytes)?;
            let bytes = read_bytes(&mut stream)?;
            stream.shutdown(Shutdown::Both).unwrap();
            serde_cbor::from_slice::<Result<Value, String>>(&bytes)?
        }
        Err(e) => bail!("Failed to connect: {}", e),
    }
    .map_err(|e| anyhow!(e))?;

    caller_lang.serialize(&val, return_buf.as_mut_ptr() as *mut _)?;
    Ok(())
}
