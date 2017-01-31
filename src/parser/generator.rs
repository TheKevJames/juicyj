use std;

use common::Token;
use lexer;
use parser::dfa;

pub struct Parser<T: Iterator<Item = Result<Token, lexer::LexerError>>> {
    tokens: T,
}

impl<T: Iterator<Item = Result<Token, lexer::LexerError>>> Parser<T> {
    pub fn new(it: T) -> Parser<T> {
        Parser { tokens: it }
    }

    pub fn get_tree(self) {
        let mut dfa = dfa::DFA::new();

        for tresult in self.tokens {
            let token = match tresult {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(42);
                }
            };

            dfa.consume(token);
        }
    }
}
