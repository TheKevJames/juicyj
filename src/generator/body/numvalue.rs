use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>)
          -> Result<Option<String>, String> {
    if node.token.lexeme.is_none() {
        return Err(format!("NumValue {:?} has no value", node));
    }

    let value = node.clone().token.lexeme.unwrap();

    // allocate 4 bytes for num
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, "4"));

    text.push(format!("{} {}", Instr::PUSH, Reg::EBX));
    externs.push(format!("{} {}", Instr::EXTERN, "__malloc"));
    text.push(format!("{} {}", Instr::CALL, "__malloc"));
    text.push(format!("{} {}", Instr::POP, Reg::EBX));

    // store value in memory
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::EAX));
    text.push(format!("{} dword [{}], {}", Instr::MOV, Reg::ESI, value));

    text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
    text.push("".to_owned());

    // TODO<codegen>: kind is int/short
    Ok(None)
}
