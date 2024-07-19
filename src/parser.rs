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

    fn unary(&mut self) -> Result<Expr,KeirError> {
        if let Some(Token::Minus) = self.peek() {
            self.advance();
            let expr = self.unary()?;
            Ok(Expr::Unary(BinaryOp::Subtract, Box::new(expr)))
        } 
        else {
            self.primary()
        }
        
    }

    fn primary(&mut self) -> Result<Expr, KeirError> {
        if let Some(token) = self.advance() {
            match token {
                Token::Number(n) => Ok(Expr::Number(*n)),
                Token::LeftParen => {
                    let expr = self.expression()?;
                    self.expect(Token::RightParen)?;
                    Ok(expr)
                }
                _ => Err(KeirError::UnexpectedToken),
            }
        } else {
            Err(KeirError::UnexpectedEOF)
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    fn expect (&mut self, expected: Token) -> Result<(), KeirError> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } 
        else {
            Err(KeirError::UnexpectedToken)
        }
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_parse_complex_expression_with_precedence() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Multiply,
            Token::LeftParen,
            Token::Number(3.0),
            Token::Minus,
            Token::Number(4.0),
            Token::RightParen,
            Token::Divide,
            Token::Number(5.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        // Expected AST: 1 + ((2 * (3 - 4)) / 5)
        let expected = Expr::Binary(
            Box::new(Expr::Number(1.0)),
            BinaryOp::Add,
            Box::new(Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Number(2.0)),
                    BinaryOp::Multiply,
                    Box::new(Expr::Binary(
                        Box::new(Expr::Number(3.0)),
                        BinaryOp::Subtract,
                        Box::new(Expr::Number(4.0)),
                    )),
                )),
                BinaryOp::Divide,
                Box::new(Expr::Number(5.0)),
            )),
        );

        assert_eq!(result, expected, "Well, well, well. Looks like our parser can handle complex expressions after all. How delightfully competent!");
    }
}
