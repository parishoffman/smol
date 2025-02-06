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

type ParseResult<T> = Result<T, ParseError>;

pub fn parse(input: &str) -> Result<Program, ParseError> {
    let mut parser = Parser::new(input);
    let program = parser.parse_program()?;
    if parser.tokens.is_empty() {
        Err(ParseError(
            "There are still leftover tokens after reading a whole program.".to_string(),
        ))
    } else {
        Ok(program)
    }
}

struct Parser<'input> {
    /// Rest of the input, ordered in reverse.
    tokens: Vec<Token<'input>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut tokens = get_tokens(input);
        tokens.reverse();
        Parser { tokens }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.last().copied()
    }

    fn next(&mut self) -> ParseResult<Token> {
        self.tokens
            .pop()
            .ok_or(ParseError("Unexpected end of input.".to_owned()))
    }

    fn next_is(&self, kind: TokenKind) -> bool {
        self.peek().map(|t| t.kind == kind).unwrap_or(false)
    }

    fn eat(&self, kind: TokenKind) -> ParseResult<()> {
        if self.next_is(kind) {
            Ok(())
        } else {
            if let Some(actual) = self.peek() {
                Err(ParseError(format!(
                    "Expected a token with kind {kind}, found a token with kind {} and text `{}`.",
                    actual.kind, actual.text
                )))
            } else {
                Err(ParseError(format!(
                    "Expected a token with kind {kind} but reached the end of input."
                )))
            }
        }
    }

    fn parse_program(&mut self) -> ParseResult<Program> {
        todo!()
    }

    fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        todo!()
    }

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        todo!()
    }
}
