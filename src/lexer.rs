use crate::error::KeirError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    EOF,

}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, KeirError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(ch) = self.peek() {
            match ch {
                '0'..='9' => tokens.push(self.tokenize_number()?),
                '+' => {
                    self.advance();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::Multiply);
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::Divide);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RightParen);
                }
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }
                _ => return Err(KeirError::InvalidCharacter(ch)),
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)

    }




}



