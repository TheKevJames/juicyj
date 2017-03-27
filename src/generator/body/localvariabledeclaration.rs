use scanner::ASTNode;
use scanner::TokenKind;

use super::statement;

pub fn go(node: &ASTNode,
          mut text: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>) {
    text.push(format!("  ; {} {} = ?",
                      node.children[0].to_label(),
                      node.children[1].children[0].to_label()));

    let variable = node.children[1].children[0].to_label();
    bss.push(format!("  {}: resb {}", variable, "32"));

    // allocate 32 bytes for lhs
    text.push(format!("  mov {}, {}", "eax", "32"));
    text.push(format!("  call __malloc"));
    text.push(format!("  mov [{}], {}", variable, "eax"));
    text.push("".to_owned());

    // resolve rhs and store in lhs
    statement::go(&node.children[1].children[1],
                  &mut text,
                  &mut bss,
                  &mut data);
    text.push(format!("  mov {}, [{}]", "edi", variable));
    text.push(format!("  mov [{}], {}", "edi", "edx"));
    text.push("".to_owned());
}
