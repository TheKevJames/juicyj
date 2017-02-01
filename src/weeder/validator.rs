use std;

use common::error;
use common::TokenKind;
use parser::Node;
use parser::Tree;

pub struct Weeder<'filename> {
    filename: &'filename str,
    has_class: bool,
    tree: Tree,
}

impl<'filename> Weeder<'filename> {
    pub fn new(filename: &'filename str, tree: Result<Tree, error::ParserError>) -> Weeder {
        let tree = match tree {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        };
        tree.clone().print();

        Weeder {
            filename: filename.split("/").last().unwrap_or(""),
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
                    Some(ref l) if l == "AbstractMethodDeclaration" => {
                        if !node.children[1].clone().has_child_kind(&TokenKind::Abstract) {
                            error!("Concrete method has no body!");
                            std::process::exit(42);
                        }
                    }
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

                        if node.children[1].clone().has_child_kind(&TokenKind::Native) {
                            match node.children[0].token.lexeme {
                                Some(ref l) if l == "MethodBody" => {
                                    error!("Native method has body!");
                                    std::process::exit(42);
                                }
                                _ => (),
                            }
                        }
                    }
                    Some(ref l) if l == "MethodHeader" => {
                        if node.children[1].clone().has_child_kind(&TokenKind::Abstract) {
                            if node.children[1].clone().has_child_kind(&TokenKind::Final) {
                                error!("Abstract method can not be final!");
                                std::process::exit(42);
                            }

                            if node.children[1].clone().has_child_kind(&TokenKind::Static) {
                                error!("Abstract method can not be static!");
                                std::process::exit(42);
                            }
                        }

                        if node.children[1].clone().has_child_kind(&TokenKind::Static) {
                            if node.children[1].clone().has_child_kind(&TokenKind::Final) {
                                error!("Static method can not be final!");
                                std::process::exit(42);
                            }
                        }

                        if node.children[1].clone().has_child_kind(&TokenKind::Native) {
                            if !node.children[1].clone().has_child_kind(&TokenKind::Static) {
                                error!("Native method must be static!");
                                std::process::exit(42);
                            }
                        }
                    }
                    Some(ref l) if l == "ClassDeclaration" => {
                        for (idx, child) in node.children.iter().enumerate() {
                            if child.token.kind == TokenKind::Class {
                                match node.children[idx - 1].token.lexeme {
                                    Some(ref l) if format!("{}.java", l) == self.filename => (),
                                    _ => {
                                        error!("Class name must match filename!");
                                        std::process::exit(42);
                                    }
                                }
                            }
                        }
                    }
                    Some(ref l) if l == "InterfaceDeclaration" => {
                        for (idx, child) in node.children.iter().enumerate() {
                            if child.token.kind == TokenKind::Interface {
                                match node.children[idx - 1].token.lexeme {
                                    Some(ref l) if format!("{}.java", l) == self.filename => (),
                                    _ => {
                                        error!("Interface name must match filename!");
                                        std::process::exit(42);
                                    }
                                }
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
