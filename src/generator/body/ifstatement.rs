extern crate rand;

use self::rand::Rng;

use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    text.push(format!("  ; if"));

    match statement::go(&node.children[2],
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let jmplabel = format!("if{}",
                           rand::thread_rng().gen_ascii_chars().take(32).collect::<String>());

    text.push(format!("  cmp {}, {}", "eax", "1"));
    text.push(format!("  jne .{}", jmplabel));
    text.push("".to_owned());

    match statement::go(&node.children[4],
                        label,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    text.push(format!(".{}:", jmplabel));
    text.push("".to_owned());

    Ok(())
}
