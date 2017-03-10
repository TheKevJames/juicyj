use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup::array;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    match resolve::expression::go(&node.children[0], current, kinds, globals) {
        Ok(x) => Ok(Type::new(array::create(&x.kind.name))),
        Err(e) => return Err(e),
    }
}
