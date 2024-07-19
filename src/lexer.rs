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

    fn tokenize_number(&mut self) -> Result<Token, KeirError> {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if !ch.is_ascii_digit() && ch != '.' {
                break;
            }
            self.advance();
        }
        let number_str = &self.input[start..self.position];
        number_str.parse::<f64>().map(Token::Number).map_err(|_| KeirError::InvalidNumber)
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            self.position += ch.len_utf8();
        }
    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_mixed_operations_and_whitespace() {
        let mut lexer = Lexer::new("  3.14  *  (  -42 + 7.5)/ 2.0  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(3.14),
                Token::Multiply,
                Token::LeftParen,
                Token::Minus,
                Token::Number(42.0),
                Token::Plus,
                Token::Number(7.5),
                Token::RightParen,
                Token::Divide,
                Token::Number(2.0),
                Token::EOF
            ],
            "Well, well, well. Looks like someone's trying to make a lexer sweat with all these mixed operations and whitespace. How delightfully devious!"
        );
    }
}




