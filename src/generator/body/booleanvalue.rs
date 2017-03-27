use scanner::ASTNode;
use scanner::TokenKind;

pub fn go(node: &ASTNode, mut text: &mut Vec<String>) -> Result<(), String> {
    match node.token.kind {
        TokenKind::False => text.push(format!("  mov {}, {}", "eax", "0")),
        TokenKind::True => text.push(format!("  mov {}, {}", "eax", "1")),
        _ => return Err(format!("attempted to parse {:?} as boolean", node)),
    }

    Ok(())
}
