use std::fmt;

use error::message::ErrorMessage;

pub struct WeederError {
    pub filename: String,
    pub message: ErrorMessage,
    /// format!'ed representation of nearby `ParseNode`s
    pub node: String,
}

impl WeederError {
    /// Constructs a WeederError from a filename, ErrorMessage, and node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use juicyj::error::ErrorMessage;
    /// use juicyj::error::WeederError;
    ///
    /// let filename = "Sample.java".to_owned();
    /// let node = "I am a node";  // Must be format!'able
    /// let error = WeederError::new(filename, ErrorMessage::IntOOB, &node);
    /// println!("{}", error);
    /// ```
    pub fn new<T>(filename: String, message: ErrorMessage, node: &T) -> WeederError
        where T: fmt::Display
    {
        WeederError {
            filename: filename,
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
               self.filename,
               self.node)
    }
}
