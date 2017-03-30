use std::collections::HashMap;

use analysis::FieldEnvironment;
use analysis::MethodEnvironment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use super::method;

lazy_static! {
    static ref INIT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Identifier, Some("INIT")), children: Vec::new() }
    };
    static ref NUMVALUEZERO: ASTNode = {
        ASTNode { token: Token::new(TokenKind::NumValue, Some("0")), children: Vec::new() }
    };
    static ref RETURN: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Return, None), children: Vec::new() }
    };
    static ref RETURNSATEMENT: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ReturnStatement")),
            children: Vec::new(),
        }
    };
    static ref SEMICOLON: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Semicolon, None), children: Vec::new() }
    };
}

pub fn go(field: &FieldEnvironment,
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<Option<ASTNode>, String> {
    let mut init = field.name.clone();
    init.flatten();
    let init_label = match init.to_label() {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    let mut init_method = MethodEnvironment::new(INIT.clone(), field.kind.clone());
    let mut body = RETURNSATEMENT.clone();

    body.children.push(RETURN.clone());
    match field.value {
        Some(ref v) => body.children.push(v.clone()),
        None => body.children.push(NUMVALUEZERO.clone()),
    }
    body.children.push(SEMICOLON.clone());

    init_method.body = Some(body);

    // build init method
    match method::go(&init_method,
                     &init_label,
                     &fields,
                     &mut text,
                     &mut externs,
                     &mut bss,
                     &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    Ok(Some(INIT.clone()))
}
