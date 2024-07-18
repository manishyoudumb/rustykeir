use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeirError {
    #[error("Wow, '{0}'? I didn't know the numbers had new characters. You're so innovative!")]
    InvalidCharacter(char),
    #[error("Is that a number or your IQ? Because it's definitely invalid here.")]
    InvalidNumber,
    #[error("Unexpected token? I'm shocked. Your code is usually so predictable.")]
    UnexpectedToken,
    #[error("Reached the end of the file already? Your attention span rivals that of a goldfish.")]
    UnexpectedEOF,
    #[error("Division by zero? Really? I thought we covered this in our elementary school.")]
    DivisionByZero,
    #[error("Invalid unary operator. Unary, binary... it's all the same to you, isn't it?")]
    InvalidUnaryOperator,
}