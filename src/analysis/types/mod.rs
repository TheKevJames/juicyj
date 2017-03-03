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

            for implemented in &current.implements {
                let found = match check::lookup(&implemented, &current, &env.kinds) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if found.kind == ClassOrInterface::CLASS {
                    return Err(format!("class {} cannot implement class {}", current, found));
                }
            }
        } else if current.kind == ClassOrInterface::INTERFACE {
            for extended in &current.extends {
                let found = match check::lookup(&extended, &current, &env.kinds) {
                    Ok(f) => f,
                    Err(e) => return Err(e),
                };
                if found.kind == ClassOrInterface::CLASS {
                    return Err(format!("interface {} cannot extend class {}", current, found));
                }
            }
        }

        let result = inheritance::verify(env, &current, &mut Vec::new());
        if result.is_err() {
            return result;
        }

        for constructor in &current.constructors {
            for parameter in &constructor.parameters {
                let result = check::verify(parameter.children[0].clone(), &current, &env.kinds);
                if result.is_err() {
                    return result;
                }
            }

            if constructor.body.children.len() == 3 {
                // TODO: are there any relevant globals here?
                let globals = Vec::new();

                let mut child = constructor.body.children[1].clone();
                match body::verify(&mut child, &current, &env.kinds, &globals) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }

        for field in &current.fields {
            let result = check::verify(field.kind.clone(), &current, &env.kinds);
            if result.is_err() {
                return result;
            }
        }

        for method in &current.methods {
            if method.body.is_none() {
                if !method.modifiers.contains(&modifier_abstract) &&
                   !method.modifiers.contains(&modifier_native) {
                    return Err(format!("concrete method {} has no body", method));
                }
            }

            for parameter in &method.parameters {
                let result = check::verify(parameter.children[0].clone(), &current, &env.kinds);
                if result.is_err() {
                    return result;
                }
            }

            let result = check::verify(method.return_type.clone(), &current, &env.kinds);
            if result.is_err() {
                return result;
            }

            if method.body.is_some() && method.clone().body.unwrap().children.len() == 3 {
                // TODO: are there any relevant globals here?
                let globals = Vec::new();

                let mut child = method.clone().body.unwrap().children[1].clone();
                match body::verify(&mut child, &current, &env.kinds, &globals) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }

            if method.modifiers.contains(&modifier_abstract) {
                if method.modifiers.contains(&modifier_final) {
                    return Err(format!("final method {} is abstract", method));
                }

                if method.modifiers.contains(&modifier_static) {
                    return Err(format!("static method {} is abstract", method));
                }
            }
        }
    }

    Ok(())
}
