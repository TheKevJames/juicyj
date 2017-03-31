use std::collections::HashMap;

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

    text.push(format!("{} dword {}, {}", Instr::MOV, Reg::EBX, "0xC0DEBABE"));
    text.push(format!("{} {}", Instr::PUSH, Reg::EBX)); // fake this param

    text.push(format!("{} {}", Instr::PUSH, Reg::EBP));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBP, Reg::ESP));
    text.push(format!("{} {}", Instr::CALL, constructor_label));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESP, Reg::EBP));
    text.push(format!("{} {}", Instr::POP, Reg::EBP));

    text.push(format!("{} {}", Instr::POP, Reg::EBX)); // fake this param
    text.push("".to_owned());

    // call this method
    text.push(format!("{} {}", Instr::PUSH, Reg::EAX)); // real this param

    text.push(format!("{} {}", Instr::PUSH, Reg::EBP));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBP, Reg::ESP));
    text.push(format!("{} {}", Instr::CALL, label));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESP, Reg::EBP));
    text.push(format!("{} {}", Instr::POP, Reg::EBP));

    // text.push(format!("{} {}", Instr::POP, Reg::EBX)); // real this param
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
                mut externs: &mut Vec<String>,
                mut bss: &mut Vec<(String, String)>)
                -> Result<(), String> {
    text.push(format!("  ; get args"));
    for (idx, param) in parameters.iter().enumerate().rev() {
        let variable = match param.name.to_label() {
            Ok(l) => format!("{}.{}", label, l),
            Err(e) => return Err(e),
        };
        let pkind = match param.kind.to_label() {
            Ok(k) => k,
            Err(e) => return Err(e),
        };
        bss.push((variable.clone(), pkind.clone()));

        externs.push(format!("{} {}", Instr::EXTERN, "__malloc"));

        // allocate space for variable
        text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, "4"));
        text.push(format!("{} {}", Instr::CALL, "__malloc"));

        // put address of variable in new space
        // 0:"esp", 4:"null", 8:"argx", ... n:"arg0", n+4:"this param", n+8:"this"
        text.push(format!("{} {}, [{}+4*{}]", Instr::MOV, Reg::EBX, Reg::ESP, idx + 2));
        text.push(format!("{} [{}], {}", Instr::MOV, Reg::EAX, Reg::EBX));

        text.push(format!("{} [{}], {}", Instr::MOV, variable, Reg::EAX));
        text.push("".to_owned());
    }

    text.push(format!("  ; get this"));
    text.push(format!("{} {}, [{}+4*{}]",
                      Instr::MOV,
                      Reg::EBX,
                      Reg::ESP,
                      parameters.len() + 2));
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
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let label = match get_label(method, &class_label, &mut text, &mut externs) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    match get_args(&method.parameters,
                   &label,
                   &mut text,
                   &mut externs,
                   &mut bss) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // generate body
    if let Some(b) = method.body.clone() {
        match body::go(&b,
                       &class_label,
                       &label,
                       &fields,
                       &mut text,
                       &mut externs,
                       &mut bss,
                       &mut data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    if method.modifiers.contains(&*STATIC) && method.return_type == *INTEGER &&
       method.name == *TEST {
        build_entrypoint(&class_label, &label, text, externs);
    }

    Ok(())
}
