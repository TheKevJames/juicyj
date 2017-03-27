extern crate rand;

use self::rand::Rng;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let strlabel = format!("str{}",
                           rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    match node.token.lexeme {
        Some(ref l) => {
            data.push(format!("{}: dw '{}'", strlabel, l));

            text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, strlabel));
            Ok(())
        }
        _ => Err(format!("StrValue {:?} has no value", node)),
    }
}
