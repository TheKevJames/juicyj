use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::obj::Type;
use analysis::types::resolve;
use analysis::types::verify;
use scanner::ASTNode;

pub fn go(mut node: &mut ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let lhs =
        match resolve::expression::go(&mut node.children[0], modifiers, current, kinds, globals) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

    let rhs =
        match resolve::expression::go(&mut node.children[2], modifiers, current, kinds, globals) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

    match verify::variable::initialized(&node.children[2], current, globals) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    for (idx, var) in globals.clone().iter().enumerate() {
        if &var.name != &node.children[0] {
            continue;
        }

        globals[idx].initialized = true;
        break;
    }

    lhs.assign(&rhs, current, kinds)
}
