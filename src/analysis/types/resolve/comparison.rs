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
    static ref BYTE: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Byte, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref CHAR: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Char, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref FALSE: Token = {
        Token::new(TokenKind::False, None)
    };
    static ref INTEGER: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Int, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref SHORT: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Short, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref TRUE: Token = {
        Token::new(TokenKind::True, None)
    };
    static ref PRIMITIVES: [Type; 5] = [BOOLEAN.clone(), BYTE.clone(), CHAR.clone(),
                                        INTEGER.clone(), SHORT.clone()];
}
const BITWISE: [TokenKind; 3] = [TokenKind::BitAnd, TokenKind::BitOr, TokenKind::BitXor];

pub fn onearg_boolean(mut node: &mut ASTNode,
                      modifiers: &Vec<ASTNode>,
                      current: &ClassOrInterfaceEnvironment,
                      kinds: &Vec<ClassOrInterfaceEnvironment>,
                      globals: &mut Vec<VariableEnvironment>)
                      -> Result<Type, String> {
    let arg =
        match resolve::expression::go(&mut node.children[0], modifiers, current, kinds, globals) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

    if arg == *BOOLEAN {
        let mut result = BOOLEAN.clone();

        let value = match arg.kind.name.token.lexeme {
            Some(ref l) if l == "false" => false,
            Some(ref l) if l == "true" => true,
            _ => return Ok(result),
        };
        result.kind.name.token.lexeme = match value {
            false => Some("true".to_owned()),
            true => Some("false".to_owned()),
        };

        Ok(result)
    } else {
        Err(format!("could not apply {:?} to {:?}",
                    node.token.kind,
                    arg.kind.name))
    }
}

pub fn twoarg(mut node: &mut ASTNode,
              modifiers: &Vec<ASTNode>,
              current: &ClassOrInterfaceEnvironment,
              kinds: &Vec<ClassOrInterfaceEnvironment>,
              globals: &mut Vec<VariableEnvironment>)
              -> Result<Type, String> {
    let lhs =
        match resolve::expression::go(&mut node.children[0], modifiers, current, kinds, globals) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
    let rhs =
        match resolve::expression::go(&mut node.children[1], modifiers, current, kinds, globals) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

    lhs.apply_comparison(&node.token.kind, &rhs, current, kinds)
}

pub fn twoarg_boolean(mut node: &mut ASTNode,
                      modifiers: &Vec<ASTNode>,
                      current: &ClassOrInterfaceEnvironment,
                      kinds: &Vec<ClassOrInterfaceEnvironment>,
                      globals: &mut Vec<VariableEnvironment>)
                      -> Result<Type, String> {
    let lhs =
        match resolve::expression::go(&mut node.children[0], modifiers, current, kinds, globals) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
    let rhs =
        match resolve::expression::go(&mut node.children[1], modifiers, current, kinds, globals) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

    if lhs == *BOOLEAN && rhs == *BOOLEAN {
        let mut result = BOOLEAN.clone();

        let vlhs = match lhs.kind.name.token.lexeme {
            Some(ref l) if l == "false" => false,
            Some(ref l) if l == "true" => true,
            _ => return Ok(result),
        };
        let vrhs = match rhs.kind.name.token.lexeme {
            Some(ref l) if l == "false" => false,
            Some(ref l) if l == "true" => true,
            _ => return Ok(result),
        };

        let value = match node.token.kind {
            TokenKind::And => vlhs && vrhs,
            TokenKind::BitAnd => vlhs | vrhs,
            TokenKind::Or => vlhs || vrhs,
            TokenKind::BitOr => vlhs | vrhs,
            TokenKind::BitXor => vlhs ^ vrhs,
            _ => false,  // TODO: impossible
        };
        result.kind.name.token.lexeme = match value {
            false => Some("false".to_owned()),
            true => Some("true".to_owned()),
        };

        match node.token.kind {
            // only prune eager comparisons
            TokenKind::BitAnd | TokenKind::BitOr | TokenKind::BitXor => {
                node.token = match value {
                    false => FALSE.clone(),
                    true => TRUE.clone(),
                };
            }
            _ => (),
        }

        Ok(result)
    } else if BITWISE.contains(&node.token.kind) {
        Err(format!("bitwise operations are not allowed"))
    } else {
        Err(format!("could not apply {:?} to {:?} and {:?}",
                    node.token.kind,
                    lhs.kind.name,
                    rhs.kind.name))
    }
}

pub fn twoarg_instanceof(mut node: &mut ASTNode,
                         modifiers: &Vec<ASTNode>,
                         current: &ClassOrInterfaceEnvironment,
                         kinds: &Vec<ClassOrInterfaceEnvironment>,
                         globals: &mut Vec<VariableEnvironment>)
                         -> Result<Type, String> {
    let lhs =
        match resolve::expression::go(&mut node.children[0], modifiers, current, kinds, globals) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
    let rhs =
        match resolve::expression::go(&mut node.children[1], modifiers, current, kinds, globals) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

    if PRIMITIVES.contains(&lhs) || PRIMITIVES.contains(&rhs) {
        return Err(format!("can not apply instanceof to primitive types"));
    }

    let mut result = BOOLEAN.clone();
    match lhs.assign(&rhs, current, kinds) {
        Ok(_) => {
            node.token = TRUE.clone();
            result.kind.name.token.lexeme = Some("true".to_owned());
        }
        Err(_) => {
            node.token = FALSE.clone();
            result.kind.name.token.lexeme = Some("false".to_owned());
        }
    }

    Ok(result)
}
