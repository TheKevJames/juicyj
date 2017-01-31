use std;

#[derive(Debug)]
pub struct LexerError {
    pub file: String,
    pub index: u32,
    pub line: String,
    pub line_number: u32,
    pub message: ErrorMessage,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = self.line_number.to_string().len() as u32;
        let pointer = (0..indent).map(|_| " ").collect::<String>() + "-->";
        let bar = (0..indent).map(|_| " ").collect::<String>() + " |";

        let indent = indent + self.index + 1;
        let carat = (0..indent).map(|_| " ").collect::<String>() + "^";
        write!(f,
               "error: {}\n{} {}\n{}\n{} | {}\n{}\n{}",
               self.message.message,
               pointer,
               self.file,
               bar,
               self.line_number,
               self.line,
               bar,
               carat)
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub arg: String,
    pub message: ErrorMessage,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {} ({})", self.message.message, self.arg)
    }
}

#[derive(Debug)]
pub struct ErrorMessage {
    pub message: &'static str,
}

pub const CHAR_NEWLINE: ErrorMessage = ErrorMessage { message: "char contains newline" };

pub const CHAR_TOO_LONG: ErrorMessage = ErrorMessage { message: "too many characters in char" };

pub const CHAR_TOO_LONG_OCTAL: ErrorMessage =
    ErrorMessage { message: "too many characters in char (maybe malformed octal?)" };

pub const COULD_NOT_READ_FILE: ErrorMessage = ErrorMessage { message: "could not read file" };

pub const INVALID_ESCAPE: ErrorMessage = ErrorMessage { message: "invalid escape character" };

pub const INVALID_FUNCTION: ErrorMessage = ErrorMessage { message: "invalid grammar function" };

pub const INVALID_OCTAL: ErrorMessage = ErrorMessage { message: "invalid octal" };

pub const INVALID_PARSE_TREE: ErrorMessage =
    ErrorMessage { message: "parse tree could not be entirely reduced" };

pub const INVALID_TOKEN: ErrorMessage = ErrorMessage { message: "unparseable token" };

pub const STRING_NEWLINE: ErrorMessage = ErrorMessage { message: "string contains newline" };

pub const STRING_NOT_TOKEN: ErrorMessage =
    ErrorMessage { message: "could not convert string to Token" };
