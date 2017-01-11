use common::Token;
use common::TokenKind;

pub struct Lexer<'src> {
    src: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &str) -> Lexer {
        Lexer { src: src }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        self.src
            .split_whitespace()
            .map(|t| {
                Token {
                    kind: TokenKind::Unknown,
                    lexeme: t,
                }
            })
            .collect()
    }
}
