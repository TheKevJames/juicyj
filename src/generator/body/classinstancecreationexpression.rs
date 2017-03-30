use std::collections::HashMap;

use generator::asm::helper::call;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref EMPTYPARAMS: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ParameterList")),
            children: Vec::new(),
        }
    };
}

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<Option<String>, String> {
    let params = match node.children.len() {
        2 => node.children[1].clone(),
        _ => EMPTYPARAMS.clone(),
    };

    call(&Reg::EBX, // Note: this should not matter, will be overridden
         &node.children[0],
         &params,
         class_label,
         label,
         fields,
         &mut text,
         &mut externs,
         &mut bss,
         &mut data)
}
