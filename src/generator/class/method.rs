use analysis::MethodEnvironment;
use analysis::VariableEnvironment;
use generator::asm::Register;
use generator::body;

use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref INTEGER: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Int, None), children: Vec::new() }
    };
    static ref STATIC: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Static, None), children: Vec::new() }
    };
    static ref TEST: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Identifier, Some("test")), children: Vec::new() }
    };
}

fn build_entrypoint(label: &String, mut text: &mut Vec<String>, mut externs: &mut Vec<String>) {
    // use this method as the entry point
    externs.push("global _start".to_owned());
    text.push("_start:".to_owned());

    // call this method
    text.push(format!("  push {}", Register::EBP));
    text.push(format!("  mov {}, {}", Register::EBP, Register::ESP));
    text.push(format!("  call {}", label));
    text.push(format!("  mov {}, {}", Register::ESP, Register::EBP));
    text.push(format!("  pop {}", Register::EBP));
    text.push("".to_owned());

    // exit with this method's return value
    text.push(format!("  mov {}, {}", Register::EBX, Register::EAX));
    text.push(format!("  mov {}, {}", Register::EAX, "1"));
    text.push(format!("  int {}", "0x80")); // TODO: syscall?
    text.push("".to_owned());
}

pub fn get_args(parameters: &Vec<VariableEnvironment>,
                label: &String,
                mut text: &mut Vec<String>,
                mut bss: &mut Vec<String>)
                -> Result<(), String> {
    text.push(format!("  ; get args"));
    for (idx, param) in parameters.iter().enumerate().rev() {
        let variable = match param.name.to_label() {
            Ok(l) => format!("{}.{}", label, l),
            Err(e) => return Err(e),
        };
        bss.push(variable.clone());

        text.push(format!("  mov {}, {}", Register::ESI, Register::ESP));
        text.push(format!("  add {}, {}", Register::ESI, 4 * (idx + 1)));
        text.push(format!("  mov [{}], {}", variable, Register::ESI));
    }
    text.push("".to_owned());

    Ok(())
}

pub fn get_label(method: &MethodEnvironment,
                 class_label: &String,
                 mut text: &mut Vec<String>,
                 mut externs: &mut Vec<String>)
                 -> Result<String, String> {
    let label = match method.to_label(class_label.clone()) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    externs.push(format!("global {}", label));
    text.push(format!("{}:", label));

    Ok(label)
}

pub fn go(method: &MethodEnvironment,
          label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    match get_args(&method.parameters, label, &mut text, &mut bss) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // generate body
    if let Some(b) = method.body.clone() {
        match body::go(&b, &label, &mut text, &mut externs, &mut bss, &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    // TODO<codegen>: else error?

    text.push("".to_owned());

    if method.modifiers.contains(&*STATIC) && method.return_type == *INTEGER &&
       method.name == *TEST {
        build_entrypoint(&label, text, externs);
    }

    Ok(())
}
