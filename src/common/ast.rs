use common::error;
use common::Token;
use common::TokenKind;
use parser::Node;
use parser::Tree;

#[derive(Debug)]
pub struct AST {
    pub package: Option<ASTNodePackage>,
    pub imports: Option<Vec<ASTNodeImport>>,
    pub root: Option<ASTNode>,
}

#[derive(Debug,Clone)]
pub struct ASTNode {
    pub token: Token,
    pub children: Vec<ASTNode>,
}

#[derive(Debug)]
pub struct ASTNodeImport {
    pub import: Vec<Token>,
}

#[derive(Debug)]
pub struct ASTNodePackage {
    pub package: Vec<Token>,
}

impl AST {
    pub fn new(parse_tree: &Result<Tree, error::ParserError>) -> Result<AST, error::ASTError> {
        let node = match parse_tree {
            &Ok(ref t) => t.clone().root,
            &Err(ref e) => return Err(error::ASTError { message: e.message.message }),
        };

        let mut imports = None;
        let mut package = None;
        let mut root = None;
        for child in node.children {
            match child.token.lexeme {
                Some(ref l) if l == "PackageDeclaration" => {
                    package = match AST::parse_package(&child) {
                        Ok(i) => Some(i),
                        Err(e) => return Err(e),
                    };
                }
                Some(ref l) if l == "ImportDeclarations" => {
                    imports = match AST::parse_imports(&child) {
                        Ok(i) => Some(i),
                        Err(e) => return Err(e),
                    };
                }
                Some(ref l) if l == "TypeDeclarations" => {
                    root = match AST::parse_types(&child) {
                        Ok(i) => Some(i),
                        Err(e) => return Err(e),
                    };
                }
                _ => return Err(error::ASTError { message: "invalid root child" }),
            }
        }

        Ok(AST {
            imports: imports,
            package: package,
            root: root,
        })
    }

    pub fn print(&self) {
        println!("{:?}", self.package);
        println!("{:?}", self.imports);
        self.root.clone().unwrap().print(0);
    }

    fn parse_imports(node: &Node) -> Result<Vec<ASTNodeImport>, error::ASTError> {
        if node.token.lexeme != Some("ImportDeclarations".to_string()) {
            return Err(error::ASTError { message: "incorrect import declarations" });
        }

        let mut imports: Vec<ASTNodeImport> = Vec::new();
        for child in node.children.clone() {
            let mut names: Vec<Token> = Vec::new();
            child.collect_child_kinds(&vec![&TokenKind::Identifier, &TokenKind::Star], &mut names);
            imports.push(ASTNodeImport { import: names });
        }

        Ok(imports)
    }

    fn parse_package(node: &Node) -> Result<ASTNodePackage, error::ASTError> {
        if node.children.len() != 3 || node.children[0].token.kind != TokenKind::Package ||
           node.children[2].token.kind != TokenKind::Semicolon {
            return Err(error::ASTError { message: "incorrect package declaration" });
        }

        let mut names: Vec<Token> = Vec::new();
        node.children[1].clone().collect_child_kinds(&vec![&TokenKind::Identifier], &mut names);

        Ok(ASTNodePackage { package: names })
    }

    fn parse_types(node: &Node) -> Result<ASTNode, error::ASTError> {
        match node.token.kind {
            TokenKind::NonTerminal => {
                match node.token.lexeme {
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
                        match AST::parse_types(&node.children[1]) {
                            Ok(child) => {
                                if child.token.kind == TokenKind::NumValue {
                                    match child.token.lexeme {
                                        Some(ref l) if l.parse().unwrap_or(0) > 2u64.pow(31) - 1 => {
                                            return Err(error::ASTError { message: "ast found int to big" });
                                        }
                                        _ => (),
                                    }
                                }

                                Ok(child)
                            }
                            Err(e) => Err(e),
                        }
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

                        if children[1].token.kind == TokenKind::NumValue {
                            match children[1].token.lexeme {
                                Some(ref l) if l.parse().unwrap_or(0) > 2u64.pow(31) => {
                                    return Err(error::ASTError { message: "ast found int to small" });
                                }
                                _ => (),
                            }
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
                    // TODO: does this miss the case of CastExpression: ( Name Dim ) UnaryNoSignExpression ?
                    Some(ref l) if node.children.len() == 1 && l == "UnaryExpression" => {
                        match AST::parse_types(&node.children[0]) {
                            Ok(node) => {
                                if node.token.kind == TokenKind::NumValue {
                                    match node.token.lexeme {
                                        Some(ref l) if l.parse().unwrap_or(0) > 2u64.pow(31) - 1 => {
                                            return Err(error::ASTError { message: "ast found int to big" });
                                        }
                                        _ => (),
                                    }
                                }

                                Ok(node)
                            },
                            Err(e) => Err(e),
                        }
                    }
                    Some(_) if node.children.len() == 1 => AST::parse_types(&node.children[0]),
                    _ => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in node.children.clone() {
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

impl ASTNode {
    pub fn print(&self, indent: u32) {
        let spaces = (0..indent).map(|_| " ").collect::<String>();
        println!("{}{:?}", spaces, self.token);

        for child in self.children.clone() {
            child.print(indent + 2);
        }
    }
}
