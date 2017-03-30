use std::collections::HashMap;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

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
    // get index
    match statement::go(&node.children[2],
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

    // increment index to address offset
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ECX, "32"));
    text.push(format!("{} {}, {}", Instr::MUL, Reg::EAX, Reg::ECX));
    text.push(format!("{} {}, {}", Instr::ADD, Reg::EAX, Reg::ECX));

    // store offset
    text.push(format!("{} {}", Instr::PUSH, Reg::EAX));

    // get address of base (value: eax, addr: esi)
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

    // get value at offset
    text.push(format!("{} {}", Instr::POP, Reg::EAX));
    text.push(format!("{} {}, {}", Instr::ADD, Reg::ESI, Reg::EAX));
    text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));

    Ok(())
}
