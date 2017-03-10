mod inheritance;
pub mod lookup;
mod obj;
mod resolve;
pub mod verify;

use analysis::environment::ClassOrInterface;
use analysis::environment::Environment;
use analysis::types::obj::Type;
use analysis::types::verify::method::statement;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref ABSTRACT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Abstract, None), children: Vec::new() }
    };
    static ref FINAL: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Final, None), children: Vec::new() }
    };
    static ref GETCLASS: ASTNode = ASTNode {
        token: Token::new(TokenKind::NonTerminal, Some("Name")),
        children: vec![ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("getClass")),
                           children: Vec::new(),
                       }],
    };
    static ref NATIVE: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Native, None), children: Vec::new() }
    };
    static ref NULL: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Null, None), children: Vec::new() }
    };
    static ref OBJECT: ASTNode = ASTNode {
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
    static ref STATIC: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Static, None), children: Vec::new() }
    };
}

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
                if found.kind == ClassOrInterface::CLASS && found.modifiers.contains(&*FINAL) {
                    return Err(format!("class {} cannot extend final class {}", current, found));
                } else if found.kind == ClassOrInterface::INTERFACE {
                    return Err(format!("class {} cannot extend interface {}", current, found));
                }

                if current.name != *OBJECT {
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
                if found.kind == ClassOrInterface::CLASS && found.name != *OBJECT {
                    return Err(format!("interface {} cannot extend class {}", current, found));
                }
                resolved.push(found.name);
            }
        }
    }

    Ok(())
}

fn verify_env(env: &Environment) -> Result<(), String> {
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

            if &constructor.name != current.name.children.last().unwrap() {
                return Err(format!("constructor {} does not share class name {}",
                                   constructor.name,
                                   current.name));
            }

            let globals = constructor.parameters.clone();
            let return_types = match statement::block(&mut constructor.body.clone(),
                                                      &constructor.modifiers,
                                                      &current,
                                                      &env.kinds,
                                                      &globals) {
                Ok(rts) => rts,
                Err(e) => return Err(e),
            };

            let constructor_return_type = Type::new(current.clone());
            for return_type in return_types {
                if return_type.kind.name == *NULL {
                    continue;
                }

                match constructor_return_type.assign(&return_type, current, &env.kinds) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(format!("constructor {} has invalid return type\nerror: {:?}",
                                           current.name,
                                           e))
                    }
                }
            }
        }

        let mut current_builder = current.clone();
        current_builder.fields = Vec::new();
        for field in &current.fields {
            let result = verify::prefixes::canonical(&field.kind, &current_builder, &env.kinds);
            if result.is_err() {
                return result;
            }

            if field.value.is_none() {
                continue;
            }

            // TODO: allow qualified names to be resolved to future fields
            // TODO: static fields can not use implicit `this`
            let rexpr = field.clone().value.unwrap();
            let rvalue = match resolve::expression::go(&rexpr,
                                                       &field.modifiers,
                                                       &current_builder,
                                                       &env.kinds,
                                                       &Vec::new()) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            let lvalue = match lookup::class::in_env(&field.kind, &current_builder, &env.kinds) {
                Ok(c) => Type::new(c),
                Err(e) => return Err(e),
            };

            match lvalue.assign(&rvalue, &current_builder, &env.kinds) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            current_builder.fields.push(field.clone());
        }

        for method in &current.methods {
            if method.body.is_none() {
                if !method.modifiers.contains(&*ABSTRACT) && !method.modifiers.contains(&*NATIVE) {
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

            if method.modifiers.contains(&*ABSTRACT) {
                if method.modifiers.contains(&*FINAL) {
                    if method.name != *GETCLASS {
                        return Err(format!("final method {} is abstract", method));
                    }
                }

                if method.modifiers.contains(&*STATIC) {
                    return Err(format!("static method {} is abstract", method));
                }
            }

            // TODO: static methods can not use implicit `this`
            if method.body.is_some() {
                let globals = method.parameters.clone();
                let return_types =
                    match statement::block(&mut method.clone().body.unwrap().clone(),
                                           &method.modifiers,
                                           &current,
                                           &env.kinds,
                                           &globals) {
                        Ok(rts) => rts,
                        Err(e) => return Err(e),
                    };

                let method_return_type =
                    match lookup::class::in_env(&method.return_type, current, &env.kinds) {
                        Ok(rt) => Type::new(rt),
                        Err(e) => return Err(e),
                    };

                for return_type in return_types {
                    if return_type.kind.name == *NULL {
                        continue;
                    }

                    match method_return_type.assign(&return_type, current, &env.kinds) {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(format!("method {} has invalid return type\nerror: {:?}",
                                               method.name,
                                               e))
                        }
                    }
                }
            }

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
