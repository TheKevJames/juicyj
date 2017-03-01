// TODO: global tokens module
use std::fmt;
use std::str::FromStr;

#[derive(Clone,Debug,PartialEq)]
#[allow(missing_docs)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
}

#[allow(missing_docs)]
impl Token {
    pub fn new(kind: TokenKind, lexeme: Option<&'static str>) -> Token {
        Token {
            kind: kind,
            lexeme: match lexeme {
                Some(l) => Some(l.to_owned()),
                _ => None,
            },
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.lexeme {
            Some(ref l) => {
                match self.kind {
                    TokenKind::NonTerminal => write!(f, "<{}>", l),
                    _ => write!(f, "{} [{:?}]", l, self.kind),
                }
            }
            _ => write!(f, "{:?}", self.kind),
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
#[allow(missing_docs)]
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
    Void,

    False,
    True,
    Null,

    Class,
    Delete,
    Instanceof,
    New,
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

    BOF,
    EOF,

    // INVALID
    AssignmentAddition,
    AssignmentAnd,
    AssignmentDivision,
    AssignmentLShift,
    AssignmentModulus,
    AssignmentMultiplication,
    AssignmentOr,
    AssignmentRRShift,
    AssignmentRShift,
    AssignmentSubtraction,
    AssignmentXor,
    Break,
    Case,
    Catch,
    Colon,
    Complement,
    Continue,
    Decrement,
    Default,
    Do,
    Double,
    Finally,
    Float,
    Goto,
    Increment,
    Long,
    LShift,
    Question,
    RRShift,
    RShift,
    Strictfp,
    Super,
    Switch,
    Synchronized,
    Throw,
    Throws,
    Transient,
    Try,
    Volatile,

    // Parsing
    NonTerminal,
}

impl FromStr for TokenKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "!" => Ok(TokenKind::Not),
            "!=" => Ok(TokenKind::NotEqual),
            "%" => Ok(TokenKind::Percent),
            "%=" => Ok(TokenKind::AssignmentModulus),
            "&" => Ok(TokenKind::BitAnd),
            "&&" => Ok(TokenKind::And),
            "&=" => Ok(TokenKind::AssignmentAnd),
            "(" => Ok(TokenKind::LParen),
            ")" => Ok(TokenKind::RParen),
            "*" => Ok(TokenKind::Star),
            "*=" => Ok(TokenKind::AssignmentMultiplication),
            "+" => Ok(TokenKind::Plus),
            "++" => Ok(TokenKind::Increment),
            "+=" => Ok(TokenKind::AssignmentAddition),
            "-" => Ok(TokenKind::Minus),
            "--" => Ok(TokenKind::Decrement),
            "-=" => Ok(TokenKind::AssignmentSubtraction),
            "/" => Ok(TokenKind::FSlash),
            "/=" => Ok(TokenKind::AssignmentDivision),
            "<" => Ok(TokenKind::LessThan),
            "<<" => Ok(TokenKind::LShift),
            "<<=" => Ok(TokenKind::AssignmentLShift),
            "<=" => Ok(TokenKind::LessThanOrEqual),
            "=" => Ok(TokenKind::Assignment),
            "==" => Ok(TokenKind::Equality),
            ">" => Ok(TokenKind::GreaterThan),
            ">=" => Ok(TokenKind::GreaterThanOrEqual),
            ">>" => Ok(TokenKind::RShift),
            ">>=" => Ok(TokenKind::AssignmentRShift),
            ">>>" => Ok(TokenKind::RRShift),
            ">>>=" => Ok(TokenKind::AssignmentRRShift),
            "?" => Ok(TokenKind::Question),
            "[" => Ok(TokenKind::LBracket),
            "]" => Ok(TokenKind::RBracket),
            "^" => Ok(TokenKind::BitXor),
            "^=" => Ok(TokenKind::AssignmentXor),
            "ABSTRACT" => Ok(TokenKind::Abstract),
            "BOF" => Ok(TokenKind::BOF),
            "BOOLEAN" => Ok(TokenKind::Boolean),
            "BREAK" => Ok(TokenKind::Break),
            "BYTE" => Ok(TokenKind::Byte),
            "CASE" => Ok(TokenKind::Case),
            "CATCH" => Ok(TokenKind::Catch),
            "CHAR" => Ok(TokenKind::Char),
            "CharacterLit" => Ok(TokenKind::CharValue),
            "CLASS" => Ok(TokenKind::Class),
            "COLON" => Ok(TokenKind::Colon),
            "COMMA" => Ok(TokenKind::Comma),
            "CONTINUE" => Ok(TokenKind::Continue),
            "DEFAULT" => Ok(TokenKind::Default),
            "DELETE" => Ok(TokenKind::Delete),
            "DO" => Ok(TokenKind::Do),
            "DOT" => Ok(TokenKind::Dot),
            "DOUBLE" => Ok(TokenKind::Double),
            "ELSE" => Ok(TokenKind::Else),
            "EOF" => Ok(TokenKind::EOF),
            "EXTENDS" => Ok(TokenKind::Extends),
            "FALSE" => Ok(TokenKind::False),
            "FINAL" => Ok(TokenKind::Final),
            "FINALLY" => Ok(TokenKind::Finally),
            "FLOAT" => Ok(TokenKind::Float),
            "FOR" => Ok(TokenKind::For),
            "GOTO" => Ok(TokenKind::Goto),
            "IDENTIFIER" => Ok(TokenKind::Identifier),
            "IF" => Ok(TokenKind::If),
            "IMPLEMENTS" => Ok(TokenKind::Implements),
            "IMPORT" => Ok(TokenKind::Import),
            "INSTANCEOF" => Ok(TokenKind::Instanceof),
            "INT" => Ok(TokenKind::Int),
            "IntegerLit" => Ok(TokenKind::NumValue),
            "INTERFACE" => Ok(TokenKind::Interface),
            "LONG" => Ok(TokenKind::Long),
            "NATIVE" => Ok(TokenKind::Native),
            "NEW" => Ok(TokenKind::New),
            "NullLit" => Ok(TokenKind::Null),
            "PACKAGE" => Ok(TokenKind::Package),
            "PRIVATE" => Ok(TokenKind::Private),
            "PROTECTED" => Ok(TokenKind::Protected),
            "PUBLIC" => Ok(TokenKind::Public),
            "RETURN" => Ok(TokenKind::Return),
            "SEMICOLON" => Ok(TokenKind::Semicolon),
            "SHORT" => Ok(TokenKind::Short),
            "STATIC" => Ok(TokenKind::Static),
            "STRICTFP" => Ok(TokenKind::Strictfp),
            "StringLit" => Ok(TokenKind::StrValue),
            "SUPER" => Ok(TokenKind::Super),
            "SWITCH" => Ok(TokenKind::Switch),
            "SYNCHRONIZED" => Ok(TokenKind::Synchronized),
            "THIS" => Ok(TokenKind::This),
            "THROW" => Ok(TokenKind::Throw),
            "THROWS" => Ok(TokenKind::Throws),
            "TRANSIENT" => Ok(TokenKind::Transient),
            "TRUE" => Ok(TokenKind::True),
            "TRY" => Ok(TokenKind::Try),
            "VOID" => Ok(TokenKind::Void),
            "VOLATILE" => Ok(TokenKind::Volatile),
            "WHILE" => Ok(TokenKind::While),
            "{" => Ok(TokenKind::LBrace),
            "|" => Ok(TokenKind::BitOr),
            "|=" => Ok(TokenKind::AssignmentOr),
            "||" => Ok(TokenKind::Or),
            "}" => Ok(TokenKind::RBrace),
            "~" => Ok(TokenKind::Complement),

            _ => Ok(TokenKind::NonTerminal),
        }
    }
}
