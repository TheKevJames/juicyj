#[derive(Debug)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub data: &'src str,
}

#[derive(Debug)]
pub enum TokenKind {
    Unknown,
}

pub struct Lexer<'src> {
    src: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &str) -> Lexer {
        return Lexer {
            src: src,
        };
    }

    pub fn tokenize(&self) -> Vec<Token> {
        return self.src
            .split_whitespace()
            .map(|t| {
                Token {
                    data: t,
                    kind: TokenKind::Unknown,
                }
            })
            .collect();
    }
}
