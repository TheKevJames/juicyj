use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::lookup;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref DOT: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Dot, None), children: Vec::new() }
    };
}

pub fn go(mut node: &mut ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &mut Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let idx = match node.children.len() {
        2 => 1,
        _ => 0,
    };

    let args = match resolve::methodinvocation::get_args(&mut node,
                                                         idx,
                                                         modifiers,
                                                         current,
                                                         kinds,
                                                         globals) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    match lookup::class::in_env(&node.children[0], current, kinds) {
        Ok(cls) => {
            let method = match lookup::method::select_method(&cls.constructors.clone(),
                                                             &args,
                                                             &cls,
                                                             kinds) {
                Ok(m) => m,
                Err(e) => {
                    return Err(format!("could not find matching constructor\ngot errors:\n\t{:?}",
                                       e))
                }
            };

            let mut fully_qualified = cls.name.clone();
            fully_qualified.flatten();
            fully_qualified.children.push(DOT.clone());
            fully_qualified.children.push(method.name.clone());
            node.children[0] = fully_qualified;

            Ok(Type::new(cls))
        }
        Err(_) => {
            // TODO: duplicate resolve?
            // remove full qualification
            let mut node_copy = node.clone();
            node_copy.children[0].flatten();
            node_copy.children[0].children.pop();
            node_copy.children[0].children.pop();

            go(&mut node_copy, modifiers, current, kinds, globals)
        }
    }
}
