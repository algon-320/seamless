use seamless::{anyhow, serde_cbor, serde_json};

use anyhow::{anyhow, bail, Result};
use seamless::remote::RemoteFunction;
use seamless::{Signature, Value, CALLABLE_FUNCTIONS};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn call(
    RemoteFunction {
        caller_lang_name: _,
        ref callee_lang_name,
        ref file,
        ref func_name,
        ref args,
    }: RemoteFunction,
) -> Result<Value> {
    let sig = || -> Result<Signature> {
        let json = CALLABLE_FUNCTIONS.as_ref().map_err(|e| anyhow!(e))?;
        let sig_json = json[callee_lang_name][file][func_name].clone();
        if sig_json.is_null() {
            bail!("function \"{}\" not found", &func_name)
        }
        let sig: Signature = serde_json::from_value(sig_json)?;
        Ok(sig)
    }()
    .map_err(|e| anyhow!("callable_functions.json: {}", e))?;

    let callee_lang = seamless::language::from_name(callee_lang_name)?;
    callee_lang.call(file, func_name, args, sig.return_type)
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("Connection established: {}", stream.peer_addr().unwrap());
    loop {
        let bytes = seamless::remote::read_bytes(&mut stream)?;
        let remote_function: RemoteFunction = serde_cbor::from_slice(&bytes).expect("invalid CBOR");
        println!("func: {:#?}", remote_function);

        let val = call(remote_function).map_err(|e| e.to_string());
        let bytes = serde_cbor::to_vec(&val)?;
        seamless::remote::send_bytes(&mut stream, &bytes)?;
        println!("call done");
    }
}

fn main() {
    const PORT: &str = "3333";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).unwrap();
    println!("Server listening on port {}", PORT);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
