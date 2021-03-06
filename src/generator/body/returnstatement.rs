use std::collections::HashMap;

use generator::asm::Instr;
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
    match node.children.len() {
        3 => {
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
        }
        _ => {
            // TODO<codegen>: ret a specific value, maybe 0?
        }
    }

    text.push(format!("{}", Instr::RET));
    text.push("".to_owned());

    // TODO<codegen>: kind is null or kind of child
    Ok(None)
}
