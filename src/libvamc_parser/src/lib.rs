mod definitions;
mod literal;

use vamc_lexer::definitions::Token;

use crate::definitions::*;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn bump(&mut self) {
        self.index += 1;
    }

    pub fn token(&self) -> &Token {
        &self.tokens[self.index]
    }
}
