use scanner::ASTNode;

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    // TODO<codegen>
    // Err(format!("NotImplemented Null {:?}", node))
    Ok(())
}
