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
    match node.children[1].token.lexeme {
        Some(ref l) if l == "DimExpr" => {
            text.push(format!("  ; new array"));
            // resolve array length
            match statement::go(&node.children[1].children[1],
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

            // store size
            text.push(format!("{} {}", Instr::PUSH, Reg::EAX));

            // allocate 4 + 4*l bytes
            text.push(format!("{} {}, {}", Instr::MOV, Reg::ECX, "4"));
            text.push(format!("{} {}, {}", Instr::MUL, Reg::EAX, Reg::ECX));
            text.push(format!("{} {}, {}", Instr::ADD, Reg::EAX, Reg::ECX));

            text.push(format!("{} {}", Instr::PUSH, Reg::EBX));
            externs.push(format!("{} {}", Instr::EXTERN, "__malloc"));
            text.push(format!("{} {}", Instr::CALL, "__malloc"));
            text.push(format!("{} {}", Instr::POP, Reg::EBX));
            text.push("".to_owned());

            // set size in array memory
            text.push(format!("  ; array.length = x"));
            text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::EAX));
            text.push(format!("{} {}", Instr::POP, Reg::EAX));
            text.push(format!("{} [{}], {}", Instr::MOV, Reg::ESI, Reg::EAX));
            text.push("".to_owned());
        }
        Some(ref l) if l == "Dim" => return Err(format!("found Dim in ArrayCreation {:?}", node)),
        _ => {
            return Err(format!("ArrayCreationExpression {:?} did not have Dim or DimExpr",
                               node))
        }
    }

    // TODO<codegen>: kind is ArrayType of subexpr
    Ok(None)
}
