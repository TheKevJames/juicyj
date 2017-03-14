use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::MethodEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref NAME: ASTNode = {
        ASTNode { token: Token::new(TokenKind::NonTerminal, Some("Name")), children: Vec::new() }
    };
    static ref DOT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Dot, None), children: Vec::new() }
    };
}

fn move_two(cls: &mut ASTNode, method: &mut ASTNode) -> Result<(), ()> {
    if cls.children.len() < 3 {
        return Err(());
    }

    let node = cls.children.pop();
    cls.children.pop(); // Dot

    if !method.children.is_empty() {
        method.children.insert(0, DOT.clone());
    }
    method.children.insert(0, node.unwrap());

    Ok(())
}

pub fn select_method(methods: &Vec<MethodEnvironment>,
                     args: &Vec<Type>,
                     cls: &ClassOrInterfaceEnvironment,
                     kinds: &Vec<ClassOrInterfaceEnvironment>)
                     -> Result<MethodEnvironment, String> {
    let mut best: Option<MethodEnvironment> = None;
    let mut distance = <u32>::max_value();

    for method in methods {
        if method.parameters.len() != args.len() {
            continue;
        }

        if method.parameters.is_empty() {
            if distance == 0 {
                best = None;
            } else {
                best = Some(method.clone());
                distance = 0;
            }

            continue;
        }

        let mut method_distance = 0;
        for (param, arg) in method.parameters.iter().zip(args.iter()) {
            let found_param = match lookup::class::in_env(&param.kind, cls, kinds) {
                Ok(fp) => Type::new(fp),
                Err(e) => return Err(e),
            };

            match found_param.edit_distance(&arg, cls, kinds) {
                Ok(ed) if ed == <u32>::max_value() => method_distance = ed,
                Ok(ed) => method_distance += ed,
                Err(e) => return Err(e),
            }
        }

        match method_distance {
            md if md == <u32>::max_value() => (),
            md if md == distance => best = None,
            md if md < distance => {
                best = Some(method.clone());
                distance = md;
            }
            _ => (),
        }
    }

    if distance == <u32>::max_value() {
        return Err(format!("no method matching parameters was found"));
    }

    if best.is_none() {
        return Err(format!("ambiguous methods were found with distance {}", distance));
    }

    Ok(best.unwrap())
}

// TODO: this has reversed args from class::in_env...
pub fn in_env(canonical: &ASTNode,
              method: &ASTNode,
              args: &Vec<Type>,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<(ClassOrInterfaceEnvironment, MethodEnvironment), String> {
    loop {
        let cls = match lookup::class::in_env(&canonical, current, kinds) {
            Ok(cls) => cls,
            Err(_) => break,
        };

        let mut remaining_method = method.clone();
        let mut potential_method = NAME.clone();
        for (idx, child) in method.children.iter().enumerate() {
            potential_method.children.push(child.clone());
            remaining_method.children.remove(0);
            if idx % 2 != 0 {
                // canonical is "a.b.c". No trailing Dot is allowed.
                continue;
            }

            let mut methods = Vec::new();
            for m in &cls.methods {
                if m.name != potential_method {
                    continue;
                }

                if !remaining_method.children.is_empty() {
                    // Method chains should be seperate MethodInvocations, no?
                    return Err(format!("TODO: clearly, I do not understand methods"));
                }

                methods.push(m.clone());
            }

            if !methods.is_empty() {
                return match select_method(&methods, args, &cls, kinds) {
                    Ok(m) => Ok((cls.clone(), m)),
                    Err(e) => Err(e),
                };
            }

            // A.B.C.f.g.h.i(), we're at f or g
            for f in &cls.fields {
                if f.name != potential_method {
                    continue;
                }

                if remaining_method.children.is_empty() {
                    // h was a field?
                    return Err(format!("TODO: uh... what? MethodIsNotAMethodError."));
                }

                return match lookup::class::in_env(&f.kind, &cls, kinds) {
                    Ok(cls) => {
                        remaining_method.children.remove(0);
                        // cls is now kind or A.B.C.f.g, remaining_method is h.i()
                        in_env(&cls.name, &remaining_method, args, &cls, kinds)
                    }
                    Err(_) => {
                        Err(format!("could not lookup kind {} of field in class {}", f.kind, cls))
                    }
                };
            }
        }

        return Err(format!("resolved method {} to class {} without that method",
                           method,
                           cls.name));
    }

    let mut canonical = canonical.clone();
    let mut method = method.clone();
    match move_two(&mut canonical, &mut method) {
        Ok(()) => in_env(&canonical, &method, args, current, kinds),
        // TODO: better error message
        Err(()) => Err(format!("could not lookup method in environment")),
    }
}

pub fn in_variables(canonical: &ASTNode,
                    method: &ASTNode,
                    args: &Vec<Type>,
                    current: &ClassOrInterfaceEnvironment,
                    kinds: &Vec<ClassOrInterfaceEnvironment>,
                    variables: &Vec<VariableEnvironment>)
                    -> Result<(ClassOrInterfaceEnvironment, MethodEnvironment), String> {
    let mut canonical = canonical.clone();
    let mut method = method.clone();

    for var in variables {
        if var.name == canonical {
            return in_env(&var.kind, &method, args, current, kinds);
        }
    }

    match move_two(&mut canonical, &mut method) {
        Ok(()) => in_variables(&canonical, &method, args, current, kinds, variables),
        // TODO: better error message
        Err(()) => Err(format!("could not lookup method in variables")),
    }
}
