use analysis::FieldEnvironment;
use analysis::MethodEnvironment;
use generator::asm::Instr;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use super::method;

lazy_static! {
    static ref DOT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Dot, None), children: Vec::new() }
    };
    static ref INIT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Identifier, Some("INIT")), children: Vec::new() }
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
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<Option<ASTNode>, String> {
    if let Some(v) = field.value.clone() {
        let mut init = field.name.clone();
        init.flatten();
        init.children.push(DOT.clone());
        init.children.push(INIT.clone());
        let init_label = match init.to_label() {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        let mut init_method = MethodEnvironment::new(init.clone(), field.kind.clone());
        let mut body = RETURNSATEMENT.clone();
        body.children.push(RETURN.clone());
        body.children.push(v.clone());
        body.children.push(SEMICOLON.clone());
        init_method.body = Some(body);

        // build init method
        externs.push(format!("{} __{}__", Instr::GLOBAL, &init_label));
        text.push(format!("__{}__:", &init_label));
        match method::go(&init_method,
                         &init_label,
                         &mut text,
                         &mut externs,
                         &mut bss,
                         &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        bss.push(label.clone());
        Ok(Some(init.clone()))
    } else {
        bss.push(label.clone());
        Ok(None)
    }
}
