use std::fmt;

pub enum ErrorMessage {
    AbstractBody,
    CharNewline,
    CharTooLong,
    CharTooLongOctal,
    ClassBadName,
    ConcreteNoBody,
    CouldNotReadFile(String),
    CouldNotReduceStack,
    FinalAbstract,
    FinalNoInit,
    InterfaceBadName,
    IntOOB,
    InvalidCast,
    InvalidEscape,
    InvalidOctal,
    InvalidParseTree,
    InvalidRootChild,
    InvalidToken,
    MultipleClasses,
    NativeBody,
    NonStaticNative,
    StaticAbstract,
    StaticFinal,
    StringNewline,
    StringNotToken(String),
    UnparseableToken(String),
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorMessage::AbstractBody => write!(f, "abstract method has body"),
            ErrorMessage::CharNewline => write!(f, "char contains newline"),
            ErrorMessage::CharTooLong => write!(f, "too many character in char"),
            ErrorMessage::CharTooLongOctal => {
                write!(f, "too many characters in char (maybe malformed octal?)")
            }
            ErrorMessage::ClassBadName => write!(f, "class is badly named"),
            ErrorMessage::ConcreteNoBody => write!(f, "concrete method has no body"),
            ErrorMessage::CouldNotReadFile(ref filename) => {
                write!(f, "could not read file {}", filename)
            }
            ErrorMessage::CouldNotReduceStack => write!(f, "could not entirely reduce stack"),
            ErrorMessage::FinalAbstract => write!(f, "final method is abstract"),
            ErrorMessage::FinalNoInit => write!(f, "final field has no initializer"),
            ErrorMessage::InterfaceBadName => write!(f, "interface is badly named"),
            ErrorMessage::IntOOB => write!(f, "integer out of bounds"),
            ErrorMessage::InvalidCast => write!(f, "invalid cast type"),
            ErrorMessage::InvalidEscape => write!(f, "invalid escape character"),
            ErrorMessage::InvalidOctal => write!(f, "invalid octal value"),
            ErrorMessage::InvalidParseTree => write!(f, "parse tree could not be entirely reduced"),
            ErrorMessage::InvalidRootChild => write!(f, "invalid child of root token"),
            ErrorMessage::InvalidToken => write!(f, "invalid token"),
            ErrorMessage::MultipleClasses => write!(f, "multiple classes"),
            ErrorMessage::NativeBody => write!(f, "native method has body"),
            ErrorMessage::NonStaticNative => write!(f, "non-static method is native"),
            ErrorMessage::StaticAbstract => write!(f, "static method is abstract"),
            ErrorMessage::StaticFinal => write!(f, "static method is final"),
            ErrorMessage::StringNewline => write!(f, "string contains newline"),
            ErrorMessage::StringNotToken(ref value) => {
                write!(f, "could not convert string '{}' to token", value)
            }
            ErrorMessage::UnparseableToken(ref token) => write!(f, "unparseable token [{}]", token),
        }
    }
}
