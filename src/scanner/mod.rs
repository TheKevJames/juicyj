//! The scanner module for juicyj. Includes the Lexer, Parser, Weeder, and AST
//! generator. Roughly corresponds to assignment 1 of the original CS444
//! project.
mod common;

mod ast;
mod lexer;
mod parser;
mod weeder;

use std;
use std::fs::File;
use std::io::Read;

pub use self::ast::AST;
pub use self::ast::ASTNode;
pub use self::ast::ASTNodeImport;
pub use self::common::Token;
pub use self::common::TokenKind;
use self::lexer::Lexer;
use self::parser::Parser;
use self::weeder::Weeder;

/// Convenience function for reading a file of a given name into a String.
///
/// # Examples
///
/// ```rust,no_run
/// let filename = "Sample.java";
/// let contents = juicyj::scanner::read_src_file(&filename.to_owned());
/// ```
pub fn read_src_file(filename: &String) -> String {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(_) => {
            error!("could not open file: {}", filename);
            std::process::exit(1);
        }
    };

    let mut src = String::new();
    match file.read_to_string(&mut src) {
        Ok(_) => {}
        Err(_) => {
            error!("could not read file {} to string", filename);
            std::process::exit(1);
        }
    };

    src
}

/// Runs a file through the scanning stack (lexer, parser, weeder, AST) and
/// exits with code 42 on a failure.
///
/// # Examples
///
/// ```rust,no_run
/// let filename = "Sample.java";
/// let contents = juicyj::scanner::read_src_file(&filename.to_owned());
/// let ast = juicyj::scanner::scan_or_exit(&filename, &contents);
/// ```
pub fn scan_or_exit(filename: &str, contents: &str) -> AST {
    let lexer = Lexer::new(&filename, &contents);

    let mut parser = Parser::new(lexer);
    let parse_tree = match parser.get_tree() {
        Ok(pt) => pt,
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    };

    let mut weeder = Weeder::new(&filename, &parse_tree);
    match weeder.verify(None) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    }

    match AST::new(&filename, &parse_tree) {
        Ok(ast) => ast,
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    }
}

// TODO: this should be #[cfg(test)], but for some reason the test macros can't
// find this module in that case.
#[allow(missing_docs)]
pub mod tests {
    use std;

    use super::ast::AST;
    use super::lexer::Lexer;
    use super::parser::Parser;
    use super::weeder::Weeder;

    pub fn scan_and_assert(filename: &str, src: &str) {
        let lexer = Lexer::new(&filename, &src);
        for token in lexer.clone().collect::<Vec<Result<_, _>>>() {
            match token {
                Ok(_) => (),
                Err(_) => {
                    println!("Lexer Error");
                    assert!(true);
                    return;
                }
            }
        }

        let mut parser = Parser::new(lexer);
        let parse_tree = match parser.get_tree() {
            Ok(pt) => pt,
            Err(_) => {
                println!("Parser Error");
                assert!(true);
                return;
            }
        };

        let mut weeder = Weeder::new(&filename, &parse_tree);
        match weeder.verify(None) {
            Ok(_) => (),
            Err(_) => {
                println!("Weeder Verification Error");
                assert!(true);
                return;
            }
        }

        match AST::new(&filename, &parse_tree) {
            Ok(_) => (),
            Err(_) => {
                println!("AST Construction Error");
                assert!(true);
                return;
            }
        };

        println!("No Error Found");
        assert!(false);
    }

    pub fn scan_or_assert(filename: &str, src: &str) -> AST {
        let lexer = Lexer::new(&filename, &src);
        for token in lexer.clone().collect::<Vec<Result<_, _>>>() {
            match token {
                Ok(_) => (),
                Err(e) => {
                    println!("Lexer Error");
                    println!("{}", e);
                    assert!(false);
                    std::process::exit(1);
                }
            }
        }

        let mut parser = Parser::new(lexer);
        let parse_tree = match parser.get_tree() {
            Ok(pt) => pt,
            Err(e) => {
                println!("Parser Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        };

        let mut weeder = Weeder::new(&filename, &parse_tree);
        match weeder.verify(None) {
            Ok(_) => (),
            Err(e) => {
                println!("Weeder Verification Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        }

        match AST::new(&filename, &parse_tree) {
            Ok(ast) => ast,
            Err(e) => {
                println!("AST Construction Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        }
    }
}
