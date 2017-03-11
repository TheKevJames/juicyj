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
    static ref STATIC: ASTNode = {
        ASTNode { token: Token::new(TokenKind::Static, None), children: Vec::new() }
    };
}

// A "Name" can refer to a bunch of things.
//   - instantiated/uninstantiated classes
//   - fields on either of the above classes
//   - etc
pub fn go(node: &ASTNode,
          modifiers: &Vec<ASTNode>,
          current: &ClassOrInterfaceEnvironment,
          kinds: &Vec<ClassOrInterfaceEnvironment>,
          globals: &Vec<VariableEnvironment>)
          -> Result<Type, String> {
    let mut node = node.clone();
    node.flatten();
    if modifiers.contains(&*STATIC) &&
       node.children.first().unwrap().token.kind == TokenKind::This {
        return Err(format!("can not use 'this' in static method"));
    }

    // TODO: make lookup::field return a FieldEnv. Then, when looking up type, keep the modifiers
    match lookup::field::in_variables(&node, &NAME.clone(), current, kinds, globals) {
        Ok(t) => return Ok(t),
        Err(_) => (),
    }

    match lookup::class::in_variables(&node, current, kinds, globals) {
        Ok(c) => return Ok(Type::new(c)),
        Err(_) => (),
    }

    match lookup::field::in_env(&node, &NAME.clone(), current, kinds) {
        Ok(t) => return Ok(t),
        Err(_) => (),
    }

    // implicit `this`
    if !modifiers.contains(&*STATIC) {
        // TODO: in_class would save some effort
        match lookup::field::in_env(&current.name, &node, current, kinds) {
            Ok(t) => return Ok(t),
            Err(_) => (),
        }
    }

    match lookup::class::in_env(&node, current, kinds) {
        Ok(f) => Ok(Type::new(f)),
        Err(e) => Err(e),
    }
}
