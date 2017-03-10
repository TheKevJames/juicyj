use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref NAME: ASTNode = {
        ASTNode { token: Token::new(TokenKind::NonTerminal, Some("Name")), children: Vec::new() }
    };
}

pub fn go(node: &ASTNode,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    match node.children.len() {
        // child[0] is a method. child[3] is an arg if children.len() == 4
        // TODO: arg
        3 | 4 => {
            // TODO: resolve first? Might have to remove trailing "Dot Identifier"?
            let mut canonical = node.children[0].clone();
            canonical.flatten();

            match lookup::method::in_variables(&canonical, &NAME.clone(), current, kinds, globals) {
                Ok(t) => return Ok(t),
                Err(_) => (),
            }

            match lookup::method::in_env(&canonical, &NAME.clone(), current, kinds) {
                Ok(t) => return Ok(t),
                Err(_) => (),
            }

            // implicit `this`
            // TODO: in_class would save some effort
            match lookup::method::in_env(&current.name, &canonical, current, kinds) {
                Ok(t) => return Ok(t),
                Err(_) => (),
            }

            Err(format!("could not resolve {:?} to method from class {:?}",
                        canonical,
                        current.name))
        }
        // child[0] is class/field. child[2] is method on previous. child[5] is
        // an arg if children.len() == 6
        // TODO: arg
        5 | 6 => {
            let lhs = match resolve::expression::go(&node.children[0], current, kinds, globals) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            let mut name = NAME.clone();
            name.children.push(node.children[2].clone());
            name.flatten();

            // TODO: in_class would save some effort
            match lookup::method::in_env(&lhs.kind.name, &name, current, kinds) {
                Ok(t) => return Ok(t),
                Err(_) => (),
            }

            Err(format!("could not resolve {:?} to method on class {:?}",
                        node.children[2],
                        lhs.kind.name))
        }
        _ => Err(format!("malformed MethodInvocation {:?}", node)),
    }
}
