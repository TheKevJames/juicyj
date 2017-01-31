use common::Token;

#[derive(Debug)]
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
}
