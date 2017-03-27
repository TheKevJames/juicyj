extern crate rand;

use self::rand::Rng;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    text.push(format!("  ; while"));

    let startlabel = format!("while{}",
                             rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());
    let endlabel = format!("while{}",
                           rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    text.push(format!(".{}:", startlabel));
    match statement::go(&node.children[2],
                        class_label,
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    text.push(format!("{} {}, {}", Instr::CMP, Reg::AL, "1"));
    text.push(format!("{} .{}", Instr::JNE, endlabel));
    text.push("".to_owned());

    match statement::go(&node.children[4],
                        class_label,
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    text.push(format!("{} .{}", Instr::JMP, startlabel));
    text.push(format!(".{}:", endlabel));
    text.push("".to_owned());

    Ok(())
}
