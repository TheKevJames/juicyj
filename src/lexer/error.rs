use std;

pub struct LexerError {
    pub file: String,
    pub index: u32,
    pub line: String,
    pub line_number: u32,
    pub message: &'static str,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = self.index.to_string().len() as u32 + 1;
        let pointer = (0..indent).map(|_| " ").collect::<String>() + "-->";
        let bar = (0..indent).map(|_| " ").collect::<String>() + " |";

        let indent = indent + self.index + 1;
        let carat = (0..indent).map(|_| " ").collect::<String>() + "^";
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
