use std;
use std::iter::Peekable;

use common::Token;
use common::TokenKind;
use lexer;
use parser::dfa;
use parser::tree;

pub struct Parser<T: Iterator<Item = Result<Token, lexer::LexerError>>> {
    dfa: dfa::DFA,
    nodes: Vec<tree::Node>,
    states: Vec<usize>,
    token_state: u8,
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Result<Token, lexer::LexerError>>> Parser<T> {
    pub fn new(it: T) -> Parser<T> {
        Parser {
            dfa: dfa::DFA::new(),
            nodes: Vec::new(),
            states: vec![0],
            token_state: 0,
            tokens: it.peekable(),
        }
    }

    fn consume(&mut self, ref token: &Token) {
        match self.dfa.consume(self.states.last().unwrap_or(&0), token) {
            Ok(transition) => {
                match transition.function {
                    dfa::Function::Reduce => self.reduce(transition, token),
                    dfa::Function::Shift => self.shift(transition),
                }
            }
            Err(e) => {
                error!("could not get transition for {:?}: {:?}", token, e);
                std::process::exit(1);
            }
        }
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

    fn reduce(&mut self, transition: dfa::Transition, ref token: &Token) {
        let mut children: Vec<tree::Node> = Vec::new();
        let ref rule = self.dfa.rules[transition.value].clone();
        for _ in 0..rule.rhs.len() {
            self.states.pop();
            match self.nodes.pop() {
                Some(n) => children.push(n),
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

        self.consume(&rule.lhs.token.clone());
        self.consume(token);
    }

    fn shift(&mut self, transition: dfa::Transition) {
        self.states.push(transition.value);
        if transition.symbol.terminality == dfa::Terminality::Terminal {
            self.nodes.push(tree::Node {
                children: Vec::new(),
                token: transition.symbol.token,
            });
        }
    }

    pub fn get_tree(&mut self) -> tree::Tree {
        while let Some(token) = self.peek() {
            match self.dfa.consume(self.states.last().unwrap_or(&0), &token) {
                Ok(transition) => {
                    match transition.function {
                        dfa::Function::Reduce => self.reduce(transition, &token),
                        dfa::Function::Shift => self.shift(transition),
                    }
                    self.tokens.next();
                }
                Err(e) => {
                    error!("could not get transition for {:?}: {:?}", token, e);
                    std::process::exit(42);
                }
            }
        }

        if self.nodes.len() != 3 {
            error!("parse tree could not be reduced to Start");
            std::process::exit(42);
        }
        tree::Tree { root: self.nodes[1].clone() }
    }
}
