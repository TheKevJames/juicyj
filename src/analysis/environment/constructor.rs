use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::environment::variable::analyze_block;
use scanner::ASTNode;
use scanner::ASTNodeImport;

#[derive(Clone,Debug)]
pub struct ConstructorEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub name: ASTNode,
    pub parameters: Vec<ASTNode>,
}

pub fn analyze_constructor_declaration(kinds: &Vec<ClassOrInterfaceEnvironment>,
                                       imports: &Vec<ASTNodeImport>,
                                       current: &mut ClassOrInterfaceEnvironment,
                                       modifiers: &ASTNode,
                                       declarator: &ASTNode,
                                       body: &ASTNode)
                                       -> Result<(), String> {
    let mut mods = Vec::new();
    for child in modifiers.clone().children {
        mods.push(child);
    }

    let name = declarator.children[0].clone();

    let mut parameters = Vec::new();
    if declarator.children.len() == 4 {
        let mut params = declarator.children[2].clone();
        let params = match params.clone().token.lexeme {
            Some(ref l) if l == "ParameterList" => params.flatten().clone(),
            _ => params,
        };
        for param in &params.children {
            parameters.push(param.clone());
        }
    }

    for constructor in current.constructors.clone() {
        if constructor.parameters == parameters {
            return Err("constructors must have unique signatures".to_owned());
        }
    }

    if body.children.len() == 3 {
        // TODO: eventually, this should need fields, etc, but since they can be
        // shadowed... meh.
        let globals = Vec::new();

        let mut child = body.children[1].clone();
        match analyze_block(kinds, imports, current, &globals, &mut child) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    current.constructors.push(ConstructorEnvironment {
        modifiers: mods,
        name: name,
        parameters: parameters,
    });

    Ok(())
}
