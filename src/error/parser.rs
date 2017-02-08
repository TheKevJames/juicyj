use std::fmt;

use error::message::ErrorMessage;

pub struct ParserError {
    pub arg: Option<String>,
    pub message: ErrorMessage,
    pub nodes: Option<String>,
}

impl ParserError {
    pub fn new(message: ErrorMessage, arg: Option<String>) -> ParserError {
        ParserError {
            arg: arg,
            message: message,
            nodes: None,
        }
    }

    pub fn with_nodes<T>(mut self, nodes: T) -> ParserError
        where T: IntoIterator,
              <T as IntoIterator>::Item: fmt::Display
    {
        self.nodes =
            Some(nodes.into_iter().map(|n| format!("{}", n)).collect::<Vec<_>>().join("\n"));
        self
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.arg {
            Some(ref r) => {
                match self.nodes {
                    Some(ref n) => write!(f, "error: {}\n{}\n\n{}", self.message, r, n),
                    _ => write!(f, "error: {}\n{}", self.message, r),
                }
            }
            _ => {
                match self.nodes {
                    Some(ref n) => write!(f, "error: {}\n\n{}", self.message, n),
                    _ => write!(f, "error: {}", self.message),
                }
            }
        }
    }
}
