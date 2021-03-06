use std::fmt;

/// Contains every error message generated by the compiler throughout each
/// layer of the juicyj stack.
pub enum ErrorMessage {
    /// Abstract methods may not have bodies.
    AbstractBody,
    /// Char literals may not contain literal newlines.
    CharNewline,
    /// Char literals may only be of length one.
    CharTooLong,
    /// Char literals may only be of length one. Since the octal literal eg.
    /// '\21' would count, we can get a bit more context as to why the char
    /// (might) be too long.
    CharTooLongOctal,
    /// Classes must have the same name as the file they are declared in.
    ClassBadName,
    /// Concrete classes must have bodies.
    ConcreteNoBody,
    /// Error reading file. Parameter: filename.
    CouldNotReadFile(String),
    /// Error running `reduce` function on parse stack.
    CouldNotReduceStack,
    /// Final methods can not be abstract.
    FinalAbstract,
    /// Final fields must have initializers.
    FinalNoInit,
    /// Single-Type import declarations must not clash.
    ImportClashSingleTogether,
    /// Interfaces must have the same name as the file they are declared in.
    InterfaceBadName,
    /// Integers must be within 32-bit range.
    IntOOB,
    /// Cannot cast to expression.
    InvalidCast,
    /// Only \b, \f, \n, \r, and \t are valid.
    InvalidEscape,
    /// Octals must be in range \0 - \399.
    InvalidOctal,
    /// Parse tree is only valid if it can be reduced to a single start symbol.
    InvalidParseTree,
    /// Root `ParseNode`s must be `package`, `import`, or `TypeDeclaration`
    InvalidRootChild,
    /// Catch-all for un-scannable tokens.
    InvalidToken,
    /// An AST must have a single canonical name identifier.
    MissingName,
    /// A single file may only contain one class.
    MultipleClasses,
    /// Native methods may not have a body.
    NativeBody,
    /// Non-static methods can not be native.
    NonStaticNative,
    /// Static methods cannot be abstract.
    StaticAbstract,
    /// Static methods cannot be final.
    StaticFinal,
    /// String literals may not contain literal newlines.
    StringNewline,
    /// Failure to parse Token (TokenKind) from String.
    StringNotToken(String),
    /// Catch-all for un-parseable tokens. Parameter: `format!`'ed token.
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
            ErrorMessage::ImportClashSingleTogether => {
                write!(f, "single-type-import declarations clash")
            }
            ErrorMessage::InterfaceBadName => write!(f, "interface is badly named"),
            ErrorMessage::IntOOB => write!(f, "integer out of bounds"),
            ErrorMessage::InvalidCast => write!(f, "invalid cast type"),
            ErrorMessage::InvalidEscape => write!(f, "invalid escape character"),
            ErrorMessage::InvalidOctal => write!(f, "invalid octal value"),
            ErrorMessage::InvalidParseTree => write!(f, "parse tree could not be entirely reduced"),
            ErrorMessage::InvalidRootChild => write!(f, "invalid child of root token"),
            ErrorMessage::InvalidToken => write!(f, "invalid token"),
            ErrorMessage::MissingName => write!(f, "missing ast name"),
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
