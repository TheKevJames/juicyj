use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::VariableEnvironment;
use analysis::types::obj::Type;
use analysis::types::resolve;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref BOOLEAN: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Boolean, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
}
const BITWISE: [TokenKind; 3] = [TokenKind::BitAnd, TokenKind::BitOr, TokenKind::BitXor];

pub fn onearg_boolean(node: &ASTNode,
                      current: &ClassOrInterfaceEnvironment,
                      kinds: &Vec<ClassOrInterfaceEnvironment>,
                      globals: &Vec<VariableEnvironment>)
                      -> Result<Type, String> {
    let arg = match resolve::expression::go(&node.children[0], current, kinds, globals) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    if arg == *BOOLEAN {
        Ok(arg)
    } else {
        Err(format!("could not apply {:?} to {:?}",
                    node.token.kind,
                    arg.kind.name))
    }
}

pub fn twoarg(node: &ASTNode,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>,
              globals: &Vec<VariableEnvironment>)
              -> Result<Type, String> {
    let lhs = match resolve::expression::go(&node.children[0], current, kinds, globals) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };
    let rhs = match resolve::expression::go(&node.children[1], current, kinds, globals) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    lhs.apply_comparison(&node.token.kind, &rhs, current, kinds)
}

pub fn twoarg_boolean(node: &ASTNode,
                      current: &ClassOrInterfaceEnvironment,
                      kinds: &Vec<ClassOrInterfaceEnvironment>,
                      globals: &Vec<VariableEnvironment>)
                      -> Result<Type, String> {
    let lhs = match resolve::expression::go(&node.children[0], current, kinds, globals) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };
    let rhs = match resolve::expression::go(&node.children[1], current, kinds, globals) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    if lhs == *BOOLEAN && rhs == *BOOLEAN {
        Ok(lhs)
    } else if BITWISE.contains(&node.token.kind) {
        Err(format!("bitwise operations are not allowed"))
    } else {
        Err(format!("could not apply {:?} to {:?} and {:?}",
                    node.token.kind,
                    lhs.kind.name,
                    rhs.kind.name))
    }
}

pub fn twoarg_instanceof(node: &ASTNode,
                         current: &ClassOrInterfaceEnvironment,
                         kinds: &Vec<ClassOrInterfaceEnvironment>,
                         globals: &Vec<VariableEnvironment>)
                         -> Result<Type, String> {
    // TODO: do these need to resolve to anything special?
    match resolve::expression::go(&node.children[0], current, kinds, globals) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }
    match resolve::expression::go(&node.children[1], current, kinds, globals) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    Ok(BOOLEAN.clone())
}
