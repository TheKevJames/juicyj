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
    text.push(format!("  ; !"));

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

    text.push(format!("{} {}, {}", Instr::CMP, Reg::EAX, "0"));
    text.push(format!("{} {}", Instr::SETE, Reg::AL));
    text.push("".to_owned());

    Ok(())
}
