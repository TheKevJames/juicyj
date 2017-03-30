use std::collections::HashMap;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::TokenKind;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<String>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    Ok(())

    // if let Some((value, children)) = node.children.split_last() {
    //     // TODO<codegen>: do field resolving correctly. Resolve `children[0]` to
    //     // instance, find `value` in that instance.
    //     // children[0] is not necessarily a Name, could be any Expression
    //     let object = children[0].clone();

    //     match object.token.kind {
    //         TokenKind::This => {
    //             let variable = match value.to_label() {
    //                 Ok(l) => format!("{}.{}", class_label, l),
    //                 Err(e) => return Err(e),
    //             };

    //             if bss.contains(&variable) {
    //                 // field
    //                 text.push(format!("  ; this.{}", variable));

    //                 text.push(format!("{} {}, [{}]", Instr::MOV, Reg::ESI, variable));
    //                 text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
    //                 text.push("".to_owned());

    //                 return Ok(());
    //             }
    //         }
    //         _ => (),
    //     }

    //     return Ok(());
    //     // return Err(format!("could not lookup FieldAccess for node {:?}", node));
    // }

    // Err(format!("malformed FieldAccess node {:?}", node))
}
