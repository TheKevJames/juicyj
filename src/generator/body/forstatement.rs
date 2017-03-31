extern crate rand;

use std::collections::HashMap;

use self::rand::Rng;

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
    text.push(format!("  ; for"));

    let idxinit = 2;
    let mut idxcond = 3;
    let mut idxupdt = 4;
    let idxbody = node.children.len() - 1;

    if node.children[idxinit].token.kind != TokenKind::Semicolon {
        idxcond += 1;
        idxupdt += 1;

        match statement::go(&node.children[idxinit],
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
    }

    let looplabel = format!("for{}",
                            rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());
    let donelabel = format!("for{}",
                            rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    text.push(format!(".{}:", looplabel));

    if node.children[idxcond].token.kind != TokenKind::Semicolon {
        idxupdt += 1;

        match statement::go(&node.children[idxcond],
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

        text.push(format!("{} {}, {}", Instr::CMP, Reg::AL, "0"));
        text.push(format!("{} .{}", Instr::JE, donelabel));
        text.push("".to_owned());
    }

    match statement::go(&node.children[idxbody],
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

    if node.children[idxupdt].token.kind != TokenKind::RParen {
        match statement::go(&node.children[idxupdt],
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
    }

    text.push(format!("{} .{}", Instr::JMP, looplabel));
    text.push(format!(".{}:", donelabel));
    text.push("".to_owned());

    Ok(None)
}
