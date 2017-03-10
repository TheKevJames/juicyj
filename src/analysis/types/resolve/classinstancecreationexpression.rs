use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>)
          -> Result<Type, String> {
    match lookup::class::in_env(&node.children[0], current, kinds) {
        Ok(cls) => {
            // TODO: ensure a constructor with these arguments exists
            Ok(Type::new(cls))
        },
        Err(e) => Err(e),
    }
}
