use std::fmt;

use error::ASTError;
use error::ErrorMessage;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::ParseNode;

#[derive(Clone,Debug)]
/// An individual node in the abstract syntax tree.
pub struct ASTNode {
    /// the associated token object for this node
    pub token: Token,
    /// all children of this node, ordered left-to-right
    pub children: Vec<ASTNode>,
}

pub struct ASTNodeImport {
    pub import: Vec<Token>,
}

pub struct ASTNodePackage {
    pub package: Vec<Token>,
}

impl ASTNode {
    /// Transforms a ParseNode to an ASTNode by simplifying tree structure
    /// wherever possible. Catches some syntax errors as the tree becomes simple
    /// enough to operate on.
    // TODO: cleanup
    pub fn new(node: &ParseNode) -> Result<ASTNode, ASTError> {
        match node.token.kind {
            TokenKind::NonTerminal => {
                match node.token.lexeme {
                    Some(ref l) if node.children.len() == 4 && l == "CastExpression" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }

                        if node.children[1].token.lexeme == Some("Expression".to_string()) {
                            match children[1].token.kind {
                                TokenKind::Dot => (),
                                TokenKind::Identifier => (),
                                TokenKind::NonTerminal => {
                                    if children[1].token.lexeme != Some("Name".to_string()) {
                                        return Err(ASTError::new(ErrorMessage::InvalidCast,
                                                                 &children[1]));
                                    }
                                }
                                _ => {
                                    return Err(ASTError::new(ErrorMessage::InvalidCast,
                                                             &children[1]));
                                }
                            }

                            let mut nodes: Vec<ParseNode> = Vec::new();
                            node.children[1].collect_child_lexeme("PrimaryNoNewArray", &mut nodes);
                            for n in &nodes {
                                if n.children.len() != 1 {
                                    return Err(ASTError::new(ErrorMessage::InvalidCast, n));
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
                        match ASTNode::new(&node.children[0]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        match ASTNode::new(&node.children[2]) {
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
                        match ASTNode::new(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }
                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 3 && l == "PrimaryNoNewArray" => {
                        ASTNode::new(&node.children[1])
                    }
                    Some(ref l) if node.children.len() == 3 && l == "QualifiedName" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
                                Ok(child) => {
                                    match child.token.lexeme {
                                        Some(ref l) if l == "QualifiedName" || l == "Name" => {
                                            for grandkid in child.children {
                                                children.push(grandkid);
                                            }
                                        }
                                        _ => children.push(child),
                                    }
                                }
                                Err(e) => return Err(e),
                            }
                        }

                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
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
                        match ASTNode::new(&node.children[1]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.children[0].token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if node.children.len() == 2 && l == "AbstractMethodDeclaration" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[0]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(_) if node.children.len() == 2 &&
                               node.children[1].token.kind == TokenKind::Semicolon => {
                        ASTNode::new(&node.children[0])
                    }
                    // TODO: does this miss the following case?
                    // CastExpression:
                    //     ( Name Dim ) UnaryNoSignExpression

                    // parent of UnaryExpression
                    Some(ref l) if node.children.len() == 1 && l == "MultiplicativeExpression" => {
                        match ASTNode::new(&node.children[0]) {
                            Ok(node) => {
                                if node.token.kind == TokenKind::NumValue {
                                    match node.token.lexeme {
                                        Some(ref l) if l.parse().unwrap_or(0) >
                                                       2u64.pow(31) - 1 => {
                                            return Err(ASTError::new(ErrorMessage::IntOOB, &node));
                                        }
                                        _ => (),
                                    }
                                }

                                Ok(node)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Some(ref l) if node.children.len() == 1 && l == "Name" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        match ASTNode::new(&node.children[0]) {
                            Ok(child) => children.push(child),
                            Err(e) => return Err(e),
                        }

                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(ref l) if l == "Modifiers" => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
                                Ok(child) => children.push(child),
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(ASTNode {
                            token: node.token.clone(),
                            children: children,
                        })
                    }
                    Some(_) if node.children.len() == 1 => ASTNode::new(&node.children[0]),
                    _ => {
                        let mut children: Vec<ASTNode> = Vec::new();
                        for child in &node.children {
                            match ASTNode::new(&child) {
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

    /// Convenience function for recursive ASTNode printing. Should be accessed
    /// with the standard print command (ie. `fmt::Display`).
    pub fn print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        match indent {
            0 => try!(write!(f, "{:width$}{}", "", self.token, width = indent)),
            _ => try!(write!(f, "{:width$}{}", "\n", self.token, width = indent)),
        }
        for child in &self.children {
            try!(child.print(f, indent + 2));
        }
        Ok(())
    }
}

impl ASTNodeImport {
    pub fn new(node: &ParseNode) -> Result<ASTNodeImport, ASTError> {
        let mut names: Vec<Token> = Vec::new();
        node.collect_child_kinds(&vec![&TokenKind::Identifier, &TokenKind::Star], &mut names);

        Ok(ASTNodeImport { import: names })
    }
}

impl ASTNodePackage {
    pub fn new(node: &ParseNode) -> Result<ASTNodePackage, ASTError> {
        let mut names: Vec<Token> = Vec::new();
        node.children[1].collect_child_kinds(&vec![&TokenKind::Identifier], &mut names);

        Ok(ASTNodePackage { package: names })
    }
}

impl PartialEq for ASTNode {
    fn eq(&self, other: &ASTNode) -> bool {
        (self.token == other.token && self.children == other.children) ||
        (self.children.len() == 1 && &self.children[0] == other) ||
        (other.children.len() == 1 && &other.children[0] == self)
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f, 0)
    }
}

impl fmt::Display for ASTNodeImport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for t in &self.import {
            try!(write!(f, "{} ", t));
        }
        Ok(())
    }
}

impl fmt::Display for ASTNodePackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for t in &self.package {
            try!(write!(f, "{} ", t));
        }
        Ok(())
    }
}
