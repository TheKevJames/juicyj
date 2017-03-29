use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use super::statement;

lazy_static! {
    static ref NAME: ASTNode = {
        ASTNode { token: Token::new(TokenKind::NonTerminal, Some("Name")), children: Vec::new() }
    };
}

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let mut node = node.clone();
    node.flatten();

    let variable = match node.to_label() {
        Ok(l) => format!("{}.{}", label, l),
        Err(e) => return Err(e),
    };

    if bss.contains(&variable) {
        // local
        text.push(format!("  ; {}", variable));

        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::ESI, variable));
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
        text.push("".to_owned());

        return Ok(());
    }

    let variable = match node.to_label() {
        Ok(l) => format!("{}.{}", class_label, l),
        Err(e) => return Err(e),
    };

    if bss.contains(&variable) {
        // field
        text.push(format!("  ; this.{}", variable));

        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::ESI, variable));
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
        text.push("".to_owned());

        return Ok(());
    }

    // check if this is a FieldAccess
    // TODO: try to fix this in env?
    node.token.lexeme = Some("FieldAccess".to_owned());
    statement::go(&node,
                  class_label,
                  label,
                  &mut text,
                  &mut externs,
                  &mut bss,
                  &mut data)

    // Err(format!("NotImplemented Name {:?}", node))
}
