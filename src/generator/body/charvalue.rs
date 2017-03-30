extern crate rand;

use self::rand::Rng;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<Option<String>, String> {
    let charlabel = format!("char{}",
                            rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    match node.token.lexeme {
        Some(ref l) => {
            data.push(format!("{}: dw '{}'", charlabel, l));

            text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, charlabel));

            // TODO<codegen>: kind is char
            Ok(None)
        }
        _ => Err(format!("CharValue {:?} has no value", node)),
    }
}
