use scanner::ASTNode;
use scanner::TokenKind;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match node.token.lexeme {
        Some(ref l) => {
            text.push(format!("  mov {}, {}", "eax", l));
            Ok(())
        }
        _ => Err(format!("could not parse NumValue from {:?}", node)),
    }
}
