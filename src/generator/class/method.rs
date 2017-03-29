use analysis::MethodEnvironment;
use analysis::VariableEnvironment;
use generator::asm::Instr;
use generator::asm::Reg;
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

fn build_entrypoint(class_label: &String,
                    label: &String,
                    mut text: &mut Vec<String>,
                    mut externs: &mut Vec<String>) {
    // use this method as the entry point
    externs.push(format!("{} {}", Instr::GLOBAL, "_start"));
    text.push(format!("{}", "_start:"));

    // TODO<codegen>: catch panic()
    let constructor_label = class_label.split_at(class_label.rfind(".").unwrap()).1;
    let constructor_label = format!("__{}{}__", class_label, constructor_label);

    text.push(format!("{} {}", Instr::PUSH, Reg::EBP));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBP, Reg::ESP));
    text.push(format!("{} {}", Instr::CALL, constructor_label));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESP, Reg::EBP));
    text.push(format!("{} {}", Instr::POP, Reg::EBP));
    text.push("".to_owned());

    // call this method
    text.push(format!("{} {}", Instr::PUSH, Reg::EBP));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBP, Reg::ESP));
    text.push(format!("{} {}", Instr::CALL, label));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESP, Reg::EBP));
    text.push(format!("{} {}", Instr::POP, Reg::EBP));
    text.push("".to_owned());

    // exit with this method's return value
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBX, Reg::EAX));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, "1"));
    text.push(format!("{} {}", Instr::INT, "0x80")); // TODO: syscall?
    text.push("".to_owned());
}

pub fn get_args(parameters: &Vec<VariableEnvironment>,
                label: &String,
                mut text: &mut Vec<String>,
                mut bss: &mut Vec<String>)
                -> Result<(), String> {
    if parameters.is_empty() {
        return Ok(());
    }

    text.push(format!("  ; get args"));
    for (idx, param) in parameters.iter().enumerate().rev() {
        let variable = match param.name.to_label() {
            Ok(l) => format!("{}.{}", label, l),
            Err(e) => return Err(e),
        };
        bss.push(variable.clone());

        text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::ESP));
        text.push(format!("{} {}, {}", Instr::ADD, Reg::ESI, 4 * (idx + 1)));
        text.push(format!("{} [{}], {}", Instr::MOV, variable, Reg::ESI));
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

    externs.push(format!("{} {}", Instr::GLOBAL, label));
    text.push(format!("{}:", label));

    Ok(label)
}

pub fn go(method: &MethodEnvironment,
          class_label: &String,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<String>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let label = match get_label(method, &class_label, &mut text, &mut externs) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    match get_args(&method.parameters, &label, &mut text, &mut bss) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // generate body
    if let Some(b) = method.body.clone() {
        match body::go(&b,
                       &class_label,
                       &label,
                       &mut text,
                       &mut externs,
                       &mut bss,
                       &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    // TODO<codegen>: else error?

    if method.modifiers.contains(&*STATIC) && method.return_type == *INTEGER &&
       method.name == *TEST {
        build_entrypoint(&class_label, &label, text, externs);
    }

    Ok(())
}
