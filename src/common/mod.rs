#[derive(Debug)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub lexeme: &'src str,
}

#[derive(Debug)]
pub enum TokenKind {
    Unknown,
}
