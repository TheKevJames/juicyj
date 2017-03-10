use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::TokenKind;

pub fn go(node: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let lookup = match node.children.len() {
        3 | 4 => {
            let mut name = node.children[0].clone();
            let lhs = match resolve::expression::go(&name, current, kinds, globals) {
                Ok(l) => {
                    let mut lhs_name = l.kind.name.clone();
                    lhs_name.flatten();
                    lookup::class::in_env(&lhs_name, current, kinds).ok()
                }
                Err(_) => None,
            };

            if lhs.is_some() {
                Some((lhs.unwrap(), node.children[2].clone()))
            } else {
                name.flatten();

                let method = name.children.pop().unwrap();
                name.children.pop();

                // TODO: if name.chlidren had 3+ anyway
                // TODO: other.x, etc?
                // TODO: just fucking write a globals.lookup
                for var in globals {
                    // this.x
                    if var.name == name {
                        name = var.kind.clone();
                        break;
                    }

                    // x
                    if var.name.children.len() == 3 &&
                       var.name.children[0].clone().token.kind == TokenKind::This &&
                       var.name.children[2] == name {
                        // let result = lookup::class::in_env(&var.kind, current, kinds);
                        // if result.is_err() {
                        //     continue;
                        // }
                        // return Some((result.ok(), method.clone()))
                        name = var.kind.clone();
                        break;
                    }
                }

                match lookup::class::in_env(&name, current, kinds) {
                    Ok(cls) => Some((cls, method.clone())),
                    // assume we don't need to resolve
                    Err(_) => Some((current.clone(), node.children[0].clone())),
                }
            }
        }
        5 | 6 => {
            let name = node.children[0].clone();
            let lhs = match resolve::expression::go(&name, current, kinds, globals) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            let mut lhs_name = lhs.kind.name.clone();
            lhs_name.flatten();
            match lookup::class::in_env(&lhs_name, current, kinds) {
                Ok(cls) => Some((cls, node.children[2].clone())),
                Err(e) => return Err(e),
            }
        }
        _ => None,
    };

    if let Some((cls, method)) = lookup {
        let mut method = method;
        method.flatten();

        let mut found = None;
        for cls_method in &cls.methods {
            if cls_method.name != method {
                continue;
            }

            found = Some(cls_method);
            break;
        }

        if found.is_none() {
            return Err(format!("could not find method {} on class {}", method, cls.name));
        }

        return Ok(Type::new(ClassOrInterfaceEnvironment::new(found.unwrap()
                                                                 .return_type
                                                                 .clone(),
                                                             ClassOrInterface::CLASS)));
    }

    Err(format!("malformated MethodInvocation {}", node))
}
