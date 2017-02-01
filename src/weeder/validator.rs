use std;

use common::error;
use common::TokenKind;
use parser::Node;
use parser::Tree;

pub struct Weeder {
    has_class: bool,
    tree: Tree,
}

impl Weeder {
    pub fn new(tree: Result<Tree, error::ParserError>) -> Weeder {
        let tree = match tree {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                std::process::exit(42);
            }
        };
        tree.clone().print();

        Weeder {
            has_class: false,
            tree: tree,
        }
    }

    pub fn verify(&mut self, node: Option<Node>) {
        let node = match node {
            Some(n) => n,
            _ => self.tree.root.clone(),
        };

        match node.token.kind {
            TokenKind::NumValue => match node.token.lexeme {
                Some(ref n) if n.starts_with("0") => {
                    debug!("Octal digit!");
                    std::process::exit(42);
                },
                _ => (),
            },
            TokenKind::Class => match self.has_class {
                true => {
                    debug!("Multiple classes!");
                    std::process::exit(42);
                },
                false => self.has_class = true,
            },
            _ => (),
        }

        for child in node.children {
            self.verify(Some(child));
        }
    }
}
