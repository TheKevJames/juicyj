use scanner::ASTNode;
use scanner::TokenKind;

use super::statement;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match node.token.kind {
        TokenKind::FSlash => text.push("  ; /".to_owned()),
        TokenKind::Minus => text.push("  ; -".to_owned()),
        TokenKind::Percent => text.push("  ; %".to_owned()),
        TokenKind::Plus => text.push("  ; +".to_owned()),
        TokenKind::Star => text.push("  ; *".to_owned()),
        _ => return Err(format!("attempted to parse {:?} as math", node)),
    }

    // get lhs
    match statement::go(&node.children[0],
                        label,
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
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // restore lhs
    text.push(format!("  mov {}, {}", "ecx", "eax"));
    text.push(format!("  pop {}", "eax"));

    match node.token.kind {
        TokenKind::FSlash => {
            text.push(format!("  xor {}, {}", "edx", "edx"));
            text.push(format!("  div {}", "ecx"));
        }
        TokenKind::Minus => text.push(format!("  sub {}, {}", "eax", "ecx")),
        TokenKind::Percent => {
            text.push(format!("  xor {}, {}", "edx", "edx"));
            text.push(format!("  div {}", "ecx"));
            text.push(format!("  mov {}, {}", "eax", "edx"));
        }
        TokenKind::Plus => text.push(format!("  add {}, {}", "eax", "ecx")),
        TokenKind::Star => text.push(format!("  imul {}, {}", "eax", "ecx")),
        _ => return Err(format!("attempted to parse {:?} as math", node)),
    }

    text.push("".to_owned());
    Ok(())
}
