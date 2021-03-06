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
    if node.clone().token.lexeme.unwrap_or("".to_owned()) == "FieldAccess" {
        return Err(format!("could not resolve node"));
    }

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

        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::ESI, variable));
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::ESI, Reg::ESI));
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
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
                              Reg::ESI,
                              Reg::EBX,
                              fidx.unwrap() + 1));
            text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
            text.push("".to_owned());

            return Ok(Some(myfields[fidx.unwrap()].1.clone()));
        }
    }

    if let Some(_) = fields.get(&field) {
        // static reference
        text.push(format!("  ; {} <static class>", field));

        // TODO<codegen>: static references shouldn't care about This value,
        // right?
        text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::EBX));
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
        text.push("".to_owned());

        return Ok(Some(field.clone()));
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
