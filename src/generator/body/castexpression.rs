use std::collections::HashMap;

use scanner::ASTNode;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<Option<String>, String> {
    // TODO<codegen>
    // Err(format!("NotImplemented CastExpression {:?}", node))

    // TODO<codegen>: kind is lhs
    Ok(None)
}
