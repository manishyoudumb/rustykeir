use crate::ast::{Expr, BinaryOp};
use crate::error::KeirError;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

