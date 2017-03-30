use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

pub fn go(node: &ASTNode, mut text: &mut Vec<String>) -> Result<Option<String>, String> {
    if node.token.lexeme.is_none() {
        return Err(format!("CharValue {:?} has no value", node));
    }

    let value = node.clone().token.lexeme.unwrap();

    // TODO<codegen>: store this somewhere so we have an address?
    text.push(format!("{} dword {}, '{}'", Instr::MOV, Reg::ESI, value));
    text.push(format!("{} {}, '{}'", Instr::MOV, Reg::EAX, value));

    // TODO<codegen>: kind is char
    Ok(None)
}
