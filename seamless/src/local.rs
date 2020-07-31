use super::{Signature, CALLABLE_FUNCTIONS};
use anyhow::{anyhow, bail, Result};

pub fn call(
    caller_lang_name: &str,
    callee_lang_name: &str,
    file: &str,
    func_name: &str,
    args: &[*const libc::c_void],
    return_buf: &mut [u8],
) -> Result<()> {
    let caller_lang = crate::language::from_name(caller_lang_name)?;
    let callee_lang = crate::language::from_name(callee_lang_name)?;

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
    let ret = callee_lang.call(file, func_name, &args, sig.return_type)?;

    caller_lang.serialize(&ret, return_buf.as_mut_ptr() as *mut _)?;
    Ok(())
}
