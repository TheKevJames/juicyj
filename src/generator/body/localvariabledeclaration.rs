use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let kind = match node.children[0].to_param() {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let name = match node.children[1].children[0].to_label() {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    text.push(format!("  ; {} {} = x", kind, name));

    let variable = match node.children[1].children[0].to_label() {
        Ok(l) => format!("{}.{}", label, l),
        Err(e) => return Err(e),
    };
    bss.push(variable.clone());

    // allocate 32 bytes for lhs
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, "32"));
    externs.push(format!("{} {}", Instr::EXTERN, "__malloc"));
    text.push(format!("{} {}", Instr::CALL, "__malloc"));
    text.push(format!("{} [{}], {}", Instr::MOV, variable, Reg::EAX));
    text.push("".to_owned());

    // resolve rhs and store in lhs
    match statement::go(&node.children[1].children[1],
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    text.push(format!("{} {}, [{}]", Instr::MOV, "edi", variable));
    text.push(format!("{} [{}], {}", Instr::MOV, "edi", Reg::EAX));
    text.push("".to_owned());

    Ok(())
}
