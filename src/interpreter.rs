use crate::ast::{Expr, BinaryOp};
use crate::error::KeirError;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&self, expr: &Expr) -> Result<f64, KeirError> {
        match expr {
            Expr::Number(n) => Ok(*n),
            Expr::Binary(left,op, right ) => {
                let left_val = self.interpret(left)?;
                let right_val = self.interpret(right)?;

                match op {
                    BinaryOp::Add => Ok(left_val + right_val),
                    BinaryOp::Subtract => Ok(left_val - right_val),
                    BinaryOp::Multiply => Ok(left_val * right_val),
                    BinaryOp::Divide => {
                        if right_val == 0.0 {
                            return Err(KeirError::DivisionByZero);
                        } else {
                        Ok(left_val / right_val)
                        }
                    }
                    
                }
            }
            Expr::Unary(op,expr ) => {
                let val:f64 = self.interpret(expr)?;
                match op {
                    BinaryOp::Subtract => Ok(-val),
                    _ => Err(KeirError::InvalidUnaryOperator),
                } 
            }
        } 
    }

}
