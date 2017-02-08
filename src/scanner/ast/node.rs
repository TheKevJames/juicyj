use std::fmt;

use scanner::common::error;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::Node;

#[derive(Clone)]
pub struct ASTNode {
    pub token: Token,
    pub children: Vec<ASTNode>,
}

pub struct ASTNodeImport {
    pub import: Vec<Token>,
}

pub struct ASTNodePackage {
    pub package: Vec<Token>,
}

impl ASTNode {
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
    pub fn new(node: &Node) -> Result<ASTNodeImport, error::ASTError> {
        let mut names: Vec<Token> = Vec::new();
        node.collect_child_kinds(&vec![&TokenKind::Identifier, &TokenKind::Star], &mut names);

        Ok(ASTNodeImport { import: names })
    }
}

impl ASTNodePackage {
    pub fn new(node: &Node) -> Result<ASTNodePackage, error::ASTError> {
        let mut names: Vec<Token> = Vec::new();
        node.children[1].collect_child_kinds(&vec![&TokenKind::Identifier], &mut names);

        Ok(ASTNodePackage { package: names })
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
