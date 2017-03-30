use std::collections::HashMap;

use scanner::ASTNode;
use scanner::TokenKind;
use generator::asm::Instr;
use generator::asm::Reg;

use super::statement;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<String>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match node.token.kind {
        TokenKind::Equality => text.push("  ; ==".to_owned()),
        TokenKind::NotEqual => text.push("  ; !=".to_owned()),
        TokenKind::LessThan => text.push("  ; <".to_owned()),
        TokenKind::LessThanOrEqual => text.push("  ; <=".to_owned()),
        TokenKind::GreaterThan => text.push("  ; >".to_owned()),
        TokenKind::GreaterThanOrEqual => text.push("  ; >=".to_owned()),
        _ => return Err(format!("attempted to parse {:?} as comparison", node)),
    }

    // get lhs
    match statement::go(&node.children[0],
                        class_label,
                        label,
                        fields,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // store lhs while we get rhs
    text.push(format!("{} {}", Instr::PUSH, Reg::EAX));

    // get rhs
    match statement::go(&node.children[1],
                        class_label,
                        label,
                        fields,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // restore lhs and do comparison
    text.push(format!("{} {}", Instr::POP, Reg::EDX));
    text.push(format!("{} {}, {}", Instr::CMP, Reg::EDX, Reg::EAX));

    match node.token.kind {
        TokenKind::Equality => {
            text.push(format!("  sete {}", "al"));
        }
        TokenKind::NotEqual => {
            text.push(format!("  setne {}", "al"));
        }
        TokenKind::LessThan => {
            text.push(format!("  setl {}", "al"));
        }
        TokenKind::LessThanOrEqual => {
            text.push(format!("  setle {}", "al"));
        }
        TokenKind::GreaterThan => {
            text.push(format!("  setg {}", "al"));
        }
        TokenKind::GreaterThanOrEqual => {
            text.push(format!("  setge {}", "al"));
        }
        _ => return Err(format!("attempted to parse {:?} as comparison", node)),
    }

    text.push("".to_owned());
    Ok(())
}
