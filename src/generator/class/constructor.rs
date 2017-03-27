use analysis::MethodEnvironment;
use generator::body;

use super::method;

pub fn go(method: &MethodEnvironment,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match method::get_args(&method.parameters, label, &mut text, &mut bss) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // TODO<codegen>: call parent constructor!

    // generate body
    if let Some(b) = method.body.clone() {
        match body::go(&b, &label, &mut text, &mut externs, &mut bss, &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    // TODO<codegen>: else error?

    text.push("".to_owned());
    Ok(())
}
