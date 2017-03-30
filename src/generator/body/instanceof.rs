use std::collections::HashMap;

use scanner::ASTNode;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<String>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    // TODO<codegen>
    // Err(format!("NotImplemented Instanceof {:?}", node))
    Ok(())
}
