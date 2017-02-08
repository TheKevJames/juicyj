use std::fmt;

use scanner::common::Token;

pub struct ASTNodeImport {
    pub import: Vec<Token>,
}

impl fmt::Display for ASTNodeImport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               self.import
                   .clone()
                   .into_iter()
                   .map(|t| format!("{}", t))
                   .collect::<Vec<String>>()
                   .join(" "))
    }
}
