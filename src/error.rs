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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keir_error_messages() {
        let errors = vec![
            (KeirError::InvalidCharacter('$'), "Wow, '$'? I didn't know the numbers had new characters. You're so innovative!"),
            (KeirError::InvalidNumber, "Is that a number or your IQ? Because it's definitely invalid here."),
            (KeirError::UnexpectedToken, "Unexpected token? I'm shocked. Your code is usually so predictable."),
            (KeirError::UnexpectedEOF, "Reached the end of the file already? Your attention span rivals that of a goldfish."),
            (KeirError::DivisionByZero, "Division by zero? Really? I thought we covered this in our elementary school."),
            (KeirError::InvalidUnaryOperator, "Invalid unary operator. Unary, binary... it's all the same to you, isn't it?"),
        ];

        for (error, expected_message) in errors {
            assert_eq!(error.to_string(), expected_message, "Our error messages are as sarcastic as ever. Bravo!");
        }
    }
}
