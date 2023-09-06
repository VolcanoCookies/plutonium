use crate::{
    ast::{BlockNode, IntegerNode, Node, ProgramNode},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct TokenIterator {
    pub i: i32,
    pub size: usize,
    pub tokens: Vec<Token>,
}

impl TokenIterator {
    pub fn peek(&self) -> Result<&Token, ()> {
        return if self.has_next() {
            Ok(&self.tokens[self.i as usize + 1])
        } else {
            Err(())
        };
    }

    pub fn curr(&self) -> Result<&Token, ()> {
        return if (self.i as usize) < self.size {
            Ok(&self.tokens[self.i as usize])
        } else {
            Err(())
        };
    }

    pub fn has_next(&self) -> bool {
        return self.i as usize + 1 < self.size;
    }

    pub fn consume(&mut self) -> &Token {
        let t = &self.tokens[self.i as usize];
        self.i += 1;
        return t;
    }
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> ProgramNode {
        let mut iter = TokenIterator {
            i: 0,
            size: tokens.len(),
            tokens: tokens,
        };

        let mut node = ProgramNode {
            statements: Vec::new(),
        };

        while iter.has_next() {
            let token = iter.consume();

            match token {
                _ => {}
            }
        }

        return node;
    }

    fn parse_token(mut iter: &TokenIterator) {}
}

fn parse_block(mut iter: &TokenIterator) -> BlockNode {
    let node = BlockNode {
        statements: Vec::new(),
    };

    while iter.has_next() {
        let token = iter.curr();

        match token._type {
            TokenType::Semi => todo!(),
            TokenType::ParOp => todo!(),
            TokenType::ParCl => todo!(),
            TokenType::BraOp => parse_block(&iter),
            TokenType::BraCl => panic!("Unexpected closing bracket"),
            TokenType::IntLiteral => todo!(),
            TokenType::FloatLiteral => todo!(),
            TokenType::StrLiteral => todo!(),
            TokenType::Identifier => todo!(),
            TokenType::Keyword => todo!(),
        };
    }

    return node;
}

fn parse_expr(mut iter: &TokenIterator) {}

fn parse_int(mut iter: &TokenIterator) -> IntegerNode {
    if let Ok(peek) = iter.peek() {
        if peek._type == TokenType::Operator {
            return;
        }
    }

    return Literal;
}

fn maybe_operator(mut iter: &TokenIterator) -> bool {
    let token = iter.curr().unwrap();

    return match token._type {
        TokenType::Operator => true,
        _ => false,
    };
}
