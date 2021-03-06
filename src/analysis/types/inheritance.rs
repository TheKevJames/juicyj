use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::Environment;
use analysis::types::lookup;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref ABSTRACT: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::Abstract, None),
            children: Vec::new(),
        }
    };
}

pub fn verify(env: &Environment,
              current: &ClassOrInterfaceEnvironment,
              visited: &mut Vec<ASTNode>)
              -> Result<ClassOrInterfaceEnvironment, String> {
    if visited.contains(&current.name) {
        return Err("cyclic class hierarchy detected".to_owned());
    }
    visited.push(current.name.clone());

    let mut child = ClassOrInterfaceEnvironment::new(current.name.clone(), current.kind.clone());
    for implemented in &current.implements {
        let found = match lookup::class::in_env(&implemented, &current, &env.kinds) {
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
    for extended in &current.extends {
        let found = match lookup::class::in_env(&extended, &current, &env.kinds) {
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
    match child.apply(&current, &env.kinds) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    if child.kind == ClassOrInterface::CLASS && !child.modifiers.contains(&*ABSTRACT) {
        for method in &child.methods {
            if method.modifiers.contains(&*ABSTRACT) {
                return Err(format!("abstract method {} found in non-abstract class {}",
                                   method.name,
                                   child.name));
            }
        }
    }

    Ok(child)
}
