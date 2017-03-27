extern crate rand;

use self::rand::Rng;

use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    text.push(format!("  ; if"));

    match statement::go(&node.children[2],
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let label = format!("if{}",
                        rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    text.push(format!("  cmp {}, {}", "eax", "1"));
    text.push(format!("  jne .{}", label));
    match statement::go(&node.children[4],
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    text.push(format!(".{}:", label));
    text.push("".to_owned());

    Ok(())
}
