#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(BinaryOp, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_construction_and_equality() {
        let expr1 = Expr::Binary(
            Box::new(Expr::Number(3.14)),
            BinaryOp::Add,
            Box::new(Expr::Unary(
                BinaryOp::Subtract,
                Box::new(Expr::Binary(
                    Box::new(Expr::Number(2.0)),
                    BinaryOp::Multiply,
                    Box::new(Expr::Number(4.0))
                ))
            ))
        );

        let expr2 = Expr::Binary(
            Box::new(Expr::Number(3.14)),
            BinaryOp::Add,
            Box::new(Expr::Unary(
                BinaryOp::Subtract,
                Box::new(Expr::Binary(
                    Box::new(Expr::Number(2.0)),
                    BinaryOp::Multiply,
                    Box::new(Expr::Number(4.0))
                ))
            ))
        );

        assert_eq!(expr1, expr2, "Well, well, well... Looks like our AST can handle complex expressions and maintain equality. How delightfully competent!");
    }
}
