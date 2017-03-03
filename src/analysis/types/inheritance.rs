use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::Environment;
use analysis::types::check;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

pub fn verify(env: &Environment,
              current: &ClassOrInterfaceEnvironment,
              visited: &mut Vec<ASTNode>)
              -> Result<ClassOrInterfaceEnvironment, String> {
    if visited.contains(&current.name) {
        return Err("cyclic class hierarchy detected".to_owned());
    }
    visited.push(current.name.clone());

    let mut child = ClassOrInterfaceEnvironment::new(current.name.clone(), current.kind.clone());
    for extended in &current.extends {
        let found = match check::lookup(&extended, &current, &env.kinds) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        match verify(env, &found, &mut visited.clone()) {
            Ok(parent) => {
                match child.inherit(&parent, &env.kinds) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    }
    for implemented in &current.implements {
        let found = match check::lookup(&implemented, &current, &env.kinds) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        match verify(env, &found, &mut Vec::new()) {
            Ok(parent) => {
                match child.inherit(&parent, &env.kinds) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    }
    match child.apply(&current, &env.kinds) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    let modifier_abstract = ASTNode {
        token: Token::new(TokenKind::Abstract, None),
        children: Vec::new(),
    };
    if child.kind == ClassOrInterface::CLASS && !child.modifiers.contains(&modifier_abstract) {
        for method in &child.methods {
            if method.modifiers.contains(&modifier_abstract) {
                return Err(format!("abstract methods found in non-abstract class {}",
                                   child.name));
            }
        }
    }

    Ok(child)
}
