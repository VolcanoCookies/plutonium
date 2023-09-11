use core::panic;

use crate::{
    ast::{Binary, Node},
    operators::ADD,
    token::Token,
};

#[derive(Debug)]
pub struct TokenIterator {
    pub i: i32,
    pub size: usize,
    pub tokens: Vec<Token>,
}

impl TokenIterator {
    pub fn peek(&self) -> Result<Token, ()> {
        return if self.has_next() {
            Ok(self.tokens[self.i as usize + 1].clone())
        } else {
            Err(())
        };
    }

    pub fn curr(self) -> Result<Token, ()> {
        return if (self.i as usize) < self.size {
            Ok(self.tokens[self.i as usize].clone())
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
    pub fn parse(tokens: Vec<Token>) -> Node {
        let mut iter = TokenIterator {
            i: 0,
            size: tokens.len(),
            tokens,
        };

        return parse_block(&mut iter);
    }
}

fn parse_block(iter: &mut TokenIterator) -> Node {
    let mut nodes: Vec<Node> = Vec::new();

    while let Ok(token) = iter.peek() {
        iter.consume();
        match token {
            Token::Semi => {}
            Token::ParOp => todo!(),
            Token::ParCl => todo!(),
            Token::BraOp => todo!(),
            Token::BraCl => {
                return Node::Block(nodes);
            }
            Token::IntLiteral(_) => todo!("impl int lit"),
            Token::FloatLiteral(_) => todo!(),
            Token::StrLiteral(_) => todo!(),
            Token::Identifier(_) => todo!(),
            Token::Keyword(word) => nodes.push(parse_keyword(word, iter)),
            Token::Binary(_) => todo!(),
            Token::UnaryOperator(_) => todo!(),
        }
    }

    return Node::Block(nodes);
}

fn parse_keyword(word: String, iter: &mut TokenIterator) -> Node {
    if word == "return" {
        return Node::Exit {
            value: Box::new(parse_expr(iter, Token::Semi)),
        };
    }

    panic!("Invalid keyword {}", word);
}

fn parse_atom(iter: &mut TokenIterator) -> Node {
    return match iter.consume() {
        Token::ParOp => parse_expr(iter, Token::ParCl),
        Token::BraOp => parse_block(iter),
        Token::IntLiteral(val) => Node::Integer(i32::from_str_radix(val, 10).unwrap()),
        Token::FloatLiteral(_) => todo!("Implement floats"),
        Token::StrLiteral(val) => Node::String(val.to_owned()),
        Token::Identifier(_) => todo!("Implement variables"),
        Token::Keyword(word) => match word.as_str() {
            "true" => Node::Boolean(true),
            "false" => Node::Boolean(false),
            "if" => parse_if(iter),
            _ => panic!("Unexpected keyword {}", word),
        },
        _ => panic!("Invalid token"),
    };
}

/*

A literal value
Parantheses containing literal value
A function call

*/
fn parse_expr(iter: &mut TokenIterator, until: Token) -> Node {
    if let Ok(peek) = iter.peek() {
        iter.consume();

        if peek == until {
            panic!("Invalid expression, end too soon.");
        }

        let first = match peek {
            Token::ParOp => parse_expr(iter, Token::ParCl),
            Token::BraOp => parse_block(iter),
            Token::IntLiteral(val) => Node::Integer(i32::from_str_radix(&val, 10).unwrap()),
            Token::FloatLiteral(_) => todo!(),
            Token::StrLiteral(_) => todo!(),
            Token::Identifier(_) => todo!(),
            Token::UnaryOperator(_) => todo!(),
            _ => panic!("Invalid token {:?}", peek),
        };

        if let Ok(peek) = iter.peek() {
            iter.consume();

            if peek == until {
                return first;
            }

            return match peek {
                Token::Binary(_) => parse_operator(first, iter),
                _ => panic!("Invalid token {:?}", peek),
            };
        } else {
            panic!("Invalid expression, end too soon.");
        }
    } else {
        panic!("Invalid expression, end too soon.");
    }
}

fn parse_expr_short(iter: &mut TokenIterator) -> Node {
    if let Ok(peek) = iter.peek() {
        iter.consume();

        return match peek {
            Token::ParOp => parse_expr(iter, Token::ParCl),
            Token::BraOp => parse_block(iter),
            Token::IntLiteral(val) => Node::Integer(i32::from_str_radix(&val, 10).unwrap()),
            Token::FloatLiteral(_) => todo!(),
            Token::StrLiteral(_) => todo!(),
            Token::Identifier(_) => todo!(),
            Token::UnaryOperator(_) => todo!(),
            _ => panic!("Invalid token {:?}", peek),
        };
    } else {
        panic!("Invalid expression, end too soon.");
    }
}

fn parse_if(iter: &mut TokenIterator) -> Node {
    let mut else_body: Option<Node> = None;

    let condition = parse_expr(iter, Token::BraOp);
    let body = parse_block(iter);
    let mut else_body = None;
    if let Ok(Token::Keyword(word)) = iter.peek() {
        if word == "else" {
            iter.consume();
            else_body = Some(Box::new(parse_block(iter)));
        }
    }

    return Node::If {
        condition: Box::new(condition),
        if_body: Box::new(body),
        else_body,
    };
}

fn parse_operator(left: Node, iter: &mut TokenIterator) -> Node {
    if let Ok(peek) = iter.peek() {
        iter.consume();

        let right = parse_expr_short(iter);

        match iter.peek() {
            Ok(Token::Binary(op)) => {}
            _ => {
                return Node::Binary {
                    op: ADD.bin,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }

    return Node::String("".to_owned());
}

fn parse_args(iter: &mut TokenIterator) -> Vec<Node> {
    let mut args: Vec<Node> = Vec::new();
    while let Ok(token) = iter.peek() {
        iter.consume();
        match token {
            Token::ParOp => args.push(parse_expr(iter, Token::ParCl)),
            Token::ParCl => {
                return args;
            }
            Token::IntLiteral(_) => todo!(),
            Token::FloatLiteral(_) => todo!(),
            Token::StrLiteral(val) => args.push(Node::String(val)),
            Token::Identifier(name) => args.push(Node::Variable(name)),
            Token::Binary(_) => todo!(),
            _ => panic!("Invalid"),
        }
    }
    return args;
}
