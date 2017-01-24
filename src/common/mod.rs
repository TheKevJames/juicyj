#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
}

#[derive(Debug)]
pub enum TokenKind {
    Assignment,
    Equality,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Not,
    NotEqual,

    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,

    Dot,
    FSlash,
    Minus,
    Percent,
    Plus,
    Star,

    And,
    Or,
    BitAnd,
    BitOr,

    Import,
    Package,

    Private,
    Protected,
    Public,

    Abstract,
    Extends,
    Implements,
    Interface,
    Final,
    Native,
    Static,

    Boolean,
    Byte,
    Char,
    Int,
    Short,
    Str,
    Void,

    False,
    True,

    Class,
    Delete,
    Instanceof,
    New,
    Object,
    This,

    Else,
    For,
    If,
    Return,
    While,

    Identifier,
    CharValue,
    NumValue,
    StrValue,

    Semicolon,
}
