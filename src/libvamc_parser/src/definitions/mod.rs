pub mod ast;

use vamc_errors::Diagnostic;
use vamc_lexer::definitions::*;

pub type ParserResult<T> = Result<T, Diagnostic>;

pub struct Parser {
    /// The tokens from the lexer.
    pub tokens: Vec<Token>,
    /// The current token's index in tokens.
    pub index: usize,
}
