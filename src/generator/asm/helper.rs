use generator::body;
use scanner::ASTNode;
use scanner::TokenKind;

use super::Instr;
use super::Reg;

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

pub fn call(method: &ASTNode,
            params: &ASTNode,
            class_label: &String,
            label: &String,
            mut text: &mut Vec<String>,
            mut externs: &mut Vec<String>,
            mut bss: &mut Vec<String>,
            mut data: &mut Vec<String>)
            -> Result<(), String> {
    let method = match build_method(&method, &params) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let mut param_labels = Vec::new();
    for param in &params.children {
        // TODO<codegen>: remove commas earlier (ie. in a single place...)
        if param.token.kind == TokenKind::Comma {
            continue;
        }

        match param.to_label() {
            Ok(l) => param_labels.push(l),
            Err(e) => return Err(e),
        }
    }
    text.push(format!("  ; {}({})", method, param_labels.join(", ")));

    // push stack frame
    text.push(format!("{} {}", Instr::PUSH, Reg::EBP));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBP, Reg::ESP));

    // push params
    for param in params.children.iter().rev() {
        // TODO<codegen>: remove commas earlier (ie. in a single place...)
        if param.token.kind == TokenKind::Comma {
            continue;
        }

        match body::go(&param,
                       class_label,
                       label,
                       &mut text,
                       &mut externs,
                       &mut bss,
                       &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        text.push(format!("{} {}", Instr::PUSH, Reg::EAX));
    }

    // call method
    externs.push(format!("{} {}", Instr::EXTERN, method));
    text.push(format!("{} {}", Instr::CALL, method));

    // pop stack by number of params
    text.push(format!("{} {}, {}", Instr::ADD, Reg::ESP, 4 * params.children.len()));

    // pop stack frame
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESP, Reg::EBP));
    text.push(format!("{} {}", Instr::POP, Reg::EBP));
    text.push("".to_owned());

    Ok(())
}
