use std::fmt;

use scanner::common::Token;

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
        for child in self.children.clone() {
            try!(child.print(f, indent + 2));
        }
        Ok(())
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
