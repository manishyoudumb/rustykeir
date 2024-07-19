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
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, KeirError> {
        self.add_sub()
    }
    fn add_sub(&mut self) -> Result<Expr, KeirError> {
        let mut left = self.mul_div()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.mul_div()?;
                    left = Expr::Binary(Box::new(left), BinaryOp::Add, Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.mul_div()?;
                    left = Expr::Binary(Box::new(left), BinaryOp::Subtract, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn mul_div(&mut self) -> Result<Expr,KeirError> {
        let mut left = self.unary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Multiply => {
                    self.advance();
                    let right = self.unary()?;
                    left = Expr::Binary(Box::new(left), BinaryOp::Multiply, Box::new(right));
                }
                Token::Divide => {
                    self.advance();
                    let right = self.unary()?;
                    left = Expr::Binary(Box::new(left), BinaryOp::Divide, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    
    }

}