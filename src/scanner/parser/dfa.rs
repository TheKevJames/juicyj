use std;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;

use scanner::common::error;
use scanner::common::Token;
use scanner::common::TokenKind;

#[derive(Debug,Clone)]
pub enum Function {
    Reduce,
    Shift,
}

#[derive(Debug,Clone)]
pub struct Rule {
    pub lhs: Symbol, // NonTerminal
    pub rhs: Vec<Symbol>,
}

#[derive(Debug)]
pub struct State {
    state: usize,
    transitions: Vec<Transition>,
}

impl State {
    fn new(s: usize) -> State {
        State {
            transitions: Vec::new(),
            state: s,
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Terminality {
    NonTerminal,
    Terminal,
}

#[derive(Debug,Clone)]
pub struct Symbol {
    pub terminality: Terminality,
    pub token: Token,
}

impl Symbol {
    pub fn new(terminality: Terminality, value: String) -> Result<Symbol, error::ParserError> {
        match value.parse() {
            Ok(kind) => {
                Ok(Symbol {
                    terminality: terminality,
                    token: Token {
                        kind: kind,
                        lexeme: Some(value),
                    },
                })
            }
            Err(_) => {
                Err(error::ParserError {
                    arg: value,
                    message: error::STRING_NOT_TOKEN,
                })
            }
        }
    }

    fn new_from_terminals(ref kinds_terminal: &Vec<TokenKind>,
                          value: String)
                          -> Result<Symbol, error::ParserError> {
        let kind = match value.parse() {
            Ok(ref kind) if kinds_terminal.contains(kind) => Terminality::Terminal,
            Ok(_) => Terminality::NonTerminal,
            _ => {
                return Err(error::ParserError {
                    arg: value,
                    message: error::STRING_NOT_TOKEN,
                })
            }
        };

        Symbol::new(kind, value)
    }
}

#[derive(Debug,Clone)]
pub struct Transition {
    pub value: usize,
    pub function: Function,
    pub start_state: usize,
    pub symbol: Symbol,
}

#[derive(Debug)]
pub struct DFA {
    pub non_terminals: Vec<Symbol>, // NonTerminal
    pub terminals: Vec<Symbol>, // Terminal
    pub rules: Vec<Rule>,
    pub start: Symbol,
    pub states: Vec<State>,
}

impl DFA {
    pub fn new() -> Result<DFA, error::ParserError> {
        let filename = "grammar/joos.lr1";
        let mut file = match File::open(filename) {
            Ok(file) => BufReader::new(file),
            Err(_) => {
                return Err(error::ParserError {
                    arg: filename.to_string(),
                    message: error::COULD_NOT_READ_FILE,
                })
            }
        };

        let mut non_terminals = Vec::<Symbol>::new();
        let mut terminals = Vec::<Symbol>::new();
        let mut rules = Vec::<Rule>::new();
        let mut states = Vec::<State>::new();

        let num_terminals: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_terminals {
            let symbol = file.by_ref().lines().next().unwrap().unwrap();
            match Symbol::new(Terminality::Terminal, symbol) {
                Ok(s) => terminals.push(s),
                Err(e) => return Err(e),
            }
        }

        let num_non_terminals: u32 =
            file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_non_terminals {
            let symbol = file.by_ref().lines().next().unwrap().unwrap();
            match Symbol::new(Terminality::NonTerminal, symbol) {
                Ok(s) => non_terminals.push(s),
                Err(e) => return Err(e),
            }
        }

        let kinds_terminal: Vec<TokenKind> =
            terminals.clone().into_iter().map(|t| t.token.kind).collect::<Vec<TokenKind>>();

        let start =
            match Symbol::new_from_terminals(&kinds_terminal,
                                             file.by_ref().lines().next().unwrap().unwrap()) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

        let num_rules: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_rules {
            let rule = file.by_ref().lines().next().unwrap().unwrap();
            let mut sides = rule.splitn(2, " ");

            let lhs = match Symbol::new(Terminality::NonTerminal,
                                        sides.next().unwrap().to_string()) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            let rhs = match sides.next() {
                Some(side) => {
                    side.split_whitespace()
                        .map(|s| match Symbol::new_from_terminals(&kinds_terminal, s.to_string()) {
                            Ok(s) => s,
                            Err(e) => {
                                println!("{}", e);
                                std::process::exit(42);
                            }
                        })
                        .collect()
                }
                _ => Vec::new(),
            };
            rules.push(Rule {
                lhs: lhs,
                rhs: rhs,
            });
        }

        let num_states: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for i in 0..num_states {
            states.push(State::new(i as usize));
        }

        let num_transitions: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_transitions {
            let transition = file.by_ref().lines().next().unwrap().unwrap();
            let mut tx = transition.split(" ");

            let start_state: usize = tx.next().unwrap().parse().unwrap();
            let symbol = tx.next().unwrap();
            let function = tx.next().unwrap();
            let value: usize = tx.next().unwrap().parse().unwrap();

            states[start_state].transitions.push(Transition {
                value: value,
                function: match function {
                    "reduce" => Function::Reduce,
                    "shift" => Function::Shift,
                    f => {
                        return Err(error::ParserError {
                            arg: f.to_string(),
                            message: error::INVALID_FUNCTION,
                        })
                    }
                },
                start_state: start_state,
                symbol: match Symbol::new_from_terminals(&kinds_terminal, symbol.to_string()) {
                    Ok(s) => s,
                    Err(e) => return Err(e),
                },
            });
        }

        Ok(DFA {
            non_terminals: non_terminals,
            rules: rules,
            start: start,
            states: states,
            terminals: terminals,
        })
    }

    pub fn consume(&self,
                   state: &usize,
                   ref token: &Token)
                   -> Result<Transition, error::ParserError> {
        let ref state = self.states[*state];
        let ref transitions = state.transitions;
        for transition in transitions {
            match token.kind {
                ref l if *l == transition.symbol.token.kind => {
                    if transition.symbol.token.kind != TokenKind::NonTerminal {
                        return Ok((*transition).clone());
                    }

                    if transition.symbol.token.lexeme == token.lexeme {
                        return Ok((*transition).clone());
                    }
                }
                _ => continue,
            }
        }

        Err(error::ParserError {
            arg: format!("{:?}", token),
            message: error::INVALID_TOKEN,
        })
    }
}
