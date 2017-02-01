use std;
use std::iter::Peekable;

use common::error;
use common::Token;
use common::TokenKind;
use parser::dfa;
use parser::tree;

pub struct Parser<T: Iterator<Item = Result<Token, error::LexerError>>> {
    dfa: dfa::DFA,
    nodes: Vec<tree::Node>,
    states: Vec<usize>,
    token_state: u8,
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Result<Token, error::LexerError>>> Parser<T> {
    pub fn new(it: T) -> Parser<T> {
        let dfa = match dfa::DFA::new() {
            Ok(dfa) => dfa,
            Err(e) => {
                error!("could not create DFA");
                error!("{}", e);
                std::process::exit(1);
            }
        };

        Parser {
            dfa: dfa,
            nodes: Vec::new(),
            states: vec![0],
            token_state: 0,
            tokens: it.peekable(),
        }
    }

    fn consume(&mut self, token: Token) -> Result<(), error::ParserError> {
        match self.dfa.consume(self.states.last().unwrap_or(&0), &token) {
            Ok(transition) => {
                let result = match transition.function {
                    dfa::Function::Reduce => self.reduce(transition, token),
                    dfa::Function::Shift => self.shift(transition, token),
                };
                match result {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn peek(&mut self) -> Option<Token> {
        if self.token_state == 0 {
            self.token_state += 1;
            Some(Token {
                kind: TokenKind::BOF,
                lexeme: None,
            })
        } else {
            match self.tokens.peek() {
                Some(&Ok(ref t)) => Some(t.clone()),
                Some(&Err(ref e)) => {
                    println!("{}", e);
                    std::process::exit(42);
                }
                _ => {
                    if self.token_state == 1 {
                        self.token_state += 1;
                        Some(Token {
                            kind: TokenKind::EOF,
                            lexeme: None,
                        })
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn reduce(&mut self,
              transition: dfa::Transition,
              token: Token)
              -> Result<(), error::ParserError> {
        let mut children: Vec<tree::Node> = Vec::new();
        let ref rule = self.dfa.rules[transition.value].clone();
        for _ in 0..rule.rhs.len() {
            self.states.pop();
            match self.nodes.pop() {
                Some(n) => children.insert(0, n),
                _ => {
                    error!("could not reduce entire rule {:?}", rule);
                    std::process::exit(1);
                }
            }
        }

        self.nodes.push(tree::Node {
            children: children,
            token: rule.lhs.token.clone(),
        });

        match self.consume(rule.lhs.token.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.consume(token) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn shift(&mut self,
             transition: dfa::Transition,
             ref token: Token)
             -> Result<(), error::ParserError> {
        self.states.push(transition.value);
        if transition.symbol.terminality == dfa::Terminality::Terminal {
            self.nodes.push(tree::Node {
                children: Vec::new(),
                token: token.clone(),
            });
        }

        Ok(())
    }

    pub fn get_tree(&mut self) -> Result<tree::Tree, error::ParserError> {
        while let Some(token) = self.peek() {
            match self.dfa.consume(self.states.last().unwrap_or(&0), &token) {
                Ok(transition) => {
                    let result = match transition.function {
                        dfa::Function::Reduce => self.reduce(transition, token.clone()),
                        dfa::Function::Shift => self.shift(transition, token.clone()),
                    };
                    match result {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                    if token.kind != TokenKind::BOF {
                        self.tokens.next();
                    }
                }
                Err(e) => {
                    for node in self.nodes.clone() {
                        node.print(0);
                    }
                    println!("Last known state: {}", self.states.last().unwrap_or(&0));
                    return Err(e);
                }
            }
        }

        // TODO: one more manual reduce step?
        match self.nodes.len() {
            3 => Ok(tree::Tree { root: self.nodes[1].clone() }),
            _ => {
                Err(error::ParserError {
                    arg: "Start".to_string(),
                    message: error::INVALID_PARSE_TREE,
                })
            }
        }
    }
}
