use std::fmt;

use error::message::ErrorMessage;

pub struct WeederError {
    pub file: String,
    pub message: ErrorMessage,
    pub node: String,
}

impl WeederError {
    pub fn new<T>(file: String, message: ErrorMessage, node: &T) -> WeederError
        where T: fmt::Display
    {
        WeederError {
            file: file,
            message: message,
            node: format!("{}", node),
        }
    }
}

impl fmt::Display for WeederError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "error: {}\n --> {}\n\n{}",
               self.message,
               self.file,
               self.node)
    }
}
