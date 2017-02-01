use std;

use common::error;
use common::TokenKind;
use parser::Node;
use parser::Tree;

pub struct Weeder {
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
            tree: tree,
        }
    }

    pub fn verify(&self, node: Option<Node>) {
        let node = match node {
            Some(n) => n,
            _ => self.tree.root.clone(),
        };

        if node.token.kind == TokenKind::NumValue {
            match node.token.lexeme {
                Some(ref n) if n.starts_with("0") => {
                    debug!("Octal digit!");
                    std::process::exit(42);
                },
                _ => (),
            }
        }

        for child in node.children {
            self.verify(Some(child));
        }
    }
}
