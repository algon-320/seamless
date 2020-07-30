use seamless::{anyhow, serde, serde_cbor, serde_derive, serde_json};

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use anyhow::{anyhow, bail, Result};

use seamless::{RemoteFunction, Signature, Type, Value, CALLABLE_FUNCTIONS};

fn call(
    RemoteFunction {
        caller_lang_name: _,
        callee_lang_name,
        file,
        func_name,
        args,
    }: RemoteFunction,
) -> Result<Value> {
    let sig = || -> Result<Signature> {
        let json = CALLABLE_FUNCTIONS.as_ref().map_err(|e| anyhow!(e))?;
        let sig_json = json[&callee_lang_name][&file][&func_name].clone();
        if sig_json.is_null() {
            bail!("function \"{}\" not found", &func_name)
        }
        let sig: Signature = serde_json::from_value(sig_json)?;
        Ok(sig)
    }()
    .map_err(|e| anyhow!("callable_functions.json: {}", e))?;

    let callee_lang = seamless::language::from_name(&callee_lang_name)?;
    callee_lang.call(&file, &func_name, &args, sig.return_type)
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("Connection established: {}", stream.peer_addr().unwrap());
    let mut buffer = vec![0u8; 1024];
    'main: loop {
        let mut length = [0u8; 8];
        match stream.read_exact(&mut length) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
        let length = usize::from_be_bytes(length);
        let mut read = 0;
        let mut bytes = Vec::new();
        while read < length {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    // closed
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
                    break 'main;
                }
            }
        }
        let remote_function: RemoteFunction = serde_cbor::from_slice(&bytes).expect("invalid CBOR");
        println!("func: {:#?}", remote_function);

        let val = call(remote_function).map_err(|e| e.to_string());
        println!("ret_val = {:?}", val);
        let bytes = serde_cbor::to_vec(&val)?;

        // 1. send length (big endian)
        stream.write_all(&usize::to_be_bytes(bytes.len())).unwrap();
        // 2. send return value
        let mut sent = 0;
        while sent < bytes.len() {
            let add = stream.write(&bytes[sent..]).unwrap();
            sent += add;
        }
        println!("call done");
    }
    println!("Connection closed: {}", stream.peer_addr().unwrap());
    Ok(())
}

fn main() {
    let port = 3333;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
