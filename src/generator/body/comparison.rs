use std::collections::HashMap;

use scanner::ASTNode;
use scanner::TokenKind;
use generator::asm::Instr;
use generator::asm::Reg;

use super::statement;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<Option<String>, String> {
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
    text.push("".to_owned());

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
            text.push(format!("{} {}", Instr::SETE, Reg::AL));
        }
        TokenKind::NotEqual => {
            text.push(format!("{} {}", Instr::SETNE, Reg::AL));
        }
        TokenKind::LessThan => {
            text.push(format!("{} {}", Instr::SETL, Reg::AL));
        }
        TokenKind::LessThanOrEqual => {
            text.push(format!("{} {}", Instr::SETLE, Reg::AL));
        }
        TokenKind::GreaterThan => {
            text.push(format!("{} {}", Instr::SETG, Reg::AL));
        }
        TokenKind::GreaterThanOrEqual => {
            text.push(format!("{} {}", Instr::SETGE, Reg::AL));
        }
        _ => return Err(format!("attempted to parse {:?} as comparison", node)),
    }

    text.push("".to_owned());

    // TODO<codegen>: kind is bool
    Ok(None)
}
