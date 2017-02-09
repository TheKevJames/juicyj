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
    pub fn new() -> Result<DFA, ParserError> {
        let filename = "grammar/joos.lr1";
        let mut file = match File::open(filename) {
            Ok(file) => BufReader::new(file),
            Err(_) => {
                return Err(ParserError::new(ErrorMessage::CouldNotReadFile(filename.to_string()),
                                            None));
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
            let function: Function = tx.next().unwrap().parse().unwrap();
            let value: usize = tx.next().unwrap().parse().unwrap();

            states[start_state].transitions.push(Transition {
                value: value,
                function: function,
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
