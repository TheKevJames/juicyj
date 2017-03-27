use generator::asm::helper::call;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref PARAMS: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ParameterList")),
            children: Vec::new(),
        }
    };
}

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let params = match node.children.len() {
        2 => node.children[1].clone(),
        _ => PARAMS.clone(),
    };

    call(&node.children[0],
         &params,
         label,
         &mut text,
         &mut externs,
         &mut bss,
         &mut data)
}
