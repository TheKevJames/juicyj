use analysis::environment::classorinterface::ClassOrInterfaceEnvironment;
use analysis::environment::variable::analyze_block;
use scanner::ASTNode;
use scanner::ASTNodeImport;
use scanner::Token;
use scanner::TokenKind;

#[derive(Clone,Debug)]
pub struct MethodEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub return_type: ASTNode,
    pub name: ASTNode,
    pub parameters: Vec<ASTNode>,
}

pub fn analyze_abstract_method_declaration(kinds: &Vec<ClassOrInterfaceEnvironment>,
                                           current: &mut ClassOrInterfaceEnvironment,
                                           header: &ASTNode)
                                           -> Result<(), String> {
    let declarator = header.children[2].clone();

    let mut modifiers = Vec::new();
    for child in header.children[0].clone().children {
        modifiers.push(child);
    }

    // TODO: analyze
    let return_type = header.children[1].clone();
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

    for method in current.methods.clone() {
        if method.name == name && method.parameters == parameters {
            return Err("methods must have unique signatures".to_owned());
        }
    }

    let new = MethodEnvironment {
        modifiers: modifiers,
        return_type: return_type,
        name: name,
        parameters: parameters,
    };

    match verify_override(kinds, current, &new) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    current.methods.push(new);
    Ok(())
}

pub fn analyze_method_declaration(kinds: &Vec<ClassOrInterfaceEnvironment>,
                                  imports: &Vec<ASTNodeImport>,
                                  current: &mut ClassOrInterfaceEnvironment,
                                  header: &ASTNode,
                                  body: &ASTNode)
                                  -> Result<(), String> {
    let declarator = header.children[2].clone();

    let mut modifiers = Vec::new();
    for child in header.children[0].clone().children {
        modifiers.push(child);
    }

    // TODO: lookup
    let return_type = header.children[1].clone();
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

    for method in current.methods.clone() {
        if method.name == name && method.parameters == parameters {
            return Err("methods must have unique signatures".to_owned());
        }
    }

    let new = MethodEnvironment {
        modifiers: modifiers,
        return_type: return_type,
        name: name,
        parameters: parameters,
    };

    match verify_override(kinds, current, &new) {
        Ok(_) => (),
        Err(e) => return Err(e),
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

    current.methods.push(new);
    Ok(())
}

// TODO: verify classes do not contain duplicate method signatures solely
// through extending
fn verify_override(kinds: &Vec<ClassOrInterfaceEnvironment>,
                   current: &ClassOrInterfaceEnvironment,
                   new: &MethodEnvironment)
                   -> Result<(), String> {
    let fnode = ASTNode {
        token: Token::new(TokenKind::Final, None),
        children: Vec::new(),
    };
    let snode = ASTNode {
        token: Token::new(TokenKind::Static, None),
        children: Vec::new(),
    };
    let public = ASTNode {
        token: Token::new(TokenKind::Public, None),
        children: Vec::new(),
    };
    let protected = ASTNode {
        token: Token::new(TokenKind::Protected, None),
        children: Vec::new(),
    };
    let private = ASTNode {
        token: Token::new(TokenKind::Private, None),
        children: Vec::new(),
    };

    for class_or_interface in kinds {
        if !current.extends.contains(&class_or_interface.name) {
            continue;
        }

        for method in &class_or_interface.methods {
            if method.name == new.name && method.parameters == new.parameters {
                if method.return_type != new.return_type {
                    return Err("cannot override method with different return type".to_owned());
                }

                if method.modifiers.contains(&fnode) {
                    return Err("methods cannot override final methods".to_owned());
                }

                if method.modifiers.contains(&public) &&
                   (new.modifiers.contains(&protected) || new.modifiers.contains(&private)) {
                    return Err("methods cannot be overriden with weaker access controls"
                        .to_owned());
                } else if method.modifiers.contains(&protected) &&
                          new.modifiers.contains(&private) {
                    return Err("methods cannot be overriden with weaker access controls"
                        .to_owned());
                }

                if method.modifiers.contains(&snode) && !new.modifiers.contains(&snode) {
                    return Err("cannot override static method with non-static method".to_owned());
                }
            }
        }
    }

    Ok(())
}
