pub mod tokenizer;
pub mod parser;
pub mod ast;
pub mod token;

use std::process::exit;
use std::{env, fs};

use crate::tokenizer::{Token, Tokenizer};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing file arg");
        exit(1);
    }

    let contents: String = fs::read_to_string(args.get(1).unwrap()).unwrap();

    let tokens: Vec<Token> = Tokenizer::tokenize(contents);

    dbg!(tokens);
}
