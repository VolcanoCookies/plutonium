#[derive(Debug, Clone)]
pub enum Node {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Operator(String),
    Variable(String),
    Block(Vec<Node>),
    // Condition, If, Else (Can be another if)
    If {
        condition: Box<Node>,
        if_body: Box<Node>,
        else_body: Option<Box<Node>>,
    },
    // Function name, Args
    Call {
        name: String,
        args: Vec<Node>,
    },
    Exit {
        value: Box<Node>,
    },
    Binary {
        op: Binary,
        left: Box<Node>,
        right: Box<Node>,
    },
    Unary {
        op: Unary,
        right: Box<Node>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binary {
    Or,
    And,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, Clone)]
pub enum Unary {
    Not,
    Negate,
}
