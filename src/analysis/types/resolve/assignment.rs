use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let lhs =
        match resolve::expression::go(&node.children[0], modifiers, current, kinds, globals) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

    let rhs =
        match resolve::expression::go(&node.children[2], modifiers, current, kinds, globals) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

    lhs.assign(&rhs, current, kinds)
}
