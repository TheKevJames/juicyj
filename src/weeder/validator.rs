use std;

use common::error;
use common::TokenKind;
use parser::Node;
use parser::Tree;

pub struct Weeder {
    has_class: bool,
    tree: Tree,
}

impl Weeder {
    pub fn new(tree: Result<Tree, error::ParserError>) -> Weeder {
        let tree = match tree {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        };
        tree.clone().print();

        Weeder {
            has_class: false,
            tree: tree,
        }
    }

    pub fn verify(&mut self, node: Option<Node>) {
        let node = match node {
            Some(n) => n,
            _ => self.tree.root.clone(),
        };

        match node.token.kind {
            TokenKind::NumValue => {
                match node.token.lexeme {
                    Some(ref n) if n.starts_with("0") && n.len() > 1 => {
                        error!("Octal digit!");
                        std::process::exit(42);
                    }
                    Some(ref n) => {
                        if n.parse::<i64>().unwrap_or(0) > (2i64.pow(31) - 1) {
                            error!("Out of bounds int!");
                            std::process::exit(42);
                        }
                    }
                    _ => (),
                }
            }
            TokenKind::Class => {
                match self.has_class {
                    true => {
                        error!("Multiple classes!");
                        std::process::exit(42);
                    }
                    false => self.has_class = true,
                }
            }
            TokenKind::NonTerminal => {
                match node.token.lexeme {
                    Some(ref l) if l == "MethodDeclaration" => {
                        if node.children[1].clone().has_child_kind(&TokenKind::Abstract) {
                            match node.children[0].token.lexeme {
                                Some(ref l) if l == "MethodBody" => {
                                    error!("Abstract method has body!");
                                    std::process::exit(42);
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        for child in node.children {
            self.verify(Some(child));
        }
    }
}
