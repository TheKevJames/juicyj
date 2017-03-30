use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

pub fn go(node: &ASTNode, mut text: &mut Vec<String>) -> Result<Option<String>, String> {
    match node.token.lexeme {
        Some(ref l) => {
            // TODO<codegen>: store this somewhere so we have an address?
            text.push(format!("{} dword {}, {}", Instr::MOV, Reg::ESI, "0"));
            text.push(format!("{} {}, '{}'", Instr::MOV, Reg::EAX, l));

            // TODO<codegen>: kind is char
            Ok(None)
        }
        _ => Err(format!("CharValue {:?} has no value", node)),
    }
}
