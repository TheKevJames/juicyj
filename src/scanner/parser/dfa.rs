use std;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::str::FromStr;

use error::ErrorMessage;
use error::ParserError;
use scanner::common::Token;
use scanner::common::TokenKind;

// TODO: move some of these to new module
#[derive(Clone)]
pub enum Function {
    Reduce,
    Shift,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Function::Reduce => write!(f, "reduce"),
            Function::Shift => write!(f, "shift"),
        }
    }
}

impl FromStr for Function {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "reduce" => Ok(Function::Reduce),
            "shift" => Ok(Function::Shift),
            _ => Err("invalid grammar function"),
        }
    }
}

#[derive(Clone)]
pub struct Rule {
    pub lhs: Symbol, // NonTerminal
    pub rhs: Vec<Symbol>,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rhs = format!("[{}]",
                          self.rhs
                              .clone()
                              .into_iter()
                              .map(|s| format!("{}", s.token))
                              .collect::<Vec<String>>()
                              .join(", "));
        write!(f, "{} -> {}", self.lhs.token, rhs)
    }
}

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

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(f, "{}", self.state));
        for transition in &self.transitions {
            try!(write!(f, "\n  {}", transition));
        }
        Ok(())
    }
}

#[derive(Clone,PartialEq)]
pub enum Terminality {
    NonTerminal,
    Terminal,
}

impl std::fmt::Display for Terminality {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Terminality::NonTerminal => write!(f, "Non-Terminal"),
            Terminality::Terminal => write!(f, "Terminal"),
        }
    }
}

#[derive(Clone)]
pub struct Symbol {
    pub terminality: Terminality,
    pub token: Token,
}

impl Symbol {
    pub fn new(terminality: Terminality, value: String) -> Result<Symbol, ParserError> {
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
            Err(_) => return Err(ParserError::new(ErrorMessage::StringNotToken(value), None)),
        }
    }

    fn new_from_terminals(ref kinds_terminal: &Vec<TokenKind>,
                          value: String)
                          -> Result<Symbol, ParserError> {
        let kind = match value.parse() {
            Ok(ref kind) if kinds_terminal.contains(kind) => Terminality::Terminal,
            Ok(_) => Terminality::NonTerminal,
            _ => return Err(ParserError::new(ErrorMessage::StringNotToken(value), None)),
        };

        Symbol::new(kind, value)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} [{}]", self.token, self.terminality)
    }
}

#[derive(Clone)]
pub struct Transition {
    pub value: usize,
    pub function: Function,
    pub start_state: usize,
    pub symbol: Symbol,
}

impl std::fmt::Display for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}: {} {} {}",
               self.start_state,
               self.symbol,
               self.function,
               self.value)
    }
}

pub struct DFA {
    pub non_terminals: Vec<Symbol>, // NonTerminal
    pub terminals: Vec<Symbol>, // Terminal
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
