// TODO: rename? this is currently find_method_and_get_return_type...
use analysis::environment::ClassOrInterfaceEnvironment;
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

// TODO: this has reversed args from class::in_env...
pub fn in_env(canonical: &ASTNode,
              method: &ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<Type, String> {
    let mut canonical = canonical.clone();

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

            for m in &cls.methods {
                if m.name != potential_method {
                    continue;
                }

                if !remaining_method.children.is_empty() {
                    // Method chains should be seperate MethodInvocations, no?
                    return Err(format!("TODO: clearly, I do not understand methods"));
                }

                let kind = m.return_type.clone();
                return match lookup::class::in_env(&kind, &cls, kinds) {
                    Ok(cls) => Ok(Type::new(cls)),
                    Err(_) => {
                        Err(format!("could not lookup kind {} of method in class {}", kind, cls))
                    }
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

                let kind = f.to_variable().kind.clone();
                return match lookup::class::in_env(&kind, &cls, kinds) {
                    Ok(cls) => {
                        remaining_method.children.remove(0);
                        // cls is now kind or A.B.C.f.g, remaining_method is h.i()
                        in_env(&cls.name, &remaining_method, &cls, kinds)
                    }
                    Err(_) => {
                        Err(format!("could not lookup kind {} of field in class {}", kind, cls))
                    }
                };
            }
        }

        // TODO: is this ambiguous?
        // return Err(...)
        break;
    }

    let mut method = method.clone();
    match move_two(&mut canonical, &mut method) {
        Ok(()) => in_env(&canonical, &method, current, kinds),
        // TODO: better error message
        Err(()) => Err(format!("could not lookup method in environment")),
    }
}

pub fn in_variables(canonical: &ASTNode,
                    method: &ASTNode,
                    current: &ClassOrInterfaceEnvironment,
                    kinds: &Vec<ClassOrInterfaceEnvironment>,
                    variables: &Vec<VariableEnvironment>)
                    -> Result<Type, String> {
    let mut canonical = canonical.clone();
    let mut method = method.clone();

    for var in variables {
        if var.name == canonical {
            return in_env(&var.kind, &method, current, kinds);
        }
    }

    match move_two(&mut canonical, &mut method) {
        Ok(()) => in_variables(&canonical, &method, current, kinds, variables),
        // TODO: better error message
        Err(()) => Err(format!("could not lookup method in variables")),
    }
}
