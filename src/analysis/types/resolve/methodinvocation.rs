use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::MethodEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref ARGUMENTLIST: ASTNode = {
        ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("ArgumentList")),
            children: Vec::new()
        }
    };
    static ref NAME: ASTNode = {
        ASTNode { token: Token::new(TokenKind::NonTerminal, Some("Name")), children: Vec::new() }
    };
    static ref PROTECTED: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Protected, None), children: Vec::new() }
    };
    static ref STATIC: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Static, None), children: Vec::new() }
    };
}

fn get_args(node: &ASTNode,
            modifiers: &Vec<ASTNode>,
            current: &ClassOrInterfaceEnvironment,
            kinds: &Vec<ClassOrInterfaceEnvironment>,
            globals: &mut Vec<VariableEnvironment>)
            -> Result<Vec<Type>, String> {
    let mut args = match node.children.len() {
        6 => node.children[4].clone(),
        4 => node.children[2].clone(),
        _ => ARGUMENTLIST.clone(),
    };
    args.flatten();

    let mut resolved = Vec::new();
    for arg in args.children {
        if arg.token.kind == TokenKind::Comma {
            continue;
        }

        match resolve::expression::go(&arg, modifiers, current, kinds, globals) {
            Ok(t) => resolved.push(t),
            Err(e) => return Err(e),
        };
    }

    Ok(resolved)
}

fn get_method(node: &ASTNode,
              modifiers: &Vec<ASTNode>,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>,
              globals: &mut Vec<VariableEnvironment>)
              -> Result<(ClassOrInterfaceEnvironment, MethodEnvironment), String> {
    let args = match get_args(node, modifiers, current, kinds, globals) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    match node.children.len() {
        // child[0] is a method.
        3 | 4 => {
            // TODO: resolve first? Might have to remove trailing "Dot Identifier"?
            let mut canonical = node.children[0].clone();
            canonical.flatten();
            if modifiers.contains(&*STATIC) &&
               canonical.children.first().unwrap().token.kind == TokenKind::This {
                return Err(format!("can not use 'this' in static method"));
            }

            let var_result = lookup::method::in_variables(&canonical,
                                                          &NAME.clone(),
                                                          &args,
                                                          current,
                                                          kinds,
                                                          globals);
            if var_result.is_ok() {
                return Ok(var_result.unwrap());
            }

            let exp_result =
                lookup::method::in_env(&canonical, &NAME.clone(), &args, current, kinds);
            if exp_result.is_ok() {
                return Ok(exp_result.unwrap());
            }

            // implicit `this`
            let mut imp_result = Err(format!("implicit 'this' can not be used in static methods"));
            if !modifiers.contains(&*STATIC) {
                // TODO: in_class would save some effort
                imp_result =
                    lookup::method::in_env(&current.name, &canonical, &args, current, kinds);
                if imp_result.is_ok() {
                    return Ok(imp_result.unwrap());
                }
            }

            Err(format!("could not resolve {:?} to method from class {:?}\n{}",
                        canonical,
                        current.name,
                        format!("got errors:\n\t{:?}\n\t{:?}\n\t{:?}",
                                var_result,
                                exp_result,
                                imp_result)))
        }
        // child[0] is class/field. child[2] is method on previous.
        5 | 6 => {
            let mut primary = NAME.clone();
            primary.children.push(node.children[0].clone());
            primary.flatten();
            if modifiers.contains(&*STATIC) &&
               primary.children.first().unwrap().token.kind == TokenKind::This {
                return Err(format!("can not use 'this' in static method"));
            }

            let lhs = match resolve::expression::go(&node.children[0],
                                                    modifiers,
                                                    current,
                                                    kinds,
                                                    globals) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            let mut name = NAME.clone();
            name.children.push(node.children[2].clone());
            name.flatten();

            // TODO: in_class would save some effort
            match lookup::method::in_env(&lhs.kind.name, &name, &args, current, kinds) {
                Ok(m) => return Ok(m),
                Err(_) => (),
            }

            Err(format!("could not resolve {:?} to method on class {:?}",
                        name,
                        lhs.kind.name))
        }
        _ => Err(format!("malformed MethodInvocation {:?}", node)),
    }
}

pub fn go(node: &ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let (cls, method) = match get_method(node, modifiers, current, kinds, globals) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    if cls.modifiers.contains(&*PROTECTED) || method.modifiers.contains(&*PROTECTED) {
        let mut current_package = current.name.clone();
        current_package.children.pop();

        let mut cls_package = cls.name.clone();
        cls_package.children.pop();

        if cls_package != current_package {
            let tcls = Type::new(cls.clone());
            let tcurrent = Type::new(current.clone());
            match tcls.assign(&tcurrent, current, kinds) {
                Ok(_) => (),
                Err(_) => {
                    return Err(format!("could not access method {} on class {} from class {}",
                                       method.name,
                                       cls.name,
                                       current.name))
                }
            }
        }
    }

    let kind = method.return_type.clone();
    match lookup::class::in_env(&kind, &cls, kinds) {
        Ok(cls) => Ok(Type::new(cls)),
        Err(_) => Err(format!("could not lookup kind {} of method in class {}", kind, cls)),
    }
}
