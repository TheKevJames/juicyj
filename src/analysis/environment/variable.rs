use std::fmt;

use scanner::ASTNode;

#[derive(Clone,Debug,PartialEq)]
pub struct VariableEnvironment {
    pub kind: ASTNode,
    pub name: ASTNode,
    pub dim: bool,
}

impl VariableEnvironment {
    pub fn new(node: ASTNode) -> VariableEnvironment {
        VariableEnvironment {
            kind: node.children[0].clone(),
            name: node.children[1].clone(),
            dim: node.children.len() == 3,
        }
    }
}

impl fmt::Display for VariableEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", self.kind, self.name));
        if self.dim {
            try!(write!(f, "[]"));
        }
        Ok(())
    }
}
