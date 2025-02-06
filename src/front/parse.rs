//! The parser

use std::fmt::Debug;

use derive_more::derive::Display;

use super::ast::*;
use super::lex::*;

#[derive(Display)]
#[display("Parse error: {}", self.0)]
pub struct ParseError(String);

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

pub fn parse(input: &str) -> Result<Program, ParseError> {
    todo!()
}
