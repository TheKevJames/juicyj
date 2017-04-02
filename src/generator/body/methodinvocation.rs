use std::collections::HashMap;

use generator::asm::helper::call;
use generator::asm::Reg;
use scanner::ASTNode;

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
    if node.children[0]
           .clone()
           .token
           .lexeme
           .unwrap_or("".to_owned()) != "FullyQualifiedMethod" {
        return Err(format!("got un-qualified method call {:?}", node));
    }

    let mut instance = node.children[0].children[0].clone();
    if instance.clone().token.lexeme.unwrap_or("".to_owned()) == "Name" {
        instance.flatten();
        instance.children.pop();
        instance.children.pop();
    }

    // get instance address
    match statement::go(&instance,
                        class_label,
                        label,
                        fields,
                        &mut text,
                        &mut externs,
                        &mut bss,
                        &mut data) {
        Ok(_) => (),
        Err(_) => {
            let mut qinstance = node.children[0].children[1].clone();
            if qinstance.clone().token.lexeme.unwrap_or("".to_owned()) == "Name" {
                qinstance.flatten();
                qinstance.children.pop();
                qinstance.children.pop();
            }

            match statement::go(&qinstance,
                                class_label,
                                label,
                                fields,
                                &mut text,
                                &mut externs,
                                &mut bss,
                                &mut data) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
    }

    call(&Reg::ESI,
         &node.children[0].children[1],
         &node.children[2],
         class_label,
         label,
         fields,
         &mut text,
         &mut externs,
         &mut bss,
         &mut data)
}
