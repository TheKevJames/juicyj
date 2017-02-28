use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

use analysis::environment::class::ClassEnvironment;
use analysis::environment::interface::InterfaceEnvironment;

#[derive(Clone,Debug)]
pub struct MethodEnvironment {
    pub modifiers: Vec<ASTNode>,
    pub return_type: ASTNode,
    pub name: ASTNode,
    pub parameters: Vec<ASTNode>,
}

pub fn analyze_abstract_method_declaration(classes: &Vec<ClassEnvironment>,
                                           extends: &Vec<ASTNode>,
                                           interfaces: &Vec<InterfaceEnvironment>,
                                           implements: &Vec<ASTNode>,
                                           methods: &mut Vec<MethodEnvironment>,
                                           header: &ASTNode)
                                           -> Result<(), String> {
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

    let new = MethodEnvironment {
        modifiers: modifiers,
        return_type: return_type,
        name: name,
        parameters: parameters,
    };

    match verify_override(classes, extends, interfaces, implements, &new) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    methods.push(new);

    Ok(())
}

pub fn analyze_method_declaration(classes: &Vec<ClassEnvironment>,
                                  extends: &Vec<ASTNode>,
                                  interfaces: &Vec<InterfaceEnvironment>,
                                  implements: &Vec<ASTNode>,
                                  methods: &mut Vec<MethodEnvironment>,
                                  header: &ASTNode)
                                  -> Result<(), String> {
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

    let new = MethodEnvironment {
        modifiers: modifiers,
        return_type: return_type,
        name: name,
        parameters: parameters,
    };

    match verify_override(classes, extends, interfaces, implements, &new) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    methods.push(new);

    Ok(())
}

fn verify_override(classes: &Vec<ClassEnvironment>,
                   extends: &Vec<ASTNode>,
                   interfaces: &Vec<InterfaceEnvironment>,
                   implements: &Vec<ASTNode>,
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

    for class in classes {
        if !extends.contains(&class.name) {
            continue;
        }

        for method in &class.methods {
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

    for interface in interfaces {
        if !implements.contains(&interface.name) {
            continue;
        }

        for method in &interface.methods {
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
