extern crate rand;

use self::rand::Rng;

use scanner::ASTNode;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let charlabel = format!("char{}",
                            rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    match node.token.lexeme {
        Some(ref l) => {
            data.push(format!("  {}: dw '{}'", charlabel, l));

            text.push(format!("  mov {}, [{}]", "eax", charlabel));
            Ok(())
        }
        _ => Err(format!("CharValue {:?} has no value", node)),
    }
}
