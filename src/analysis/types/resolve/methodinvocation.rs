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
    static ref ARGUMENT: Token = {
        Token::new(TokenKind::NonTerminal, Some("Argument"))
    };
    static ref DOT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Dot, None), children: Vec::new() }
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

pub fn get_args(mut node: &mut ASTNode,
                idx: usize,
                modifiers: &Vec<ASTNode>,
                current: &ClassOrInterfaceEnvironment,
                kinds: &Vec<ClassOrInterfaceEnvironment>,
                globals: &mut Vec<VariableEnvironment>)
                -> Result<Vec<Type>, String> {
    if idx == 0 {
        return Ok(Vec::new());
    }

    node.children[idx].flatten();

    let mut resolved = Vec::new();
    for mut arg in &mut node.children[idx].children {
        if arg.token.kind == TokenKind::Comma {
            continue;
        }

        if arg.clone().token.lexeme.unwrap_or("".to_owned()) == "Argument" {
            // already has type info
            match resolve::expression::go(&mut arg.children[1].clone(),
                                          modifiers,
                                          current,
                                          kinds,
                                          globals) {
                Ok(t) => resolved.push(t),
                Err(e) => return Err(e),
            }
            continue;
        }

        let kind = match resolve::expression::go(&mut arg, modifiers, current, kinds, globals) {
            Ok(t) => {
                resolved.push(t.clone());
                t.kind.name
            }
            Err(e) => return Err(e),
        };

        let name = arg.clone();
        arg.token = ARGUMENT.clone();
        arg.children = vec![kind, name];
    }

    Ok(resolved)
}

fn get_method(mut node: &mut ASTNode,
              modifiers: &Vec<ASTNode>,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>,
              globals: &mut Vec<VariableEnvironment>)
              -> Result<(ClassOrInterfaceEnvironment, MethodEnvironment), String> {
    let idx = match node.children.len() {
        6 => 4,
        4 => 2,
        _ => 0,
    };

    let args = match get_args(node, idx, modifiers, current, kinds, globals) {
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

            let lhs = match resolve::expression::go(&mut node.children[0],
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

pub fn go(node: &mut ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let (cls, method) = match get_method(node, modifiers, current, kinds, globals) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };

    let mut fully_qualified = cls.name.clone();
    fully_qualified.flatten();
    fully_qualified.children.push(DOT.clone());
    fully_qualified.children.push(method.name.clone());
    match node.children.len() {
        3 | 4 => node.children[0] = fully_qualified,
        5 | 6 => {
            node.children[0] = fully_qualified;
            node.children.remove(2);
            node.children.remove(1);
        }
        _ => (),
    }

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
