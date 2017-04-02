use std::fmt;
use std::str::FromStr;

use error::ErrorMessage;
use error::ParserError;
use scanner::common::Token;
use scanner::common::TokenKind;

#[derive(Clone)]
pub enum Function {
    Reduce,
    Shift,
}

#[derive(Clone)]
pub struct Rule {
    pub lhs: Symbol,
    pub rhs: Vec<Symbol>,
}

pub struct State {
    pub state: usize,
    pub transitions: Vec<Transition>,
}

#[derive(Clone)]
pub struct Symbol {
    pub terminality: Terminality,
    pub token: Token,
}

#[derive(Clone,PartialEq)]
pub enum Terminality {
    NonTerminal,
    Terminal,
}

#[derive(Clone)]
pub struct Transition {
    pub value: usize,
    pub function: Function,
    pub start_state: usize,
    pub symbol: Symbol,
}

impl State {
    pub fn new(s: usize) -> State {
        State {
            transitions: Vec::new(),
            state: s,
        }
    }
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

    pub fn new_from_terminals(ref kinds_terminal: &Vec<TokenKind>,
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

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Function::Reduce => write!(f, "reduce"),
            Function::Shift => write!(f, "shift"),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.state));
        for transition in &self.transitions {
            try!(write!(f, "\n  {}", transition));
        }
        Ok(())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.token, self.terminality)
    }
}

impl fmt::Display for Terminality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Terminality::NonTerminal => write!(f, "Non-Terminal"),
            Terminality::Terminal => write!(f, "Terminal"),
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}: {} {} {}",
               self.start_state,
               self.symbol,
               self.function,
               self.value)
    }
}
