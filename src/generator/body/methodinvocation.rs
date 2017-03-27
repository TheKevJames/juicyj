use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::TokenKind;

use super::statement;

fn build_method(name: &ASTNode, params: &ASTNode) -> Result<String, String> {
    let mut method: Vec<String> = Vec::new();

    method.push("__".to_owned());
    match name.to_label() {
        Ok(l) => {
            if l == "java.io.PrintStream.nativeWrite" {
                // TODO: more general NATIVE lookup?
                method.push("NATIVE".to_owned());
            }
            method.push(l);
        }
        Err(e) => return Err(e),
    }

    method.push("_".to_owned());
    for param in &params.children {
        match param.to_param() {
            Ok(p) => method.push(p),
            Err(e) => return Err(e),
        }
    }
    method.push("_".to_owned());

    Ok(method.join(""))
}

pub fn go(node: &ASTNode,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let method = match build_method(&node.children[0], &node.children[2]) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let mut params = Vec::new();
    for param in &node.children[2].children {
        // TODO: remove commas earlier (ie. in a single place...)
        if param.token.kind == TokenKind::Comma {
            continue;
        }

        match param.to_label() {
            Ok(l) => params.push(l),
            Err(e) => return Err(e),
        }
    }
    text.push(format!("  ; {}({})", method, params.join(", ")));

    // push stack frame
    text.push(format!("{} {}", Instr::PUSH, Reg::EBP));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBP, Reg::ESP));
    // push params
    for param in node.children[2].children.iter().rev() {
        if param.token.kind == TokenKind::Comma {
            continue;
        }

        match statement::go(&param, label, &mut text, &mut externs, &mut bss, &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        text.push(format!("{} {}", Instr::PUSH, Reg::EAX));
    }

    externs.push(format!("{} {}", Instr::EXTERN, method));
    text.push(format!("{} {}", Instr::CALL, method));

    // pop stack by number of params
    text.push(format!("{} {}, {}",
                      Instr::ADD,
                      Reg::ESP,
                      4 * node.children[2].children.len()));
    // pop stack frame
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESP, Reg::EBP));
    text.push(format!("{} {}", Instr::POP, Reg::EBP));
    text.push("".to_owned());

    Ok(())
}
