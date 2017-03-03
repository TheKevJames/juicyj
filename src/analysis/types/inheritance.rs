use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::Environment;
use analysis::types::check;
use scanner::ASTNode;

pub fn verify(env: &Environment,
              current: &ClassOrInterfaceEnvironment,
              visited: &mut Vec<ASTNode>)
              -> Result<(), String> {
    if visited.contains(&current.name) {
        return Err("cyclic class hierarchy detected".to_owned());
    }
    visited.push(current.name.clone());

    for extended in &current.extends {
        let found = match check::lookup(&extended, &current, &env.kinds) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let result = verify(env, &found, &mut visited.clone());
        if result.is_err() {
            return result;
        }
    }

    Ok(())

    // TODO: ensure non-abstract class does not contain un-overriden abstract
    // methods or define new ones

    // TODO: foreach method analyze override:
    // match verify_override(env.kinds, current, &method) {
    //     Ok(_) => (),
    //     Err(e) => return Err(e),
    // }
}
