pub mod ast;
pub mod consts;
mod operators;
pub mod parser;
pub mod token;
pub mod tokenizer;

use std::process::exit;
use std::{env, fs};

use crate::parser::Parser;
use crate::token::Token;
use crate::tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing file arg");
        exit(1);
    }

    let contents: String = fs::read_to_string(args.get(1).unwrap()).unwrap();

    let mut tokens: Vec<Token> = Tokenizer::tokenize(contents);

    dbg!(tokens.clone());

    tokens.insert(0, Token::Semi);

    let ast = Parser::parse(tokens);

    dbg!(ast);
}
