use std::fmt;

use scanner::ASTNode;

#[derive(Clone,Debug,PartialEq)]
pub struct ParameterEnvironment {
    pub kind: ASTNode,
    pub name: ASTNode,
    pub dim: bool,
}

impl ParameterEnvironment {
    pub fn new(node: ASTNode) -> ParameterEnvironment {
        ParameterEnvironment {
            kind: node.children[0].clone(),
            name: node.children[1].clone(),
            dim: node.children.len() == 3,
        }
    }
}

impl fmt::Display for ParameterEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", self.kind, self.name));
        if self.dim {
            try!(write!(f, "[]"));
        }
        Ok(())
    }
}

// ??
// impl PartialEq for ParameterEnvironment {
//     fn eq(&self, other: &ParameterEnvironment) -> bool {
//         self.kind == other.kind && self.dim == other.dim
//     }
// }
