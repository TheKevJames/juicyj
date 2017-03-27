use scanner::ASTNode;
use scanner::TokenKind;

use super::statement;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match node.token.kind {
        TokenKind::Minus => text.push("  ; -".to_owned()),
        TokenKind::Plus => text.push("  ; +".to_owned()),
        _ => return Err(format!("attempted to parse {:?} as math", node)),
    }

    // get lhs
    match statement::go(&node.children[0],
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // store lhs while we get rhs
    text.push(format!("  push {}", "eax"));

    // get rhs
    match statement::go(&node.children[1],
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // restore lhs
    text.push(format!("  mov {}, {}", "edx", "eax"));
    text.push(format!("  pop {}", "eax"));

    match node.token.kind {
        TokenKind::Minus => text.push(format!("  add {}, {}", "eax", "edx")),
        TokenKind::Plus => text.push(format!("  sub {}, {}", "eax", "edx")),
        _ => return Err(format!("attempted to parse {:?} as math", node)),
    }

    text.push("".to_owned());
    Ok(())
}
