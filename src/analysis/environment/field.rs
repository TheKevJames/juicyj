use scanner::ASTNode;

#[derive(Clone,Debug)]
pub struct FieldEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub kind: ASTNode,
    pub name: ASTNode,
}

pub fn analyze_constant_declaration(fields: &mut Vec<FieldEnvironment>,
                                    declaration: &ASTNode)
                                    -> Result<(), String> {
    let mut modifiers = Vec::new();
    for kid in declaration.children[0].clone().children {
        modifiers.push(kid);
    }

    let kind = declaration.children[1].clone();
    let name = declaration.children[2].clone().children[0].clone();

    for field in fields.clone() {
        if field.name == name {
            return Err("field names must be unique".to_owned());
        }
    }

    fields.push(FieldEnvironment {
        modifiers: modifiers,
        kind: kind,
        name: name,
    });

    Ok(())
}

pub fn analyze_field_declaration(fields: &mut Vec<FieldEnvironment>,
                                 declaration: &ASTNode)
                                 -> Result<(), String> {
    let mut modifiers = Vec::new();
    for child in declaration.children[0].clone().children {
        modifiers.push(child);
    }

    let kind = declaration.children[1].clone();
    let name = declaration.children[2].clone().children[0].clone();

    for field in fields.clone() {
        if field.name == name {
            return Err("field names must be unique".to_owned());
        }
    }

    fields.push(FieldEnvironment {
        modifiers: modifiers,
        kind: kind,
        name: name,
    });

    Ok(())
}
