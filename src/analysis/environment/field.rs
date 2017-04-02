use analysis::environment::ClassOrInterfaceEnvironment;
use scanner::ASTNode;

#[derive(Clone,Debug)]
pub struct FieldEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub kind: ASTNode,
    pub name: ASTNode,
    pub value: Option<ASTNode>,
}

impl FieldEnvironment {
    pub fn new(name: ASTNode, kind: ASTNode) -> FieldEnvironment {
        let mut kind = kind.clone();
        kind.flatten();

        FieldEnvironment {
            modifiers: Vec::new(),
            kind: match kind.clone().token.lexeme {
                Some(ref l) if l == "ArrayType" => {
                    // Remove Dim or DimExpr
                    kind.children.truncate(1);
                    // Flatten Name
                    kind.children[0].flatten();
                    kind
                }
                _ => kind,
            },
            name: name, // TODO: maybe flatten this?
            value: None,
        }
    }
}

pub fn analyze_constant_declaration(current: &mut ClassOrInterfaceEnvironment,
                                    declaration: &ASTNode)
                                    -> Result<(), String> {
    let mut new = FieldEnvironment::new(declaration.children[2].clone(),
                                        declaration.children[1].clone());

    for child in declaration.children[0].clone().children {
        new.modifiers.push(child);
    }

    let mut kind = declaration.children[4].clone().flatten().clone();
    new.value = Some(match kind.clone().token.lexeme {
                         Some(ref l) if l == "ArrayType" => {
        // Remove Dim or DimExpr
        kind.children.truncate(1);
        // Flatten Name
        kind.children[0].flatten();
        kind
    }
                         _ => kind,
                     });

    for field in current.fields.clone() {
        if field.name == new.name {
            return Err("field names must be unique".to_owned());
        }
    }

    current.fields.push(new);
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

    for field in fields.clone() {
        if field.name == new.name {
            return Err("field names must be unique".to_owned());
        }
    }

    fields.push(new);
    Ok(())
}
