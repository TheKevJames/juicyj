use std::collections::HashMap;

use analysis::MethodEnvironment;
use generator::asm::helper::call;
use generator::asm::Instr;
use generator::asm::Reg;
use generator::body;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use super::method;

lazy_static! {
    static ref EMPTYPARAMS: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ParameterList")),
            children: Vec::new(),
        }
    };
}

pub fn go(method: &MethodEnvironment,
          class_label: &String,
          init_fields: &Vec<(String, ASTNode)>,
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<(), String> {
    let label = match method::get_label(method, &class_label, &mut text, &mut externs) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    match method::get_args(&method.parameters,
                           &label,
                           &mut text,
                           &mut externs,
                           &mut bss) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // allocate 32 bytes for this and 32 bytes for each field
    let myfields = fields.get(class_label);
    if myfields.is_none() {
        return Err(format!("could not find own fields for {:?}", class_label));
    }

    let space = 4 * (myfields.unwrap().len() + 1);
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, space));

    text.push(format!("{} {}", Instr::PUSH, Reg::EBX));
    externs.push(format!("{} {}", Instr::EXTERN, "__malloc"));
    text.push(format!("{} {}", Instr::CALL, "__malloc"));
    text.push(format!("{} {}", Instr::POP, Reg::EBX));

    text.push(format!("{} {}, {}", Instr::MOV, Reg::EBX, Reg::EAX));
    text.push(format!("{} dword [{}], {}", Instr::MOV, Reg::EBX, "0xBAADCAFE"));
    text.push("".to_owned());

    // call parent constructor
    if let Some(p) = method.parent.clone() {
        text.push(format!("  ; implicit super()"));
        let mut parent = p.clone();
        parent.children.pop();
        parent.children.pop();

        let plabel = match parent.to_label() {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        match call(&Reg::EBX, // Note: this should not matter, will be overridden
                   &p,
                   &EMPTYPARAMS.clone(),
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

        let pfields = fields.get(&plabel);
        if pfields.is_none() {
            return Err(format!("could not find super class fields for {:?}", p));
        }

        let numpfields = pfields.unwrap().len();
        if numpfields != 0 {
            // insert parent fields into local memory allocation
            text.push(format!("  ; copy super() fields into this"));
            text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::EAX));
            for idx in 0..pfields.unwrap().len() {
                text.push(format!("{} {}, [{}+4*{}]", Instr::MOV, Reg::EAX, Reg::ESI, idx + 1));
                text.push(format!("{} [{}+4*{}], {}", Instr::MOV, Reg::EBX, idx + 1, Reg::EAX));
            }
            text.push("".to_owned());
        }
    }

    // initialize fields
    for &(ref field, ref init) in init_fields {
        text.push(format!("  ; init {}", field));

        // get initial value
        match call(&Reg::EBX,
                   &init,
                   &EMPTYPARAMS.clone(),
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

        let flookup = field.split_at(field.rfind(".").unwrap());
        let fclass = fields.get(flookup.0);
        if fclass.is_none() {
            return Err(format!("could not find fields for {:?}", field));
        }

        let mut key = flookup.1.to_owned();
        key.remove(0);
        let fidx = fclass.unwrap().iter().position(|fld| fld.0 == key);
        if fidx.is_none() {
            return Err(format!("could not find matching field for {:?} in {:?}",
                               field,
                               fields));
        }

        text.push(format!("{} [{}+4*{}], {}",
                          Instr::MOV,
                          Reg::EBX,
                          fidx.unwrap() + 1,
                          Reg::ESI));
        text.push("".to_owned());
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

    // return this
    text.push(format!("{} {}, {}", Instr::MOV, Reg::ESI, Reg::EBX));
    text.push(format!("{} {}, {}", Instr::MOV, Reg::EAX, Reg::EBX)); // TODO<codegen>: verify
    text.push(format!("{}", Instr::RET));
    text.push("".to_owned());

    Ok(())
}
