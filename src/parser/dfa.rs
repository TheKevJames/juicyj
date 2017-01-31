use std;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;

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
    pub fn new(s: u32) -> State {
        State {
            transitions: Vec::new(),
            state: s,
        }
    }
}

#[derive(Debug)]
pub enum Symbol {
    NonTerminal { symbol: String },
    Terminal { symbol: String },
    Any { symbol: String },
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
    pub non_terminals: Vec<Symbol>, // NonTerminal
    pub terminals: Vec<Symbol>, // Terminal
    pub rules: Vec<Rule>,
    pub states: Vec<State>,
    pub start: Symbol,
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
            terminals.push(Symbol::Terminal { symbol: symbol });
        }

        let num_non_terminals: u32 =
            file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_non_terminals {
            let symbol = file.by_ref().lines().next().unwrap().unwrap();
            non_terminals.push(Symbol::NonTerminal { symbol: symbol });
        }

        let start = Symbol::Any { symbol: file.by_ref().lines().next().unwrap().unwrap() };

        let num_rules: u32 = file.by_ref().lines().next().unwrap().unwrap().parse().unwrap();
        for _ in 0..num_rules {
            let rule = file.by_ref().lines().next().unwrap().unwrap();
            let mut sides = rule.splitn(2, " ");
            let (lhs, rhs) = (sides.next().unwrap(), sides.next().unwrap());

            let lhs = Symbol::NonTerminal { symbol: lhs.to_string() };
            let rhs = rhs.split_whitespace()
                .map(|s| Symbol::Any { symbol: s.to_string() })
                .collect();
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
                symbol: Symbol::Any { symbol: symbol.to_string() },
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
}
