extern crate rand;

use self::rand::Rng;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    text.push(format!("  ; if-else"));

    match statement::go(&node.children[2],
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let elselabel = format!("ifelse{}",
                            rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());
    let donelabel = format!("ifelse{}",
                            rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    text.push(format!("{} {}, {}", Instr::CMP, Reg::AL, "1"));
    text.push(format!("{} .{}", Instr::JNE, elselabel));
    text.push("".to_owned());

    match statement::go(&node.children[4],
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    text.push(format!("{} .{}", Instr::JMP, donelabel));
    text.push("".to_owned());

    text.push(format!(".{}:", elselabel));

    match statement::go(&node.children[6],
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    text.push(format!(".{}:", donelabel));
    text.push("".to_owned());

    Ok(())
}
