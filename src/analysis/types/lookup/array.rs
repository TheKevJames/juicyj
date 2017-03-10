use analysis::environment::ClassOrInterface;
use analysis::environment::ClassOrInterfaceEnvironment;
use analysis::environment::FieldEnvironment;
use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

lazy_static! {
    static ref CLONEABLE: ASTNode = ASTNode {
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
                           token: Token::new(TokenKind::Identifier, Some("Cloneable")),
                           children: Vec::new(),
                       }],
    };
    static ref LENGTH: FieldEnvironment = {
        let length_name = ASTNode {
            token: Token::new(TokenKind::Identifier, Some("length")),
            children: Vec::new(),
        };
        let length_kind = ASTNode {
            token: Token::new(TokenKind::Int, None),
            children: Vec::new(),
        };
        let mut length = FieldEnvironment::new(length_name, length_kind);

        length.modifiers.push(ASTNode {
            token: Token::new(TokenKind::Public, None),
            children: Vec::new(),
        });
        length.modifiers.push(ASTNode {
            token: Token::new(TokenKind::Final, None),
            children: Vec::new(),
        });
        length
    };
    static ref OBJECT: ASTNode = ASTNode {
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
                           token: Token::new(TokenKind::Identifier, Some("Object")),
                           children: Vec::new(),
                       }],
    };
    static ref SERIALIZABLE: ASTNode = ASTNode {
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
                           token: Token::new(TokenKind::Identifier, Some("io")),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Dot, None),
                           children: Vec::new(),
                       },
                       ASTNode {
                           token: Token::new(TokenKind::Identifier, Some("Serializable")),
                           children: Vec::new(),
                       }],
    };
}

pub fn create(name: &ASTNode) -> ClassOrInterfaceEnvironment {
    let mut name = name.clone();
    // remove Dim or DimExpr
    name.children.truncate(1);
    name.children[0].flatten();
    let mut array = ClassOrInterfaceEnvironment::new(name, ClassOrInterface::CLASS);

    array.extends.push(OBJECT.clone());

    array.implements.push(CLONEABLE.clone());
    array.implements.push(SERIALIZABLE.clone());

    // TODO: .clone() (inherited from Object as protected) is public

    array.fields.push(LENGTH.clone());

    return array;
}
