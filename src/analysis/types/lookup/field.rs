// TODO: rename? this is currently find_field_and_get_kind...
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

fn move_two(cls: &mut ASTNode, field: &mut ASTNode) -> Result<(), ()> {
    if cls.children.len() < 3 {
        return Err(());
    }

    let node = cls.children.pop();
    cls.children.pop(); // Dot

    if !field.children.is_empty() {
        field.children.insert(0, DOT.clone());
    }
    field.children.insert(0, node.unwrap());

    Ok(())
}

pub fn in_env(canonical: &ASTNode,
              field: &ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>)
              -> Result<Type, String> {
    let mut canonical = canonical.clone();

    loop {
        let cls = match lookup::class::in_env(&canonical, current, kinds) {
            Ok(cls) => cls,
            Err(_) => break,
        };

        let mut remaining_field = field.clone();
        let mut potential_field = NAME.clone();
        for (idx, child) in field.children.iter().enumerate() {
            potential_field.children.push(child.clone());
            remaining_field.children.remove(0);
            if idx % 2 != 0 {
                // canonical is "a.b.c". No trailing Dot is allowed.
            }

            for f in &cls.fields {
                if f.name != potential_field {
                    continue;
                }

                let kind = f.to_variable().kind.clone();
                let result = match lookup::class::in_env(&kind, &cls, kinds) {
                    Ok(cls) => cls,
                    Err(_) => return Err(format!("could not lookup kind {} of field in class {}", kind, cls)),
                };

                if remaining_field.children.is_empty() {
                    return Ok(Type::new(result));
                }

                // A.B.C.f.g.h, we're at f or g
                // result = kind of A.B.C.f.g, remaining_field = .h
                remaining_field.children.remove(0);
                // TODO: in_class would save some effort
                return in_env(&result.name, &remaining_field, current, kinds);
            }
        }

        // TODO: is this ambiguous?
        // return Err(...)
        break;
    }

    let mut field = field.clone();
    match move_two(&mut canonical, &mut field) {
        Ok(()) => in_env(&canonical, &field, current, kinds),
        // TODO: better error message
        Err(()) => Err(format!("could not lookup field in environment")),
    }
}

pub fn in_variables(canonical: &ASTNode,
                    field: &ASTNode,
                    current: &ClassOrInterfaceEnvironment,
                    kinds: &Vec<ClassOrInterfaceEnvironment>,
                    variables: &Vec<VariableEnvironment>)
                    -> Result<Type, String> {
    let mut canonical = canonical.clone();
    let mut field = field.clone();

    for var in variables {
        if var.name == canonical {
            return in_env(&var.kind, &field, current, kinds);
        }
    }

    match move_two(&mut canonical, &mut field) {
        Ok(()) => in_variables(&canonical, &field, current, kinds, variables),
        // TODO: better error message
        Err(()) => Err(format!("could not lookup field in variables")),
    }
}
