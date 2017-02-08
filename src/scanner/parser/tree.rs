use scanner::common::Token;
use scanner::common::TokenKind;

#[derive(Debug,Clone)]
pub struct Tree {
    pub root: Node,
}

impl Tree {
    pub fn print(self) {
        self.root.print(0);
    }
}

#[derive(Debug,Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub token: Token,
}

impl Node {
    pub fn print(self, indent: u32) {
        let spaces = (0..indent).map(|_| " ").collect::<String>();
        println!("{}{:?}", spaces, self.token);

        for child in self.children {
            child.print(indent + 2);
        }
    }

    pub fn collect_child_kinds(self, kinds: &Vec<&TokenKind>, collector: &mut Vec<Token>) {
        if kinds.contains(&&self.token.kind) {
            collector.push(self.token.clone());
        }

        for child in self.children {
            child.collect_child_kinds(kinds, collector);
        }
    }

    pub fn collect_child_lexeme(self, lexeme: &str, collector: &mut Vec<Node>) {
        match self.token.lexeme {
            Some(ref l) if l == lexeme => collector.push(self.clone()),
            _ => (),
        }

        for child in self.children {
            child.collect_child_lexeme(lexeme, collector);
        }
    }

    pub fn has_child_kind(self, kind: &TokenKind) -> bool {
        if &self.token.kind == kind {
            return true;
        }

        for child in self.children {
            if child.has_child_kind(&kind) {
                return true;
            }
        }

        false
    }
}
