use scanner::common::Token;

#[derive(Clone)]
pub struct ASTNode {
    pub token: Token,
    pub children: Vec<ASTNode>,
}

// TODO: Display
impl ASTNode {
    pub fn print(&self, indent: u32) {
        let spaces = (0..indent).map(|_| " ").collect::<String>();
        println!("{}{}", spaces, self.token);

        for child in self.children.clone() {
            child.print(indent + 2);
        }
    }
}
