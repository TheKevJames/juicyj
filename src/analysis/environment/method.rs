use scanner::ASTNode;

#[derive(Clone,Debug)]
pub struct MethodEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub return_type: ASTNode,
    pub name: ASTNode,
    pub parameters: Vec<ASTNode>,
}

pub fn analyze_abstract_method_declaration(methods: &mut Vec<MethodEnvironment>,
                                           header: &ASTNode)
                                           -> Result<(), String> {
    // TODO: a class with an abstract method must be abstract itself
    //
    // TODO: if non-static, cannot override static
    // TODO: cannot override method with different return type
    // TODO: cannot override permissions with looser permissions
    // TODO: cannot override final method
    let mut modifiers = Vec::new();
    for child in header.children[0].clone().children {
        modifiers.push(child);
    }

    let return_type = header.children[1].clone();
    let name = header.children[2].clone().children[0].clone();

    let mut parameters = Vec::new();
    if header.children[2].children.len() == 4 {
        let mut param = header.children[2].clone().children[2].clone();
        while param.clone().token.lexeme.unwrap_or("".to_owned()) != "Parameter" {
            parameters.push(param.children[2].clone());
            param = param.children[0].clone();
        }
        parameters.push(param.clone());
    }

    for method in methods.clone() {
        if method.name == name && method.parameters == parameters {
            return Err("methods must have unique signatures".to_owned());
        }
    }

    methods.push(MethodEnvironment {
        modifiers: modifiers,
        return_type: return_type,
        name: name,
        parameters: parameters,
    });

    Ok(())
}

pub fn analyze_method_declaration(methods: &mut Vec<MethodEnvironment>,
                                  header: &ASTNode)
                                  -> Result<(), String> {
    // TODO: if non-static, cannot override static
    // TODO: cannot override method with different return type
    // TODO: cannot override permissions with looser permissions
    // TODO: cannot override final method
    let mut modifiers = Vec::new();
    for child in header.children[0].clone().children {
        modifiers.push(child);
    }

    let return_type = header.children[1].clone();
    let name = header.children[2].clone().children[0].clone();

    let mut parameters = Vec::new();
    if header.children[2].children.len() == 4 {
        let mut param = header.children[2].clone().children[2].clone();
        while param.clone().token.lexeme.unwrap_or("".to_owned()) != "Parameter" {
            parameters.push(param.children[2].clone());
            param = param.children[0].clone();
        }
        parameters.push(param.clone());
    }

    for method in methods.clone() {
        if method.name == name && method.parameters == parameters {
            return Err("methods must have unique signatures".to_owned());
        }
    }

    methods.push(MethodEnvironment {
        modifiers: modifiers,
        return_type: return_type,
        name: name,
        parameters: parameters,
    });

    Ok(())
}
