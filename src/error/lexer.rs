use std::fmt;

use error::message::ErrorMessage;

pub struct LexerError {
    pub file: String,
    pub index: u32,
    pub line: String,
    pub line_number: u32,
    pub message: ErrorMessage,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indent_bar = self.line_number.to_string().len() as u32;
        let indent_ptr = indent_bar + self.index + 1;

        let pointer = (0..indent_bar).map(|_| " ").collect::<String>() + "-->";
        let bar = (0..indent_bar).map(|_| " ").collect::<String>() + " |";
        let carat = (0..indent_ptr).map(|_| " ").collect::<String>() + " ^";

        write!(f,
               "error: {}\n{} {}\n{}\n{} | {}\n{}\n{}",
               self.message,
               pointer,
               self.file,
               bar,
               self.line_number,
               self.line,
               bar,
               carat)
    }
}
