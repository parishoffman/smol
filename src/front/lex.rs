//! The lexer.

use derive_more::Display;
use regex::Regex;

/// Tokens in the program
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("kind: '{kind}', part of input: '{text}'")]
pub struct Token<'src> {
    /// What token class this token belongs to.
    kind: TokenKind,
    /// What part of the input this token carries.
    text: &'src str,
}

/// Token classes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display)]
enum TokenKind {
    #[display("id")]
    Id,
    #[display("num")]
    Num,
    #[display(":=")]
    Assign,
    #[display("$print")]
    Print,
    #[display("$read")]
    Read,
    #[display("$if")]
    If,
    #[display("{{")]
    LBrace,
    #[display("}}")]
    RBrace,
    #[display("+")]
    Plus,
    #[display("-")]
    Minus,
    #[display("*")]
    Mul,
    #[display("/")]
    Div,
    #[display("<")]
    Lt,
}

pub struct LexError(usize, char);

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error: unexpected character {:?} at {}", self.1, self.0)
    }
}

impl std::fmt::Debug for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error: unexpected character {:?} at {}", self.1, self.0)
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    whitespace: Regex,
    matchers: Vec<(Regex, TokenKind)>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            pos: 0,
            whitespace: Regex::new(r"\A(?:[ \t\f\r\n\v]|(?://.*))*").unwrap(),
            matchers: vec![],  // todo: fill this in
        }
    }

    /// Has the lexer reached the end of input?
    pub fn end_of_input(&self) -> bool {
        self.pos == self.input.len()
    }

    // Skip comments and whitespace
    fn skip_whitespace(&mut self) {
        if let Some(m) = self.whitespace.find(&self.input[self.pos..]) {
            self.pos += m.end()
        }
    }

    /// Get the next token if possible.
    ///
    /// The return type distinguishes between end-of-input and lexer error.
    pub fn next(&mut self) -> Result<Option<Token>, LexError> {
        todo!()
    }
}
