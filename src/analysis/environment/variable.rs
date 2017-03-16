use std::fmt;

use scanner::ASTNode;
use scanner::Token;
use scanner::TokenKind;

#[derive(Clone,Debug)]
pub struct VariableEnvironment {
    pub kind: ASTNode,
    pub name: ASTNode,
    pub dim: bool,
    pub initialized: bool,
}

impl VariableEnvironment {
    pub fn new(node: ASTNode) -> VariableEnvironment {
        let dim = node.children[0].clone().token.lexeme.unwrap_or("".to_owned()) == "ArrayType";
        let mut kind = node.children[0].clone().flatten().clone();

        VariableEnvironment {
            kind: match kind.clone().token.lexeme {
                Some(ref l) if l == "ArrayType" => {
                    // Remove Dim or DimExpr
                    kind.children.truncate(1);
                    // Flatten Name
                    kind.children[0].flatten();
                    kind
                }
                _ => kind,
            },
            name: {
                let name = match node.children[1].clone().token.kind {
                    TokenKind::Assignment => node.children[1].clone().children[0].clone(),
                    _ => node.children[1].clone(),
                };

                let lex = name.clone().token.lexeme.unwrap_or("".to_owned());
                if lex == "ArrayType" || lex == "Name" {
                    name
                } else {
                    ASTNode {
                        token: Token::new(TokenKind::NonTerminal, Some("Name")),
                        children: vec![name],
                    }
                }
            },
            dim: dim,
            initialized: false,
        }
    }
}

impl PartialEq for VariableEnvironment {
    fn eq(&self, other: &VariableEnvironment) -> bool {
        self.kind == other.kind && self.dim == other.dim
    }
}

impl fmt::Display for VariableEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", self.kind, self.name));
        if self.dim {
            try!(write!(f, "[]"));
        }
        Ok(())
    }
}
