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
    match node.children[1].token.lexeme {
        Some(ref l) if l == "DimExpr" => {
            // resolve array length
            match statement::go(&node.children[1].children[1],
                                class_label,
                                label,
                                &mut text,
                                &mut externs,
                                &mut bss,
                                &mut data) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            // allocate 32 + 32*l bytes
            text.push(format!("{} {}, {}", Instr::MOV, Reg::ECX, "32"));
            text.push(format!("{} {}, {}", Instr::MUL, Reg::EAX, Reg::ECX));
            text.push(format!("{} {}, {}", Instr::ADD, Reg::EAX, Reg::ECX));
            externs.push(format!("{} {}", Instr::EXTERN, "__malloc"));
            text.push(format!("{} {}", Instr::CALL, "__malloc"));
        }
        Some(ref l) if l == "Dim" => return Err(format!("found Dim in ArrayCreation {:?}", node)),
        _ => {
            return Err(format!("ArrayCreationExpression {:?} did not have Dim or DimExpr",
                               node))
        }
    }

    Ok(())
}
