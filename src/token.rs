#[derive(Debug, Clone)]
pub enum TokenType {
    Semi,
    ParOp,
    ParCl,
    BraOp,
    BraCl,
    IntLiteral,
    FloatLiteral,
    StrLiteral,
    Identifier,
    Keyword,
    Operator,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub value: Option<String>,
}

impl Token {}
