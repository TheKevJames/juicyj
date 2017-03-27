use analysis::MethodEnvironment;
use generator::asm::helper::call;
use generator::body;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use super::method;

lazy_static! {
    static ref EMPTYPARAMS: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ParameterList")),
            children: Vec::new(),
        }
    };
}

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

    // call parent constructor
    if let Some(p) = method.parent.clone() {
        match call(&p,
                   &EMPTYPARAMS.clone(),
                   label,
                   &mut text,
                   &mut externs,
                   &mut bss,
                   &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    // generate body
    if let Some(b) = method.body.clone() {
        match body::go(&b, &label, &mut text, &mut externs, &mut bss, &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    // TODO<codegen>: else error?

    // TODO<codegen>: return instance

    text.push("".to_owned());
    Ok(())
}
