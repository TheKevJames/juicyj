use std;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;

use common::Token;

#[derive(Debug)]
pub enum Function {
    Reduce,
    Shift,
}

#[derive(Debug)]
pub struct Rule {
    lhs: Symbol, // NonTerminal
    rhs: Vec<Symbol>,
}

#[derive(Debug)]
pub struct State {
    state: u32,
    transitions: Vec<Transition>,
}

impl State {
    fn new(s: u32) -> State {
        State {
            transitions: Vec::new(),
            state: s,
        }
    }
}

#[derive(Debug)]
pub enum Terminality {
    NonTerminal,
    Terminal,
    Any,
}

#[derive(Debug)]
pub struct Symbol {
    terminality: Terminality,
    token: Token,
}

impl Symbol {
    fn new(terminality: Terminality, value: String) -> Symbol {
        match value.parse() {
            Ok(kind) => {
                Symbol {
                    terminality: terminality,
                    token: Token {
                        kind: kind,
                        lexeme: Some(value),
                    },
                }
            }
            Err(_) => {
                error!("could not convert string {} to Token", value);
                std::process::exit(1);
            }
        }
    }
}

#[derive(Debug)]
pub struct Transition {
    end_state: u32,
    function: Function,
    start_state: u32,
    symbol: Symbol,
}

#[derive(Debug)]
pub struct DFA {
    current: usize,
    non_terminals: Vec<Symbol>, // NonTerminal
    terminals: Vec<Symbol>, // Terminal
    rules: Vec<Rule>,
    states: Vec<State>,
    start: Symbol,
}

impl DFA {
    pub fn new() -> DFA {
        let mut file = match File::open("grammar/joos.lr1") {
            Ok(file) => BufReader::new(file),
            Err(_) => {
                error!("could not read grammar file");
                std::process::exit(1);
            }
        };

        let mut non_terminals = Vec::<Symbol>::new();
        let mut terminals = Vec::<Symbol>::new();
        let mut rules = Vec::<Rule>::new();
        let mut states = Vec::<State>::new();

        let num_terminals: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_terminals {
            let symbol = file.by_ref().lines().next().unwrap().unwrap();
            terminals.push(Symbol::new(Terminality::Terminal, symbol));
        }

        let num_non_terminals: u32 =
            file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_non_terminals {
            let symbol = file.by_ref().lines().next().unwrap().unwrap();
            non_terminals.push(Symbol::new(Terminality::NonTerminal, symbol));
        }

        let start = Symbol::new(Terminality::Any,
                                file.by_ref().lines().next().unwrap().unwrap());

        let num_rules: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_rules {
            let rule = file.by_ref().lines().next().unwrap().unwrap();
            let mut sides = rule.splitn(2, " ");

            let lhs = Symbol::new(Terminality::NonTerminal, sides.next().unwrap().to_string());
            let rhs = match sides.next() {
                Some(side) => {
                    side.split_whitespace()
                        .map(|s| Symbol::new(Terminality::Any, s.to_string()))
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
            states.push(State::new(i));
        }

        let num_transitions: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_transitions {
            let transition = file.by_ref().lines().next().unwrap().unwrap();
            let mut tx = transition.split(" ");

            let start_state = tx.next().unwrap().parse().unwrap();
            let symbol = tx.next().unwrap();
            let function = tx.next().unwrap();
            let end_state = tx.next().unwrap().parse().unwrap();

            states[start_state as usize].transitions.push(Transition {
                end_state: end_state,
                function: match function {
                    "reduce" => Function::Reduce,
                    "shift" => Function::Shift,
                    f => {
                        error!("invalid function {}", f);
                        std::process::exit(1);
                    }
                },
                start_state: start_state,
                symbol: Symbol::new(Terminality::Any, symbol.to_string()),
            });
        }

        DFA {
            current: 0,
            non_terminals: non_terminals,
            rules: rules,
            start: start,
            states: states,
            terminals: terminals,
        }
    }

    pub fn consume(&mut self, token: Token) {
        let ref states = self.states[self.current];
        let ref transitions = states.transitions;
        for transition in transitions {
            match token.kind {
                ref l if *l == transition.symbol.token.kind => {
                    debug!("match {:?} {:?}", transition, token)
                }
                _ => debug!("{:?} {:?}", transition, token),
            }
        }
    }
}
