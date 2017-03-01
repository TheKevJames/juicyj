use error::ErrorMessage;
use error::WeederError;
use scanner::common::TokenKind;
use scanner::parser::ParseNode;
use scanner::parser::ParseTree;

pub struct Weeder<'filename, 'tree> {
    filename: &'filename str,
    has_class: bool,
    tree: &'tree ParseTree,
}

impl<'filename, 'tree> Weeder<'filename, 'tree> {
    pub fn new(filename: &'filename str, tree: &'tree ParseTree) -> Weeder<'filename, 'tree> {
        Weeder {
            filename: filename.split("/").last().unwrap_or(""),
            has_class: false,
            tree: tree,
        }
    }

    fn error(&self, message: ErrorMessage, node: &ParseNode) -> Result<(), WeederError> {
        Err(WeederError::new(self.filename.to_owned(), message, &node))
    }

    pub fn verify(&mut self, node: Option<ParseNode>) -> Result<(), WeederError> {
        let node = match node {
            Some(n) => n,
            _ => self.tree.root.clone(),
        };

        match node.token.kind {
            TokenKind::NumValue => {
                match node.token.lexeme {
                    Some(ref l) if l.starts_with("0") && l.len() > 1 => {
                        return self.error(ErrorMessage::InvalidOctal, &node);
                    }
                    Some(ref l) if l.parse().unwrap_or(0) > 2u64.pow(31) => {
                        return self.error(ErrorMessage::IntOOB, &node);
                    }
                    _ => (),
                }
            }
            TokenKind::Class if self.has_class => {
                return self.error(ErrorMessage::MultipleClasses, &node);
            }
            TokenKind::Class => self.has_class = true,
            TokenKind::NonTerminal => {
                match node.token.lexeme {
                    Some(ref l) if l == "AbstractMethodDeclaration" => {
                        if !node.children[0].clone().has_child_kind(&TokenKind::Abstract) &&
                           !node.children[0].clone().has_child_kind(&TokenKind::Native) {
                            return self.error(ErrorMessage::ConcreteNoBody, &node);
                        }
                    }
                    Some(ref l) if l == "MethodDeclaration" &&
                                   node.children[1].token.lexeme ==
                                   Some("MethodBody".to_string()) => {
                        if node.children[0].clone().has_child_kind(&TokenKind::Abstract) {
                            return self.error(ErrorMessage::AbstractBody, &node);
                        }

                        if node.children[0].clone().has_child_kind(&TokenKind::Native) {
                            return self.error(ErrorMessage::NativeBody, &node);
                        }
                    }
                    Some(ref l) if l == "MethodHeader" => {
                        if node.children[0].clone().has_child_kind(&TokenKind::Abstract) {
                            if node.children[0].clone().has_child_kind(&TokenKind::Final) {
                                return self.error(ErrorMessage::FinalAbstract, &node);
                            }

                            if node.children[0].clone().has_child_kind(&TokenKind::Static) {
                                return self.error(ErrorMessage::StaticAbstract, &node);
                            }
                        }

                        if node.children[0].clone().has_child_kind(&TokenKind::Static) {
                            if node.children[0].clone().has_child_kind(&TokenKind::Final) {
                                return self.error(ErrorMessage::StaticFinal, &node);
                            }
                        }

                        if node.children[0].clone().has_child_kind(&TokenKind::Native) {
                            if !node.children[0].clone().has_child_kind(&TokenKind::Static) {
                                return self.error(ErrorMessage::NonStaticNative, &node);
                            }
                        }
                    }
                    Some(ref l) if l == "FieldDeclaration" &&
                                   node.children[0].clone().has_child_kind(&TokenKind::Final) &&
                                   node.children[2].clone().children.len() != 3 => {
                        return self.error(ErrorMessage::FinalNoInit, &node);
                    }
                    Some(ref l) if l == "ClassDeclaration" => {
                        for (idx, child) in node.children.iter().enumerate() {
                            if child.token.kind == TokenKind::Class {
                                match node.children[idx + 1].clone().token.lexeme {
                                    Some(ref lexeme) if format!("{}.java", lexeme) !=
                                                        self.filename => {
                                        return self.error(ErrorMessage::ClassBadName, &node);
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
                                    Some(ref lexeme) if format!("{}.java", lexeme) !=
                                                        self.filename => {
                                        return self.error(ErrorMessage::InterfaceBadName, &node);
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
