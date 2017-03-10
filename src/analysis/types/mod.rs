mod body;
mod inheritance;
pub mod lookup;
pub mod verify;

use analysis::environment::ClassOrInterface;
use analysis::environment::Environment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

fn rebuild_env(env: &Environment) -> Result<Environment, String> {
    let mut new = Environment { kinds: Vec::new() };

    for current in &env.kinds {
        match inheritance::verify(env, &current, &mut Vec::new()) {
            Ok(inherit) => new.kinds.push(inherit),
            Err(e) => return Err(e),
        };
    }

    Ok(new)
}

fn verify_env_inheritable(env: &Environment) -> Result<(), String> {
    let modifier_final = ASTNode {
        token: Token::new(TokenKind::Final, None),
        children: Vec::new(),
    };
    let object = ASTNode {
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
        match verify::prefixes::package(&current.name, &current, &env.kinds) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        if current.kind == ClassOrInterface::CLASS {
            for extended in &current.extends {
                let found = match lookup::class::in_env(&extended, &current, &env.kinds) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if found.kind == ClassOrInterface::CLASS &&
                   found.modifiers.contains(&modifier_final) {
                    return Err(format!("class {} cannot extend final class {}", current, found));
                } else if found.kind == ClassOrInterface::INTERFACE {
                    return Err(format!("class {} cannot extend interface {}", current, found));
                }

                if current.name != object {
                    let mut zero_argument_parent = false;
                    for constructor in found.constructors {
                        if constructor.parameters.is_empty() {
                            zero_argument_parent = true;
                            break;
                        }
                    }
                    if !zero_argument_parent {
                        return Err(format!("class {} has missing zero-argument constructor in {}",
                                           current.name,
                                           found.name));
                    }
                }
            }

            let mut resolved = Vec::new();
            for implemented in &current.implements {
                let found = match lookup::class::in_env(&implemented, &current, &env.kinds) {
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
                let found = match lookup::class::in_env(&extended, &current, &env.kinds) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if resolved.contains(&found.name) {
                    return Err(format!("type {} must not be repeated in interface extends",
                                       found.name));
                }
                if found.kind == ClassOrInterface::CLASS && found.name != object {
                    return Err(format!("interface {} cannot extend class {}", current, found));
                }
                resolved.push(found.name);
            }
        }
    }

    Ok(())
}

fn verify_env(env: &Environment) -> Result<(), String> {
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

    for current in &env.kinds {
        for constructor in &current.constructors {
            let mut params = Vec::new();
            for parameter in &constructor.parameters {
                if params.contains(&parameter.name) {
                    return Err(format!("constructor has multiple parameters with same name {}",
                                       parameter.name));
                }
                params.push(parameter.name.clone());

                let result = verify::prefixes::canonical(&parameter.kind, &current, &env.kinds);
                if result.is_err() {
                    return result;
                }
            }

            let mut globals = Vec::new();
            for param in &constructor.parameters {
                globals.push(param.clone());
            }
            for field in &current.fields {
                globals.push(field.to_variable());
            }
            match body::verifybody(&mut constructor.body.clone(),
                                   &current,
                                   &env.kinds,
                                   &globals) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            if &constructor.name != current.name.children.last().unwrap() {
                return Err(format!("constructor {} does not share class name {}",
                                   constructor.name,
                                   current.name));
            }
        }

        // TODO: non-static fields must be initialized in order and without implicit `this`:
        // http://titanium.cs.berkeley.edu/doc/java-langspec-1.0/8.doc.html#38013
        for field in &current.fields {
            let result = verify::prefixes::canonical(&field.kind, &current, &env.kinds);
            if result.is_err() {
                return result;
            }

            // TODO: static fields can not use implicit `this`
        }

        for method in &current.methods {
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

            let result = verify::prefixes::canonical(&method.return_type, &current, &env.kinds);
            if result.is_err() {
                return result;
            }

            if method.body.is_some() {
                let mut globals = Vec::new();
                for param in &method.parameters {
                    globals.push(param.clone());
                }
                for field in &current.fields {
                    globals.push(field.to_variable());
                }
                match body::verifybody(&mut method.clone().body.unwrap().clone(),
                                       &current,
                                       &env.kinds,
                                       &globals) {
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

            // TODO: static methods can not use implicit `this`
        }
    }

    Ok(())
}

pub fn verify(env: Environment) -> Result<(), String> {
    match verify_env_inheritable(&env) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let env = match rebuild_env(&env) {
        Ok(e) => e,
        Err(e) => return Err(e),
    };

    verify_env(&env)
}
