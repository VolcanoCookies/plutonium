use crate::operators::{BinaryOperator, UnaryOperator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Semi,
    ParOp,
    ParCl,
    BraOp,
    BraCl,
    IntLiteral(String),
    FloatLiteral(String),
    StrLiteral(String),
    Identifier(String),
    Keyword(String),
    Binary(BinaryOperator),
    UnaryOperator(UnaryOperator),
}
