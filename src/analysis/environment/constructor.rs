use scanner::ASTNode;

#[derive(Clone,Debug)]
pub struct ConstructorEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub parameters: Vec<ASTNode>,
}

pub fn analyze_constructor_declaration(constructors: &mut Vec<ConstructorEnvironment>,
                                       declaration: &ASTNode)
                                       -> Result<(), String> {
    let mut modifiers = Vec::new();
    for child in declaration.children[0].clone().children {
        modifiers.push(child);
    }

    let name = declaration.children[1].clone().children[0].clone();

    let mut parameters = Vec::new();
    if declaration.children[1].children.len() == 4 {
        let mut param = declaration.children[1].clone().children[2].clone();
        while param.clone().token.lexeme.unwrap_or("".to_owned()) != "Parameter" {
            parameters.push(param.children[2].clone());
            param = param.children[0].clone();
        }
        parameters.push(param.clone());
    }

    for constructor in constructors.clone() {
        if constructor.parameters == parameters {
            return Err("constructors must have unique signatures".to_owned());
        }
    }

    constructors.push(ConstructorEnvironment {
        modifiers: modifiers,
        name: name,
        parameters: parameters,
    });

    Ok(())
}
