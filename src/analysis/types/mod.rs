mod body;
mod check;
mod inheritance;

use analysis::environment::ClassOrInterface;
use analysis::environment::Environment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

pub fn verify(env: &Environment) -> Result<(), String> {
    let modifier_abstract = ASTNode {
        token: Token::new(TokenKind::Abstract, None),
        children: Vec::new(),
    };
    let modifier_final = ASTNode {
        token: Token::new(TokenKind::Final, None),
        children: Vec::new(),
    };
    let modifier_native = ASTNode {
        token: Token::new(TokenKind::Native, None),
        children: Vec::new(),
    };
    let modifier_static = ASTNode {
        token: Token::new(TokenKind::Static, None),
        children: Vec::new(),
    };
    let object_name = ASTNode {
        token: Token::new(TokenKind::NonTerminal, Some("Name")),
        children: vec![ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("java")),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Dot, None),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("lang")),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Dot, None),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("Object")),
                           children: Vec::new(),
                       }],
    };

    for current in &env.kinds {
        if current.kind == ClassOrInterface::CLASS {
            for extended in &current.extends {
                let found = match check::lookup(&extended, &current, &env.kinds) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if found.kind == ClassOrInterface::CLASS &&
                   found.modifiers.contains(&modifier_final) {
                    return Err(format!("class {} cannot extend final class {}", current, found));
                } else if found.kind == ClassOrInterface::INTERFACE {
                    return Err(format!("class {} cannot extend interface {}", current, found));
                }
            }

            let mut resolved = Vec::new();
            for implemented in &current.implements {
                let found = match check::lookup(&implemented, &current, &env.kinds) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if resolved.contains(&found.name) {
                    return Err(format!("interface {} must not be repeated in class implements",
                                       found.name));
                }
                if found.kind == ClassOrInterface::CLASS {
                    return Err(format!("class {} cannot implement class {}", current, found));
                }
                resolved.push(found.name);
            }
        } else if current.kind == ClassOrInterface::INTERFACE {
            let mut resolved = Vec::new();
            for extended in &current.extends {
                let found = match check::lookup(&extended, &current, &env.kinds) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if resolved.contains(&found.name) {
                    return Err(format!("type {} must not be repeated in interface extends",
                                       found.name));
                }
                if found.kind == ClassOrInterface::CLASS && found.name != object_name {
                    return Err(format!("interface {} cannot extend class {}", current, found));
                }
                resolved.push(found.name);
            }
        }

        let inherited = match inheritance::verify(env, &current, &mut Vec::new()) {
            Ok(inherit) => inherit,
            Err(e) => return Err(e),
        };

        for constructor in &inherited.constructors {
            let mut params = Vec::new();
            for parameter in &constructor.parameters {
                if params.contains(&parameter.name) {
                    return Err(format!("constructor has multiple parameters with same name {}",
                                       parameter.name));
                }
                params.push(parameter.name.clone());

                let result = check::verify(parameter.kind.clone(), &inherited, &env.kinds);
                if result.is_err() {
                    return result;
                }
            }

            match body::verify(&mut constructor.body.clone(),
                               &inherited,
                               &env.kinds,
                               &constructor.parameters.clone()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        for field in &inherited.fields {
            let result = check::verify(field.kind.clone(), &inherited, &env.kinds);
            if result.is_err() {
                return result;
            }
        }

        for method in &inherited.methods {
            if method.body.is_none() {
                if !method.modifiers.contains(&modifier_abstract) &&
                   !method.modifiers.contains(&modifier_native) {
                    return Err(format!("concrete method {} has no body", method));
                }
            }

            let mut params = Vec::new();
            for parameter in &method.parameters {
                if params.contains(&parameter.name) {
                    return Err(format!("method has multiple parameters with same name {}",
                                       parameter.name));
                }
                params.push(parameter.name.clone());
            }

            let result = check::verify(method.return_type.clone(), &inherited, &env.kinds);
            if result.is_err() {
                return result;
            }

            if method.body.is_some() {
                match body::verify(&mut method.clone().body.unwrap().clone(),
                                   &inherited,
                                   &env.kinds,
                                   &method.parameters.clone()) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }

            if method.modifiers.contains(&modifier_abstract) {
                if method.modifiers.contains(&modifier_final) {
                    let node_get_class = ASTNode {
                        token: Token::new(TokenKind::NonTerminal, Some("Name")),
                        children: vec![ASTNode {
                                           token: Token::new(TokenKind::Identifier,
                                                             Some("getClass")),
                                           children: Vec::new(),
                                       }],
                    };
                    if method.name != node_get_class {
                        return Err(format!("final method {} is abstract", method));
                    }
                }

                if method.modifiers.contains(&modifier_static) {
                    return Err(format!("static method {} is abstract", method));
                }
            }
        }
    }

    Ok(())
}
