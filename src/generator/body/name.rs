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
    let mut node = node.clone();
    node.flatten();

    let (field, variable) = match node.to_label() {
        Ok(l) => (l.clone(), format!("{}.{}", label, l)),
        Err(e) => return Err(e),
    };

    if bss.contains(&variable) {
        // local
        text.push(format!("  ; {}", variable));

        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::ESI, variable));
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
        text.push("".to_owned());

        return Ok(());
    }

    if let Some(myfields) = fields.get(class_label) {
        // implicit-this field
        let fidx = myfields.iter().position(|fld| fld == &field);
        if fidx.is_some() {
            text.push(format!("  ; <this>.{}", field));

            text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::EBX));
            text.push(format!("{} {}, {}", Instr::ADD, Reg::ESI, 32 * fidx.unwrap()));
            text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
            text.push("".to_owned());

            return Ok(());
        }
    }

    // check if this is a FieldAccess
    // TODO: try to fix this in env?
    node.token.lexeme = Some("FieldAccess".to_owned());
    statement::go(&node,
                  class_label,
                  label,
                  fields,
                  &mut text,
                  &mut externs,
                  &mut bss,
                  &mut data)

    // Err(format!("NotImplemented Name {:?}", node))
}
