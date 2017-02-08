use std::fmt;

use scanner::common::Token;
use scanner::common::TokenKind;

#[derive(Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub token: Token,
}

#[derive(Clone)]
pub struct ParseTree {
    pub root: ParseNode,
}

impl ParseNode {
    pub fn collect_child_kinds(&self, kinds: &Vec<&TokenKind>, collector: &mut Vec<Token>) {
        if kinds.contains(&&self.token.kind) {
            collector.push(self.token.clone());
        }

        for child in &self.children {
            child.collect_child_kinds(kinds, collector);
        }
    }

    pub fn collect_child_lexeme(&self, lexeme: &str, collector: &mut Vec<ParseNode>) {
        match self.token.lexeme {
            Some(ref l) if l == lexeme => collector.push(self.clone()),
            _ => (),
        }

        for child in &self.children {
            child.collect_child_lexeme(lexeme, collector);
        }
    }

    pub fn has_child_kind(&self, kind: &TokenKind) -> bool {
        if &self.token.kind == kind {
            return true;
        }

        for child in &self.children {
            if child.has_child_kind(&kind) {
                return true;
            }
        }

        false
    }

    pub fn print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        match indent {
            0 => try!(write!(f, "{:width$}{}", "", self.token, width = indent)),
            _ => try!(write!(f, "{:width$}{}", "\n", self.token, width = indent)),
        }
        for child in &self.children {
            try!(child.print(f, indent + 2));
        }
        Ok(())
    }
}

impl fmt::Display for ParseNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f, 0)
    }
}

impl fmt::Display for ParseTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
