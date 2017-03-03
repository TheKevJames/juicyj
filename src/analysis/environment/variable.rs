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
            kind: node.children[0].clone(),
            name: node.children[1].clone(),
            dim: node.children.len() == 3,
        }
    }
}

impl PartialEq for VariableEnvironment {
    fn eq(&self, other: &VariableEnvironment) -> bool {
        if self.dim != other.dim {
            return false;
        }

        if self.kind == other.kind {
            return true;
        }

        // TODO: is this enough for type lookup?
        if let Some(ref lexeme) = self.kind.token.lexeme {
            if self.kind.token.lexeme == other.kind.token.lexeme && lexeme == "Name" {
                return self.kind.children.last() == other.kind.children.last();
            }
        }

        false
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
