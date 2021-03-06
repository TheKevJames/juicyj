use std::collections::HashMap;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::TokenKind;

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
        TokenKind::FSlash => text.push("  ; /".to_owned()),
        TokenKind::Minus => text.push("  ; -".to_owned()),
        TokenKind::Percent => text.push("  ; %".to_owned()),
        TokenKind::Plus => text.push("  ; +".to_owned()),
        TokenKind::Star => text.push("  ; *".to_owned()),
        _ => return Err(format!("attempted to parse {:?} as math", node)),
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

    // restore lhs
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ECX, Reg::EAX));
    text.push(format!("{} {}", Instr::POP, Reg::EAX));

    match node.token.kind {
        TokenKind::FSlash => {
            text.push(format!("{} {}, {}", Instr::XOR, Reg::EDX, Reg::EDX));

            // catch division by zero
            text.push(format!("{} {}, {}", Instr::CMP, Reg::ECX, Reg::EDX));
            text.push(format!("{} {}", Instr::JE, "__exception"));
            externs.push(format!("{} {}", Instr::EXTERN, "__exception"));

            text.push(format!("{} {}", Instr::DIV, Reg::ECX));
        }
        TokenKind::Minus => text.push(format!("{} {}, {}", Instr::SUB, Reg::EAX, Reg::ECX)),
        TokenKind::Percent => {
            text.push(format!("{} {}, {}", Instr::XOR, Reg::EDX, Reg::EDX));
            text.push(format!("{} {}", Instr::DIV, Reg::ECX));
            text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, Reg::EDX));
        }
        TokenKind::Plus => text.push(format!("{} {}, {}", Instr::ADD, Reg::EAX, Reg::ECX)),
        TokenKind::Star => text.push(format!("{} {}, {}", Instr::MUL, Reg::EAX, Reg::ECX)),
        _ => return Err(format!("attempted to parse {:?} as math", node)),
    }

    text.push("".to_owned());

    // TODO<codegen>: kind is lhs-ish
    Ok(None)
}
