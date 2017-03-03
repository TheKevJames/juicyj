use std::fmt;

use scanner::ASTNode;

#[derive(Clone,Debug)]
pub struct VariableEnvironment {
    pub kind: ASTNode,
    pub name: ASTNode,
    pub dim: bool,
}

impl VariableEnvironment {
    pub fn new(node: ASTNode) -> VariableEnvironment {
        VariableEnvironment {
            kind: node.children[0].clone().flatten().clone(),
            name: node.children[1].clone(),
            dim: node.children.len() == 3,
        }
    }
}

impl PartialEq for VariableEnvironment {
    fn eq(&self, other: &VariableEnvironment) -> bool {
        self.kind == other.kind && self.dim == other.dim
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
