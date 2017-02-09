//! An error module for juicyj. Contains individual error implementations for
//! every layer of the compiler as well as a global enum for error messages.
mod message;

mod ast;
mod lexer;
mod parser;
mod weeder;

pub use self::message::ErrorMessage;

pub use self::ast::ASTError;
pub use self::lexer::LexerError;
pub use self::parser::ParserError;
pub use self::weeder::WeederError;
