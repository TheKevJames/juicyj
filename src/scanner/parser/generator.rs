use std;
use std::iter::Peekable;

use error::ErrorMessage;
use error::LexerError;
use error::ParserError;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::common::Function;
use scanner::parser::common::Terminality;
use scanner::parser::common::Transition;
use scanner::parser::dfa::DFA;
use scanner::parser::tree::ParseNode;
use scanner::parser::tree::ParseTree;

pub struct Parser<T: Iterator<Item = Result<Token, LexerError>>> {
    dfa: DFA,
    nodes: Vec<ParseNode>,
    states: Vec<usize>,
    token_state: u8,
    tokens: Peekable<T>,
}

// TODO: cleanup
impl<T: Iterator<Item = Result<Token, LexerError>>> Parser<T> {
    pub fn new(it: T) -> Parser<T> {
        let dfa = match DFA::new() {
            Ok(dfa) => dfa,
            Err(e) => {
                // TODO: get this out of here
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

    fn consume(&mut self, token: Token) -> Result<(), ParserError> {
        match self.dfa.consume(self.states.last().unwrap_or(&0), &token) {
            Ok(transition) => {
                let result = match transition.function {
                    Function::Reduce => self.reduce(transition, token),
                    Function::Shift => self.shift(transition, token),
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
                    // TODO: get this out of here
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

    fn reduce(&mut self, transition: Transition, token: Token) -> Result<(), ParserError> {
        let mut children: Vec<ParseNode> = Vec::new();
        let ref rule = self.dfa.rules[transition.value].clone();
        for _ in 0..rule.rhs.len() {
            self.states.pop();
            match self.nodes.pop() {
                Some(n) => children.insert(0, n),
                _ => {
                    return Err(ParserError::new(ErrorMessage::CouldNotReduceStack,
                                                Some(format!("{}", rule))));
                }
            }
        }

        self.nodes.push(ParseNode {
            children: children,
            token: rule.lhs.token.clone(),
        });

        match self.consume(rule.lhs.token.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.consume(token) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn shift(&mut self, transition: Transition, ref token: Token) -> Result<(), ParserError> {
        self.states.push(transition.value);
        if transition.symbol.terminality == Terminality::Terminal {
            self.nodes.push(ParseNode {
                children: Vec::new(),
                token: token.clone(),
            });
        }

        Ok(())
    }

    pub fn get_tree(&mut self) -> Result<ParseTree, ParserError> {
        while let Some(token) = self.peek() {
            match self.dfa.consume(self.states.last().unwrap_or(&0), &token) {
                Ok(transition) => {
                    let result = match transition.function {
                        Function::Reduce => self.reduce(transition, token.clone()),
                        Function::Shift => self.shift(transition, token.clone()),
                    };
                    match result {
                        Ok(_) => (),
                        Err(e) => return Err(e.with_nodes(self.nodes.clone())),
                    }
                    if token.kind != TokenKind::BOF {
                        self.tokens.next();
                    }
                }
                Err(e) => return Err(e.with_nodes(self.nodes.clone())),
            }
        }

        // TODO: one more manual reduce step?
        match self.nodes.len() {
            3 => Ok(ParseTree { root: self.nodes[1].clone() }),
            _ => {
                Err(ParserError::new(ErrorMessage::InvalidParseTree, None)
                    .with_nodes(self.nodes.clone()))
            }
        }
    }
}
