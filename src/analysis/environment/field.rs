use scanner::ASTNode;

#[derive(Clone,Debug)]
pub struct FieldEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub kind: ASTNode,
    pub name: ASTNode,
}

impl FieldEnvironment {
    pub fn new(name: ASTNode, kind: ASTNode) -> FieldEnvironment {
        FieldEnvironment {
            modifiers: Vec::new(),
            kind: kind,
            name: name,
        }
    }
}

pub fn analyze_constant_declaration(fields: &mut Vec<FieldEnvironment>,
                                    declaration: &ASTNode)
                                    -> Result<(), String> {
    let mut new = FieldEnvironment::new(declaration.children[2].clone(),
                                        declaration.children[1].clone());

    for child in declaration.children[0].clone().children {
        new.modifiers.push(child);
    }

    if new.name.token.lexeme == None {
        // if `name` is an Assignment rather than a Name
        new.name = new.name.children[0].clone();
    }

    for field in fields.clone() {
        if field.name == new.name {
            return Err("field names must be unique".to_owned());
        }
    }

    fields.push(new);
    Ok(())
}

pub fn analyze_field_declaration(fields: &mut Vec<FieldEnvironment>,
                                 declaration: &ASTNode)
                                 -> Result<(), String> {
    let mut new = FieldEnvironment::new(declaration.children[2].clone(),
                                        declaration.children[1].clone());

    for child in declaration.children[0].clone().children {
        new.modifiers.push(child);
    }

    if new.name.token.lexeme == None {
        // if `name` is an Assignment rather than a Name
        new.name = new.name.children[0].clone();
    }

    for field in fields.clone() {
        if field.name == new.name {
            return Err("field names must be unique".to_owned());
        }
    }

    fields.push(new);
    Ok(())
}
