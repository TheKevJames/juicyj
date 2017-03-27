mod inheritance;
pub mod lookup;
mod obj;
mod resolve;
pub mod verify;

use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
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
    static ref DOT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Dot, None), children: Vec::new() }
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
    static ref VOID: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Void, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
}

fn rebuild_env(mut env: &mut Environment) -> Result<(), String> {
    let old = env.clone();

    env.kinds = Vec::new();
    for current in &old.kinds {
        match inheritance::verify(&old, &current, &mut Vec::new()) {
            Ok(inherit) => env.kinds.push(inherit),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

fn verify_env_inheritable(mut env: &mut Environment) -> Result<(), String> {
    let kinds = env.kinds.clone();

    for mut current in &mut env.kinds {
        match verify::prefixes::package(&current.name, &current, &kinds) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        if current.kind == ClassOrInterface::CLASS {
            for extended in &current.extends {
                let found = match lookup::class::in_env(&extended, &current, &kinds) {
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
                    for parent_constructor in found.constructors {
                        if parent_constructor.parameters.is_empty() {
                            zero_argument_parent = true;

                            let mut fully_qualified = found.name.clone();
                            fully_qualified.flatten();
                            fully_qualified.children.push(DOT.clone());
                            fully_qualified.children.push(parent_constructor.name.clone());
                            for mut constructor in &mut current.constructors {
                                constructor.parent = Some(fully_qualified.clone());
                            }

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
                let found = match lookup::class::in_env(&implemented, &current, &kinds) {
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
                let found = match lookup::class::in_env(&extended, &current, &kinds) {
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

fn verify_env(mut env: &mut Environment) -> Result<(), String> {
    let kinds = env.kinds.clone();

    for mut current in &mut env.kinds {
        for constructor in &current.constructors {
            let mut params = Vec::new();
            for parameter in &constructor.parameters {
                if params.contains(&parameter.name) {
                    return Err(format!("constructor has multiple parameters with same name {}",
                                       parameter.name));
                }
                params.push(parameter.name.clone());

                let result = verify::prefixes::canonical(&parameter.kind, &current, &kinds);
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
            let return_types =
                match statement::block(&mut constructor.clone().body.unwrap().clone(),
                                       &constructor.modifiers,
                                       &current,
                                       &kinds,
                                       &globals) {
                    Ok(rts) => rts,
                    Err(e) => return Err(e),
                };

            let constructor_return_type = Type::new(current.clone());
            for return_type in &return_types {
                match constructor_return_type.assign(&return_type, current, &kinds) {
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

        let mut env_builder = Vec::new();
        for cls in &kinds {
            if cls.name != current.name {
                env_builder.push(cls.clone());
            }
        }

        for mut field in &mut current.fields {
            env_builder.push(current_builder.clone());

            let result = verify::prefixes::canonical(&field.kind, &current_builder, &env_builder);
            if result.is_err() {
                return result;
            }

            if field.value.is_none() {
                continue;
            }

            // TODO: allow qualified names to be resolved to future fields
            let mut rexpr = field.clone().value.unwrap();
            let rvalue = match resolve::expression::go(&mut rexpr,
                                                       &field.modifiers,
                                                       &current_builder,
                                                       &env_builder,
                                                       &mut Vec::new()) {
                Ok(t) => {
                    field.value = Some(rexpr);
                    t
                }
                Err(e) => return Err(e),
            };

            let lvalue = match lookup::class::in_env(&field.kind, &current_builder, &env_builder) {
                Ok(c) => Type::new(c),
                Err(e) => return Err(e),
            };

            match lvalue.assign(&rvalue, &current_builder, &env_builder) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            current_builder.fields.push(field.clone());

            env_builder.pop();
        }

        let curr = current.clone();
        for mut method in &mut current.methods {
            if method.body.is_none() {
                if !method.modifiers.contains(&*ABSTRACT) && !method.modifiers.contains(&*NATIVE) {
                    return Err(format!("concrete method {} has no body", method));
                }
            }

            let globals = method.parameters.clone();

            let mut params = Vec::new();
            for mut parameter in &mut method.parameters {
                if params.contains(&parameter.name) {
                    return Err(format!("method has multiple parameters with same name {}",
                                       parameter.name));
                }
                params.push(parameter.name.clone());

                match resolve::expression::go(&mut parameter.kind,
                                              &method.modifiers,
                                              &curr,
                                              &kinds,
                                              &mut Vec::new()) {
                    Ok(t) => parameter.kind = t.kind.name,
                    Err(e) => return Err(e),
                }
            }

            let result = verify::prefixes::canonical(&method.return_type, &curr, &kinds);
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

            if method.body.is_some() {
                let mut body = method.clone().body.unwrap().clone();
                let return_types =
                    match statement::block(&mut body, &method.modifiers, &curr, &kinds, &globals) {
                        Ok(rts) => {
                            method.body = Some(body);
                            rts
                        }
                        Err(e) => return Err(e),
                    };

                let method_return_type =
                    match lookup::class::in_env(&method.return_type, &curr, &kinds) {
                        Ok(rt) => Type::new(rt),
                        Err(e) => return Err(e),
                    };

                for return_type in &return_types {
                    match method_return_type.assign(&return_type, &curr, &kinds) {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(format!("{} method {} has invalid return type\nerror: {:?}",
                                               curr.name,
                                               method.name,
                                               e))
                        }
                    }
                }

                if return_types.is_empty() && method_return_type != *VOID {
                    return Err(format!("non-void {} method {} has no return type",
                                       curr.name,
                                       method.name));
                }
            }

        }
    }

    Ok(())
}

pub fn verify(mut env: &mut Environment) -> Result<(), String> {
    match verify_env_inheritable(&mut env) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match rebuild_env(&mut env) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    verify_env(&mut env)
}
