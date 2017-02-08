use std;

mod common;

mod ast;
mod lexer;
mod parser;
mod weeder;

use self::ast::AST;
use self::lexer::Lexer;
use self::parser::Parser;
use self::weeder::Weeder;

pub fn scan_or_exit(file: &str, src: &str) {
    let lexer = Lexer::new(&file, &src);

    let mut parser = Parser::new(lexer);
    let parse_tree = match parser.get_tree() {
        Ok(pt) => pt,
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    };

    let mut weeder = Weeder::new(&file, &parse_tree);
    match weeder.verify(None) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    }

    match AST::new(&parse_tree) {
        Ok(ast) => println!("{}", ast),
        Err(e) => {
            println!("{}", e);
            std::process::exit(42);
        }
    };
}

// TODO: this should be #[cfg(test)], but for some reason the test macros can't
// find this module in that case.
pub mod tests {
    use std;

    use super::ast::AST;
    use super::lexer::Lexer;
    use super::parser::Parser;
    use super::weeder::Weeder;

    pub fn scan_and_assert(file: &str, src: &str) {
        let lexer = Lexer::new(&file, &src);
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

        let mut weeder = Weeder::new(&file, &parse_tree);
        match weeder.verify(None) {
            Ok(_) => (),
            Err(_) => {
                println!("Weeder Verification Error");
                assert!(true);
                return;
            }
        }

        match AST::new(&parse_tree) {
            Ok(_) => {
                println!("No Error Found");
                assert!(false);
            }
            Err(_) => assert!(true),
        };
    }

    pub fn scan_or_assert(file: &str, src: &str) {
        let lexer = Lexer::new(&file, &src);
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

        let mut weeder = Weeder::new(&file, &parse_tree);
        match weeder.verify(None) {
            Ok(_) => (),
            Err(e) => {
                println!("Weeder Verification Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        }

        match AST::new(&parse_tree) {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("AST Construction Error");
                println!("{}", e);
                assert!(false);
                std::process::exit(1);
            }
        };
    }
}
