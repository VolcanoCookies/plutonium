use crate::ast::Binary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOperator {
    pub op: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOperator {
    pub op: &'static str,
    pub precedence: i32,
    pub bin: Binary,
}

pub const ADD: BinaryOperator = BinaryOperator {
    op: "+",
    precedence: 10,
    bin: Binary::Add,
};

pub const SUB: BinaryOperator = BinaryOperator {
    op: "-",
    precedence: 10,
    bin: Binary::Subtract,
};

pub const MUL: BinaryOperator = BinaryOperator {
    op: "*",
    precedence: 20,
    bin: Binary::Multiply,
};

pub const DIV: BinaryOperator = BinaryOperator {
    op: "/",
    precedence: 20,
    bin: Binary::Divide,
};

pub const BINARY_OPERATORS: &[BinaryOperator] = &[ADD, SUB, MUL, DIV];
