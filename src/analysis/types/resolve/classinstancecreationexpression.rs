use analysis::environment::ClassOrInterfaceEnvironment;
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
}

// TODO: dedup with methodinvocation::get_args
fn get_args(node: &ASTNode,
            modifiers: &Vec<ASTNode>,
            current: &ClassOrInterfaceEnvironment,
            kinds: &Vec<ClassOrInterfaceEnvironment>,
            globals: &Vec<VariableEnvironment>)
            -> Result<Vec<Type>, String> {
    let mut args = match node.children.len() {
        2 => node.children[1].clone(),
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

pub fn go(node: &ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let args = match get_args(node, modifiers, current, kinds, globals) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    match lookup::class::in_env(&node.children[0], current, kinds) {
        Ok(cls) => {
            match lookup::method::select_method(&cls.constructors.clone(), &args, &cls, kinds) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("could not find matching constructor\ngot errors:\n\t{:?}",
                                       e))
                }
            };

            Ok(Type::new(cls))
        }
        Err(e) => Err(e),
    }
}
