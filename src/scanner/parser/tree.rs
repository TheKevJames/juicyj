use scanner::common::Token;
use scanner::common::TokenKind;

#[derive(Clone)]
pub struct ParseTree {
    pub root: ParseNode,
}

impl ParseTree {
    pub fn print(self) {
        self.root.print(0);
    }
}

#[derive(Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub token: Token,
}

impl ParseNode {
    pub fn print(self, indent: u32) {
        let spaces = (0..indent).map(|_| " ").collect::<String>();
        println!("{}{}", spaces, self.token);

        for child in self.children {
            child.print(indent + 2);
        }
    }

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
}
