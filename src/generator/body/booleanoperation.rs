extern crate rand;

use self::rand::Rng;

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
        TokenKind::And => text.push("  ; &&".to_owned()),
        TokenKind::BitAnd => text.push("  ; &".to_owned()),
        TokenKind::BitOr => text.push("  ; |".to_owned()),
        TokenKind::BitXor => text.push("  ; ^".to_owned()),
        TokenKind::Or => text.push("  ; ||".to_owned()),
        _ => return Err(format!("attempted to parse {:?} as boolean operation", node)),
    }

    let lazylabel = format!("lazy{}",
                            rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

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

    match node.token.kind {
        TokenKind::And => {
            text.push(format!("  cmp {}, {}", "eax", "0"));
            text.push(format!("  je .{}", lazylabel));
        }
        TokenKind::Or => {
            text.push(format!("  cmp {}, {}", "eax", "0"));
            text.push(format!("  jne .{}", lazylabel));
        }
        _ => (),
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

    // restore lhs and do operation
    text.push(format!("  pop {}", "edx"));

    match node.token.kind {
        TokenKind::And => text.push(format!(".{}:", lazylabel)),
        TokenKind::BitAnd => text.push(format!("  and {}, {}", "eax", "edx")),
        TokenKind::BitOr => text.push(format!("  or {}, {}", "eax", "edx")),
        TokenKind::BitXor => text.push(format!("  xor {}, {}", "eax", "edx")),
        TokenKind::Or => text.push(format!(".{}:", lazylabel)),
        _ => return Err(format!("attempted to parse {:?} as boolean operation", node)),
    }

    text.push("".to_owned());
    Ok(())
}
