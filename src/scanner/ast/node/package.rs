use std::fmt;

use scanner::common::Token;

pub struct ASTNodePackage {
    pub package: Vec<Token>,
}

impl fmt::Display for ASTNodePackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               self.package
                   .clone()
                   .into_iter()
                   .map(|t| format!("{}", t))
                   .collect::<Vec<String>>()
                   .join(" "))
    }
}
