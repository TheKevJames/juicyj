use std;

#[derive(Debug)]
pub struct LexerError {
    pub file: String,
    pub index: u32,
    pub line: String,
    pub line_number: u32,
    pub message: LexerErrorMessage,
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
pub struct LexerErrorMessage {
    pub message: &'static str,
}

pub const CHAR_NEWLINE: LexerErrorMessage = LexerErrorMessage { message: "char contains newline" };

pub const CHAR_TOO_LONG: LexerErrorMessage =
    LexerErrorMessage { message: "too many characters in char" };

pub const CHAR_TOO_LONG_OCTAL: LexerErrorMessage =
    LexerErrorMessage { message: "too many characters in char (maybe malformed octal?)" };

pub const INVALID_ESCAPE: LexerErrorMessage =
    LexerErrorMessage { message: "invalid escape character" };

pub const INVALID_OCTAL: LexerErrorMessage = LexerErrorMessage { message: "invalid octal" };

pub const INVALID_TOKEN: LexerErrorMessage = LexerErrorMessage { message: "unparseable token" };

pub const STRING_NEWLINE: LexerErrorMessage =
    LexerErrorMessage { message: "string contains newline" };
