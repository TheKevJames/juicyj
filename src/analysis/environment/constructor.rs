use scanner::ASTNode;

use analysis::environment::variable::analyze_block;

#[derive(Clone,Debug)]
pub struct ConstructorEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub parameters: Vec<ASTNode>,
}

pub fn analyze_constructor_declaration(constructors: &mut Vec<ConstructorEnvironment>,
                                       header: &ASTNode,
                                       body: &ASTNode)
                                       -> Result<(), String> {
    let mut modifiers = Vec::new();
    for child in header.children[0].clone().children {
        modifiers.push(child);
    }

    let name = header.children[1].clone().children[0].clone();

    let mut parameters = Vec::new();
    if header.children[1].children.len() == 4 {
        let mut param = header.children[1].clone().children[2].clone();
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

    if body.children.len() == 3 {
        // TODO: eventually, this should need fields, etc, but since they can be
        // shadowed... meh.
        let globals = Vec::new();

        let mut child = body.children[1].clone();
        match analyze_block(&globals, &mut child) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    constructors.push(ConstructorEnvironment {
        modifiers: modifiers,
        name: name,
        parameters: parameters,
    });

    Ok(())
}
