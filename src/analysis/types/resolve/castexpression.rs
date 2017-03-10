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
    match resolve::expression::go(&node.children[1], current, kinds, globals) {
        // CastExpression has 5 children iff it contains a DimExpr
        Ok(ref x) if node.children.len() == 5 => Ok(Type::new(array::create(&x.kind.name))),
        x => x,
    }
}
