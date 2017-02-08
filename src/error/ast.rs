use std::fmt;

use error::message::ErrorMessage;

pub struct ASTError {
    pub message: ErrorMessage,
    pub node: String,
}

impl ASTError {
    pub fn new<T>(message: ErrorMessage, node: &T) -> ASTError
        where T: fmt::Display
    {
        ASTError {
            message: message,
            node: format!("{}", node),
        }
    }
}

impl fmt::Display for ASTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}\n\n{}", self.message, self.node)
    }
}
