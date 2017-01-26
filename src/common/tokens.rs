// use std;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
}

// impl std::fmt::Debug for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self.lexeme {
//             Some(ref l) => write!(f, "{}", l),
//             None => write!(f, "{:?}", self.kind),
//         }
//     }
// }

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
    BitXor,

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

    Comma,
    Semicolon,
}
