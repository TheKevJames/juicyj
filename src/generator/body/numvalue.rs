use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

pub fn go(node: &ASTNode, mut text: &mut Vec<String>) -> Result<(), String> {
    match node.token.lexeme {
        Some(ref l) => {
            text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, l));
            Ok(())
        }
        _ => Err(format!("could not parse NumValue from {:?}", node)),
    }
}
