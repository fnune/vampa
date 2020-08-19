pub mod ast;

use std::{iter::Peekable, vec::IntoIter};

use vamc_errors::Diagnostic;
use vamc_lexer::definitions::*;

pub type ParserResult<T> = Result<T, Diagnostic>;

pub struct Parser {
    /// The tokens from the lexer.
    pub tokens: Peekable<IntoIter<Token>>,
    /// The latest token.
    pub current: Token,
}
