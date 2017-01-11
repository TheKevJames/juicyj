#[derive(Debug)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub data: &'src str,
}

#[derive(Debug)]
pub enum TokenKind {
    Unknown,
}
