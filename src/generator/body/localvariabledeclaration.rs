use scanner::ASTNode;

use super::statement;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let kind = match node.children[0].to_param() {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let name = match node.children[1].children[0].to_label() {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    text.push(format!("  ; {} {}", kind, name));

    let variable = match node.children[1].children[0].to_label() {
        Ok(l) => l,
        Err(e) => return Err(e),
    };
    bss.push(format!("  {}: resb {}", variable, "32"));

    // allocate 32 bytes for lhs
    text.push(format!("  mov {}, {}", "eax", "32"));
    externs.push(format!("extern {}", "__malloc"));
    text.push(format!("  call {}", "__malloc"));
    text.push(format!("  mov [{}], {}", variable, "eax"));
    text.push("".to_owned());

    // resolve rhs and store in lhs
    match statement::go(&node.children[1].children[1],
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    text.push(format!("  mov {}, [{}]", "edi", variable));
    text.push(format!("  mov [{}], {}", "edi", "eax"));
    text.push("".to_owned());

    Ok(())
}
