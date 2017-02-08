use std::fmt;

use scanner::common::error;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::Node;
use scanner::parser::Tree;

mod node;

use self::node::ASTNodeImport;
use self::node::ASTNode;
use self::node::ASTNodePackage;

pub struct AST {
    pub package: Option<ASTNodePackage>,
    pub imports: Vec<ASTNodeImport>,
    pub root: Option<ASTNode>,
}

impl AST {
    pub fn new(parse_tree: &Tree) -> Result<AST, error::ASTError> {
        let mut imports = Vec::new();
        let mut package = None;
        let mut root = None;
        for child in &parse_tree.root.children {
            match child.token.lexeme {
                Some(ref l) if l == "PackageDeclaration" => {
                    package = match ASTNodePackage::new(&child) {
                        Ok(i) => Some(i),
                        Err(e) => return Err(e),
                    };
                }
                Some(ref l) if l == "ImportDeclarations" => {
                    let mut statements: Vec<Node> = Vec::new();
                    child.collect_child_lexeme("ImportDeclaration", &mut statements);

                    for child in statements {
                        imports.push(match ASTNodeImport::new(&child) {
                            Ok(i) => i,
                            Err(e) => return Err(e),
                        });
                    }
                }
                Some(ref l) if l == "TypeDeclarations" => {
                    root = match AST::parse_types(&child) {
                        Ok(i) => Some(i),
                        Err(e) => return Err(e),
                    };
                }
                _ => return Err(error::ASTError { message: error::INVALID_ROOT_CHILD }),
            }
        }

        Ok(AST {
            imports: imports,
            package: package,
            root: root,
        })
    }

    fn parse_types(node: &Node) -> Result<ASTNode, error::ASTError> {
        match node.token.kind {
            TokenKind::NonTerminal => {
                match node.token.lexeme {
                    Some(ref l) if node.children.len() == 4 && l == "CastExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match AST::parse_types(&child) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }

                        if node.children[1].token.lexeme == Some("Expression".to_string()) {
                            match children[1].token.kind {
                                // TODO: does this cover x.y ?
                                TokenKind::Identifier => (),
                                _ => {
                                    println!("{}", children[1]);
                                    return Err(error::ASTError { message: error::INVALID_CAST });
                                }
                            }

                            let mut nodes: Vec<Node> = Vec::new();
                            node.children[1].collect_child_lexeme("PrimaryNoNewArray", &mut nodes);
                            for n in nodes {
                                if n.children.len() != 1 {
                                    return Err(error::ASTError { message: error::INVALID_CAST });
                                }
                            }
                        }

                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 3 &&
                                   (l.ends_with("Expression") || l == "VariableDeclarator") => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match AST::parse_types(&node.children[0]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        match AST::parse_types(&node.children[2]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        Ok(ASTNode {
                            token: node.children[1].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 3 && l == "ReturnStatement" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match AST::parse_types(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 3 && l == "PrimaryNoNewArray" => {
                        AST::parse_types(&node.children[1])
                    }
                    Some(ref l) if node.children.len() == 2 && l == "UnaryExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        children.push(ASTNode {
                            token: Token {
                                kind: TokenKind::NumValue,
                                lexeme: Some("0".to_string()),
                            },
                            children: Vec::new(),
                        });
                        match AST::parse_types(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
                    }
                    Some(_) if node.children.len() == 2 &&
                               node.children[1].token.kind == TokenKind::Semicolon => {
                        AST::parse_types(&node.children[0])
                    }
                    // TODO: does this miss the following case?
                    // CastExpression:
                    //     ( Name Dim ) UnaryNoSignExpression

                    // parent of UnaryExpression
                    Some(ref l) if node.children.len() == 1 && l == "MultiplicativeExpression" => {
                        match AST::parse_types(&node.children[0]) {
                            Ok(node) => {
                                if node.token.kind == TokenKind::NumValue {
                                    match node.token.lexeme {
                                        Some(ref l) if l.parse().unwrap_or(0) >
                                                       2u64.pow(31) - 1 => {
                                            return Err(error::ASTError { message: error::INT_OOB });
                                        }
                                        _ => (),
                                    }
                                }

                                Ok(node)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Some(_) if node.children.len() == 1 => AST::parse_types(&node.children[0]),
                    _ => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match AST::parse_types(&child) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                }
            }
            _ => {
                Ok(ASTNode {
                    token: node.token.clone(),
                    children: Vec::new(),
                })
            }
        }
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.package {
            Some(ref p) => try!(writeln!(f, "Package: {}", p)),
            None => try!(writeln!(f, "[no package declaration]")),
        }

        if !self.imports.is_empty() {
            try!(writeln!(f, "Imports:"));
        }
        for i in &self.imports {
            try!(writeln!(f, "{:2}", i));
        }

        write!(f, "{}", self.root.clone().unwrap())
    }
}
