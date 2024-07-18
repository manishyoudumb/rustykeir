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
