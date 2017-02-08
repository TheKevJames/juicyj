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

// TODO: Display
impl ASTNode {
    pub fn print(&self, indent: u32) {
        let spaces = (0..indent).map(|_| " ").collect::<String>();
        println!("{}{}", spaces, self.token);

        for child in self.children.clone() {
            child.print(indent + 2);
        }
    }
}
