//! The parser

use std::fmt::Debug;

use derive_more::derive::Display;

use super::ast::*;
use super::lex::*;

#[derive(Display)]
#[display("{}", self.0)]
pub struct ParseError(String);

impl From<LexError> for ParseError {
    fn from(lex_error: LexError) -> Self {
        ParseError(format!("{}", lex_error))
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

pub fn parse(input: &str) -> Result<Program, ParseError> {
    todo!()
}
