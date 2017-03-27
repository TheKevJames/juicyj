extern crate rand;

use self::rand::Rng;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::TokenKind;

use super::statement;

pub fn go(node: &ASTNode,
          class_label: &String,
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
                        class_label,
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
            text.push(format!("{} {}, {}", Instr::CMP, Reg::EAX, "0"));
            text.push(format!("{} .{}", Instr::JE, lazylabel));
        }
        TokenKind::Or => {
            text.push(format!("{} {}, {}", Instr::CMP, Reg::EAX, "0"));
            text.push(format!("{} .{}", Instr::JNE, lazylabel));
        }
        _ => (),
    }

    // store lhs while we get rhs
    text.push(format!("{} {}", Instr::PUSH, Reg::EAX));

    // get rhs
    match statement::go(&node.children[1],
                        class_label,
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // restore lhs and do operation
    text.push(format!("{} {}", Instr::POP, Reg::EDX));

    match node.token.kind {
        TokenKind::And => text.push(format!(".{}:", lazylabel)),
        TokenKind::BitAnd => text.push(format!("{} {}, {}", Instr::AND, Reg::EAX, Reg::EDX)),
        TokenKind::BitOr => text.push(format!("{} {}, {}", Instr::OR, Reg::EAX, Reg::EDX)),
        TokenKind::BitXor => text.push(format!("{} {}, {}", Instr::XOR, Reg::EAX, Reg::EDX)),
        TokenKind::Or => text.push(format!(".{}:", lazylabel)),
        _ => return Err(format!("attempted to parse {:?} as boolean operation", node)),
    }

    text.push("".to_owned());
    Ok(())
}
