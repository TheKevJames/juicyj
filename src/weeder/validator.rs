use std;

use common::error;
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
}
