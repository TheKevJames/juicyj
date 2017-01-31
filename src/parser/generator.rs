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
        let real_tokens = self.tokens.map(|t| {
            match t {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(42);
                }
            }
        }).collect::<Vec<Token>>();
        debug!("got tokens {:?}", real_tokens);

        let dfa = dfa::DFA::new();
        debug!("got dfa {:?}", dfa);
    }
}
