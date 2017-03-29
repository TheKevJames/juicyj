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

    // store lhs address
    text.push(format!("{} {}", Instr::PUSH, Reg::ESI));

    // get rhs
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

    // store rhs value in lhs address
    text.push(format!("{} {}", Instr::POP, Reg::EDI));
    text.push(format!("{} [{}], {}", Instr::MOV, Reg::EDI, Reg::EAX));
    text.push("".to_owned());

    return Ok(());
}
