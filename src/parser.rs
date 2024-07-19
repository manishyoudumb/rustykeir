use crate::ast::{Expr, BinaryOp};
use crate::error::KeirError;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}


impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, KeirError> {
        self.parse_expression()
    }
}