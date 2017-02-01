use common::error;
use common::TokenKind;
use parser::Node;
use parser::Tree;

pub struct Weeder<'filename, 'tree> {
    filename: &'filename str,
    has_class: bool,
    tree: &'tree Tree,
}

impl<'filename, 'tree> Weeder<'filename, 'tree> {
    pub fn new(filename: &'filename str,
               tree: &'tree Tree)
               -> Weeder<'filename, 'tree> {
        Weeder {
            filename: filename.split("/").last().unwrap_or(""),
            has_class: false,
            tree: tree,
        }
    }

    pub fn verify(&mut self, node: Option<Node>) -> Result<(), error::WeederError> {
        let node = match node {
            Some(n) => n,
            _ => self.tree.root.clone(),
        };

        match node.token.kind {
            TokenKind::NumValue => {
                match node.token.lexeme {
                    Some(ref l) if l.starts_with("0") && l.len() > 1 => {
                        return Err(error::WeederError { message: "weeder found octal digit!" });
                    }
                    Some(ref l) if l.parse().unwrap_or(0) > 2u64.pow(31) => {
                        return Err(error::WeederError { message: "weeder found int out of range" });
                    }
                    _ => (),
                }
            }
            TokenKind::Class if self.has_class => {
                return Err(error::WeederError { message: "weeder found multiple classes!" });
            }
            TokenKind::Class => self.has_class = true,
            TokenKind::NonTerminal => {
                match node.token.lexeme {
                    Some(ref l) if l == "AbstractMethodDeclaration" => {
                        if !node.children[0].clone().has_child_kind(&TokenKind::Abstract) &&
                           !node.children[0].clone().has_child_kind(&TokenKind::Native) {
                            return Err(error::WeederError {
                                message: "weeder found concrete method with no body",
                            });
                        }
                    }
                    Some(ref l) if l == "MethodDeclaration" &&
                                   node.children[1].token.lexeme ==
                                   Some("MethodBody".to_string()) => {
                        if node.children[0].clone().has_child_kind(&TokenKind::Abstract) {
                            return Err(error::WeederError {
                                message: "weeder found abstract method with body",
                            });
                        }

                        if node.children[0].clone().has_child_kind(&TokenKind::Native) {
                            return Err(error::WeederError {
                                message: "weeder found native method with body",
                            });
                        }
                    }
                    Some(ref l) if l == "MethodHeader" => {
                        if node.children[0].clone().has_child_kind(&TokenKind::Abstract) {
                            if node.children[0].clone().has_child_kind(&TokenKind::Final) {
                                return Err(error::WeederError {
                                    message: "weeder found final abstract method",
                                });
                            }

                            if node.children[0].clone().has_child_kind(&TokenKind::Static) {
                                return Err(error::WeederError {
                                    message: "weeder found static abstract method",
                                });
                            }
                        }

                        if node.children[0].clone().has_child_kind(&TokenKind::Static) {
                            if node.children[0].clone().has_child_kind(&TokenKind::Final) {
                                return Err(error::WeederError {
                                    message: "weeder found static final method",
                                });
                            }
                        }

                        if node.children[0].clone().has_child_kind(&TokenKind::Native) {
                            if !node.children[0].clone().has_child_kind(&TokenKind::Static) {
                                return Err(error::WeederError {
                                    message: "weeder found non-static native method",
                                });
                            }
                        }
                    }
                    Some(ref l) if l == "FieldDeclaration" &&
                                   node.children[0].clone().has_child_kind(&TokenKind::Final) &&
                                   node.children[2].clone().children.len() != 3 => {
                        return Err(error::WeederError {
                            message: "weeder found final field with no initializer",
                        });
                    }
                    Some(ref l) if l == "ClassDeclaration" => {
                        for (idx, child) in node.children.iter().enumerate() {
                            if child.token.kind == TokenKind::Class {
                                match node.children[idx + 1].clone().token.lexeme {
                                    Some(lexeme) => {
                                        if format!("{}.java", lexeme) != self.filename {
                                            return Err(error::WeederError {
                                                message: "weeder found mis-named class",
                                            });
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    Some(ref l) if l == "InterfaceDeclaration" => {
                        for (idx, child) in node.children.iter().enumerate() {
                            if child.token.kind == TokenKind::Interface {
                                match node.children[idx + 1].clone().token.lexeme {
                                    Some(lexeme) => {
                                        if format!("{}.java", lexeme) != self.filename {
                                            return Err(error::WeederError {
                                                message: "weeder found mis-named interface",
                                            });
                                        }
                                    }
                                    _ => (),
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
            match self.verify(Some(child)) {
                Err(e) => return Err(e),
                _ => (),
            }
        }

        Ok(())
    }
}
