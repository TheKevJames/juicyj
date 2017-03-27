use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match node.children.len() {
        3 => {
            match statement::go(&node.children[1],
                                &mut text,
                                &mut externs,
                                &mut bss,
                                &mut data) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            text.push(format!("  {}", "ret"));
        }
        _ => {
            // TODO: ret a specific value, maybe 0?
            text.push(format!("  {}", "ret"));
        }
    }

    Ok(())
}
