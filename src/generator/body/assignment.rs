use std::collections::HashMap;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

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
    let kind = match statement::go(&node.children[0],
                                   class_label,
                                   label,
                                   fields,
                                   &mut text,
                                   &mut externs,
                                   &mut bss,
                                   &mut data) {
        Ok(k) => (k),
        Err(e) => return Err(e),
    };

    // store lhs address
    text.push(format!("{} {}", Instr::PUSH, Reg::ESI));
    text.push("".to_owned());

    // get rhs
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

    // store rhs value in lhs address
    text.push(format!("{} {}", Instr::POP, Reg::EDI));
    text.push(format!("{} [{}], {}", Instr::MOV, Reg::EDI, Reg::EAX));
    text.push("".to_owned());

    return Ok(kind);
}
