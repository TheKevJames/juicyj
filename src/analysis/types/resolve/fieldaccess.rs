use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::TokenKind;

// TODO: should it be more like this?
// pub fn go(node: &ASTNode,
//           current: &ClassOrInterfaceEnvironment,
//           kinds: &Vec<ClassOrInterfaceEnvironment>,
//           globals: &Vec<VariableEnvironment>)
//           -> Result<Type, String> {
//     let lhs = match resolve::expression::go(&node.children[0], current, kinds, globals) {
//         Ok(t) => t,
//         Err(e) => return Err(e),
//     };

//     // TODO: in_class would save some effort
//     lookup::field::in_env(&lhs.kind.name, &node.children[2], current, kinds)
// }

pub fn go(node: &ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let cls = match node.children[0].token.kind {
        TokenKind::This => current.clone(),
        _ => {
            let lhs = match resolve::expression::go(&node.children[0],
                                                    modifiers,
                                                    current,
                                                    kinds,
                                                    globals) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };

            let mut lhs_kind = lhs.kind.name.clone();
            lhs_kind.flatten();
            if lhs_kind == current.name {
                current.clone()
            } else {
                match lookup::class::in_env(&lhs_kind, current, kinds) {
                    Ok(cls) => cls,
                    Err(e) => return Err(e),
                }
            }
        }
    };

    for field in &cls.fields {
        if field.name == node.children[2] {
            match lookup::class::in_env(&field.kind, &cls, kinds) {
                Ok(cls) => return Ok(Type::new(cls)),
                Err(_) => (),
            }
        }
    }

    Err(format!("could not find field {} in class {}",
                node.children[2],
                cls.name))
}
