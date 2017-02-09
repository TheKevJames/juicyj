use std;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;

use error::ErrorMessage;
use error::ParserError;
use scanner::common::Token;
use scanner::common::TokenKind;
use scanner::parser::common::Function;
use scanner::parser::common::Rule;
use scanner::parser::common::State;
use scanner::parser::common::Symbol;
use scanner::parser::common::Terminality;
use scanner::parser::common::Transition;

pub struct DFA {
    pub non_terminals: Vec<Symbol>,
    pub terminals: Vec<Symbol>,
    pub rules: Vec<Rule>,
    pub start: Symbol,
    pub states: Vec<State>,
}

impl DFA {
    // TODO: cleanup
    pub fn new() -> DFA {
        let filename = "grammar/joos.lr1";
        let mut file = match File::open(filename) {
            Ok(file) => BufReader::new(file),
            Err(_) => {
                let error = ParserError::new(ErrorMessage::CouldNotReadFile(filename.to_string()),
                                             None);
                println!("{}", error);
                std::process::exit(1);
            }
        };

        let mut terminals = Vec::new();
        for _ in 0..file.by_ref().lines().next().unwrap().unwrap().parse().unwrap() {
            let symbol = file.by_ref().lines().next().unwrap().unwrap();
            match Symbol::new(Terminality::Terminal, symbol) {
                Ok(s) => terminals.push(s),
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
        }

        let mut non_terminals = Vec::new();
        for _ in 0..file.by_ref().lines().next().unwrap().unwrap().parse().unwrap() {
            let symbol = file.by_ref().lines().next().unwrap().unwrap();
            match Symbol::new(Terminality::NonTerminal, symbol) {
                Ok(s) => non_terminals.push(s),
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
        }

        let kinds_terminal = terminals.clone().into_iter().map(|t| t.token.kind).collect();

        let start =
            match Symbol::new_from_terminals(&kinds_terminal,
                                             file.by_ref().lines().next().unwrap().unwrap()) {
                Ok(s) => s,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

        let mut rules = Vec::new();
        for _ in 0..file.by_ref().lines().next().unwrap().unwrap().parse().unwrap() {
            let rule = file.by_ref().lines().next().unwrap().unwrap();
            let mut sides = rule.splitn(2, " ");

            let lhs = match Symbol::new(Terminality::NonTerminal,
                                        sides.next().unwrap().to_string()) {
                Ok(s) => s,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
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

        let mut states = Vec::new();
        for i in 0..file.by_ref().lines().next().unwrap().unwrap().parse().unwrap() {
            states.push(State::new(i as usize));
        }

        for _ in 0..file.by_ref().lines().next().unwrap().unwrap().parse().unwrap() {
            let transition = file.by_ref().lines().next().unwrap().unwrap();
            let mut tx = transition.split(" ");

            let start_state: usize = tx.next().unwrap().parse().unwrap();
            let symbol = tx.next().unwrap();
            let function: Function = tx.next().unwrap().parse().unwrap();
            let value: usize = tx.next().unwrap().parse().unwrap();

            states[start_state].transitions.push(Transition {
                value: value,
                function: function,
                start_state: start_state,
                symbol: match Symbol::new_from_terminals(&kinds_terminal, symbol.to_string()) {
                    Ok(s) => s,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                },
            });
        }

        DFA {
            non_terminals: non_terminals,
            rules: rules,
            start: start,
            states: states,
            terminals: terminals,
        }
    }

    pub fn consume(&self, state: &usize, ref token: &Token) -> Result<Transition, ParserError> {
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

        Err(ParserError::new(ErrorMessage::UnparseableToken(format!("{}", token)),
                             Some(format!("last known state: {}", state))))
    }
}
