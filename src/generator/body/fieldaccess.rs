use std::collections::HashMap;

use generator::asm::Instr;
use generator::asm::Reg;
use scanner::ASTNode;
use scanner::TokenKind;

use super::statement;

pub fn go(node: &ASTNode,
          class_label: &String,
          label: &String,
          fields: &HashMap<String, Vec<(String, String)>>,
          mut text: &mut Vec<String>,
          mut externs: &mut Vec<String>,
          mut bss: &mut Vec<(String, String)>,
          mut data: &mut Vec<String>)
          -> Result<Option<String>, String> {
    let kind = match statement::go(&node.children[0],
                                   class_label,
                                   label,
                                   fields,
                                   &mut text,
                                   &mut externs,
                                   &mut bss,
                                   &mut data) {
        Ok(k) => k,
        Err(e) => return Err(e),
    };

    if kind.is_none() {
        // TODO<codegen>
        return Ok(None);
        return Err(format!("NotImplemented: kind of {:?} is not known",
                           node.children[0]));
    }

    let mut kind = kind.unwrap();
    // TODO<codegen>: resolve parameter types in Env
    if kind == "String" {
        kind = "java.lang.String".to_owned();
    }

    if let Some(myfields) = fields.get(&kind) {
        let field = match node.children[2].to_label() {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        let fidx = myfields.iter().position(|fld| fld.0 == field);
        if fidx.is_none() {
            return Err(format!("could not find matching field for {:?}.{:?} in {:?}",
                               &kind,
                               &field,
                               fields));
        }

        text.push(format!("{} {}, {}", Instr::ADD, Reg::ESI, 32 * (fidx.unwrap() + 1)));
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
        text.push("".to_owned());

        return Ok(Some(myfields[fidx.unwrap()].1.clone()));
    }

    // ArrayTypes
    // TODO<codegen>: do this properly
    if kind == "" && node.children[2].clone().token.lexeme.unwrap() == "length" {
        text.push(format!("{} {}, [{}]", Instr::MOV, Reg::EAX, Reg::ESI));
        text.push("".to_owned());

        // TODO<codegen>: kind is int
        return Ok(None);
    }

    Err(format!("could not find field {:?} in {:?}", kind, fields))
}
