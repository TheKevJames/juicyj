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
    let mut node = node.clone();
    node.flatten();

    let (field, variable) = match node.to_label() {
        Ok(l) => (l.clone(), format!("{}.{}", label, l)),
        Err(e) => return Err(e),
    };

    let vidx = bss.iter().position(|v| v.0 == variable);
    if vidx.is_some() {
        // local
        text.push(format!("  ; {}", variable));

        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, variable));
        text.push("".to_owned());

        return Ok(Some(bss[vidx.unwrap()].1.clone()));
    }

    if let Some(myfields) = fields.get(class_label) {
        // implicit-this field
        let fidx = myfields.iter().position(|fld| fld.0 == field);
        if fidx.is_some() {
            text.push(format!("  ; <this>.{}", field));

            text.push(format!("{} {}, [{}+4*{}]",
                              Instr::MOV,
                              Reg::EAX,
                              Reg::EBX,
                              fidx.unwrap() + 1));
            text.push("".to_owned());

            return Ok(Some(myfields[fidx.unwrap()].1.clone()));
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
}
