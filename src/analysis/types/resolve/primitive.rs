use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::types::obj::Type;
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
    static ref INTEGER: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Int, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref NULL: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Null, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref SHORT: Type = {
        let node = ASTNode { token: Token::new(TokenKind::Short, None), children: Vec::new() };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
    static ref STRING: Type = {
        let node = ASTNode {
            token: Token::new(TokenKind::NonTerminal, Some("Name")),
            children: vec![ASTNode {
                               token: Token::new(TokenKind::Identifier, Some("java")),
                               children: Vec::new(),
                           },
                           ASTNode {
                               token: Token::new(TokenKind::Dot, None),
                               children: Vec::new(),
                           },
                           ASTNode {
                               token: Token::new(TokenKind::Identifier, Some("lang")),
                               children: Vec::new(),
                           },
                           ASTNode {
                               token: Token::new(TokenKind::Dot, None),
                               children: Vec::new(),
                           },
                           ASTNode {
                               token: Token::new(TokenKind::Identifier, Some("String")),
                               children: Vec::new(),
                           }],
        };
        Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS))
    };
}

pub fn go(node: &ASTNode) -> Result<Type, String> {
    match node.token.kind {
        // primitives
        TokenKind::Boolean => Ok(BOOLEAN.clone()),
        TokenKind::Byte => Ok(BYTE.clone()),
        TokenKind::Char => Ok(CHAR.clone()),
        TokenKind::Int => Ok(INTEGER.clone()),
        TokenKind::Null => Ok(NULL.clone()),
        TokenKind::Short => Ok(SHORT.clone()),
        // primitive values
        // TODO: lexemes?
        TokenKind::CharValue => Ok(CHAR.clone()),
        TokenKind::NumValue => {
            // TODO: do this better. Maybe a ResolvedToken type?
            let node = ASTNode {
                token: Token {
                    kind: TokenKind::Int,
                    lexeme: node.token.clone().lexeme,
                },
                children: Vec::new(),
            };
            Ok(Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS)))
        }
        TokenKind::StrValue => Ok(STRING.clone()),
        TokenKind::True | TokenKind::False => {
            // TODO: do this better. Maybe a ResolvedToken type?
            let value = if node.token.kind == TokenKind::True {
                "true"
            } else {
                "false"
            };

            let node = ASTNode {
                token: Token::new(TokenKind::Boolean, Some(value)),
                children: Vec::new(),
            };
            Ok(Type::new(ClassOrInterfaceEnvironment::new(node, ClassOrInterface::CLASS)))
        }
        _ => Err(format!("invalid primitive type {:?}", node)),
    }
}
