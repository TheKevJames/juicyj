use std;

use common::Token;
use common::TokenKind;
use lexer;
use parser::dfa;

pub struct Parser<T: Iterator<Item = Result<Token, lexer::LexerError>>> {
    nodes: Vec<Token>,
    states: Vec<dfa::Transition>,
    tokens: T,
}

impl<T: Iterator<Item = Result<Token, lexer::LexerError>>> Parser<T> {
    pub fn new(it: T) -> Parser<T> {
        Parser {
            nodes: Vec::new(),
            states: vec![dfa::Transition {
                             end_state: 0,
                             function: dfa::Function::Shift,
                             start_state: 0,
                             symbol: dfa::Symbol::new(dfa::Terminality::Any, "".to_string()),
                         }],
            tokens: it,
        }
    }

    fn reduce(&mut self, transition: dfa::Transition) {
        debug!("reducing: {:?}", transition);

        let previous = self.states.pop();

        self.nodes.push(transition.symbol.token);
    }

    fn shift(&mut self, transition: dfa::Transition) {
        debug!("shifting: {:?}", transition);

        self.states.push(transition.clone());

        if transition.symbol.terminality == dfa::Terminality::Terminal {
            self.nodes.push(transition.symbol.token);
        }
    }

    pub fn get_tree(&mut self) {
        let mut dfa = dfa::DFA::new();

        match dfa.consume(&Token {
            kind: TokenKind::BOF,
            lexeme: None,
        }) {
            Ok(transition) => {
                match transition.function {
                    dfa::Function::Reduce => self.reduce(transition),
                    dfa::Function::Shift => self.shift(transition),
                }
            }
            Err(e) => {
                error!("could not get BOF transition: {:?}", e);
            }
        }

        while let Some(tresult) = self.tokens.next() {
            let token = match tresult {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(42);
                }
            };

            match dfa.consume(&token) {
                Ok(transition) => {
                    match transition.function {
                        dfa::Function::Reduce => self.reduce(transition),
                        dfa::Function::Shift => self.shift(transition),
                    }
                }
                Err(e) => {
                    error!("could not get transition for {:?}: {:?}", token, e);
                }
            }
        }

        match dfa.consume(&Token {
            kind: TokenKind::EOF,
            lexeme: None,
        }) {
            Ok(transition) => {
                match transition.function {
                    dfa::Function::Reduce => self.reduce(transition),
                    dfa::Function::Shift => self.shift(transition),
                }
            }
            Err(e) => {
                error!("could not get EOF transition: {:?}", e);
            }
        }
    }
}
