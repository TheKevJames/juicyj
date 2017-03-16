use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;

pub fn go(node: &ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let array =
        match resolve::expression::go(&node.children[0], modifiers, current, kinds, globals) {
            Ok(a) => a,
            Err(e) => return Err(e),
        };

    if array.kind.name.clone().token.lexeme.unwrap_or("".to_owned()) != "ArrayType" {
        return Err(format!("got invalid array type {:?}", array));
    }

    match resolve::expression::go(&node.children[2], modifiers, current, kinds, globals) {
        Ok(ref idx) if idx.is_coercible_to_int() => (),
        Ok(idx) => return Err(format!("got invalid index type {:?}", idx.kind.name)),
        Err(e) => return Err(e),
    }

    resolve::expression::go(&array.kind.name.children[0],
                            modifiers,
                            current,
                            kinds,
                            globals)
}
