use common::Token;

pub struct Parser<T: Iterator<Item = Token>> {
    tokens: T,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(it: T) -> Parser<T> {
        Parser { tokens: it }
    }

    pub fn get_tree(self) {
        let tokens = self.tokens.collect::<Vec<Token>>();

        debug!("got tokens {:?}", tokens);
    }
}
